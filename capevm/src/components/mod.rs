#[cfg(feature = "jvm")]
mod jvm;
#[cfg(feature = "uart")]
mod uart;

pub fn init() {
    #[cfg(feature = "jvm")]
    jvm::init();
    #[cfg(feature = "uart")]
    uart::init();
}