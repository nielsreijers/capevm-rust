mod heap;

use crate::avrora;

#[allow(unused_macros)]
#[macro_export]
macro_rules! avr_println {
    ($s:expr) => { {
        crate::avrora::print_flash_string!($s);
    } };
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    avrora::print_flash_string!("TEST FAILED!");
    avrora::exit();
}

#[avr_defmt_test::tests(avr_exit=crate::avrora::exit,
                        avr_println=avr_println)]
mod vm_tests {
    #[init]
    fn init() {
        crate::init();
    }

    #[test]
    fn test_heap() {
        super::heap::test_heap();
    }
}