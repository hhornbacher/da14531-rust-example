use alloc::boxed::Box;
use da14531_hal::{
    cm::interrupt,
    gpio::{AfPwm0, Disconnected, Pin},
    nvic::Nvic,
    timer::{ClockSel, PwmMode, Timer0, TimerClockDiv},
};

use crate::{app::Sound, app_impl::app};

use super::Da14531Peripherals;

struct SoundGenerator {
    sound: Sound,
    pos: usize,
    repeat: bool,
}

impl SoundGenerator {
    fn new(sound: Sound, repeat: bool) -> Self {
        Self {
            sound,
            pos: 0,
            repeat,
        }
    }

    fn next(&mut self) -> Option<(u16, u16)> {
        let result = match self.sound {
            Sound::Connected => {
                let offset = self.pos % 4;
                if self.pos != 0 && offset == 0 && !self.repeat {
                    return None;
                }
                ((1500 - (offset * 200)) as u16, 5)
            }
            Sound::Disconnected => {
                let offset = self.pos % 4;
                if self.pos != 0 && offset == 0 && !self.repeat {
                    return None;
                }
                ((800 + (offset * 200)) as u16, 5)
            }
            Sound::UnlockSuccess => {
                let offset = self.pos % 10;
                if self.pos != 0 && offset == 0 && !self.repeat {
                    return None;
                }
                ((1010 - (offset * 100)) as u16, 3)
            }
            Sound::UnlockFail => {
                let offset = self.pos % 8;
                let offset_2 = self.pos % 2;
                let offset_4 = self.pos % 4;
                if self.pos != 0 && offset == 0 && !self.repeat {
                    return None;
                }
                let freq = if offset_2 == 0 { 880 } else { 1046 };
                let duration = match offset_4 {
                    0 => 10,
                    1 => 5,
                    2 => 10,
                    3 => 15,
                    _ => 0,
                };

                (freq as u16, duration)
            }
            Sound::Alarm => {
                let offset = self.pos % 2;

                ((3040 + (offset * 300)) as u16, 10)
            }
        };

        self.pos += self.pos.wrapping_add(1);

        Some(result)
    }
}

pub(super) struct Audio {
    sound_generator: Option<SoundGenerator>,
    pwm_counter: usize,
    finish_callback: Option<Box<dyn FnOnce()>>,
    current_duration: usize,
}

impl Audio {
    pub(super) fn new() -> Self {
        Self {
            sound_generator: None,
            pwm_counter: 0,
            finish_callback: None,
            current_duration: 0,
        }
    }
}

impl Da14531Peripherals {
    #[inline]
    pub(super) fn audio_init(
        _pin: Pin<AfPwm0>,
        pwm_timer: &mut Timer0,
        interrupt_controller: &mut Nvic,
    ) {
        pwm_timer.init(
            interrupt_controller,
            ClockSel::SystemClock,
            PwmMode::High,
            TimerClockDiv::Off,
        );
        pwm_timer.register_handler(pwm_handler);
    }

    pub(super) fn audio_play_sound(
        &mut self,
        sound: Sound,
        repeat: bool,
        finish_callback: Option<Box<dyn FnOnce()>>,
    ) {
        self.pwm_timer.stop();

        interrupt::free(|cs| {
            let mut audio = self.audio.borrow(cs).borrow_mut();
            audio.sound_generator = Some(SoundGenerator::new(sound, repeat));
            audio.pwm_counter = 0;
            audio.current_duration = 0;
            audio.finish_callback = finish_callback;
        });

        self.set_frequency(0);
        self.pwm_timer.start();
    }

    pub fn audio_on_pwm_interrupt(&mut self) {
        let (stop, next_tune) = interrupt::free(|cs| {
            let mut audio = self.audio.borrow(cs).borrow_mut();

            let mut stop = false;
            let mut next_tune = 0;

            if audio.pwm_counter == audio.current_duration {
                if let Some(sound_generator) = audio.sound_generator.as_mut() {
                    if let Some((tune, duration)) = sound_generator.next() {
                        audio.current_duration = duration as usize;
                        audio.pwm_counter = 0;
                        next_tune = tune;
                    } else {
                        stop = true;
                    }
                }
            }

            audio.pwm_counter += 1;

            (stop, next_tune)
        });

        if !stop {
            self.set_frequency(next_tune);
        } else {
            self.stop_sound();
        }
    }

    fn stop_sound(&mut self) {
        self.pwm_timer.stop();

        interrupt::free(|cs| {
            let mut audio = self.audio.borrow(cs).borrow_mut();
            audio.sound_generator = None;
            audio.pwm_counter = 0;
            audio.current_duration = 0;

            if let Some(finish_callback) = audio.finish_callback.take() {
                finish_callback();
            }
        });
    }

    fn set_frequency(&mut self, freq: u16) {
        self.pwm_timer.set_pwm(0xffff, freq / 3 * 2, freq / 3);
    }
}

fn pwm_handler() {
    app().peripherals().audio_on_pwm_interrupt();
}
