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

//! `OpenCL` Device API.

#![allow(unused_unsafe)]
#![allow(non_camel_case_types, non_upper_case_globals)]
#![allow(
    clippy::not_unsafe_ptr_arg_deref,
    clippy::too_many_lines,
    clippy::wildcard_in_or_patterns
)]

pub use crate::constants::cl_ext::{
    CL_DEVICE_INTEGER_DOT_PRODUCT_ACCELERATION_PROPERTIES_4x8BIT_PACKED_KHR,
    CL_DEVICE_AVAILABLE_ASYNC_QUEUES_AMD, CL_DEVICE_BOARD_NAME_AMD,
    CL_DEVICE_COMMAND_BUFFER_CAPABILITIES_KHR,
    CL_DEVICE_COMMAND_BUFFER_REQUIRED_QUEUE_PROPERTIES_KHR, CL_DEVICE_COMPUTE_CAPABILITY_MAJOR_NV,
    CL_DEVICE_COMPUTE_CAPABILITY_MINOR_NV, CL_DEVICE_DOUBLE_FP_CONFIG,
    CL_DEVICE_EXTERNAL_MEMORY_IMPORT_HANDLE_TYPES_KHR, CL_DEVICE_FEATURE_CAPABILITIES_INTEL,
    CL_DEVICE_GFXIP_MAJOR_AMD, CL_DEVICE_GFXIP_MINOR_AMD, CL_DEVICE_GLOBAL_FREE_MEMORY_AMD,
    CL_DEVICE_GLOBAL_MEM_CHANNELS_AMD, CL_DEVICE_GLOBAL_MEM_CHANNEL_BANKS_AMD,
    CL_DEVICE_GLOBAL_MEM_CHANNEL_BANK_WIDTH_AMD, CL_DEVICE_GPU_OVERLAP_NV,
    CL_DEVICE_HALF_FP_CONFIG, CL_DEVICE_ID_INTEL,
    CL_DEVICE_INTEGER_DOT_PRODUCT_ACCELERATION_PROPERTIES_8BIT_KHR,
    CL_DEVICE_INTEGER_DOT_PRODUCT_CAPABILITIES_KHR, CL_DEVICE_INTEGRATED_MEMORY_NV,
    CL_DEVICE_IP_VERSION_INTEL, CL_DEVICE_KERNEL_EXEC_TIMEOUT_NV, CL_DEVICE_LOCAL_MEM_BANKS_AMD,
    CL_DEVICE_LOCAL_MEM_SIZE_PER_COMPUTE_UNIT_AMD, CL_DEVICE_LUID_KHR, CL_DEVICE_LUID_VALID_KHR,
    CL_DEVICE_MAX_WORK_GROUP_SIZE_AMD, CL_DEVICE_NODE_MASK_KHR,
    CL_DEVICE_NUM_EUS_PER_SUB_SLICE_INTEL, CL_DEVICE_NUM_SLICES_INTEL,
    CL_DEVICE_NUM_SUB_SLICES_PER_SLICE_INTEL, CL_DEVICE_NUM_THREADS_PER_EU_INTEL,
    CL_DEVICE_PCIE_ID_AMD, CL_DEVICE_PCI_BUS_ID_NV, CL_DEVICE_PCI_BUS_INFO_KHR,
    CL_DEVICE_PCI_SLOT_ID_NV, CL_DEVICE_PREFERRED_CONSTANT_BUFFER_SIZE_AMD,
    CL_DEVICE_PREFERRED_WORK_GROUP_SIZE_AMD, CL_DEVICE_PROFILING_TIMER_OFFSET_AMD,
    CL_DEVICE_REGISTERS_PER_BLOCK_NV, CL_DEVICE_SEMAPHORE_EXPORT_HANDLE_TYPES_KHR,
    CL_DEVICE_SEMAPHORE_IMPORT_HANDLE_TYPES_KHR, CL_DEVICE_SEMAPHORE_TYPES_KHR,
    CL_DEVICE_SIMD_INSTRUCTION_WIDTH_AMD, CL_DEVICE_SIMD_PER_COMPUTE_UNIT_AMD,
    CL_DEVICE_SIMD_WIDTH_AMD, CL_DEVICE_THREAD_TRACE_SUPPORTED_AMD, CL_DEVICE_TOPOLOGY_AMD,
    CL_DEVICE_UUID_KHR, CL_DEVICE_WARP_SIZE_NV, CL_DEVICE_WAVEFRONT_WIDTH_AMD, CL_DRIVER_UUID_KHR,
    CL_LUID_SIZE_KHR, CL_UUID_SIZE_KHR,
};
pub use crate::constants::{
    CL_DEVICE_ADDRESS_BITS, CL_DEVICE_AFFINITY_DOMAIN_L1_CACHE, CL_DEVICE_AFFINITY_DOMAIN_L2_CACHE,
    CL_DEVICE_AFFINITY_DOMAIN_L3_CACHE, CL_DEVICE_AFFINITY_DOMAIN_L4_CACHE,
    CL_DEVICE_AFFINITY_DOMAIN_NEXT_PARTITIONABLE, CL_DEVICE_AFFINITY_DOMAIN_NUMA,
    CL_DEVICE_ATOMIC_FENCE_CAPABILITIES, CL_DEVICE_ATOMIC_MEMORY_CAPABILITIES,
    CL_DEVICE_ATOMIC_ORDER_ACQ_REL, CL_DEVICE_ATOMIC_ORDER_RELAXED, CL_DEVICE_ATOMIC_ORDER_SEQ_CST,
    CL_DEVICE_ATOMIC_SCOPE_ALL_DEVICES, CL_DEVICE_ATOMIC_SCOPE_DEVICE,
    CL_DEVICE_ATOMIC_SCOPE_WORK_GROUP, CL_DEVICE_ATOMIC_SCOPE_WORK_ITEM, CL_DEVICE_AVAILABLE,
    CL_DEVICE_BUILT_IN_KERNELS, CL_DEVICE_BUILT_IN_KERNELS_WITH_VERSION,
    CL_DEVICE_COMPILER_AVAILABLE, CL_DEVICE_DEVICE_ENQUEUE_CAPABILITIES, CL_DEVICE_ENDIAN_LITTLE,
    CL_DEVICE_ERROR_CORRECTION_SUPPORT, CL_DEVICE_EXECUTION_CAPABILITIES, CL_DEVICE_EXTENSIONS,
    CL_DEVICE_EXTENSIONS_WITH_VERSION, CL_DEVICE_GENERIC_ADDRESS_SPACE_SUPPORT,
    CL_DEVICE_GLOBAL_MEM_CACHELINE_SIZE, CL_DEVICE_GLOBAL_MEM_CACHE_SIZE,
    CL_DEVICE_GLOBAL_MEM_CACHE_TYPE, CL_DEVICE_GLOBAL_MEM_SIZE,
    CL_DEVICE_GLOBAL_VARIABLE_PREFERRED_TOTAL_SIZE, CL_DEVICE_HOST_UNIFIED_MEMORY,
    CL_DEVICE_ILS_WITH_VERSION, CL_DEVICE_IL_VERSION, CL_DEVICE_IMAGE2D_MAX_HEIGHT,
    CL_DEVICE_IMAGE2D_MAX_WIDTH, CL_DEVICE_IMAGE3D_MAX_DEPTH, CL_DEVICE_IMAGE3D_MAX_HEIGHT,
    CL_DEVICE_IMAGE3D_MAX_WIDTH, CL_DEVICE_IMAGE_BASE_ADDRESS_ALIGNMENT,
    CL_DEVICE_IMAGE_MAX_ARRAY_SIZE, CL_DEVICE_IMAGE_MAX_BUFFER_SIZE,
    CL_DEVICE_IMAGE_PITCH_ALIGNMENT, CL_DEVICE_IMAGE_SUPPORT,
    CL_DEVICE_LATEST_CONFORMANCE_VERSION_PASSED, CL_DEVICE_LINKER_AVAILABLE,
    CL_DEVICE_LOCAL_MEM_SIZE, CL_DEVICE_LOCAL_MEM_TYPE, CL_DEVICE_MAX_CLOCK_FREQUENCY,
    CL_DEVICE_MAX_COMPUTE_UNITS, CL_DEVICE_MAX_CONSTANT_ARGS, CL_DEVICE_MAX_CONSTANT_BUFFER_SIZE,
    CL_DEVICE_MAX_GLOBAL_VARIABLE_SIZE, CL_DEVICE_MAX_MEM_ALLOC_SIZE, CL_DEVICE_MAX_NUM_SUB_GROUPS,
    CL_DEVICE_MAX_ON_DEVICE_EVENTS, CL_DEVICE_MAX_ON_DEVICE_QUEUES, CL_DEVICE_MAX_PARAMETER_SIZE,
    CL_DEVICE_MAX_PIPE_ARGS, CL_DEVICE_MAX_READ_IMAGE_ARGS, CL_DEVICE_MAX_READ_WRITE_IMAGE_ARGS,
    CL_DEVICE_MAX_SAMPLERS, CL_DEVICE_MAX_WORK_GROUP_SIZE, CL_DEVICE_MAX_WORK_ITEM_DIMENSIONS,
    CL_DEVICE_MAX_WORK_ITEM_SIZES, CL_DEVICE_MAX_WRITE_IMAGE_ARGS, CL_DEVICE_MEM_BASE_ADDR_ALIGN,
    CL_DEVICE_MIN_DATA_TYPE_ALIGN_SIZE, CL_DEVICE_NAME, CL_DEVICE_NATIVE_VECTOR_WIDTH_CHAR,
    CL_DEVICE_NATIVE_VECTOR_WIDTH_DOUBLE, CL_DEVICE_NATIVE_VECTOR_WIDTH_FLOAT,
    CL_DEVICE_NATIVE_VECTOR_WIDTH_HALF, CL_DEVICE_NATIVE_VECTOR_WIDTH_INT,
    CL_DEVICE_NATIVE_VECTOR_WIDTH_LONG, CL_DEVICE_NATIVE_VECTOR_WIDTH_SHORT,
    CL_DEVICE_NON_UNIFORM_WORK_GROUP_SUPPORT, CL_DEVICE_NOT_FOUND, CL_DEVICE_NUMERIC_VERSION,
    CL_DEVICE_OPENCL_C_ALL_VERSIONS, CL_DEVICE_OPENCL_C_FEATURES, CL_DEVICE_OPENCL_C_VERSION,
    CL_DEVICE_PARENT_DEVICE, CL_DEVICE_PARTITION_AFFINITY_DOMAIN,
    CL_DEVICE_PARTITION_MAX_SUB_DEVICES, CL_DEVICE_PARTITION_PROPERTIES, CL_DEVICE_PARTITION_TYPE,
    CL_DEVICE_PIPE_MAX_ACTIVE_RESERVATIONS, CL_DEVICE_PIPE_MAX_PACKET_SIZE, CL_DEVICE_PIPE_SUPPORT,
    CL_DEVICE_PLATFORM, CL_DEVICE_PREFERRED_GLOBAL_ATOMIC_ALIGNMENT,
    CL_DEVICE_PREFERRED_INTEROP_USER_SYNC, CL_DEVICE_PREFERRED_LOCAL_ATOMIC_ALIGNMENT,
    CL_DEVICE_PREFERRED_PLATFORM_ATOMIC_ALIGNMENT, CL_DEVICE_PREFERRED_VECTOR_WIDTH_CHAR,
    CL_DEVICE_PREFERRED_VECTOR_WIDTH_DOUBLE, CL_DEVICE_PREFERRED_VECTOR_WIDTH_FLOAT,
    CL_DEVICE_PREFERRED_VECTOR_WIDTH_HALF, CL_DEVICE_PREFERRED_VECTOR_WIDTH_INT,
    CL_DEVICE_PREFERRED_VECTOR_WIDTH_LONG, CL_DEVICE_PREFERRED_VECTOR_WIDTH_SHORT,
    CL_DEVICE_PREFERRED_WORK_GROUP_SIZE_MULTIPLE, CL_DEVICE_PRINTF_BUFFER_SIZE, CL_DEVICE_PROFILE,
    CL_DEVICE_PROFILING_TIMER_RESOLUTION, CL_DEVICE_QUEUE_ON_DEVICE_MAX_SIZE,
    CL_DEVICE_QUEUE_ON_DEVICE_PREFERRED_SIZE, CL_DEVICE_QUEUE_ON_DEVICE_PROPERTIES,
    CL_DEVICE_QUEUE_ON_HOST_PROPERTIES, CL_DEVICE_QUEUE_REPLACEABLE_DEFAULT,
    CL_DEVICE_QUEUE_SUPPORTED, CL_DEVICE_REFERENCE_COUNT, CL_DEVICE_SINGLE_FP_CONFIG,
    CL_DEVICE_SUB_GROUP_INDEPENDENT_FORWARD_PROGRESS, CL_DEVICE_SVM_ATOMICS,
    CL_DEVICE_SVM_CAPABILITIES, CL_DEVICE_SVM_COARSE_GRAIN_BUFFER, CL_DEVICE_SVM_FINE_GRAIN_BUFFER,
    CL_DEVICE_SVM_FINE_GRAIN_SYSTEM, CL_DEVICE_TYPE, CL_DEVICE_TYPE_ACCELERATOR,
    CL_DEVICE_TYPE_ALL, CL_DEVICE_TYPE_CPU, CL_DEVICE_TYPE_CUSTOM, CL_DEVICE_TYPE_DEFAULT,
    CL_DEVICE_TYPE_GPU, CL_DEVICE_VENDOR, CL_DEVICE_VENDOR_ID, CL_DEVICE_VERSION,
    CL_DEVICE_WORK_GROUP_COLLECTIVE_FUNCTIONS_SUPPORT, CL_DRIVER_VERSION, CL_EXEC_KERNEL,
    CL_EXEC_NATIVE_KERNEL, CL_FALSE, CL_FP_CORRECTLY_ROUNDED_DIVIDE_SQRT, CL_FP_DENORM, CL_FP_FMA,
    CL_FP_INF_NAN, CL_FP_ROUND_TO_INF, CL_FP_ROUND_TO_NEAREST, CL_FP_ROUND_TO_ZERO,
    CL_FP_SOFT_FLOAT, CL_GLOBAL, CL_LOCAL, CL_NONE, CL_READ_ONLY_CACHE, CL_READ_WRITE_CACHE,
    CL_SUCCESS, CL_TRUE, CL_VERSION_MAJOR_BITS, CL_VERSION_MAJOR_MASK, CL_VERSION_MINOR_BITS,
    CL_VERSION_MINOR_MASK, CL_VERSION_PATCH_BITS, CL_VERSION_PATCH_MASK,
};
pub use crate::types::cl_ext::{
    cl_amd_device_topology, cl_device_integer_dot_product_acceleration_properties_khr,
    cl_device_pci_bus_info_khr,
};
pub use crate::types::{
    cl_command_queue, cl_context, cl_device_fp_config, cl_device_id, cl_device_info,
    cl_device_partition_property, cl_device_svm_capabilities, cl_device_type, cl_double, cl_float,
    cl_int, cl_name_version, cl_platform_id, cl_uint, cl_ulong,
};

