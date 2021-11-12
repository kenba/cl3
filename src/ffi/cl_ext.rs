// Copyright (c) 2021 Via Technology Ltd. All Rights Reserved.
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

//! FFI bindings for cl_ext.h  
//! cl_ext.h contains OpenCL extensions that don't have external (OpenGL, D3D) dependencies.  
//! OpenCL extensions are documented in the [OpenCL-Registry](https://github.com/KhronosGroup/OpenCL-Registry)

#![allow(non_camel_case_types, non_upper_case_globals)]

pub use cl_sys::{
    cl_bitfield, cl_bool, cl_channel_type, cl_command_queue, cl_command_queue_properties,
    cl_command_type, cl_context, cl_device_id, cl_device_info, cl_event, cl_event_info,
    cl_image_format, cl_int, cl_kernel, cl_kernel_exec_info, cl_kernel_info, cl_map_flags, cl_mem,
    cl_mem_flags, cl_mem_info, cl_mem_migration_flags, cl_platform_id, cl_platform_info,
    cl_program, cl_program_info, cl_queue_properties, cl_sampler_properties, cl_uchar, cl_uint,
    cl_ulong,
};

use libc::{c_void, intptr_t, size_t};

// pub const CL_DEVICE_DOUBLE_FP_CONFIG: cl_device_info = 0x1032;
// pub const CL_DEVICE_HALF_FP_CONFIG: cl_device_info = 0x1033;

pub const CL_PLATFORM_ICD_SUFFIX_KHR: cl_platform_info = 0x0920;

/// New property to clGetDeviceInfo for retrieving supported intermediate languages.
pub const CL_DEVICE_IL_VERSION_KHR: cl_device_info = 0x105B;

/// New property to clGetProgramInfo for retrieving for retrieving the IL of a program.
pub const CL_PROGRAM_IL_KHR: cl_program_info = 0x1169;

pub const CL_DEVICE_IMAGE_PITCH_ALIGNMENT_KHR: cl_device_info = 0x104A;
pub const CL_DEVICE_IMAGE_BASE_ADDRESS_ALIGNMENT_KHR: cl_device_info = 0x104B;

pub const CL_CONTEXT_MEMORY_INITIALIZE_KHR: cl_uint = 0x2030;

pub const CL_CONTEXT_TERMINATED_KHR: cl_int = -1121;

pub const CL_DEVICE_TERMINATE_CAPABILITY_KHR: cl_uint = 0x2031;
pub const CL_CONTEXT_TERMINATE_KHR: cl_uint = 0x2032;

pub const CL_DEVICE_SPIR_VERSIONS: cl_uint = 0x40E0;
pub const CL_PROGRAM_BINARY_TYPE_INTERMEDIATE: cl_uint = 0x40E1;

pub type cl_nv_device_attribute_query = cl_uint;
pub const CL_DEVICE_COMPUTE_CAPABILITY_MAJOR_NV: cl_nv_device_attribute_query = 0x4000;
pub const CL_DEVICE_COMPUTE_CAPABILITY_MINOR_NV: cl_nv_device_attribute_query = 0x4001;
pub const CL_DEVICE_REGISTERS_PER_BLOCK_NV: cl_nv_device_attribute_query = 0x4002;
pub const CL_DEVICE_WARP_SIZE_NV: cl_nv_device_attribute_query = 0x4003;
pub const CL_DEVICE_GPU_OVERLAP_NV: cl_nv_device_attribute_query = 0x4004;
pub const CL_DEVICE_KERNEL_EXEC_TIMEOUT_NV: cl_nv_device_attribute_query = 0x4005;
pub const CL_DEVICE_INTEGRATED_MEMORY_NV: cl_nv_device_attribute_query = 0x4006;

// undocumented tokens for clGetDeviceInfo, see: https://anteru.net/blog/2014/associating-opencl-device-ids-with-gpus/
pub const CL_DEVICE_PCI_BUS_ID_NV: cl_nv_device_attribute_query = 0x4008;
pub const CL_DEVICE_PCI_SLOT_ID_NV: cl_nv_device_attribute_query = 0x4009;

pub type cl_amd_device_attribute_query = cl_uint;

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct cl_amd_device_topology {
    r#type: u32,
    unused: [u8; 17],
    pub bus: u8,
    pub device: u8,
    pub function: u8,
}

pub const CL_DEVICE_PROFILING_TIMER_OFFSET_AMD: cl_amd_device_attribute_query = 0x4036;
pub const CL_DEVICE_TOPOLOGY_AMD: cl_amd_device_attribute_query = 0x4037;
pub const CL_DEVICE_BOARD_NAME_AMD: cl_amd_device_attribute_query = 0x4038;
pub const CL_DEVICE_GLOBAL_FREE_MEMORY_AMD: cl_amd_device_attribute_query = 0x4039;
pub const CL_DEVICE_SIMD_PER_COMPUTE_UNIT_AMD: cl_amd_device_attribute_query = 0x4040;
pub const CL_DEVICE_SIMD_WIDTH_AMD: cl_amd_device_attribute_query = 0x4041;
pub const CL_DEVICE_SIMD_INSTRUCTION_WIDTH_AMD: cl_amd_device_attribute_query = 0x4042;
pub const CL_DEVICE_WAVEFRONT_WIDTH_AMD: cl_amd_device_attribute_query = 0x4043;
pub const CL_DEVICE_GLOBAL_MEM_CHANNELS_AMD: cl_amd_device_attribute_query = 0x4044;
pub const CL_DEVICE_GLOBAL_MEM_CHANNEL_BANKS_AMD: cl_amd_device_attribute_query = 0x4045;
pub const CL_DEVICE_GLOBAL_MEM_CHANNEL_BANK_WIDTH_AMD: cl_amd_device_attribute_query = 0x4046;
pub const CL_DEVICE_LOCAL_MEM_SIZE_PER_COMPUTE_UNIT_AMD: cl_amd_device_attribute_query = 0x4047;
pub const CL_DEVICE_LOCAL_MEM_BANKS_AMD: cl_amd_device_attribute_query = 0x4048;
pub const CL_DEVICE_THREAD_TRACE_SUPPORTED_AMD: cl_amd_device_attribute_query = 0x4049;
pub const CL_DEVICE_GFXIP_MAJOR_AMD: cl_amd_device_attribute_query = 0x404A;
pub const CL_DEVICE_GFXIP_MINOR_AMD: cl_amd_device_attribute_query = 0x404B;
pub const CL_DEVICE_AVAILABLE_ASYNC_QUEUES_AMD: cl_amd_device_attribute_query = 0x404C;
pub const CL_DEVICE_PREFERRED_WORK_GROUP_SIZE_AMD: cl_amd_device_attribute_query = 0x4030;
pub const CL_DEVICE_MAX_WORK_GROUP_SIZE_AMD: cl_amd_device_attribute_query = 0x4031;
pub const CL_DEVICE_PREFERRED_CONSTANT_BUFFER_SIZE_AMD: cl_amd_device_attribute_query = 0x4033;
pub const CL_DEVICE_PCIE_ID_AMD: cl_amd_device_attribute_query = 0x4034;

pub const CL_PRINTF_CALLBACK_ARM: cl_uint = 0x40B0;
pub const CL_PRINTF_BUFFERSIZE_ARM: cl_uint = 0x40B1;

pub type cl_device_partition_property_ext = cl_ulong;
pub const CL_DEVICE_PARTITION_EQUALLY_EXT: cl_device_partition_property_ext = 0x4050;
pub const CL_DEVICE_PARTITION_BY_COUNTS_EXT: cl_device_partition_property_ext = 0x4051;
pub const CL_DEVICE_PARTITION_BY_NAMES_EXT: cl_device_partition_property_ext = 0x4052;
pub const CL_DEVICE_PARTITION_BY_AFFINITY_DOMAIN_EXT: cl_device_partition_property_ext = 0x4053;

pub const CL_DEVICE_PARENT_DEVICE_EXT: cl_device_info = 0x4054;
pub const CL_DEVICE_PARTITION_TYPES_EXT: cl_device_info = 0x4055;
pub const CL_DEVICE_AFFINITY_DOMAINS_EXT: cl_device_info = 0x4056;
pub const CL_DEVICE_REFERENCE_COUNT_EXT: cl_device_info = 0x4057;
pub const CL_DEVICE_PARTITION_STYLE_EXT: cl_device_info = 0x4058;

// error codes
pub const CL_DEVICE_PARTITION_FAILED_EXT: cl_int = -1057;
pub const CL_INVALID_PARTITION_COUNT_EXT: cl_int = -1058;
pub const CL_INVALID_PARTITION_NAME_EXT: cl_int = -1059;

pub const CL_AFFINITY_DOMAIN_L1_CACHE_EXT: cl_uint = 0x1;
pub const CL_AFFINITY_DOMAIN_L2_CACHE_EXT: cl_uint = 0x2;
pub const CL_AFFINITY_DOMAIN_L3_CACHE_EXT: cl_uint = 0x3;
pub const CL_AFFINITY_DOMAIN_L4_CACHE_EXT: cl_uint = 0x4;
pub const CL_AFFINITY_DOMAIN_NUMA_EXT: cl_uint = 0x10;
pub const CL_AFFINITY_DOMAIN_NEXT_FISSIONABLE_EXT: cl_uint = 0x100;

pub const CL_PROPERTIES_LIST_END_EXT: cl_device_partition_property_ext = 0;
pub const CL_PARTITION_BY_COUNTS_LIST_END_EXT: cl_device_partition_property_ext = 0;
pub const CL_PARTITION_BY_NAMES_LIST_END_EXT: cl_device_partition_property_ext = 0xFFFFFFFF;

pub type cl_properties = cl_ulong;
pub type cl_queue_properties_khr = cl_properties;

// cl_ext_migrate_memobject extension

pub type cl_mem_migration_flags_ext = cl_bitfield;
pub const CL_MIGRATE_MEM_OBJECT_HOST_EXT: cl_mem_migration_flags_ext = 0x1;
pub const CL_COMMAND_MIGRATE_MEM_OBJECT_EXT: cl_mem_migration_flags_ext = 0x4040;

// cl_ext_cxx_for_opencl
pub const CL_DEVICE_CXX_FOR_OPENCL_NUMERIC_VERSION_EXT: cl_uint = 0x4230;

