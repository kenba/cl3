pub(crate) use opencl_dynamic_sys::constants as OpenClConstants;
pub(crate) use opencl_dynamic_sys::types as OpenClTypes;

pub fn is_opencl_runtime_available() -> bool {
    crate::runtime::load_dynamic_runtime().is_ok()
}

pub fn load_dynamic_runtime() -> Result<&'static opencl_dynamic_sys::OpenClRuntime, i32> {
    const CL_RUNTIME_LOAD_FAILED: i32 = -2000;

    opencl_dynamic_sys::load_library()
        .as_ref()
        .map_err(|_| CL_RUNTIME_LOAD_FAILED)
}

macro_rules! cl_call {
    ($func:ident($($arg:expr),* $(,)?)) => {{
        crate::runtime::load_dynamic_runtime()?.$func($($arg),*)
    }};
    ($namespace:ident::$func:ident($($arg:expr),* $(,)?)) => {{
        crate::runtime::load_dynamic_runtime()?.$func($($arg),*)
    }}
}
