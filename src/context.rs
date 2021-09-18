// Copyright (c) 2020-2021 Via Technology Ltd. All Rights Reserved.
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

//! OpenCL Context API.

#![allow(non_camel_case_types)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]

pub use cl_sys::{CL_CONTEXT_INTEROP_USER_SYNC, CL_CONTEXT_PLATFORM};

use super::error_codes::{CL_INVALID_VALUE, CL_SUCCESS};
use super::info_type::InfoType;
use super::types::{
    cl_context, cl_context_info, cl_context_properties, cl_device_id, cl_device_type, cl_int,
    cl_uint,
};
use super::{api_info_size, api_info_value, api_info_vector};
use cl_sys::{
    clCreateContext, clCreateContextFromType, clGetContextInfo, clReleaseContext, clRetainContext,
};

use libc::{c_char, c_void, intptr_t, size_t};
use std::mem;
use std::ptr;

// clSetContextDestructorCallback is CL_VERSION_3_0, not in cl_sys yet
#[cfg_attr(not(target_os = "macos"), link(name = "OpenCL"))]
#[cfg_attr(target_os = "macos", link(name = "OpenCL", kind = "framework"))]
extern "system" {
    pub fn clSetContextDestructorCallback(
        context: cl_context,
        pfn_notify: extern "C" fn(cl_context, *const c_void),
        user_data: *mut c_void,
    ) -> cl_int;
}

/// Create an OpenCL context.  
/// Calls clCreateContext to create an OpenCL context.
///
/// * `devices` - a slice of unique devices for an OpenCL platform.
/// * `properties` - a null terminated list of cl_context_properties, see
/// [Context Properties](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#context-properties-table).
/// * `pfn_notify` - an optional callback function that can be registered by the application.
/// * `user_data` - passed as the user_data argument when pfn_notify is called.
///
/// returns a Result containing the new OpenCL context
/// or the error code from the OpenCL C API function.
#[inline]
pub fn create_context(
    devices: &[cl_device_id],
    properties: *const cl_context_properties,
    pfn_notify: Option<extern "C" fn(*const c_char, *const c_void, size_t, *mut c_void)>,
    user_data: *mut c_void,
) -> Result<cl_context, cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    let context = unsafe {
        clCreateContext(
            properties,
            devices.len() as cl_uint,
            devices.as_ptr(),
            pfn_notify,
            user_data,
            &mut status,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(context)
    }
}

/// Create an OpenCL context from a specific device type.  
/// Calls clCreateContextFromType to create an OpenCL context.
///
/// * `device_type` - the type of OpenCL device, see:
/// [Device Types](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#device-types-table).
/// * `properties` - a null terminated list of cl_context_properties, see:
/// [Context Properties](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#context-properties-table).
/// * `pfn_notify` - an optional callback function that can be registered by the application.
/// * `user_data` - passed as the user_data argument when pfn_notify is called.
///
/// returns a Result containing the new OpenCL context
/// or the error code from the OpenCL C API function.
#[inline]
pub fn create_context_from_type(
    device_type: cl_device_type,
    properties: *const cl_context_properties,
    pfn_notify: Option<extern "C" fn(*const c_char, *const c_void, size_t, *mut c_void)>,
    user_data: *mut c_void,
) -> Result<cl_context, cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    let context = unsafe {
        clCreateContextFromType(properties, device_type, pfn_notify, user_data, &mut status)
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(context)
    }
}

/// Retain an OpenCL context.  
/// Calls clRetainContext to increment the context reference count.
///
/// * `context` - the cl_context of the OpenCL context.
///
/// returns an empty Result or the error code from the OpenCL C API function.
#[inline]
pub fn retain_context(context: cl_context) -> Result<(), cl_int> {
    let status: cl_int = unsafe { clRetainContext(context) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(())
    }
}

/// Release an OpenCL context.  
/// Calls clReleaseContext to decrement the context reference count.
///
/// * `context` - the cl_context of the OpenCL context.
///
/// returns an empty Result or the error code from the OpenCL C API function.
#[inline]
pub fn release_context(context: cl_context) -> Result<(), cl_int> {
    let status: cl_int = unsafe { clReleaseContext(context) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(())
    }
}

/// Get data about an OpenCL context.
/// Calls clGetContextInfo to get the desired data about the context.
pub fn get_context_data(
    context: cl_context,
    param_name: cl_context_info,
) -> Result<Vec<u8>, cl_int> {
    api_info_size!(get_size, clGetContextInfo);
    let size = get_size(context, param_name)?;
    api_info_vector!(get_vector, u8, clGetContextInfo);
    get_vector(context, param_name, size)
}