// cl_qcom_ext_host_ptr
pub type cl_qcom_ext_host_ptr = cl_uint;
pub const CL_MEM_EXT_HOST_PTR_QCOM: cl_qcom_ext_host_ptr = 1 << 29;

pub const CL_DEVICE_EXT_MEM_PADDING_IN_BYTES_QCOM: cl_qcom_ext_host_ptr = 0x40A0;
pub const CL_DEVICE_PAGE_SIZE_QCOM: cl_qcom_ext_host_ptr = 0x40A1;
pub const CL_IMAGE_ROW_ALIGNMENT_QCOM: cl_qcom_ext_host_ptr = 0x40A2;
pub const CL_IMAGE_SLICE_ALIGNMENT_QCOM: cl_qcom_ext_host_ptr = 0x40A3;
pub const CL_MEM_HOST_UNCACHED_QCOM: cl_qcom_ext_host_ptr = 0x40A4;
pub const CL_MEM_HOST_WRITEBACK_QCOM: cl_qcom_ext_host_ptr = 0x40A5;
pub const CL_MEM_HOST_WRITETHROUGH_QCOM: cl_qcom_ext_host_ptr = 0x40A6;
pub const CL_MEM_HOST_WRITE_COMBINING_QCOM: cl_qcom_ext_host_ptr = 0x40A7;

pub type cl_image_pitch_info_qcom = cl_uint;

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct cl_mem_ext_host_ptr {
    pub allocation_type: cl_uint,
    pub host_cache_policy: cl_uint,
}

pub const CL_MEM_ION_HOST_PTR_QCOM: cl_qcom_ext_host_ptr = 0x40A8;
pub const CL_MEM_HOST_IOCOHERENT_QCOM: cl_qcom_ext_host_ptr = 0x40A9;

#[derive(Debug)]
#[repr(C)]
pub struct cl_mem_ion_host_ptr {
    pub ext_host_ptr: cl_mem_ext_host_ptr,
    pub ion_filedesc: cl_int,
    pub ion_hostptr: *mut c_void,
}

pub const CL_MEM_ANDROID_NATIVE_BUFFER_HOST_PTR_QCOM: cl_qcom_ext_host_ptr = 0x40C6;

#[derive(Debug)]
#[repr(C)]
pub struct cl_mem_android_native_buffer_host_ptr {
    pub ext_host_ptr: cl_mem_ext_host_ptr,
    pub anb_ptr: *mut c_void,
}

// cl_img_yuv_image extension
pub const CL_NV21_IMG: cl_channel_type = 0x40D0;
pub const CL_YV12_IMG: cl_channel_type = 0x40D1;

// cl_img_cached_allocations extension
pub const CL_MEM_USE_UNCACHED_CPU_MEMORY_IMG: cl_mem_flags = 1 << 26;
pub const CL_MEM_USE_CACHED_CPU_MEMORY_IMG: cl_mem_flags = 1 << 27;

// cl_img_use_gralloc_ptr extension
pub const CL_MEM_USE_GRALLOC_PTR_IMG: cl_mem_flags = 1 << 28;

pub const CL_COMMAND_ACQUIRE_GRALLOC_OBJECTS_IMG: cl_event_info = 0x40D2;
pub const CL_COMMAND_RELEASE_GRALLOC_OBJECTS_IMG: cl_event_info = 0x40D3;

pub const CL_GRALLOC_RESOURCE_NOT_ACQUIRED_IMG: cl_int = 0x40D4;
pub const CL_INVALID_GRALLOC_OBJECT_IMG: cl_int = 0x40D5;

// cl_img_generate_mipmap extension
pub type cl_mipmap_filter_mode_img = cl_uint;
pub const CL_MIPMAP_FILTER_ANY_IMG: cl_mipmap_filter_mode_img = 0x0;
pub const CL_MIPMAP_FILTER_BOX_IMG: cl_mipmap_filter_mode_img = 0x1;

pub const CL_COMMAND_GENERATE_MIPMAP_IMG: cl_event_info = 0x40D6;

// cl_img_mem_properties extension
pub const CL_MEM_ALLOC_FLAGS_IMG: cl_properties = 0x40D7;

pub type cl_mem_alloc_flags_img = cl_bitfield;
pub const CL_MEM_ALLOC_RELAX_REQUIREMENTS_IMG: cl_mem_alloc_flags_img = 1 << 0;

// cl_khr_subgroups extension
pub type cl_kernel_sub_group_info = cl_uint;

pub const CL_KERNEL_MAX_SUB_GROUP_SIZE_FOR_NDRANGE_KHR: cl_kernel_sub_group_info = 0x2033;
pub const CL_KERNEL_SUB_GROUP_COUNT_FOR_NDRANGE_KHR: cl_kernel_sub_group_info = 0x2034;

// cl_khr_mipmap_image extension
pub const CL_SAMPLER_MIP_FILTER_MODE_KHR: cl_sampler_properties = 0x1155;
pub const CL_SAMPLER_LOD_MIN_KHR: cl_sampler_properties = 0x1156;
pub const CL_SAMPLER_LOD_MAX_KHR: cl_sampler_properties = 0x1157;

// cl_khr_priority_hints extension
pub type cl_queue_priority_khr = cl_uint;

pub const CL_QUEUE_PRIORITY_KHR: cl_command_queue_properties = 0x1096;

pub const CL_QUEUE_PRIORITY_HIGH_KHR: cl_queue_priority_khr = 1 << 0;
pub const CL_QUEUE_PRIORITY_MED_KHR: cl_queue_priority_khr = 1 << 1;
pub const CL_QUEUE_PRIORITY_LOW_KHR: cl_queue_priority_khr = 1 << 2;

// cl_khr_throttle_hints extension
pub type cl_queue_throttle_khr = cl_uint;

pub const CL_QUEUE_THROTTLE_KHR: cl_command_queue_properties = 0x1097;

pub const CL_QUEUE_THROTTLE_HIGH_KHR: cl_queue_throttle_khr = 1 << 0;
pub const CL_QUEUE_THROTTLE_MED_KHR: cl_queue_throttle_khr = 1 << 1;
pub const CL_QUEUE_THROTTLE_LOW_KHR: cl_queue_throttle_khr = 1 << 2;

// cl_khr_subgroup_named_barrier

pub const CL_DEVICE_MAX_NAMED_BARRIER_COUNT_KHR: cl_device_info = 0x2035;

// cl_khr_extended_versioning
pub type cl_version_khr = cl_uint;

pub const CL_VERSION_MAJOR_BITS_KHR: cl_version_khr = 10;
pub const CL_VERSION_MINOR_BITS_KHR: cl_version_khr = 10;
pub const CL_VERSION_PATCH_BITS_KHR: cl_version_khr = 12;

pub const CL_VERSION_MAJOR_MASK_KHR: cl_version_khr = (1 << CL_VERSION_MAJOR_BITS_KHR) - 1;
pub const CL_VERSION_MINOR_MASK_KHR: cl_version_khr = (1 << CL_VERSION_MINOR_BITS_KHR) - 1;
pub const CL_VERSION_PATCH_MASK_KHR: cl_version_khr = (1 << CL_VERSION_PATCH_BITS_KHR) - 1;

#[inline]
pub fn version_major_khr(version: cl_version_khr) -> cl_version_khr {
    version >> (CL_VERSION_MINOR_BITS_KHR + CL_VERSION_PATCH_BITS_KHR)
}

#[inline]
pub fn version_minor_khr(version: cl_version_khr) -> cl_version_khr {
    (version >> CL_VERSION_PATCH_BITS_KHR) & CL_VERSION_MINOR_MASK_KHR
}

#[inline]
pub fn version_patch_khr(version: cl_version_khr) -> cl_version_khr {
    version & CL_VERSION_PATCH_MASK_KHR
}

#[inline]
pub fn make_version_khr(
    major: cl_version_khr,
    minor: cl_version_khr,
    patch: cl_version_khr,
) -> cl_version_khr {
    ((major & CL_VERSION_MAJOR_MASK_KHR) << (CL_VERSION_MINOR_BITS_KHR + CL_VERSION_PATCH_BITS_KHR))
        | ((minor & CL_VERSION_MINOR_MASK_KHR) << CL_VERSION_PATCH_BITS_KHR)
        | (patch & CL_VERSION_PATCH_MASK_KHR)
}

pub const CL_NAME_VERSION_MAX_NAME_SIZE_KHR: usize = 64;

#[derive(Debug)]
#[repr(C)]
pub struct cl_name_version_khr {
    pub version: cl_version_khr,
    pub name: [cl_uchar; CL_NAME_VERSION_MAX_NAME_SIZE_KHR],
}

pub const CL_PLATFORM_NUMERIC_VERSION_KHR: cl_platform_info = 0x0906;
pub const CL_PLATFORM_EXTENSIONS_WITH_VERSION_KHR: cl_platform_info = 0x0907;

pub const CL_DEVICE_NUMERIC_VERSION_KHR: cl_device_info = 0x105E;
pub const CL_DEVICE_OPENCL_C_NUMERIC_VERSION_KHR: cl_device_info = 0x105F;
pub const CL_DEVICE_EXTENSIONS_WITH_VERSION_KHR: cl_device_info = 0x1060;
pub const CL_DEVICE_ILS_WITH_VERSION_KHR: cl_device_info = 0x1061;
pub const CL_DEVICE_BUILT_IN_KERNELS_WITH_VERSION_KHR: cl_device_info = 0x1062;

// cl_khr_device_uuid extension

pub const CL_UUID_SIZE_KHR: usize = 16;
pub const CL_LUID_SIZE_KHR: usize = 8;

pub const CL_DEVICE_UUID_KHR: cl_device_info = 0x106A;
pub const CL_DRIVER_UUID_KHR: cl_device_info = 0x106B;
pub const CL_DEVICE_LUID_VALID_KHR: cl_device_info = 0x106C;
pub const CL_DEVICE_LUID_KHR: cl_device_info = 0x106D;
pub const CL_DEVICE_NODE_MASK_KHR: cl_device_info = 0x106E;

