/*
* Build with: cargo build --release
* Run with: espflash flash ./target/riscv32imc-unknown-none-elf/release/esp-embassy --monitor
*/

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, system::SystemControl};
use esp_hal_embassy::{self};

#[main]
async fn main(spawner: Spawner) {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);

    let clocks = ClockControl::max(system.clock_control).freeze();
    // let delay = Delay::new(&clocks);

    let timg0 = esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG0, &clocks);

    esp_hal_embassy::init(&clocks, timg0.timer0);
    esp_println::logger::init_logger_from_env();

    // let _init = esp_wifi::initialize(
    //     esp_wifi::EspWifiInitFor::Wifi,
    //     timg0.timer0,
    //     esp_hal::rng::Rng::new(peripherals.RNG),
    //     peripherals.RADIO_CLK,
    //     &clocks,
    // )
    // .unwrap();

    spawner.spawn(task_1()).ok();
    spawner.spawn(task_2()).ok();
}

#[embassy_executor::task]
async fn task_1() {
    loop {
        log::info!("Hello from an embassy thread no. 1");
        Timer::after(Duration::from_millis(2_000)).await;
    }
}

#[embassy_executor::task]
async fn task_2() {
    loop {
        log::info!("Hello from an embassy thread no. 2");
        Timer::after(Duration::from_millis(500)).await;
    }
}
