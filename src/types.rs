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
    cl_addressing_mode, cl_bitfield, cl_bool, cl_buffer_create_type, cl_buffer_region,
    cl_build_status, cl_channel_order, cl_channel_type, cl_command_queue, cl_command_queue_info,
    cl_command_queue_properties, cl_command_type, cl_context, cl_context_info,
    cl_context_properties, cl_device_affinity_domain, cl_device_atomic_capabilities,
    cl_device_device_enqueue_capabilities, cl_device_exec_capabilities, cl_device_fp_config,
    cl_device_id, cl_device_info, cl_device_local_mem_type, cl_device_mem_cache_type,
    cl_device_partition_property, cl_device_svm_capabilities, cl_device_type, cl_event,
    cl_event_info, cl_filter_mode, cl_image_desc, cl_image_format, cl_image_info, cl_kernel,
    cl_kernel_arg_access_qualifier, cl_kernel_arg_address_qualifier, cl_kernel_arg_info,
    cl_kernel_arg_type_qualifier, cl_kernel_exec_info, cl_kernel_info, cl_kernel_sub_group_info,
    cl_kernel_work_group_info, cl_khronos_vendor_id, cl_map_flags, cl_mem, cl_mem_flags,
    cl_mem_info, cl_mem_migration_flags, cl_mem_object_type, cl_mem_properties, cl_name_version,
    cl_pipe_info, cl_pipe_properties, cl_platform_id, cl_platform_info, cl_profiling_info,
    cl_program, cl_program_binary_type, cl_program_build_info, cl_program_info, cl_properties,
    cl_queue_properties, cl_sampler, cl_sampler_info, cl_sampler_properties, cl_svm_mem_flags,
    cl_version,
};

#[cfg(feature = "static_runtime")]
pub use crate::runtime::OpenClTypes::{
    cl_GLenum, cl_GLint, cl_GLuint, cl_char, cl_double, cl_float, cl_half, cl_int, cl_long,
    cl_short, cl_uchar, cl_uint, cl_ulong, cl_ushort, DXGI_FORMAT,
};

#[cfg(feature = "dynamic_runtime")]
pub use crate::runtime::OpenClTypes::cl_platform::{
    cl_GLenum, cl_GLint, cl_GLuint, cl_char, cl_double, cl_float, cl_half, cl_int, cl_long,
    cl_short, cl_uchar, cl_uint, cl_ulong, cl_ushort, DXGI_FORMAT,
};

pub mod cl_d3d11 {
    pub use crate::runtime::OpenClTypes::cl_d3d11::{
        cl_d3d11_device_set_khr, cl_d3d11_device_source_khr,
    };
}

pub mod cl_dx9_media_sharing {
    pub use crate::runtime::OpenClTypes::cl_dx9_media_sharing::{
        cl_dx9_device_set_intel, cl_dx9_device_source_intel, cl_dx9_media_adapter_set_khr,
        cl_dx9_media_adapter_type_khr, cl_dx9_surface_info_khr, IDirect3DSurface9_ptr, D3DFORMAT,
        HANDLE,
    };
}

pub mod cl_egl {
    pub use crate::runtime::OpenClTypes::cl_egl::{
        cl_egl_image_properties_khr, CLeglDisplayKHR, CLeglImageKHR, CLeglSyncKHR,
    };
}

pub mod cl_ext {
    #[cfg(feature = "static_runtime")]
    pub use crate::runtime::OpenClTypes::{
        cl_accelerator_info_intel, cl_accelerator_intel, cl_accelerator_type_intel,
        cl_amd_device_attribute_query, cl_amd_device_topology, cl_command_buffer_flags_khr,
        cl_command_buffer_info_khr, cl_command_buffer_khr, cl_command_buffer_properties_khr,
        cl_command_buffer_state_khr, cl_command_buffer_structure_type_khr,
        cl_command_queue_capabilities_intel, cl_command_termination_reason_arm,
        cl_context_memory_initialize_khr, cl_device_command_buffer_capabilities_khr,
        cl_device_controlled_termination_capabilities_arm, cl_device_feature_capabilities_intel,
        cl_device_fp_atomic_capabilities_ext,
        cl_device_integer_dot_product_acceleration_properties_khr,
        cl_device_integer_dot_product_capabilities_khr, cl_device_partition_property_ext,
        cl_device_pci_bus_info_khr, cl_device_scheduling_controls_capabilities_arm,
        cl_device_svm_capabilities_arm, cl_device_unified_shared_memory_capabilities_intel,
        cl_diagnostics_verbose_level, cl_external_memory_handle_type_khr,
        cl_external_semaphore_handle_type_khr, cl_icdl_info, cl_image_pitch_info_qcom,
        cl_image_requirements_info_ext, cl_import_properties_arm, cl_intel_avc_motion_estimation,
        cl_kernel_exec_info_arm, cl_mem_advice_intel, cl_mem_alloc_flags_img,
        cl_mem_alloc_flags_intel, cl_mem_android_native_buffer_host_ptr, cl_mem_ext_host_ptr,
        cl_mem_info_intel, cl_mem_ion_host_ptr, cl_mem_migration_flags_ext,
        cl_mem_properties_intel, cl_mipmap_filter_mode_img, cl_motion_detect_desc_intel,
        cl_motion_estimation_desc_intel, cl_mutable_base_config_khr, cl_mutable_command_info_khr,
        cl_mutable_command_khr, cl_mutable_dispatch_arg_khr, cl_mutable_dispatch_asserts_khr,
        cl_mutable_dispatch_config_khr, cl_mutable_dispatch_exec_info_khr,
        cl_mutable_dispatch_fields_khr, cl_name_version_khr,
        cl_ndrange_kernel_command_properties_khr, cl_nv_device_attribute_query,
        cl_platform_command_buffer_capabilities_khr, cl_qcom_ext_host_ptr,
        cl_queue_family_properties_intel, cl_queue_priority_khr, cl_queue_properties_khr,
        cl_queue_throttle_khr, cl_semaphore_info_khr, cl_semaphore_khr, cl_semaphore_payload_khr,
        cl_semaphore_properties_khr, cl_semaphore_reimport_properties_khr, cl_semaphore_type_khr,
        cl_svm_mem_flags_arm, cl_sync_point_khr, cl_unified_shared_memory_type_intel,
        cl_version_khr,
    };

