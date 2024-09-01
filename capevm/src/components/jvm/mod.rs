#![allow(dead_code)]

use crate::avrora;
use core::ops::Deref;

pub fn init() {
    avrora::print_flash_string!("jvm initialising...");
}

use crate::heap::{Heap, HeapInt, HeapMemory, HeapObj, HeapRef, SafeHeapRef};

pub struct Stackframe<'a> {
    pub heapobj: HeapObj<'a>
}

impl<'a> Stackframe<'a> {
    /// Layout:
    /// Ints:
    ///     - I_IDX_INTSTACK_BASE fixed int slots, followed by
    ///     - I_IDX_MAX_INTSTACK stack slots, growing up
    ///     - I_IDX_NR_INT_LOCAL local slots
    /// Refs: 
    ///     - I_IDX_REFSTACK_BASE fixed ref slots, followed by
    ///     - I_IDX_MAX_REFSTACK stack slots, growing up
    ///     - I_IDX_NR_REF_LOCAL local slots

    const I_IDX_PC:            usize = 0;
    const I_IDX_INT_SP:        usize = 1;
    const I_IDX_REF_SP:        usize = 2;
    const I_IDX_NR_INT_LOCAL:  usize = 3;
    const I_IDX_NR_REF_LOCAL:  usize = 4;
    const I_IDX_MAX_INTSTACK:  usize = 5;
    const I_IDX_MAX_REFSTACK:  usize = 6;
    const I_IDX_INTSTACK_BASE: usize = 6;
    const R_IDX_PARENT_FRAME:  usize = 0;
    const R_IDX_REFSTACK_BASE: usize = 0;

    pub fn create(heapmemory: &mut HeapMemory, local_ints: usize, max_intstack: usize, local_refs: usize, max_refstack: usize) -> Stackframe {
        let nr_of_ints = Stackframe::I_IDX_INTSTACK_BASE + max_intstack + local_ints;
        let nr_of_refs = Stackframe::R_IDX_REFSTACK_BASE + max_refstack + local_refs;

        let mut frame = Stackframe {
            heapobj: heapmemory.malloc(nr_of_ints, nr_of_refs).unwrap()
        };
        frame.heapobj.set_int(Stackframe::I_IDX_PC, 0);
        frame.heapobj.set_int(Stackframe::I_IDX_INT_SP, 0);
        frame.heapobj.set_int(Stackframe::I_IDX_REF_SP, 0);
        frame.heapobj.set_int(Stackframe::I_IDX_NR_INT_LOCAL, local_ints);
        frame.heapobj.set_int(Stackframe::I_IDX_NR_REF_LOCAL, local_refs);
        frame.heapobj.set_int(Stackframe::I_IDX_MAX_INTSTACK, max_intstack);
        frame.heapobj.set_int(Stackframe::I_IDX_MAX_REFSTACK, max_refstack);

        // Initialise locals
        for i in 0..local_ints {
            frame.set_int_local(i, 0);
        }
        for i in 0..local_refs {
            frame.set_ref_local(i, None);
        }
        // And PC
        frame.set_pc(0);

        frame
    }

    pub fn get_parent_frame(&self) -> Option<HeapRef> {
        self.heapobj.get_ref(Stackframe::R_IDX_PARENT_FRAME)
    }

    pub fn set_parent_frame(&mut self, val: HeapRef) {
        self.heapobj.set_ref(Stackframe::R_IDX_PARENT_FRAME, Some(val));
    }

    pub fn get_pc(&self) -> HeapInt {
        self.heapobj.get_int(Stackframe::I_IDX_PC)
    }

    pub fn set_pc(&mut self, val: HeapInt) {
        self.heapobj.set_int(Stackframe::I_IDX_PC, val);
    }

    pub fn get_int_local(&self, idx: usize) -> HeapInt {
        if idx >= self.heapobj.get_int(Stackframe::I_IDX_NR_INT_LOCAL) {
            panic!("Local int idx out of range");
        }

        let idx = idx + Stackframe::I_IDX_INTSTACK_BASE + Stackframe::I_IDX_MAX_INTSTACK;
        self.heapobj.get_int(idx)
    }

