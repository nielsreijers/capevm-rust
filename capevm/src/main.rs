#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]

mod avrora;
mod components;
mod heap;
#[cfg(test)]
mod tests;

#[cfg(not(test))]
#[arduino_hal::entry]
fn main() -> ! {
    init();
    avrora::print_flash_string!("Done");
    avrora::exit();
}

fn init() {
    components::init();
}
