// Copyright (c) 2020-2022 Via Technology Ltd.
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

//! `OpenCL` API data types.

#![allow(non_camel_case_types)]

pub use opencl_sys::{
    cl_addressing_mode, cl_bitfield, cl_bool, cl_buffer_create_type, cl_buffer_region,
    cl_build_status, cl_channel_order, cl_channel_type, cl_char, cl_command_queue,
    cl_command_queue_info, cl_command_queue_properties, cl_command_type, cl_context,
    cl_context_info, cl_context_properties, cl_device_affinity_domain,
    cl_device_atomic_capabilities, cl_device_device_enqueue_capabilities,
    cl_device_exec_capabilities, cl_device_fp_config, cl_device_id, cl_device_info,
    cl_device_local_mem_type, cl_device_mem_cache_type, cl_device_partition_property,
    cl_device_svm_capabilities, cl_device_type, cl_double, cl_event, cl_event_info, cl_filter_mode,
    cl_float, cl_half, cl_image_desc, cl_image_format, cl_image_info, cl_int, cl_kernel,
    cl_kernel_arg_access_qualifier, cl_kernel_arg_address_qualifier, cl_kernel_arg_info,
    cl_kernel_arg_type_qualifier, cl_kernel_exec_info, cl_kernel_info, cl_kernel_sub_group_info,
    cl_kernel_work_group_info, cl_khronos_vendor_id, cl_long, cl_map_flags, cl_mem, cl_mem_flags,
    cl_mem_info, cl_mem_migration_flags, cl_mem_object_type, cl_mem_properties, cl_name_version,
    cl_pipe_info, cl_pipe_properties, cl_platform_id, cl_platform_info, cl_profiling_info,
    cl_program, cl_program_binary_type, cl_program_build_info, cl_program_info, cl_properties,
    cl_queue_properties, cl_sampler, cl_sampler_info, cl_sampler_properties, cl_short,
    cl_svm_mem_flags, cl_uchar, cl_uint, cl_ulong, cl_ushort, cl_version, CL_BLOCKING, CL_FALSE,
    CL_NON_BLOCKING, CL_TRUE,
};