// cl_context_info
#[derive(Clone, Copy, Debug)]
pub enum ContextInfo {
    CL_CONTEXT_REFERENCE_COUNT = 0x1080,
    CL_CONTEXT_DEVICES = 0x1081,
    CL_CONTEXT_PROPERTIES = 0x1082,
    CL_CONTEXT_NUM_DEVICES = 0x1083,
}

/// Get specific information about an OpenCL context.  
/// Calls clGetContextInfo to get the desired information about the context.
///
/// * `context` - the cl_context of the OpenCL context.
/// * `param_name` - the type of platform information being queried, see:
/// [Context Attributes](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#context-info-table).
///
/// returns a Result containing the desired information in an InfoType enum
/// or the error code from the OpenCL C API function.
pub fn get_context_info(context: cl_context, param_name: ContextInfo) -> Result<InfoType, cl_int> {
    api_info_size!(get_size, clGetContextInfo);

    let param_id = param_name as cl_context_info;
    match param_name {
        ContextInfo::CL_CONTEXT_REFERENCE_COUNT | ContextInfo::CL_CONTEXT_NUM_DEVICES => {
            api_info_value!(get_value, cl_uint, clGetContextInfo);
            Ok(InfoType::Uint(get_value(context, param_id)?))
        }

        ContextInfo::CL_CONTEXT_DEVICES | ContextInfo::CL_CONTEXT_PROPERTIES => {
            api_info_vector!(get_vec, intptr_t, clGetContextInfo);
            let size = get_size(context, param_id)?;
            Ok(InfoType::VecIntPtr(get_vec(context, param_id, size)?))
        }
    }
}

// #ifdef CL_VERSION_3_0
/// Register a callback function with a context that is called when the context is destroyed.  
/// Calls clSetContextDestructorCallback.  
/// CL_VERSION_3_0
///
/// * `context` - the cl_context of the OpenCL context.
/// * `pfn_notify` - callback function to be registered by the application.
/// * `user_data` - passed as the user_data argument when pfn_notify is called.
///
/// returns an empty Result or the error code from the OpenCL C API function.
#[cfg(feature = "CL_VERSION_3_0")]
#[inline]
pub fn set_context_destructor_callback(
    context: cl_context,
    pfn_notify: extern "C" fn(cl_context, *const c_void),
    user_data: *mut c_void,
) -> Result<(), cl_int> {
    let status: cl_int = unsafe { clSetContextDestructorCallback(context, pfn_notify, user_data) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(())
    }
}
// #endif

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::{get_device_ids, CL_DEVICE_TYPE_GPU};
    use crate::platform::get_platform_ids;

    #[test]
    fn test_context() {
        let platform_ids = get_platform_ids().unwrap();

        // Choose the first platform
        let platform_id = platform_ids[0];

        let device_ids = get_device_ids(platform_id, CL_DEVICE_TYPE_GPU).unwrap();
        assert!(0 < device_ids.len());

        let context = create_context(&device_ids, ptr::null(), None, ptr::null_mut());
        let context = context.unwrap();

        let value = get_context_info(context, ContextInfo::CL_CONTEXT_REFERENCE_COUNT).unwrap();
        let value = value.to_uint();
        println!("CL_CONTEXT_REFERENCE_COUNT: {}", value);
        assert!(0 < value);

        let value = get_context_info(context, ContextInfo::CL_CONTEXT_DEVICES).unwrap();
        let value = value.to_vec_intptr();
        println!("CL_CONTEXT_DEVICES: {}", value.len());
        println!("CL_CONTEXT_DEVICES: {:?}", value);
        assert!(0 < value.len());

        let value = get_context_info(context, ContextInfo::CL_CONTEXT_PROPERTIES).unwrap();
        let value = value.to_vec_intptr();
        println!("CL_CONTEXT_PROPERTIES: {}", value.len());
        println!("CL_CONTEXT_PROPERTIES: {:?}", value);
        // assert!(0 < value.len());

        let value = get_context_info(context, ContextInfo::CL_CONTEXT_NUM_DEVICES).unwrap();
        let value = value.to_uint();
        println!("CL_CONTEXT_NUM_DEVICES: {}", value);
        assert!(0 < value);

        release_context(context).unwrap();
    }
}
