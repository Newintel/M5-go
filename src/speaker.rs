use std::borrow::BorrowMut;

use esp_idf_hal::{
    delay::FreeRtos,
    gpio::OutputPin,
    ledc::{config::TimerConfig, LedcChannel, LedcDriver, LedcTimer, LedcTimerDriver},
    peripheral::Peripheral,
    units::Hertz,
};

use crate::Delay;

pub struct Speaker<P: OutputPin, C: LedcChannel, T: LedcTimer>
where
    C: Peripheral<P = C>,
    T: Peripheral<P = T>,
{
    pin: P,
    channel: C,
    timer: T,
}

impl<P: OutputPin, C: LedcChannel, T: LedcTimer> Speaker<P, C, T>
where
    C: Peripheral<P = C>,
    T: Peripheral<P = T>,
{
    pub fn new(pin: P, channel: C, timer: T) -> Self {
        Self {
            pin,
            channel,
            timer,
        }
    }

    pub fn do_sound(&mut self, freq: u32, delay: Delay, volume: Option<u32>) {
        // The driver must me reconfigurated every time otherwise it is impossble to change the frequence
        let config = TimerConfig::new().frequency(Hertz(freq));
        let mut speaker = LedcDriver::new(
            self.channel.borrow_mut(),
            LedcTimerDriver::new(self.timer.borrow_mut(), &config).unwrap(),
            self.pin.borrow_mut(),
        )
        .unwrap();

        speaker.set_duty(volume.unwrap_or(1)).unwrap();

        match delay {
            Delay::Us(us) => FreeRtos::delay_us(us),
            Delay::Ms(ms) => FreeRtos::delay_ms(ms),
        };

        speaker.disable().unwrap();
    }
}
