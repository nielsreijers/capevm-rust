use crate::avrora;

pub fn init() {
    avrora::print_flash_string!("uart initialising...");
}