use super::info_type::InfoType;
use super::{api_info_size, api_info_value, api_info_vector};
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
/// * `platform` - the `cl_platform_id` of the `OpenCL` platform.
/// * `device_type` - the type of device, see
/// [Device Types](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#device-types-table).
///
/// returns a Result containing a vector of available device ids
/// or the error code from the `OpenCL` C API function.
pub fn get_device_ids(
    platform: cl_platform_id,
    device_type: cl_device_type,
) -> Result<Vec<cl_device_id>, cl_int> {
    // Get the number of devices of device_type
    let mut count: cl_uint = 0;
    let mut status = unsafe {
        cl_call!(clGetDeviceIDs(
            platform,
            device_type,
            0,
            ptr::null_mut(),
            &mut count
        ))
    };

    if (CL_SUCCESS != status) && (CL_DEVICE_NOT_FOUND != status) {
        Err(status)
    } else if 0 < count {
        // Get the device ids.
        let len = count as size_t;
        let mut ids: Vec<cl_device_id> = Vec::with_capacity(len);
        unsafe {
            status = cl_call!(clGetDeviceIDs(
                platform,
                device_type,
                count,
                ids.as_mut_ptr(),
                ptr::null_mut(),
            ));
            ids.set_len(len);
        };

        if CL_SUCCESS == status {
            Ok(ids)
        } else {
            Err(status)
        }
    } else {
        Ok(Vec::default())
    }
}

/// Get data about an `OpenCL` device.
/// Calls clGetDeviceInfo to get the desired data about the device.
pub fn get_device_data(
    device: cl_device_id,
    param_name: cl_device_info,
) -> Result<Vec<u8>, cl_int> {
    api_info_size!(get_size, clGetDeviceInfo);
    let size = get_size(device, param_name)?;
    api_info_vector!(get_vector, u8, clGetDeviceInfo);
    get_vector(device, param_name, size)
}

