use display_interface_spi::SPIInterfaceNoCS;
use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::{DrawTarget, Point},
    text::{Alignment, Text},
    Drawable,
};
use esp_idf_hal::{
    delay::FreeRtos,
    gpio::{Gpio8, Output, OutputPin, PinDriver},
    spi::{SpiConfig, SpiDeviceDriver, SpiDriver, SPI2},
    units::Hertz,
};
use ili9341::{DisplaySize240x320, Ili9341};

pub struct Screen<'a, DC: OutputPin, RST: OutputPin, BL: OutputPin> {
    pub driver: Ili9341<
        SPIInterfaceNoCS<SpiDeviceDriver<'a, SpiDriver<'a>>, PinDriver<'a, DC, Output>>,
        PinDriver<'a, RST, Output>,
    >,
    bl: PinDriver<'a, BL, Output>,
}

impl<'a, DC: OutputPin, RST: OutputPin, BL: OutputPin> Screen<'a, DC, RST, BL> {
    pub fn new<CS: OutputPin, SDO: OutputPin, SCLK: OutputPin>(
        cs: CS,
        sdo: SDO,
        sclk: SCLK,
        dc: DC,
        rst: RST,
        blk: BL,
        spi2: SPI2,
    ) -> Self {
        let spi_config = SpiConfig::new().baudrate(Hertz(10 * 1000 * 1000));

        let lcd_spi_master = SpiDeviceDriver::new_single(
            spi2,
            sclk,
            sdo,
            // Needed a pin that implements esp_idf_hal::gpio::InputPin
            // Otherwise, there is a bound required error
            None as Option<Gpio8>,
            esp_idf_hal::spi::Dma::Disabled,
            Some(cs),
            &spi_config,
        )
        .expect("Creating screen spi master failed");

        let spi_display_interface =
            SPIInterfaceNoCS::new(lcd_spi_master, PinDriver::output(dc).unwrap());

        // Orientation Landscape for a 320 * 240 screen
        let mut lcd = Ili9341::new(
            spi_display_interface,
            PinDriver::output(rst).unwrap(),
            &mut FreeRtos,
            ili9341::Orientation::Landscape,
            DisplaySize240x320,
        )
        .expect("Failed to initialize LCD ILI9341.");

        lcd.command(ili9341::Command::DisplayInvertionOn, &[])
            .expect("Failed to issue Display Invertion ON command");
        lcd.command(ili9341::Command::MemoryAccessControl, &[0x00 | 0x08])
            .expect("Failed to issue MemoryAccessControl command");

        Self {
            driver: lcd,
            bl: PinDriver::output(blk).unwrap(),
        }
    }

    pub fn is_on(&self) -> bool {
        self.bl.is_set_high()
    }

    pub fn turn_on(&mut self) {
        self.bl.set_high().unwrap();
    }

    pub fn turn_off(&mut self) {
        self.bl.set_low().unwrap();
    }

    pub fn fill_background(&mut self, color: Rgb565) {
        self.driver.clear(color).expect("Failed setting background")
    }

    pub fn draw_text(
        &mut self,
        text: &str,
        position: Point,
        alignment: Alignment,
        color: Rgb565,
    ) -> Point {
        let character_style = MonoTextStyle::new(&FONT_10X20, color);

        let text_drawable = Text::with_alignment(text, position, character_style, alignment);

        text_drawable
            .draw(&mut self.driver)
            .expect(format!("Draw text '{text}' in position {position} failed").as_str())
    }
}
