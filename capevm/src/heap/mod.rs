// Heap         : The main data store.
//
// HeapObj      : Struct through which to access an object's int and ref fields.
//                Obtained by borrowing from &mut Heap.objs so we can only have
//                1 live at any time.
//
// HeapRef      : Opaque reference to an object borrowed from &Heap.refs.
//                We can hold multiple references at the same time, but can't access
//                the referent's fields until it's upgraded to a HeapObj.
//                No HeapObj or HeapRef can be live when GC runs.
//
// SafeHeapRef  : Like a HeapRef, but safe to keep while GC runs. A little more
//                expensive to use since it is an index into a list of references
//                in the heap, instead of a direct reference to the object.

mod iterators;
mod gc;
mod macros;

use core::ptr::addr_of;
use core::ptr::addr_of_mut;
use core::ops::Deref;
use core::cell::RefCell;

use crate::avrora;

const MAX_SAFE_REFS: u8 = 10;
const MEM_SIZE: usize = 3072;

pub struct Heap {
    pub objs: HeapMemory,
    pub refs: HeapRefs,
}

pub struct HeapMemory {
    free_offset: usize,
    bytes:       [u8; MEM_SIZE]
}

type RawObjectPtr = *mut ObjectHeader;

pub struct HeapRefs {
    pub safe_refs: core::cell::RefCell<[Option<RawObjectPtr>; MAX_SAFE_REFS as usize]>
}

pub struct HeapObj<'a> {
    r: &'a mut ObjectHeader
}

pub struct HeapRef<'a> {
    r: &'a ObjectHeader
}

pub struct SafeHeapRef {
    handle_idx: u8
}

pub type HeapInt = usize;

#[derive(Copy, Clone, PartialEq)]
#[repr(usize)]
enum GCColor {
    WHITE = core::usize::MAX,
    GREY = 1,
    BLACK = 2
}

#[derive(Clone, Copy, PartialEq)]
pub struct ObjectHeader {
    gc_color: GCColor,
    gc_shift: usize,
    nr_of_ints: usize,
    nr_of_refs: usize,
}



impl ObjectHeader {
    fn object_size(&self) -> usize {
        core::mem::size_of::<ObjectHeader>()
        + self.nr_of_ints * core::mem::size_of::<HeapRef>()
        + self.nr_of_refs * core::mem::size_of::<HeapInt>()
    }
}



impl HeapMemory {
    pub fn malloc(&mut self, nr_of_ints: usize, nr_of_refs: usize) -> Option<HeapObj> {
        let header = ObjectHeader {
            gc_color: GCColor::WHITE,
            gc_shift: 0,
            nr_of_ints,
            nr_of_refs
        };
        let requested_size = header.object_size();

        if requested_size > MEM_SIZE - self.free_offset {
            None
        } else {
            let obj_addr = (addr_of_mut!(self.bytes) as usize + self.free_offset) as RawObjectPtr;
            self.free_offset += requested_size;

            // Set the chunk header in the heap
            unsafe { *obj_addr = header; }
            let mut obj = unsafe { HeapObj { r: &mut *obj_addr }  };

            // Should this just be some core::mem call to set every byte to 0?
            for i in 0..obj.nr_of_ints {
                obj.set_int(i, 0);
            }
            for i in 0..obj.nr_of_refs {
                obj.set_ref::<HeapRef>(i, None);
            }
            return Some(obj);
        }
    }

