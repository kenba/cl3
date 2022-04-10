// Copyright (c) 2020-2022 Via Technology Ltd. All Rights Reserved.
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

//! OpenCL Platform API.

#![allow(non_camel_case_types)]
#![allow(clippy::wildcard_in_or_patterns)]

pub use cl_sys::{
    CL_PLATFORM_EXTENSIONS, CL_PLATFORM_HOST_TIMER_RESOLUTION, CL_PLATFORM_NAME,
    CL_PLATFORM_PROFILE, CL_PLATFORM_VENDOR, CL_PLATFORM_VERSION,
};

use super::ffi::cl_ext::{
    CL_PLATFORM_EXTERNAL_MEMORY_IMPORT_HANDLE_TYPES_KHR,
    CL_PLATFORM_SEMAPHORE_EXPORT_HANDLE_TYPES_KHR, CL_PLATFORM_SEMAPHORE_IMPORT_HANDLE_TYPES_KHR,
    CL_PLATFORM_SEMAPHORE_TYPES_KHR,
};

use super::error_codes::CL_SUCCESS;
use super::info_type::InfoType;
use super::types::{cl_int, cl_name_version, cl_platform_id, cl_platform_info, cl_uint, cl_ulong};
use super::{api_info_size, api_info_value, api_info_vector};
use cl_sys::{clGetPlatformIDs, clGetPlatformInfo};

use libc::{c_void, size_t};
use std::mem;
use std::ptr;

// cl_platform_info constants missing in cl_sys
pub const CL_PLATFORM_NUMERIC_VERSION: cl_platform_info = 0x0906;
pub const CL_PLATFORM_EXTENSIONS_WITH_VERSION: cl_platform_info = 0x0907;

