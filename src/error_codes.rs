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

//! `OpenCL` API Error Codes.

use crate::{
    constants::{cl_d3d11::*, cl_dx9_media_sharing::*, cl_egl::*, cl_ext::*, cl_gl::*, *},
    types::*,
};

use std::fmt;
use thiserror::Error;

#[must_use]
pub const fn error_text(error_code: cl_int) -> &'static str {
    match error_code {
        CL_SUCCESS => "CL_SUCCESS",
        CL_DEVICE_NOT_FOUND => "CL_DEVICE_NOT_FOUND",
        CL_DEVICE_NOT_AVAILABLE => "CL_DEVICE_NOT_AVAILABLE",
        CL_COMPILER_NOT_AVAILABLE => "CL_COMPILER_NOT_AVAILABLE",
        CL_MEM_OBJECT_ALLOCATION_FAILURE => "CL_MEM_OBJECT_ALLOCATION_FAILURE",
        CL_OUT_OF_RESOURCES => "CL_OUT_OF_RESOURCES",
        CL_OUT_OF_HOST_MEMORY => "CL_OUT_OF_HOST_MEMORY",
        CL_PROFILING_INFO_NOT_AVAILABLE => "CL_PROFILING_INFO_NOT_AVAILABLE",
        CL_MEM_COPY_OVERLAP => "CL_MEM_COPY_OVERLAP",
        CL_IMAGE_FORMAT_MISMATCH => "CL_IMAGE_FORMAT_MISMATCH",
        CL_IMAGE_FORMAT_NOT_SUPPORTED => "CL_IMAGE_FORMAT_NOT_SUPPORTED",
        CL_BUILD_PROGRAM_FAILURE => "CL_BUILD_PROGRAM_FAILURE",
        CL_MAP_FAILURE => "CL_MAP_FAILURE",
        CL_MISALIGNED_SUB_BUFFER_OFFSET => "CL_MISALIGNED_SUB_BUFFER_OFFSET",
        CL_EXEC_STATUS_ERROR_FOR_EVENTS_IN_WAIT_LIST => {
            "CL_EXEC_STATUS_ERROR_FOR_EVENTS_IN_WAIT_LIST"
        }
        CL_COMPILE_PROGRAM_FAILURE => "CL_COMPILE_PROGRAM_FAILURE",
        CL_LINKER_NOT_AVAILABLE => "CL_LINKER_NOT_AVAILABLE",
        CL_LINK_PROGRAM_FAILURE => "CL_LINK_PROGRAM_FAILURE",
        CL_DEVICE_PARTITION_FAILED => "CL_DEVICE_PARTITION_FAILED",
        CL_KERNEL_ARG_INFO_NOT_AVAILABLE => "CL_KERNEL_ARG_INFO_NOT_AVAILABLE",

        CL_INVALID_VALUE => "CL_INVALID_VALUE",
        CL_INVALID_DEVICE_TYPE => "CL_INVALID_DEVICE_TYPE",
        CL_INVALID_PLATFORM => "CL_INVALID_PLATFORM",
        CL_INVALID_DEVICE => "CL_INVALID_DEVICE",
        CL_INVALID_CONTEXT => "CL_INVALID_CONTEXT",
        CL_INVALID_QUEUE_PROPERTIES => "CL_INVALID_QUEUE_PROPERTIES",
        CL_INVALID_COMMAND_QUEUE => "CL_INVALID_COMMAND_QUEUE",
        CL_INVALID_HOST_PTR => "CL_INVALID_HOST_PTR",
        CL_INVALID_MEM_OBJECT => "CL_INVALID_MEM_OBJECT",
        CL_INVALID_IMAGE_FORMAT_DESCRIPTOR => "CL_INVALID_IMAGE_FORMAT_DESCRIPTOR",
        CL_INVALID_IMAGE_SIZE => "CL_INVALID_IMAGE_SIZE",
        CL_INVALID_SAMPLER => "CL_INVALID_SAMPLER",
        CL_INVALID_BINARY => "CL_INVALID_BINARY",
        CL_INVALID_BUILD_OPTIONS => "CL_INVALID_BUILD_OPTIONS",
        CL_INVALID_PROGRAM => "CL_INVALID_PROGRAM",
        CL_INVALID_PROGRAM_EXECUTABLE => "CL_INVALID_PROGRAM_EXECUTABLE",
        CL_INVALID_KERNEL_NAME => "CL_INVALID_KERNEL_NAME",
        CL_INVALID_KERNEL_DEFINITION => "CL_INVALID_KERNEL_DEFINITION",
        CL_INVALID_KERNEL => "CL_INVALID_KERNEL",
        CL_INVALID_ARG_INDEX => "CL_INVALID_ARG_INDEX",
        CL_INVALID_ARG_VALUE => "CL_INVALID_ARG_VALUE",
        CL_INVALID_ARG_SIZE => "CL_INVALID_ARG_SIZE",
        CL_INVALID_KERNEL_ARGS => "CL_INVALID_KERNEL_ARGS",
        CL_INVALID_WORK_DIMENSION => "CL_INVALID_WORK_DIMENSION",
        CL_INVALID_WORK_GROUP_SIZE => "CL_INVALID_WORK_GROUP_SIZE",
        CL_INVALID_WORK_ITEM_SIZE => "CL_INVALID_WORK_ITEM_SIZE",
        CL_INVALID_GLOBAL_OFFSET => "CL_INVALID_GLOBAL_OFFSET",
        CL_INVALID_EVENT_WAIT_LIST => "CL_INVALID_EVENT_WAIT_LIST",
        CL_INVALID_EVENT => "CL_INVALID_EVENT",
        CL_INVALID_OPERATION => "CL_INVALID_OPERATION",
        CL_INVALID_GL_OBJECT => "CL_INVALID_GL_OBJECT",
        CL_INVALID_BUFFER_SIZE => "CL_INVALID_BUFFER_SIZE",
        CL_INVALID_MIP_LEVEL => "CL_INVALID_MIP_LEVEL",
        CL_INVALID_GLOBAL_WORK_SIZE => "CL_INVALID_GLOBAL_WORK_SIZE",
        CL_INVALID_PROPERTY => "CL_INVALID_PROPERTY",
        CL_INVALID_IMAGE_DESCRIPTOR => "CL_INVALID_IMAGE_DESCRIPTOR",
        CL_INVALID_COMPILER_OPTIONS => "CL_INVALID_COMPILER_OPTIONS",
        CL_INVALID_LINKER_OPTIONS => "CL_INVALID_LINKER_OPTIONS",
        CL_INVALID_DEVICE_PARTITION_COUNT => "CL_INVALID_DEVICE_PARTITION_COUNT",
        CL_INVALID_PIPE_SIZE => "CL_INVALID_PIPE_SIZE",
        CL_INVALID_DEVICE_QUEUE => "CL_INVALID_DEVICE_QUEUE",
        CL_INVALID_SPEC_ID => "CL_INVALID_SPEC_ID",
        CL_MAX_SIZE_RESTRICTION_EXCEEDED => "CL_MAX_SIZE_RESTRICTION_EXCEEDED",

        CL_INVALID_GL_SHAREGROUP_REFERENCE_KHR => "CL_INVALID_GL_SHAREGROUP_REFERENCE_KHR",
        CL_PLATFORM_NOT_FOUND_KHR => "CL_PLATFORM_NOT_FOUND_KHR",

        CL_INVALID_D3D11_DEVICE_KHR => "CL_INVALID_D3D11_DEVICE_KHR",
        CL_INVALID_D3D11_RESOURCE_KHR => "CL_INVALID_D3D11_RESOURCE_KHR",
        CL_D3D11_RESOURCE_ALREADY_ACQUIRED_KHR => "CL_D3D11_RESOURCE_ALREADY_ACQUIRED_KHR",
        CL_D3D11_RESOURCE_NOT_ACQUIRED_KHR => "CL_D3D11_RESOURCE_NOT_ACQUIRED_KHR",

        CL_INVALID_DX9_MEDIA_ADAPTER_KHR => "CL_INVALID_DX9_MEDIA_ADAPTER_KHR",
        CL_INVALID_DX9_MEDIA_SURFACE_KHR => "CL_INVALID_DX9_MEDIA_SURFACE_KHR",
        CL_DX9_MEDIA_SURFACE_ALREADY_ACQUIRED_KHR => "CL_DX9_MEDIA_SURFACE_ALREADY_ACQUIRED_KHR",
        CL_DX9_MEDIA_SURFACE_NOT_ACQUIRED_KHR => "CL_DX9_MEDIA_SURFACE_NOT_ACQUIRED_KHR",

        CL_DEVICE_PARTITION_FAILED_EXT => "CL_DEVICE_PARTITION_FAILED_EXT",
        CL_INVALID_PARTITION_COUNT_EXT => "CL_INVALID_PARTITION_COUNT_EXT",
        CL_INVALID_PARTITION_NAME_EXT => "CL_INVALID_PARTITION_NAME_EXT",

        CL_EGL_RESOURCE_NOT_ACQUIRED_KHR => "CL_EGL_RESOURCE_NOT_ACQUIRED_KHR",
        CL_INVALID_EGL_OBJECT_KHR => "CL_INVALID_EGL_OBJECT_KHR",

        CL_INVALID_ACCELERATOR_INTEL => "CL_INVALID_ACCELERATOR_INTEL",
        CL_INVALID_ACCELERATOR_TYPE_INTEL => "CL_INVALID_ACCELERATOR_TYPE_INTEL",
        CL_INVALID_ACCELERATOR_DESCRIPTOR_INTEL => "CL_INVALID_ACCELERATOR_DESCRIPTOR_INTEL",
        CL_ACCELERATOR_TYPE_NOT_SUPPORTED_INTEL => "CL_ACCELERATOR_TYPE_NOT_SUPPORTED_INTEL",

        CL_COMMAND_TERMINATED_ITSELF_WITH_FAILURE_ARM => {
            "CL_COMMAND_TERMINATED_ITSELF_WITH_FAILURE_ARM"
        }

        CL_CONTEXT_TERMINATED_KHR => "CL_CONTEXT_TERMINATED_KHR",
        CL_INVALID_SEMAPHORE_KHR => "CL_INVALID_SEMAPHORE_KHR",

        CL_INVALID_COMMAND_BUFFER_KHR => "CL_INVALID_COMMAND_BUFFER_KHR",
        CL_INVALID_SYNC_POINT_WAIT_LIST_KHR => "CL_INVALID_SYNC_POINT_WAIT_LIST_KHR",
        CL_INCOMPATIBLE_COMMAND_QUEUE_KHR => "CL_INCOMPATIBLE_COMMAND_QUEUE_KHR",

        _ => "UNKNOWN_ERROR",
    }
}

