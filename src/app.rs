use core::{marker::PhantomData, u8};

use alloc::boxed::Box;
use da14531_sdk::app_modules::timer::AppTimer;
use rtt_target::{rprint, rprintln};

/// Stop advertising and go to hibernation after X secs
const APP_ADVERTISTE_STOP_TIMER: u32 = 60;

/// Type of sound to play
#[derive(Clone, Copy)]
pub enum Sound {
    Connected,
    Disconnected,
    UnlockSuccess,
    UnlockFail,
    Alarm,
}

/// Defines an interface to access the peripherals
pub trait PeripheralsDriver {
    fn new() -> Self;
    fn play_sound(
        &mut self,
        sound: Sound,
        repeat: bool,
        finish_callback: Option<Box<dyn FnOnce()>>,
    );
    fn start_hibernation(&mut self);
    fn get_temperature(&self) -> u16;
    fn feed_watchdog(&mut self);
    fn set_led(&mut self, state: bool);
}

/// Defines an interface to control the BLE stack
pub trait BleDriver {
    fn start_adverstising();
    fn stop_adverstising();
    fn disconnect(connection_handle: u32);
}

/// Holds the state of the application
pub struct App<P, BLE>
where
    Self: 'static,
    P: 'static + PeripheralsDriver,
    BLE: 'static + BleDriver,
{
    ///
    hibernation_timer: Option<AppTimer>,
    peripherals: Option<P>,
    alarm_on: bool,
    connection_handle: Option<u32>,
    led_state: bool,
    _ble: PhantomData<BLE>,
}

/// Business logic of the application
impl<P, BLE> App<P, BLE>
where
    P: PeripheralsDriver,
    BLE: BleDriver,
{
    /// Create new instance of App
    pub const fn new() -> Self {
        Self {
            hibernation_timer: None,
            peripherals: None,
            _ble: PhantomData,
            alarm_on: false,
            connection_handle: None,
            led_state: false,
        }
    }

    /// Initialize peripherals
    pub fn init_peripherals(&mut self) {
        rprint!("Initializing peripherals...");

        self.peripherals = Some(P::new());

        rprintln!("done!");
    }

    /// Start timer that stops advertisement after `APP_ADVERTISTE_STOP_TIMER` seconds
    fn start_hibernation_timer(&mut self) {
        if self.hibernation_timer.is_none() {
            self.hibernation_timer = AppTimer::new(
                APP_ADVERTISTE_STOP_TIMER * 100,
                Box::new(|| {
                    BLE::stop_adverstising();
                    // self.on_init_hibernation();
                }),
            )
        }
    }

    /// Cancel hibernation timer
    fn cancel_hibernation_timer(&mut self) {
        if let Some(timer) = self.hibernation_timer.take() {
            timer.cancel();
        }
    }

    /// Start advertising handler
    pub fn on_start_advertising(&mut self) {
        rprintln!("App::on_start_advertising()");

        self.start_hibernation_timer();

        BLE::start_adverstising();
    }

    /// Start hibernation handler
    pub fn on_start_hibernation(&mut self) {
        rprintln!("App::on_start_hibernation()");
        self.peripherals().start_hibernation();
    }

    /// Set LED
    pub fn on_set_led(&mut self, state: bool) {
        self.peripherals().set_led(state);
        self.led_state = state;
    }

    /// Get state of the LED
    pub fn get_led_state(&mut self) -> bool {
        self.led_state
    }

    /// Get the die temperature
    pub fn get_temperature(&mut self) -> u16 {
        self.peripherals().get_temperature()
    }

    /// Connect event handler
    pub fn on_connect(&mut self, connection_handle: Option<u32>) {
        self.connection_handle = connection_handle;

        if connection_handle.is_some() {
            self.cancel_hibernation_timer();
            self.peripherals
                .as_mut()
                .unwrap()
                .play_sound(Sound::Connected, false, None);
        } else {
            self.cancel_hibernation_timer();
        }
    }

    /// Disonnect event handler
    pub fn on_disconnect(&mut self) {
        self.connection_handle = None;
        self.peripherals
            .as_mut()
            .unwrap()
            .play_sound(Sound::Disconnected, false, None);
        self.start_hibernation_timer();
    }

    /// Alarm event handler
    pub fn on_alarm(&mut self) {
        self.alarm_on = true;
        self.peripherals().play_sound(Sound::Alarm, true, None);
    }

    pub fn feed_watchdog(&mut self) {
        self.peripherals().feed_watchdog();
    }

    pub fn peripherals(&mut self) -> &mut P {
        self.peripherals.as_mut().unwrap()
    }
}
