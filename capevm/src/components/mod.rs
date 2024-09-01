#[cfg(feature = "jvm")]
pub mod jvm;
#[cfg(feature = "uart")]
pub mod uart;

pub fn init() {
    #[cfg(feature = "jvm")]
    jvm::init();
    #[cfg(feature = "uart")]
    uart::init();
}