use core::cell::RefCell;

use alloc::boxed::Box;
use da14531_hal::{
    cm::{interrupt::Mutex, peripheral::SCB, Peripherals as CmPeripherals},
    crg_aon::sleep::SleepConfig,
    crg_aon::{CrgAon, CrgAonExt},
    crg_top::{CrgTop, CrgTopExt},
    gpadc::{
        config::{
            AdcConfig, AdcInputTemp, AdcInputVbatHigh, AdcInputVbatLow, AdcInputVddd, Attenuation,
            Averaging, Chopper, SampleTime,
        },
        GpAdc, GpAdcExt,
    },
    gpio::{p0::Parts, Output, Pin},
    hal::{adc::Channel, digital::v2::{OutputPin, PinState}},
    i2c::I2cExt,
    nvic::{Irq, Nvic, NvicExt},
    pac::{Peripherals, GPADC, NVIC},
    sys_wdog::{SysWdog, SysWdogExt},
    timer::{BaseClockDiv, Timer0, Timer0Ext},
};
use da14531_sdk::platform::{
    driver::syscntl::{dcdc_turn_on_in_boost, SyscntlDcdcLevel::SYSCNTL_DCDC_LEVEL_3V0},
    system_library::patch_func,
};
use rtt_target::rprintln;

use crate::{
    app::{PeripheralsDriver, Sound},
    app_impl::app,
};

use self::audio::Audio;

mod audio;

/// This struct contains all relevant peripherals and implements the `PeripheralsDriver` trait
pub struct Da14531Peripherals {
    sys_wdog: SysWdog,
    nvic: Nvic,
    scb: SCB,
    crg_aon: CrgAon,
    crg_top: CrgTop,

    /// Configuration for the sleep mode
    sleep_config: SleepConfig,

    /// Used for PWM0 (audio)
    pwm_timer: Timer0,

    /// Temperature ADC
    adc: GpAdc,

    /// PWM piezo peripheral (In `Mutex<...>` since it needs to be interrupt safe)
    audio: Mutex<RefCell<Audio>>,

    /// LED pin
    led_pin: Pin<Output>,
}

impl Da14531Peripherals {
    pub fn new() -> Self {
        dcdc_turn_on_in_boost(SYSCNTL_DCDC_LEVEL_3V0);

        patch_func();

        let dp = Peripherals::take().unwrap();
        let cp = CmPeripherals::take().unwrap();

        let p0 = Parts::new(dp.GPIO);

        // Get necessary peripherals
        let mut nvic = cp.NVIC.constrain();
        let scb = cp.SCB;
        let crg_top = dp.CRG_TOP.constrain();
        let mut crg_aon = dp.CRG_AON.constrain();
        let mut pwm_timer = dp.TIMER0.constrain();
        let mut sys_wdog = dp.SYS_WDOG.constrain();
        let i2c = dp.I2C.constrain();
        let adc = dp.GPADC.constrain();

        // Enable pad latch
        crg_aon.set_pad_latch_en(true);

        // Setup pins
        let _spi_flash_en = p0.p0_01.into_output(PinState::High); // Disallow spontaneous SPI Flash wake-up
        let wakeup_pin = p0.p0_05.into_floating_input();
        let pwm_buzzer = p0.p0_11.degrade().into_alternate();
        let led_pin = p0.p0_08.degrade().into_output(PinState::Low);

        pwm_timer.enable_clock();
        pwm_timer.set_clock_div(BaseClockDiv::Div8);

        Self::audio_init(pwm_buzzer, &mut pwm_timer, &mut nvic);

        let sleep_config = SleepConfig::default().enable_pin(wakeup_pin);

        Da14531Peripherals {
            sys_wdog,
            nvic: nvic,
            crg_aon,
            crg_top,
            sleep_config,
            scb,
            adc,
            pwm_timer,
            led_pin,
            audio: Mutex::new(RefCell::new(Audio::new())),
        }
    }
}

impl PeripheralsDriver for Da14531Peripherals {
    fn new() -> Self {
        Self::new()
    }

    /// Play a sound with modulated PWM on a piezo buzzer
    fn play_sound(
        &mut self,
        sound: Sound,
        repeat: bool,
        finish_callback: Option<Box<dyn FnOnce()>>,
    ) {
        self.audio_play_sound(sound, repeat, finish_callback);
    }

    /// Put MCU in hibernation mode
    fn start_hibernation(&mut self) {
        self.crg_aon.init_sleep(
            &mut self.nvic,
            &mut self.crg_top,
            &mut self.sys_wdog,
            &mut self.scb,
            &self.sleep_config,
        );
    }

    /// Get die temperature in milli °C (28230 = 28.23°C)
    fn get_temperature(&self) -> u16 {
        self.adc.init(
            AdcConfig::default()
                .set_channel_pos(AdcInputTemp)
                .set_chopper_mode(Chopper::On)
                .set_sample_time(SampleTime::Cycles15X8)
                .set_averaging(Averaging::SamplesX32),
        );

        self.adc.start_conversion();
        self.adc.wait_for_conversion();
        let result = self.adc.current_sample();
        let temp =
            (25000 + ((result as i32 - 30272i32) as f32 / (1.45 * 64.0) * 1000.0) as i32) as u16;
        rprintln!("AdcInputTemp value: {}", result);
        rprintln!("AdcInputTemp temp: {} m°C", temp);

        self.adc.disable();

        temp
    }

    /// Feed the dog :)
    fn feed_watchdog(&mut self) {
        self.sys_wdog.feed();
    }

    /// Turn LED on/off
    fn set_led(&mut self, state: bool) {
        self.led_pin.set_state(match state{
            true => PinState::High,
            false => PinState::Low,
        });
    }
}
