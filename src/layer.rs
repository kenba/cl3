// Copyright (c) 2023-2024 Via Technology Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! `OpenCL` layer extensions

#[cfg(feature = "static_runtime")]
pub use opencl_sys::cl_layer::*;
#[cfg(feature = "static_runtime")]
pub use opencl_sys::*;

#[allow(unused_imports)]
use libc::{c_void, size_t};
#[allow(unused_imports)]
use std::ptr;

/// Query information about the `OpenCL` layer.
/// Calls `clGetLayerInfo`.
pub fn get_layer_data(param_name: cl_layer_info) -> Result<Vec<u8>, cl_int> {
    let mut size: size_t = 0;
    let status = unsafe { cl_call!(clGetLayerInfo(param_name, 0, ptr::null_mut(), &mut size)) };
    if CL_SUCCESS == status {
        let mut data: Vec<u8> = Vec::with_capacity(size);
        let status = unsafe {
            cl_call!(clGetLayerInfo(
                param_name,
                size,
                data.as_mut_ptr().cast::<c_void>(),
                ptr::null_mut(),
            ))
        };
        if CL_SUCCESS == status {
            Ok(data)
        } else {
            Err(status)
        }
    } else {
        Err(status)
    }
}

/// Initialise `OpenCL` layer(s).
/// Calls `clGetLayerInfo`.
///
/// # Safety
///
/// This is unsafe if `target_dispatch` is not valid.
#[allow(clippy::cast_possible_truncation)]
pub unsafe fn init_layer(
    target_dispatch: &[cl_icd_dispatch],
) -> Result<&[cl_icd_dispatch], cl_int> {
    let mut num_entries_ret: cl_uint = 0;
    let mut layer_dispatch_ret: *const cl_icd_dispatch = ptr::null();
    let status = clInitLayer(
        target_dispatch.len() as cl_uint,
        target_dispatch.as_ptr(),
        &mut num_entries_ret,
        &mut layer_dispatch_ret,
    );
    if CL_SUCCESS == status {
        let slice = std::slice::from_raw_parts(layer_dispatch_ret, num_entries_ret as usize);
        Ok(slice)
    } else {
        Err(status)
    }
}
