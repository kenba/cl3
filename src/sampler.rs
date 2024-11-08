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

//! `OpenCL` Sampler API.

#![allow(non_camel_case_types, deprecated)]
#![allow(clippy::not_unsafe_ptr_arg_deref, clippy::wildcard_in_or_patterns)]

use crate::{constants::*, types::*};

use super::info_type::InfoType;
use super::{api_info_size, api_info_value, api_info_vector};
use libc::{c_void, intptr_t, size_t};
use std::mem;
use std::ptr;

/// Create an `OpenCL` buffer `sampler` for a context.
/// Calls `clCreateSampler` to create an `OpenCL` `sampler` object.
/// Deprecated in `CL_VERSION_2_0` by `create_sampler_with_properties`.
///
/// * `context` - a valid `OpenCL` context.
/// * `normalized_coords` - same interpretation as `CL_SAMPLER_NORMALIZED_COORDS`.
/// * `addressing_mode` - same interpretation as `CL_SAMPLER_ADDRESSING_MODE`.
/// * `filter_mode` - same interpretation as  `CL_SAMPLER_FILTER_MODE`.
///
/// `CL_SAMPLER_NORMALIZED_COORDS`, `CL_SAMPLER_ADDRESSING_MODE` and `CL_SAMPLER_FILTER_MODE`
/// are described in: [Sampler Properties](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#sampler-properties-table) table.
/// returns a Result containing the new `OpenCL` `sampler` object
/// or the error code from the `OpenCL` C API function.
#[cfg_attr(
    any(
        feature = "CL_VERSION_2_0",
        feature = "CL_VERSION_2_1",
        feature = "CL_VERSION_2_2",
        feature = "CL_VERSION_3_0"
    ),
    deprecated(
        since = "0.1.0",
        note = "From CL_VERSION_2_0 use create_sampler_with_properties"
    )
)]
#[inline]
pub fn create_sampler(
    context: cl_context,
    normalize_coords: cl_bool,
    addressing_mode: cl_addressing_mode,
    filter_mode: cl_filter_mode,
) -> Result<cl_sampler, cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    let sampler: cl_sampler = unsafe {
        cl_call!(clCreateSampler(
            context,
            normalize_coords,
            addressing_mode,
            filter_mode,
            &mut status,
        ))
    };
    if CL_SUCCESS == status {
        Ok(sampler)
    } else {
        Err(status)
    }
}

/// Create an `OpenCL` buffer sampler for a context.
/// Calls `clCreateSamplerWithProperties` to create an `OpenCL` `sampler` object.
/// `CL_VERSION_2_0`
///
/// * `context` - a valid `OpenCL` context.
/// * `sampler_properties` - an optional null terminated list of properties, see:
/// [Sampler Properties](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#sampler-properties-table).
///
/// returns a Result containing the new `OpenCL` `sampler` object
/// or the error code from the `OpenCL` C API function.
#[cfg(feature = "CL_VERSION_2_0")]
#[inline]
pub fn create_sampler_with_properties(
    context: cl_context,
    properties: *const cl_sampler_properties,
) -> Result<cl_sampler, cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    let sampler: cl_sampler = unsafe {
        cl_call!(clCreateSamplerWithProperties(
            context,
            properties,
            &mut status
        ))
    };
    if CL_SUCCESS == status {
        Ok(sampler)
    } else {
        Err(status)
    }
}

/// Retain an `OpenCL` sampler.
/// Calls `clRetainSampler` to increment the `sampler` reference count.
///
/// * `sampler` - the `OpenCL` sampler.
///
/// returns an empty Result or the error code from the `OpenCL` C API function.
///
/// # Safety
///
/// This function is unsafe because it changes the `OpenCL` object reference count.
#[inline]
pub unsafe fn retain_sampler(sampler: cl_sampler) -> Result<(), cl_int> {
    let status: cl_int = cl_call!(clRetainSampler(sampler));
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}

/// Release an `OpenCL` sampler.
/// Calls `clReleaseMemObject` to decrement the `sampler` reference count.
///
/// * `sampler` - the `OpenCL` sampler.
///
/// returns an empty Result or the error code from the `OpenCL` C API function.
///
/// # Safety
///
/// This function is unsafe because it changes the `OpenCL` object reference count.
#[inline]
pub unsafe fn release_sampler(sampler: cl_sampler) -> Result<(), cl_int> {
    let status: cl_int = cl_call!(clReleaseSampler(sampler));
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}

/// Get data about an `OpenCL` sampler object.
/// Calls `clGetDeviceInfo` to get the desired data about the sampler object.
pub fn get_sampler_data(
    sampler: cl_sampler,
    param_name: cl_sampler_info,
) -> Result<Vec<u8>, cl_int> {
    api_info_size!(get_size, clGetSamplerInfo);
    let size = get_size(sampler, param_name)?;
    api_info_vector!(get_vector, u8, clGetSamplerInfo);
    get_vector(sampler, param_name, size)
}

/// Get information specific to an `OpenCL` sampler object.
/// Calls `clGetImageInfo` to get the desired information about the sampler object.
///
/// * `sampler` - the `OpenCL` sampler object.
/// * `param_name` - the type of sampler information being queried, see:
/// [Sampler Object Queries](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#sampler-info-table).
///
/// returns a Result containing the desired information in an `InfoType` enum
/// or the error code from the `OpenCL` C API function.
pub fn get_sampler_info(
    sampler: cl_sampler,
    param_name: cl_sampler_info,
) -> Result<InfoType, cl_int> {
    match param_name {
        CL_SAMPLER_REFERENCE_COUNT
        | CL_SAMPLER_NORMALIZED_COORDS
        | CL_SAMPLER_ADDRESSING_MODE
        | CL_SAMPLER_FILTER_MODE => {
            api_info_value!(get_value, cl_uint, clGetSamplerInfo);
            Ok(InfoType::Uint(get_value(sampler, param_name)?))
        }

        CL_SAMPLER_CONTEXT => {
            api_info_value!(get_value, intptr_t, clGetSamplerInfo);
            Ok(InfoType::Ptr(get_value(sampler, param_name)?))
        }

        CL_SAMPLER_PROPERTIES // CL_VERSION_3_0
        => {
            api_info_size!(get_size, clGetSamplerInfo);
            api_info_vector!(get_vec, cl_ulong, clGetSamplerInfo);
            let size = get_size(sampler, param_name)?;
            Ok(InfoType::VecUlong(get_vec(sampler, param_name, size)?))
        }

        CL_SAMPLER_MIP_FILTER_MODE
        | CL_SAMPLER_LOD_MIN
        | CL_SAMPLER_LOD_MAX
        | _ =>
        Ok(InfoType::VecUchar(get_sampler_data(sampler, param_name)?))
    }
}
