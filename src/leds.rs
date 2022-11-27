use esp_idf_sys::gpio_num_t;
use smart_leds::RGB8;
use ws2812_esp32_rmt_driver::Ws2812Esp32RmtDriver;

/// A driver for the side led bars
pub struct Leds {
    driver: Ws2812Esp32RmtDriver,
    lights: Vec<RGB8>,
}

impl Leds {
    pub fn new(gpio_num: gpio_num_t) -> Self {
        let driver = Ws2812Esp32RmtDriver::new(0, 15)
            .expect(format!("Error creating leds driver from pin {gpio_num}").as_str());
        let lights = vec![RGB8::default(); 10];

        Self { driver, lights }
    }

    /// Light the lights up
    pub fn display(&mut self) {
        for color in &self.lights {
            self.driver.write(color.as_ref()).unwrap();
        }
    }

    pub fn set_color_at_index(&mut self, index: usize, color: RGB8) {
        self.lights.remove(index);
        self.lights.insert(index, color);
    }

    pub fn clear(&mut self) {
        self.lights = std::iter::repeat(RGB8::default()).take(10).collect();
    }

    pub fn fill(&mut self, color: RGB8) {
        self.lights = vec![color; 10]
    }
}
