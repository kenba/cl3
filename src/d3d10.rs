// Copyright (c) 2021-2024 Via Technology Ltd.
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

//! FFI bindings for `cl_d3d10.h`
//! `cl_d3d10.h` contains `OpenCL` extensions that provide interoperability with `Direct3D` 10.
//! `OpenCL` extensions are documented in the [OpenCL-Registry](https://github.com/KhronosGroup/OpenCL-Registry)

#![allow(unused_unsafe)]
#![allow(clippy::missing_safety_doc)]

use crate::{constants::*, types::*};

#[allow(unused_imports)]
use libc::c_void;
#[allow(unused_imports)]
use std::ptr;

#[cfg(feature = "cl_khr_d3d10_sharing")]
pub fn get_supported_d3d10_texture_formats_intel(
    context: cl_context,
    flags: cl_mem_flags,
    image_type: cl_mem_object_type,
) -> Result<Vec<cl_uint>, cl_int> {
    let mut count: cl_uint = 0;
    let status: cl_int = unsafe {
        cl_call!(cl_icd::clGetSupportedD3D10TextureFormatsINTEL(
            context,
            flags,
            image_type,
            0,
            ptr::null_mut(),
            &mut count,
        ))
    };
    if CL_SUCCESS != status {
        Err(status)
    } else if 0 < count {
        // Get the d3d11_formats.
        let len = count as usize;
        let mut ids: Vec<cl_uint> = Vec::with_capacity(len);
        let status: cl_int = unsafe {
            cl_call!(cl_d3d10::clGetSupportedD3D10TextureFormatsINTEL(
                context,
                flags,
                image_type,
                count,
                ids.as_mut_ptr(),
                ptr::null_mut(),
            ))
        };
        if CL_SUCCESS == status {
            Ok(ids)
        } else {
            Err(status)
        }
    } else {
        Ok(Vec::default())
    }
}
