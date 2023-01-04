use esp_idf_hal::{delay::FreeRtos, prelude::Peripherals};

const ADDRESS: u8 = 0x16;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    let mut m5 = m5_go::M5Go::new(peripherals)?;

    loop {
        let mut buffer = [0; 1];
        if m5.port_a.read(ADDRESS, &mut buffer, 100).is_ok() {
            println!("Read: {:#02x}", buffer[0]);
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
