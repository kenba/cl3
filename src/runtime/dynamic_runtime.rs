pub(crate) use opencl_dynamic_sys::constants as OpenClConstants;
pub(crate) use opencl_dynamic_sys::types as OpenClTypes;

pub fn is_opencl_runtime_available() -> bool {
    load_dynamic_runtime().is_ok()
}

pub fn load_dynamic_runtime() -> Result<&'static opencl_dynamic_sys::OpenClRuntime, i32> {
    opencl_dynamic_sys::load_library()
        .as_ref()
        .map_err(|_| opencl_dynamic_sys::constants::CL_RUNTIME_LOAD_FAILED)
}

macro_rules! cl_call {
    ($func:ident($($arg:expr),* $(,)?)) => {{
        if let Some(result) = crate::runtime::load_dynamic_runtime()?.$func($($arg),*) {
            result
        } else {
            return Err(opencl_dynamic_sys::constants::CL_FUNCTION_NOT_AVAILABLE)
        }
    }};
    ($namespace:ident::$func:ident($($arg:expr),* $(,)?)) => {{
        cl_call!($func($($arg),*))
    }}
}
