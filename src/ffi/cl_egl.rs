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

//! FFI bindings for cl_egl.h  
//! OpenCL extensions are documented in the [OpenCL-Registry](https://github.com/KhronosGroup/OpenCL-Registry)

#![allow(non_camel_case_types, non_upper_case_globals)]

pub use cl_sys::{
    cl_command_queue, cl_command_type, cl_context, cl_event, cl_int, cl_mem, cl_mem_flags, cl_uint,
};
use libc::{c_void, intptr_t};

// Command type for events created with clEnqueueAcquireEGLObjectsKHR
pub const CL_COMMAND_EGL_FENCE_SYNC_OBJECT_KHR: cl_command_type = 0x202F;
pub const CL_COMMAND_ACQUIRE_EGL_OBJECTS_KHR: cl_command_type = 0x202D;
pub const CL_COMMAND_RELEASE_EGL_OBJECTS_KHR: cl_command_type = 0x202E;

// Error type for clCreateFromEGLImageKHR
pub const CL_INVALID_EGL_OBJECT_KHR: cl_int = -1093;
pub const CL_EGL_RESOURCE_NOT_ACQUIRED_KHR: cl_int = -1092;

// CLeglImageKHR is an opaque handle to an EGLImage
pub type CLeglImageKHR = *mut c_void;

// CLeglDisplayKHR is an opaque handle to an EGLDisplay
pub type CLeglDisplayKHR = *mut c_void;

// CLeglSyncKHR is an opaque handle to an EGLSync object
pub type CLeglSyncKHR = *mut c_void;

// properties passed to clCreateFromEGLImageKHR
pub type cl_egl_image_properties_khr = intptr_t;

#[cfg_attr(not(target_os = "macos"), link(name = "OpenCL"))]
#[cfg_attr(target_os = "macos", link(name = "OpenCL", kind = "framework"))]
extern "system" {

    pub fn clCreateFromEGLImageKHR(
        context: cl_context,
        egldisplay: CLeglDisplayKHR,
        eglimage: CLeglImageKHR,
        flags: cl_mem_flags,
        properties: *const cl_egl_image_properties_khr,
        errcode_ret: *mut cl_int,
    ) -> cl_mem;

    pub fn clEnqueueAcquireEGLObjectsKHR(
        command_queue: cl_command_queue,
        num_objects: cl_uint,
        mem_objects: *const cl_mem,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueReleaseEGLObjectsKHR(
        command_queue: cl_command_queue,
        num_objects: cl_uint,
        mem_objects: *const cl_mem,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clCreateEventFromEGLSyncKHR(
        command_queue: cl_command_queue,
        sync: CLeglSyncKHR,
        display: CLeglDisplayKHR,
        errcode_ret: *mut cl_int,
    ) -> cl_int;

}
