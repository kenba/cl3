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

#![allow(non_camel_case_types)]

use super::error_codes::{CL_DEVICE_NOT_FOUND, CL_SUCCESS};

pub use cl_sys::{
    CL_DEVICE_TYPE_DEFAULT, CL_DEVICE_TYPE_CPU,
    CL_DEVICE_TYPE_GPU, CL_DEVICE_TYPE_ACCELERATOR,CL_DEVICE_TYPE_CUSTOM, CL_DEVICE_TYPE_ALL,
    CL_FP_DENORM, CL_FP_INF_NAN, CL_FP_ROUND_TO_NEAREST, CL_FP_ROUND_TO_ZERO,
    CL_FP_ROUND_TO_INF, CL_FP_FMA, CL_FP_SOFT_FLOAT, CL_FP_CORRECTLY_ROUNDED_DIVIDE_SQRT,
    CL_NONE, CL_READ_ONLY_CACHE, CL_READ_WRITE_CACHE, CL_LOCAL, CL_GLOBAL,
    CL_EXEC_KERNEL, CL_EXEC_NATIVE_KERNEL, 
    CL_DEVICE_AFFINITY_DOMAIN_NUMA, CL_DEVICE_AFFINITY_DOMAIN_L4_CACHE,
    CL_DEVICE_AFFINITY_DOMAIN_L3_CACHE, CL_DEVICE_AFFINITY_DOMAIN_L2_CACHE,
    CL_DEVICE_AFFINITY_DOMAIN_L1_CACHE, CL_DEVICE_AFFINITY_DOMAIN_NEXT_PARTITIONABLE,
    CL_DEVICE_SVM_COARSE_GRAIN_BUFFER, CL_DEVICE_SVM_FINE_GRAIN_BUFFER,
    CL_DEVICE_SVM_FINE_GRAIN_SYSTEM, CL_DEVICE_SVM_ATOMICS,
};

use super::info_type::InfoType;
#[allow(unused_imports)]
use super::types::{
    cl_command_queue, cl_context, cl_device_affinity_domain, cl_device_exec_capabilities,
    cl_device_fp_config, cl_device_id, cl_device_info, cl_device_local_mem_type,
    cl_device_mem_cache_type, cl_device_partition_property, cl_device_svm_capabilities,
    cl_device_type, cl_int, cl_name_version, cl_platform_id, cl_uint, cl_ulong,
};
use super::{api_info_size, api_info_value, api_info_vector};
#[allow(unused_imports)]
use cl_sys::{
    clCreateSubDevices, clGetDeviceIDs, clGetDeviceInfo, clReleaseDevice, clRetainDevice, 
    clSetDefaultDeviceCommandQueue, 
};

// clGetDeviceAndHostTimer, clGetHostTimer, are incorrect in cl_sys
#[cfg_attr(not(target_os = "macos"), link(name = "OpenCL"))]
#[cfg_attr(target_os = "macos", link(name = "OpenCL", kind = "framework"))]
extern "system" {
    pub fn clGetDeviceAndHostTimer(
        device: cl_device_id,
        device_timestamp: *mut cl_ulong,
        host_timestamp: *mut cl_ulong,
    ) -> cl_int;

    pub fn clGetHostTimer(device: cl_device_id, host_timestamp: *mut cl_ulong) -> cl_int;
}

use libc::{c_void, intptr_t, size_t};
use std::mem;
use std::ptr;

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

// cl_device_info
#[derive(Clone, Copy, Debug)]
pub enum DeviceInfo {
    CL_DEVICE_TYPE = 0x1000,
    CL_DEVICE_VENDOR_ID = 0x1001,
    CL_DEVICE_MAX_COMPUTE_UNITS = 0x1002,
    CL_DEVICE_MAX_WORK_ITEM_DIMENSIONS = 0x1003,
    CL_DEVICE_MAX_WORK_GROUP_SIZE = 0x1004,
    CL_DEVICE_MAX_WORK_ITEM_SIZES = 0x1005,
    CL_DEVICE_PREFERRED_VECTOR_WIDTH_CHAR = 0x1006,
    CL_DEVICE_PREFERRED_VECTOR_WIDTH_SHORT = 0x1007,
    CL_DEVICE_PREFERRED_VECTOR_WIDTH_INT = 0x1008,
    CL_DEVICE_PREFERRED_VECTOR_WIDTH_LONG = 0x1009,
    CL_DEVICE_PREFERRED_VECTOR_WIDTH_FLOAT = 0x100A,
    CL_DEVICE_PREFERRED_VECTOR_WIDTH_DOUBLE = 0x100B,
    CL_DEVICE_MAX_CLOCK_FREQUENCY = 0x100C,
    CL_DEVICE_ADDRESS_BITS = 0x100D,
    CL_DEVICE_MAX_READ_IMAGE_ARGS = 0x100E,
    CL_DEVICE_MAX_WRITE_IMAGE_ARGS = 0x100F,
    CL_DEVICE_MAX_MEM_ALLOC_SIZE = 0x1010,
    CL_DEVICE_IMAGE2D_MAX_WIDTH = 0x1011,
    CL_DEVICE_IMAGE2D_MAX_HEIGHT = 0x1012,
    CL_DEVICE_IMAGE3D_MAX_WIDTH = 0x1013,
    CL_DEVICE_IMAGE3D_MAX_HEIGHT = 0x1014,
    CL_DEVICE_IMAGE3D_MAX_DEPTH = 0x1015,
    CL_DEVICE_IMAGE_SUPPORT = 0x1016,
    CL_DEVICE_MAX_PARAMETER_SIZE = 0x1017,
    CL_DEVICE_MAX_SAMPLERS = 0x1018,
    CL_DEVICE_MEM_BASE_ADDR_ALIGN = 0x1019,
    CL_DEVICE_MIN_DATA_TYPE_ALIGN_SIZE = 0x101A,
    CL_DEVICE_SINGLE_FP_CONFIG = 0x101B,
    CL_DEVICE_GLOBAL_MEM_CACHE_TYPE = 0x101C,
    CL_DEVICE_GLOBAL_MEM_CACHELINE_SIZE = 0x101D,
    CL_DEVICE_GLOBAL_MEM_CACHE_SIZE = 0x101E,
    CL_DEVICE_GLOBAL_MEM_SIZE = 0x101F,
    CL_DEVICE_MAX_CONSTANT_BUFFER_SIZE = 0x1020,
    CL_DEVICE_MAX_CONSTANT_ARGS = 0x1021,
    CL_DEVICE_LOCAL_MEM_TYPE = 0x1022,
    CL_DEVICE_LOCAL_MEM_SIZE = 0x1023,
    CL_DEVICE_ERROR_CORRECTION_SUPPORT = 0x1024,
    CL_DEVICE_PROFILING_TIMER_RESOLUTION = 0x1025,
    CL_DEVICE_ENDIAN_LITTLE = 0x1026,
    CL_DEVICE_AVAILABLE = 0x1027,
    CL_DEVICE_COMPILER_AVAILABLE = 0x1028,
    CL_DEVICE_EXECUTION_CAPABILITIES = 0x1029,
    // CL_DEVICE_QUEUE_PROPERTIES = 0x102A, // DEPRECATED 2.0
    // CL_VERSION_2_0
    CL_DEVICE_QUEUE_ON_HOST_PROPERTIES = 0x102A,
    CL_DEVICE_NAME = 0x102B,
    CL_DEVICE_VENDOR = 0x102C,
    CL_DRIVER_VERSION = 0x102D,
    CL_DEVICE_PROFILE = 0x102E,
    CL_DEVICE_VERSION = 0x102F,
    CL_DEVICE_EXTENSIONS = 0x1030,
    CL_DEVICE_PLATFORM = 0x1031,
    // CL_VERSION_1_2
    CL_DEVICE_DOUBLE_FP_CONFIG = 0x1032,
    CL_DEVICE_HALF_FP_CONFIG = 0x1033, // defined in "cl_ext.h
    // #ifdef CL_VERSION_1_1
    CL_DEVICE_PREFERRED_VECTOR_WIDTH_HALF = 0x1034,
    CL_DEVICE_HOST_UNIFIED_MEMORY = 0x1035, // DEPRECATED 2.0
    CL_DEVICE_NATIVE_VECTOR_WIDTH_CHAR = 0x1036,
    CL_DEVICE_NATIVE_VECTOR_WIDTH_SHORT = 0x1037,
    CL_DEVICE_NATIVE_VECTOR_WIDTH_INT = 0x1038,
    CL_DEVICE_NATIVE_VECTOR_WIDTH_LONG = 0x1039,
    CL_DEVICE_NATIVE_VECTOR_WIDTH_FLOAT = 0x103A,
    CL_DEVICE_NATIVE_VECTOR_WIDTH_DOUBLE = 0x103B,
    CL_DEVICE_NATIVE_VECTOR_WIDTH_HALF = 0x103C,
    CL_DEVICE_OPENCL_C_VERSION = 0x103D, // DEPRECATED 3.0

