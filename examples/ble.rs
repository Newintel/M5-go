use esp_idf_hal::{delay::FreeRtos, prelude::Peripherals};
use m5_go::{BleConfig, M5Go};

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    let mut m5 = M5Go::new(peripherals)?;

    let config = BleConfig::new()
        .on_receive(|str| Some(format!("Received: {}", String::from_utf8_lossy(str))));
    m5.setup_ble(config);

    let ble = m5.ble.unwrap();

    println!("{:#?}", ble.mac);

    ble.start();

    loop {
        FreeRtos::delay_ms(1000);
    }
}
