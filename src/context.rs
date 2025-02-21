// Copyright (c) 2020-2024 Via Technology Ltd.
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

//! `OpenCL` Context API.

#![allow(unused_unsafe)]
#![allow(non_camel_case_types)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]

pub use opencl_sys::{
    CL_CONTEXT_DEVICES, CL_CONTEXT_INTEROP_USER_SYNC, CL_CONTEXT_NUM_DEVICES, CL_CONTEXT_PLATFORM,
    CL_CONTEXT_PROPERTIES, CL_CONTEXT_REFERENCE_COUNT, CL_INVALID_VALUE, CL_SUCCESS, cl_context,
    cl_context_info, cl_context_properties, cl_device_id, cl_device_type, cl_int, cl_uint,
};

use super::info_type::InfoType;
use super::{api_info_size, api_info_value, api_info_vector};
use libc::{c_char, c_void, intptr_t, size_t};
use std::mem;
use std::ptr;

/// Create an `OpenCL` context.
/// Calls clCreateContext to create an `OpenCL` context.
///
/// * `devices` - a slice of unique devices for an `OpenCL` platform.
/// * `properties` - a null terminated list of `cl_context_properties`, see
///   [Context Properties](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#context-properties-table).
/// * `pfn_notify` - an optional callback function that can be registered by the application.
/// * `user_data` - passed as the `user_data` argument when `pfn_notify` is called.
///
/// returns a Result containing the new `OpenCL` context
/// or the error code from the `OpenCL` C API function.
#[allow(unused_unsafe)]
#[allow(clippy::cast_possible_truncation)]
#[inline]
pub fn create_context(
    devices: &[cl_device_id],
    properties: *const cl_context_properties,
    pfn_notify: Option<unsafe extern "C" fn(*const c_char, *const c_void, size_t, *mut c_void)>,
    user_data: *mut c_void,
) -> Result<cl_context, cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    let context = unsafe {
        cl_call!(clCreateContext(
            properties,
            devices.len() as cl_uint,
            devices.as_ptr(),
            pfn_notify,
            user_data,
            &mut status,
        ))
    };
    if CL_SUCCESS == status {
        Ok(context)
    } else {
        Err(status)
    }
}

/// Create an `OpenCL` context from a specific device type.
/// Calls `clCreateContextFromType` to create an `OpenCL` context.
///
/// * `device_type` - the type of `OpenCL` device, see:
///   [Device Types](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#device-types-table).
/// * `properties` - a null terminated list of `cl_context_properties`, see:
///   [Context Properties](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#context-properties-table).
/// * `pfn_notify` - an optional callback function that can be registered by the application.
/// * `user_data` - passed as the `user_data` argument when `pfn_notify` is called.
///
/// returns a Result containing the new `OpenCL` context
/// or the error code from the `OpenCL` C API function.
#[inline]
pub fn create_context_from_type(
    device_type: cl_device_type,
    properties: *const cl_context_properties,
    pfn_notify: Option<unsafe extern "C" fn(*const c_char, *const c_void, size_t, *mut c_void)>,
    user_data: *mut c_void,
) -> Result<cl_context, cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    let context = unsafe {
        cl_call!(clCreateContextFromType(
            properties,
            device_type,
            pfn_notify,
            user_data,
            &mut status
        ))
    };
    if CL_SUCCESS == status {
        Ok(context)
    } else {
        Err(status)
    }
}

/// Retain an `OpenCL` context.
/// Calls clRetainContext to increment the context reference count.
///
/// * `context` - the `cl_context` of the `OpenCL` context.
///
/// returns an empty Result or the error code from the `OpenCL` C API function.
///
/// # Safety
///
/// This function is unsafe because it changes the `OpenCL` object reference count.
#[inline]
pub unsafe fn retain_context(context: cl_context) -> Result<(), cl_int> {
    let status: cl_int = cl_call!(clRetainContext(context));
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}

/// Release an `OpenCL` context.
/// Calls clReleaseContext to decrement the context reference count.
///
/// * `context` - the `cl_context` of the `OpenCL` context.
///
/// returns an empty Result or the error code from the `OpenCL` C API function.
///
/// # Safety
///
/// This function is unsafe because it changes the `OpenCL` object reference count.
#[inline]
pub unsafe fn release_context(context: cl_context) -> Result<(), cl_int> {
    let status: cl_int = cl_call!(clReleaseContext(context));
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}

