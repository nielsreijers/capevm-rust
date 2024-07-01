use core::ptr::addr_of;
use core::ops::DerefMut;
use super::{GCColor, Heap, HeapRef, ObjectHeader};

impl Heap {
    pub fn gc(&mut self) {
        self.gc_mark_root_set();
        self.gc_mark_recursive();
        self.gc_calculate_shifts();
        self.gc_update_refs();
        self.gc_compact();
    }

    fn gc_mark_root_set(&mut self) {
        for heapobj in &mut self.objs {
            heapobj.r.gc_color = GCColor::WHITE;
        }
        for safe_ref in self.refs.safe_refs.borrow().iter() {
            if let Some(chunkheader) = safe_ref {
                let heapobj = self.objs.get(*chunkheader);
                heapobj.r.gc_color = GCColor::GREY;
            }
        }
    }

    fn gc_mark_recursive(&mut self) {
        let mut done = false;
        while !done {
            done = true;
            for heapobj in &mut self.objs {
                if heapobj.r.gc_color == GCColor::GREY {
                    done = false;
                    heapobj.r.gc_color = GCColor::BLACK;
                    for i in 0..heapobj.nr_of_refs {
                        if let Some(childref) = heapobj.get_ref(i) {
                            let child = unsafe {
                                &mut *(addr_of!(*childref.r) as *mut ObjectHeader)
                            };
                            child.gc_color = GCColor::GREY;
                        }
                    }
                }
            }
        }
    }

    fn gc_calculate_shifts(&mut self) {
        let mut shift = 0_usize;
        for heapobj in &mut self.objs {
            if heapobj.r.gc_color == GCColor::WHITE {
                shift += heapobj.object_size();
            } else {
                heapobj.r.gc_shift = shift;
            }
        }
    }

    fn gc_update_refs(&mut self) {
        // Update the safe reference list 
        for safe_ref in self.refs.safe_refs.borrow_mut().deref_mut() {
            if let Some(safe_ref) = safe_ref {
                unsafe {
                    let new_address = (*safe_ref as usize - (**safe_ref).gc_shift) as *mut ObjectHeader;
                    *safe_ref = new_address; 
                }
            }
        }

        // Update child references within objects
        for mut heapobj in &mut self.objs {
            for i in 0..heapobj.nr_of_refs {
                if let Some(childref) = heapobj.get_ref(i) {
                    let shifted_addr = (childref.r as *const ObjectHeader as usize) - childref.r.gc_shift;
                    let shifted_ref = unsafe{
                        HeapRef {
                            r: &*(shifted_addr as *const ObjectHeader)
                        }
                    };
                    heapobj.set_ref(i, Some(shifted_ref));
                }
            }
        }
    }

    fn gc_compact(&mut self) {
        let mut bytes_saved: usize = 0;
        let mut offset = 0;

        while offset < self.objs.free_offset {
            unsafe {
                let header = &mut *((addr_of!(self.objs.bytes) as usize + offset) as *mut ObjectHeader);
                let object_size = header.object_size();
                if header.gc_color == GCColor::WHITE {
                    bytes_saved += header.object_size();
                } else {
                    let src = header as *mut ObjectHeader as *mut u8;
                    let dst = src.sub((*header).gc_shift);
                    core::ptr::copy(src, dst, object_size);
                }
                offset += object_size;
            }
        }
        self.objs.free_offset -= bytes_saved;
    }
}