#![no_std]
#![no_main]

use panic_halt as _;

mod avrora;
mod plugins;

#[arduino_hal::entry]
fn main() -> ! {
    plugins::init();
    avrora::print_flash_string!("Done");
    loop {}
}