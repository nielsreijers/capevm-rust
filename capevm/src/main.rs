#![no_std]
#![no_main]

use panic_halt as _;

mod avrora;

#[arduino_hal::entry]
fn main() -> ! {
    loop {
        avrora::print_all_regs();
        avrora::print_i16(-1024);
        avrora::print_u16(1024);
        avrora::print_u16_hex(1024);
        avrora::print_i32(-1024*1024);
        avrora::print_u32(1024*1024);
        avrora::print_u32_hex(1024*1024);
        avrora::print_all_regs();
        avrora::print_pc();
        avrora::print_sp();
        avrora::print_r1();

        arduino_hal::delay_ms(1000);
    }
}