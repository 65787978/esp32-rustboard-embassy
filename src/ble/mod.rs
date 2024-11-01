use bleps::{
    ad_structure::{
        create_advertising_data, AdStructure, BR_EDR_NOT_SUPPORTED, LE_GENERAL_DISCOVERABLE,
    },
    asynch::Ble,
    att::Uuid,
    attribute_server::{AttributeServer, NotificationData, WorkResult},
    gatt, HciConnector,
};
use esp_alloc as _;
use esp_hal::{
    peripherals::{BT, RADIO_CLK, RNG, TIMG0},
    rng::Rng,
    time,
    timer::timg::TimerGroup,
};
use esp_println::println;
use esp_wifi::{ble::controller::asynch::BleConnector, init, EspWifiInitFor};
use user_config::DEVICE_NAME;
mod user_config;
struct Blee {
    report: u8,
}

impl Blee {
    pub async fn new() -> Self {
        Self { report: 1 }
    }
}

#[embassy_executor::task]
pub async fn send_report(bluetooth: BT, timg0: TIMG0, rng: RNG, radio_clk: RADIO_CLK) {
    /* allocate heap memory */
    esp_alloc::heap_allocator!(72 * 1024);

    let mut bluetooth = bluetooth;
    let timg0 = TimerGroup::new(timg0);

    let init = init(EspWifiInitFor::Ble, timg0.timer0, Rng::new(rng), radio_clk).unwrap();

    let connector = BleConnector::new(&init, &mut bluetooth);

    let now = || time::now().duration_since_epoch().to_millis();
    let mut ble = Ble::new(connector, now);

    loop {
        println!("{:?}", ble.init().await);
        println!("{:?}", ble.cmd_set_le_advertising_parameters().await);
        println!(
            "{:?}",
            ble.cmd_set_le_advertising_data(
                create_advertising_data(&[
                    AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
                    AdStructure::ServiceUuids16(&[Uuid::Uuid16(0x1809)]),
                    AdStructure::CompleteLocalName(DEVICE_NAME),
                ])
                .unwrap()
            )
            .await
        );

        println!("{:?}", ble.cmd_set_le_advertise_enable(true).await);

        println!("started advertising");
    }
}
