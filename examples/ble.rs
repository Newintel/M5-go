use esp_idf_hal::{delay::FreeRtos, prelude::Peripherals};
use m5_go::{ble::BleConfig, M5Go};

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    let mut m5 = M5Go::new(peripherals)?;

    let config = BleConfig::new()
        .on_receive(|str| Some(format!("Received: {}", String::from_utf8_lossy(str))));

    m5.setup_ble(config);

    let ble = m5.ble.unwrap();

    ble.start()?;

    println!("mac : {}", m5.mac);

    loop {
        if m5.button_a.is_low() {
            println!("message sent");
            ble.send("Hello from M5Go!".to_string()).unwrap();
        }
        FreeRtos::delay_ms(100);
    }
}