// cl_khr_pci_bus_info
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct cl_device_pci_bus_info_khr {
    pub pci_domain: cl_uint,
    pub pci_bus: cl_uint,
    pub pci_device: cl_uint,
    pub pci_function: cl_uint,
}

pub const CL_DEVICE_PCI_BUS_INFO_KHR: cl_device_info = 0x410F;

// cl_khr_integer_dot_product

pub type cl_device_integer_dot_product_capabilities_khr = cl_bitfield;
pub const CL_DEVICE_INTEGER_DOT_PRODUCT_INPUT_4x8BIT_PACKED_KHR:
    cl_device_integer_dot_product_capabilities_khr = 1 << 0;
pub const CL_DEVICE_INTEGER_DOT_PRODUCT_INPUT_4x8BIT_KHR:
    cl_device_integer_dot_product_capabilities_khr = 1 << 1;

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct cl_device_integer_dot_product_acceleration_properties_khr {
    pub signed_accelerated: cl_bool,
    pub unsigned_accelerated: cl_bool,
    pub mixed_signedness_accelerated: cl_bool,
    pub accumulating_saturating_signed_accelerated: cl_bool,
    pub accumulating_saturating_unsigned_accelerated: cl_bool,
    pub accumulating_saturating_mixed_signedness_accelerated: cl_bool,
}

pub const CL_DEVICE_INTEGER_DOT_PRODUCT_CAPABILITIES_KHR: cl_device_info = 0x1073;
pub const CL_DEVICE_INTEGER_DOT_PRODUCT_ACCELERATION_PROPERTIES_8BIT_KHR: cl_device_info = 0x1074;
pub const CL_DEVICE_INTEGER_DOT_PRODUCT_ACCELERATION_PROPERTIES_4x8BIT_PACKED_KHR: cl_device_info =
    0x1075;

// cl_khr_external_memory

pub type cl_external_memory_handle_type_khr = cl_uint;

pub const CL_PLATFORM_EXTERNAL_MEMORY_IMPORT_HANDLE_TYPES_KHR: cl_platform_info = 0x2044;

pub const CL_DEVICE_EXTERNAL_MEMORY_IMPORT_HANDLE_TYPES_KHR: cl_device_info = 0x204F;

// cl_mem_properties
pub const CL_DEVICE_HANDLE_LIST_KHR: cl_ulong = 0x2051;
pub const CL_DEVICE_HANDLE_LIST_END_KHR: cl_ulong = 0;

pub const CL_COMMAND_ACQUIRE_EXTERNAL_MEM_OBJECTS_KHR: cl_command_type = 0x2047;
pub const CL_COMMAND_RELEASE_EXTERNAL_MEM_OBJECTS_KHR: cl_command_type = 0x2048;

// cl_khr_external_memory_dma_buf
pub const CL_EXTERNAL_MEMORY_HANDLE_DMA_BUF_KHR: cl_external_memory_handle_type_khr = 0x2067;

// cl_khr_external_memory_dx
pub const CL_EXTERNAL_MEMORY_HANDLE_D3D11_TEXTURE_KHR: cl_external_memory_handle_type_khr = 0x2063;
pub const CL_EXTERNAL_MEMORY_HANDLE_D3D11_TEXTURE_KMT_KHR: cl_external_memory_handle_type_khr =
    0x2064;
pub const CL_EXTERNAL_MEMORY_HANDLE_D3D12_HEAP_KHR: cl_external_memory_handle_type_khr = 0x2065;
pub const CL_EXTERNAL_MEMORY_HANDLE_D3D12_RESOURCE_KHR: cl_external_memory_handle_type_khr = 0x2066;

// cl_khr_external_memory_opaque_fd
pub const CL_EXTERNAL_MEMORY_HANDLE_OPAQUE_FD_KHR: cl_external_memory_handle_type_khr = 0x2060;

// cl_khr_external_memory_win32
pub const CL_EXTERNAL_MEMORY_HANDLE_OPAQUE_WIN32_KHR: cl_external_memory_handle_type_khr = 0x2061;
pub const CL_EXTERNAL_MEMORY_HANDLE_OPAQUE_WIN32_KMT_KHR: cl_external_memory_handle_type_khr =
    0x2062;

// cl_khr_external_semaphore
pub type cl_semaphore_khr = *mut c_void;
pub type cl_external_semaphore_handle_type_khr = cl_uint;
pub type cl_semaphore_properties_khr = cl_properties;

pub const CL_PLATFORM_SEMAPHORE_IMPORT_HANDLE_TYPES_KHR: cl_platform_info = 0x2037;
pub const CL_PLATFORM_SEMAPHORE_EXPORT_HANDLE_TYPES_KHR: cl_platform_info = 0x2038;

pub const CL_DEVICE_SEMAPHORE_IMPORT_HANDLE_TYPES_KHR: cl_device_info = 0x204D;
pub const CL_DEVICE_SEMAPHORE_EXPORT_HANDLE_TYPES_KHR: cl_device_info = 0x204E;

pub const CL_SEMAPHORE_EXPORT_HANDLE_TYPES_KHR: cl_semaphore_properties_khr = 0x203F;
pub const CL_SEMAPHORE_EXPORT_HANDLE_TYPES_LIST_END_KHR: cl_semaphore_properties_khr = 0;

// cl_khr_external_semaphore_dx_fence
pub const CL_SEMAPHORE_HANDLE_D3D12_FENCE_KHR: cl_external_semaphore_handle_type_khr = 0x2059;

// cl_khr_external_semaphore_opaque_fd
pub const CL_SEMAPHORE_HANDLE_OPAQUE_FD_KHR: cl_external_semaphore_handle_type_khr = 0x2055;

// cl_khr_external_semaphore_sync_fd
pub const CL_SEMAPHORE_HANDLE_SYNC_FD_KHR: cl_external_semaphore_handle_type_khr = 0x2058;

// cl_khr_external_semaphore_win32
pub const CL_SEMAPHORE_HANDLE_OPAQUE_WIN32_KHR: cl_external_semaphore_handle_type_khr = 0x2056;
pub const CL_SEMAPHORE_HANDLE_OPAQUE_WIN32_KMT_KHR: cl_external_semaphore_handle_type_khr = 0x2057;

// cl_khr_semaphore
pub type cl_semaphore_info_khr = cl_uint;
pub type cl_semaphore_type_khr = cl_uint;
pub type cl_semaphore_payload_khr = cl_ulong;

pub const CL_SEMAPHORE_TYPE_BINARY_KHR: cl_semaphore_type_khr = 1;

pub const CL_PLATFORM_SEMAPHORE_TYPES_KHR: cl_platform_info = 0x2036;

pub const CL_DEVICE_SEMAPHORE_TYPES_KHR: cl_device_info = 0x204C;

pub const CL_SEMAPHORE_CONTEXT_KHR: cl_semaphore_info_khr = 0x2039;
pub const CL_SEMAPHORE_REFERENCE_COUNT_KHR: cl_semaphore_info_khr = 0x203A;
pub const CL_SEMAPHORE_PROPERTIES_KHR: cl_semaphore_info_khr = 0x203B;
pub const CL_SEMAPHORE_PAYLOAD_KHR: cl_semaphore_info_khr = 0x203C;

pub const CL_SEMAPHORE_TYPE_KHR: cl_semaphore_info_khr = 0x203D;

pub const CL_COMMAND_SEMAPHORE_WAIT_KHR: cl_command_type = 0x2042;
pub const CL_COMMAND_SEMAPHORE_SIGNAL_KHR: cl_command_type = 0x2043;

// Error codes
pub const CL_INVALID_SEMAPHORE_KHR: cl_int = -1142;

// cl_arm_import_memory extension

pub type cl_import_properties_arm = intptr_t;

/// Default and valid properties name for cl_arm_import_memory
pub const CL_IMPORT_TYPE_ARM: cl_import_properties_arm = 0x40B2;

/// Host process memory type default value for CL_IMPORT_TYPE_ARM property
pub const CL_IMPORT_TYPE_HOST_ARM: cl_import_properties_arm = 0x40B3;

/// DMA BUF memory type value for CL_IMPORT_TYPE_ARM property
pub const CL_IMPORT_TYPE_DMA_BUF_ARM: cl_import_properties_arm = 0x40B4;

/// Protected memory property
pub const CL_IMPORT_TYPE_PROTECTED_ARM: cl_import_properties_arm = 0x40B5;

/// Android hardware buffer type value for CL_IMPORT_TYPE_ARM property
pub const CL_IMPORT_TYPE_ANDROID_HARDWARE_BUFFER_ARM: cl_import_properties_arm = 0x41E2;

/// Data consistency with host property
pub const CL_IMPORT_DMA_BUF_DATA_CONSISTENCY_WITH_HOST_ARM: cl_import_properties_arm = 0x41E3;

/// Index of plane in a multiplanar hardware buffer
pub const CL_IMPORT_ANDROID_HARDWARE_BUFFER_PLANE_INDEX_ARM: cl_import_properties_arm = 0x41EF;

/// Index of layer in a multilayer hardware buffer
pub const CL_IMPORT_ANDROID_HARDWARE_BUFFER_LAYER_INDEX_ARM: cl_import_properties_arm = 0x41F0;

/// Import memory size value to indicate a size for the whole buffer.
pub const CL_IMPORT_MEMORY_WHOLE_ALLOCATION_ARM: cl_import_properties_arm =
    cl_import_properties_arm::MAX;

pub const CL_DEVICE_SVM_CAPABILITIES_ARM: cl_device_info = 0x40B6;

pub const CL_MEM_USES_SVM_POINTER_ARM: cl_mem_info = 0x40B7;

pub type cl_kernel_exec_info_arm = cl_uint;
pub const CL_KERNEL_EXEC_INFO_SVM_PTRS_ARM: cl_kernel_exec_info_arm = 0x40B8;
pub const CL_KERNEL_EXEC_INFO_SVM_FINE_GRAIN_SYSTEM_ARM: cl_kernel_exec_info_arm = 0x40B9;

pub const CL_COMMAND_SVM_FREE_ARM: cl_event_info = 0x40BA;
pub const CL_COMMAND_SVM_MEMCPY_ARM: cl_event_info = 0x40BB;
pub const CL_COMMAND_SVM_MEMFILL_ARM: cl_event_info = 0x40BC;
pub const CL_COMMAND_SVM_MAP_ARM: cl_event_info = 0x40BD;
pub const CL_COMMAND_SVM_UNMAP_ARM: cl_event_info = 0x40BF;