    pub fn set_int_local(&mut self, idx: usize, val: HeapInt) {
        if idx >= self.heapobj.get_int(Stackframe::I_IDX_NR_INT_LOCAL) {
            panic!("Local int idx out of range");
        }

        let idx = idx + Stackframe::I_IDX_INTSTACK_BASE + Stackframe::I_IDX_MAX_INTSTACK;
        self.heapobj.set_int(idx, val);
    }

    pub fn get_ref_local(&self, idx: usize) -> Option<HeapRef> {
        if idx >= self.heapobj.get_int(Stackframe::I_IDX_NR_REF_LOCAL) {
            panic!("Local ref idx out of range");
        }

        let idx = idx + Stackframe::R_IDX_REFSTACK_BASE + Stackframe::I_IDX_MAX_REFSTACK;
        self.heapobj.get_ref(idx)
    }

    pub fn set_ref_local(&mut self, idx: usize, val: Option<HeapRef>) {
        if idx >= self.heapobj.get_int(Stackframe::I_IDX_NR_REF_LOCAL) {
            panic!("Local ref idx out of range");
        }

        let idx = idx + Stackframe::R_IDX_REFSTACK_BASE + Stackframe::I_IDX_MAX_REFSTACK;
        self.heapobj.set_ref(idx, val);
    }

    pub fn push_int(&mut self, val: HeapInt) {
        let int_sp = self.heapobj.get_int(Stackframe::I_IDX_INT_SP) as usize;
        let idx = Stackframe::I_IDX_INTSTACK_BASE + int_sp + 1;

        if idx >= self.heapobj.get_int(Stackframe::I_IDX_MAX_INTSTACK) {
            panic!("Int stack overflow");
        }

        self.heapobj.set_int(idx, val);
        self.heapobj.set_int(Stackframe::I_IDX_INT_SP, int_sp + 1);
    }

    pub fn pop_int(&mut self) -> HeapInt {
        let int_sp = self.heapobj.get_int(Stackframe::I_IDX_INT_SP) as usize;
        let idx = Stackframe::I_IDX_INTSTACK_BASE + int_sp;

        if int_sp == 0 {
            panic!("Int stack underflow");
        }

        self.heapobj.set_int(Stackframe::I_IDX_INT_SP, int_sp - 1);
        return self.heapobj.get_int(idx);
    }

    pub fn push_ref(&mut self, val: Option<HeapRef>) {
        let ref_sp = self.heapobj.get_int(Stackframe::I_IDX_REF_SP) as usize;
        let idx = Stackframe::R_IDX_REFSTACK_BASE + ref_sp + 1;

        if idx >= self.heapobj.get_int(Stackframe::I_IDX_MAX_REFSTACK) {
            panic!("Ref stack overflow");
        }

        self.heapobj.set_int(Stackframe::I_IDX_REF_SP, ref_sp + 1);
        self.heapobj.set_ref(idx, val);
    }

    pub fn pop_ref(&mut self) -> Option<HeapRef> {
        let ref_sp = self.heapobj.get_int(Stackframe::I_IDX_REF_SP) as usize;
        let idx = Stackframe::R_IDX_REFSTACK_BASE + ref_sp;

        if ref_sp == 0 {
            panic!("Ref stack underflow");
        }

        self.heapobj.set_int(Stackframe::I_IDX_REF_SP, ref_sp - 1);
        return self.heapobj.get_ref(idx);
    }
}

impl<'a> Deref for Stackframe<'a> {
    type Target = HeapObj<'a>;
    fn deref(&self) -> &Self::Target {
        &self.heapobj    
    }
}

pub struct JVM {
    pub heap: &'static mut Heap,
    pub current_stack_frame: SafeHeapRef
}

impl JVM {
    pub fn create(main_local_ints: usize, main_max_int_stack: usize, main_local_refs: usize, main_max_ref_stack: usize) -> JVM {
        let heap: &'static mut Heap = crate::heap::init_heap();

        let main_frame = Stackframe::create(&mut heap.objs, main_local_ints, main_max_int_stack, main_local_refs, main_max_ref_stack);
        let main_frame_safe_ref = heap.refs.get_safe(main_frame.deref());

        JVM {
            heap: heap,
            current_stack_frame: main_frame_safe_ref
        }
    }
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! get_current_stackframe {
    ($jvm:expr) => { {
        use crate::components::jvm::Stackframe;
        Stackframe{
            heapobj: $jvm.heap.objs.get(&($jvm.current_stack_frame))
        }
    } }
}
