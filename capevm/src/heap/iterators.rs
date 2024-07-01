use core::ptr::addr_of;
use super::{HeapMemory, HeapObj, ObjectHeader};

impl<'a> IntoIterator for &'a mut HeapMemory {
    type IntoIter = HeapIteratorMut<'a>;
    type Item = HeapObj<'a>;
    fn into_iter(self) -> Self::IntoIter {
        HeapIteratorMut {
            heap: self,
            next_offset: 0
        }
    }
}

pub struct HeapIteratorMut<'a> {
    heap: &'a mut HeapMemory,
    next_offset: usize
}

impl<'a> Iterator for HeapIteratorMut<'a> {
    type Item = HeapObj<'a>;
    fn next(& mut self) -> Option<Self::Item> {
        if self.next_offset == self.heap.free_offset {
            None
        } else {
            unsafe {
                let address = addr_of!(self.heap.bytes) as usize + self.next_offset;
                let heapobj = HeapObj { r: &mut *(address as *mut ObjectHeader) };
                self.next_offset += heapobj.object_size() as usize;
                Some(heapobj)
            }
        }
    }
}