/// Get specific information about an `OpenCL` device.
/// Calls clGetDeviceInfo to get the desired information about the device.
///  # Examples
/// ```
/// use cl3::platform::get_platform_ids;
/// use cl3::device::{get_device_ids, get_device_info, CL_DEVICE_TYPE, CL_DEVICE_TYPE_GPU, CL_DEVICE_VENDOR, CL_DEVICE_VERSION};
/// use cl3::types::cl_ulong;
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
/// // Choose the first device
/// let device_id = device_ids[0];
///
/// let value = get_device_info(device_id, CL_DEVICE_TYPE).unwrap();
/// let value = cl_ulong::from(value);
/// println!("CL_DEVICE_TYPE: {}", value);
/// assert_eq!(CL_DEVICE_TYPE_GPU, value);
///
/// let value = get_device_info(device_id, CL_DEVICE_VENDOR).unwrap();
/// let value = String::from(value);
/// println!("CL_DEVICE_VENDOR: {}", value);
/// assert!(!value.is_empty());
///
/// let value = get_device_info(device_id, CL_DEVICE_VERSION).unwrap();
/// let value:String = value.into();
/// println!("CL_DEVICE_VERSION: {}", value);
/// assert!(!value.is_empty());
/// ```
/// * `device` - the `cl_device_id` of the `OpenCL` device.
/// * `param_name` - the type of device information being queried, see
/// [Device Queries](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#device-queries-table).
///
/// returns a Result containing the desired information in an `InfoType` enum
/// or the error code from the `OpenCL` C API function.
pub fn get_device_info(
    device: cl_device_id,
    param_name: cl_device_info,
) -> Result<InfoType, cl_int> {
    api_info_size!(get_size, clGetDeviceInfo);

    match param_name {
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
        | CL_DEVICE_HOST_UNIFIED_MEMORY
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

        | CL_DEVICE_LUID_VALID_KHR // cl_khr_device_uuid
        | CL_DEVICE_NODE_MASK_KHR // cl_khr_device_uuid

        | CL_DEVICE_COMPUTE_CAPABILITY_MAJOR_NV // cl_nv_device_attribute_query
        | CL_DEVICE_COMPUTE_CAPABILITY_MINOR_NV // cl_nv_device_attribute_query
        | CL_DEVICE_REGISTERS_PER_BLOCK_NV // cl_nv_device_attribute_query
        | CL_DEVICE_WARP_SIZE_NV // cl_nv_device_attribute_query
        | CL_DEVICE_GPU_OVERLAP_NV // cl_nv_device_attribute_query
        | CL_DEVICE_KERNEL_EXEC_TIMEOUT_NV // cl_nv_device_attribute_query
        | CL_DEVICE_INTEGRATED_MEMORY_NV // cl_nv_device_attribute_query

        | CL_DEVICE_PCI_BUS_ID_NV // cl_nv_device_attribute_query, undocumented
        | CL_DEVICE_PCI_SLOT_ID_NV // cl_nv_device_attribute_query, undocumented

        | CL_DEVICE_SIMD_PER_COMPUTE_UNIT_AMD // cl_amd_device_attribute_query
        | CL_DEVICE_SIMD_WIDTH_AMD // cl_amd_device_attribute_query
        | CL_DEVICE_SIMD_INSTRUCTION_WIDTH_AMD // cl_amd_device_attribute_query
        | CL_DEVICE_WAVEFRONT_WIDTH_AMD // cl_amd_device_attribute_query
        | CL_DEVICE_GLOBAL_MEM_CHANNELS_AMD // cl_amd_device_attribute_query
        | CL_DEVICE_GLOBAL_MEM_CHANNEL_BANKS_AMD // cl_amd_device_attribute_query
        | CL_DEVICE_GLOBAL_MEM_CHANNEL_BANK_WIDTH_AMD // cl_amd_device_attribute_query
        | CL_DEVICE_LOCAL_MEM_SIZE_PER_COMPUTE_UNIT_AMD // cl_amd_device_attribute_query
        | CL_DEVICE_LOCAL_MEM_BANKS_AMD // cl_amd_device_attribute_query
        | CL_DEVICE_THREAD_TRACE_SUPPORTED_AMD // cl_amd_device_attribute_query
        | CL_DEVICE_GFXIP_MAJOR_AMD // cl_amd_device_attribute_query
        | CL_DEVICE_GFXIP_MINOR_AMD // cl_amd_device_attribute_query
        | CL_DEVICE_AVAILABLE_ASYNC_QUEUES_AMD // cl_amd_device_attribute_query
        | CL_DEVICE_PCIE_ID_AMD // cl_amd_device_attribute_query
        | CL_DEVICE_IP_VERSION_INTEL // cl_intel_device_attribute_query
        | CL_DEVICE_ID_INTEL // cl_intel_device_attribute_query
        | CL_DEVICE_NUM_SLICES_INTEL // cl_intel_device_attribute_query
        | CL_DEVICE_NUM_SUB_SLICES_PER_SLICE_INTEL // cl_intel_device_attribute_query
        | CL_DEVICE_NUM_EUS_PER_SUB_SLICE_INTEL // cl_intel_device_attribute_query
        | CL_DEVICE_NUM_THREADS_PER_EU_INTEL // cl_intel_device_attribute_query
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
        | CL_DEVICE_INTEGER_DOT_PRODUCT_CAPABILITIES_KHR
        | CL_DEVICE_FEATURE_CAPABILITIES_INTEL // cl_intel_device_attribute_query
        | CL_DEVICE_COMMAND_BUFFER_CAPABILITIES_KHR // cl_khr_command_buffer
        | CL_DEVICE_COMMAND_BUFFER_REQUIRED_QUEUE_PROPERTIES_KHR // cl_khr_command_buffer
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
        | CL_DEVICE_PROFILING_TIMER_OFFSET_AMD // cl_amd_device_attribute_query
        | CL_DEVICE_GLOBAL_FREE_MEMORY_AMD // cl_amd_device_attribute_query
        | CL_DEVICE_PREFERRED_WORK_GROUP_SIZE_AMD // cl_amd_device_attribute_query
        | CL_DEVICE_MAX_WORK_GROUP_SIZE_AMD // cl_amd_device_attribute_query
        | CL_DEVICE_PREFERRED_CONSTANT_BUFFER_SIZE_AMD // cl_amd_device_attribute_query
        => {
            api_info_value!(get_value, size_t, clGetDeviceInfo);
            Ok(InfoType::Size(get_value(device, param_name)?))
        }

        CL_DEVICE_PLATFORM | CL_DEVICE_PARENT_DEVICE => {
            api_info_value!(get_value, intptr_t, clGetDeviceInfo);
            Ok(InfoType::Ptr(get_value(device, param_name)?))
        }

        CL_DEVICE_EXTERNAL_MEMORY_IMPORT_HANDLE_TYPES_KHR // cl_khr_external_memory
        | CL_DEVICE_SEMAPHORE_IMPORT_HANDLE_TYPES_KHR // cl_khr_external_semaphore
        | CL_DEVICE_SEMAPHORE_EXPORT_HANDLE_TYPES_KHR // cl_khr_external_semaphore
        | CL_DEVICE_SEMAPHORE_TYPES_KHR // cl_khr_semaphore
        => {
            api_info_vector!(get_vec, cl_uint, clGetDeviceInfo);
            let size = get_size(device, param_name)?;
            Ok(InfoType::VecUshort(get_vec(device, param_name, size)?))
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

        CL_DEVICE_UUID_KHR // cl_khr_device_uuid
        | CL_DRIVER_UUID_KHR // cl_khr_device_uuid
        => {
            let mut value: [u8; CL_UUID_SIZE_KHR] = [0; CL_UUID_SIZE_KHR];
            let status = unsafe {
                cl_call!(clGetDeviceInfo(
                    device,
                    param_name,
                    CL_UUID_SIZE_KHR,
                    value.as_mut_ptr().cast::<c_void>(),
                     ptr::null_mut(),))
                    };
            if CL_SUCCESS == status {
                Ok(InfoType::Uuid(value))
            } else {
                Err(status)
            }
        }

        CL_DEVICE_LUID_KHR // cl_khr_device_uuid
        => {
            let mut value: [u8; CL_LUID_SIZE_KHR] = [0; CL_LUID_SIZE_KHR];
            let status = unsafe {
                cl_call!(clGetDeviceInfo(
                    device,
                    param_name,
                    CL_LUID_SIZE_KHR,
                    value.as_mut_ptr().cast::<c_void>(),
                    ptr::null_mut(),))
                };
            if CL_SUCCESS == status {
                Ok(InfoType::Luid(value))
            } else {
                Err(status)
            }
        }

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
        | CL_DEVICE_INTEGER_DOT_PRODUCT_ACCELERATION_PROPERTIES_8BIT_KHR // cl_device_integer_dot_product_acceleration_properties_khr
        | CL_DEVICE_INTEGER_DOT_PRODUCT_ACCELERATION_PROPERTIES_4x8BIT_PACKED_KHR // cl_device_integer_dot_product_acceleration_properties_khr
        | CL_DEVICE_TOPOLOGY_AMD // cl_amd_device_attribute_query
        | CL_DEVICE_BOARD_NAME_AMD // cl_amd_device_attribute_query
        | CL_DEVICE_PCI_BUS_INFO_KHR // cl_khr_pci_bus_info
        | _
        => {
            Ok(InfoType::VecUchar(get_device_data(device, param_name)?))
        }
    }
}

/// Convert a u8 slice (e.g. from `get_device_info`) into a `cl_amd_device_topology structure`.
///
/// # Panics
///
/// `if bytes.len() != size_of::<cl_amd_device_topology>`
#[must_use]
pub fn get_amd_device_topology(bytes: &[u8]) -> cl_amd_device_topology {
    let size = bytes.len();
    assert_eq!(size, std::mem::size_of::<cl_amd_device_topology>());
    let mut topology = cl_amd_device_topology::default();
    unsafe {
        std::slice::from_raw_parts_mut(std::ptr::addr_of_mut!(topology).cast::<u8>(), size)
            .copy_from_slice(bytes);
    }
    topology
}

/// Convert a u8 slice (e.g. from `get_device_info`) into a `cl_device_pci_bus_info_khr structure`.
///
/// # Panics
///
/// `if bytes.len() != size_of::<cl_device_pci_bus_info_khr>`
#[must_use]
pub fn get_device_pci_bus_info_khr(bytes: &[u8]) -> cl_device_pci_bus_info_khr {
    let size = bytes.len();
    assert_eq!(size, std::mem::size_of::<cl_device_pci_bus_info_khr>());
    let mut pci_bus_info = cl_device_pci_bus_info_khr::default();
    unsafe {
        std::slice::from_raw_parts_mut(std::ptr::addr_of_mut!(pci_bus_info).cast::<u8>(), size)
            .copy_from_slice(bytes);
    }
    pci_bus_info
}

/// Convert a u8 slice (e.g. from `get_device_info`) into a `cl_device_integer_dot_product_acceleration_properties_khr structure`.
///
/// # Panics
///
/// if `bytes.len() != size_of::<cl_device_integer_dot_product_acceleration_properties_khr>`
#[must_use]
pub fn get_device_integer_dot_product_acceleration_properties_khr(
    bytes: &[u8],
) -> cl_device_integer_dot_product_acceleration_properties_khr {
    let size = bytes.len();
    assert_eq!(
        size,
        std::mem::size_of::<cl_device_integer_dot_product_acceleration_properties_khr>()
    );
    let mut value = cl_device_integer_dot_product_acceleration_properties_khr::default();
    unsafe {
        std::slice::from_raw_parts_mut(std::ptr::addr_of_mut!(value).cast::<u8>(), size)
            .copy_from_slice(bytes);
    }
    value
}

// cl_device_partition_property:
pub const CL_DEVICE_PARTITION_EQUALLY: cl_device_partition_property = 0x1086;
pub const CL_DEVICE_PARTITION_BY_COUNTS: cl_device_partition_property = 0x1087;
pub const CL_DEVICE_PARTITION_BY_COUNTS_LIST_END: cl_device_partition_property = 0x0;
pub const CL_DEVICE_PARTITION_BY_AFFINITY_DOMAIN: cl_device_partition_property = 0x1088;

// helper function for create_sub_devices
#[cfg(feature = "CL_VERSION_1_2")]
#[inline]
fn count_sub_devices(
    in_device: cl_device_id,
    properties: &[cl_device_partition_property],
) -> Result<cl_uint, cl_int> {
    let mut count: cl_uint = 0;
    let status: cl_int = unsafe {
        cl_call!(clCreateSubDevices(
            in_device,
            properties.as_ptr(),
            0,
            ptr::null_mut(),
            &mut count,
        ))
    };
    if CL_SUCCESS == status {
        Ok(count)
    } else {
        Err(status)
    }
}

/// Create sub-devices by partitioning an `OpenCL` device.
/// Calls `clCreateSubDevices` to get the partitioned sub-devices.
///
/// * `in_device` - the `cl_device_id` of the `OpenCL` device to partition.
/// * `properties` - the slice of `cl_device_partition_property`, see
/// [Subdevice Partition](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#subdevice-partition-table).
///
/// returns a Result containing a vector of available sub-device ids
/// or the error code from the `OpenCL` C API function.
#[cfg(feature = "CL_VERSION_1_2")]
#[inline]
#[allow(clippy::cast_possible_truncation)]
pub fn create_sub_devices(
    in_device: cl_device_id,
    properties: &[cl_device_partition_property],
) -> Result<Vec<cl_device_id>, cl_int> {
    // get the number of partitions
    let num_devices: cl_uint = count_sub_devices(in_device, properties)?;

    // partition in_device
    let mut ids: Vec<cl_device_id> = Vec::with_capacity(num_devices as size_t);
    let status: cl_int = unsafe {
        ids.set_len(num_devices as size_t);
        cl_call!(clCreateSubDevices(
            in_device,
            properties.as_ptr(),
            num_devices * mem::size_of::<cl_device_id>() as cl_uint,
            ids.as_mut_ptr(),
            ptr::null_mut(),
        ))
    };

    if CL_SUCCESS == status {
        Ok(ids)
    } else {
        Err(status)
    }
}

/// Retain an `OpenCL` device.
/// Calls `clRetainDevice` to increment the device reference count
/// if device is a valid sub-device created by a call to clCreateSubDevices.
///
/// * `device` - the `cl_device_id` of the `OpenCL` device.
///
/// returns an empty Result or the error code from the `OpenCL` C API function.
///
/// # Safety
///
/// This function is unsafe because it changes the `OpenCL` object reference count.
#[cfg(feature = "CL_VERSION_1_2")]
#[inline]
pub unsafe fn retain_device(device: cl_device_id) -> Result<(), cl_int> {
    let status: cl_int = cl_call!(clRetainDevice(device));
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}

/// Release an `OpenCL` device.
/// Calls `clReleaseDevice` to decrement the device reference count
/// if device is a valid sub-device created by a call to clCreateSubDevices.
///
/// * `device` - the `cl_device_id` of the `OpenCL` device.
///
/// returns an empty Result or the error code from the `OpenCL` C API function.
///
/// # Safety
///
/// This function is unsafe because it changes the `OpenCL` object reference count.
#[cfg(feature = "CL_VERSION_1_2")]
#[inline]
pub unsafe fn release_device(device: cl_device_id) -> Result<(), cl_int> {
    let status: cl_int = cl_call!(clReleaseDevice(device));
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}

/// Replace the default command queue on an `OpenCL` device.
/// Calls `clSetDefaultDeviceCommandQueue` to replace the default command queue
/// `CL_VERSION_2_1`
///
/// * `context` - the `OpenCL` context used to create `command_queue`.
/// * `device` - a valid `OpenCL` device associated with context.
/// * `command_queue` - a command queue object which replaces the default
/// device command queue.
///
/// returns an empty Result or the error code from the `OpenCL` C API function.
#[cfg(feature = "CL_VERSION_2_1")]
#[inline]
pub fn set_default_device_command_queue(
    context: cl_context,
    device: cl_device_id,
    command_queue: cl_command_queue,
) -> Result<(), cl_int> {
    let status: cl_int = unsafe {
        cl_call!(clSetDefaultDeviceCommandQueue(
            context,
            device,
            command_queue
        ))
    };
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}

/// Query device and host timestamps.
/// Calls `clGetDeviceAndHostTimer`
/// `CL_VERSION_2_1`
///
/// * `device` - a valid `OpenCL` device.
///
/// returns a Result containing `device_timestamp` and `host_timestamp` in a 2D array
/// or the error code from the `OpenCL` C API function.
#[cfg(feature = "CL_VERSION_2_1")]
#[inline]
pub fn get_device_and_host_timer(device: cl_device_id) -> Result<[cl_ulong; 2], cl_int> {
    let mut device_timestamp: cl_ulong = 0;
    let mut host_timestamp: cl_ulong = 0;
    let status: cl_int = unsafe {
        cl_call!(clGetDeviceAndHostTimer(
            device,
            &mut device_timestamp,
            &mut host_timestamp
        ))
    };
    if CL_SUCCESS == status {
        Ok([device_timestamp, host_timestamp])
    } else {
        Err(status)
    }
}

/// The current value of the host clock as seen by device.
/// Calls `clGetHostTimer`
/// `CL_VERSION_2_1`
///
/// * `device` - a valid `OpenCL` `device`.
///
/// returns a Result containing `host_timestamp`
/// or the error code from the `OpenCL` C API function.
#[cfg(feature = "CL_VERSION_2_1")]
#[inline]
pub fn get_host_timer(device: cl_device_id) -> Result<cl_ulong, cl_int> {
    let mut host_timestamp: cl_ulong = 0;
    let status: cl_int = unsafe { cl_call!(clGetHostTimer(device, &mut host_timestamp)) };
    if CL_SUCCESS == status {
        Ok(host_timestamp)
    } else {
        Err(status)
    }
}
// #endif

/// Device Vendor Ids.
/// The `PCie` IDs of some `OpenCL` device vendors as returned by `get_device_info`,
/// i.e.: `clGetDeviceInfo` - `CL_DEVICE_VENDOR_ID`
/// They were obtained from the `PCIe` ID Repository: <https://pci-ids.ucw.cz/>
pub const AMD_DEVICE_VENDOR_ID: cl_uint = 0x1002;
pub const IBM_DEVICE_VENDOR_ID: cl_uint = 0x1014;
pub const APPLE_DEVICE_VENDOR_ID: cl_uint = 0x106b;
pub const NVIDIA_DEVICE_VENDOR_ID: cl_uint = 0x10de;
pub const XILINX_DEVICE_VENDOR_ID: cl_uint = 0x10ee;
pub const BROADCOM_DEVICE_VENDOR_ID: cl_uint = 0x1166;
pub const ALTERA_DEVICE_VENDOR_ID: cl_uint = 0x1172;
pub const ARM_DEVICE_VENDOR_ID: cl_uint = 0x13b5;
pub const VIA_TECHNOLOGIES_DEVICE_VENDOR_ID: cl_uint = 0x1412;
pub const TEXAS_INSTRUMENTS_DEVICE_VENDOR_ID: cl_uint = 0x104c;
pub const QUALCOMM_DEVICE_VENDOR_ID: cl_uint = 0x168c;
pub const INTEL_DEVICE_VENDOR_ID: cl_uint = 0x8086;
// Integrated AMD cards on Apple don't have the usual AMD ID
pub const AMD_ON_APPLE_DEVICE_VENDOR_ID: cl_uint = 0x0102_1d00;

/// A text representation of an `OpenCL` vendor id.
#[must_use]
pub const fn vendor_id_text(vendor_id: cl_uint) -> &'static str {
    match vendor_id {
        AMD_DEVICE_VENDOR_ID => "AMD",
        IBM_DEVICE_VENDOR_ID => "IBM",
        NVIDIA_DEVICE_VENDOR_ID => "NVIDIA",
        XILINX_DEVICE_VENDOR_ID => "XILINX",
        BROADCOM_DEVICE_VENDOR_ID => "BROADCOM",
        ALTERA_DEVICE_VENDOR_ID => "ALTERA",
        ARM_DEVICE_VENDOR_ID => "ARM",
        VIA_TECHNOLOGIES_DEVICE_VENDOR_ID => "VIA_TECHNOLOGIES",
        TEXAS_INSTRUMENTS_DEVICE_VENDOR_ID => "TEXAS_INSTRUMENTS",
        QUALCOMM_DEVICE_VENDOR_ID => "QUALCOMM",
        INTEL_DEVICE_VENDOR_ID => "INTEL",
        AMD_ON_APPLE_DEVICE_VENDOR_ID => "AMD_ON_APPLE",

        _ => "UNKNOWN_VENDOR",
    }
}

