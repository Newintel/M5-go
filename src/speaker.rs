use std::borrow::BorrowMut;

use esp_idf_hal::{
    gpio::OutputPin,
    ledc::{config::TimerConfig, LedcChannel, LedcDriver, LedcTimer, LedcTimerDriver},
    peripheral::Peripheral,
    units::Hertz,
};

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

    pub fn speaker_from_struct(speaker: &mut Self, freq: u32) -> LedcDriver<'_>
    where
        C: Peripheral<P = C>,
        T: Peripheral<P = T>,
    {
        let config = TimerConfig::new().frequency(Hertz(freq));
        let driver = LedcTimerDriver::new(speaker.timer.borrow_mut(), &config).unwrap();
        LedcDriver::new(
            speaker.channel.borrow_mut(),
            driver,
            speaker.pin.borrow_mut(),
        )
        .unwrap()
    }
}
