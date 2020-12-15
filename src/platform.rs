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

//! OpenCL Platform API.

use super::error_codes::{CL_INVALID_VALUE, CL_SUCCESS};
use super::ffi::cl::{clGetPlatformIDs, clGetPlatformInfo};
use super::info_type::InfoType;
use super::types::{cl_int, cl_name_version, cl_platform_id, cl_platform_info, cl_uint, cl_ulong};
use super::{api_info_size, api_info_value, api_info_vector};

use libc::{c_void, size_t};
use std::mem;
use std::ptr;

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
    } else {
        if 0 < count {
            // Get the platform ids.
            let len = count as usize;
            let mut ids: Vec<cl_platform_id> = Vec::with_capacity(len);
            unsafe {
                ids.set_len(len);
                status = clGetPlatformIDs(count, ids.as_mut_ptr(), ptr::null_mut());
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

// cl_platform_info
pub const CL_PLATFORM_PROFILE: cl_platform_info = 0x0900;
pub const CL_PLATFORM_VERSION: cl_platform_info = 0x0901;
pub const CL_PLATFORM_NAME: cl_platform_info = 0x0902;
pub const CL_PLATFORM_VENDOR: cl_platform_info = 0x0903;
pub const CL_PLATFORM_EXTENSIONS: cl_platform_info = 0x0904;
/// CL_VERSION_2_1
pub const CL_PLATFORM_HOST_TIMER_RESOLUTION: cl_platform_info = 0x0905;
/// CL_VERSION_3_0
pub const CL_PLATFORM_NUMERIC_VERSION: cl_platform_info = 0x0906;
/// CL_VERSION_3_0
pub const CL_PLATFORM_EXTENSIONS_WITH_VERSION: cl_platform_info = 0x0907;

/// Get specific information about an OpenCL platform.
/// Calls clGetPlatformInfo to get the desired information about the platform.
///  # Examples
/// ```
/// use cl3::platform::{
///   get_platform_ids, get_platform_info, CL_PLATFORM_NAME, CL_PLATFORM_VERSION,
/// };
///
/// let platform_ids = get_platform_ids().unwrap();
/// assert!(0 < platform_ids.len());
///
/// // Choose a the first platform
/// let platform_id = platform_ids[0];
///
/// let value = get_platform_info(platform_id, CL_PLATFORM_NAME).unwrap();
/// let value = value.to_str().unwrap().into_string().unwrap();
/// println!("CL_PLATFORM_NAME: {}", value);
///
/// assert!(0 < value.len());
///
/// let value = get_platform_info(platform_id, CL_PLATFORM_VERSION).unwrap();
/// let value = value.to_str().unwrap().into_string().unwrap();
/// println!("CL_PLATFORM_VERSION: {}", value);
/// assert!(0 < value.len());
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
    api_info_size!(get_size, clGetPlatformInfo);

    match param_name {
        CL_PLATFORM_PROFILE
        | CL_PLATFORM_VERSION
        | CL_PLATFORM_NAME
        | CL_PLATFORM_VENDOR
        | CL_PLATFORM_EXTENSIONS => {
            api_info_vector!(get_string, u8, clGetPlatformInfo);
            let size = get_size(platform, param_name)?;
            Ok(InfoType::Str(get_string(platform, param_name, size)?))
        }
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
            api_info_vector!(get_vec, cl_name_version, clGetPlatformInfo);
            let size = get_size(platform, param_name)?;
            Ok(InfoType::VecNameVersion(get_vec(
                platform, param_name, size,
            )?))
        }
        _ => Err(CL_INVALID_VALUE),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_platform_info() {
        let platform_ids = get_platform_ids().unwrap();
        println!("Number of platforms: {}", platform_ids.len());
        assert!(0 < platform_ids.len());

        // Choose a platform which is CL_VERSION_2_1 compliant
        let platform_id = platform_ids[1];

        let value = get_platform_info(platform_id, CL_PLATFORM_PROFILE).unwrap();
        let value = value.to_str().unwrap();
        println!("CL_PLATFORM_PROFILE: {:?}", value);
        let value = value.into_string().unwrap();
        assert!(0 < value.len());

        let value = get_platform_info(platform_id, CL_PLATFORM_VERSION).unwrap();
        let value = value.to_str().unwrap();
        println!("CL_PLATFORM_VERSION: {:?}", value);
        let value = value.into_string().unwrap();
        assert!(0 < value.len());

        let value = get_platform_info(platform_id, CL_PLATFORM_NAME).unwrap();
        let value = value.to_str().unwrap();
        println!("CL_PLATFORM_NAME: {:?}", value);
        let value = value.into_string().unwrap();
        assert!(0 < value.len());

        let value = get_platform_info(platform_id, CL_PLATFORM_VENDOR).unwrap();
        let value = value.to_str().unwrap();
        println!("CL_PLATFORM_VENDOR: {:?}", value);
        let value = value.into_string().unwrap();
        assert!(0 < value.len());

        let value = get_platform_info(platform_id, CL_PLATFORM_EXTENSIONS).unwrap();
        let value = value.to_str().unwrap();
        println!("CL_PLATFORM_EXTENSIONS: {:?}", value);
        let value = value.into_string().unwrap();
        assert!(0 < value.len());

        let value = get_platform_info(platform_id, CL_PLATFORM_HOST_TIMER_RESOLUTION).unwrap();
        let value = value.to_ulong();
        println!("CL_PLATFORM_HOST_TIMER_RESOLUTION: {}", value);
        assert!(0 < value);
    }

    #[test]
    #[cfg(feature = "CL_VERSION_3_0")]
    fn test_get_platform_info_3_0() {
        let platform_ids = get_platform_ids().unwrap();

        // Choose a platform which is CL_VERSION_2_1 compliant
        let platform_id = platform_ids[1];

        let value = get_platform_info(platform_id, CL_PLATFORM_NUMERIC_VERSION).unwrap();
        let value = value.to_uint();
        println!("CL_PLATFORM_NUMERIC_VERSION: {}", value);
        assert!(0 < value);

        let value = get_platform_info(platform_id, CL_PLATFORM_EXTENSIONS_WITH_VERSION).unwrap();
        let value = value.to_vec_name_version();
        println!("CL_PLATFORM_EXTENSIONS_WITH_VERSION: {}", value.len());
        println!("CL_PLATFORM_EXTENSIONS_WITH_VERSION: {:?}", value);
        assert!(0 < value.len());
    }
}
