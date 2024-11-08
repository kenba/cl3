pub fn load_runtime() -> Result<&'static opencl_dynamic_sys::OpenClRuntime, i32> {
    const CL_RUNTIME_LOAD_FAILED: i32 = -2000;

    opencl_dynamic_sys::load_library().as_ref().map_err(|_| CL_RUNTIME_LOAD_FAILED)
}