/// Get data about an `OpenCL` context.
/// Calls `clGetContextInfo` to get the desired data about the context.
pub fn get_context_data(
    context: cl_context,
    param_name: cl_context_info,
) -> Result<Vec<u8>, cl_int> {
    api_info_size!(get_size, clGetContextInfo);
    let size = get_size(context, param_name)?;
    api_info_vector!(get_vector, u8, clGetContextInfo);
    get_vector(context, param_name, size)
}

/// Get specific information about an `OpenCL` context.
/// Calls `clGetContextInfo` to get the desired information about the context.
///
/// * `context` - the `cl_context` of the `OpenCL` context.
/// * `param_name` - the type of platform information being queried, see:
///   [Context Attributes](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#context-info-table).
///
/// returns a Result containing the desired information in an `InfoType` enum
/// or the error code from the `OpenCL` C API function.
pub fn get_context_info(
    context: cl_context,
    param_name: cl_context_info,
) -> Result<InfoType, cl_int> {
    api_info_size!(get_size, clGetContextInfo);

    match param_name {
        CL_CONTEXT_REFERENCE_COUNT | CL_CONTEXT_NUM_DEVICES => {
            api_info_value!(get_value, cl_uint, clGetContextInfo);
            Ok(InfoType::Uint(get_value(context, param_name)?))
        }

        CL_CONTEXT_DEVICES | CL_CONTEXT_PROPERTIES => {
            api_info_vector!(get_vec, intptr_t, clGetContextInfo);
            let size = get_size(context, param_name)?;
            Ok(InfoType::VecIntPtr(get_vec(context, param_name, size)?))
        }

        _ => Ok(InfoType::VecUchar(get_context_data(context, param_name)?)),
    }
}

/// Register a callback function with a context that is called when the `context` is destroyed.
/// Calls `clSetContextDestructorCallback`.
/// `CL_VERSION_3_0`
///
/// * `context` - the `cl_context` of the `OpenCL` context.
/// * `pfn_notify` - callback function to be registered by the application.
/// * `user_data` - passed as the `user_data` argument when `pfn_notify` is called.
///
/// returns an empty Result or the error code from the `OpenCL` C API function.
#[cfg(any(feature = "CL_VERSION_3_0", feature = "dynamic"))]
#[inline]
pub fn set_context_destructor_callback(
    context: cl_context,
    pfn_notify: Option<unsafe extern "C" fn(cl_context, *mut c_void)>,
    user_data: *mut c_void,
) -> Result<(), cl_int> {
    let status: cl_int = unsafe {
        cl_call!(clSetContextDestructorCallback(
            context, pfn_notify, user_data
        ))
    };
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}
// #endif

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::{CL_DEVICE_TYPE_GPU, get_device_ids};
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

        let value = get_context_info(context, CL_CONTEXT_REFERENCE_COUNT).unwrap();
        let value = cl_uint::from(value);
        println!("CL_CONTEXT_REFERENCE_COUNT: {}", value);
        assert!(0 < value);

        let value = get_context_info(context, CL_CONTEXT_DEVICES).unwrap();
        let value = Vec::<intptr_t>::from(value);
        println!("CL_CONTEXT_DEVICES: {}", value.len());
        println!("CL_CONTEXT_DEVICES: {:?}", value);
        assert!(0 < value.len());

        let value = get_context_info(context, CL_CONTEXT_PROPERTIES).unwrap();
        let value = Vec::<intptr_t>::from(value);
        println!("CL_CONTEXT_PROPERTIES: {}", value.len());
        println!("CL_CONTEXT_PROPERTIES: {:?}", value);
        // assert!(0 < value.len());

        let value = get_context_info(context, CL_CONTEXT_NUM_DEVICES).unwrap();
        let value = cl_uint::from(value);
        println!("CL_CONTEXT_NUM_DEVICES: {}", value);
        assert!(0 < value);

        unsafe { release_context(context).unwrap() };
    }
}
