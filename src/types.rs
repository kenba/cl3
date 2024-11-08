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

pub use crate::runtime::OpenClTypes::{
    cl_GLenum, cl_GLint, cl_GLsync, cl_GLuint, cl_accelerator_info_intel, cl_accelerator_intel,
    cl_accelerator_type_intel, cl_addressing_mode, cl_amd_device_topology, cl_bool,
    cl_buffer_create_type, cl_command_buffer_info_khr, cl_command_buffer_khr,
    cl_command_buffer_properties_khr, cl_command_queue, cl_command_queue_info,
    cl_command_queue_properties, cl_command_type, cl_context, cl_context_info,
    cl_context_properties, cl_device_id, cl_device_info,
    cl_device_integer_dot_product_acceleration_properties_khr, cl_device_partition_property,
    cl_device_partition_property_ext, cl_device_pci_bus_info_khr, cl_device_type,
    cl_egl::{cl_egl_image_properties_khr, CLeglDisplayKHR, CLeglImageKHR},
    cl_event, cl_event_info, cl_external_semaphore_handle_type_khr, cl_filter_mode,
    cl_gl_context_info, cl_gl_texture_info,
    cl_icd::{
        cl_dx9_device_set_intel, cl_dx9_device_source_intel, cl_icd_dispatch, CLeglSyncKHR,
        IDirect3DSurface9_ptr, HANDLE,
    },
    cl_icdl_info, cl_image_desc, cl_image_format, cl_image_info, cl_image_pitch_info_qcom,
    cl_image_requirements_info_ext, cl_import_properties_arm, cl_int, cl_kernel,
    cl_kernel_arg_info, cl_kernel_exec_info, cl_kernel_exec_info_arm, cl_kernel_info,
    cl_kernel_sub_group_info, cl_kernel_work_group_info,
    cl_layer::cl_layer_info,
    cl_map_flags, cl_mem, cl_mem_advice_intel, cl_mem_flags, cl_mem_info, cl_mem_info_intel,
    cl_mem_migration_flags, cl_mem_migration_flags_ext, cl_mem_object_type, cl_mem_properties,
    cl_mem_properties_intel, cl_mipmap_filter_mode_img, cl_mutable_base_config_khr,
    cl_mutable_command_info_khr, cl_mutable_command_khr, cl_name_version,
    cl_ndrange_kernel_command_properties_khr, cl_pipe_info, cl_platform_id, cl_platform_info,
    cl_profiling_info, cl_program, cl_program_build_info, cl_program_info, cl_queue_properties,
    cl_queue_properties_khr, cl_sampler, cl_sampler_info, cl_sampler_properties,
    cl_semaphore_info_khr, cl_semaphore_khr, cl_semaphore_payload_khr, cl_semaphore_properties_khr,
    cl_semaphore_reimport_properties_khr, cl_svm_mem_flags, cl_svm_mem_flags_arm,
    cl_sync_point_khr, cl_uchar, cl_uint, cl_ulong, cl_unified_shared_memory_type_intel,
};
