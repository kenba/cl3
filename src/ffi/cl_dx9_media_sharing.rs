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
//! cl_ecl_dx9_media_sharingxt.h contains OpenCL extensions that provide interoperability with Direct3D 9.  
//! OpenCL extensions are documented in the [OpenCL-Registry](https://github.com/KhronosGroup/OpenCL-Registry)

#![allow(non_camel_case_types, non_upper_case_globals)]

pub use cl_sys::{
    cl_command_queue, cl_command_type, cl_context, cl_context_info, cl_device_id, cl_event,
    cl_image_info, cl_int, cl_mem, cl_mem_flags, cl_mem_info, cl_platform_id, cl_uint,
};
use libc::c_void;

// cl_khr_dx9_media_sharing

pub type IDirect3DSurface9_ptr = *mut c_void;
pub type HANDLE = *mut c_void;

// #if defined(_WIN32)
#[derive(Debug)]
#[repr(C)]
pub struct cl_dx9_surface_info_khr {
    pub resource: IDirect3DSurface9_ptr,
    pub shared_handle: HANDLE,
}
// #endif

pub const CL_INVALID_DX9_MEDIA_ADAPTER_KHR: cl_int = -1010;
pub const CL_INVALID_DX9_MEDIA_SURFACE_KHR: cl_int = -1011;
pub const CL_DX9_MEDIA_SURFACE_ALREADY_ACQUIRED_KHR: cl_int = -1012;
pub const CL_DX9_MEDIA_SURFACE_NOT_ACQUIRED_KHR: cl_int = -1013;

pub type cl_dx9_media_adapter_type_khr = cl_uint;
pub const CL_ADAPTER_D3D9_KHR: cl_dx9_media_adapter_type_khr = 0x2020;
pub const CL_ADAPTER_D3D9EX_KHR: cl_dx9_media_adapter_type_khr = 0x2021;
pub const CL_ADAPTER_DXVA_KHR: cl_dx9_media_adapter_type_khr = 0x2022;

pub type cl_dx9_media_adapter_set_khr = cl_uint;
pub const CL_PREFERRED_DEVICES_FOR_DX9_MEDIA_ADAPTER_KHR: cl_dx9_media_adapter_set_khr = 0x2023;
pub const CL_ALL_DEVICES_FOR_DX9_MEDIA_ADAPTER_KHR: cl_dx9_media_adapter_set_khr = 0x2024;

// cl_context_info
pub const CL_CONTEXT_ADAPTER_D3D9_KHR: cl_context_info = 0x2025;
pub const CL_CONTEXT_ADAPTER_D3D9EX_KHR: cl_context_info = 0x2026;
pub const CL_CONTEXT_ADAPTER_DXVA_KHR: cl_context_info = 0x2027;

// cl_mem_info
pub const CL_MEM_DX9_MEDIA_ADAPTER_TYPE_KHR: cl_mem_info = 0x2028;
pub const CL_MEM_DX9_MEDIA_SURFACE_INFO_KHR: cl_mem_info = 0x2029;

// cl_image_info
pub const CL_IMAGE_DX9_MEDIA_PLANE_KHR: cl_image_info = 0x202A;

// cl_command_type
pub const CL_COMMAND_ACQUIRE_DX9_MEDIA_SURFACES_KHR: cl_command_type = 0x202B;
pub const CL_COMMAND_RELEASE_DX9_MEDIA_SURFACES_KHR: cl_command_type = 0x202C;

// cl_intel_dx9_media_sharing

pub const CL_INVALID_DX9_DEVICE_INTEL: cl_int = -1010;
pub const CL_INVALID_DX9_RESOURCE_INTEL: cl_int = -1011;
pub const CL_DX9_RESOURCE_ALREADY_ACQUIRED_INTEL: cl_int = -1012;
pub const CL_DX9_RESOURCE_NOT_ACQUIRED_INTEL: cl_int = -1013;