    // #endif
    // #ifdef CL_VERSION_1_2
    CL_DEVICE_LINKER_AVAILABLE = 0x103E,
    CL_DEVICE_BUILT_IN_KERNELS = 0x103F,
    CL_DEVICE_IMAGE_MAX_BUFFER_SIZE = 0x1040,
    CL_DEVICE_IMAGE_MAX_ARRAY_SIZE = 0x1041,
    CL_DEVICE_PARENT_DEVICE = 0x1042,
    CL_DEVICE_PARTITION_MAX_SUB_DEVICES = 0x1043,
    CL_DEVICE_PARTITION_PROPERTIES = 0x1044,
    CL_DEVICE_PARTITION_AFFINITY_DOMAIN = 0x1045,
    CL_DEVICE_PARTITION_TYPE = 0x1046,
    CL_DEVICE_REFERENCE_COUNT = 0x1047,
    CL_DEVICE_PREFERRED_INTEROP_USER_SYNC = 0x1048,
    CL_DEVICE_PRINTF_BUFFER_SIZE = 0x1049,
    // #endif
    // #ifdef CL_VERSION_2_0
    CL_DEVICE_IMAGE_PITCH_ALIGNMENT = 0x104A,
    CL_DEVICE_IMAGE_BASE_ADDRESS_ALIGNMENT = 0x104B,
    CL_DEVICE_MAX_READ_WRITE_IMAGE_ARGS = 0x104C,
    CL_DEVICE_MAX_GLOBAL_VARIABLE_SIZE = 0x104D,
    CL_DEVICE_QUEUE_ON_DEVICE_PROPERTIES = 0x104E,
    CL_DEVICE_QUEUE_ON_DEVICE_PREFERRED_SIZE = 0x104F,
    CL_DEVICE_QUEUE_ON_DEVICE_MAX_SIZE = 0x1050,
    CL_DEVICE_MAX_ON_DEVICE_QUEUES = 0x1051,
    CL_DEVICE_MAX_ON_DEVICE_EVENTS = 0x1052,
    CL_DEVICE_SVM_CAPABILITIES = 0x1053,
    CL_DEVICE_GLOBAL_VARIABLE_PREFERRED_TOTAL_SIZE = 0x1054,
    CL_DEVICE_MAX_PIPE_ARGS = 0x1055,
    CL_DEVICE_PIPE_MAX_ACTIVE_RESERVATIONS = 0x1056,
    CL_DEVICE_PIPE_MAX_PACKET_SIZE = 0x1057,
    CL_DEVICE_PREFERRED_PLATFORM_ATOMIC_ALIGNMENT = 0x1058,
    CL_DEVICE_PREFERRED_GLOBAL_ATOMIC_ALIGNMENT = 0x1059,
    CL_DEVICE_PREFERRED_LOCAL_ATOMIC_ALIGNMENT = 0x105A,
    // #endif
    // #ifdef CL_VERSION_2_1
    CL_DEVICE_IL_VERSION = 0x105B,
    CL_DEVICE_MAX_NUM_SUB_GROUPS = 0x105C,
    CL_DEVICE_SUB_GROUP_INDEPENDENT_FORWARD_PROGRESS = 0x105D,
    // #endif
    // #ifdef CL_VERSION_3_0
    CL_DEVICE_NUMERIC_VERSION = 0x105E,
    CL_DEVICE_EXTENSIONS_WITH_VERSION = 0x1060,
    CL_DEVICE_ILS_WITH_VERSION = 0x1061,
    CL_DEVICE_BUILT_IN_KERNELS_WITH_VERSION = 0x1062,
    CL_DEVICE_ATOMIC_MEMORY_CAPABILITIES = 0x1063,
    CL_DEVICE_ATOMIC_FENCE_CAPABILITIES = 0x1064,
    CL_DEVICE_NON_UNIFORM_WORK_GROUP_SUPPORT = 0x1065,
    CL_DEVICE_OPENCL_C_ALL_VERSIONS = 0x1066,
    CL_DEVICE_PREFERRED_WORK_GROUP_SIZE_MULTIPLE = 0x1067,
    CL_DEVICE_WORK_GROUP_COLLECTIVE_FUNCTIONS_SUPPORT = 0x1068,
    CL_DEVICE_GENERIC_ADDRESS_SPACE_SUPPORT = 0x1069,
    // 0x106A to 0x106E - Reserved for upcoming KHR extension
    CL_DEVICE_OPENCL_C_FEATURES = 0x106F,
    CL_DEVICE_DEVICE_ENQUEUE_CAPABILITIES = 0x1070,
    CL_DEVICE_PIPE_SUPPORT = 0x1071,
    CL_DEVICE_LATEST_CONFORMANCE_VERSION_PASSED = 0x1072,
    // #endif
}

