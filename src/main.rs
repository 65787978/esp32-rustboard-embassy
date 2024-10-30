/*
* Build with: cargo build --release
* Run with: espflash flash ./target/riscv32imc-unknown-none-elf/release/esp-embassy --monitor
*/

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use esp32_rustboard_embassy::*;
use esp_backtrace as _;
use esp_hal::{peripherals::Peripherals, system::RadioClockController};
use esp_hal_embassy::{self, main};
#[main]
async fn main(spawner: Spawner) {
    esp_println::logger::init_logger_from_env();

    let peripherals: Peripherals = esp_hal::init(esp_hal::Config::default());

    let gpio = peripherals.GPIO;
    let io_mux = peripherals.IO_MUX;

    let bluetooth = peripherals.BT;
    let timg0 = peripherals.TIMG0;
    let rng = peripherals.RNG;
    let radio_clk = peripherals.RADIO_CLK;

    spawner.spawn(scan_matrix(gpio, io_mux)).ok();
    spawner
        .spawn(send_report(bluetooth, timg0, rng, radio_clk))
        .ok();
}
