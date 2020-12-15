// Copyright (c) 2020 Via Technology Ltd. All Rights Reserved.
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

//! OpenCL Sampler API.

use super::error_codes::{CL_INVALID_VALUE, CL_SUCCESS};
#[allow(unused_imports)]
use super::ffi::cl::{
    clCreateSampler, clCreateSamplerWithProperties, clGetSamplerInfo, clReleaseSampler,
    clRetainSampler,
};
use super::info_type::InfoType;
use super::types::{
    cl_addressing_mode, cl_bool, cl_context, cl_filter_mode, cl_int, cl_sampler, cl_sampler_info,
    cl_sampler_properties, cl_uint, cl_ulong,
};
use super::{api_info_size, api_info_value, api_info_vector};
use libc::{c_void, intptr_t, size_t};
use std::mem;
use std::ptr;

/// Create an OpenCL buffer sampler for a context.  
/// Calls clCreateSampler to create an OpenCL sampler object.  
/// CL_VERSION_1_2
///
/// * `context` - a valid OpenCL context.
/// * `normalized_coords` - same interpretation as CL_SAMPLER_NORMALIZED_COORDS.
/// * `addressing_mode` - same interpretation as CL_SAMPLER_ADDRESSING_MODE.
/// * `filter_mode` - same interpretation as  CL_SAMPLER_FILTER_MODE.
///
/// CL_SAMPLER_NORMALIZED_COORDS, CL_SAMPLER_ADDRESSING_MODE and CL_SAMPLER_FILTER_MODE
/// are described in: [Sampler Properties](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#sampler-properties-table) table.  
/// returns a Result containing the new OpenCL sampler object
/// or the error code from the OpenCL C API function.
#[cfg(feature = "CL_VERSION_1_2")]
pub fn create_sampler(
    context: cl_context,
    normalize_coords: cl_bool,
    addressing_mode: cl_addressing_mode,
    filter_mode: cl_filter_mode,
) -> Result<cl_sampler, cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    let sampler: cl_sampler = unsafe {
        clCreateSampler(
            context,
            normalize_coords,
            addressing_mode,
            filter_mode,
            &mut status,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(sampler)
    }
}

/// Create an OpenCL buffer sampler for a context.  
/// Calls clCreateSamplerWithProperties to create an OpenCL sampler object.  
/// CL_VERSION_2_0
///
/// * `context` - a valid OpenCL context.
/// * `sampler_properties` - an optional null terminated list of properties, see:
/// [Sampler Properties](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#sampler-properties-table).
///
/// returns a Result containing the new OpenCL sampler object
/// or the error code from the OpenCL C API function.
#[cfg(feature = "CL_VERSION_2_0")]
pub fn create_sampler_with_properties(
    context: cl_context,
    properties: *const cl_sampler_properties,
) -> Result<cl_sampler, cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    let sampler: cl_sampler =
        unsafe { clCreateSamplerWithProperties(context, properties, &mut status) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(sampler)
    }
}

/// Retain an OpenCL sampler.  
/// Calls clRetainSampler to increment the sampler reference count.
///
/// * `sampler` - the OpenCL sampler.
///
/// returns an empty Result or the error code from the OpenCL C API function.
pub fn retain_sampler(sampler: cl_sampler) -> Result<(), cl_int> {
    let status: cl_int = unsafe { clRetainSampler(sampler) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(())
    }
}

/// Release an OpenCL sampler.  
/// Calls clReleaseMemObject to decrement the sampler reference count.
///
/// * `sampler` - the OpenCL sampler.
///
/// returns an empty Result or the error code from the OpenCL C API function.
pub fn release_sampler(sampler: cl_sampler) -> Result<(), cl_int> {
    let status: cl_int = unsafe { clReleaseSampler(sampler) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(())
    }
}

// cl_sampler_info:
pub const CL_SAMPLER_REFERENCE_COUNT: cl_sampler_info = 0x1150;
pub const CL_SAMPLER_CONTEXT: cl_sampler_info = 0x1151;
pub const CL_SAMPLER_NORMALIZED_COORDS: cl_sampler_info = 0x1152;
pub const CL_SAMPLER_ADDRESSING_MODE: cl_sampler_info = 0x1153;
pub const CL_SAMPLER_FILTER_MODE: cl_sampler_info = 0x1154;
// #ifdef CL_VERSION_2_0
// TODO not defined in OpenCL API specs
// pub const CL_SAMPLER_MIP_FILTER_MODE: cl_sampler_info = 0x1155;
// pub const CL_SAMPLER_LOD_MIN: cl_sampler_info = 0x1156;
// pub const CL_SAMPLER_LOD_MAX: cl_sampler_info = 0x1157;
// #endif
// #ifdef CL_VERSION_3_0
pub const CL_SAMPLER_PROPERTIES: cl_sampler_info = 0x1158;
// #endif

/// Get information specific to an OpenCL sampler object.  
/// Calls clGetImageInfo to get the desired information about the sampler object.
///
/// * `sampler` - the OpenCL sampler object.
/// * `param_name` - the type of sampler information being queried, see:
/// [Sampler Object Queries](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#sampler-info-table).
///
/// returns a Result containing the desired information in an InfoType enum
/// or the error code from the OpenCL C API function.
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
            Ok(InfoType::VecUlong(get_vec(sampler, param_name, size,)?))
        }

        _ => Err(CL_INVALID_VALUE),
    }
}