#[derive(Debug, Error)]
/// `ClError` is a newtype around the `OpenCL` `cl_int` error number
pub struct ClError(pub cl_int);

/// Implement the From trait
impl From<cl_int> for ClError {
    fn from(error: cl_int) -> Self {
        Self(error)
    }
}

/// Implement the From trait for &str
impl From<ClError> for &str {
    fn from(error: ClError) -> Self {
        error_text(error.0)
    }
}

/// Implement the From trait for String
impl From<ClError> for String {
    fn from(error: ClError) -> Self {
        Self::from(error_text(error.0))
    }
}

/// Implement the Display trait
impl fmt::Display for ClError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", error_text(self.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_text() {
        let cl_success_text = error_text(CL_SUCCESS);
        assert_eq!("CL_SUCCESS", cl_success_text);

        let cl_device_not_found_text = error_text(CL_DEVICE_NOT_FOUND);
        assert_eq!("CL_DEVICE_NOT_FOUND", cl_device_not_found_text);

        let cl_max_size_restriction_exceeded_text = error_text(CL_MAX_SIZE_RESTRICTION_EXCEEDED);
        assert_eq!(
            "CL_MAX_SIZE_RESTRICTION_EXCEEDED",
            cl_max_size_restriction_exceeded_text
        );

        let cl_platform_not_found_khr_text = error_text(CL_PLATFORM_NOT_FOUND_KHR);
        assert_eq!("CL_PLATFORM_NOT_FOUND_KHR", cl_platform_not_found_khr_text);

        let unknown_error_text = error_text(CL_MAX_SIZE_RESTRICTION_EXCEEDED - 1);
        assert_eq!("UNKNOWN_ERROR", unknown_error_text);
    }

    #[test]
    fn test_error_type() {
        let cl_success_text = error_text(CL_SUCCESS);
        assert_eq!("CL_SUCCESS", cl_success_text);

        let error_01: ClError = From::from(CL_DEVICE_NOT_FOUND);
        println!("CL_DEVICE_NOT_FOUND: {:?}", error_01);
        println!("CL_DEVICE_NOT_FOUND: {}", error_01);
        println!("CL_DEVICE_NOT_FOUND: {}", String::from(error_01));

        let error_30: ClError = From::from(CL_INVALID_VALUE);
        println!("CL_INVALID_VALUE: {:?}", error_30);
        println!("CL_INVALID_VALUE: {}", error_30);
        let error_30_str: &str = error_30.into();
        println!("CL_INVALID_VALUE: {}", error_30_str);

        let error_72: ClError = From::from(CL_MAX_SIZE_RESTRICTION_EXCEEDED);
        println!("CL_MAX_SIZE_RESTRICTION_EXCEEDED: {:?}", error_72);
        println!("CL_MAX_SIZE_RESTRICTION_EXCEEDED: {}", error_72);
        println!(
            "CL_MAX_SIZE_RESTRICTION_EXCEEDED: {}",
            String::from(error_72)
        );

        let error_unknown: ClError = From::from(CL_MAX_SIZE_RESTRICTION_EXCEEDED - 1);
        println!("UNKNOWN_ERROR: {:?}", error_unknown);
        println!("UNKNOWN_ERROR: {}", error_unknown);
        println!("UNKNOWN_ERROR: {}", String::from(error_unknown));
    }
}
