// Copyright (c) 2021 Via Technology Ltd. All Rights Reserved.
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

//! FFI bindings for cl_d3d10.h  
//! cl_d3d11.h contains OpenCL extensions that provide interoperability with Direct3D 11.  
//! OpenCL extensions are documented in the [OpenCL-Registry](https://github.com/KhronosGroup/OpenCL-Registry)

#[allow(unused_imports)]
use super::error_codes::{CL_INVALID_VALUE, CL_SUCCESS};
pub use super::ffi::cl_d3d11::*;
#[allow(unused_imports)]
pub use cl_sys::{cl_device_id, cl_platform_id, cl_uint};
#[allow(unused_imports)]
use libc::c_void;
#[allow(unused_imports)]
use std::ptr;

#[cfg(feature = "cl_khr_d3d11_sharing")]
pub fn get_device_ids_from_dx3d11_khr(
    platform: cl_platform_id,
    d3d_device_source: cl_d3d11_device_source_khr,
    d3d_object: *mut c_void,
    d3d_device_set: cl_d3d11_device_set_khr,
) -> Result<Vec<cl_device_id>, cl_int> {
    let mut count: cl_uint = 0;
    let status: cl_int = unsafe {
        clGetDeviceIDsFromD3D11KHR(
            platform,
            d3d_device_source,
            d3d_object,
            d3d_device_set,
            0,
            ptr::null_mut(),
            &mut count,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        if 0 < count {
            // Get the device ids.
            let len = count as usize;
            let mut ids: Vec<cl_device_id> = Vec::with_capacity(len);
            let status: cl_int = unsafe {
                clGetDeviceIDsFromD3D11KHR(
                    platform,
                    d3d_device_source,
                    d3d_object,
                    d3d_device_set,
                    count,
                    ids.as_mut_ptr(),
                    ptr::null_mut(),
                )
            };
            if CL_SUCCESS != status {
                Err(status)
            } else {
                Ok(ids)
            }
        } else {
            Ok(Vec::default())
        }
    }
}

#[cfg(feature = "cl_khr_d3d11_sharing")]
pub fn create_from_d3d11_buffer_khr(
    context: cl_context,
    flags: cl_mem_flags,
    resource: ID3D11Buffer_ptr,
) -> Result<cl_mem, cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    let mem = unsafe { clCreateFromD3D11BufferKHR(context, flags, resource, &mut status) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(mem)
    }
}

#[cfg(feature = "cl_khr_d3d11_sharing")]
pub fn create_from_d3d11_texture2d_khr(
    context: cl_context,
    flags: cl_mem_flags,
    resource: ID3D11Texture2D_ptr,
    subresource: cl_uint,
) -> Result<cl_mem, cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    let mem = unsafe {
        clCreateFromD3D11Texture2DKHR(context, flags, resource, subresource, &mut status)
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(mem)
    }
}

#[cfg(feature = "cl_khr_d3d11_sharing")]
pub fn create_from_d3d11_texture3d_khr(
    context: cl_context,
    flags: cl_mem_flags,
    resource: ID3D11Texture3D_ptr,
    subresource: cl_uint,
) -> Result<cl_mem, cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    let mem = unsafe {
        clCreateFromD3D11Texture3DKHR(context, flags, resource, subresource, &mut status)
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(mem)
    }
}

#[cfg(feature = "cl_khr_d3d11_sharing")]
pub fn enqueue_acquire_dx11_objects_khr(
    command_queue: cl_command_queue,
    num_objects: cl_uint,
    mem_objects: *const cl_mem,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = unsafe {
        clEnqueueAcquireD3D11ObjectsKHR(
            command_queue,
            num_objects,
            mem_objects,
            num_events_in_wait_list,
            event_wait_list,
            &mut event,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(event)
    }
}

#[cfg(feature = "cl_khr_d3d11_sharing")]
pub fn enqueue_release_dx11_objects_khr(
    command_queue: cl_command_queue,
    num_objects: cl_uint,
    mem_objects: *const cl_mem,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = unsafe {
        clEnqueueReleaseD3D11ObjectsKHR(
            command_queue,
            num_objects,
            mem_objects,
            num_events_in_wait_list,
            event_wait_list,
            &mut event,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(event)
    }
}