pub type cl_dx9_device_source_intel = cl_uint;
pub const CL_D3D9_DEVICE_INTEL: cl_dx9_device_source_intel = 0x4022;
pub const CL_D3D9EX_DEVICE_INTEL: cl_dx9_device_source_intel = 0x4070;
pub const CL_DXVA_DEVICE_INTEL: cl_dx9_device_source_intel = 0x4071;

pub type cl_dx9_device_set_intel = cl_uint;
pub const CL_PREFERRED_DEVICES_FOR_DX9_INTEL: cl_dx9_device_set_intel = 0x4024;
pub const CL_ALL_DEVICES_FOR_DX9_INTEL: cl_dx9_device_set_intel = 0x4025;

// cl_context_info
pub const CL_CONTEXT_D3D9_DEVICE_INTEL: cl_context_info = 0x4026;
pub const CL_CONTEXT_D3D9EX_DEVICE_INTEL: cl_context_info = 0x4072;
pub const CL_CONTEXT_DXVA_DEVICE_INTEL: cl_context_info = 0x4073;

// cl_mem_info
pub const CL_MEM_DX9_RESOURCE_INTEL: cl_mem_info = 0x4027;
pub const CL_MEM_DX9_SHARED_HANDLE_INTEL: cl_mem_info = 0x4074;

// cl_image_info
pub const CL_IMAGE_DX9_PLANE_INTEL: cl_image_info = 0x4075;

// cl_command_type
pub const CL_COMMAND_ACQUIRE_DX9_OBJECTS_INTEL: cl_command_type = 0x402A;
pub const CL_COMMAND_RELEASE_DX9_OBJECTS_INTEL: cl_command_type = 0x402B;

#[cfg_attr(not(target_os = "macos"), link(name = "OpenCL"))]
#[cfg_attr(target_os = "macos", link(name = "OpenCL", kind = "framework"))]
extern "system" {

    pub fn clGetDeviceIDsFromDX9MediaAdapterKHR(
        platform: cl_platform_id,
        num_media_adapters: cl_uint,
        media_adapter_type: *mut cl_dx9_media_adapter_type_khr,
        media_adapters: *mut c_void,
        media_adapter_set: cl_dx9_media_adapter_set_khr,
        num_entries: cl_uint,
        devices: *mut cl_device_id,
        num_devices: *mut cl_uint,
    ) -> cl_int;

    pub fn clCreateFromDX9MediaSurfaceKHR(
        context: cl_context,
        flags: cl_mem_flags,
        adapter_type: cl_dx9_media_adapter_type_khr,
        surface_info: *mut c_void,
        plane: cl_uint,
        errcode_ret: *mut cl_int,
    ) -> cl_mem;

    pub fn clEnqueueAcquireDX9MediaSurfacesKHR(
        command_queue: cl_command_queue,
        num_objects: cl_uint,
        mem_objects: *const cl_mem,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueReleaseDX9MediaSurfacesKHR(
        command_queue: cl_command_queue,
        num_objects: cl_uint,
        mem_objects: *const cl_mem,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clGetDeviceIDsFromDX9INTEL(
        platform: cl_platform_id,
        dx9_device_source: cl_dx9_device_source_intel,
        dx9_object: *mut c_void,
        dx9_device_set: cl_dx9_device_set_intel,
        num_entries: cl_uint,
        devices: *mut cl_device_id,
        num_devices: *mut cl_uint,
    ) -> cl_int;

    pub fn clCreateFromDX9MediaSurfaceINTEL(
        context: cl_context,
        flags: cl_mem_flags,
        resource: IDirect3DSurface9_ptr,
        sharedHandle: HANDLE,
        plane: cl_uint,
        errcode_ret: *mut cl_int,
    ) -> cl_mem;

    pub fn clEnqueueAcquireDX9ObjectsINTEL(
        command_queue: cl_command_queue,
        num_objects: cl_uint,
        mem_objects: *const cl_mem,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueReleaseDX9ObjectsINTEL(
        command_queue: cl_command_queue,
        num_objects: cl_uint,
        mem_objects: *const cl_mem,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

}
