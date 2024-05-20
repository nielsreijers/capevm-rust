#![no_std]
#![no_main]

use panic_halt as _;

mod avrora;
mod components;

#[arduino_hal::entry]
fn main() -> ! {
    components::init();
    avrora::print_flash_string!("Done");
    loop {}
}