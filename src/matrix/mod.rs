use embassy_time::Instant;
use esp_hal::{
    gpio::{self, Input, Io, Level, Output, Pull},
    peripherals::{Peripherals, GPIO, IO_MUX},
};

pub mod user_config;
use crate::{delay::delay_us, PRESSED_KEYS};
struct Matrix<'a> {
    rows: [Output<'a>; user_config::ROWS],
    cols: [Input<'a>; user_config::COLS],
    row_count: i8,
    col_count: i8,
}

impl<'a> Matrix<'a> {
    pub async fn new(gpio: GPIO, io_mux: IO_MUX) -> Self {
        let io: Io = Io::new(gpio, io_mux);

        Self {
            rows: [
                Output::new(io.pins.gpio0, Level::Low),
                Output::new(io.pins.gpio1, Level::Low),
                Output::new(io.pins.gpio12, Level::Low),
                Output::new(io.pins.gpio18, Level::Low),
            ],
            cols: [
                Input::new(io.pins.gpio2, Pull::Down),
                Input::new(io.pins.gpio3, Pull::Down),
                Input::new(io.pins.gpio10, Pull::Down),
                Input::new(io.pins.gpio6, Pull::Down),
                Input::new(io.pins.gpio7, Pull::Down),
                Input::new(io.pins.gpio4, Pull::Down),
            ],
            row_count: 0,
            col_count: 0,
        }
    }
}

#[embassy_executor::task]
pub async fn scan_matrix(gpio: GPIO, io_mux: IO_MUX) {
    let mut matrix = Matrix::new(gpio, io_mux).await;

    loop {
        for row in matrix.rows.iter_mut() {
            /* set row to high */
            row.set_high();

            /* delay so pin can propagate */
            delay_us(10).await;

            for col in matrix.cols.iter() {
                /* check if a col is high */
                if col.is_high() {
                    match PRESSED_KEYS.try_lock() {
                        /* lock the hashmap */
                        Ok(mut pressed_keys_lock) => {
                            /* check if the key has been pressed already*/
                            if !pressed_keys_lock
                                .contains_key(&(matrix.row_count, matrix.col_count))
                            {
                                /* store pressed keys */
                                pressed_keys_lock
                                    .insert(
                                        (matrix.row_count, matrix.col_count),
                                        (Instant::now(), false),
                                    )
                                    .unwrap();

                                log::info!("Pressed keys stored!");
                            }
                        }
                        Err(_) => {}
                    }
                }
                /* increment col */
                matrix.col_count += 1;
            }

            /* set row to low */
            row.set_low();

            /* increment row */
            matrix.row_count += 1;

            /* reset col count */
            matrix.col_count = 0;
        }
        /* reset row count */
        matrix.row_count = 0;
    }
}