    #[cfg(feature = "dynamic_runtime")]
    pub use crate::runtime::OpenClTypes::cl_ext::{
        cl_accelerator_info_intel, cl_accelerator_intel, cl_accelerator_type_intel,
        cl_amd_device_attribute_query, cl_amd_device_topology, cl_command_buffer_flags_khr,
        cl_command_buffer_info_khr, cl_command_buffer_khr, cl_command_buffer_properties_khr,
        cl_command_buffer_state_khr, cl_command_buffer_structure_type_khr,
        cl_command_queue_capabilities_intel, cl_command_termination_reason_arm,
        cl_context_memory_initialize_khr, cl_device_command_buffer_capabilities_khr,
        cl_device_controlled_termination_capabilities_arm, cl_device_feature_capabilities_intel,
        cl_device_fp_atomic_capabilities_ext,
        cl_device_integer_dot_product_acceleration_properties_khr,
        cl_device_integer_dot_product_capabilities_khr, cl_device_partition_property_ext,
        cl_device_pci_bus_info_khr, cl_device_scheduling_controls_capabilities_arm,
        cl_device_svm_capabilities_arm, cl_device_unified_shared_memory_capabilities_intel,
        cl_diagnostics_verbose_level, cl_external_memory_handle_type_khr,
        cl_external_semaphore_handle_type_khr, cl_icdl_info, cl_image_pitch_info_qcom,
        cl_image_requirements_info_ext, cl_import_properties_arm, cl_intel_avc_motion_estimation,
        cl_kernel_exec_info_arm, cl_mem_advice_intel, cl_mem_alloc_flags_img,
        cl_mem_alloc_flags_intel, cl_mem_android_native_buffer_host_ptr, cl_mem_ext_host_ptr,
        cl_mem_info_intel, cl_mem_ion_host_ptr, cl_mem_migration_flags_ext,
        cl_mem_properties_intel, cl_mipmap_filter_mode_img, cl_motion_detect_desc_intel,
        cl_motion_estimation_desc_intel, cl_mutable_base_config_khr, cl_mutable_command_info_khr,
        cl_mutable_command_khr, cl_mutable_dispatch_arg_khr, cl_mutable_dispatch_asserts_khr,
        cl_mutable_dispatch_config_khr, cl_mutable_dispatch_exec_info_khr,
        cl_mutable_dispatch_fields_khr, cl_name_version_khr,
        cl_ndrange_kernel_command_properties_khr, cl_nv_device_attribute_query,
        cl_platform_command_buffer_capabilities_khr, cl_qcom_ext_host_ptr,
        cl_queue_family_properties_intel, cl_queue_priority_khr, cl_queue_properties_khr,
        cl_queue_throttle_khr, cl_semaphore_info_khr, cl_semaphore_khr, cl_semaphore_payload_khr,
        cl_semaphore_properties_khr, cl_semaphore_reimport_properties_khr, cl_semaphore_type_khr,
        cl_svm_mem_flags_arm, cl_sync_point_khr, cl_unified_shared_memory_type_intel,
        cl_version_khr,
    };
}

pub mod cl_gl {
    #[cfg(feature = "static_runtime")]
    pub use crate::runtime::OpenClTypes::{
        cl_GLsync, cl_gl_context_info, cl_gl_object_type, cl_gl_platform_info, cl_gl_texture_info,
    };

    #[cfg(feature = "dynamic_runtime")]
    pub use crate::runtime::OpenClTypes::cl_gl::{
        cl_GLsync, cl_gl_context_info, cl_gl_object_type, cl_gl_platform_info, cl_gl_texture_info,
    };
}

pub mod cl_icd {
    pub use crate::runtime::OpenClTypes::cl_icd::cl_icd_dispatch;
}

pub mod cl_layer {
    pub use crate::runtime::OpenClTypes::cl_layer::{cl_layer_api_version, cl_layer_info};
}
