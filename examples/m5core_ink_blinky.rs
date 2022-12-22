use esp_idf_hal::{delay::FreeRtos, prelude::Peripherals};

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    let mut m5 = m5_go::M5CoreInk::new(peripherals)?;

    loop {
        m5.led.set_high()?;
        FreeRtos::delay_ms(100);
        m5.led.set_low()?;
        FreeRtos::delay_ms(100);
    }
}
