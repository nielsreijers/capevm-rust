use crate::components::jvm::JVM;
use crate::{to_mut, to_ref, get_current_stackframe};

#[inline(never)]
fn exec_sadd(vm: &mut JVM) {
    let mut stackframe = get_current_stackframe!(*vm);

    let a = stackframe.pop_int();
    let b = stackframe.pop_int();
    stackframe.push_int(a + b);
}

#[inline(never)]
fn exec_saload(vm: &mut JVM) {
    let mut stackframe = get_current_stackframe!(*vm);

    let idx = stackframe.pop_int();
    if let Some(array_ref) = stackframe.pop_ref() {
        let array_ref = to_ref!(vm.heap, array_ref);
        let array = to_mut!(vm.heap, array_ref);
        let val = array.get_int(idx as usize);
        get_current_stackframe!(vm).push_int(val);
    } else {
        panic!("TODO: Throw null ref exception here.");
    }
}

pub fn test_sadd() {
    // Arrange
    let mut vm = JVM::create(10, 10, 10, 10);

    let mut stackframe = get_current_stackframe!(vm);
    stackframe.push_int(32);
    stackframe.push_int(10);

    // Act
    exec_sadd(&mut vm);

    // Assert
    let mut stackframe = get_current_stackframe!(vm);
    let val = stackframe.pop_int();
    assert_eq!(val, 42);
}

pub fn test_saload() {
    // Arrange
    let mut vm = JVM::create(10, 10, 10, 10);

    let mut array = vm.heap.objs.malloc(10, 0).unwrap();
    array.set_int(3, 42);
    let array_ref = vm.heap.refs.get(array);

    let mut stackframe = get_current_stackframe!(vm);

    stackframe.push_ref(Some(array_ref));
    stackframe.push_int(3);

    // Act
    exec_saload(&mut vm);

    // Assert
    let mut stackframe = get_current_stackframe!(vm);
    let val = stackframe.pop_int();
    assert_eq!(val, 42);
}