pub type cl_device_svm_capabilities_arm = cl_bitfield;
pub const CL_DEVICE_SVM_COARSE_GRAIN_BUFFER_ARM: cl_device_svm_capabilities_arm = 1 << 0;
pub const CL_DEVICE_SVM_FINE_GRAIN_BUFFER_ARM: cl_device_svm_capabilities_arm = 1 << 1;
pub const CL_DEVICE_SVM_FINE_GRAIN_SYSTEM_ARM: cl_device_svm_capabilities_arm = 1 << 2;
pub const CL_DEVICE_SVM_ATOMICS_ARM: cl_device_svm_capabilities_arm = 1 << 3;

pub type cl_svm_mem_flags_arm = cl_bitfield;
pub const CL_MEM_SVM_FINE_GRAIN_BUFFER_ARM: cl_svm_mem_flags_arm = 1 << 10;
pub const CL_MEM_SVM_ATOMICS_ARM: cl_svm_mem_flags_arm = 1 << 11;

// cl_arm_get_core_id extension
pub const CL_DEVICE_COMPUTE_UNITS_BITFIELD_ARM: cl_device_info = 0x40BF;

// cl_arm_job_slot_selection
pub const CL_DEVICE_JOB_SLOTS_ARM: cl_device_info = 0x41E0;
pub const CL_QUEUE_JOB_SLOT_ARM: cl_device_info = 0x41E1;

// cl_arm_scheduling_controls
pub const CL_DEVICE_SCHEDULING_CONTROLS_CAPABILITIES_ARM: cl_device_info = 0x41E4;

pub type cl_device_scheduling_controls_capabilities_arm = cl_bitfield;
pub const CL_DEVICE_SCHEDULING_KERNEL_BATCHING_ARM: cl_device_scheduling_controls_capabilities_arm =
    1 << 0;
pub const CL_DEVICE_SCHEDULING_WORKGROUP_BATCH_SIZE_ARM:
    cl_device_scheduling_controls_capabilities_arm = 1 << 1;
pub const CL_DEVICE_SCHEDULING_WORKGROUP_BATCH_SIZE_MODIFIER_ARM:
    cl_device_scheduling_controls_capabilities_arm = 1 << 2;
pub const CL_DEVICE_SCHEDULING_DEFERRED_FLUSH_ARM: cl_device_scheduling_controls_capabilities_arm =
    1 << 3;
pub const CL_DEVICE_SCHEDULING_REGISTER_ALLOCATION_ARM:
    cl_device_scheduling_controls_capabilities_arm = 1 << 4;

pub const CL_DEVICE_SUPPORTED_REGISTER_ALLOCATIONS_ARM: cl_device_info = 0x41EB;

pub const CL_KERNEL_EXEC_INFO_WORKGROUP_BATCH_SIZE_ARM: cl_kernel_info = 0x41E5;
pub const CL_KERNEL_EXEC_INFO_WORKGROUP_BATCH_SIZE_MODIFIER_ARM: cl_kernel_info = 0x41E6;

pub const CL_QUEUE_KERNEL_BATCHING_ARM: cl_queue_properties = 0x41E7;
pub const CL_QUEUE_DEFERRED_FLUSH_ARM: cl_queue_properties = 0x41EC;

// cl_arm_controlled_kernel_termination
pub const CL_COMMAND_TERMINATED_ITSELF_WITH_FAILURE_ARM: cl_int = -1108;

pub const CL_DEVICE_CONTROLLED_TERMINATION_CAPABILITIES_ARM: cl_device_info = 0x41EE;

pub type cl_device_controlled_termination_capabilities_arm = cl_bitfield;
pub const CL_DEVICE_CONTROLLED_TERMINATION_SUCCESS_ARM:
    cl_device_controlled_termination_capabilities_arm = 1 << 0;
pub const CL_DEVICE_CONTROLLED_TERMINATION_FAILURE_ARM:
    cl_device_controlled_termination_capabilities_arm = 1 << 1;
pub const CL_DEVICE_CONTROLLED_TERMINATION_QUERY_ARM:
    cl_device_controlled_termination_capabilities_arm = 1 << 2;

pub const CL_EVENT_COMMAND_TERMINATION_REASON_ARM: cl_event_info = 0x41ED;

// Values returned for event termination reason query
pub type cl_command_termination_reason_arm = cl_uint;
pub const CL_COMMAND_TERMINATION_COMPLETION_ARM: cl_command_termination_reason_arm = 0;
pub const CL_COMMAND_TERMINATION_CONTROLLED_SUCCESS_ARM: cl_command_termination_reason_arm = 1;
pub const CL_COMMAND_TERMINATION_CONTROLLED_FAILURE_ARM: cl_command_termination_reason_arm = 2;
pub const CL_COMMAND_TERMINATION_ERROR_ARM: cl_command_termination_reason_arm = 3;

// cl_intel_thread_local_exec extension
pub const CL_QUEUE_THREAD_LOCAL_EXEC_ENABLE_INTEL: cl_bitfield = 1 << 31;

// cl_intel_device_attribute_query
pub type cl_device_feature_capabilities_intel = cl_bitfield;
pub const CL_DEVICE_FEATURE_FLAG_DP4A_INTEL: cl_device_feature_capabilities_intel = 1 << 0;
pub const CL_DEVICE_FEATURE_FLAG_DPAS_INTEL: cl_device_feature_capabilities_intel = 1 << 1;

pub const CL_DEVICE_IP_VERSION_INTEL: cl_device_info = 0x4250;
pub const CL_DEVICE_ID_INTEL: cl_device_info = 0x4251;
pub const CL_DEVICE_NUM_SLICES_INTEL: cl_device_info = 0x4252;
pub const CL_DEVICE_NUM_SUB_SLICES_PER_SLICE_INTEL: cl_device_info = 0x4253;
pub const CL_DEVICE_NUM_EUS_PER_SUB_SLICE_INTEL: cl_device_info = 0x4254;
pub const CL_DEVICE_NUM_THREADS_PER_EU_INTEL: cl_device_info = 0x4255;
pub const CL_DEVICE_FEATURE_CAPABILITIES_INTEL: cl_device_info = 0x4256;

// cl_intel_device_partition_by_names extension

pub const CL_DEVICE_PARTITION_BY_NAMES_INTEL: cl_device_info = 0x4052;

// cl_intel_accelerator extension
// cl_intel_motion_estimation extension
// cl_intel_advanced_motion_estimation extension

pub type cl_accelerator_intel = *mut c_void;

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct cl_motion_estimation_desc_intel {
    pub mb_block_type: cl_uint,
    pub subpixel_mode: cl_uint,
    pub sad_adjust_mode: cl_uint,
    pub search_path_type: cl_uint,
}

// error codes
pub const CL_INVALID_ACCELERATOR_INTEL: cl_int = -1094;
pub const CL_INVALID_ACCELERATOR_TYPE_INTEL: cl_int = -1095;
pub const CL_INVALID_ACCELERATOR_DESCRIPTOR_INTEL: cl_int = -1096;
pub const CL_ACCELERATOR_TYPE_NOT_SUPPORTED_INTEL: cl_int = -1097;

pub type cl_accelerator_type_intel = cl_uint;
pub const CL_ACCELERATOR_TYPE_MOTION_ESTIMATION_INTEL: cl_accelerator_type_intel = 0x0;

pub type cl_accelerator_info_intel = cl_uint;
pub const CL_ACCELERATOR_DESCRIPTOR_INTEL: cl_accelerator_info_intel = 0x4090;
pub const CL_ACCELERATOR_REFERENCE_COUNT_INTEL: cl_accelerator_info_intel = 0x4091;
pub const CL_ACCELERATOR_CONTEXT_INTEL: cl_accelerator_info_intel = 0x4092;
pub const CL_ACCELERATOR_TYPE_INTEL: cl_accelerator_info_intel = 0x4093;

// cl_motion_detect_desc_intel flags
pub type cl_motion_detect_desc_intel = cl_uint;
pub const CL_ME_MB_TYPE_16x16_INTEL: cl_motion_detect_desc_intel = 0x0;
pub const CL_ME_MB_TYPE_8x8_INTEL: cl_motion_detect_desc_intel = 0x1;
pub const CL_ME_MB_TYPE_4x4_INTEL: cl_motion_detect_desc_intel = 0x2;

pub const CL_ME_SUBPIXEL_MODE_INTEGER_INTEL: cl_motion_detect_desc_intel = 0x0;
pub const CL_ME_SUBPIXEL_MODE_HPEL_INTEL: cl_motion_detect_desc_intel = 0x1;
pub const CL_ME_SUBPIXEL_MODE_QPEL_INTEL: cl_motion_detect_desc_intel = 0x2;

pub const CL_ME_SAD_ADJUST_MODE_NONE_INTEL: cl_motion_detect_desc_intel = 0x0;
pub const CL_ME_SAD_ADJUST_MODE_HAAR_INTEL: cl_motion_detect_desc_intel = 0x1;

pub const CL_ME_SEARCH_PATH_RADIUS_2_2_INTEL: cl_motion_detect_desc_intel = 0x0;
pub const CL_ME_SEARCH_PATH_RADIUS_4_4_INTEL: cl_motion_detect_desc_intel = 0x1;
pub const CL_ME_SEARCH_PATH_RADIUS_16_12_INTEL: cl_motion_detect_desc_intel = 0x5;

pub const CL_ME_SKIP_BLOCK_TYPE_16x16_INTEL: cl_motion_detect_desc_intel = 0x0;
pub const CL_ME_CHROMA_INTRA_PREDICT_ENABLED_INTEL: cl_motion_detect_desc_intel = 0x1;
pub const CL_ME_LUMA_INTRA_PREDICT_ENABLED_INTEL: cl_motion_detect_desc_intel = 0x2;
pub const CL_ME_SKIP_BLOCK_TYPE_8x8_INTEL: cl_motion_detect_desc_intel = 0x4;