/// Get specific information about an OpenCL device.  
/// Calls clGetDeviceInfo to get the desired information about the device.
///  # Examples
/// ```
/// use cl3::platform::get_platform_ids;
/// use cl3::device::{get_device_ids, get_device_info, CL_DEVICE_TYPE_GPU, DeviceInfo,};
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
/// let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_TYPE).unwrap();
/// let value = value.to_ulong();
/// println!("CL_DEVICE_TYPE: {}", value);
/// assert_eq!(CL_DEVICE_TYPE_GPU, value);
///
/// let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_VENDOR).unwrap();
/// let value = value.to_str().unwrap();
/// println!("CL_DEVICE_VENDOR: {:?}", value);
/// assert!(0 < value.to_bytes().len());
///
/// let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_VERSION).unwrap();
/// let value = value.to_str().unwrap();
/// println!("CL_DEVICE_VERSION: {:?}", value);
/// assert!(0 < value.to_bytes().len());
/// ```
/// * `device` - the cl_device_id of the OpenCL device.
/// * `param_name` - the type of device information being queried, see
/// [Device Queries](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#device-queries-table).
///
/// returns a Result containing the desired information in an InfoType enum
/// or the error code from the OpenCL C API function.
pub fn get_device_info(device: cl_device_id, param_name: DeviceInfo) -> Result<InfoType, cl_int> {
    api_info_size!(get_size, clGetDeviceInfo);

    let param_id = param_name as cl_device_info;
    match param_name {
        DeviceInfo::CL_DEVICE_NAME
        | DeviceInfo::CL_DEVICE_VENDOR
        | DeviceInfo::CL_DRIVER_VERSION
        | DeviceInfo::CL_DEVICE_PROFILE
        | DeviceInfo::CL_DEVICE_VERSION
        | DeviceInfo::CL_DEVICE_EXTENSIONS
        | DeviceInfo::CL_DEVICE_OPENCL_C_VERSION
        | DeviceInfo::CL_DEVICE_BUILT_IN_KERNELS
        | DeviceInfo::CL_DEVICE_IL_VERSION
        | DeviceInfo::CL_DEVICE_LATEST_CONFORMANCE_VERSION_PASSED // CL_VERSION_3_0
        => {
            api_info_vector!(get_string, u8, clGetDeviceInfo);
            let size = get_size(device, param_id)?;
            Ok(InfoType::Str(get_string(device, param_id, size)?))
        }

        DeviceInfo::CL_DEVICE_VENDOR_ID
        | DeviceInfo::CL_DEVICE_MAX_COMPUTE_UNITS
        | DeviceInfo::CL_DEVICE_MAX_WORK_ITEM_DIMENSIONS
        | DeviceInfo::CL_DEVICE_PREFERRED_VECTOR_WIDTH_CHAR
        | DeviceInfo::CL_DEVICE_PREFERRED_VECTOR_WIDTH_SHORT
        | DeviceInfo::CL_DEVICE_PREFERRED_VECTOR_WIDTH_INT
        | DeviceInfo::CL_DEVICE_PREFERRED_VECTOR_WIDTH_LONG
        | DeviceInfo::CL_DEVICE_PREFERRED_VECTOR_WIDTH_FLOAT
        | DeviceInfo::CL_DEVICE_PREFERRED_VECTOR_WIDTH_DOUBLE
        | DeviceInfo::CL_DEVICE_MAX_CLOCK_FREQUENCY
        | DeviceInfo::CL_DEVICE_ADDRESS_BITS
        | DeviceInfo::CL_DEVICE_MAX_READ_IMAGE_ARGS
        | DeviceInfo::CL_DEVICE_MAX_WRITE_IMAGE_ARGS
        | DeviceInfo::CL_DEVICE_IMAGE_SUPPORT
        | DeviceInfo::CL_DEVICE_MAX_SAMPLERS
        | DeviceInfo::CL_DEVICE_MEM_BASE_ADDR_ALIGN
        | DeviceInfo::CL_DEVICE_MIN_DATA_TYPE_ALIGN_SIZE
        | DeviceInfo::CL_DEVICE_GLOBAL_MEM_CACHE_TYPE
        | DeviceInfo::CL_DEVICE_GLOBAL_MEM_CACHELINE_SIZE
        | DeviceInfo::CL_DEVICE_MAX_CONSTANT_ARGS
        | DeviceInfo::CL_DEVICE_LOCAL_MEM_TYPE
        | DeviceInfo::CL_DEVICE_ERROR_CORRECTION_SUPPORT
        | DeviceInfo::CL_DEVICE_ENDIAN_LITTLE
        | DeviceInfo::CL_DEVICE_AVAILABLE
        | DeviceInfo::CL_DEVICE_COMPILER_AVAILABLE
        | DeviceInfo::CL_DEVICE_PREFERRED_VECTOR_WIDTH_HALF
        | DeviceInfo::CL_DEVICE_HOST_UNIFIED_MEMORY
        | DeviceInfo::CL_DEVICE_NATIVE_VECTOR_WIDTH_CHAR
        | DeviceInfo::CL_DEVICE_NATIVE_VECTOR_WIDTH_SHORT
        | DeviceInfo::CL_DEVICE_NATIVE_VECTOR_WIDTH_INT
        | DeviceInfo::CL_DEVICE_NATIVE_VECTOR_WIDTH_LONG
        | DeviceInfo::CL_DEVICE_NATIVE_VECTOR_WIDTH_FLOAT
        | DeviceInfo::CL_DEVICE_NATIVE_VECTOR_WIDTH_DOUBLE
        | DeviceInfo::CL_DEVICE_NATIVE_VECTOR_WIDTH_HALF
        | DeviceInfo::CL_DEVICE_LINKER_AVAILABLE
        | DeviceInfo::CL_DEVICE_PARTITION_MAX_SUB_DEVICES
        | DeviceInfo::CL_DEVICE_REFERENCE_COUNT
        | DeviceInfo::CL_DEVICE_PREFERRED_INTEROP_USER_SYNC
        | DeviceInfo::CL_DEVICE_IMAGE_PITCH_ALIGNMENT
        | DeviceInfo::CL_DEVICE_IMAGE_BASE_ADDRESS_ALIGNMENT
        | DeviceInfo::CL_DEVICE_MAX_READ_WRITE_IMAGE_ARGS
        | DeviceInfo::CL_DEVICE_MAX_ON_DEVICE_QUEUES
        | DeviceInfo::CL_DEVICE_MAX_ON_DEVICE_EVENTS
        | DeviceInfo::CL_DEVICE_MAX_PIPE_ARGS
        | DeviceInfo::CL_DEVICE_PIPE_MAX_ACTIVE_RESERVATIONS
        | DeviceInfo::CL_DEVICE_PIPE_MAX_PACKET_SIZE
        | DeviceInfo::CL_DEVICE_PREFERRED_PLATFORM_ATOMIC_ALIGNMENT
        | DeviceInfo::CL_DEVICE_PREFERRED_GLOBAL_ATOMIC_ALIGNMENT
        | DeviceInfo::CL_DEVICE_PREFERRED_LOCAL_ATOMIC_ALIGNMENT
        | DeviceInfo::CL_DEVICE_MAX_NUM_SUB_GROUPS
        | DeviceInfo::CL_DEVICE_SUB_GROUP_INDEPENDENT_FORWARD_PROGRESS
        | DeviceInfo::CL_DEVICE_NUMERIC_VERSION // CL_VERSION_3_0
        | DeviceInfo::CL_DEVICE_NON_UNIFORM_WORK_GROUP_SUPPORT // CL_VERSION_3_0
        | DeviceInfo::CL_DEVICE_WORK_GROUP_COLLECTIVE_FUNCTIONS_SUPPORT // CL_VERSION_3_0
        | DeviceInfo::CL_DEVICE_GENERIC_ADDRESS_SPACE_SUPPORT // CL_VERSION_3_0
        | DeviceInfo::CL_DEVICE_PIPE_SUPPORT // CL_VERSION_3_0
        => {
            api_info_value!(get_value, cl_uint, clGetDeviceInfo);
            Ok(InfoType::Uint(get_value(device, param_id)?))
        }

        DeviceInfo::CL_DEVICE_TYPE
        | DeviceInfo::CL_DEVICE_MAX_MEM_ALLOC_SIZE
        | DeviceInfo::CL_DEVICE_SINGLE_FP_CONFIG
        | DeviceInfo::CL_DEVICE_GLOBAL_MEM_CACHE_SIZE
        | DeviceInfo::CL_DEVICE_GLOBAL_MEM_SIZE
        | DeviceInfo::CL_DEVICE_MAX_CONSTANT_BUFFER_SIZE
        | DeviceInfo::CL_DEVICE_LOCAL_MEM_SIZE
        | DeviceInfo::CL_DEVICE_EXECUTION_CAPABILITIES
        | DeviceInfo::CL_DEVICE_QUEUE_ON_HOST_PROPERTIES
        | DeviceInfo::CL_DEVICE_DOUBLE_FP_CONFIG
        | DeviceInfo::CL_DEVICE_HALF_FP_CONFIG
        | DeviceInfo::CL_DEVICE_SVM_CAPABILITIES
        | DeviceInfo::CL_DEVICE_ATOMIC_MEMORY_CAPABILITIES // CL_VERSION_3_0
        | DeviceInfo::CL_DEVICE_ATOMIC_FENCE_CAPABILITIES // CL_VERSION_3_0
        | DeviceInfo::CL_DEVICE_DEVICE_ENQUEUE_CAPABILITIES // CL_VERSION_3_0
         => {
            api_info_value!(get_value, cl_ulong, clGetDeviceInfo);
            Ok(InfoType::Ulong(get_value(device, param_id)?))
        }

        DeviceInfo::CL_DEVICE_MAX_WORK_GROUP_SIZE
        | DeviceInfo::CL_DEVICE_IMAGE2D_MAX_WIDTH
        | DeviceInfo::CL_DEVICE_IMAGE2D_MAX_HEIGHT
        | DeviceInfo::CL_DEVICE_IMAGE3D_MAX_WIDTH
        | DeviceInfo::CL_DEVICE_IMAGE3D_MAX_HEIGHT
        | DeviceInfo::CL_DEVICE_IMAGE3D_MAX_DEPTH
        | DeviceInfo::CL_DEVICE_MAX_PARAMETER_SIZE
        | DeviceInfo::CL_DEVICE_PROFILING_TIMER_RESOLUTION
        | DeviceInfo::CL_DEVICE_IMAGE_MAX_BUFFER_SIZE
        | DeviceInfo::CL_DEVICE_IMAGE_MAX_ARRAY_SIZE
        | DeviceInfo::CL_DEVICE_PRINTF_BUFFER_SIZE
        | DeviceInfo::CL_DEVICE_MAX_GLOBAL_VARIABLE_SIZE
        | DeviceInfo::CL_DEVICE_QUEUE_ON_DEVICE_PREFERRED_SIZE
        | DeviceInfo::CL_DEVICE_QUEUE_ON_DEVICE_MAX_SIZE
        | DeviceInfo::CL_DEVICE_GLOBAL_VARIABLE_PREFERRED_TOTAL_SIZE 
        | DeviceInfo::CL_DEVICE_PREFERRED_WORK_GROUP_SIZE_MULTIPLE // CL_VERSION_3_0
        => {
            api_info_value!(get_value, size_t, clGetDeviceInfo);
            Ok(InfoType::Size(get_value(device, param_id)?))
        }

        DeviceInfo::CL_DEVICE_PLATFORM | DeviceInfo::CL_DEVICE_PARENT_DEVICE => {
            api_info_value!(get_value, intptr_t, clGetDeviceInfo);
            Ok(InfoType::Ptr(get_value(device, param_id)?))
        }

        DeviceInfo::CL_DEVICE_PARTITION_AFFINITY_DOMAIN => {
            api_info_vector!(get_vec, cl_ulong, clGetDeviceInfo);
            let size = get_size(device, param_id)?;
            Ok(InfoType::VecUlong(get_vec(device, param_id, size)?))
        }

        DeviceInfo::CL_DEVICE_MAX_WORK_ITEM_SIZES => {
            api_info_vector!(get_vec, size_t, clGetDeviceInfo);
            let size = get_size(device, param_id)?;
            Ok(InfoType::VecSize(get_vec(device, param_id, size)?))
        }

        DeviceInfo::CL_DEVICE_PARTITION_PROPERTIES
        | DeviceInfo::CL_DEVICE_PARTITION_TYPE
        | DeviceInfo::CL_DEVICE_QUEUE_ON_DEVICE_PROPERTIES => {
            api_info_vector!(get_vec, intptr_t, clGetDeviceInfo);
            let size = get_size(device, param_id)?;
            Ok(InfoType::VecIntPtr(get_vec(device, param_id, size)?))
        }

        // CL_VERSION_3_0
        DeviceInfo::CL_DEVICE_EXTENSIONS_WITH_VERSION
        | DeviceInfo::CL_DEVICE_ILS_WITH_VERSION
        | DeviceInfo::CL_DEVICE_BUILT_IN_KERNELS_WITH_VERSION 
        | DeviceInfo::CL_DEVICE_OPENCL_C_ALL_VERSIONS
        | DeviceInfo::CL_DEVICE_OPENCL_C_FEATURES => {
            api_info_vector!(get_vec, cl_name_version, clGetDeviceInfo);
            let size = get_size(device, param_id)?;
            Ok(InfoType::VecNameVersion(get_vec(device, param_id, size)?))
        }

        // _ => Err(CL_INVALID_VALUE),
    }
}