/// A text representation of an `OpenCL` device type, see:
/// [Device Types](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#device-types-table).
#[must_use]
pub const fn device_type_text(dev_type: cl_device_type) -> &'static str {
    match dev_type {
        CL_DEVICE_TYPE_DEFAULT => "CL_DEVICE_TYPE_DEFAULT",
        CL_DEVICE_TYPE_CPU => "CL_DEVICE_TYPE_CPU",
        CL_DEVICE_TYPE_GPU => "CL_DEVICE_TYPE_GPU",
        CL_DEVICE_TYPE_ACCELERATOR => "CL_DEVICE_TYPE_ACCELERATOR",
        CL_DEVICE_TYPE_CUSTOM => "CL_DEVICE_TYPE_CUSTOM",
        CL_DEVICE_TYPE_ALL => "CL_DEVICE_TYPE_ALL",

        _ => "COMBINED_DEVICE_TYPE",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error_codes::ClError;
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

        // Choose the first platform
        let platform_id = platform_ids[0];

        let device_ids = get_device_ids(platform_id, CL_DEVICE_TYPE_GPU).unwrap();
        println!("CL_DEVICE_TYPE_GPU count: {}", device_ids.len());
        assert!(0 < device_ids.len());

        let device_id = device_ids[0];

        let value = get_device_info(device_id, CL_DEVICE_TYPE).unwrap();
        let value: cl_ulong = value.into();
        println!("CL_DEVICE_TYPE: {}", value);
        println!("Device type is: {}", device_type_text(value));
        assert!(0 < value);
        let value = get_device_info(device_id, CL_DEVICE_VENDOR_ID).unwrap();
        let value: cl_uint = value.into();
        println!("CL_DEVICE_VENDOR_ID: {:X}", value);
        println!("Device vendor is: {}", vendor_id_text(value));
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_VERSION).unwrap();
        let value: String = value.into();
        println!("CL_DEVICE_VERSION: {}", value);
        assert!(!value.is_empty());

        let opencl_2: &str = "OpenCL 2";
        let is_opencl_2: bool = value.contains(opencl_2);

        let opencl_2_1: &str = "OpenCL 2.1";
        let is_opencl_2_1: bool = value.contains(opencl_2_1);

        let value = get_device_info(device_id, CL_DEVICE_MAX_COMPUTE_UNITS).unwrap();
        let value = cl_uint::from(value);
        println!("CL_DEVICE_MAX_COMPUTE_UNITS: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_MAX_WORK_ITEM_DIMENSIONS).unwrap();
        let value = cl_uint::from(value);
        println!("CL_DEVICE_MAX_WORK_ITEM_DIMENSIONS: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_MAX_WORK_GROUP_SIZE).unwrap();
        let value: size_t = From::from(value);
        println!("CL_DEVICE_MAX_WORK_GROUP_SIZE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_MAX_WORK_ITEM_SIZES).unwrap();
        let value = Vec::<size_t>::from(value);
        println!("CL_DEVICE_MAX_WORK_ITEM_SIZES len: {:?}", value.len());
        println!("CL_DEVICE_MAX_WORK_ITEM_SIZES: {:?}", value);
        assert!(0 < value.len());

        let value = get_device_info(device_id, CL_DEVICE_PREFERRED_VECTOR_WIDTH_CHAR).unwrap();
        let value = cl_uint::from(value);
        println!("CL_DEVICE_PREFERRED_VECTOR_WIDTH_CHAR: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_PREFERRED_VECTOR_WIDTH_SHORT).unwrap();
        let value = cl_uint::from(value);
        println!("CL_DEVICE_PREFERRED_VECTOR_WIDTH_SHORT: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_PREFERRED_VECTOR_WIDTH_INT).unwrap();
        let value = cl_uint::from(value);
        println!("CL_DEVICE_PREFERRED_VECTOR_WIDTH_INT: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_PREFERRED_VECTOR_WIDTH_LONG).unwrap();
        let value = cl_uint::from(value);
        println!("CL_DEVICE_PREFERRED_VECTOR_WIDTH_LONG: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_PREFERRED_VECTOR_WIDTH_FLOAT).unwrap();
        let value = cl_uint::from(value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_PREFERRED_VECTOR_WIDTH_DOUBLE).unwrap();
        let value = cl_uint::from(value);
        println!("CL_DEVICE_PREFERRED_VECTOR_WIDTH_DOUBLE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_MAX_CLOCK_FREQUENCY).unwrap();
        let value = cl_uint::from(value);
        println!("CL_DEVICE_MAX_CLOCK_FREQUENCY: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_ADDRESS_BITS).unwrap();
        let value = cl_uint::from(value);
        println!("CL_DEVICE_ADDRESS_BITS: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_MAX_READ_IMAGE_ARGS).unwrap();
        let value = cl_uint::from(value);
        println!("CL_DEVICE_MAX_READ_IMAGE_ARGS: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_MAX_WRITE_IMAGE_ARGS).unwrap();
        let value = cl_uint::from(value);
        println!("CL_DEVICE_MAX_WRITE_IMAGE_ARGS: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_MAX_MEM_ALLOC_SIZE).unwrap();
        let value = cl_ulong::from(value);
        println!("CL_DEVICE_MAX_MEM_ALLOC_SIZE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_IMAGE2D_MAX_WIDTH).unwrap();
        let value = size_t::from(value);
        println!("CL_DEVICE_IMAGE2D_MAX_WIDTH: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_IMAGE2D_MAX_HEIGHT).unwrap();
        let value = size_t::from(value);
        println!("CL_DEVICE_IMAGE2D_MAX_HEIGHT: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_IMAGE3D_MAX_WIDTH).unwrap();
        let value = size_t::from(value);
        println!("CL_DEVICE_IMAGE3D_MAX_WIDTH: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_IMAGE3D_MAX_HEIGHT).unwrap();
        let value = size_t::from(value);
        println!("CL_DEVICE_IMAGE3D_MAX_HEIGHT: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_IMAGE3D_MAX_DEPTH).unwrap();
        let value = size_t::from(value);
        println!("CL_DEVICE_IMAGE3D_MAX_DEPTH: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_IMAGE_SUPPORT).unwrap();
        let value = cl_uint::from(value);
        println!("CL_DEVICE_IMAGE_SUPPORT: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_MAX_PARAMETER_SIZE).unwrap();
        let value = size_t::from(value);
        println!("CL_DEVICE_MAX_PARAMETER_SIZE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_MAX_SAMPLERS).unwrap();
        let value = cl_uint::from(value);
        println!("CL_DEVICE_MAX_SAMPLERS: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_MEM_BASE_ADDR_ALIGN).unwrap();
        let value = cl_uint::from(value);
        println!("CL_DEVICE_MEM_BASE_ADDR_ALIGN: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_MIN_DATA_TYPE_ALIGN_SIZE).unwrap();
        let value = cl_uint::from(value);
        println!("CL_DEVICE_MIN_DATA_TYPE_ALIGN_SIZE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_SINGLE_FP_CONFIG).unwrap();
        let value = cl_ulong::from(value);
        println!("CL_DEVICE_SINGLE_FP_CONFIG: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_GLOBAL_MEM_CACHE_TYPE).unwrap();
        let value = cl_uint::from(value);
        println!("CL_DEVICE_GLOBAL_MEM_CACHE_TYPE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_GLOBAL_MEM_CACHELINE_SIZE).unwrap();
        let value = cl_uint::from(value);
        println!("CL_DEVICE_GLOBAL_MEM_CACHELINE_SIZE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_GLOBAL_MEM_CACHE_SIZE).unwrap();
        let value = cl_ulong::from(value);
        println!("CL_DEVICE_GLOBAL_MEM_CACHE_SIZE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_GLOBAL_MEM_SIZE).unwrap();
        let value = cl_ulong::from(value);
        println!("CL_DEVICE_GLOBAL_MEM_SIZE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_MAX_CONSTANT_BUFFER_SIZE).unwrap();
        let value = cl_ulong::from(value);
        println!("CL_DEVICE_MAX_CONSTANT_BUFFER_SIZE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_MAX_CONSTANT_ARGS).unwrap();
        let value = cl_uint::from(value);
        println!("CL_DEVICE_MAX_CONSTANT_ARGS: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_LOCAL_MEM_TYPE).unwrap();
        let value = cl_uint::from(value);
        println!("CL_DEVICE_LOCAL_MEM_TYPE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_LOCAL_MEM_SIZE).unwrap();
        let value = cl_ulong::from(value);
        println!("CL_DEVICE_LOCAL_MEM_SIZE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_ERROR_CORRECTION_SUPPORT).unwrap();
        let value = cl_uint::from(value);
        println!("CL_DEVICE_ERROR_CORRECTION_SUPPORT: {}", value);

        let value = get_device_info(device_id, CL_DEVICE_PROFILING_TIMER_RESOLUTION).unwrap();
        let value = size_t::from(value);
        println!("CL_DEVICE_PROFILING_TIMER_RESOLUTION: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_ENDIAN_LITTLE).unwrap();
        let value = cl_uint::from(value);
        println!("CL_DEVICE_ENDIAN_LITTLE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_AVAILABLE).unwrap();
        let value = cl_uint::from(value);
        println!("CL_DEVICE_AVAILABLE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_COMPILER_AVAILABLE).unwrap();
        let value = cl_uint::from(value);
        println!("CL_DEVICE_COMPILER_AVAILABLE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_EXECUTION_CAPABILITIES).unwrap();
        let value = cl_ulong::from(value);
        println!("CL_DEVICE_EXECUTION_CAPABILITIES: {}", value);
        assert!(0 < value);

        if is_opencl_2 {
            let value = get_device_info(device_id, CL_DEVICE_QUEUE_ON_HOST_PROPERTIES).unwrap();
            let value = cl_ulong::from(value);
            println!("CL_DEVICE_QUEUE_ON_HOST_PROPERTIES: {}", value);
            assert!(0 < value);
        }

        let value = get_device_info(device_id, CL_DEVICE_NAME).unwrap();
        let value = String::from(value);
        println!("CL_DEVICE_NAME: {}", value);
        assert!(!value.is_empty());

        let value = get_device_info(device_id, CL_DEVICE_VENDOR).unwrap();
        let value = String::from(value);
        println!("CL_DEVICE_VENDOR: {}", value);
        assert!(!value.is_empty());

        let value = get_device_info(device_id, CL_DRIVER_VERSION).unwrap();
        let value = String::from(value);
        println!("CL_DRIVER_VERSION: {}", value);
        assert!(!value.is_empty());

        let value = get_device_info(device_id, CL_DEVICE_PROFILE).unwrap();
        let value = String::from(value);
        println!("CL_DEVICE_PROFILE: {}", value);
        assert!(!value.is_empty());

        let value = get_device_info(device_id, CL_DEVICE_EXTENSIONS).unwrap();
        let value = String::from(value);
        println!("CL_DEVICE_EXTENSIONS: {}", value);
        assert!(!value.is_empty());

        let value = get_device_info(device_id, CL_DEVICE_PLATFORM).unwrap();
        let value = intptr_t::from(value);
        println!("CL_DEVICE_PLATFORM: {}", value);
        assert!(0 < value);

        // Device may not support double fp precision
        match get_device_info(device_id, CL_DEVICE_DOUBLE_FP_CONFIG) {
            Ok(value) => {
                let value = cl_ulong::from(value);
                println!("CL_DEVICE_DOUBLE_FP_CONFIG: {}", value)
            }
            Err(e) => println!("OpenCL error, CL_DEVICE_DOUBLE_FP_CONFIG: {}", ClError(e)),
        };

        // Device may not support half fp precision
        match get_device_info(device_id, CL_DEVICE_HALF_FP_CONFIG) {
            Ok(value) => {
                let value = cl_ulong::from(value);
                println!("CL_DEVICE_HALF_FP_CONFIG: {}", value)
            }
            Err(e) => println!("OpenCL error, CL_DEVICE_HALF_FP_CONFIG: {}", ClError(e)),
        };

        let value = get_device_info(device_id, CL_DEVICE_PREFERRED_VECTOR_WIDTH_HALF).unwrap();
        let value = cl_uint::from(value);
        println!("CL_DEVICE_PREFERRED_VECTOR_WIDTH_HALF: {}", value);

        let value = get_device_info(device_id, CL_DEVICE_NATIVE_VECTOR_WIDTH_CHAR).unwrap();
        let value = cl_uint::from(value);
        println!("CL_DEVICE_NATIVE_VECTOR_WIDTH_CHAR: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_NATIVE_VECTOR_WIDTH_SHORT).unwrap();
        let value = cl_uint::from(value);
        println!("CL_DEVICE_NATIVE_VECTOR_WIDTH_SHORT: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_NATIVE_VECTOR_WIDTH_INT).unwrap();
        let value = cl_uint::from(value);
        println!("CL_DEVICE_NATIVE_VECTOR_WIDTH_INT: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_NATIVE_VECTOR_WIDTH_LONG).unwrap();
        let value = cl_uint::from(value);
        println!("CL_DEVICE_NATIVE_VECTOR_WIDTH_LONG: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_NATIVE_VECTOR_WIDTH_FLOAT).unwrap();
        let value = cl_uint::from(value);
        println!("CL_DEVICE_NATIVE_VECTOR_WIDTH_FLOAT: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_NATIVE_VECTOR_WIDTH_DOUBLE).unwrap();
        let value = cl_uint::from(value);
        println!("CL_DEVICE_NATIVE_VECTOR_WIDTH_DOUBLE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_NATIVE_VECTOR_WIDTH_HALF).unwrap();
        let value = cl_uint::from(value);
        println!("CL_DEVICE_NATIVE_VECTOR_WIDTH_HALF: {}", value);

        let value = get_device_info(device_id, CL_DEVICE_OPENCL_C_VERSION).unwrap();
        let value = String::from(value);
        println!("CL_DEVICE_OPENCL_C_VERSION: {}", value);
        assert!(!value.is_empty());

        let value = get_device_info(device_id, CL_DEVICE_LINKER_AVAILABLE).unwrap();
        let value = cl_uint::from(value);
        println!("CL_DEVICE_LINKER_AVAILABLE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_BUILT_IN_KERNELS).unwrap();
        let value = String::from(value);
        println!("CL_DEVICE_BUILT_IN_KERNELS: {}", value);

        let value = get_device_info(device_id, CL_DEVICE_IMAGE_MAX_BUFFER_SIZE).unwrap();
        let value = size_t::from(value);
        println!("CL_DEVICE_IMAGE_MAX_BUFFER_SIZE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_IMAGE_MAX_ARRAY_SIZE).unwrap();
        let value = size_t::from(value);
        println!("CL_DEVICE_IMAGE_MAX_ARRAY_SIZE: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_PARENT_DEVICE).unwrap();
        let value = intptr_t::from(value);
        println!("CL_DEVICE_PARENT_DEVICE: {}", value);
        assert!(0 == value);

        let value = get_device_info(device_id, CL_DEVICE_PARTITION_MAX_SUB_DEVICES).unwrap();
        let value = cl_uint::from(value);
        println!("CL_DEVICE_PARTITION_MAX_SUB_DEVICES: {}", value);

        let value = get_device_info(device_id, CL_DEVICE_PARTITION_PROPERTIES).unwrap();
        let value = Vec::<intptr_t>::from(value);
        println!("CL_DEVICE_PARTITION_PROPERTIES: {}", value.len());
        println!("CL_DEVICE_PARTITION_PROPERTIES: {:?}", value);
        assert!(0 < value.len());

        let value = get_device_info(device_id, CL_DEVICE_PARTITION_AFFINITY_DOMAIN).unwrap();
        let value = Vec::<cl_ulong>::from(value);
        println!("CL_DEVICE_PARTITION_AFFINITY_DOMAIN: {}", value.len());
        println!("CL_DEVICE_PARTITION_AFFINITY_DOMAIN: {:?}", value);
        assert!(0 < value.len());

        let value = get_device_info(device_id, CL_DEVICE_PARTITION_TYPE).unwrap();
        let value = Vec::<intptr_t>::from(value);
        println!("CL_DEVICE_PARTITION_TYPE: {}", value.len());
        println!("CL_DEVICE_PARTITION_TYPE: {:?}", value);

        let value = get_device_info(device_id, CL_DEVICE_REFERENCE_COUNT).unwrap();
        let value = cl_uint::from(value);
        println!("CL_DEVICE_REFERENCE_COUNT: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_PREFERRED_INTEROP_USER_SYNC).unwrap();
        let value = cl_uint::from(value);
        println!("CL_DEVICE_PREFERRED_INTEROP_USER_SYNC: {}", value);

        let value = get_device_info(device_id, CL_DEVICE_PRINTF_BUFFER_SIZE).unwrap();
        let value = size_t::from(value);
        println!("CL_DEVICE_PRINTF_BUFFER_SIZE: {}", value);
        assert!(0 < value);

        // cl_khr_device_uuid extension
        match get_device_info(device_id, CL_DEVICE_UUID_KHR) {
            Ok(value) => {
                println!("CL_DEVICE_UUID_KHR: {}", value);
            }
            Err(e) => println!("OpenCL error, CL_DEVICE_UUID_KHR: {}", ClError(e)),
        };

        // cl_khr_device_uuid extension
        match get_device_info(device_id, CL_DRIVER_UUID_KHR) {
            Ok(value) => {
                println!("CL_DRIVER_UUID_KHR: {}", value);
            }
            Err(e) => println!("OpenCL error, CL_DRIVER_UUID_KHR: {}", ClError(e)),
        };

        // cl_khr_device_uuid extension
        match get_device_info(device_id, CL_DEVICE_LUID_VALID_KHR) {
            Ok(value) => {
                let value = cl_uint::from(value);
                println!("CL_DEVICE_LUID_VALID_KHR: {:?}", value);
            }
            Err(e) => println!("OpenCL error, CL_DEVICE_LUID_VALID_KHR: {}", ClError(e)),
        };

        // cl_khr_device_uuid extension
        match get_device_info(device_id, CL_DEVICE_LUID_KHR) {
            Ok(value) => {
                println!("CL_DEVICE_LUID_KHR: {}", value);
            }
            Err(e) => println!("OpenCL error, CL_DEVICE_LUID_KHR: {}", ClError(e)),
        };

        // cl_khr_device_uuid extension
        match get_device_info(device_id, CL_DEVICE_NODE_MASK_KHR) {
            Ok(value) => {
                let value = cl_uint::from(value);
                println!("CL_DEVICE_NODE_MASK_KHR: {:?}", value);
            }
            Err(e) => println!("OpenCL error, CL_DEVICE_NODE_MASK_KHR: {}", ClError(e)),
        };

        // Nvidia specific extension
        match get_device_info(device_id, CL_DEVICE_COMPUTE_CAPABILITY_MAJOR_NV) {
            Ok(value) => {
                let value = cl_uint::from(value);
                println!("CL_DEVICE_COMPUTE_CAPABILITY_MAJOR_NV: {}", value)
            }
            Err(e) => println!(
                "OpenCL error, CL_DEVICE_COMPUTE_CAPABILITY_MAJOR_NV: {}",
                ClError(e)
            ),
        };

        // Nvidia specific extension
        match get_device_info(device_id, CL_DEVICE_COMPUTE_CAPABILITY_MINOR_NV) {
            Ok(value) => {
                let value = cl_uint::from(value);
                println!("CL_DEVICE_COMPUTE_CAPABILITY_MINOR_NV: {}", value)
            }
            Err(e) => println!(
                "OpenCL error, CL_DEVICE_COMPUTE_CAPABILITY_MINOR_NV: {}",
                ClError(e)
            ),
        };

        // Nvidia specific extension
        match get_device_info(device_id, CL_DEVICE_REGISTERS_PER_BLOCK_NV) {
            Ok(value) => {
                let value = cl_uint::from(value);
                println!("CL_DEVICE_REGISTERS_PER_BLOCK_NV: {}", value)
            }
            Err(e) => println!(
                "OpenCL error, CL_DEVICE_REGISTERS_PER_BLOCK_NV: {}",
                ClError(e)
            ),
        };

        // Nvidia specific extension
        match get_device_info(device_id, CL_DEVICE_WARP_SIZE_NV) {
            Ok(value) => {
                let value = cl_uint::from(value);
                println!("CL_DEVICE_WARP_SIZE_NV: {}", value)
            }
            Err(e) => println!("OpenCL error, CL_DEVICE_WARP_SIZE_NV: {}", ClError(e)),
        };

        // Nvidia specific extension
        match get_device_info(device_id, CL_DEVICE_GPU_OVERLAP_NV) {
            Ok(value) => {
                let value = cl_uint::from(value);
                println!("CL_DEVICE_GPU_OVERLAP_NV: {}", value)
            }
            Err(e) => println!("OpenCL error, CL_DEVICE_GPU_OVERLAP_NV: {}", ClError(e)),
        };

        // Nvidia specific extension
        match get_device_info(device_id, CL_DEVICE_KERNEL_EXEC_TIMEOUT_NV) {
            Ok(value) => {
                let value = cl_uint::from(value);
                println!("CL_DEVICE_KERNEL_EXEC_TIMEOUT_NV: {}", value)
            }
            Err(e) => println!(
                "OpenCL error, CL_DEVICE_KERNEL_EXEC_TIMEOUT_NV: {}",
                ClError(e)
            ),
        };

        // Nvidia specific extension
        match get_device_info(device_id, CL_DEVICE_INTEGRATED_MEMORY_NV) {
            Ok(value) => {
                let value = cl_uint::from(value);
                println!("CL_DEVICE_INTEGRATED_MEMORY_NV: {}", value)
            }
            Err(e) => println!(
                "OpenCL error, CL_DEVICE_INTEGRATED_MEMORY_NV: {}",
                ClError(e)
            ),
        };

        // Nvidia specific extension, undocumented
        match get_device_info(device_id, CL_DEVICE_PCI_BUS_ID_NV) {
            Ok(value) => {
                let value = cl_uint::from(value);
                println!("CL_DEVICE_PCI_BUS_ID_NV: {}", value)
            }
            Err(e) => println!("OpenCL error, CL_DEVICE_PCI_BUS_ID_NV: {}", ClError(e)),
        };

        // Nvidia specific extension, undocumented
        match get_device_info(device_id, CL_DEVICE_PCI_SLOT_ID_NV) {
            Ok(value) => {
                let value = cl_uint::from(value);
                println!("CL_DEVICE_PCI_SLOT_ID_NV: {}", value)
            }
            Err(e) => println!("OpenCL error, CL_DEVICE_PCI_SLOT_ID_NV: {}", ClError(e)),
        };

        match get_device_info(device_id, CL_DEVICE_PROFILING_TIMER_OFFSET_AMD) {
            Ok(value) => {
                let value = size_t::from(value);
                println!("CL_DEVICE_PROFILING_TIMER_OFFSET_AMD: {}", value)
            }
            Err(e) => println!(
                "OpenCL error, CL_DEVICE_PROFILING_TIMER_OFFSET_AMD: {}",
                ClError(e)
            ),
        };

        match get_device_info(device_id, CL_DEVICE_TOPOLOGY_AMD) {
            Ok(value) => {
                let value = Vec::<u8>::from(value);
                println!("CL_DEVICE_TOPOLOGY_AMD: {:?}", value);

                let topology = get_amd_device_topology(&value);
                println!("CL_DEVICE_TOPOLOGY_AMD bus: {}", topology.bus);
                println!("CL_DEVICE_TOPOLOGY_AMD device: {}", topology.device);
                println!("CL_DEVICE_TOPOLOGY_AMD function: {}", topology.function);
            }
            Err(e) => println!("OpenCL error, CL_DEVICE_TOPOLOGY_AMD: {}", ClError(e)),
        };

        match get_device_info(device_id, CL_DEVICE_BOARD_NAME_AMD) {
            Ok(value) => {
                let value = String::from(value);
                println!("CL_DEVICE_BOARD_NAME_AMD: {}", value)
            }
            Err(e) => println!("OpenCL error, CL_DEVICE_BOARD_NAME_AMD: {}", ClError(e)),
        };

        match get_device_info(device_id, CL_DEVICE_GLOBAL_FREE_MEMORY_AMD) {
            Ok(value) => {
                let value = size_t::from(value);
                println!("CL_DEVICE_GLOBAL_FREE_MEMORY_AMD: {}", value)
            }
            Err(e) => println!(
                "OpenCL error, CL_DEVICE_GLOBAL_FREE_MEMORY_AMD: {}",
                ClError(e)
            ),
        };

        match get_device_info(device_id, CL_DEVICE_SIMD_PER_COMPUTE_UNIT_AMD) {
            Ok(value) => {
                let value = cl_uint::from(value);
                println!("CL_DEVICE_SIMD_PER_COMPUTE_UNIT_AMD: {}", value)
            }
            Err(e) => println!(
                "OpenCL error, CL_DEVICE_SIMD_PER_COMPUTE_UNIT_AMD: {}",
                ClError(e)
            ),
        };

        match get_device_info(device_id, CL_DEVICE_SIMD_WIDTH_AMD) {
            Ok(value) => {
                let value = cl_uint::from(value);
                println!("CL_DEVICE_SIMD_WIDTH_AMD: {}", value)
            }
            Err(e) => println!("OpenCL error, CL_DEVICE_SIMD_WIDTH_AMD: {}", ClError(e)),
        };

        match get_device_info(device_id, CL_DEVICE_SIMD_INSTRUCTION_WIDTH_AMD) {
            Ok(value) => {
                let value = cl_uint::from(value);
                println!("CL_DEVICE_SIMD_INSTRUCTION_WIDTH_AMD: {}", value)
            }
            Err(e) => println!(
                "OpenCL error, CL_DEVICE_SIMD_INSTRUCTION_WIDTH_AMD: {}",
                ClError(e)
            ),
        };

        match get_device_info(device_id, CL_DEVICE_WAVEFRONT_WIDTH_AMD) {
            Ok(value) => {
                let value = cl_uint::from(value);
                println!("CL_DEVICE_WAVEFRONT_WIDTH_AMD: {}", value)
            }
            Err(e) => println!(
                "OpenCL error, CL_DEVICE_WAVEFRONT_WIDTH_AMD: {}",
                ClError(e)
            ),
        };

        match get_device_info(device_id, CL_DEVICE_GLOBAL_MEM_CHANNELS_AMD) {
            Ok(value) => {
                let value = cl_uint::from(value);
                println!("CL_DEVICE_GLOBAL_MEM_CHANNELS_AMD: {}", value)
            }
            Err(e) => println!(
                "OpenCL error, CL_DEVICE_GLOBAL_MEM_CHANNELS_AMD: {}",
                ClError(e)
            ),
        };

        match get_device_info(device_id, CL_DEVICE_GLOBAL_MEM_CHANNEL_BANKS_AMD) {
            Ok(value) => {
                let value = cl_uint::from(value);
                println!("CL_DEVICE_GLOBAL_MEM_CHANNEL_BANKS_AMD: {}", value)
            }
            Err(e) => println!(
                "OpenCL error, CL_DEVICE_GLOBAL_MEM_CHANNEL_BANKS_AMD: {}",
                ClError(e)
            ),
        };

        match get_device_info(device_id, CL_DEVICE_GLOBAL_MEM_CHANNEL_BANK_WIDTH_AMD) {
            Ok(value) => {
                let value = cl_uint::from(value);
                println!("CL_DEVICE_GLOBAL_MEM_CHANNEL_BANK_WIDTH_AMD: {}", value)
            }
            Err(e) => println!(
                "OpenCL error, CL_DEVICE_GLOBAL_MEM_CHANNEL_BANK_WIDTH_AMD: {}",
                ClError(e)
            ),
        };

        match get_device_info(device_id, CL_DEVICE_LOCAL_MEM_SIZE_PER_COMPUTE_UNIT_AMD) {
            Ok(value) => {
                let value = cl_uint::from(value);
                println!("CL_DEVICE_LOCAL_MEM_SIZE_PER_COMPUTE_UNIT_AMD: {}", value)
            }
            Err(e) => println!(
                "OpenCL error, CL_DEVICE_LOCAL_MEM_SIZE_PER_COMPUTE_UNIT_AMD: {}",
                ClError(e)
            ),
        };

        match get_device_info(device_id, CL_DEVICE_LOCAL_MEM_BANKS_AMD) {
            Ok(value) => {
                let value = cl_uint::from(value);
                println!("CL_DEVICE_LOCAL_MEM_BANKS_AMD: {}", value)
            }
            Err(e) => println!(
                "OpenCL error, CL_DEVICE_LOCAL_MEM_BANKS_AMD: {}",
                ClError(e)
            ),
        };

        match get_device_info(device_id, CL_DEVICE_THREAD_TRACE_SUPPORTED_AMD) {
            Ok(value) => {
                let value = cl_uint::from(value);
                println!("CL_DEVICE_THREAD_TRACE_SUPPORTED_AMD: {}", value)
            }
            Err(e) => println!(
                "OpenCL error, CL_DEVICE_THREAD_TRACE_SUPPORTED_AMD: {}",
                ClError(e)
            ),
        };

        match get_device_info(device_id, CL_DEVICE_GFXIP_MAJOR_AMD) {
            Ok(value) => {
                let value = cl_uint::from(value);
                println!("CL_DEVICE_GFXIP_MAJOR_AMD: {}", value)
            }
            Err(e) => println!("OpenCL error, CL_DEVICE_GFXIP_MAJOR_AMD: {}", ClError(e)),
        };

        match get_device_info(device_id, CL_DEVICE_GFXIP_MINOR_AMD) {
            Ok(value) => {
                let value = cl_uint::from(value);
                println!("CL_DEVICE_GFXIP_MINOR_AMD: {}", value)
            }
            Err(e) => println!("OpenCL error, CL_DEVICE_GFXIP_MINOR_AMD: {}", ClError(e)),
        };

        match get_device_info(device_id, CL_DEVICE_AVAILABLE_ASYNC_QUEUES_AMD) {
            Ok(value) => {
                let value = cl_uint::from(value);
                println!("CL_DEVICE_AVAILABLE_ASYNC_QUEUES_AMD: {}", value)
            }
            Err(e) => println!(
                "OpenCL error, CL_DEVICE_AVAILABLE_ASYNC_QUEUES_AMD: {}",
                ClError(e)
            ),
        };

        match get_device_info(device_id, CL_DEVICE_PREFERRED_WORK_GROUP_SIZE_AMD) {
            Ok(value) => {
                let value = size_t::from(value);
                println!("CL_DEVICE_PREFERRED_WORK_GROUP_SIZE_AMD: {}", value)
            }
            Err(e) => println!(
                "OpenCL error, CL_DEVICE_PREFERRED_WORK_GROUP_SIZE_AMD: {}",
                ClError(e)
            ),
        };

        match get_device_info(device_id, CL_DEVICE_MAX_WORK_GROUP_SIZE_AMD) {
            Ok(value) => {
                let value = size_t::from(value);
                println!("CL_DEVICE_MAX_WORK_GROUP_SIZE_AMD: {}", value)
            }
            Err(e) => println!(
                "OpenCL error, CL_DEVICE_MAX_WORK_GROUP_SIZE_AMD: {}",
                ClError(e)
            ),
        };

        match get_device_info(device_id, CL_DEVICE_PREFERRED_CONSTANT_BUFFER_SIZE_AMD) {
            Ok(value) => {
                let value = size_t::from(value);
                println!("CL_DEVICE_PREFERRED_CONSTANT_BUFFER_SIZE_AMD: {}", value)
            }
            Err(e) => println!(
                "OpenCL error, CL_DEVICE_PREFERRED_CONSTANT_BUFFER_SIZE_AMD: {}",
                ClError(e)
            ),
        };

        match get_device_info(device_id, CL_DEVICE_PCIE_ID_AMD) {
            Ok(value) => {
                let value = cl_uint::from(value);
                println!("CL_DEVICE_PCIE_ID_AMD: {}", value)
            }
            Err(e) => println!("OpenCL error, CL_DEVICE_PCIE_ID_AMD: {}", ClError(e)),
        };

        match get_device_info(device_id, CL_DEVICE_PCI_BUS_INFO_KHR) {
            Ok(value) => {
                let value = Vec::<u8>::from(value);
                println!("CL_DEVICE_PCI_BUS_INFO_KHR: {:?}", value);

                let pci_bus_info = get_device_pci_bus_info_khr(&value);
                println!(
                    "CL_DEVICE_PCI_BUS_INFO_KHR pci_domain: {}",
                    pci_bus_info.pci_domain
                );
                println!(
                    "CL_DEVICE_PCI_BUS_INFO_KHR pci_bus: {}",
                    pci_bus_info.pci_bus
                );
                println!(
                    "CL_DEVICE_PCI_BUS_INFO_KHR pci_device: {}",
                    pci_bus_info.pci_device
                );
                println!(
                    "CL_DEVICE_PCI_BUS_INFO_KHR pci_function: {}",
                    pci_bus_info.pci_function
                );
            }
            Err(e) => println!("OpenCL error, CL_DEVICE_PCI_BUS_INFO_KHR: {}", ClError(e)),
        };

        match get_device_info(device_id, CL_DEVICE_IP_VERSION_INTEL) {
            Ok(value) => {
                let value = cl_uint::from(value);
                println!("CL_DEVICE_IP_VERSION_INTEL: {:?}", value)
            }
            Err(e) => println!("OpenCL error, CL_DEVICE_IP_VERSION_INTEL: {}", ClError(e)),
        };

        match get_device_info(device_id, CL_DEVICE_ID_INTEL) {
            Ok(value) => {
                let value = cl_uint::from(value);
                println!("CL_DEVICE_ID_INTEL: {}", value)
            }
            Err(e) => println!("OpenCL error, CL_DEVICE_ID_INTEL: {}", ClError(e)),
        };

        match get_device_info(device_id, CL_DEVICE_NUM_SLICES_INTEL) {
            Ok(value) => {
                let value = cl_uint::from(value);
                println!("CL_DEVICE_NUM_SLICES_INTEL: {}", value)
            }
            Err(e) => println!("OpenCL error, CL_DEVICE_NUM_SLICES_INTEL: {}", ClError(e)),
        };

        match get_device_info(device_id, CL_DEVICE_NUM_SUB_SLICES_PER_SLICE_INTEL) {
            Ok(value) => {
                let value = cl_uint::from(value);
                println!("CL_DEVICE_NUM_SUB_SLICES_PER_SLICE_INTEL: {}", value)
            }
            Err(e) => println!(
                "OpenCL error, CL_DEVICE_NUM_SUB_SLICES_PER_SLICE_INTEL: {}",
                ClError(e)
            ),
        };

        match get_device_info(device_id, CL_DEVICE_NUM_EUS_PER_SUB_SLICE_INTEL) {
            Ok(value) => {
                let value = cl_uint::from(value);
                println!("CL_DEVICE_NUM_EUS_PER_SUB_SLICE_INTEL: {}", value)
            }
            Err(e) => println!(
                "OpenCL error, CL_DEVICE_NUM_EUS_PER_SUB_SLICE_INTEL: {}",
                ClError(e)
            ),
        };

        match get_device_info(device_id, CL_DEVICE_NUM_THREADS_PER_EU_INTEL) {
            Ok(value) => {
                let value = cl_uint::from(value);
                println!("CL_DEVICE_NUM_THREADS_PER_EU_INTEL: {}", value)
            }
            Err(e) => println!(
                "OpenCL error, CL_DEVICE_NUM_THREADS_PER_EU_INTEL: {}",
                ClError(e)
            ),
        };

        match get_device_info(device_id, CL_DEVICE_FEATURE_CAPABILITIES_INTEL) {
            Ok(value) => {
                let value = cl_ulong::from(value);
                println!("CL_DEVICE_FEATURE_CAPABILITIES_INTEL: {}", value)
            }
            Err(e) => println!(
                "OpenCL error, CL_DEVICE_FEATURE_CAPABILITIES_INTEL: {}",
                ClError(e)
            ),
        };

        match get_device_info(device_id, CL_DEVICE_EXTERNAL_MEMORY_IMPORT_HANDLE_TYPES_KHR) {
            Ok(value) => {
                let value: Vec<u32> = value.into();
                println!(
                    "CL_DEVICE_EXTERNAL_MEMORY_IMPORT_HANDLE_TYPES_KHR: {:?}",
                    value
                )
            }
            Err(e) => println!(
                "OpenCL error, CL_DEVICE_EXTERNAL_MEMORY_IMPORT_HANDLE_TYPES_KHR: {}",
                ClError(e)
            ),
        };

        match get_device_info(device_id, CL_DEVICE_SEMAPHORE_IMPORT_HANDLE_TYPES_KHR) {
            Ok(value) => {
                let value: Vec<u32> = value.into();
                println!("CL_DEVICE_SEMAPHORE_IMPORT_HANDLE_TYPES_KHR: {:?}", value)
            }
            Err(e) => println!(
                "OpenCL error, CL_DEVICE_SEMAPHORE_IMPORT_HANDLE_TYPES_KHR: {}",
                ClError(e)
            ),
        };

        match get_device_info(device_id, CL_DEVICE_SEMAPHORE_EXPORT_HANDLE_TYPES_KHR) {
            Ok(value) => {
                let value: Vec<u32> = value.into();
                println!("CL_DEVICE_SEMAPHORE_EXPORT_HANDLE_TYPES_KHR: {:?}", value)
            }
            Err(e) => println!(
                "OpenCL error, CL_DEVICE_SEMAPHORE_EXPORT_HANDLE_TYPES_KHR: {}",
                ClError(e)
            ),
        };

        match get_device_info(device_id, CL_DEVICE_SEMAPHORE_TYPES_KHR) {
            Ok(value) => {
                let value: Vec<u32> = value.into();
                println!("CL_DEVICE_SEMAPHORE_TYPES_KHR: {:?}", value)
            }
            Err(e) => println!(
                "OpenCL error, CL_DEVICE_SEMAPHORE_TYPES_KHR: {}",
                ClError(e)
            ),
        };

        match get_device_info(device_id, CL_DEVICE_COMMAND_BUFFER_CAPABILITIES_KHR) {
            Ok(value) => {
                let value: cl_ulong = value.into();
                println!("CL_DEVICE_COMMAND_BUFFER_CAPABILITIES_KHR: {}", value)
            }
            Err(e) => println!(
                "OpenCL error, CL_DEVICE_COMMAND_BUFFER_CAPABILITIES_KHR: {}",
                ClError(e)
            ),
        };

        match get_device_info(
            device_id,
            CL_DEVICE_COMMAND_BUFFER_REQUIRED_QUEUE_PROPERTIES_KHR,
        ) {
            Ok(value) => {
                let value: cl_ulong = value.into();
                println!(
                    "CL_DEVICE_COMMAND_BUFFER_REQUIRED_QUEUE_PROPERTIES_KHR: {}",
                    value
                )
            }
            Err(e) => println!(
                "OpenCL error, CL_DEVICE_COMMAND_BUFFER_REQUIRED_QUEUE_PROPERTIES_KHR: {}",
                ClError(e)
            ),
        };

        // CL_VERSION_2_0
        if is_opencl_2 {
            let value = get_device_info(device_id, CL_DEVICE_IMAGE_PITCH_ALIGNMENT).unwrap();
            let value = cl_uint::from(value);
            println!("CL_DEVICE_IMAGE_PITCH_ALIGNMENT: {}", value);
            assert!(0 < value);

            let value = get_device_info(device_id, CL_DEVICE_IMAGE_BASE_ADDRESS_ALIGNMENT).unwrap();
            let value = cl_uint::from(value);
            println!("CL_DEVICE_IMAGE_BASE_ADDRESS_ALIGNMENT: {}", value);
            assert!(0 < value);

            let value = get_device_info(device_id, CL_DEVICE_MAX_READ_WRITE_IMAGE_ARGS).unwrap();
            let value = cl_uint::from(value);
            println!("CL_DEVICE_MAX_READ_WRITE_IMAGE_ARGS: {}", value);
            assert!(0 < value);

            let value = get_device_info(device_id, CL_DEVICE_MAX_GLOBAL_VARIABLE_SIZE).unwrap();
            let value = size_t::from(value);
            println!("CL_DEVICE_MAX_GLOBAL_VARIABLE_SIZE: {}", value);
            assert!(0 < value);

            let value = get_device_info(device_id, CL_DEVICE_QUEUE_ON_DEVICE_PROPERTIES).unwrap();
            let value = Vec::<intptr_t>::from(value);
            println!("CL_DEVICE_QUEUE_ON_DEVICE_PROPERTIES: {}", value.len());
            println!("CL_DEVICE_QUEUE_ON_DEVICE_PROPERTIES: {:?}", value);
            assert!(0 < value.len());

            let value =
                get_device_info(device_id, CL_DEVICE_QUEUE_ON_DEVICE_PREFERRED_SIZE).unwrap();
            let value = size_t::from(value);
            println!("CL_DEVICE_QUEUE_ON_DEVICE_PREFERRED_SIZE: {}", value);
            assert!(0 < value);

            let value = get_device_info(device_id, CL_DEVICE_QUEUE_ON_DEVICE_MAX_SIZE).unwrap();
            let value = size_t::from(value);
            println!("CL_DEVICE_QUEUE_ON_DEVICE_MAX_SIZE: {}", value);
            assert!(0 < value);

            let value = get_device_info(device_id, CL_DEVICE_MAX_ON_DEVICE_QUEUES).unwrap();
            let value = cl_uint::from(value);
            println!("CL_DEVICE_MAX_ON_DEVICE_QUEUES: {}", value);
            assert!(0 < value);

            let value = get_device_info(device_id, CL_DEVICE_MAX_ON_DEVICE_EVENTS).unwrap();
            let value = cl_uint::from(value);
            println!("CL_DEVICE_MAX_ON_DEVICE_EVENTS: {}", value);
            assert!(0 < value);

            let value = get_device_info(device_id, CL_DEVICE_SVM_CAPABILITIES).unwrap();
            let value = cl_ulong::from(value);
            println!("CL_DEVICE_SVM_CAPABILITIES: {}", value);
            assert!(0 < value);

            let value =
                get_device_info(device_id, CL_DEVICE_GLOBAL_VARIABLE_PREFERRED_TOTAL_SIZE).unwrap();
            let value = size_t::from(value);
            println!("CL_DEVICE_GLOBAL_VARIABLE_PREFERRED_TOTAL_SIZE: {}", value);
            assert!(0 < value);

            let value = get_device_info(device_id, CL_DEVICE_MAX_PIPE_ARGS).unwrap();
            let value = cl_uint::from(value);
            println!("CL_DEVICE_MAX_PIPE_ARGS: {}", value);
            assert!(0 < value);

            let value = get_device_info(device_id, CL_DEVICE_PIPE_MAX_ACTIVE_RESERVATIONS).unwrap();
            let value = cl_uint::from(value);
            println!("CL_DEVICE_PIPE_MAX_ACTIVE_RESERVATIONS: {}", value);
            assert!(0 < value);

            let value = get_device_info(device_id, CL_DEVICE_PIPE_MAX_PACKET_SIZE).unwrap();
            let value = cl_uint::from(value);
            println!("CL_DEVICE_PIPE_MAX_PACKET_SIZE: {}", value);
            assert!(0 < value);

            let value =
                get_device_info(device_id, CL_DEVICE_PREFERRED_PLATFORM_ATOMIC_ALIGNMENT).unwrap();
            let value = cl_uint::from(value);
            println!("CL_DEVICE_PREFERRED_PLATFORM_ATOMIC_ALIGNMENT: {}", value);
            // assert!(0 < value);

            let value =
                get_device_info(device_id, CL_DEVICE_PREFERRED_GLOBAL_ATOMIC_ALIGNMENT).unwrap();
            let value = cl_uint::from(value);
            println!("CL_DEVICE_PREFERRED_GLOBAL_ATOMIC_ALIGNMENT: {}", value);
            // assert!(0 < value);

            let value =
                get_device_info(device_id, CL_DEVICE_PREFERRED_LOCAL_ATOMIC_ALIGNMENT).unwrap();
            let value = cl_uint::from(value);
            println!("CL_DEVICE_PREFERRED_LOCAL_ATOMIC_ALIGNMENT: {}", value);
            // assert!(0 < value);

            // CL_VERSION_2_1
            if is_opencl_2_1 {
                let value = get_device_info(device_id, CL_DEVICE_IL_VERSION).unwrap();
                let value = String::from(value);
                println!("CL_DEVICE_IL_VERSION: {}", value);
                assert!(!value.is_empty());

                let value = get_device_info(device_id, CL_DEVICE_MAX_NUM_SUB_GROUPS).unwrap();
                let value = cl_uint::from(value);
                println!("CL_DEVICE_MAX_NUM_SUB_GROUPS: {}", value);
                assert!(0 < value);

                let value =
                    get_device_info(device_id, CL_DEVICE_SUB_GROUP_INDEPENDENT_FORWARD_PROGRESS)
                        .unwrap();
                let value = cl_uint::from(value);
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
        let platform_id = if platform_ids.len() > 1 {
            platform_ids[1]
        } else {
            platform_ids[0]
        };

        let device_ids = get_device_ids(platform_id, CL_DEVICE_TYPE_GPU).unwrap();
        println!("CL_DEVICE_TYPE_GPU count: {}", device_ids.len());
        assert!(0 < device_ids.len());

        let device_id = device_ids[0];

        // CL_VERSION_3_0
        let value = if let Ok(value) = get_device_info(device_id, CL_DEVICE_NUMERIC_VERSION) {
            value
        } else {
            println!("OpenCL device doesn't support OpenCL 3.0 API");
            return;
        };

        let value = cl_uint::from(value);
        println!("CL_DEVICE_NUMERIC_VERSION: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_EXTENSIONS_WITH_VERSION).unwrap();
        let value = Vec::<cl_name_version>::from(value);
        println!("CL_DEVICE_EXTENSIONS_WITH_VERSION: {}", value.len());
        println!("CL_DEVICE_EXTENSIONS_WITH_VERSION: {:?}", value);
        assert!(0 < value.len());

        let value = get_device_info(device_id, CL_DEVICE_ILS_WITH_VERSION).unwrap();
        let value = Vec::<cl_name_version>::from(value);
        println!("CL_DEVICE_ILS_WITH_VERSION: {}", value.len());
        println!("CL_DEVICE_ILS_WITH_VERSION: {:?}", value);
        assert!(0 < value.len());

        let value = get_device_info(device_id, CL_DEVICE_BUILT_IN_KERNELS_WITH_VERSION).unwrap();
        let value = Vec::<cl_name_version>::from(value);
        println!("CL_DEVICE_BUILT_IN_KERNELS_WITH_VERSION: {}", value.len());
        println!("CL_DEVICE_BUILT_IN_KERNELS_WITH_VERSION: {:?}", value);
        assert!(0 < value.len());

        let value = get_device_info(device_id, CL_DEVICE_ATOMIC_MEMORY_CAPABILITIES).unwrap();
        let value = cl_ulong::from(value);
        println!("CL_DEVICE_ATOMIC_MEMORY_CAPABILITIES: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_ATOMIC_MEMORY_CAPABILITIES).unwrap();
        let value = cl_ulong::from(value);
        println!("CL_DEVICE_ATOMIC_FENCE_CAPABILITIES: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_NON_UNIFORM_WORK_GROUP_SUPPORT).unwrap();
        let value = cl_uint::from(value);
        println!("CL_DEVICE_NON_UNIFORM_WORK_GROUP_SUPPORT: {}", value);
        // assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_OPENCL_C_ALL_VERSIONS).unwrap();
        let value = Vec::<cl_name_version>::from(value);
        println!("CL_DEVICE_OPENCL_C_ALL_VERSIONS: {}", value.len());
        println!("CL_DEVICE_OPENCL_C_ALL_VERSIONS: {:?}", value);
        assert!(0 < value.len());

        let value =
            get_device_info(device_id, CL_DEVICE_PREFERRED_WORK_GROUP_SIZE_MULTIPLE).unwrap();
        let value = size_t::from(value);
        println!("CL_DEVICE_PREFERRED_WORK_GROUP_SIZE_MULTIPLE: {}", value);
        assert!(0 < value);

        let value =
            get_device_info(device_id, CL_DEVICE_WORK_GROUP_COLLECTIVE_FUNCTIONS_SUPPORT).unwrap();
        let value = cl_uint::from(value);
        println!(
            "CL_DEVICE_WORK_GROUP_COLLECTIVE_FUNCTIONS_SUPPORT: {}",
            value
        );
        // assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_GENERIC_ADDRESS_SPACE_SUPPORT).unwrap();
        let value = cl_uint::from(value);
        println!("CL_DEVICE_GENERIC_ADDRESS_SPACE_SUPPORT: {}", value);
        // assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_OPENCL_C_FEATURES).unwrap();
        let value = Vec::<cl_name_version>::from(value);
        println!("CL_DEVICE_OPENCL_C_FEATURES: {}", value.len());
        println!("CL_DEVICE_OPENCL_C_FEATURES: {:?}", value);
        assert!(0 < value.len());

        let value = get_device_info(device_id, CL_DEVICE_DEVICE_ENQUEUE_CAPABILITIES).unwrap();
        let value = cl_ulong::from(value);
        println!("CL_DEVICE_DEVICE_ENQUEUE_CAPABILITIES: {}", value);
        assert!(0 < value);

        let value = get_device_info(device_id, CL_DEVICE_PIPE_SUPPORT).unwrap();
        let value = cl_uint::from(value);
        println!("CL_DEVICE_PIPE_SUPPORT: {}", value);
        // assert!(0 < value);

        let value =
            get_device_info(device_id, CL_DEVICE_LATEST_CONFORMANCE_VERSION_PASSED).unwrap();
        let value = String::from(value);
        println!("CL_DEVICE_LATEST_CONFORMANCE_VERSION_PASSED: {}", value);
        assert!(!value.is_empty());
    }

    #[cfg(feature = "CL_VERSION_1_2")]
    #[test]
    fn test_get_sub_devices() {
        let platform_ids = get_platform_ids().unwrap();
        assert!(0 < platform_ids.len());

        // Find an `OpenCL` device with sub devices

        let mut device_id = ptr::null_mut();
        let mut has_sub_devices: bool = false;

        for p in platform_ids {
            let device_ids = get_device_ids(p, CL_DEVICE_TYPE_CPU).unwrap();

            for dev_id in device_ids {
                let value = get_device_info(dev_id, CL_DEVICE_PARTITION_MAX_SUB_DEVICES).unwrap();
                let max_sub_devices = cl_uint::from(value);

                has_sub_devices = 1 < max_sub_devices;
                if has_sub_devices {
                    device_id = dev_id;
                    break;
                }
            }
        }

        if has_sub_devices {
            let properties: [cl_device_partition_property; 3] = [CL_DEVICE_PARTITION_EQUALLY, 2, 0];
            let sub_devices = create_sub_devices(device_id, &properties).unwrap();

            println!("CL_DEVICE_TYPE_CPU count: {}", sub_devices.len());
            assert!(0 < sub_devices.len());

            for device in sub_devices {
                unsafe { release_device(device).unwrap() };
            }
        } else {
            println!("OpenCL device capable of sub division not found");
        }
    }
}