pub const CL_ME_FORWARD_INPUT_MODE_INTEL: cl_motion_detect_desc_intel = 0x1;
pub const CL_ME_BACKWARD_INPUT_MODE_INTEL: cl_motion_detect_desc_intel = 0x2;
pub const CL_ME_BIDIRECTION_INPUT_MODE_INTEL: cl_motion_detect_desc_intel = 0x3;

pub const CL_ME_BIDIR_WEIGHT_QUARTER_INTEL: cl_motion_detect_desc_intel = 16;
pub const CL_ME_BIDIR_WEIGHT_THIRD_INTEL: cl_motion_detect_desc_intel = 21;
pub const CL_ME_BIDIR_WEIGHT_HALF_INTEL: cl_motion_detect_desc_intel = 32;
pub const CL_ME_BIDIR_WEIGHT_TWO_THIRD_INTEL: cl_motion_detect_desc_intel = 43;
pub const CL_ME_BIDIR_WEIGHT_THREE_QUARTER_INTEL: cl_motion_detect_desc_intel = 48;

pub const CL_ME_COST_PENALTY_NONE_INTEL: cl_motion_detect_desc_intel = 0x0;
pub const CL_ME_COST_PENALTY_LOW_INTEL: cl_motion_detect_desc_intel = 0x1;
pub const CL_ME_COST_PENALTY_NORMAL_INTEL: cl_motion_detect_desc_intel = 0x2;
pub const CL_ME_COST_PENALTY_HIGH_INTEL: cl_motion_detect_desc_intel = 0x3;

pub const CL_ME_COST_PRECISION_QPEL_INTEL: cl_motion_detect_desc_intel = 0x0;
pub const CL_ME_COST_PRECISION_HPEL_INTEL: cl_motion_detect_desc_intel = 0x1;
pub const CL_ME_COST_PRECISION_PEL_INTEL: cl_motion_detect_desc_intel = 0x2;
pub const CL_ME_COST_PRECISION_DPEL_INTEL: cl_motion_detect_desc_intel = 0x3;

pub const CL_ME_LUMA_PREDICTOR_MODE_VERTICAL_INTEL: cl_motion_detect_desc_intel = 0x0;
pub const CL_ME_LUMA_PREDICTOR_MODE_HORIZONTAL_INTEL: cl_motion_detect_desc_intel = 0x1;
pub const CL_ME_LUMA_PREDICTOR_MODE_DC_INTEL: cl_motion_detect_desc_intel = 0x2;
pub const CL_ME_LUMA_PREDICTOR_MODE_DIAGONAL_DOWN_LEFT_INTEL: cl_motion_detect_desc_intel = 0x3;

pub const CL_ME_LUMA_PREDICTOR_MODE_DIAGONAL_DOWN_RIGHT_INTEL: cl_motion_detect_desc_intel = 0x4;
pub const CL_ME_LUMA_PREDICTOR_MODE_PLANE_INTEL: cl_motion_detect_desc_intel = 0x4;
pub const CL_ME_LUMA_PREDICTOR_MODE_VERTICAL_RIGHT_INTEL: cl_motion_detect_desc_intel = 0x5;
pub const CL_ME_LUMA_PREDICTOR_MODE_HORIZONTAL_DOWN_INTEL: cl_motion_detect_desc_intel = 0x6;
pub const CL_ME_LUMA_PREDICTOR_MODE_VERTICAL_LEFT_INTEL: cl_motion_detect_desc_intel = 0x7;
pub const CL_ME_LUMA_PREDICTOR_MODE_HORIZONTAL_UP_INTEL: cl_motion_detect_desc_intel = 0x8;

pub const CL_ME_CHROMA_PREDICTOR_MODE_DC_INTEL: cl_motion_detect_desc_intel = 0x0;
pub const CL_ME_CHROMA_PREDICTOR_MODE_HORIZONTAL_INTEL: cl_motion_detect_desc_intel = 0x1;
pub const CL_ME_CHROMA_PREDICTOR_MODE_VERTICAL_INTEL: cl_motion_detect_desc_intel = 0x2;
pub const CL_ME_CHROMA_PREDICTOR_MODE_PLANE_INTEL: cl_motion_detect_desc_intel = 0x3;

pub const CL_DEVICE_ME_VERSION_INTEL: cl_device_info = 0x407E;

pub const CL_ME_VERSION_LEGACY_INTEL: cl_uint = 0x0;
pub const CL_ME_VERSION_ADVANCED_VER_1_INTEL: cl_uint = 0x1;
pub const CL_ME_VERSION_ADVANCED_VER_2_INTEL: cl_uint = 0x2;

// cl_intel_simultaneous_sharing extension
pub const CL_DEVICE_SIMULTANEOUS_INTEROPS_INTEL: cl_uint = 0x4104;
pub const CL_DEVICE_NUM_SIMULTANEOUS_INTEROPS_INTEL: cl_uint = 0x4105;

// cl_intel_egl_image_yuv extension
pub const CL_EGL_YUV_PLANE_INTEL: cl_uint = 0x4107;

// cl_intel_packed_yuv extension
pub const CL_YUYV_INTEL: cl_uint = 0x4076;
pub const CL_UYVY_INTEL: cl_uint = 0x4077;
pub const CL_YVYU_INTEL: cl_uint = 0x4078;
pub const CL_VYUY_INTEL: cl_uint = 0x4079;

// cl_intel_required_subgroup_size extension
pub const CL_DEVICE_SUB_GROUP_SIZES_INTEL: cl_uint = 0x4108;
pub const CL_KERNEL_SPILL_MEM_SIZE_INTEL: cl_uint = 0x4109;
pub const CL_KERNEL_COMPILE_SUB_GROUP_SIZE_INTEL: cl_uint = 0x410A;

// cl_intel_driver_diagnostics extension
pub const CL_CONTEXT_SHOW_DIAGNOSTICS_INTEL: cl_uint = 0x4106;

pub type cl_diagnostics_verbose_level = cl_uint;
pub const CL_CONTEXT_DIAGNOSTICS_LEVEL_ALL_INTEL: cl_uint = 0xff;
pub const CL_CONTEXT_DIAGNOSTICS_LEVEL_GOOD_INTEL: cl_uint = 1;
pub const CL_CONTEXT_DIAGNOSTICS_LEVEL_BAD_INTEL: cl_uint = 1 << 1;
pub const CL_CONTEXT_DIAGNOSTICS_LEVEL_NEUTRAL_INTEL: cl_uint = 1 << 2;

// cl_intel_planar_yuv extension
pub const CL_NV12_INTEL: cl_uint = 0x410E;

pub const CL_MEM_NO_ACCESS_INTEL: cl_uint = 1 << 24;
pub const CL_MEM_ACCESS_FLAGS_UNRESTRICTED_INTEL: cl_uint = 1 << 25;

pub const CL_DEVICE_PLANAR_YUV_MAX_WIDTH_INTEL: cl_uint = 0x417E;
pub const CL_DEVICE_PLANAR_YUV_MAX_HEIGHT_INTEL: cl_uint = 0x417F;

// cl_intel_device_side_avc_motion_estimation extension
pub type cl_intel_avc_motion_estimation = cl_uint;

pub const CL_DEVICE_AVC_ME_VERSION_INTEL: cl_uint = 0x410B;
pub const CL_DEVICE_AVC_ME_SUPPORTS_TEXTURE_SAMPLER_USE_INTEL: cl_uint = 0x410C;
pub const CL_DEVICE_AVC_ME_SUPPORTS_PREEMPTION_INTEL: cl_uint = 0x410D;

pub const CL_AVC_ME_VERSION_0_INTEL: cl_intel_avc_motion_estimation = 0x0;
pub const CL_AVC_ME_VERSION_1_INTEL: cl_intel_avc_motion_estimation = 0x1;

pub const CL_AVC_ME_MAJOR_16x16_INTEL: cl_intel_avc_motion_estimation = 0x0;
pub const CL_AVC_ME_MAJOR_16x8_INTEL: cl_intel_avc_motion_estimation = 0x1;
pub const CL_AVC_ME_MAJOR_8x16_INTEL: cl_intel_avc_motion_estimation = 0x2;
pub const CL_AVC_ME_MAJOR_8x8_INTEL: cl_intel_avc_motion_estimation = 0x3;

pub const CL_AVC_ME_MINOR_8x8_INTEL: cl_intel_avc_motion_estimation = 0x0;
pub const CL_AVC_ME_MINOR_8x4_INTEL: cl_intel_avc_motion_estimation = 0x1;
pub const CL_AVC_ME_MINOR_4x8_INTEL: cl_intel_avc_motion_estimation = 0x2;
pub const CL_AVC_ME_MINOR_4x4_INTEL: cl_intel_avc_motion_estimation = 0x3;

pub const CL_AVC_ME_MAJOR_FORWARD_INTEL: cl_intel_avc_motion_estimation = 0x0;
pub const CL_AVC_ME_MAJOR_BACKWARD_INTEL: cl_intel_avc_motion_estimation = 0x1;
pub const CL_AVC_ME_MAJOR_BIDIRECTIONAL_INTEL: cl_intel_avc_motion_estimation = 0x2;

pub const CL_AVC_ME_PARTITION_MASK_ALL_INTEL: cl_intel_avc_motion_estimation = 0x0;
pub const CL_AVC_ME_PARTITION_MASK_16x16_INTEL: cl_intel_avc_motion_estimation = 0x7E;
pub const CL_AVC_ME_PARTITION_MASK_16x8_INTEL: cl_intel_avc_motion_estimation = 0x7D;
pub const CL_AVC_ME_PARTITION_MASK_8x16_INTEL: cl_intel_avc_motion_estimation = 0x7B;
pub const CL_AVC_ME_PARTITION_MASK_8x8_INTEL: cl_intel_avc_motion_estimation = 0x77;
pub const CL_AVC_ME_PARTITION_MASK_8x4_INTEL: cl_intel_avc_motion_estimation = 0x6F;
pub const CL_AVC_ME_PARTITION_MASK_4x8_INTEL: cl_intel_avc_motion_estimation = 0x5F;
pub const CL_AVC_ME_PARTITION_MASK_4x4_INTEL: cl_intel_avc_motion_estimation = 0x3F;

