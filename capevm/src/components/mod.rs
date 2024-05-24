#[cfg(feature = "jvm")]
mod jvm;
#[cfg(feature = "uart")]
mod uart;

pub struct Component {
    init: fn()
}

inventory::collect!(Component);

pub fn init() {
    for component in inventory::iter::<Component> {
        (component.init)();
    }
}
