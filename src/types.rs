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

//! OpenCL API data types.

#![allow(non_camel_case_types)]

// Note: cl_half = u16
// WARNING!  Unlike cl_ types in cl_platform.h, cl_bool is not guaranteed to be the same size as the bool in kernels.
pub use cl_sys::{
    cl_addressing_mode, cl_bitfield, cl_bool, cl_buffer_create_type, cl_build_status,
    cl_channel_order, cl_channel_type, cl_command_queue, cl_command_queue_info,
    cl_command_queue_properties, cl_command_type, cl_context, cl_context_info,
    cl_context_properties, cl_device_affinity_domain, cl_device_exec_capabilities,
    cl_device_fp_config, cl_device_id, cl_device_info, cl_device_local_mem_type,
    cl_device_mem_cache_type, cl_device_partition_property, cl_device_svm_capabilities,
    cl_device_type, cl_double, cl_event, cl_event_info, cl_filter_mode, cl_float, cl_half,
    cl_image_info, cl_int, cl_kernel, cl_kernel_arg_access_qualifier,
    cl_kernel_arg_address_qualifier, cl_kernel_arg_info, cl_kernel_arg_type_qualifier,
    cl_kernel_exec_info, cl_kernel_info, cl_kernel_sub_group_info, cl_kernel_work_group_info,
    cl_long, cl_map_flags, cl_mem, cl_mem_flags, cl_mem_info, cl_mem_migration_flags,
    cl_mem_object_type, cl_pipe_info, cl_pipe_properties, cl_platform_id, cl_platform_info,
    cl_profiling_info, cl_program, cl_program_binary_type, cl_program_build_info, cl_program_info,
    cl_queue_properties, cl_sampler, cl_sampler_info, cl_sampler_properties, cl_short,
    cl_svm_mem_flags, cl_uchar, cl_uint, cl_ulong, cl_ushort, CL_BLOCKING, CL_FALSE,
    CL_NON_BLOCKING, CL_TRUE,
};

use libc::size_t;

// Not defined in cl_sys
pub type cl_properties = cl_ulong;

// CL_VERSION_3_0
pub type cl_device_atomic_capabilities = cl_bitfield;
pub type cl_device_device_enqueue_capabilities = cl_bitfield;
pub type cl_khronos_vendor_id = cl_uint;
pub type cl_mem_properties = cl_properties;
pub type cl_version = cl_uint;

// Note: these structures are defined in cl_sys without the Debug trait.
#[derive(Debug)]
#[repr(C)]
pub struct cl_image_format {
    pub image_channel_order: cl_channel_order,
    pub image_channel_data_type: cl_channel_type,
}

#[derive(Debug)]
#[repr(C)]
pub struct cl_image_desc {
    pub image_type: cl_mem_object_type,
    pub image_width: size_t,
    pub image_height: size_t,
    pub image_depth: size_t,
    pub image_array_size: size_t,
    pub image_row_pitch: size_t,
    pub image_slice_pitch: size_t,
    pub num_mip_levels: cl_uint,
    pub num_samples: cl_uint,
    pub mem_object: cl_mem, // called buffer before OpenCL 2.0
}

#[derive(Debug)]
#[repr(C)]
pub struct cl_buffer_region {
    pub origin: size_t,
    pub size: size_t,
}

// CL_VERSION_3_0
pub const CL_NAME_VERSION_MAX_NAME_SIZE: usize = 64;
#[derive(Debug)]
#[repr(C)]
pub struct cl_name_version {
    pub version: cl_version,
    pub name: [cl_uchar; CL_NAME_VERSION_MAX_NAME_SIZE],
}
