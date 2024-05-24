use crate::avrora;

pub fn init() {
    avrora::print_flash_string!("jvm initialising...");
}

inventory::submit! {
    super::Component{ init }
}
