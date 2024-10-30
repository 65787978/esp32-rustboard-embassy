use bleps::{
    ad_structure::{create_advertising_data, AdStructure},
    asynch::Ble,
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
use esp_wifi::{ble::controller::asynch::BleConnector, init, EspWifiInitFor};

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

    loop {}
}