/// Get the available platforms.  
/// Calls clGetPlatformIDs to get the available platform ids.
///  # Examples
/// ```
/// use cl3::platform::get_platform_ids;
///
/// let platform_ids = get_platform_ids().unwrap();
/// println!("Number of OpenCL platforms: {}", platform_ids.len());
/// assert!(0 < platform_ids.len());
/// ```
/// returns a Result containing a vector of available platform ids
/// or the error code from the OpenCL C API function.
pub fn get_platform_ids() -> Result<Vec<cl_platform_id>, cl_int> {
    // Get the number of platforms
    let mut count: cl_uint = 0;
    let mut status = unsafe { clGetPlatformIDs(0, ptr::null_mut(), &mut count) };

    if CL_SUCCESS != status {
        Err(status)
    } else if 0 < count {
        // Get the platform ids.
        let len = count as usize;
        let mut ids: Vec<cl_platform_id> = Vec::with_capacity(len);
        unsafe {
            status = clGetPlatformIDs(count, ids.as_mut_ptr(), ptr::null_mut());
            ids.set_len(len);
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

/// Get data about an OpenCL platform.
/// Calls clGetPlatformInfo to get the desired data about the platform.
pub fn get_platform_data(
    platform: cl_platform_id,
    param_name: cl_platform_info,
) -> Result<Vec<u8>, cl_int> {
    api_info_size!(get_size, clGetPlatformInfo);
    let size = get_size(platform, param_name)?;
    api_info_vector!(get_vector, u8, clGetPlatformInfo);
    get_vector(platform, param_name, size)
}

/// Get specific information about an OpenCL platform.
/// Calls clGetPlatformInfo to get the desired information about the platform.
///  # Examples
/// ```
/// use cl3::platform::{get_platform_ids, get_platform_info, CL_PLATFORM_NAME, CL_PLATFORM_VERSION};
///
/// let platform_ids = get_platform_ids().unwrap();
/// assert!(0 < platform_ids.len());
///
/// // Choose a the first platform
/// let platform_id = platform_ids[0];
///
/// let value = get_platform_info(platform_id, CL_PLATFORM_NAME).unwrap();
/// let value: String = value.into();
/// println!("CL_PLATFORM_NAME: {}", value);
/// assert!(!value.is_empty());
///
/// let value = get_platform_info(platform_id, CL_PLATFORM_VERSION).unwrap();
/// let value = String::from(value);
/// println!("CL_PLATFORM_VERSION: {}", value);
/// assert!(!value.is_empty());
/// ```
/// * `platform` - the cl_platform_id of the OpenCL platform.
/// * `param_name` - the type of platform information being queried, see
/// [Platform Queries](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#platform-queries-table).
///
/// returns a Result containing the desired information in an InfoType enum
/// or the error code from the OpenCL C API function.
pub fn get_platform_info(
    platform: cl_platform_id,
    param_name: cl_platform_info,
) -> Result<InfoType, cl_int> {
    match param_name {
        // CL_VERSION_3_0
        CL_PLATFORM_NUMERIC_VERSION => {
            api_info_value!(get_value, cl_uint, clGetPlatformInfo);
            Ok(InfoType::Uint(get_value(platform, param_name)?))
        }

        // CL_VERSION_2_1
        CL_PLATFORM_HOST_TIMER_RESOLUTION => {
            api_info_value!(get_value, cl_ulong, clGetPlatformInfo);
            Ok(InfoType::Ulong(get_value(platform, param_name)?))
        }

        // CL_VERSION_3_0
        CL_PLATFORM_EXTENSIONS_WITH_VERSION => {
            api_info_size!(get_size, clGetPlatformInfo);
            let size = get_size(platform, param_name)?;
            api_info_vector!(get_vec, cl_name_version, clGetPlatformInfo);
            Ok(InfoType::VecNameVersion(get_vec(
                platform, param_name, size,
            )?))
        }

        CL_PLATFORM_EXTERNAL_MEMORY_IMPORT_HANDLE_TYPES_KHR // cl_khr_external_memory
        | CL_PLATFORM_SEMAPHORE_IMPORT_HANDLE_TYPES_KHR // cl_khr_external_semaphore
        | CL_PLATFORM_SEMAPHORE_EXPORT_HANDLE_TYPES_KHR // cl_khr_external_semaphore
        | CL_PLATFORM_SEMAPHORE_TYPES_KHR // cl_khr_semaphore
        => {
            api_info_size!(get_size, clGetPlatformInfo);
            api_info_vector!(get_vec, cl_uint, clGetPlatformInfo);
            let size = get_size(platform, param_name)?;
            Ok(InfoType::VecUshort(get_vec(platform, param_name, size)?))
        }

        CL_PLATFORM_PROFILE
        | CL_PLATFORM_VERSION
        | CL_PLATFORM_NAME
        | CL_PLATFORM_VENDOR
        | CL_PLATFORM_EXTENSIONS
        | _ => Ok(InfoType::VecUchar(get_platform_data(platform, param_name)?)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error_codes::error_text;

    #[test]
    fn test_get_platform_info() {
        let platform_ids = get_platform_ids().unwrap();
        println!("Number of platforms: {}", platform_ids.len());
        assert!(0 < platform_ids.len());

        // Choose the first platform
        let platform_id = platform_ids[0];

        let value = get_platform_info(platform_id, CL_PLATFORM_PROFILE).unwrap();
        let value: String = value.into();
        println!("CL_PLATFORM_PROFILE: {}", value);
        assert!(!value.is_empty());

        let value = get_platform_info(platform_id, CL_PLATFORM_VERSION).unwrap();
        let value: String = value.into();
        println!("CL_PLATFORM_VERSION: {}", value);
        assert!(!value.is_empty());

        let value = get_platform_info(platform_id, CL_PLATFORM_NAME).unwrap();
        let value: String = value.into();
        println!("CL_PLATFORM_NAME: {}", value);
        assert!(!value.is_empty());

        let value = get_platform_info(platform_id, CL_PLATFORM_VENDOR).unwrap();
        let value: String = value.into();
        println!("CL_PLATFORM_VENDOR: {}", value);
        assert!(!value.is_empty());

        let value = get_platform_info(platform_id, CL_PLATFORM_EXTENSIONS).unwrap();
        let value: String = value.into();
        println!("CL_PLATFORM_EXTENSIONS: {}", value);
        assert!(!value.is_empty());

        // CL_VERSION_2_1 value, may not be supported
        match get_platform_info(platform_id, CL_PLATFORM_HOST_TIMER_RESOLUTION) {
            Ok(value) => {
                let value = cl_ulong::from(value);
                println!("CL_PLATFORM_HOST_TIMER_RESOLUTION: {}", value)
            }
            Err(e) => println!(
                "OpenCL error, CL_PLATFORM_HOST_TIMER_RESOLUTION: {}",
                error_text(e)
            ),
        };
    }

    #[test]
    fn test_get_platform_info_3_0() {
        let platform_ids = get_platform_ids().unwrap();

        // Choose the first platform
        let platform_id = platform_ids[0];

        let value = get_platform_info(platform_id, CL_PLATFORM_VERSION).unwrap();
        let value: String = value.into();
        println!("CL_PLATFORM_VERSION: {}", value);
        assert!(!value.is_empty());

        let opencl_3: &str = "OpenCL 3";
        let is_opencl_3: bool = value.contains(opencl_3);

        if is_opencl_3 {
            let value = get_platform_info(platform_id, CL_PLATFORM_NUMERIC_VERSION).unwrap();
            let value = cl_uint::from(value);
            println!("CL_PLATFORM_NUMERIC_VERSION: {}", value);
            assert!(0 < value);

            let value =
                get_platform_info(platform_id, CL_PLATFORM_EXTENSIONS_WITH_VERSION).unwrap();
            println!("CL_PLATFORM_EXTENSIONS_WITH_VERSION: {}", value);

            let value = Vec::<cl_name_version>::from(value);
            println!("CL_PLATFORM_EXTENSIONS_WITH_VERSION count: {}", value.len());
            assert!(0 < value.len());
        }
    }
}
