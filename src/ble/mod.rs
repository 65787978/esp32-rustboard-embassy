use bleps::{
    ad_structure::{
        create_advertising_data, AdStructure, BR_EDR_NOT_SUPPORTED, LE_GENERAL_DISCOVERABLE,
    },
    async_attribute_server::AttributeServer,
    asynch::Ble,
    att::Uuid,
    attribute_server::NotificationData,
    gatt, HciConnector,
};
use embassy_futures::select::select;
use embassy_futures::{block_on, join::join};
use esp_alloc as _;
use esp_hal::{
    peripheral::Peripheral,
    peripherals::{BT, RADIO_CLK, RNG, TIMG0},
    rng::Rng,
    time,
    timer::timg::TimerGroup,
};
use esp_println::println;
use esp_wifi::{
    ble::{self, controller::asynch::BleConnector},
    init, EspWifiInitFor,
};

use user_config::DEVICE_NAME;
mod user_config;

struct BleKeyboard<'d> {
    ble: Ble<BleConnector<'d>>,
    report: u8,
}

impl<'d> BleKeyboard<'d> {
    pub async fn init(bluetooth: &'d mut BT, timg0: TIMG0, rng: RNG, radio_clk: RADIO_CLK) -> Self {
        /* allocate heap memory */
        esp_alloc::heap_allocator!(72 * 1024);

        let timg0 = TimerGroup::new(timg0);

        let init = init(EspWifiInitFor::Ble, timg0.timer0, Rng::new(rng), radio_clk).unwrap();

        let connector = BleConnector::new(&init, bluetooth);

        let now = || time::now().duration_since_epoch().to_millis();
        let ble = Ble::new(connector, now);

        Self { ble, report: 1 }
    }

    async fn start_advertising(&mut self) {
        loop {
            self.ble.init().await.unwrap();
            self.ble.cmd_set_le_advertising_parameters().await.unwrap();

            self.ble
                .cmd_set_le_advertising_data(
                    create_advertising_data(&[
                        AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
                        AdStructure::ServiceUuids16(&[Uuid::Uuid16(0x1812)]),
                        AdStructure::CompleteLocalName(DEVICE_NAME),
                    ])
                    .unwrap(),
                )
                .await
                .unwrap();

            self.ble.cmd_set_le_advertise_enable(true).await.unwrap();

            let mut wf = |offset: usize, data: &[u8]| {
                println!("RECEIVED: {} {:?}", offset, data);
            };

            let mut rf = |_offset: usize, data: &mut [u8]| {
                data[..5].copy_from_slice(&b"Hello"[..]);
                5
            };

            gatt!([service {
                uuid: "937312e0-2354-11eb-9f10-fbc30a62cf38",
                characteristics: [characteristic {
                    uuid: "957312e0-2354-11eb-9f10-fbc30a62cf38",
                    read: rf,
                    write: wf,
                },],
            },]);

            let mut rng = bleps::no_rng::NoRng;
            let mut server = AttributeServer::new(&mut self.ble, &mut gatt_attributes, &mut rng);

            let mut notifier = || async {
                let mut data = [0u8; 12];
                data.copy_from_slice(b"notification");

                NotificationData::new(777, &data)
            };
            server.run(&mut notifier).await.unwrap();
        }
    }

    async fn send_report(&mut self) {

        // gatt!()
    }
}

#[embassy_executor::task]
pub async fn send_report(mut bluetooth: BT, timg0: TIMG0, rng: RNG, radio_clk: RADIO_CLK) {
    let mut ble_keyboard = BleKeyboard::init(&mut bluetooth, timg0, rng, radio_clk).await;

    ble_keyboard.start_advertising().await;
    // select(ble_keyboard.start_advertising(), configure_gatt()).await;
}

async fn configure_gatt() {
    // gatt!()
}
