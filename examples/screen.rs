use embedded_graphics::mono_font::ascii::FONT_10X20;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::{
    prelude::{Point, RgbColor},
    text::Alignment,
};
use esp_idf_hal::prelude::Peripherals;

fn main() {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    let mut m5 = m5_go::M5Go::new(peripherals).unwrap();

    m5.screen.turn_on();

    m5.screen.fill_background(Rgb565::GREEN);

    let font = FONT_10X20;

    let next_position = m5.screen.draw_text(
        "I am text",
        Point::new(0, 15),
        Alignment::Left,
        Rgb565::BLACK,
        &font,
    );

    m5.screen.draw_text(
        " and i come after",
        next_position,
        Alignment::Left,
        Rgb565::BLACK,
        &font,
    );
}