pub const CL_AVC_ME_SEARCH_WINDOW_EXHAUSTIVE_INTEL: cl_intel_avc_motion_estimation = 0x0;
pub const CL_AVC_ME_SEARCH_WINDOW_SMALL_INTEL: cl_intel_avc_motion_estimation = 0x1;
pub const CL_AVC_ME_SEARCH_WINDOW_TINY_INTEL: cl_intel_avc_motion_estimation = 0x2;
pub const CL_AVC_ME_SEARCH_WINDOW_EXTRA_TINY_INTEL: cl_intel_avc_motion_estimation = 0x3;
pub const CL_AVC_ME_SEARCH_WINDOW_DIAMOND_INTEL: cl_intel_avc_motion_estimation = 0x4;
pub const CL_AVC_ME_SEARCH_WINDOW_LARGE_DIAMOND_INTEL: cl_intel_avc_motion_estimation = 0x5;
pub const CL_AVC_ME_SEARCH_WINDOW_RESERVED0_INTEL: cl_intel_avc_motion_estimation = 0x6;
pub const CL_AVC_ME_SEARCH_WINDOW_RESERVED1_INTEL: cl_intel_avc_motion_estimation = 0x7;
pub const CL_AVC_ME_SEARCH_WINDOW_CUSTOM_INTEL: cl_intel_avc_motion_estimation = 0x8;
pub const CL_AVC_ME_SEARCH_WINDOW_16x12_RADIUS_INTEL: cl_intel_avc_motion_estimation = 0x9;
pub const CL_AVC_ME_SEARCH_WINDOW_4x4_RADIUS_INTEL: cl_intel_avc_motion_estimation = 0x2;
pub const CL_AVC_ME_SEARCH_WINDOW_2x2_RADIUS_INTEL: cl_intel_avc_motion_estimation = 0xa;

pub const CL_AVC_ME_SAD_ADJUST_MODE_NONE_INTEL: cl_intel_avc_motion_estimation = 0x0;
pub const CL_AVC_ME_SAD_ADJUST_MODE_HAAR_INTEL: cl_intel_avc_motion_estimation = 0x2;

pub const CL_AVC_ME_SUBPIXEL_MODE_INTEGER_INTEL: cl_intel_avc_motion_estimation = 0x0;
pub const CL_AVC_ME_SUBPIXEL_MODE_HPEL_INTEL: cl_intel_avc_motion_estimation = 0x1;
pub const CL_AVC_ME_SUBPIXEL_MODE_QPEL_INTEL: cl_intel_avc_motion_estimation = 0x3;

pub const CL_AVC_ME_COST_PRECISION_QPEL_INTEL: cl_intel_avc_motion_estimation = 0x0;
pub const CL_AVC_ME_COST_PRECISION_HPEL_INTEL: cl_intel_avc_motion_estimation = 0x1;
pub const CL_AVC_ME_COST_PRECISION_PEL_INTEL: cl_intel_avc_motion_estimation = 0x2;
pub const CL_AVC_ME_COST_PRECISION_DPEL_INTEL: cl_intel_avc_motion_estimation = 0x3;

pub const CL_AVC_ME_BIDIR_WEIGHT_QUARTER_INTEL: cl_intel_avc_motion_estimation = 0x10;
pub const CL_AVC_ME_BIDIR_WEIGHT_THIRD_INTEL: cl_intel_avc_motion_estimation = 0x15;
pub const CL_AVC_ME_BIDIR_WEIGHT_HALF_INTEL: cl_intel_avc_motion_estimation = 0x20;
pub const CL_AVC_ME_BIDIR_WEIGHT_TWO_THIRD_INTEL: cl_intel_avc_motion_estimation = 0x2B;
pub const CL_AVC_ME_BIDIR_WEIGHT_THREE_QUARTER_INTEL: cl_intel_avc_motion_estimation = 0x30;

pub const CL_AVC_ME_BORDER_REACHED_LEFT_INTEL: cl_intel_avc_motion_estimation = 0x0;
pub const CL_AVC_ME_BORDER_REACHED_RIGHT_INTEL: cl_intel_avc_motion_estimation = 0x2;
pub const CL_AVC_ME_BORDER_REACHED_TOP_INTEL: cl_intel_avc_motion_estimation = 0x4;
pub const CL_AVC_ME_BORDER_REACHED_BOTTOM_INTEL: cl_intel_avc_motion_estimation = 0x8;

pub const CL_AVC_ME_SKIP_BLOCK_PARTITION_16x16_INTEL: cl_intel_avc_motion_estimation = 0x0;
pub const CL_AVC_ME_SKIP_BLOCK_PARTITION_8x8_INTEL: cl_intel_avc_motion_estimation = 0x4000;

pub const CL_AVC_ME_SKIP_BLOCK_16x16_FORWARD_ENABLE_INTEL: cl_intel_avc_motion_estimation =
    0x1 << 24;
pub const CL_AVC_ME_SKIP_BLOCK_16x16_BACKWARD_ENABLE_INTEL: cl_intel_avc_motion_estimation =
    0x2 << 24;
pub const CL_AVC_ME_SKIP_BLOCK_16x16_DUAL_ENABLE_INTEL: cl_intel_avc_motion_estimation = 0x3 << 24;
pub const CL_AVC_ME_SKIP_BLOCK_8x8_FORWARD_ENABLE_INTEL: cl_intel_avc_motion_estimation =
    0x55 << 24;
pub const CL_AVC_ME_SKIP_BLOCK_8x8_BACKWARD_ENABLE_INTEL: cl_intel_avc_motion_estimation =
    0xAA << 24;
pub const CL_AVC_ME_SKIP_BLOCK_8x8_DUAL_ENABLE_INTEL: cl_intel_avc_motion_estimation = 0xFF << 24;
pub const CL_AVC_ME_SKIP_BLOCK_8x8_0_FORWARD_ENABLE_INTEL: cl_intel_avc_motion_estimation =
    0x1 << 24;
pub const CL_AVC_ME_SKIP_BLOCK_8x8_0_BACKWARD_ENABLE_INTEL: cl_intel_avc_motion_estimation =
    0x2 << 24;
pub const CL_AVC_ME_SKIP_BLOCK_8x8_1_FORWARD_ENABLE_INTEL: cl_intel_avc_motion_estimation =
    0x1 << 26;
pub const CL_AVC_ME_SKIP_BLOCK_8x8_1_BACKWARD_ENABLE_INTEL: cl_intel_avc_motion_estimation =
    0x2 << 26;
pub const CL_AVC_ME_SKIP_BLOCK_8x8_2_FORWARD_ENABLE_INTEL: cl_intel_avc_motion_estimation =
    0x1 << 28;
pub const CL_AVC_ME_SKIP_BLOCK_8x8_2_BACKWARD_ENABLE_INTEL: cl_intel_avc_motion_estimation =
    0x2 << 28;
pub const CL_AVC_ME_SKIP_BLOCK_8x8_3_FORWARD_ENABLE_INTEL: cl_intel_avc_motion_estimation =
    0x1 << 30;
pub const CL_AVC_ME_SKIP_BLOCK_8x8_3_BACKWARD_ENABLE_INTEL: cl_intel_avc_motion_estimation =
    0x2 << 30;

pub const CL_AVC_ME_BLOCK_BASED_SKIP_4x4_INTEL: cl_intel_avc_motion_estimation = 0x00;
pub const CL_AVC_ME_BLOCK_BASED_SKIP_8x8_INTEL: cl_intel_avc_motion_estimation = 0x80;

pub const CL_AVC_ME_INTRA_16x16_INTEL: cl_intel_avc_motion_estimation = 0x0;
pub const CL_AVC_ME_INTRA_8x8_INTEL: cl_intel_avc_motion_estimation = 0x1;
pub const CL_AVC_ME_INTRA_4x4_INTEL: cl_intel_avc_motion_estimation = 0x2;

pub const CL_AVC_ME_INTRA_LUMA_PARTITION_MASK_16x16_INTEL: cl_intel_avc_motion_estimation = 0x6;
pub const CL_AVC_ME_INTRA_LUMA_PARTITION_MASK_8x8_INTEL: cl_intel_avc_motion_estimation = 0x5;
pub const CL_AVC_ME_INTRA_LUMA_PARTITION_MASK_4x4_INTEL: cl_intel_avc_motion_estimation = 0x3;

pub const CL_AVC_ME_INTRA_NEIGHBOR_LEFT_MASK_ENABLE_INTEL: cl_intel_avc_motion_estimation = 0x60;
pub const CL_AVC_ME_INTRA_NEIGHBOR_UPPER_MASK_ENABLE_INTEL: cl_intel_avc_motion_estimation = 0x10;
pub const CL_AVC_ME_INTRA_NEIGHBOR_UPPER_RIGHT_MASK_ENABLE_INTEL: cl_intel_avc_motion_estimation =
    0x8;
pub const CL_AVC_ME_INTRA_NEIGHBOR_UPPER_LEFT_MASK_ENABLE_INTEL: cl_intel_avc_motion_estimation =
    0x4;

pub const CL_AVC_ME_LUMA_PREDICTOR_MODE_VERTICAL_INTEL: cl_intel_avc_motion_estimation = 0x0;
pub const CL_AVC_ME_LUMA_PREDICTOR_MODE_HORIZONTAL_INTEL: cl_intel_avc_motion_estimation = 0x1;
pub const CL_AVC_ME_LUMA_PREDICTOR_MODE_DC_INTEL: cl_intel_avc_motion_estimation = 0x2;
pub const CL_AVC_ME_LUMA_PREDICTOR_MODE_DIAGONAL_DOWN_LEFT_INTEL: cl_intel_avc_motion_estimation =
    0x3;
pub const CL_AVC_ME_LUMA_PREDICTOR_MODE_DIAGONAL_DOWN_RIGHT_INTEL: cl_intel_avc_motion_estimation =
    0x4;
