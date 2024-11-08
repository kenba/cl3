macro_rules! cl_call {
    ($func:ident($($arg:expr),* $(,)?)) => {{
        opencl_sys::$func($($arg),*)
    }}
}
