use std::{
    cell::RefCell,
    sync::{mpsc::sync_channel, Arc, Mutex, MutexGuard, TryLockError},
};

use esp_idf_ble::{
    AdvertiseData, AttributeValue, AutoResponse, BtUuid, EspBle, GattCharacteristic,
    GattDescriptor, GattService, GattServiceEvent, ServiceUuid,
};
use esp_idf_hal::delay::FreeRtos;
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};

use esp_idf_sys::*;

use log::{info, warn};

pub struct Ble {
    ble: EspBle,
    config: Arc<Mutex<RefCell<BleConfig>>>,
}

#[derive(Default)]
pub struct BleConfig {
    pub on_receive: Option<Box<dyn Fn(&[u8]) -> Option<String> + Send + Sync>>,
    commands: Vec<String>,
}

impl BleConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn on_receive<F>(mut self, f: F) -> Self
    where
        F: Fn(&[u8]) -> Option<String> + Send + Sync + 'static,
    {
        self.on_receive = Some(Box::new(f));
        self
    }

    pub fn send(&mut self, command: String) {
        self.commands.push(command);
    }

    fn next_command(&mut self) -> Option<String> {
        self.commands.pop()
    }
}

impl Ble {
    pub fn new(config: BleConfig) -> Self {
        esp_idf_svc::log::EspLogger::initialize_default();

        #[allow(unused)]
        let sys_loop_stack = Arc::new(EspSystemEventLoop::take().expect("Unable to init sys_loop"));

        #[allow(unused)]
        let default_nvs = Arc::new(EspDefaultNvsPartition::take().unwrap());

        FreeRtos::delay_us(100_u32);

        let mut ble = EspBle::new("ESP32".into(), default_nvs).unwrap();

        let config = Arc::new(Mutex::new(RefCell::new(config)));
        let read_config = Arc::clone(&config);
        let write_config = Arc::clone(&config);

        let (s, r) = sync_channel(1);

        ble.register_gatt_service_application(1, move |gatts_if, reg| {
            if let GattServiceEvent::Register(reg) = reg {
                info!("Service registered with {:?}", reg);
                s.send(gatts_if).expect("Unable to send result");
            } else {
                warn!("What are you doing here??");
            }
        })
        .expect("Unable to register service");

        let svc_uuid = BtUuid::Uuid16(ServiceUuid::Battery as u16);

        let svc = GattService::new_primary(svc_uuid, 4, 1);

        info!("GattService to be created: {:?}", svc);

        let gatts_if = r.recv().expect("Unable to receive value");

        let (s, r) = sync_channel(1);

        ble.register_connect_handler(gatts_if, |_gatts_if, connect| {
            if let GattServiceEvent::Connect(connect) = connect {
                info!("Connect event: {:?}", connect);
            }
        });

        ble.create_service(gatts_if, svc, move |gatts_if, create| {
            if let GattServiceEvent::Create(create) = create {
                info!(
                    "Service created with {{ \tgatts_if: {}\tstatus: {}\n\thandle: {}\n}}",
                    gatts_if, create.status, create.service_handle
                );
                s.send(create.service_handle).expect("Unable to send value");
            }
        })
        .expect("Unable to create service");

        let svc_handle = r.recv().expect("Unable to receive value");

        ble.start_service(svc_handle, |_, start| {
            if let GattServiceEvent::StartComplete(start) = start {
                info!("Service started for handle: {}", start.service_handle);
            }
        })
        .expect("Unable to start ble service");

        let attr_value: AttributeValue<12> = AttributeValue::new_with_value(&[
            0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x57, 0x6F, 0x72, 0x6C, 0x64,
        ]);
        let charac = GattCharacteristic::new(
            BtUuid::Uuid16(0xff01),
            (ESP_GATT_PERM_READ | ESP_GATT_PERM_WRITE) as _,
            (ESP_GATT_CHAR_PROP_BIT_READ | ESP_GATT_CHAR_PROP_BIT_WRITE) as _,
            attr_value,
            AutoResponse::ByApp,
        );

        let (s, r) = sync_channel(1);

        ble.add_characteristic(svc_handle, charac, move |_, add_char| {
            if let GattServiceEvent::AddCharacteristicComplete(add_char) = add_char {
                info!("Attr added with handle: {}", add_char.attr_handle);
                s.send(add_char.attr_handle).expect("Unable to send value");
            }
        })
        .expect("Unable to add characteristic");

        let char_attr_handle = r.recv().expect("Unable to recv attr_handle");

        let data = ble
            .read_attribute_value(char_attr_handle)
            .expect("Unable to read characteristic value");
        info!("Characteristic values: {:?}", data);

        let cdesc = GattDescriptor::new(
            BtUuid::Uuid16(ESP_GATT_UUID_CHAR_CLIENT_CONFIG as u16),
            ESP_GATT_PERM_READ as _,
        );
        ble.add_descriptor(svc_handle, cdesc, |_, add_desc| {
            if let GattServiceEvent::AddDescriptorComplete(add_desc) = add_desc {
                info!("Descriptor added with handle: {}", add_desc.attr_handle);
            }
        })
        .expect("Unable to add characteristic");

        ble.register_read_handler(char_attr_handle, move |gatts_if, read| {
            let val = read_config
                .try_lock()
                .ok()
                .and_then(|config| config.borrow_mut().next_command())
                .unwrap_or("NONE".to_string());

            if let GattServiceEvent::Read(read) = read {
                esp_idf_ble::send(
                    gatts_if,
                    char_attr_handle,
                    read.conn_id,
                    read.trans_id,
                    esp_gatt_status_t_ESP_GATT_OK,
                    &val.as_bytes(),
                )
                .expect("Unable to send read response");
            }
        });

        ble.register_write_handler(char_attr_handle, move |gatts_if, write| {
            if let GattServiceEvent::Write(write) = write {
                if write.is_prep {
                    warn!("Unsupported write");
                } else {
                    let value =
                        unsafe { std::slice::from_raw_parts(write.value, write.len as usize) };
                    let back = write_config.try_lock().ok().and_then(|config| {
                        config
                            .borrow_mut()
                            .on_receive
                            .as_ref()
                            .and_then(|f| f(value))
                    });
                    info!(
                        "Write event received for {} with: {:?}",
                        char_attr_handle,
                        String::from_utf8_lossy(value)
                    );

                    if write.need_rsp {
                        let back = back.unwrap_or(String::new());
                        info!("need rsp");
                        info!("Sending back response: {:?}", back);
                        esp_idf_ble::send(
                            gatts_if,
                            char_attr_handle,
                            write.conn_id,
                            write.trans_id,
                            esp_gatt_status_t_ESP_GATT_OK,
                            &back.as_bytes(),
                        )
                        .expect("Unable to send response");
                    }
                }
            }
        });

        let adv_data = AdvertiseData {
            include_name: true,
            include_txpower: false,
            min_interval: 6,
            max_interval: 16,
            service_uuid: Some(BtUuid::Uuid128([
                0xfb, 0x34, 0x9b, 0x5f, 0x80, 0x00, 0x00, 0x80, 0x00, 0x10, 0x00, 0x00, 0xFF, 0x00,
                0x00, 0x00,
            ])),
            flag: (ESP_BLE_ADV_FLAG_GEN_DISC | ESP_BLE_ADV_FLAG_BREDR_NOT_SPT) as _,
            ..Default::default()
        };
        ble.configure_advertising_data(adv_data, |_| {
            info!("advertising configured");
        })
        .expect("Failed to configure advertising data");

        let scan_rsp_data = AdvertiseData {
            include_name: false,
            include_txpower: true,
            set_scan_rsp: true,
            service_uuid: Some(BtUuid::Uuid128([
                0xfb, 0x34, 0x9b, 0x5f, 0x80, 0x00, 0x00, 0x80, 0x00, 0x10, 0x00, 0x00, 0xFF, 0x00,
                0x00, 0x00,
            ])),
            ..Default::default()
        };

        ble.configure_advertising_data(scan_rsp_data, |_| {
            info!("Advertising configured");
        })
        .expect("Failed to configure advertising data");

        Self { ble, config }
    }

    pub fn start(&self) -> Result<(), EspError> {
        self.ble.start_advertise(|_| {
            info!("advertising started");
        })
    }

    pub fn send(
        &self,
        command: String,
    ) -> Result<(), TryLockError<MutexGuard<'_, RefCell<BleConfig>>>> {
        self.config.try_lock().and_then(|config| {
            config.borrow_mut().send(command);
            Ok(())
        })
    }
}
