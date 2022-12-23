mod ble;
mod io;
mod leds;
mod screen;
mod speaker;

use ble::Ble;
pub use ble::BleConfig;
pub use leds::WithBrightness;

use esp_idf_hal::{
    gpio::{Gpio25, Gpio27, Gpio32, Gpio33, Gpio37, Gpio38, Gpio39, Gpio8, Input, PinDriver},
    ledc::{CHANNEL0, TIMER0},
    prelude::Peripherals,
    uart::{UartConfig, UartDriver},
    units::Hertz,
};
use io::IOPort;

use leds::Leds;
use screen::Screen;
use speaker::Speaker;

pub struct M5Go<'a> {
    pub button_a: PinDriver<'a, Gpio39, Input>,
    pub button_b: PinDriver<'a, Gpio38, Input>,
    pub button_c: PinDriver<'a, Gpio37, Input>,
    pub leds: Leds,
    pub screen: Screen<'a, Gpio27, Gpio33, Gpio32>,
    pub port_b: IOPort<'a>,
    pub port_c: UartDriver<'a>,
    pub speaker: Speaker<Gpio25, CHANNEL0, TIMER0>,
    pub ble: Option<Ble>,
}

impl<'a> M5Go<'a> {
    pub fn new(peripherals: Peripherals) -> anyhow::Result<Self> {
        // Port C
        let port_c_config = UartConfig::new().baudrate(Hertz(9600));
        let port_c = UartDriver::new(
            peripherals.uart2,
            peripherals.pins.gpio17,
            peripherals.pins.gpio16,
            None as Option<Gpio8>,
            None as Option<Gpio8>,
            &port_c_config,
        )?;

        // Port B
        let io_b = peripherals.pins.gpio26;
        let input_b = peripherals.pins.gpio36;
        let adc1 = peripherals.adc1;
        let port_b = IOPort::new(io_b, input_b, adc1)?;

        // Buttons
        let button_a = PinDriver::input(peripherals.pins.gpio39)?;
        let button_b = PinDriver::input(peripherals.pins.gpio38)?;
        let button_c = PinDriver::input(peripherals.pins.gpio37)?;

        // Screen
        let blk = peripherals.pins.gpio32;
        let sclk = peripherals.pins.gpio18;
        let sdo = peripherals.pins.gpio23;
        let cs = peripherals.pins.gpio14;
        let dc = peripherals.pins.gpio27;
        let reset = peripherals.pins.gpio33;

        let screen = Screen::new(cs, sdo, sclk, dc, reset, blk, peripherals.spi2);

        // Leds
        let leds = Leds::new(15);

        // Speaker
        let speaker_pin = peripherals.pins.gpio25;
        let channel0 = peripherals.ledc.channel0;
        let timer0 = peripherals.ledc.timer0;
        let speaker = Speaker::new(speaker_pin, channel0, timer0);

        Ok(Self {
            button_a,
            button_b,
            button_c,
            leds,
            screen,
            port_c,
            speaker,
            port_b,
            ble: None,
        })
    }

    pub fn setup_ble(&mut self, config: BleConfig) {
        let ble = Ble::new(config);
        self.ble = Some(ble);
    }
}

#[derive(Clone, Copy)]
pub enum Note {
    C = 4186,
    Cs = 4435,
    D = 4699,
    Eb = 4978,
    E = 5274,
    F = 5588,
    Fs = 5920,
    G = 6272,
    Gs = 6645,
    A = 7040,
    Bb = 7459,
    B = 7902,
    NONE = 0,
}

impl Note {
    pub fn octave(self, octave: u8) -> u32 {
        (self as u32) / (1 << (8 - octave))
    }
}

pub enum Delay {
    Us(u32),
    Ms(u32),
}
