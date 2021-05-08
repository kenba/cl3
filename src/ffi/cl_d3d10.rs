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

//! FFI bindings for cl_dx9_media_sharing.h  
//! cl_ecl_dx9_media_sharingxt.h contains OpenCL extensions that external D3D dependencies.  
//! OpenCL extensions are documented in the [OpenCL-Registry](https://github.com/KhronosGroup/OpenCL-Registry)

#![allow(non_camel_case_types, non_upper_case_globals)]

pub use cl_sys::{
    cl_command_queue, cl_command_type, cl_context, cl_context_info, cl_device_id, cl_event,
    cl_image_info, cl_int, cl_mem, cl_mem_flags, cl_mem_info, cl_platform_id, cl_uint,
};
use libc::c_void;

// cl_khr_d3d10_sharing

pub type ID3D10Buffer_ptr = *mut c_void;
pub type ID3D10Texture2D_ptr = *mut c_void;
pub type ID3D10Texture3D_ptr = *mut c_void;

pub const CL_INVALID_D3D10_DEVICE_KHR: cl_int = -1002;
pub const CL_INVALID_D3D10_RESOURCE_KHR: cl_int = -1003;
pub const CL_D3D10_RESOURCE_ALREADY_ACQUIRED_KHR: cl_int = -1004;
pub const CL_D3D10_RESOURCE_NOT_ACQUIRED_KHR: cl_int = -1005;

pub type cl_d3d10_device_source_khr = cl_uint;
pub const CL_D3D10_DEVICE_KHR: cl_d3d10_device_source_khr = 0x4010;
pub const CL_D3D10_DXGI_ADAPTER_KHR: cl_d3d10_device_source_khr = 0x4011;

pub type cl_d3d10_device_set_khr = cl_uint;
pub const CL_PREFERRED_DEVICES_FOR_D3D10_KHR: cl_d3d10_device_set_khr = 0x4012;
pub const CL_ALL_DEVICES_FOR_D3D10_KHR: cl_d3d10_device_set_khr = 0x4013;

// cl_context_info
pub const CL_CONTEXT_D3D10_DEVICE_KHR: cl_context_info = 0x4014;
pub const CL_CONTEXT_D3D10_PREFER_SHARED_RESOURCES_KHR: cl_context_info = 0x402C;

// cl_mem_info
pub const CL_MEM_D3D10_RESOURCE_KHR: cl_mem_info = 0x4015;

// cl_image_info
pub const CL_IMAGE_D3D10_SUBRESOURCE_KHR: cl_image_info = 0x4016;

// cl_command_type
pub const CL_COMMAND_ACQUIRE_D3D10_OBJECTS_KHR: cl_command_type = 0x4017;
pub const CL_COMMAND_RELEASE_D3D10_OBJECTS_KHR: cl_command_type = 0x4018;

#[cfg_attr(not(target_os = "macos"), link(name = "OpenCL"))]
#[cfg_attr(target_os = "macos", link(name = "OpenCL", kind = "framework"))]
extern "system" {

    pub fn clGetDeviceIDsFromD3D10KHR(
        platform: cl_platform_id,
        d3d_device_source: cl_d3d10_device_source_khr,
        d3d_object: *mut c_void,
        d3d_device_set: cl_d3d10_device_set_khr,
        num_entries: cl_uint,
        devices: *mut cl_device_id,
        num_devices: *mut cl_uint,
    ) -> cl_int;

    pub fn clCreateFromD3D10BufferKHR(
        context: cl_context,
        flags: cl_mem_flags,
        resource: ID3D10Buffer_ptr,
        errcode_ret: *mut cl_int,
    ) -> cl_mem;

    pub fn clCreateFromD3D10Texture2DKHR(
        context: cl_context,
        flags: cl_mem_flags,
        resource: ID3D10Texture2D_ptr,
        subresource: cl_uint,
        errcode_ret: *mut cl_int,
    ) -> cl_mem;

    pub fn clCreateFromD3D10Texture3DKHR(
        context: cl_context,
        flags: cl_mem_flags,
        resource: ID3D10Texture3D_ptr,
        subresource: cl_uint,
        errcode_ret: *mut cl_int,
    ) -> cl_mem;

    pub fn clEnqueueAcquireD3D10ObjectsKHR(
        command_queue: cl_command_queue,
        num_objects: cl_uint,
        mem_objects: *const cl_mem,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueReleaseD3D10ObjectsKHR(
        command_queue: cl_command_queue,
        num_objects: cl_uint,
        mem_objects: *const cl_mem,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;
}
