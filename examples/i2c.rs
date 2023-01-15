/**
 * This example shows how to use the I2C bus on the M5Stack.
 * It reads the temperature and humidity from a SHT30 sensor contained in the M5Stack Env II component and displays it on the screen.
 */
use embedded_graphics::{
    mono_font::ascii::FONT_10X20,
    pixelcolor::Rgb565,
    prelude::{Point, RgbColor},
    text::Alignment,
};
use esp_idf_hal::{delay::FreeRtos, prelude::Peripherals};

const ADDRESS: u8 = 0x44;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    let mut m5 = m5_go::M5Go::new(peripherals)?;

    m5.screen.turn_on();
    m5.screen.fill_background(Rgb565::BLACK);

    let mut last_c = 0_f32;
    let mut last_f = 0_f32;
    let mut last_h = 0_f32;

    // Init sensor : will send a message every 0.5 seconds
    m5.port_a
        .write(ADDRESS, &[0x20, 0x32], 100)
        .ok()
        .or_else(|| {
            println!("Write failed");
            None
        });

    loop {
        let mut buffer = [0; 6];

        if m5.port_a.read(ADDRESS, &mut buffer, 100).is_ok() {
            println!("Read: {:?}", buffer);
            let data = buffer
                .to_vec()
                .iter_mut()
                .map(|i| f32::from(*i))
                .collect::<Vec<f32>>();
            let c = ((((data[0] * 256.0) + data[1]) * 175.) / 65535.0) - 45.;
            let f = (c * 1.8) + 32.;
            let h = (((data[3] * 256.0) + data[4]) * 100.) / 65535.0;

            if c != last_c || f != last_f || h != last_h {
                m5.screen.fill_background(Rgb565::BLACK);
                last_c = c;
                last_f = f;
                last_h = h;
                m5.screen.draw_text(
                    format!("Temperature : {:.2}C", c).as_str(),
                    Point::new(m5.screen.driver.width() as i32 / 2, 20),
                    Alignment::Center,
                    Rgb565::WHITE,
                    &FONT_10X20,
                );
                m5.screen.draw_text(
                    format!("Temperature : {:.2}F", f).as_str(),
                    Point::new(m5.screen.driver.width() as i32 / 2, 60),
                    Alignment::Center,
                    Rgb565::WHITE,
                    &FONT_10X20,
                );
                m5.screen.draw_text(
                    format!("Relative Humidity : {:.2}", h).as_str(),
                    Point::new(m5.screen.driver.width() as i32 / 2, 100),
                    Alignment::Center,
                    Rgb565::WHITE,
                    &FONT_10X20,
                );
            }
        } else {
            println!("Read failed");
        }
        if m5.button_a.is_low() {
            println!("send");
            m5.port_a
                .write(ADDRESS, "Hello!".as_bytes(), 100)
                .expect("Write failed");
        }
        FreeRtos::delay_ms(100);
    }
}