pub const CL_AVC_ME_LUMA_PREDICTOR_MODE_PLANE_INTEL: cl_intel_avc_motion_estimation = 0x4;
pub const CL_AVC_ME_LUMA_PREDICTOR_MODE_VERTICAL_RIGHT_INTEL: cl_intel_avc_motion_estimation = 0x5;
pub const CL_AVC_ME_LUMA_PREDICTOR_MODE_HORIZONTAL_DOWN_INTEL: cl_intel_avc_motion_estimation = 0x6;
pub const CL_AVC_ME_LUMA_PREDICTOR_MODE_VERTICAL_LEFT_INTEL: cl_intel_avc_motion_estimation = 0x7;
pub const CL_AVC_ME_LUMA_PREDICTOR_MODE_HORIZONTAL_UP_INTEL: cl_intel_avc_motion_estimation = 0x8;
pub const CL_AVC_ME_CHROMA_PREDICTOR_MODE_DC_INTEL: cl_intel_avc_motion_estimation = 0x0;
pub const CL_AVC_ME_CHROMA_PREDICTOR_MODE_HORIZONTAL_INTEL: cl_intel_avc_motion_estimation = 0x1;
pub const CL_AVC_ME_CHROMA_PREDICTOR_MODE_VERTICAL_INTEL: cl_intel_avc_motion_estimation = 0x2;
pub const CL_AVC_ME_CHROMA_PREDICTOR_MODE_PLANE_INTEL: cl_intel_avc_motion_estimation = 0x3;

pub const CL_AVC_ME_FRAME_FORWARD_INTEL: cl_intel_avc_motion_estimation = 0x1;
pub const CL_AVC_ME_FRAME_BACKWARD_INTEL: cl_intel_avc_motion_estimation = 0x2;
pub const CL_AVC_ME_FRAME_DUAL_INTEL: cl_intel_avc_motion_estimation = 0x3;

pub const CL_AVC_ME_SLICE_TYPE_PRED_INTEL: cl_intel_avc_motion_estimation = 0x0;
pub const CL_AVC_ME_SLICE_TYPE_BPRED_INTEL: cl_intel_avc_motion_estimation = 0x1;
pub const CL_AVC_ME_SLICE_TYPE_INTRA_INTEL: cl_intel_avc_motion_estimation = 0x2;

pub const CL_AVC_ME_INTERLACED_SCAN_TOP_FIELD_INTEL: cl_intel_avc_motion_estimation = 0x0;
pub const CL_AVC_ME_INTERLACED_SCAN_BOTTOM_FIELD_INTEL: cl_intel_avc_motion_estimation = 0x1;

// cl_intel_unified_shared_memory extension
pub const CL_DEVICE_HOST_MEM_CAPABILITIES_INTEL: cl_device_info = 0x4190;
pub const CL_DEVICE_DEVICE_MEM_CAPABILITIES_INTEL: cl_device_info = 0x4191;
pub const CL_DEVICE_SINGLE_DEVICE_SHARED_MEM_CAPABILITIES_INTEL: cl_device_info = 0x4192;
pub const CL_DEVICE_CROSS_DEVICE_SHARED_MEM_CAPABILITIES_INTEL: cl_device_info = 0x4193;
pub const CL_DEVICE_SHARED_SYSTEM_MEM_CAPABILITIES_INTEL: cl_device_info = 0x4194;

pub type cl_device_unified_shared_memory_capabilities_intel = cl_bitfield;
pub const CL_UNIFIED_SHARED_MEMORY_ACCESS_INTEL:
    cl_device_unified_shared_memory_capabilities_intel = 1 << 0;
pub const CL_UNIFIED_SHARED_MEMORY_ATOMIC_ACCESS_INTEL:
    cl_device_unified_shared_memory_capabilities_intel = 1 << 1;
pub const CL_UNIFIED_SHARED_MEMORY_CONCURRENT_ACCESS_INTEL:
    cl_device_unified_shared_memory_capabilities_intel = 1 << 2;
pub const CL_UNIFIED_SHARED_MEMORY_CONCURRENT_ATOMIC_ACCESS_INTEL:
    cl_device_unified_shared_memory_capabilities_intel = 1 << 3;

pub type cl_mem_properties_intel = cl_properties;
pub const CL_MEM_ALLOC_FLAGS_INTEL: cl_mem_properties_intel = 0x4195;

pub type cl_mem_alloc_flags_intel = cl_bitfield;
pub const CL_MEM_ALLOC_WRITE_COMBINED_INTEL: cl_mem_alloc_flags_intel = 1 << 0;

pub type cl_mem_info_intel = cl_uint;
pub const CL_MEM_ALLOC_TYPE_INTEL: cl_mem_info_intel = 0x419A;
pub const CL_MEM_ALLOC_BASE_PTR_INTEL: cl_mem_info_intel = 0x419B;
pub const CL_MEM_ALLOC_SIZE_INTEL: cl_mem_info_intel = 0x419C;
pub const CL_MEM_ALLOC_DEVICE_INTEL: cl_mem_info_intel = 0x419D;

pub type cl_unified_shared_memory_type_intel = cl_uint;
pub const CL_MEM_TYPE_UNKNOWN_INTEL: cl_unified_shared_memory_type_intel = 0x4196;
pub const CL_MEM_TYPE_HOST_INTEL: cl_unified_shared_memory_type_intel = 0x4197;
pub const CL_MEM_TYPE_DEVICE_INTEL: cl_unified_shared_memory_type_intel = 0x4198;
pub const CL_MEM_TYPE_SHARED_INTEL: cl_unified_shared_memory_type_intel = 0x4199;

pub type cl_mem_advice_intel = cl_uint;

pub const CL_KERNEL_EXEC_INFO_INDIRECT_HOST_ACCESS_INTEL: cl_kernel_exec_info = 0x4200;
pub const CL_KERNEL_EXEC_INFO_INDIRECT_DEVICE_ACCESS_INTEL: cl_kernel_exec_info = 0x4201;
pub const CL_KERNEL_EXEC_INFO_INDIRECT_SHARED_ACCESS_INTEL: cl_kernel_exec_info = 0x4202;
pub const CL_KERNEL_EXEC_INFO_USM_PTRS_INTEL: cl_kernel_exec_info = 0x4203;

pub const CL_COMMAND_MEMFILL_INTEL: cl_command_type = 0x4204;
pub const CL_COMMAND_MEMCPY_INTEL: cl_command_type = 0x4205;
pub const CL_COMMAND_MIGRATEMEM_INTEL: cl_command_type = 0x4206;
pub const CL_COMMAND_MEMADVISE_INTEL: cl_command_type = 0x4207;

// cl_intel_mem_channel_property extension

pub const CL_MEM_CHANNEL_INTEL: cl_uint = 0x4213;

// cl_intel_mem_force_host_memory

pub const CL_MEM_FORCE_HOST_MEMORY_INTEL: cl_mem_flags = 1 << 20;

// cl_intel_command_queue_families

pub type cl_command_queue_capabilities_intel = cl_bitfield;

pub const CL_QUEUE_FAMILY_MAX_NAME_SIZE_INTEL: usize = 64;

#[derive(Debug)]
#[repr(C)]
pub struct cl_queue_family_properties_intel {
    pub properties: cl_command_queue_properties,
    pub capabilities: cl_command_queue_capabilities_intel,
    pub count: cl_uint,
    pub name: [cl_uchar; CL_QUEUE_FAMILY_MAX_NAME_SIZE_INTEL],
}

pub const CL_DEVICE_QUEUE_FAMILY_PROPERTIES_INTEL: cl_device_info = 0x418B;

pub const CL_QUEUE_FAMILY_INTEL: cl_queue_properties = 0x418C;
pub const CL_QUEUE_INDEX_INTEL: cl_queue_properties = 0x418D;

pub const CL_QUEUE_DEFAULT_CAPABILITIES_INTEL: cl_command_queue_capabilities_intel = 0;
pub const CL_QUEUE_CAPABILITY_CREATE_SINGLE_QUEUE_EVENTS_INTEL:
    cl_command_queue_capabilities_intel = 1 << 0;
pub const CL_QUEUE_CAPABILITY_CREATE_CROSS_QUEUE_EVENTS_INTEL: cl_command_queue_capabilities_intel =
    1 << 1;
pub const CL_QUEUE_CAPABILITY_SINGLE_QUEUE_EVENT_WAIT_LIST_INTEL:
    cl_command_queue_capabilities_intel = 1 << 2;
pub const CL_QUEUE_CAPABILITY_CROSS_QUEUE_EVENT_WAIT_LIST_INTEL:
    cl_command_queue_capabilities_intel = 1 << 3;
pub const CL_QUEUE_CAPABILITY_TRANSFER_BUFFER_INTEL: cl_command_queue_capabilities_intel = 1 << 8;
pub const CL_QUEUE_CAPABILITY_TRANSFER_BUFFER_RECT_INTEL: cl_command_queue_capabilities_intel =
    1 << 9;
pub const CL_QUEUE_CAPABILITY_MAP_BUFFER_INTEL: cl_command_queue_capabilities_intel = 1 << 10;
pub const CL_QUEUE_CAPABILITY_FILL_BUFFER_INTEL: cl_command_queue_capabilities_intel = 1 << 11;
pub const CL_QUEUE_CAPABILITY_TRANSFER_IMAGE_INTEL: cl_command_queue_capabilities_intel = 1 << 12;
pub const CL_QUEUE_CAPABILITY_MAP_IMAGE_INTEL: cl_command_queue_capabilities_intel = 1 << 13;
pub const CL_QUEUE_CAPABILITY_FILL_IMAGE_INTEL: cl_command_queue_capabilities_intel = 1 << 14;
pub const CL_QUEUE_CAPABILITY_TRANSFER_BUFFER_IMAGE_INTEL: cl_command_queue_capabilities_intel =
    1 << 15;
pub const CL_QUEUE_CAPABILITY_TRANSFER_IMAGE_BUFFER_INTEL: cl_command_queue_capabilities_intel =
    1 << 16;
pub const CL_QUEUE_CAPABILITY_MARKER_INTEL: cl_command_queue_capabilities_intel = 1 << 24;
pub const CL_QUEUE_CAPABILITY_BARRIER_INTEL: cl_command_queue_capabilities_intel = 1 << 25;
pub const CL_QUEUE_CAPABILITY_KERNEL_INTEL: cl_command_queue_capabilities_intel = 1 << 26;

