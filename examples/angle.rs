use esp_idf_hal::{delay::FreeRtos, prelude::Peripherals};
use smart_leds::colors::WHITE;

use m5_go::{leds::WithBrightness, M5Go};

const MAX_READ: u16 = 4095;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    let mut m5 = M5Go::new(peripherals)?;

    let mut last_read = 0;

    m5.leds.off();

    loop {
        let read = m5.port_b.read()?;
        if read.abs_diff(last_read) > 10 {
            last_read = read;
            m5.leds
                .fill(WHITE.with_brightness(brightness_from_read(read)));
            m5.leds.display();
        }
        FreeRtos::delay_ms(50);
    }
}

fn brightness_from_read(read: u16) -> u16 {
    let brightness = (read as f32 / MAX_READ as f32) * 255.;
    brightness as u16
}
