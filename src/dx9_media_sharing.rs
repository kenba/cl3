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

//! FFI bindings for `cl_dx9_media_sharing.h`
//! `cl_ecl_dx9_media_sharing.h` contains `OpenCL` extensions that provide interoperability with `Direct3D` 9.
//! `OpenCL` extensions are documented in the [OpenCL-Registry](https://github.com/KhronosGroup/OpenCL-Registry)

#![allow(non_camel_case_types)]
#![allow(clippy::missing_safety_doc)]

pub use crate::constants::cl_dx9_media_sharing::*;
pub use crate::constants::{CL_INVALID_VALUE, CL_SUCCESS};
pub use crate::types::cl_dx9_media_sharing::*;
pub use crate::types::{
    cl_command_queue, cl_context, cl_device_id, cl_event, cl_int, cl_mem, cl_mem_flags,
    cl_mem_object_type, cl_platform_id, cl_uint,
};

#[allow(unused_imports)]
use libc::c_void;
#[allow(unused_imports)]
use std::ptr;

#[cfg(feature = "cl_intel_dx9_media_sharing")]
pub unsafe fn get_device_ids_from_dx9_intel(
    platform: cl_platform_id,
    dx9_device_source: cl_dx9_device_source_intel,
    dx9_object: *mut c_void,
    dx9_device_set: cl_dx9_device_set_intel,
) -> Result<Vec<cl_device_id>, cl_int> {
    let mut count: cl_uint = 0;
    let status: cl_int = cl_call!(cl_dx9_media_sharing::clGetDeviceIDsFromDX9INTEL(
        platform,
        dx9_device_source,
        dx9_object,
        dx9_device_set,
        0,
        ptr::null_mut(),
        &mut count,
    ));
    if CL_SUCCESS != status {
        Err(status)
    } else if 0 < count {
        // Get the device ids.
        let len = count as usize;
        let mut ids: Vec<cl_device_id> = Vec::with_capacity(len);
        let status: cl_int = cl_call!(cl_dx9_media_sharing::clGetDeviceIDsFromDX9INTEL(
            platform,
            dx9_device_source,
            dx9_object,
            dx9_device_set,
            count,
            ids.as_mut_ptr(),
            ptr::null_mut(),
        ));
        if CL_SUCCESS == status {
            Ok(ids)
        } else {
            Err(status)
        }
    } else {
        Ok(Vec::default())
    }
}

#[cfg(feature = "cl_intel_dx9_media_sharing")]
pub unsafe fn create_from_dx9_media_surface_intel(
    context: cl_context,
    flags: cl_mem_flags,
    resource: IDirect3DSurface9_ptr,
    shared_handle: HANDLE,
    plane: cl_uint,
) -> Result<cl_mem, cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    let mem = cl_call!(cl_dx9_media_sharing::clCreateFromDX9MediaSurfaceINTEL(
        context,
        flags,
        resource,
        shared_handle,
        plane,
        &mut status,
    ));
    if CL_SUCCESS == status {
        Ok(mem)
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_intel_dx9_media_sharing")]
pub unsafe fn enqueue_acquire_dx9_objects_intel(
    command_queue: cl_command_queue,
    num_objects: cl_uint,
    mem_objects: *const cl_mem,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = cl_call!(cl_dx9_media_sharing::clEnqueueAcquireDX9ObjectsINTEL(
        command_queue,
        num_objects,
        mem_objects,
        num_events_in_wait_list,
        event_wait_list,
        &mut event,
    ));
    if CL_SUCCESS == status {
        Ok(event)
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_intel_dx9_media_sharing")]
pub unsafe fn enqueue_release_dx9_objects_intel(
    command_queue: cl_command_queue,
    num_objects: cl_uint,
    mem_objects: *const cl_mem,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = cl_call!(cl_dx9_media_sharing::clEnqueueReleaseDX9ObjectsINTEL(
        command_queue,
        num_objects,
        mem_objects,
        num_events_in_wait_list,
        event_wait_list,
        &mut event,
    ));
    if CL_SUCCESS == status {
        Ok(event)
    } else {
        Err(status)
    }
}

#[cfg(any(
    feature = "cl_khr_dx9_media_sharing",
    feature = "cl_intel_dx9_media_sharing"
))]
pub unsafe fn get_supported_dx9_media_surface_formats_intel(
    context: cl_context,
    flags: cl_mem_flags,
    image_type: cl_mem_object_type,
    plane: cl_uint,
) -> Result<Vec<cl_uint>, cl_int> {
    let mut count: cl_uint = 0;
    let status: cl_int = cl_call!(
        cl_dx9_media_sharing::clGetSupportedDX9MediaSurfaceFormatsINTEL(
            context,
            flags,
            image_type,
            plane,
            0,
            ptr::null_mut(),
            &mut count,
        )
    );
    if CL_SUCCESS != status {
        Err(status)
    } else if 0 < count {
        // Get the dx9 formats.
        let len = count as usize;
        let mut ids: Vec<cl_uint> = Vec::with_capacity(len);
        let status: cl_int = cl_call!(
            cl_dx9_media_sharing::clGetSupportedDX9MediaSurfaceFormatsINTEL(
                context,
                flags,
                image_type,
                plane,
                count,
                ids.as_mut_ptr(),
                ptr::null_mut(),
            )
        );
        if CL_SUCCESS == status {
            Ok(ids)
        } else {
            Err(status)
        }
    } else {
        Ok(Vec::default())
    }
}
