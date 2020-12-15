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

//! OpenCL Device API.

use super::error_codes::{CL_DEVICE_NOT_FOUND, CL_INVALID_VALUE, CL_SUCCESS};
use super::ffi::cl::{clGetDeviceIDs, clGetDeviceInfo, clCreateSubDevices,
    clRetainDevice, clReleaseDevice, clSetDefaultDeviceCommandQueue,
    clGetDeviceAndHostTimer, clGetHostTimer,
};
use super::info_type::InfoType;
use super::types::{
    cl_device_id, cl_device_info, cl_device_type, cl_int, cl_name_version,
    cl_platform_id, cl_uint, cl_ulong, cl_device_partition_property, cl_device_affinity_domain,
    cl_context, cl_command_queue, cl_device_svm_capabilities,
};
use super::{api_info_size, api_info_value, api_info_vector};

use libc::{c_void, intptr_t, size_t};
use std::mem;
use std::ptr;

// cl_device_type - bitfield
pub const CL_DEVICE_TYPE_DEFAULT: cl_device_type = 1 << 0;
pub const CL_DEVICE_TYPE_CPU: cl_device_type = 1 << 1;
pub const CL_DEVICE_TYPE_GPU: cl_device_type = 1 << 2;
pub const CL_DEVICE_TYPE_ACCELERATOR: cl_device_type = 1 << 3;
/// CL_VERSION_1_2
pub const CL_DEVICE_TYPE_CUSTOM: cl_device_type = 1 << 4;
pub const CL_DEVICE_TYPE_ALL: cl_device_type = 0xFFFFFFFF;