    pub fn get_option<'a, 'b, T>(&'a mut self, referent: Option<T>) -> Option<HeapObj<'a>>
            where T: Into<RawObjectPtr> {
        referent.and_then(|c| Some(self.get(c)))
    }


    pub fn get<'a, 'b, T>(&'a mut self, reference: T) -> HeapObj<'a>
            where T: Into<RawObjectPtr> {
        unsafe {
            HeapObj {
                r: &mut *reference.into()
            }
        }
    }
}



impl HeapRefs {
    pub fn get_option<'a, 'b, T>(&'a self, referent: Option<T>) -> Option<HeapRef<'a>>
            where T: Into<RawObjectPtr> {
        referent.and_then(|c| Some(self.get(c)))
    }

    pub fn get<'a, 'b, T>(&'a self, referent: T) -> HeapRef<'a>
            where T: Into<RawObjectPtr> {
        unsafe {
            HeapRef {
                r: & *(referent.into())
            }    
        }
    }

    pub fn get_safe<T>(&self, referent: T) -> SafeHeapRef
            where T: Into<RawObjectPtr> {
        let mut safe_refs = self.safe_refs.borrow_mut();
        for i in 0..MAX_SAFE_REFS {
            if safe_refs[i as usize] == None {
                safe_refs[i as usize] = Some(referent.into());

                return SafeHeapRef {
                    handle_idx: i
                };    
            }
        }
        panic!("Out of safe handles");
    }
}

impl Drop for SafeHeapRef {
    fn drop(&mut self) {
        get_heap().refs.safe_refs.borrow_mut()[self.handle_idx as usize] = None;
    }
}

impl Into<RawObjectPtr> for &ObjectHeader {
    fn into(self) -> RawObjectPtr {
        return self as *const ObjectHeader as RawObjectPtr
    }
}
impl<'a> Into<RawObjectPtr> for HeapObj<'a> {
    fn into(self) -> RawObjectPtr {
        return self.r as *const ObjectHeader as RawObjectPtr
    }
}
impl<'a> Into<RawObjectPtr> for &HeapObj<'a> {
    fn into(self) -> RawObjectPtr {
        return self.r as *const ObjectHeader as RawObjectPtr
    }
}
impl<'a> Into<RawObjectPtr> for HeapRef<'a> {
    fn into(self) -> RawObjectPtr {
        return self.r as *const ObjectHeader as RawObjectPtr
    }
}
impl<'a> Into<RawObjectPtr> for &HeapRef<'a> {
    fn into(self) -> RawObjectPtr {
        return self.r as *const ObjectHeader as RawObjectPtr
    }
}
impl<'a> Into<RawObjectPtr> for &SafeHeapRef {
    fn into(self) -> RawObjectPtr {
        return get_heap().refs.safe_refs.borrow_mut()[self.handle_idx as usize].unwrap();
    }
}

impl<'a> HeapObj<'a> {
    fn get_int_addr(&self, idx: usize) -> *mut HeapInt {
        (addr_of!(*self.r) as usize
        + core::mem::size_of::<ObjectHeader>()
        + core::mem::size_of::<HeapInt>() * idx) as *mut HeapInt
    }

    fn get_ref_addr(&self, idx: usize) -> *mut Option<HeapRef> {
        (self.get_int_addr(self.nr_of_ints) as usize
        + core::mem::size_of::<HeapRef>() * idx) as *mut Option<HeapRef>
    }

    pub fn get_int(&self, idx: usize) -> HeapInt {
        if idx >= self.nr_of_ints {
            panic!("Index out of range");
        }
        let addr = self.get_int_addr(idx);
        let val: HeapInt;
        unsafe {
            val = *addr;
        }
        return val;
    }

    pub fn set_int(&mut self, idx: usize, val: HeapInt) {
        if idx >= self.nr_of_ints {
            panic!("Index out of range");
        }
        let addr = self.get_int_addr(idx);
        unsafe {
            *addr = val;
        }
    }

    // Intentionally tie the result's lifetime to self so
    // gc cannot run as long as the returned HeapRef is alive.
    pub fn get_ref<'s>(&'s self, idx: usize) -> Option<HeapRef<'s>> {
        if idx >= self.nr_of_refs {
            panic!("Index out of range");
        }
        let addr = self.get_ref_addr(idx);
        unsafe {
            let raw_value = *(addr as *const usize);
            return core::mem::transmute(raw_value);
        }
    }

    pub fn set_ref<T>(&mut self, idx: usize, val: Option<T>)
        where T: Into<RawObjectPtr> {
        if idx >= self.nr_of_refs {
            panic!("Index out of range");
        }
        let addr = self.get_ref_addr(idx);
        let rawptr: RawObjectPtr = match val {
            None => core::ptr::null_mut(),
            Some(v) => v.into()
        };
        unsafe {
            *addr = core::mem::transmute(rawptr);
        }
    }
}

impl HeapRef<'_> {
    pub fn avrora_print_address(&self) {
        avrora::print_u16_hex(self.r as *const ObjectHeader as u16);
    }
}

impl<'a> Deref for HeapObj<'a> {
    type Target = ObjectHeader;
    fn deref(&self) -> &Self::Target {
        self.r
    }
}

impl<'a> Deref for HeapRef<'a> {
    type Target = ObjectHeader;
    fn deref(&self) -> &Self::Target {
        self.r
    }
}

static mut HEAP: Heap = Heap {
    objs: HeapMemory {
        free_offset: 0,
        bytes: [ 0_u8; MEM_SIZE ]},
    refs: HeapRefs {
        safe_refs: RefCell::new([None, None, None, None, None, None, None, None, None, None])}
};

fn get_heap() -> &'static mut Heap {
    unsafe { &mut *addr_of_mut!(HEAP) }
}

pub fn init_heap() -> &'static mut Heap {
    unsafe {
        HEAP = Heap {
            objs: HeapMemory {
                free_offset: 0,
                bytes: [ 0_u8; MEM_SIZE ]},
            refs: HeapRefs {
                safe_refs: RefCell::new([None, None, None, None, None, None, None, None, None, None])}
        };
    }

    unsafe { &mut *addr_of_mut!(HEAP) }
}
