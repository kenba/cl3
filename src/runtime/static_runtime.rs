pub(crate) use opencl_sys as OpenClTypes;
pub(crate) use opencl_sys as OpenClConstants;

macro_rules! cl_call {
    ($func:ident($($arg:expr),* $(,)?)) => {{
        opencl_sys::$func($($arg),*)
    }};
    ($namespace:ident::$func:ident($($arg:expr),* $(,)?)) => {{
        opencl_sys::$namespace::$func($($arg),*)
    }}
}
