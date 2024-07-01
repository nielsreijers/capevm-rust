use crate::heap::{Heap, SafeHeapRef};
use crate::{to_mut, to_ref, to_ref_option, to_safe_ref};

fn prepend_to_list<'a>(heap: &'a mut Heap, list: SafeHeapRef, node: SafeHeapRef) -> SafeHeapRef {
    let mut new_head = to_mut!(heap, &node);
    new_head.set_ref(0, Some(to_ref!(heap, &list)));
    node
}

fn append_to_list(heap: &mut Heap, list: SafeHeapRef, node: SafeHeapRef) -> SafeHeapRef {
    let mut finger = to_mut!(heap, &list);
    while let Some(next) = finger.get_ref(0) {
        finger = to_mut!(heap, &next);
    }
    finger.set_ref(0, Some(to_ref!(heap, &node)));
    list
}

pub fn test_heap() {
    let heap = crate::heap::init_heap();
    let _ = heap.objs.malloc(1, 1).unwrap(); // Create some garbage

    let mut head = heap.objs.malloc(1, 1).unwrap();
    head.set_int(0, 0);
    let mut list = to_safe_ref!(heap, &head);

    for i in 1..5_usize {
        let _garbage = heap.objs.malloc(1, 1).unwrap(); // Create some garbage
        let mut new_node = heap.objs.malloc(1, 1).unwrap();
        new_node.set_int(0, i);
        let new_node = to_safe_ref!(heap, &new_node);

        if i % 2 == 0 {
            list = prepend_to_list(heap, list, new_node);
        } else {
            list = append_to_list(heap, list, new_node);
        }
    }

    heap.gc();

    let mut finger = to_ref_option!(heap, Some(to_ref!(heap, &list)));

    let expected = [4_u16, 2_u16, 0_u16, 1_u16, 3_u16];
    let mut i = 0_u8;
    while let Some(node) = finger {
        let node = to_mut!(heap, &node);
        assert_eq!(node.get_int(0) as u16, expected[i as usize]);
        i += 1;
        finger = to_ref_option!(heap, node.get_ref(0));
    }
}