/// Get the list of available devices of the given type on a platform.  
/// Calls clGetDeviceIDs to get the available device ids on the platform.
///  # Examples
/// ```
/// use cl3::platform::get_platform_ids;
/// use cl3::device::{get_device_ids, CL_DEVICE_TYPE_GPU};
///
/// let platform_ids = get_platform_ids().unwrap();
/// assert!(0 < platform_ids.len());
/// 
/// // Choose a the first platform
/// let platform_id = platform_ids[0];
/// 
/// let device_ids = get_device_ids(platform_id, CL_DEVICE_TYPE_GPU).unwrap();
/// println!("CL_DEVICE_TYPE_GPU count: {}", device_ids.len());
/// assert!(0 < device_ids.len());
/// ```
/// * `platform` - the cl_platform_id of the OpenCL platform.
/// * `device_type` - the type of device, see
/// [Device Types](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#device-types-table).
/// 
/// returns a Result containing a vector of available device ids
/// or the error code from the OpenCL C API function.
pub fn get_device_ids(
    platform: cl_platform_id,
    device_type: cl_device_type,
) -> Result<Vec<cl_device_id>, cl_int> {
    // Get the number of devices of device_type
    let mut count: cl_uint = 0;
    let mut status =
        unsafe { clGetDeviceIDs(platform, device_type, 0, ptr::null_mut(), &mut count) };

    if (CL_SUCCESS != status) && (CL_DEVICE_NOT_FOUND != status) {
        Err(status)
    } else {
        if 0 < count {
            // Get the device ids.
            let len = count as usize;
            let mut ids: Vec<cl_device_id> = Vec::with_capacity(len);
            unsafe {
                ids.set_len(len);
                status = clGetDeviceIDs(
                    platform,
                    device_type,
                    count,
                    ids.as_mut_ptr(),
                    ptr::null_mut(),
                );
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

// cl_device_svm_capabilities
pub const CL_DEVICE_SVM_COARSE_GRAIN_BUFFER: cl_device_svm_capabilities = 1 << 0;
pub const CL_DEVICE_SVM_FINE_GRAIN_BUFFER: cl_device_svm_capabilities = 1 << 1;
pub const CL_DEVICE_SVM_FINE_GRAIN_SYSTEM: cl_device_svm_capabilities = 1 << 2;
pub const CL_DEVICE_SVM_ATOMICS: cl_device_svm_capabilities = 1 << 3;

// cl_device_info
pub const CL_DEVICE_TYPE: cl_device_info = 0x1000;
pub const CL_DEVICE_VENDOR_ID: cl_device_info = 0x1001;
pub const CL_DEVICE_MAX_COMPUTE_UNITS: cl_device_info = 0x1002;
pub const CL_DEVICE_MAX_WORK_ITEM_DIMENSIONS: cl_device_info = 0x1003;
pub const CL_DEVICE_MAX_WORK_GROUP_SIZE: cl_device_info = 0x1004;
pub const CL_DEVICE_MAX_WORK_ITEM_SIZES: cl_device_info = 0x1005;
pub const CL_DEVICE_PREFERRED_VECTOR_WIDTH_CHAR: cl_device_info = 0x1006;
pub const CL_DEVICE_PREFERRED_VECTOR_WIDTH_SHORT: cl_device_info = 0x1007;
pub const CL_DEVICE_PREFERRED_VECTOR_WIDTH_INT: cl_device_info = 0x1008;
pub const CL_DEVICE_PREFERRED_VECTOR_WIDTH_LONG: cl_device_info = 0x1009;
pub const CL_DEVICE_PREFERRED_VECTOR_WIDTH_FLOAT: cl_device_info = 0x100A;
pub const CL_DEVICE_PREFERRED_VECTOR_WIDTH_DOUBLE: cl_device_info = 0x100B;
pub const CL_DEVICE_MAX_CLOCK_FREQUENCY: cl_device_info = 0x100C;
pub const CL_DEVICE_ADDRESS_BITS: cl_device_info = 0x100D;
pub const CL_DEVICE_MAX_READ_IMAGE_ARGS: cl_device_info = 0x100E;
pub const CL_DEVICE_MAX_WRITE_IMAGE_ARGS: cl_device_info = 0x100F;
pub const CL_DEVICE_MAX_MEM_ALLOC_SIZE: cl_device_info = 0x1010;
pub const CL_DEVICE_IMAGE2D_MAX_WIDTH: cl_device_info = 0x1011;
pub const CL_DEVICE_IMAGE2D_MAX_HEIGHT: cl_device_info = 0x1012;
pub const CL_DEVICE_IMAGE3D_MAX_WIDTH: cl_device_info = 0x1013;
pub const CL_DEVICE_IMAGE3D_MAX_HEIGHT: cl_device_info = 0x1014;
pub const CL_DEVICE_IMAGE3D_MAX_DEPTH: cl_device_info = 0x1015;
pub const CL_DEVICE_IMAGE_SUPPORT: cl_device_info = 0x1016;
pub const CL_DEVICE_MAX_PARAMETER_SIZE: cl_device_info = 0x1017;
pub const CL_DEVICE_MAX_SAMPLERS: cl_device_info = 0x1018;
pub const CL_DEVICE_MEM_BASE_ADDR_ALIGN: cl_device_info = 0x1019;
pub const CL_DEVICE_MIN_DATA_TYPE_ALIGN_SIZE: cl_device_info = 0x101A;
pub const CL_DEVICE_SINGLE_FP_CONFIG: cl_device_info = 0x101B;
pub const CL_DEVICE_GLOBAL_MEM_CACHE_TYPE: cl_device_info = 0x101C;
pub const CL_DEVICE_GLOBAL_MEM_CACHELINE_SIZE: cl_device_info = 0x101D;
pub const CL_DEVICE_GLOBAL_MEM_CACHE_SIZE: cl_device_info = 0x101E;
pub const CL_DEVICE_GLOBAL_MEM_SIZE: cl_device_info = 0x101F;
pub const CL_DEVICE_MAX_CONSTANT_BUFFER_SIZE: cl_device_info = 0x1020;
pub const CL_DEVICE_MAX_CONSTANT_ARGS: cl_device_info = 0x1021;
pub const CL_DEVICE_LOCAL_MEM_TYPE: cl_device_info = 0x1022;
pub const CL_DEVICE_LOCAL_MEM_SIZE: cl_device_info = 0x1023;
pub const CL_DEVICE_ERROR_CORRECTION_SUPPORT: cl_device_info = 0x1024;
pub const CL_DEVICE_PROFILING_TIMER_RESOLUTION: cl_device_info = 0x1025;
pub const CL_DEVICE_ENDIAN_LITTLE: cl_device_info = 0x1026;
pub const CL_DEVICE_AVAILABLE: cl_device_info = 0x1027;
pub const CL_DEVICE_COMPILER_AVAILABLE: cl_device_info = 0x1028;
pub const CL_DEVICE_EXECUTION_CAPABILITIES: cl_device_info = 0x1029;
// pub const CL_DEVICE_QUEUE_PROPERTIES: cl_device_info = 0x102A; // DEPRECATED 2.0
// #ifdef CL_VERSION_2_0
pub const CL_DEVICE_QUEUE_ON_HOST_PROPERTIES: cl_device_info = 0x102A;
// #endif
pub const CL_DEVICE_NAME: cl_device_info = 0x102B;
pub const CL_DEVICE_VENDOR: cl_device_info = 0x102C;
pub const CL_DRIVER_VERSION: cl_device_info = 0x102D;
pub const CL_DEVICE_PROFILE: cl_device_info = 0x102E;
pub const CL_DEVICE_VERSION: cl_device_info = 0x102F;
pub const CL_DEVICE_EXTENSIONS: cl_device_info = 0x1030;
pub const CL_DEVICE_PLATFORM: cl_device_info = 0x1031;
// #ifdef CL_VERSION_1_2
pub const CL_DEVICE_DOUBLE_FP_CONFIG: cl_device_info = 0x1032;
// #endif
pub const CL_DEVICE_HALF_FP_CONFIG: cl_device_info = 0x1033; // defined in "cl_ext.h

// #ifdef CL_VERSION_1_1
pub const CL_DEVICE_PREFERRED_VECTOR_WIDTH_HALF: cl_device_info = 0x1034;
pub const CL_DEVICE_HOST_UNIFIED_MEMORY: cl_device_info = 0x1035; // DEPRECATED 2.0
pub const CL_DEVICE_NATIVE_VECTOR_WIDTH_CHAR: cl_device_info = 0x1036;
pub const CL_DEVICE_NATIVE_VECTOR_WIDTH_SHORT: cl_device_info = 0x1037;
pub const CL_DEVICE_NATIVE_VECTOR_WIDTH_INT: cl_device_info = 0x1038;
pub const CL_DEVICE_NATIVE_VECTOR_WIDTH_LONG: cl_device_info = 0x1039;
pub const CL_DEVICE_NATIVE_VECTOR_WIDTH_FLOAT: cl_device_info = 0x103A;
pub const CL_DEVICE_NATIVE_VECTOR_WIDTH_DOUBLE: cl_device_info = 0x103B;
pub const CL_DEVICE_NATIVE_VECTOR_WIDTH_HALF: cl_device_info = 0x103C;
pub const CL_DEVICE_OPENCL_C_VERSION: cl_device_info = 0x103D; // DEPRECATED 3.0

// #endif

// #ifdef CL_VERSION_1_2
pub const CL_DEVICE_LINKER_AVAILABLE: cl_device_info = 0x103E;
pub const CL_DEVICE_BUILT_IN_KERNELS: cl_device_info = 0x103F;
pub const CL_DEVICE_IMAGE_MAX_BUFFER_SIZE: cl_device_info = 0x1040;
pub const CL_DEVICE_IMAGE_MAX_ARRAY_SIZE: cl_device_info = 0x1041;
pub const CL_DEVICE_PARENT_DEVICE: cl_device_info = 0x1042;
pub const CL_DEVICE_PARTITION_MAX_SUB_DEVICES: cl_device_info = 0x1043;
pub const CL_DEVICE_PARTITION_PROPERTIES: cl_device_info = 0x1044;
pub const CL_DEVICE_PARTITION_AFFINITY_DOMAIN: cl_device_info = 0x1045;
pub const CL_DEVICE_PARTITION_TYPE: cl_device_info = 0x1046;
pub const CL_DEVICE_REFERENCE_COUNT: cl_device_info = 0x1047;
pub const CL_DEVICE_PREFERRED_INTEROP_USER_SYNC: cl_device_info = 0x1048;
pub const CL_DEVICE_PRINTF_BUFFER_SIZE: cl_device_info = 0x1049;
// #endif
// #ifdef CL_VERSION_2_0
pub const CL_DEVICE_IMAGE_PITCH_ALIGNMENT: cl_device_info = 0x104A;
pub const CL_DEVICE_IMAGE_BASE_ADDRESS_ALIGNMENT: cl_device_info = 0x104B;
pub const CL_DEVICE_MAX_READ_WRITE_IMAGE_ARGS: cl_device_info = 0x104C;
pub const CL_DEVICE_MAX_GLOBAL_VARIABLE_SIZE: cl_device_info = 0x104D;
pub const CL_DEVICE_QUEUE_ON_DEVICE_PROPERTIES: cl_device_info = 0x104E;
pub const CL_DEVICE_QUEUE_ON_DEVICE_PREFERRED_SIZE: cl_device_info = 0x104F;
pub const CL_DEVICE_QUEUE_ON_DEVICE_MAX_SIZE: cl_device_info = 0x1050;
pub const CL_DEVICE_MAX_ON_DEVICE_QUEUES: cl_device_info = 0x1051;
pub const CL_DEVICE_MAX_ON_DEVICE_EVENTS: cl_device_info = 0x1052;
pub const CL_DEVICE_SVM_CAPABILITIES: cl_device_info = 0x1053;
pub const CL_DEVICE_GLOBAL_VARIABLE_PREFERRED_TOTAL_SIZE: cl_device_info = 0x1054;
pub const CL_DEVICE_MAX_PIPE_ARGS: cl_device_info = 0x1055;
pub const CL_DEVICE_PIPE_MAX_ACTIVE_RESERVATIONS: cl_device_info = 0x1056;
pub const CL_DEVICE_PIPE_MAX_PACKET_SIZE: cl_device_info = 0x1057;
pub const CL_DEVICE_PREFERRED_PLATFORM_ATOMIC_ALIGNMENT: cl_device_info = 0x1058;
pub const CL_DEVICE_PREFERRED_GLOBAL_ATOMIC_ALIGNMENT: cl_device_info = 0x1059;
pub const CL_DEVICE_PREFERRED_LOCAL_ATOMIC_ALIGNMENT: cl_device_info = 0x105A;
// #endif
// #ifdef CL_VERSION_2_1
pub const CL_DEVICE_IL_VERSION: cl_device_info = 0x105B;
pub const CL_DEVICE_MAX_NUM_SUB_GROUPS: cl_device_info = 0x105C;
pub const CL_DEVICE_SUB_GROUP_INDEPENDENT_FORWARD_PROGRESS: cl_device_info = 0x105D;
// #endif
// #ifdef CL_VERSION_3_0
pub const CL_DEVICE_NUMERIC_VERSION: cl_device_info = 0x105E;
pub const CL_DEVICE_EXTENSIONS_WITH_VERSION: cl_device_info = 0x1060;
pub const CL_DEVICE_ILS_WITH_VERSION: cl_device_info = 0x1061;
pub const CL_DEVICE_BUILT_IN_KERNELS_WITH_VERSION: cl_device_info = 0x1062;
pub const CL_DEVICE_ATOMIC_MEMORY_CAPABILITIES: cl_device_info = 0x1063;
pub const CL_DEVICE_ATOMIC_FENCE_CAPABILITIES: cl_device_info = 0x1064;
pub const CL_DEVICE_NON_UNIFORM_WORK_GROUP_SUPPORT: cl_device_info = 0x1065;
pub const CL_DEVICE_OPENCL_C_ALL_VERSIONS: cl_device_info = 0x1066;
pub const CL_DEVICE_PREFERRED_WORK_GROUP_SIZE_MULTIPLE: cl_device_info = 0x1067;
pub const CL_DEVICE_WORK_GROUP_COLLECTIVE_FUNCTIONS_SUPPORT: cl_device_info = 0x1068;
pub const CL_DEVICE_GENERIC_ADDRESS_SPACE_SUPPORT: cl_device_info = 0x1069;
// 0x106A to 0x106E - Reserved for upcoming KHR extension
pub const CL_DEVICE_OPENCL_C_FEATURES: cl_device_info = 0x106F;
pub const CL_DEVICE_DEVICE_ENQUEUE_CAPABILITIES: cl_device_info = 0x1070;
pub const CL_DEVICE_PIPE_SUPPORT: cl_device_info = 0x1071;
pub const CL_DEVICE_LATEST_CONFORMANCE_VERSION_PASSED: cl_device_info = 0x1072;
// #endif

/// Get specific information about an OpenCL device.  
/// Calls clGetDeviceInfo to get the desired information about the device.
///  # Examples
/// ```
/// use cl3::platform::get_platform_ids;
/// use cl3::device::{get_device_ids, get_device_info,
/// CL_DEVICE_TYPE_GPU, CL_DEVICE_TYPE, CL_DEVICE_VENDOR, CL_DEVICE_VERSION};
///
/// let platform_ids = get_platform_ids().unwrap();
/// assert!(0 < platform_ids.len());
/// 
/// // Choose a the first platform
/// let platform_id = platform_ids[0];
/// 
/// let device_ids = get_device_ids(platform_id, CL_DEVICE_TYPE_GPU).unwrap();
/// println!("CL_DEVICE_TYPE_GPU count: {}", device_ids.len());
/// assert!(0 < device_ids.len());
///
/// // Choose a the first device
/// let device_id = device_ids[0];
/// 
/// let value = get_device_info(device_id, CL_DEVICE_TYPE).unwrap();
/// let value = value.to_ulong();
/// println!("CL_DEVICE_TYPE: {}", value);
/// assert_eq!(CL_DEVICE_TYPE_GPU, value);
/// 
/// let value = get_device_info(device_id, CL_DEVICE_VENDOR).unwrap();
/// let value = value.to_str().unwrap();
/// println!("CL_DEVICE_VENDOR: {:?}", value);
/// let value = value.into_string().unwrap();
/// assert!(0 < value.len());
/// 
/// let value = get_device_info(device_id, CL_DEVICE_VERSION).unwrap();
/// let value = value.to_str().unwrap();
/// println!("CL_DEVICE_VERSION: {:?}", value);
/// let value = value.into_string().unwrap();
/// assert!(0 < value.len());
/// ```
/// * `device` - the cl_device_id of the OpenCL device.
/// * `param_name` - the type of device information being queried, see
/// [Device Queries](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#device-queries-table).
///
/// returns a Result containing the desired information in an InfoType enum
/// or the error code from the OpenCL C API function.
pub fn get_device_info(
    device: cl_device_id,
    param_name: cl_device_info,
) -> Result<InfoType, cl_int> {
    api_info_size!(get_size, clGetDeviceInfo);

    match param_name {
        CL_DEVICE_NAME
        | CL_DEVICE_VENDOR
        | CL_DRIVER_VERSION
        | CL_DEVICE_PROFILE
        | CL_DEVICE_VERSION
        | CL_DEVICE_EXTENSIONS
        | CL_DEVICE_OPENCL_C_VERSION
        | CL_DEVICE_BUILT_IN_KERNELS
        | CL_DEVICE_IL_VERSION
        | CL_DEVICE_LATEST_CONFORMANCE_VERSION_PASSED // CL_VERSION_3_0
        => {
            api_info_vector!(get_string, u8, clGetDeviceInfo);
            let size = get_size(device, param_name)?;
            Ok(InfoType::Str(get_string(device, param_name, size)?))
        }

        CL_DEVICE_VENDOR_ID
        | CL_DEVICE_MAX_COMPUTE_UNITS
        | CL_DEVICE_MAX_WORK_ITEM_DIMENSIONS
        | CL_DEVICE_PREFERRED_VECTOR_WIDTH_CHAR
        | CL_DEVICE_PREFERRED_VECTOR_WIDTH_SHORT
        | CL_DEVICE_PREFERRED_VECTOR_WIDTH_INT
        | CL_DEVICE_PREFERRED_VECTOR_WIDTH_LONG
        | CL_DEVICE_PREFERRED_VECTOR_WIDTH_FLOAT
        | CL_DEVICE_PREFERRED_VECTOR_WIDTH_DOUBLE
        | CL_DEVICE_MAX_CLOCK_FREQUENCY
        | CL_DEVICE_ADDRESS_BITS
        | CL_DEVICE_MAX_READ_IMAGE_ARGS
        | CL_DEVICE_MAX_WRITE_IMAGE_ARGS
        | CL_DEVICE_IMAGE_SUPPORT
        | CL_DEVICE_MAX_SAMPLERS
        | CL_DEVICE_MEM_BASE_ADDR_ALIGN
        | CL_DEVICE_MIN_DATA_TYPE_ALIGN_SIZE
        | CL_DEVICE_GLOBAL_MEM_CACHE_TYPE
        | CL_DEVICE_GLOBAL_MEM_CACHELINE_SIZE
        | CL_DEVICE_MAX_CONSTANT_ARGS
        | CL_DEVICE_LOCAL_MEM_TYPE
        | CL_DEVICE_ERROR_CORRECTION_SUPPORT
        | CL_DEVICE_ENDIAN_LITTLE
        | CL_DEVICE_AVAILABLE
        | CL_DEVICE_COMPILER_AVAILABLE
        | CL_DEVICE_PREFERRED_VECTOR_WIDTH_HALF
        | CL_DEVICE_NATIVE_VECTOR_WIDTH_CHAR
        | CL_DEVICE_NATIVE_VECTOR_WIDTH_SHORT
        | CL_DEVICE_NATIVE_VECTOR_WIDTH_INT
        | CL_DEVICE_NATIVE_VECTOR_WIDTH_LONG
        | CL_DEVICE_NATIVE_VECTOR_WIDTH_FLOAT
        | CL_DEVICE_NATIVE_VECTOR_WIDTH_DOUBLE
        | CL_DEVICE_NATIVE_VECTOR_WIDTH_HALF
        | CL_DEVICE_LINKER_AVAILABLE
        | CL_DEVICE_PARTITION_MAX_SUB_DEVICES
        | CL_DEVICE_REFERENCE_COUNT
        | CL_DEVICE_PREFERRED_INTEROP_USER_SYNC
        | CL_DEVICE_IMAGE_PITCH_ALIGNMENT
        | CL_DEVICE_IMAGE_BASE_ADDRESS_ALIGNMENT
        | CL_DEVICE_MAX_READ_WRITE_IMAGE_ARGS
        | CL_DEVICE_MAX_ON_DEVICE_QUEUES
        | CL_DEVICE_MAX_ON_DEVICE_EVENTS
        | CL_DEVICE_MAX_PIPE_ARGS
        | CL_DEVICE_PIPE_MAX_ACTIVE_RESERVATIONS
        | CL_DEVICE_PIPE_MAX_PACKET_SIZE
        | CL_DEVICE_PREFERRED_PLATFORM_ATOMIC_ALIGNMENT
        | CL_DEVICE_PREFERRED_GLOBAL_ATOMIC_ALIGNMENT
        | CL_DEVICE_PREFERRED_LOCAL_ATOMIC_ALIGNMENT
        | CL_DEVICE_MAX_NUM_SUB_GROUPS
        | CL_DEVICE_SUB_GROUP_INDEPENDENT_FORWARD_PROGRESS
        | CL_DEVICE_NUMERIC_VERSION // CL_VERSION_3_0
        | CL_DEVICE_NON_UNIFORM_WORK_GROUP_SUPPORT // CL_VERSION_3_0
        | CL_DEVICE_WORK_GROUP_COLLECTIVE_FUNCTIONS_SUPPORT // CL_VERSION_3_0
        | CL_DEVICE_GENERIC_ADDRESS_SPACE_SUPPORT // CL_VERSION_3_0
        | CL_DEVICE_PIPE_SUPPORT // CL_VERSION_3_0
        => {
            api_info_value!(get_value, cl_uint, clGetDeviceInfo);
            Ok(InfoType::Uint(get_value(device, param_name)?))
        }

        CL_DEVICE_TYPE
        | CL_DEVICE_MAX_MEM_ALLOC_SIZE
        | CL_DEVICE_SINGLE_FP_CONFIG
        | CL_DEVICE_GLOBAL_MEM_CACHE_SIZE
        | CL_DEVICE_GLOBAL_MEM_SIZE
        | CL_DEVICE_MAX_CONSTANT_BUFFER_SIZE
        | CL_DEVICE_LOCAL_MEM_SIZE
        | CL_DEVICE_EXECUTION_CAPABILITIES
        | CL_DEVICE_QUEUE_ON_HOST_PROPERTIES
        | CL_DEVICE_DOUBLE_FP_CONFIG
        | CL_DEVICE_HALF_FP_CONFIG
        | CL_DEVICE_SVM_CAPABILITIES
        | CL_DEVICE_ATOMIC_MEMORY_CAPABILITIES // CL_VERSION_3_0
        | CL_DEVICE_ATOMIC_FENCE_CAPABILITIES // CL_VERSION_3_0
        | CL_DEVICE_DEVICE_ENQUEUE_CAPABILITIES // CL_VERSION_3_0
         => {
            api_info_value!(get_value, cl_ulong, clGetDeviceInfo);
            Ok(InfoType::Ulong(get_value(device, param_name)?))
        }
        CL_DEVICE_MAX_WORK_GROUP_SIZE
        | CL_DEVICE_IMAGE2D_MAX_WIDTH
        | CL_DEVICE_IMAGE2D_MAX_HEIGHT
        | CL_DEVICE_IMAGE3D_MAX_WIDTH
        | CL_DEVICE_IMAGE3D_MAX_HEIGHT
        | CL_DEVICE_IMAGE3D_MAX_DEPTH
        | CL_DEVICE_MAX_PARAMETER_SIZE
        | CL_DEVICE_PROFILING_TIMER_RESOLUTION
        | CL_DEVICE_IMAGE_MAX_BUFFER_SIZE
        | CL_DEVICE_IMAGE_MAX_ARRAY_SIZE
        | CL_DEVICE_PRINTF_BUFFER_SIZE
        | CL_DEVICE_MAX_GLOBAL_VARIABLE_SIZE
        | CL_DEVICE_QUEUE_ON_DEVICE_PREFERRED_SIZE
        | CL_DEVICE_QUEUE_ON_DEVICE_MAX_SIZE
        | CL_DEVICE_GLOBAL_VARIABLE_PREFERRED_TOTAL_SIZE 
        | CL_DEVICE_PREFERRED_WORK_GROUP_SIZE_MULTIPLE // CL_VERSION_3_0
        => {
            api_info_value!(get_value, size_t, clGetDeviceInfo);
            Ok(InfoType::Size(get_value(device, param_name)?))
        }

        CL_DEVICE_PLATFORM | CL_DEVICE_PARENT_DEVICE => {
            api_info_value!(get_value, intptr_t, clGetDeviceInfo);
            Ok(InfoType::Ptr(get_value(device, param_name)?))
        }

        CL_DEVICE_PARTITION_AFFINITY_DOMAIN => {
            api_info_vector!(get_vec, cl_ulong, clGetDeviceInfo);
            let size = get_size(device, param_name)?;
            Ok(InfoType::VecUlong(get_vec(device, param_name, size)?))
        }

        CL_DEVICE_MAX_WORK_ITEM_SIZES => {
            api_info_vector!(get_vec, size_t, clGetDeviceInfo);
            let size = get_size(device, param_name)?;
            Ok(InfoType::VecSize(get_vec(device, param_name, size)?))
        }

        CL_DEVICE_PARTITION_PROPERTIES
        | CL_DEVICE_PARTITION_TYPE
        | CL_DEVICE_QUEUE_ON_DEVICE_PROPERTIES => {
            api_info_vector!(get_vec, intptr_t, clGetDeviceInfo);
            let size = get_size(device, param_name)?;
            Ok(InfoType::VecIntPtr(get_vec(device, param_name, size)?))
        }

        // CL_VERSION_3_0
        CL_DEVICE_EXTENSIONS_WITH_VERSION
        | CL_DEVICE_ILS_WITH_VERSION
        | CL_DEVICE_BUILT_IN_KERNELS_WITH_VERSION 
        | CL_DEVICE_OPENCL_C_ALL_VERSIONS
        | CL_DEVICE_OPENCL_C_FEATURES => {
            api_info_vector!(get_vec, cl_name_version, clGetDeviceInfo);
            let size = get_size(device, param_name)?;
            Ok(InfoType::VecNameVersion(get_vec(device, param_name, size)?))
        }
        _ => Err(CL_INVALID_VALUE),
    }
}

// cl_device_partition_property:
pub const CL_DEVICE_PARTITION_EQUALLY:cl_uint = 0x1086;
pub const CL_DEVICE_PARTITION_BY_COUNTS: cl_uint = 0x1087;
pub const CL_DEVICE_PARTITION_BY_COUNTS_LIST_END: cl_uint = 0x0;
pub const CL_DEVICE_PARTITION_BY_AFFINITY_DOMAIN: cl_device_affinity_domain = 0x1088;

// helper function for create_sub_devices
fn count_sub_devices(
    in_device: cl_device_id,
    properties: &[cl_device_partition_property],) -> Result<cl_uint, cl_int> {
    let mut count: cl_uint = 0;
    let status: cl_int =
        unsafe { clCreateSubDevices(in_device, properties.as_ptr(), 0, ptr::null_mut(), &mut count) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(count)
    }
}

/// Create sub-devices by partitioning an OpenCL device.
/// Calls clCreateSubDevices to get the partitioned sub-devices.
///
/// * `in_device` - the cl_device_id of the OpenCL device to partition.
/// * `properties` - the slice of cl_device_partition_property, see
/// [Subdevice Partition](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#subdevice-partition-table).
/// 
/// returns a Result containing a vector of available sub-device ids
/// or the error code from the OpenCL C API function.
pub fn create_sub_devices(
    in_device: cl_device_id,
    properties: &[cl_device_partition_property],
) -> Result<Vec<cl_device_id>, cl_int> {
    // get the number of partitions
    let num_devices: cl_uint = count_sub_devices(in_device, properties)?;

    // partition in_device
    let mut ids: Vec<cl_device_id> = Vec::with_capacity(num_devices as usize);
    unsafe { ids.set_len(num_devices as usize) };
    let status: cl_int =
    unsafe { clCreateSubDevices(
        in_device,
        properties.as_ptr(),
        num_devices,
        ids.as_mut_ptr(),
        ptr::null_mut(),)
    };

    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(ids)
    }
}

/// Retain an OpenCL device.  
/// Calls clRetainDevice to increment the device reference count
/// if device is a valid sub-device created by a call to clCreateSubDevices.
///
/// * `device` - the cl_device_id of the OpenCL device.
/// 
/// returns an empty Result or the error code from the OpenCL C API function.
pub fn retain_device(device: cl_device_id) -> Result<(), cl_int> {
    let status: cl_int = unsafe { clRetainDevice(device) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(())
    }
}

/// Release an OpenCL device.  
/// Calls clReleaseDevice to decrement the device reference count
/// if device is a valid sub-device created by a call to clCreateSubDevices.
///
/// * `device` - the cl_device_id of the OpenCL device.
/// 
/// returns an empty Result or the error code from the OpenCL C API function.
pub fn release_device(device: cl_device_id) -> Result<(), cl_int> {
    let status: cl_int = unsafe { clReleaseDevice(device) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(())
    }
}

// #ifdef CL_VERSION_2_1
/// Replace the default command queue on an OpenCL device.  
/// Calls clSetDefaultDeviceCommandQueue to replace the default command queue  
/// CL_VERSION_2_1
///
/// * `context` - the OpenCL context used to create command_queue.
/// * `device` - a valid OpenCL device associated with context.
/// * `command_queue` - a command queue object which replaces the default
/// device command queue.
/// 
/// returns an empty Result or the error code from the OpenCL C API function.
#[cfg(feature = "CL_VERSION_2_1")]
pub fn set_default_device_command_queue(
    context: cl_context,
    device: cl_device_id,
    command_queue: cl_command_queue,) -> Result<(), cl_int> {
    let status: cl_int = unsafe { 
        clSetDefaultDeviceCommandQueue(context, device, command_queue)
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(())
    }
}

/// Query device and host timestamps.  
/// Calls clGetDeviceAndHostTimer  
/// CL_VERSION_2_1
///
/// * `device` - a valid OpenCL device.
/// 
/// returns a Result containing device_timestamp and host_timestamp in a 2D array
/// or the error code from the OpenCL C API function.
#[cfg(feature = "CL_VERSION_2_1")]
pub fn get_device_and_host_timer(device: cl_device_id,) -> Result<[cl_ulong; 2], cl_int> {
    let mut device_timestamp: cl_ulong = 0;
    let mut host_timestamp: cl_ulong = 0;
    let status: cl_int = unsafe { 
        clGetDeviceAndHostTimer(device, &mut device_timestamp, &mut host_timestamp)
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok([device_timestamp, host_timestamp])
    }
}

/// The current value of the host clock as seen by device.  
/// Calls clGetHostTimer  
/// CL_VERSION_2_1
///
/// * `device` - a valid OpenCL device.
/// 
/// returns a Result containing host_timestamp 
/// or the error code from the OpenCL C API function.
#[cfg(feature = "CL_VERSION_2_1")]
pub fn get_host_timer(device: cl_device_id,) -> Result<cl_ulong, cl_int> {
    let mut host_timestamp: cl_ulong = 0;
    let status: cl_int = unsafe { 
        clGetHostTimer(device, &mut host_timestamp)
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(host_timestamp)
    }
}
// #endif

#[cfg(test)]
mod tests {
    use super::*;
    use crate::platform::get_platform_ids;

    #[test]
    fn test_get_platform_devices() {
        let platform_ids = get_platform_ids().unwrap();
        assert!(0 < platform_ids.len());

        let device_ids = get_device_ids(platform_ids[0], CL_DEVICE_TYPE_ALL).unwrap();
        println!("Platform[0]->number of devices: {}", device_ids.len());
        assert!(0 < device_ids.len());
    }

    #[test]
    fn test_get_device_info() {
        let platform_ids = get_platform_ids().unwrap();

        // Choose the platform with the most compliant GPU
        let platform_id = platform_ids[1];

        let device_ids = get_device_ids(platform_id, CL_DEVICE_TYPE_GPU).unwrap();
        println!("CL_DEVICE_TYPE_GPU count: {}", device_ids.len());
        assert!(0 < device_ids.len());

        let device_id = device_ids[0];

        let value = get_device_info(device_id, CL_DEVICE_TYPE).unwrap();
        let value = value.to_ulong();
        println!("CL_DEVICE_TYPE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_VENDOR_ID).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_VENDOR_ID: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_MAX_COMPUTE_UNITS).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_MAX_COMPUTE_UNITS: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_MAX_WORK_ITEM_DIMENSIONS).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_MAX_WORK_ITEM_DIMENSIONS: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_MAX_WORK_GROUP_SIZE).unwrap();
        let value = value.to_size();
        println!("CL_DEVICE_MAX_WORK_GROUP_SIZE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_MAX_WORK_ITEM_SIZES).unwrap();
        let value = value.to_vec_size();
        println!("CL_DEVICE_MAX_WORK_ITEM_SIZES len: {:?}", value.len());
        println!("CL_DEVICE_MAX_WORK_ITEM_SIZES: {:?}", value);
        assert!(0 < value.len());

        let value = get_device_info(device_id, CL_DEVICE_PREFERRED_VECTOR_WIDTH_CHAR).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_PREFERRED_VECTOR_WIDTH_CHAR: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_PREFERRED_VECTOR_WIDTH_SHORT).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_PREFERRED_VECTOR_WIDTH_SHORT: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_PREFERRED_VECTOR_WIDTH_INT).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_PREFERRED_VECTOR_WIDTH_INT: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_PREFERRED_VECTOR_WIDTH_LONG).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_PREFERRED_VECTOR_WIDTH_LONG: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_PREFERRED_VECTOR_WIDTH_FLOAT).unwrap();
        let value = value.to_uint();
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_PREFERRED_VECTOR_WIDTH_DOUBLE).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_PREFERRED_VECTOR_WIDTH_DOUBLE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_MAX_CLOCK_FREQUENCY).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_MAX_CLOCK_FREQUENCY: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_ADDRESS_BITS).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_ADDRESS_BITS: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_MAX_READ_IMAGE_ARGS).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_MAX_READ_IMAGE_ARGS: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_MAX_WRITE_IMAGE_ARGS).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_MAX_WRITE_IMAGE_ARGS: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_MAX_MEM_ALLOC_SIZE).unwrap();
        let value = value.to_ulong();
        println!("CL_DEVICE_MAX_MEM_ALLOC_SIZE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_IMAGE2D_MAX_WIDTH).unwrap();
        let value = value.to_size();
        println!("CL_DEVICE_IMAGE2D_MAX_WIDTH: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_IMAGE2D_MAX_HEIGHT).unwrap();
        let value = value.to_size();
        println!("CL_DEVICE_IMAGE2D_MAX_HEIGHT: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_IMAGE3D_MAX_WIDTH).unwrap();
        let value = value.to_size();
        println!("CL_DEVICE_IMAGE3D_MAX_WIDTH: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_IMAGE3D_MAX_HEIGHT).unwrap();
        let value = value.to_size();
        println!("CL_DEVICE_IMAGE3D_MAX_HEIGHT: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_IMAGE3D_MAX_DEPTH).unwrap();
        let value = value.to_size();
        println!("CL_DEVICE_IMAGE3D_MAX_DEPTH: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_IMAGE_SUPPORT).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_IMAGE_SUPPORT: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_MAX_PARAMETER_SIZE).unwrap();
        let value = value.to_size();
        println!("CL_DEVICE_MAX_PARAMETER_SIZE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_MAX_SAMPLERS).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_MAX_SAMPLERS: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_MEM_BASE_ADDR_ALIGN).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_MEM_BASE_ADDR_ALIGN: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_MIN_DATA_TYPE_ALIGN_SIZE).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_MIN_DATA_TYPE_ALIGN_SIZE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_SINGLE_FP_CONFIG).unwrap();
        let value = value.to_ulong();
        println!("CL_DEVICE_SINGLE_FP_CONFIG: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_GLOBAL_MEM_CACHE_TYPE).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_GLOBAL_MEM_CACHE_TYPE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_GLOBAL_MEM_CACHELINE_SIZE).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_GLOBAL_MEM_CACHELINE_SIZE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_GLOBAL_MEM_CACHE_SIZE).unwrap();
        let value = value.to_ulong();
        println!("CL_DEVICE_GLOBAL_MEM_CACHE_SIZE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_GLOBAL_MEM_SIZE).unwrap();
        let value = value.to_ulong();
        println!("CL_DEVICE_GLOBAL_MEM_SIZE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_MAX_CONSTANT_BUFFER_SIZE).unwrap();
        let value = value.to_ulong();
        println!("CL_DEVICE_MAX_CONSTANT_BUFFER_SIZE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_MAX_CONSTANT_ARGS).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_MAX_CONSTANT_ARGS: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_LOCAL_MEM_TYPE).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_LOCAL_MEM_TYPE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_LOCAL_MEM_SIZE).unwrap();
        let value = value.to_ulong();
        println!("CL_DEVICE_LOCAL_MEM_SIZE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_ERROR_CORRECTION_SUPPORT).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_ERROR_CORRECTION_SUPPORT: {}", value);
        // assert!(0 == value);

        let value = get_device_info(device_id, CL_DEVICE_PROFILING_TIMER_RESOLUTION).unwrap();
        let value = value.to_size();
        println!("CL_DEVICE_PROFILING_TIMER_RESOLUTION: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_ENDIAN_LITTLE).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_ENDIAN_LITTLE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_AVAILABLE).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_AVAILABLE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_COMPILER_AVAILABLE).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_COMPILER_AVAILABLE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_EXECUTION_CAPABILITIES).unwrap();
        let value = value.to_ulong();
        println!("CL_DEVICE_EXECUTION_CAPABILITIES: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_QUEUE_ON_HOST_PROPERTIES).unwrap();
        let value = value.to_ulong();
        println!("CL_DEVICE_QUEUE_ON_HOST_PROPERTIES: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_NAME).unwrap();
        let value = value.to_str().unwrap();
        println!("CL_DEVICE_NAME: {:?}", value);
        let value = value.into_string().unwrap();
        assert!(0 < value.len());

        let value = get_device_info(device_id, CL_DEVICE_VENDOR).unwrap();
        let value = value.to_str().unwrap();
        println!("CL_DEVICE_VENDOR: {:?}", value);
        let value = value.into_string().unwrap();
        assert!(0 < value.len());

        let value = get_device_info(device_id, CL_DRIVER_VERSION).unwrap();
        let value = value.to_str().unwrap();
        println!("CL_DRIVER_VERSION: {:?}", value);
        let value = value.into_string().unwrap();
        assert!(0 < value.len());

        let value = get_device_info(device_id, CL_DEVICE_PROFILE).unwrap();
        let value = value.to_str().unwrap();
        println!("CL_DEVICE_PROFILE: {:?}", value);
        let value = value.into_string().unwrap();
        assert!(0 < value.len());

        let value = get_device_info(device_id, CL_DEVICE_VERSION).unwrap();
        let value = value.to_str().unwrap();
        println!("CL_DEVICE_VERSION: {:?}", value);
        let value = value.into_string().unwrap();
        assert!(0 < value.len());

        let value = get_device_info(device_id, CL_DEVICE_EXTENSIONS).unwrap();
        let value = value.to_str().unwrap();
        println!("CL_DEVICE_EXTENSIONS: {:?}", value);
        let value = value.into_string().unwrap();
        assert!(0 < value.len());

        let value = get_device_info(device_id, CL_DEVICE_PLATFORM).unwrap();
        let value = value.to_ptr();
        println!("CL_DEVICE_PLATFORM: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_DOUBLE_FP_CONFIG).unwrap();
        let value = value.to_ulong();
        println!("CL_DEVICE_DOUBLE_FP_CONFIG: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_HALF_FP_CONFIG).unwrap();
        let value = value.to_ulong();
        println!("CL_DEVICE_HALF_FP_CONFIG: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_PREFERRED_VECTOR_WIDTH_HALF).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_PREFERRED_VECTOR_WIDTH_HALF: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_NATIVE_VECTOR_WIDTH_CHAR).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_NATIVE_VECTOR_WIDTH_CHAR: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_NATIVE_VECTOR_WIDTH_SHORT).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_NATIVE_VECTOR_WIDTH_SHORT: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_NATIVE_VECTOR_WIDTH_INT).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_NATIVE_VECTOR_WIDTH_INT: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_NATIVE_VECTOR_WIDTH_LONG).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_NATIVE_VECTOR_WIDTH_LONG: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_NATIVE_VECTOR_WIDTH_FLOAT).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_NATIVE_VECTOR_WIDTH_FLOAT: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_NATIVE_VECTOR_WIDTH_DOUBLE).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_NATIVE_VECTOR_WIDTH_DOUBLE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_NATIVE_VECTOR_WIDTH_HALF).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_NATIVE_VECTOR_WIDTH_HALF: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_OPENCL_C_VERSION).unwrap();
        let value = value.to_str().unwrap();
        println!("CL_DEVICE_OPENCL_C_VERSION: {:?}", value);
        let value = value.into_string().unwrap();
        assert!(0 < value.len());

        let value = get_device_info(device_id, CL_DEVICE_LINKER_AVAILABLE).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_LINKER_AVAILABLE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_BUILT_IN_KERNELS).unwrap();
        let value = value.to_str().unwrap();
        println!("CL_DEVICE_BUILT_IN_KERNELS: {:?}", value);
        let value = value.into_string().unwrap();
        assert!(0 < value.len());

        let value = get_device_info(device_id, CL_DEVICE_IMAGE_MAX_BUFFER_SIZE).unwrap();
        let value = value.to_size();
        println!("CL_DEVICE_IMAGE_MAX_BUFFER_SIZE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_IMAGE_MAX_ARRAY_SIZE).unwrap();
        let value = value.to_size();
        println!("CL_DEVICE_IMAGE_MAX_ARRAY_SIZE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_PARENT_DEVICE).unwrap();
        let value = value.to_ptr();
        println!("CL_DEVICE_PARENT_DEVICE: {}", value);
        assert!(0 == value);

        let value = get_device_info(device_id, CL_DEVICE_PARTITION_MAX_SUB_DEVICES).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_PARTITION_MAX_SUB_DEVICES: {}", value);
        assert!(0 == value);

        let value = get_device_info(device_id, CL_DEVICE_PARTITION_PROPERTIES).unwrap();
        let value = value.to_vec_intptr();
        println!("CL_DEVICE_PARTITION_PROPERTIES: {}", value.len());
        println!("CL_DEVICE_PARTITION_PROPERTIES: {:?}", value);
        assert!(0 < value.len());

        let value = get_device_info(device_id, CL_DEVICE_PARTITION_AFFINITY_DOMAIN).unwrap();
        let value = value.to_vec_ulong();
        println!("CL_DEVICE_PARTITION_AFFINITY_DOMAIN: {}", value.len());
        println!("CL_DEVICE_PARTITION_AFFINITY_DOMAIN: {:?}", value);
        assert!(0 < value.len());

        let value = get_device_info(device_id, CL_DEVICE_PARTITION_TYPE).unwrap();
        let value = value.to_vec_intptr();
        println!("CL_DEVICE_PARTITION_TYPE: {}", value.len());
        println!("CL_DEVICE_PARTITION_TYPE: {:?}", value);
        assert!(0 < value.len());

        let value = get_device_info(device_id, CL_DEVICE_REFERENCE_COUNT).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_REFERENCE_COUNT: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_PREFERRED_INTEROP_USER_SYNC).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_PREFERRED_INTEROP_USER_SYNC: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_PRINTF_BUFFER_SIZE).unwrap();
        let value = value.to_size();
        println!("CL_DEVICE_PRINTF_BUFFER_SIZE: {}", value);
        assert!(0 < value);

        // CL_VERSION_2_0
        let value = get_device_info(device_id, CL_DEVICE_IMAGE_PITCH_ALIGNMENT).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_IMAGE_PITCH_ALIGNMENT: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_IMAGE_BASE_ADDRESS_ALIGNMENT).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_IMAGE_BASE_ADDRESS_ALIGNMENT: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_MAX_READ_WRITE_IMAGE_ARGS).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_MAX_READ_WRITE_IMAGE_ARGS: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_MAX_GLOBAL_VARIABLE_SIZE).unwrap();
        let value = value.to_size();
        println!("CL_DEVICE_MAX_GLOBAL_VARIABLE_SIZE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_QUEUE_ON_DEVICE_PROPERTIES).unwrap();
        let value = value.to_vec_intptr();
        println!("CL_DEVICE_QUEUE_ON_DEVICE_PROPERTIES: {}", value.len());
        println!("CL_DEVICE_QUEUE_ON_DEVICE_PROPERTIES: {:?}", value);
        assert!(0 < value.len());

        let value = get_device_info(device_id, CL_DEVICE_QUEUE_ON_DEVICE_PREFERRED_SIZE).unwrap();
        let value = value.to_size();
        println!("CL_DEVICE_QUEUE_ON_DEVICE_PREFERRED_SIZE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_QUEUE_ON_DEVICE_MAX_SIZE).unwrap();
        let value = value.to_size();
        println!("CL_DEVICE_QUEUE_ON_DEVICE_MAX_SIZE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_MAX_ON_DEVICE_QUEUES).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_MAX_ON_DEVICE_QUEUES: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_MAX_ON_DEVICE_EVENTS).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_MAX_ON_DEVICE_EVENTS: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_SVM_CAPABILITIES).unwrap();
        let value = value.to_ulong();
        println!("CL_DEVICE_SVM_CAPABILITIES: {}", value);
        assert!(0 < value);

        let value =
            get_device_info(device_id, CL_DEVICE_GLOBAL_VARIABLE_PREFERRED_TOTAL_SIZE).unwrap();
        let value = value.to_size();
        println!("CL_DEVICE_GLOBAL_VARIABLE_PREFERRED_TOTAL_SIZE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_MAX_PIPE_ARGS).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_MAX_PIPE_ARGS: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_PIPE_MAX_ACTIVE_RESERVATIONS).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_PIPE_MAX_ACTIVE_RESERVATIONS: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_PIPE_MAX_PACKET_SIZE).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_PIPE_MAX_PACKET_SIZE: {}", value);
        assert!(0 < value);

        let value =
            get_device_info(device_id, CL_DEVICE_PREFERRED_PLATFORM_ATOMIC_ALIGNMENT).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_PREFERRED_PLATFORM_ATOMIC_ALIGNMENT: {}", value);
        assert!(0 < value);

        let value =
            get_device_info(device_id, CL_DEVICE_PREFERRED_GLOBAL_ATOMIC_ALIGNMENT).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_PREFERRED_GLOBAL_ATOMIC_ALIGNMENT: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_PREFERRED_LOCAL_ATOMIC_ALIGNMENT).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_PREFERRED_LOCAL_ATOMIC_ALIGNMENT: {}", value);
        assert!(0 < value);

        // CL_VERSION_2_1
        let value = get_device_info(device_id, CL_DEVICE_IL_VERSION).unwrap();
        let value = value.to_str().unwrap();
        println!("CL_DEVICE_IL_VERSION: {:?}", value);
        let value = value.into_string().unwrap();
        assert!(0 < value.len());

        let value = get_device_info(device_id, CL_DEVICE_MAX_NUM_SUB_GROUPS).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_MAX_NUM_SUB_GROUPS: {}", value);
        assert!(0 < value);

        let value =
            get_device_info(device_id, CL_DEVICE_SUB_GROUP_INDEPENDENT_FORWARD_PROGRESS).unwrap();
        let value = value.to_uint();
        println!(
            "CL_DEVICE_SUB_GROUP_INDEPENDENT_FORWARD_PROGRESS: {}",
            value
        );
        assert!(0 < value);
    }

    #[test]
    #[cfg(feature = "CL_VERSION_3_0")]
    fn test_get_device_info_3_0() {
        let platform_ids = get_platform_ids().unwrap();

        // Choose the platform with the most compliant GPU
        let platform_id = platform_ids[1];

        let device_ids = get_device_ids(platform_id, CL_DEVICE_TYPE_GPU).unwrap();
        println!("CL_DEVICE_TYPE_GPU count: {}", device_ids.len());
        assert!(0 < device_ids.len());

        let device_id = device_ids[0];

        // CL_VERSION_3_0
        let value = get_device_info(device_id, CL_DEVICE_NUMERIC_VERSION).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_MAX_NUM_SUB_GROUPS: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_EXTENSIONS_WITH_VERSION).unwrap();
        let value = value.to_vec_name_version();
        println!("CL_DEVICE_EXTENSIONS_WITH_VERSION: {}", value.len());
        println!("CL_DEVICE_EXTENSIONS_WITH_VERSION: {:?}", value);
        assert!(0 < value.len());

        let value = get_device_info(device_id, CL_DEVICE_ILS_WITH_VERSION).unwrap();
        let value = value.to_vec_name_version();
        println!("CL_DEVICE_ILS_WITH_VERSION: {}", value.len());
        println!("CL_DEVICE_ILS_WITH_VERSION: {:?}", value);
        assert!(0 < value.len());

        let value = get_device_info(device_id, CL_DEVICE_BUILT_IN_KERNELS_WITH_VERSION).unwrap();
        let value = value.to_vec_name_version();
        println!("CL_DEVICE_BUILT_IN_KERNELS_WITH_VERSION: {}", value.len());
        println!("CL_DEVICE_BUILT_IN_KERNELS_WITH_VERSION: {:?}", value);
        assert!(0 < value.len());

        let value = get_device_info(device_id, CL_DEVICE_ATOMIC_MEMORY_CAPABILITIES).unwrap();
        let value = value.to_ulong();
        println!("CL_DEVICE_ATOMIC_MEMORY_CAPABILITIES: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_ATOMIC_MEMORY_CAPABILITIES).unwrap();
        let value = value.to_ulong();
        println!("CL_DEVICE_ATOMIC_FENCE_CAPABILITIES: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_NON_UNIFORM_WORK_GROUP_SUPPORT).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_NON_UNIFORM_WORK_GROUP_SUPPORT: {}", value);
        // assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_OPENCL_C_ALL_VERSIONS).unwrap();
        let value = value.to_vec_name_version();
        println!("CL_DEVICE_OPENCL_C_ALL_VERSIONS: {}", value.len());
        println!("CL_DEVICE_OPENCL_C_ALL_VERSIONS: {:?}", value);
        assert!(0 < value.len());

        let value = get_device_info(device_id, CL_DEVICE_PREFERRED_WORK_GROUP_SIZE_MULTIPLE).unwrap();
        let value = value.to_size();
        println!("CL_DEVICE_PREFERRED_WORK_GROUP_SIZE_MULTIPLE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_WORK_GROUP_COLLECTIVE_FUNCTIONS_SUPPORT).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_WORK_GROUP_COLLECTIVE_FUNCTIONS_SUPPORT: {}", value);
        // assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_GENERIC_ADDRESS_SPACE_SUPPORT).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_GENERIC_ADDRESS_SPACE_SUPPORT: {}", value);
        // assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_OPENCL_C_FEATURES).unwrap();
        let value = value.to_vec_name_version();
        println!("CL_DEVICE_OPENCL_C_FEATURES: {}", value.len());
        println!("CL_DEVICE_OPENCL_C_FEATURES: {:?}", value);
        assert!(0 < value.len());

        let value = get_device_info(device_id, CL_DEVICE_DEVICE_ENQUEUE_CAPABILITIES).unwrap();
        let value = value.to_ulong();
        println!("CL_DEVICE_DEVICE_ENQUEUE_CAPABILITIES: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_PIPE_SUPPORT).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_PIPE_SUPPORT: {}", value);
        // assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_LATEST_CONFORMANCE_VERSION_PASSED).unwrap();
        let value = value.to_str().unwrap();
        println!("CL_DEVICE_LATEST_CONFORMANCE_VERSION_PASSED: {:?}", value);
        let value = value.into_string().unwrap();
        assert!(0 < value.len());
    }
}
