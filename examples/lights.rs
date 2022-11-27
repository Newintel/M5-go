use esp_idf_hal::{delay::FreeRtos, prelude::Peripherals};
use smart_leds::colors::{BROWN, CYAN, LIME_GREEN, MAGENTA, PINK, YELLOW};

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    let mut m5 = m5_go::M5Go::new(peripherals)?;

    let colors = [MAGENTA, YELLOW, BROWN, LIME_GREEN, CYAN, PINK];

    loop {
        for color in colors {
            for index in 0..10 {
                m5.leds.clear();
                m5.leds.set_color_at_index(index, color);
                m5.leds.display();
                FreeRtos::delay_ms(100);
            }
            FreeRtos::delay_ms(500);
        }
    }
}
