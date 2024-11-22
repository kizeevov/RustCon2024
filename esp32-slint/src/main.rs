#![no_std]
#![no_main]

extern crate alloc;

mod esp32;
mod instant;

use esp_backtrace as _;
use esp_hal::prelude::*;

slint::include_modules!();

#[entry]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();

    esp32::init();

    let window = MainWindow::new().unwrap();
    let _ = window.run();
    panic!("The event loop should not return");
}