// cl_device_partition_property:
pub const CL_DEVICE_PARTITION_EQUALLY: cl_device_partition_property = 0x1086;
pub const CL_DEVICE_PARTITION_BY_COUNTS: cl_device_partition_property = 0x1087;
pub const CL_DEVICE_PARTITION_BY_COUNTS_LIST_END: cl_device_partition_property = 0x0;
pub const CL_DEVICE_PARTITION_BY_AFFINITY_DOMAIN: cl_device_partition_property = 0x1088;

// helper function for create_sub_devices
#[inline]
fn count_sub_devices(
    in_device: cl_device_id,
    properties: &[cl_device_partition_property],
) -> Result<cl_uint, cl_int> {
    let mut count: cl_uint = 0;
    let status: cl_int = unsafe {
        clCreateSubDevices(
            in_device,
            properties.as_ptr(),
            0,
            ptr::null_mut(),
            &mut count,
        )
    };
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
#[inline]
pub fn create_sub_devices(
    in_device: cl_device_id,
    properties: &[cl_device_partition_property],
) -> Result<Vec<cl_device_id>, cl_int> {
    // get the number of partitions
    let num_devices: cl_uint = count_sub_devices(in_device, properties)?;

    // partition in_device
    let mut ids: Vec<cl_device_id> = Vec::with_capacity(num_devices as usize);
    let status: cl_int = unsafe {
        ids.set_len(num_devices as usize);
        clCreateSubDevices(
            in_device,
            properties.as_ptr(),
            num_devices * mem::size_of::<cl_device_id>() as cl_uint,
            ids.as_mut_ptr(),
            ptr::null_mut(),
        )
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
#[inline]
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
#[inline]
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
#[inline]
pub fn set_default_device_command_queue(
    context: cl_context,
    device: cl_device_id,
    command_queue: cl_command_queue,
) -> Result<(), cl_int> {
    let status: cl_int = unsafe { clSetDefaultDeviceCommandQueue(context, device, command_queue) };
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
#[inline]
pub fn get_device_and_host_timer(device: cl_device_id) -> Result<[cl_ulong; 2], cl_int> {
    let mut device_timestamp: cl_ulong = 0;
    let mut host_timestamp: cl_ulong = 0;
    let status: cl_int =
        unsafe { clGetDeviceAndHostTimer(device, &mut device_timestamp, &mut host_timestamp) };
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
#[inline]
pub fn get_host_timer(device: cl_device_id) -> Result<cl_ulong, cl_int> {
    let mut host_timestamp: cl_ulong = 0;
    let status: cl_int = unsafe { clGetHostTimer(device, &mut host_timestamp) };
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
    use crate::error_codes::error_text;

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

        // Choose the first platform
        let platform_id = platform_ids[0];

        let device_ids = get_device_ids(platform_id, CL_DEVICE_TYPE_GPU).unwrap();
        println!("CL_DEVICE_TYPE_GPU count: {}", device_ids.len());
        assert!(0 < device_ids.len());

        let device_id = device_ids[0];

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_TYPE).unwrap();
        let value = value.to_ulong();
        println!("CL_DEVICE_TYPE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_VENDOR_ID).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_VENDOR_ID: {:X}", value);
        assert!(0 < value);

        let vendor_text = match value {
            0x1002 => "AMD",
            0x10DE => "Nvidia",
            0x8086 => "Intel",
            _ => "unknown",
        };
        println!("Device vendor is: {}", vendor_text);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_VERSION).unwrap();
        let value = value.to_str().unwrap();
        println!("CL_DEVICE_VERSION: {:?}", value);
        let value = value.into_string().unwrap();
        assert!(0 < value.len());

        let opencl_2: String = "OpenCL 2".to_string();
        let is_opencl_2: bool = value.contains(&opencl_2);

        let opencl_2_1: String = "OpenCL 2.1".to_string();
        let is_opencl_2_1: bool = value.contains(&opencl_2_1);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_MAX_COMPUTE_UNITS).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_MAX_COMPUTE_UNITS: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_MAX_WORK_ITEM_DIMENSIONS).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_MAX_WORK_ITEM_DIMENSIONS: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_MAX_WORK_GROUP_SIZE).unwrap();
        let value = value.to_size();
        println!("CL_DEVICE_MAX_WORK_GROUP_SIZE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_MAX_WORK_ITEM_SIZES).unwrap();
        let value = value.to_vec_size();
        println!("CL_DEVICE_MAX_WORK_ITEM_SIZES len: {:?}", value.len());
        println!("CL_DEVICE_MAX_WORK_ITEM_SIZES: {:?}", value);
        assert!(0 < value.len());

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_PREFERRED_VECTOR_WIDTH_CHAR).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_PREFERRED_VECTOR_WIDTH_CHAR: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_PREFERRED_VECTOR_WIDTH_SHORT).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_PREFERRED_VECTOR_WIDTH_SHORT: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_PREFERRED_VECTOR_WIDTH_INT).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_PREFERRED_VECTOR_WIDTH_INT: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_PREFERRED_VECTOR_WIDTH_LONG).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_PREFERRED_VECTOR_WIDTH_LONG: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_PREFERRED_VECTOR_WIDTH_FLOAT).unwrap();
        let value = value.to_uint();
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_PREFERRED_VECTOR_WIDTH_DOUBLE).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_PREFERRED_VECTOR_WIDTH_DOUBLE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_MAX_CLOCK_FREQUENCY).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_MAX_CLOCK_FREQUENCY: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_ADDRESS_BITS).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_ADDRESS_BITS: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_MAX_READ_IMAGE_ARGS).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_MAX_READ_IMAGE_ARGS: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_MAX_WRITE_IMAGE_ARGS).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_MAX_WRITE_IMAGE_ARGS: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_MAX_MEM_ALLOC_SIZE).unwrap();
        let value = value.to_ulong();
        println!("CL_DEVICE_MAX_MEM_ALLOC_SIZE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_IMAGE2D_MAX_WIDTH).unwrap();
        let value = value.to_size();
        println!("CL_DEVICE_IMAGE2D_MAX_WIDTH: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_IMAGE2D_MAX_HEIGHT).unwrap();
        let value = value.to_size();
        println!("CL_DEVICE_IMAGE2D_MAX_HEIGHT: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_IMAGE3D_MAX_WIDTH).unwrap();
        let value = value.to_size();
        println!("CL_DEVICE_IMAGE3D_MAX_WIDTH: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_IMAGE3D_MAX_HEIGHT).unwrap();
        let value = value.to_size();
        println!("CL_DEVICE_IMAGE3D_MAX_HEIGHT: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_IMAGE3D_MAX_DEPTH).unwrap();
        let value = value.to_size();
        println!("CL_DEVICE_IMAGE3D_MAX_DEPTH: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_IMAGE_SUPPORT).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_IMAGE_SUPPORT: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_MAX_PARAMETER_SIZE).unwrap();
        let value = value.to_size();
        println!("CL_DEVICE_MAX_PARAMETER_SIZE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_MAX_SAMPLERS).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_MAX_SAMPLERS: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_MEM_BASE_ADDR_ALIGN).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_MEM_BASE_ADDR_ALIGN: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_MIN_DATA_TYPE_ALIGN_SIZE).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_MIN_DATA_TYPE_ALIGN_SIZE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_SINGLE_FP_CONFIG).unwrap();
        let value = value.to_ulong();
        println!("CL_DEVICE_SINGLE_FP_CONFIG: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_GLOBAL_MEM_CACHE_TYPE).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_GLOBAL_MEM_CACHE_TYPE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_GLOBAL_MEM_CACHELINE_SIZE).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_GLOBAL_MEM_CACHELINE_SIZE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_GLOBAL_MEM_CACHE_SIZE).unwrap();
        let value = value.to_ulong();
        println!("CL_DEVICE_GLOBAL_MEM_CACHE_SIZE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_GLOBAL_MEM_SIZE).unwrap();
        let value = value.to_ulong();
        println!("CL_DEVICE_GLOBAL_MEM_SIZE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_MAX_CONSTANT_BUFFER_SIZE).unwrap();
        let value = value.to_ulong();
        println!("CL_DEVICE_MAX_CONSTANT_BUFFER_SIZE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_MAX_CONSTANT_ARGS).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_MAX_CONSTANT_ARGS: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_LOCAL_MEM_TYPE).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_LOCAL_MEM_TYPE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_LOCAL_MEM_SIZE).unwrap();
        let value = value.to_ulong();
        println!("CL_DEVICE_LOCAL_MEM_SIZE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_ERROR_CORRECTION_SUPPORT).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_ERROR_CORRECTION_SUPPORT: {}", value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_PROFILING_TIMER_RESOLUTION).unwrap();
        let value = value.to_size();
        println!("CL_DEVICE_PROFILING_TIMER_RESOLUTION: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_ENDIAN_LITTLE).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_ENDIAN_LITTLE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_AVAILABLE).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_AVAILABLE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_COMPILER_AVAILABLE).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_COMPILER_AVAILABLE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_EXECUTION_CAPABILITIES).unwrap();
        let value = value.to_ulong();
        println!("CL_DEVICE_EXECUTION_CAPABILITIES: {}", value);
        assert!(0 < value);

        if is_opencl_2 {
            let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_QUEUE_ON_HOST_PROPERTIES).unwrap();
            let value = value.to_ulong();
            println!("CL_DEVICE_QUEUE_ON_HOST_PROPERTIES: {}", value);
            assert!(0 < value);
        }

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_NAME).unwrap();
        let value = value.to_str().unwrap();
        println!("CL_DEVICE_NAME: {:?}", value);
        let value = value.into_string().unwrap();
        assert!(0 < value.len());

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_VENDOR).unwrap();
        let value = value.to_str().unwrap();
        println!("CL_DEVICE_VENDOR: {:?}", value);
        let value = value.into_string().unwrap();
        assert!(0 < value.len());

        let value = get_device_info(device_id, DeviceInfo::CL_DRIVER_VERSION).unwrap();
        let value = value.to_str().unwrap();
        println!("CL_DRIVER_VERSION: {:?}", value);
        let value = value.into_string().unwrap();
        assert!(0 < value.len());

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_PROFILE).unwrap();
        let value = value.to_str().unwrap();
        println!("CL_DEVICE_PROFILE: {:?}", value);
        let value = value.into_string().unwrap();
        assert!(0 < value.len());

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_EXTENSIONS).unwrap();
        let value = value.to_str().unwrap();
        println!("CL_DEVICE_EXTENSIONS: {:?}", value);
        let value = value.into_string().unwrap();
        assert!(0 < value.len());

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_PLATFORM).unwrap();
        let value = value.to_ptr();
        println!("CL_DEVICE_PLATFORM: {}", value);
        assert!(0 < value);

        // Device may not support double fp precision
        match get_device_info(device_id, DeviceInfo::CL_DEVICE_DOUBLE_FP_CONFIG) {
            Ok(value) => {
                let value = value.to_ulong();
                println!("CL_DEVICE_DOUBLE_FP_CONFIG: {}", value)
            }
            Err(e) => println!("OpenCL error, CL_DEVICE_DOUBLE_FP_CONFIG: {}", error_text(e))
        };

        // Device may not support half fp precision
        match get_device_info(device_id, DeviceInfo::CL_DEVICE_HALF_FP_CONFIG) {
            Ok(value) => {
                let value = value.to_ulong();
                println!("CL_DEVICE_HALF_FP_CONFIG: {}", value)
            }
            Err(e) => println!("OpenCL error, CL_DEVICE_HALF_FP_CONFIG: {}", error_text(e))
        };

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_PREFERRED_VECTOR_WIDTH_HALF).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_PREFERRED_VECTOR_WIDTH_HALF: {}", value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_NATIVE_VECTOR_WIDTH_CHAR).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_NATIVE_VECTOR_WIDTH_CHAR: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_NATIVE_VECTOR_WIDTH_SHORT).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_NATIVE_VECTOR_WIDTH_SHORT: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_NATIVE_VECTOR_WIDTH_INT).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_NATIVE_VECTOR_WIDTH_INT: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_NATIVE_VECTOR_WIDTH_LONG).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_NATIVE_VECTOR_WIDTH_LONG: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_NATIVE_VECTOR_WIDTH_FLOAT).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_NATIVE_VECTOR_WIDTH_FLOAT: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_NATIVE_VECTOR_WIDTH_DOUBLE).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_NATIVE_VECTOR_WIDTH_DOUBLE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_NATIVE_VECTOR_WIDTH_HALF).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_NATIVE_VECTOR_WIDTH_HALF: {}", value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_OPENCL_C_VERSION).unwrap();
        let value = value.to_str().unwrap();
        println!("CL_DEVICE_OPENCL_C_VERSION: {:?}", value);
        let value = value.into_string().unwrap();
        assert!(0 < value.len());

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_LINKER_AVAILABLE).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_LINKER_AVAILABLE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_BUILT_IN_KERNELS).unwrap();
        let value = value.to_str().unwrap();
        println!("CL_DEVICE_BUILT_IN_KERNELS: {:?}", value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_IMAGE_MAX_BUFFER_SIZE).unwrap();
        let value = value.to_size();
        println!("CL_DEVICE_IMAGE_MAX_BUFFER_SIZE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_IMAGE_MAX_ARRAY_SIZE).unwrap();
        let value = value.to_size();
        println!("CL_DEVICE_IMAGE_MAX_ARRAY_SIZE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_PARENT_DEVICE).unwrap();
        let value = value.to_ptr();
        println!("CL_DEVICE_PARENT_DEVICE: {}", value);
        assert!(0 == value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_PARTITION_MAX_SUB_DEVICES).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_PARTITION_MAX_SUB_DEVICES: {}", value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_PARTITION_PROPERTIES).unwrap();
        let value = value.to_vec_intptr();
        println!("CL_DEVICE_PARTITION_PROPERTIES: {}", value.len());
        println!("CL_DEVICE_PARTITION_PROPERTIES: {:?}", value);
        assert!(0 < value.len());

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_PARTITION_AFFINITY_DOMAIN).unwrap();
        let value = value.to_vec_ulong();
        println!("CL_DEVICE_PARTITION_AFFINITY_DOMAIN: {}", value.len());
        println!("CL_DEVICE_PARTITION_AFFINITY_DOMAIN: {:?}", value);
        assert!(0 < value.len());

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_PARTITION_TYPE).unwrap();
        let value = value.to_vec_intptr();
        println!("CL_DEVICE_PARTITION_TYPE: {}", value.len());
        println!("CL_DEVICE_PARTITION_TYPE: {:?}", value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_REFERENCE_COUNT).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_REFERENCE_COUNT: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_PREFERRED_INTEROP_USER_SYNC).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_PREFERRED_INTEROP_USER_SYNC: {}", value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_PRINTF_BUFFER_SIZE).unwrap();
        let value = value.to_size();
        println!("CL_DEVICE_PRINTF_BUFFER_SIZE: {}", value);
        assert!(0 < value);

        // CL_VERSION_2_0
        if is_opencl_2 {
            let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_IMAGE_PITCH_ALIGNMENT).unwrap();
            let value = value.to_uint();
            println!("CL_DEVICE_IMAGE_PITCH_ALIGNMENT: {}", value);
            assert!(0 < value);

            let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_IMAGE_BASE_ADDRESS_ALIGNMENT).unwrap();
            let value = value.to_uint();
            println!("CL_DEVICE_IMAGE_BASE_ADDRESS_ALIGNMENT: {}", value);
            assert!(0 < value);

            let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_MAX_READ_WRITE_IMAGE_ARGS).unwrap();
            let value = value.to_uint();
            println!("CL_DEVICE_MAX_READ_WRITE_IMAGE_ARGS: {}", value);
            assert!(0 < value);

            let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_MAX_GLOBAL_VARIABLE_SIZE).unwrap();
            let value = value.to_size();
            println!("CL_DEVICE_MAX_GLOBAL_VARIABLE_SIZE: {}", value);
            assert!(0 < value);

            let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_QUEUE_ON_DEVICE_PROPERTIES).unwrap();
            let value = value.to_vec_intptr();
            println!("CL_DEVICE_QUEUE_ON_DEVICE_PROPERTIES: {}", value.len());
            println!("CL_DEVICE_QUEUE_ON_DEVICE_PROPERTIES: {:?}", value);
            assert!(0 < value.len());

            let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_QUEUE_ON_DEVICE_PREFERRED_SIZE).unwrap();
            let value = value.to_size();
            println!("CL_DEVICE_QUEUE_ON_DEVICE_PREFERRED_SIZE: {}", value);
            assert!(0 < value);

            let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_QUEUE_ON_DEVICE_MAX_SIZE).unwrap();
            let value = value.to_size();
            println!("CL_DEVICE_QUEUE_ON_DEVICE_MAX_SIZE: {}", value);
            assert!(0 < value);

            let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_MAX_ON_DEVICE_QUEUES).unwrap();
            let value = value.to_uint();
            println!("CL_DEVICE_MAX_ON_DEVICE_QUEUES: {}", value);
            assert!(0 < value);

            let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_MAX_ON_DEVICE_EVENTS).unwrap();
            let value = value.to_uint();
            println!("CL_DEVICE_MAX_ON_DEVICE_EVENTS: {}", value);
            assert!(0 < value);

            let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_SVM_CAPABILITIES).unwrap();
            let value = value.to_ulong();
            println!("CL_DEVICE_SVM_CAPABILITIES: {}", value);
            assert!(0 < value);

            let value =
                get_device_info(device_id, DeviceInfo::CL_DEVICE_GLOBAL_VARIABLE_PREFERRED_TOTAL_SIZE).unwrap();
            let value = value.to_size();
            println!("CL_DEVICE_GLOBAL_VARIABLE_PREFERRED_TOTAL_SIZE: {}", value);
            assert!(0 < value);

            let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_MAX_PIPE_ARGS).unwrap();
            let value = value.to_uint();
            println!("CL_DEVICE_MAX_PIPE_ARGS: {}", value);
            assert!(0 < value);

            let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_PIPE_MAX_ACTIVE_RESERVATIONS).unwrap();
            let value = value.to_uint();
            println!("CL_DEVICE_PIPE_MAX_ACTIVE_RESERVATIONS: {}", value);
            assert!(0 < value);

            let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_PIPE_MAX_PACKET_SIZE).unwrap();
            let value = value.to_uint();
            println!("CL_DEVICE_PIPE_MAX_PACKET_SIZE: {}", value);
            assert!(0 < value);

            let value =
                get_device_info(device_id, DeviceInfo::CL_DEVICE_PREFERRED_PLATFORM_ATOMIC_ALIGNMENT).unwrap();
            let value = value.to_uint();
            println!("CL_DEVICE_PREFERRED_PLATFORM_ATOMIC_ALIGNMENT: {}", value);
            // assert!(0 < value);

            let value =
                get_device_info(device_id, DeviceInfo::CL_DEVICE_PREFERRED_GLOBAL_ATOMIC_ALIGNMENT).unwrap();
            let value = value.to_uint();
            println!("CL_DEVICE_PREFERRED_GLOBAL_ATOMIC_ALIGNMENT: {}", value);
            // assert!(0 < value);

            let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_PREFERRED_LOCAL_ATOMIC_ALIGNMENT).unwrap();
            let value = value.to_uint();
            println!("CL_DEVICE_PREFERRED_LOCAL_ATOMIC_ALIGNMENT: {}", value);
            // assert!(0 < value);


            // CL_VERSION_2_1
            if is_opencl_2_1 {
                let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_IL_VERSION).unwrap();
                let value = value.to_str().unwrap();
                println!("CL_DEVICE_IL_VERSION: {:?}", value);
                let value = value.into_string().unwrap();
                assert!(0 < value.len());

                let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_MAX_NUM_SUB_GROUPS).unwrap();
                let value = value.to_uint();
                println!("CL_DEVICE_MAX_NUM_SUB_GROUPS: {}", value);
                assert!(0 < value);

                let value =
                    get_device_info(device_id, DeviceInfo::CL_DEVICE_SUB_GROUP_INDEPENDENT_FORWARD_PROGRESS).unwrap();
                let value = value.to_uint();
                println!(
                    "CL_DEVICE_SUB_GROUP_INDEPENDENT_FORWARD_PROGRESS: {}",
                    value
                );
                assert!(0 < value);
            }
        }
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
        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_NUMERIC_VERSION).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_NUMERIC_VERSION: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_EXTENSIONS_WITH_VERSION).unwrap();
        let value = value.to_vec_name_version();
        println!("CL_DEVICE_EXTENSIONS_WITH_VERSION: {}", value.len());
        println!("CL_DEVICE_EXTENSIONS_WITH_VERSION: {:?}", value);
        assert!(0 < value.len());

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_ILS_WITH_VERSION).unwrap();
        let value = value.to_vec_name_version();
        println!("CL_DEVICE_ILS_WITH_VERSION: {}", value.len());
        println!("CL_DEVICE_ILS_WITH_VERSION: {:?}", value);
        assert!(0 < value.len());

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_BUILT_IN_KERNELS_WITH_VERSION).unwrap();
        let value = value.to_vec_name_version();
        println!("CL_DEVICE_BUILT_IN_KERNELS_WITH_VERSION: {}", value.len());
        println!("CL_DEVICE_BUILT_IN_KERNELS_WITH_VERSION: {:?}", value);
        assert!(0 < value.len());

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_ATOMIC_MEMORY_CAPABILITIES).unwrap();
        let value = value.to_ulong();
        println!("CL_DEVICE_ATOMIC_MEMORY_CAPABILITIES: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_ATOMIC_MEMORY_CAPABILITIES).unwrap();
        let value = value.to_ulong();
        println!("CL_DEVICE_ATOMIC_FENCE_CAPABILITIES: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_NON_UNIFORM_WORK_GROUP_SUPPORT).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_NON_UNIFORM_WORK_GROUP_SUPPORT: {}", value);
        // assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_OPENCL_C_ALL_VERSIONS).unwrap();
        let value = value.to_vec_name_version();
        println!("CL_DEVICE_OPENCL_C_ALL_VERSIONS: {}", value.len());
        println!("CL_DEVICE_OPENCL_C_ALL_VERSIONS: {:?}", value);
        assert!(0 < value.len());

        let value =
            get_device_info(device_id, DeviceInfo::CL_DEVICE_PREFERRED_WORK_GROUP_SIZE_MULTIPLE).unwrap();
        let value = value.to_size();
        println!("CL_DEVICE_PREFERRED_WORK_GROUP_SIZE_MULTIPLE: {}", value);
        assert!(0 < value);

        let value =
            get_device_info(device_id, DeviceInfo::CL_DEVICE_WORK_GROUP_COLLECTIVE_FUNCTIONS_SUPPORT).unwrap();
        let value = value.to_uint();
        println!(
            "CL_DEVICE_WORK_GROUP_COLLECTIVE_FUNCTIONS_SUPPORT: {}",
            value
        );
        // assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_GENERIC_ADDRESS_SPACE_SUPPORT).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_GENERIC_ADDRESS_SPACE_SUPPORT: {}", value);
        // assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_OPENCL_C_FEATURES).unwrap();
        let value = value.to_vec_name_version();
        println!("CL_DEVICE_OPENCL_C_FEATURES: {}", value.len());
        println!("CL_DEVICE_OPENCL_C_FEATURES: {:?}", value);
        assert!(0 < value.len());

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_DEVICE_ENQUEUE_CAPABILITIES).unwrap();
        let value = value.to_ulong();
        println!("CL_DEVICE_DEVICE_ENQUEUE_CAPABILITIES: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_PIPE_SUPPORT).unwrap();
        let value = value.to_uint();
        println!("CL_DEVICE_PIPE_SUPPORT: {}", value);
        // assert!(0 < value);

        let value =
            get_device_info(device_id, DeviceInfo::CL_DEVICE_LATEST_CONFORMANCE_VERSION_PASSED).unwrap();
        let value = value.to_str().unwrap();
        println!("CL_DEVICE_LATEST_CONFORMANCE_VERSION_PASSED: {:?}", value);
        let value = value.into_string().unwrap();
        assert!(0 < value.len());
    }

    #[test]
    fn test_get_sub_devices() {
        let platform_ids = get_platform_ids().unwrap();
        assert!(0 < platform_ids.len());

        // Find an OpenCL device with sub devices

        let mut device_id = ptr::null_mut();
        let mut has_sub_devices: bool = false;

        for p in platform_ids {
            let device_ids = get_device_ids(p, CL_DEVICE_TYPE_CPU).unwrap();

            for dev_id in device_ids {
                let value = get_device_info(dev_id, DeviceInfo::CL_DEVICE_PARTITION_MAX_SUB_DEVICES).unwrap();
                let max_sub_devices = value.to_uint();

                has_sub_devices = 1 < max_sub_devices;
                if has_sub_devices {
                    device_id = dev_id;
                    break;
                }
            }
        }

        if has_sub_devices {
            let properties: [cl_device_partition_property; 3] =
                [CL_DEVICE_PARTITION_EQUALLY, 4, 0];
            let sub_devices = create_sub_devices(device_id, &properties).unwrap();

            println!("CL_DEVICE_TYPE_CPU count: {}", sub_devices.len());
            assert!(0 < sub_devices.len());

            for device in sub_devices {
                release_device(device).unwrap();
            }
        } else {
            println!("OpenCL device capable of sub division not found");
        }
    }
}
