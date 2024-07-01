#[macro_export]
#[allow(unused_macros)]
macro_rules! to_mut {
    ($heap:expr, $obj:expr) => {{
        let tmp = $heap.refs.get($obj);
        $heap.objs.get(tmp)
    }};
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! to_ref {
    ($heap:expr, $obj:expr) => {{
        $heap.refs.get($obj)
    }};
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! to_mut_option {
    ($heap:expr, $obj:expr) => {{
        let tmp = $heap.refs.get_option($obj);
        $heap.objs.get_option(tmp)
    }};
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! to_ref_option {
    ($heap:expr, $obj:expr) => {{
        $heap.refs.get_option($obj)
    }};
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! to_safe_ref {
    ($heap:expr, $obj:expr) => {{
        $heap.refs.get_safe($obj)
    }};
}