#[cfg_attr(not(target_os = "macos"), link(name = "OpenCL"))]
#[cfg_attr(target_os = "macos", link(name = "OpenCL", kind = "framework"))]
extern "system" {
    pub fn clSetMemObjectDestructorAPPLE(
        memobj: cl_mem,
        pfn_notify: extern "C" fn(cl_mem, *const c_void),
        user_data: *mut c_void,
    ) -> cl_int;

    pub fn clLogMessagesToSystemLogAPPLE(
        errstr: *const cl_uchar,
        private_info: *const c_void,
        cb: size_t,
        user_data: *mut c_void,
    );

    pub fn clLogMessagesToStdoutAPPLE(
        errstr: *const cl_uchar,
        private_info: *const c_void,
        cb: size_t,
        user_data: *mut c_void,
    );

    pub fn clLogMessagesToStderrAPPLE(
        errstr: *const cl_uchar,
        private_info: *const c_void,
        cb: size_t,
        user_data: *mut c_void,
    );

    pub fn clIcdGetPlatformIDsKHR(
        num_entries: cl_uint,
        platforms: *mut cl_platform_id,
        num_platforms: *mut cl_uint,
    ) -> cl_int;

    pub fn clCreateProgramWithILKHR(
        context: cl_context,
        il: *const c_void,
        length: size_t,
        errcode_ret: *mut cl_int,
    ) -> cl_program;

    pub fn clTerminateContextKHR(context: cl_context) -> cl_int;

    pub fn clCreateCommandQueueWithPropertiesKHR(
        context: cl_context,
        device: cl_device_id,
        properties: *const cl_queue_properties_khr,
        errcode_ret: *mut cl_int,
    ) -> cl_command_queue;

    pub fn clReleaseDeviceEXT(device: cl_device_id) -> cl_int;

    pub fn clRetainDeviceEXT(device: cl_device_id) -> cl_int;

    pub fn clCreateSubDevicesEXT(
        in_device: cl_device_id,
        properties: *const cl_device_partition_property_ext,
        num_entries: cl_uint,
        out_devices: *mut cl_device_id,
        num_devices: *mut cl_uint,
    ) -> cl_int;

    pub fn clEnqueueMigrateMemObjectEXT(
        command_queue: cl_command_queue,
        num_mem_objects: cl_uint,
        mem_objects: *const cl_mem,
        flags: cl_mem_migration_flags_ext,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clGetDeviceImageInfoQCOM(
        device: cl_device_id,
        image_width: size_t,
        image_height: size_t,
        image_format: *const cl_image_format,
        param_name: cl_image_pitch_info_qcom,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;

    pub fn clEnqueueAcquireGrallocObjectsIMG(
        command_queue: cl_command_queue,
        num_objects: cl_uint,
        mem_objects: *const cl_mem,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueReleaseGrallocObjectsIMG(
        command_queue: cl_command_queue,
        num_objects: cl_uint,
        mem_objects: *const cl_mem,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueGenerateMipmapIMG(
        command_queue: cl_command_queue,
        src_image: cl_mem,
        dst_image: cl_mem,
        mipmap_filter_mode: cl_mipmap_filter_mode_img,
        array_region: *const size_t,
        mip_region: *const size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clGetKernelSubGroupInfoKHR(
        in_kernel: cl_kernel,
        device: cl_device_id,
        param_name: cl_kernel_sub_group_info,
        input_value_size: size_t,
        input_value: *const c_void,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;

    pub fn clGetKernelSuggestedLocalWorkSizeKHR(
        command_queue: cl_command_queue,
        kernel: cl_kernel,
        work_dim: cl_uint,
        global_work_offset: *const size_t,
        global_work_size: *const size_t,
        suggested_local_work_size: *mut size_t,
    ) -> cl_int;

    pub fn clEnqueueAcquireExternalMemObjectsKHR(
        command_queue: cl_command_queue,
        num_mem_objects: cl_uint,
        mem_objects: *const cl_mem,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueReleaseExternalMemObjectsKHR(
        command_queue: cl_command_queue,
        num_mem_objects: cl_uint,
        mem_objects: *const cl_mem,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clGetSemaphoreHandleForTypeKHR(
        sema_object: cl_semaphore_khr,
        device: cl_device_id,
        handle_type: cl_external_semaphore_handle_type_khr,
        handle_size: size_t,
        handle_ptr: *mut c_void,
        handle_size_ret: *mut size_t,
    ) -> cl_int;

    pub fn clCreateSemaphoreWithPropertiesKHR(
        context: cl_context,
        sema_props: *const cl_semaphore_properties_khr,
        errcode_ret: *mut cl_int,
    ) -> cl_semaphore_khr;

    pub fn clEnqueueWaitSemaphoresKHR(
        command_queue: cl_command_queue,
        num_sema_objects: cl_uint,
        sema_objects: *const cl_semaphore_khr,
        sema_payload_list: *const cl_semaphore_payload_khr,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueSignalSemaphoresKHR(
        command_queue: cl_command_queue,
        num_sema_objects: cl_uint,
        sema_objects: *const cl_semaphore_khr,
        sema_payload_list: *const cl_semaphore_payload_khr,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clGetSemaphoreInfoKHR(
        sema_object: cl_semaphore_khr,
        param_name: cl_semaphore_info_khr,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;

    pub fn clReleaseSemaphoreKHR(sema_object: cl_semaphore_khr) -> cl_int;

    pub fn clRetainSemaphoreKHR(sema_object: cl_semaphore_khr) -> cl_int;

    pub fn clImportMemoryARM(
        context: cl_context,
        flags: cl_mem_flags,
        properties: *const cl_import_properties_arm,
        memory: *mut c_void,
        size: size_t,
        errcode_ret: *mut cl_int,
    ) -> cl_mem;

    pub fn clSVMAllocARM(
        context: cl_context,
        flags: cl_svm_mem_flags_arm,
        size: size_t,
        alignment: cl_uint,
    ) -> *mut c_void;

    pub fn clSVMFreeARM(context: cl_context, svm_pointer: *mut c_void);

    pub fn clEnqueueSVMFreeARM(
        command_queue: cl_command_queue,
        num_svm_pointers: cl_uint,
        svm_pointers: *const *const c_void,
        pfn_free_func: Option<
            extern "C" fn(
                queue: cl_command_queue,
                num_svm_pointers: cl_uint,
                svm_pointers: *const *const c_void,
                user_data: *mut c_void,
            ),
        >,
        user_data: *mut c_void,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueSVMMemcpyARM(
        command_queue: cl_command_queue,
        blocking_copy: cl_bool,
        dst_ptr: *mut c_void,
        src_ptr: *const c_void,
        size: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueSVMMemFillARM(
        command_queue: cl_command_queue,
        svm_ptr: *mut c_void,
        pattern: *const c_void,
        pattern_size: size_t,
        size: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueSVMMapARM(
        command_queue: cl_command_queue,
        blocking_map: cl_bool,
        flags: cl_map_flags,
        svm_ptr: *mut c_void,
        size: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueSVMUnmapARM(
        command_queue: cl_command_queue,
        svm_ptr: *mut c_void,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clSetKernelArgSVMPointerARM(
        kernel: cl_kernel,
        arg_index: cl_uint,
        arg_value: *const c_void,
    ) -> cl_int;

    pub fn clSetKernelExecInfoARM(
        kernel: cl_kernel,
        param_name: cl_kernel_exec_info_arm,
        param_value_size: size_t,
        param_value: *const c_void,
    ) -> cl_int;

    pub fn clCreateAcceleratorINTEL(
        context: cl_context,
        accelerator_type: cl_accelerator_type_intel,
        descriptor_size: size_t,
        descriptor: *const c_void,
        errcode_ret: *mut cl_int,
    ) -> cl_accelerator_intel;

    pub fn clGetAcceleratorInfoINTEL(
        accelerator: cl_accelerator_intel,
        param_name: cl_accelerator_info_intel,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;

    pub fn clRetainAcceleratorINTEL(accelerator: cl_accelerator_intel) -> cl_int;

    pub fn clReleaseAcceleratorINTEL(accelerator: cl_accelerator_intel) -> cl_int;

    pub fn clHostMemAllocINTEL(
        context: cl_context,
        properties: *const cl_mem_properties_intel,
        size: size_t,
        alignment: cl_uint,
        errcode_ret: *mut cl_int,
    );

    pub fn clDeviceMemAllocINTEL(
        context: cl_context,
        device: cl_device_id,
        properties: *const cl_mem_properties_intel,
        size: size_t,
        alignment: cl_uint,
        errcode_ret: *mut cl_int,
    );

    pub fn clSharedMemAllocINTEL(
        context: cl_context,
        device: cl_device_id,
        properties: *const cl_mem_properties_intel,
        size: size_t,
        alignment: cl_uint,
        errcode_ret: *mut cl_int,
    );

    pub fn clMemFreeINTEL(context: cl_context) -> cl_int;

    pub fn clMemBlockingFreeINTEL(context: cl_context, ptr: *mut c_void) -> cl_int;

    pub fn clGetMemAllocInfoINTEL(
        context: cl_context,
        ptr: *const c_void,
        param_name: cl_mem_info_intel,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;

    pub fn clSetKernelArgMemPointerINTEL(
        kernel: cl_kernel,
        arg_index: cl_uint,
        arg_value: *const c_void,
    ) -> cl_int;

    pub fn clEnqueueMemsetINTEL(
        command_queue: cl_command_queue,
        dst_ptr: *mut c_void,
        value: cl_int,
        size: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueMemFillINTEL(
        command_queue: cl_command_queue,
        dst_ptr: *mut c_void,
        pattern: *const c_void,
        pattern_size: size_t,
        size: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueMemcpyINTEL(
        command_queue: cl_command_queue,
        blocking: cl_bool,
        dst_ptr: *mut c_void,
        src_ptr: *const c_void,
        size: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueMigrateMemINTEL(
        command_queue: cl_command_queue,
        ptr: *const c_void,
        size: size_t,
        flags: cl_mem_migration_flags,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueMemAdviseINTEL(
        command_queue: cl_command_queue,
        ptr: *const c_void,
        size: size_t,
        advice: cl_mem_advice_intel,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clCreateBufferWithPropertiesINTEL(
        context: cl_context,
        properties: *const cl_mem_properties_intel,
        flags: cl_mem_flags,
        size: size_t,
        host_ptr: *mut c_void,
        errcode_ret: *mut cl_int,
    ) -> cl_mem;
}
