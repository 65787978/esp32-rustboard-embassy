#![no_std]

use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};
use embassy_time::Instant;

use heapless::FnvIndexMap;

pub use ble::send_report;
pub use matrix::scan_matrix;
mod ble;
mod matrix;

const INDEXMAP_MAX_SIZE: usize = 16;

pub static PRESSED_KEYS: Mutex<
    CriticalSectionRawMutex,
    FnvIndexMap<(i8, i8), (Instant, bool), INDEXMAP_MAX_SIZE>,
> = Mutex::new(FnvIndexMap::new());

pub mod delay {
    use embassy_time::{Duration, Timer};

    pub async fn delay_ms(delay: u64) {
        let duration = Duration::from_millis(delay);
        Timer::after(duration).await;
    }

    pub async fn delay_us(delay: u64) {
        let duration = Duration::from_millis(delay);
        Timer::after(duration).await;
    }
}
