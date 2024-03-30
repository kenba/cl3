// Copyright (c) 2020-2024 Via Technology Ltd. All Rights Reserved.
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

//! `OpenCL` Event Object API.

#![allow(non_camel_case_types)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]

pub use opencl_sys::{
    cl_command_queue, cl_command_type, cl_context, cl_event, cl_event_info, cl_int,
    cl_profiling_info, cl_uint, cl_ulong, CL_COMMAND_ACQUIRE_GL_OBJECTS, CL_COMMAND_BARRIER,
    CL_COMMAND_COMMAND_BUFFER_KHR, CL_COMMAND_COPY_BUFFER, CL_COMMAND_COPY_BUFFER_RECT,
    CL_COMMAND_COPY_BUFFER_TO_IMAGE, CL_COMMAND_COPY_IMAGE, CL_COMMAND_COPY_IMAGE_TO_BUFFER,
    CL_COMMAND_FILL_BUFFER, CL_COMMAND_FILL_IMAGE, CL_COMMAND_MAP_BUFFER, CL_COMMAND_MAP_IMAGE,
    CL_COMMAND_MARKER, CL_COMMAND_MEMADVISE_INTEL, CL_COMMAND_MEMCPY_INTEL,
    CL_COMMAND_MEMFILL_INTEL, CL_COMMAND_MIGRATEMEM_INTEL, CL_COMMAND_MIGRATE_MEM_OBJECTS,
    CL_COMMAND_NATIVE_KERNEL, CL_COMMAND_NDRANGE_KERNEL, CL_COMMAND_READ_BUFFER,
    CL_COMMAND_READ_BUFFER_RECT, CL_COMMAND_READ_IMAGE, CL_COMMAND_RELEASE_GL_OBJECTS,
    CL_COMMAND_SVM_FREE, CL_COMMAND_SVM_MAP, CL_COMMAND_SVM_MEMCPY, CL_COMMAND_SVM_MEMFILL,
    CL_COMMAND_SVM_MIGRATE_MEM, CL_COMMAND_SVM_UNMAP, CL_COMMAND_TASK, CL_COMMAND_UNMAP_MEM_OBJECT,
    CL_COMMAND_USER, CL_COMMAND_WRITE_BUFFER, CL_COMMAND_WRITE_BUFFER_RECT, CL_COMMAND_WRITE_IMAGE,
    CL_COMPLETE, CL_EVENT_COMMAND_EXECUTION_STATUS, CL_EVENT_COMMAND_QUEUE, CL_EVENT_COMMAND_TYPE,
    CL_EVENT_CONTEXT, CL_EVENT_REFERENCE_COUNT, CL_INVALID_VALUE, CL_PROFILING_COMMAND_COMPLETE,
    CL_PROFILING_COMMAND_END, CL_PROFILING_COMMAND_QUEUED, CL_PROFILING_COMMAND_START,
    CL_PROFILING_COMMAND_SUBMIT, CL_QUEUED, CL_RUNNING, CL_SUBMITTED, CL_SUCCESS,
};

pub use opencl_sys::cl_egl::{
    CL_COMMAND_ACQUIRE_EGL_OBJECTS_KHR, CL_COMMAND_EGL_FENCE_SYNC_OBJECT_KHR,
    CL_COMMAND_RELEASE_EGL_OBJECTS_KHR,
};

use opencl_sys::{
    clCreateUserEvent, clGetEventInfo, clGetEventProfilingInfo, clReleaseEvent, clRetainEvent,
    clSetEventCallback, clSetUserEventStatus, clWaitForEvents,
};

use super::info_type::InfoType;
use super::{api_info_size, api_info_value, api_info_vector};
use libc::{c_void, intptr_t, size_t};
use std::fmt;
use std::mem;
use std::ptr;

/// Wait for `OpenCL` events to complete.  
/// Calls `clWaitForEvents`.
///
/// * `events` - a slice of `OpenCL` events.
///
/// returns an empty Result or the error code from the `OpenCL` C API function.
#[inline]
#[allow(clippy::cast_possible_truncation)]
pub fn wait_for_events(events: &[cl_event]) -> Result<(), cl_int> {
    let status: cl_int = unsafe { clWaitForEvents(events.len() as cl_uint, events.as_ptr()) };
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}

/// Get data about an `OpenCL` event.
/// Calls `clGetEventInfo` to get the desired data about the event.
pub fn get_event_data(event: cl_event, param_name: cl_event_info) -> Result<Vec<u8>, cl_int> {
    api_info_size!(get_size, clGetEventInfo);
    let size = get_size(event, param_name)?;
    api_info_vector!(get_vector, u8, clGetEventInfo);
    get_vector(event, param_name, size)
}

/// Get specific information about an `OpenCL` event.  
/// Calls `clGetEventInfo` to get the desired information about the event.
///
/// * `event` - the `OpenCL` event.
/// * `param_name` - the type of program information being queried, see:
/// [Event Object Queries](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#event-info-table).
///
/// returns a Result containing the desired information in an `InfoType` enum
/// or the error code from the `OpenCL` C API function.
pub fn get_event_info(event: cl_event, param_name: cl_event_info) -> Result<InfoType, cl_int> {
    match param_name {
        CL_EVENT_COMMAND_EXECUTION_STATUS => {
            api_info_value!(get_value, cl_int, clGetEventInfo);
            Ok(InfoType::Int(get_value(event, param_name)?))
        }

        CL_EVENT_COMMAND_TYPE | CL_EVENT_REFERENCE_COUNT => {
            api_info_value!(get_value, cl_uint, clGetEventInfo);
            Ok(InfoType::Uint(get_value(event, param_name)?))
        }

        CL_EVENT_COMMAND_QUEUE | CL_EVENT_CONTEXT => {
            api_info_value!(get_value, intptr_t, clGetEventInfo);
            Ok(InfoType::Ptr(get_value(event, param_name)?))
        }

        _ => Ok(InfoType::VecUchar(get_event_data(event, param_name)?)),
    }
}

/// Create an `OpenCL` user event object.  
/// Calls `clCreateUserEvent` to create an `OpenCL` event.  
///
/// * `context` - a valid `OpenCL` context.
///
/// returns a Result containing the new `OpenCL` event object
/// or the error code from the `OpenCL` C API function.
#[inline]
pub fn create_user_event(context: cl_context) -> Result<cl_event, cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    let event: cl_event = unsafe { clCreateUserEvent(context, &mut status) };
    if CL_SUCCESS == status {
        Ok(event)
    } else {
        Err(status)
    }
}

/// Retain an `OpenCL` event.  
/// Calls clRetainEvent to increment the event reference count.
///
/// * `event` - the `OpenCL` event.
///
/// returns an empty Result or the error code from the `OpenCL` C API function.
///
/// # Safety
///
/// This function is unsafe because it changes the `OpenCL` object reference count.
#[inline]
pub unsafe fn retain_event(event: cl_event) -> Result<(), cl_int> {
    let status: cl_int = clRetainEvent(event);
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}

/// Release an `OpenCL` event.  
/// Calls `clReleaseEvent` to decrement the event reference count.
///
/// * `event` - the `OpenCL` event.
///
/// returns an empty Result or the error code from the `OpenCL` C API function.
///
/// # Safety
///
/// This function is unsafe because it changes the `OpenCL` object reference count.
#[inline]
pub unsafe fn release_event(event: cl_event) -> Result<(), cl_int> {
    let status: cl_int = clReleaseEvent(event);
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}

/// Set the execution status of a user event object.  
/// Calls `clSetUserEventStatus` to set the execution status.
///
/// * `event` - the `OpenCL` event.
/// * `execution_status` - the `OpenCL` `execution_status`.
///
/// returns an empty Result or the error code from the `OpenCL` C API function.
#[inline]
pub fn set_user_event_status(event: cl_event, execution_status: cl_int) -> Result<(), cl_int> {
    let status: cl_int = unsafe { clSetUserEventStatus(event, execution_status) };
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}

/// Register a user callback function for a specific command execution status,
/// Calls `clSetEventCallback` to register a callback function.  
///
/// * `event` - the `OpenCL` event.
/// * `pfn_notify` - function pointer to the callback function.
/// * `user_data` - passed as an argument when `pfn_notify` is called, or `ptr::null_mut()`.
///
/// returns an empty Result or the error code from the `OpenCL` C API function.
#[inline]
pub fn set_event_callback(
    event: cl_event,
    command_exec_callback_type: cl_int,
    pfn_notify: extern "C" fn(cl_event, cl_int, *mut c_void),
    user_data: *mut c_void,
) -> Result<(), cl_int> {
    let status: cl_int = unsafe {
        clSetEventCallback(
            event,
            command_exec_callback_type,
            Some(pfn_notify),
            user_data,
        )
    };
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}

/// Get profiling data about an `OpenCL` event.
/// Calls `clGetEventProfilingInfo` to get the desired profiling data about the event.
pub fn get_event_profiling_data(
    event: cl_event,
    param_name: cl_profiling_info,
) -> Result<Vec<u8>, cl_int> {
    api_info_size!(get_size, clGetEventProfilingInfo);
    let size = get_size(event, param_name)?;
    api_info_vector!(get_vector, u8, clGetEventProfilingInfo);
    get_vector(event, param_name, size)
}

/// Get profiling information for a command associated with an event when
/// profiling is enabled.  
/// Calls clGetEventProfilingInfo to get the desired information.
///
/// * `event` - the `OpenCL` event.
/// * `param_name` - the type of event profiling information being queried, see:
/// [Event Profiling Queries](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#event-profiling-info-table).
///
/// returns a Result containing the desired information in an `InfoType` enum
/// or the error code from the `OpenCL` C API function.
pub fn get_event_profiling_info(
    event: cl_event,
    param_name: cl_profiling_info,
) -> Result<InfoType, cl_int> {
    match param_name {
        CL_PROFILING_COMMAND_QUEUED
        | CL_PROFILING_COMMAND_SUBMIT
        | CL_PROFILING_COMMAND_START
        | CL_PROFILING_COMMAND_END
        | CL_PROFILING_COMMAND_COMPLETE // CL_VERSION_2_0
         => {
            api_info_value!(get_value, cl_ulong, clGetEventProfilingInfo);
            Ok(InfoType::Ulong(get_value(event, param_name)?))
        }

        _ => Ok(InfoType::VecUchar(get_event_profiling_data(event, param_name)?))
    }
}

#[must_use]
pub const fn status_text(status: cl_int) -> &'static str {
    match status {
        CL_COMPLETE => "CL_COMPLETE",
        CL_RUNNING => "CL_RUNNING",
        CL_SUBMITTED => "CL_SUBMITTED",
        CL_QUEUED => "CL_QUEUED",
        _ => "UNKNOWN_STATUS",
    }
}

#[derive(Debug)]
/// `CommandExecutionStatus` is a newtype around the `OpenCL` command execution status
pub struct CommandExecutionStatus(pub cl_int);

/// Implement the From trait
impl From<cl_int> for CommandExecutionStatus {
    fn from(status: cl_int) -> Self {
        Self(status)
    }
}

/// Implement the Display trait
impl fmt::Display for CommandExecutionStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", status_text(self.0))
    }
}

#[must_use]
pub const fn command_type_text(command_type: cl_command_type) -> &'static str {
    match command_type {
        CL_COMMAND_NDRANGE_KERNEL => "CL_COMMAND_NDRANGE_KERNEL",
        CL_COMMAND_TASK => "CL_COMMAND_TASK",
        CL_COMMAND_NATIVE_KERNEL => "CL_COMMAND_NATIVE_KERNEL",
        CL_COMMAND_READ_BUFFER => "CL_COMMAND_READ_BUFFER",
        CL_COMMAND_WRITE_BUFFER => "CL_COMMAND_WRITE_BUFFER",
        CL_COMMAND_COPY_BUFFER => "CL_COMMAND_COPY_BUFFER",
        CL_COMMAND_READ_IMAGE => "CL_COMMAND_READ_IMAGE",
        CL_COMMAND_WRITE_IMAGE => "CL_COMMAND_WRITE_IMAGE",
        CL_COMMAND_COPY_IMAGE => "CL_COMMAND_COPY_IMAGE",
        CL_COMMAND_COPY_IMAGE_TO_BUFFER => "CL_COMMAND_COPY_IMAGE_TO_BUFFER",
        CL_COMMAND_COPY_BUFFER_TO_IMAGE => "CL_COMMAND_COPY_BUFFER_TO_IMAGE",
        CL_COMMAND_MAP_BUFFER => "CL_COMMAND_MAP_BUFFER",
        CL_COMMAND_MAP_IMAGE => "CL_COMMAND_MAP_IMAGE",
        CL_COMMAND_UNMAP_MEM_OBJECT => "CL_COMMAND_UNMAP_MEM_OBJECT",
        CL_COMMAND_MARKER => "CL_COMMAND_MARKER",
        CL_COMMAND_ACQUIRE_GL_OBJECTS => "CL_COMMAND_ACQUIRE_GL_OBJECTS",
        CL_COMMAND_RELEASE_GL_OBJECTS => "CL_COMMAND_RELEASE_GL_OBJECTS",
        CL_COMMAND_READ_BUFFER_RECT => "CL_COMMAND_READ_BUFFER_RECT",
        CL_COMMAND_WRITE_BUFFER_RECT => "CL_COMMAND_WRITE_BUFFER_RECT",
        CL_COMMAND_COPY_BUFFER_RECT => "CL_COMMAND_COPY_BUFFER_RECT",
        CL_COMMAND_USER => "CL_COMMAND_USER",
        CL_COMMAND_BARRIER => "CL_COMMAND_BARRIER",
        CL_COMMAND_MIGRATE_MEM_OBJECTS => "CL_COMMAND_MIGRATE_MEM_OBJECTS",
        CL_COMMAND_FILL_BUFFER => "CL_COMMAND_FILL_BUFFER",
        CL_COMMAND_FILL_IMAGE => "CL_COMMAND_FILL_IMAGE",
        CL_COMMAND_SVM_FREE => "CL_COMMAND_SVM_FREE",
        CL_COMMAND_SVM_MEMCPY => "CL_COMMAND_SVM_MEMCPY",
        CL_COMMAND_SVM_MEMFILL => "CL_COMMAND_SVM_MEMFILL",
        CL_COMMAND_SVM_MAP => "CL_COMMAND_SVM_MAP",
        CL_COMMAND_SVM_UNMAP => "CL_COMMAND_SVM_UNMAP",
        CL_COMMAND_SVM_MIGRATE_MEM => "CL_COMMAND_SVM_MIGRATE_MEM",

        // cl_egl values
        CL_COMMAND_ACQUIRE_EGL_OBJECTS_KHR => "CL_COMMAND_ACQUIRE_EGL_OBJECTS_KHR",
        CL_COMMAND_RELEASE_EGL_OBJECTS_KHR => "CL_COMMAND_RELEASE_EGL_OBJECTS_KHR",
        CL_COMMAND_EGL_FENCE_SYNC_OBJECT_KHR => "CL_COMMAND_EGL_FENCE_SYNC_OBJECT_KHR",

        // cl_ext values
        CL_COMMAND_MEMFILL_INTEL => "CL_COMMAND_MEMFILL_INTEL",
        CL_COMMAND_MEMCPY_INTEL => "CL_COMMAND_MEMCPY_INTEL",
        CL_COMMAND_MIGRATEMEM_INTEL => "CL_COMMAND_MIGRATEMEM_INTEL",
        CL_COMMAND_MEMADVISE_INTEL => "CL_COMMAND_MEMADVISE_INTEL",

        // cl_khr_command_buffer
        CL_COMMAND_COMMAND_BUFFER_KHR => "CL_COMMAND_COMMAND_BUFFER_KHR",

        _ => "UNKNOWN_COMMAND_TYPE",
    }
}

#[derive(Debug)]
/// `EventCommandType` is a newtype around the `OpenCL` `cl_command_type`
pub struct EventCommandType(pub cl_command_type);

/// Implement the From trait for `EventCommandType`
impl From<cl_command_type> for EventCommandType {
    fn from(command_type: cl_command_type) -> Self {
        Self(command_type)
    }
}

/// Implement the Display trait for `EventCommandType`
impl fmt::Display for EventCommandType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", command_type_text(self.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_text() {
        let text = status_text(CL_COMPLETE);
        assert_eq!("CL_COMPLETE", text);

        let text = status_text(CL_RUNNING);
        assert_eq!("CL_RUNNING", text);

        let text = status_text(CL_SUBMITTED);
        assert_eq!("CL_SUBMITTED", text);

        let text = status_text(CL_QUEUED);
        assert_eq!("CL_QUEUED", text);

        let text = status_text(CL_QUEUED + 1);
        assert_eq!("UNKNOWN_STATUS", text);
    }

    #[test]
    fn test_command_type_text() {
        let text = command_type_text(CL_COMMAND_NDRANGE_KERNEL);
        assert_eq!("CL_COMMAND_NDRANGE_KERNEL", text);

        let text = command_type_text(CL_COMMAND_COPY_IMAGE);
        assert_eq!("CL_COMMAND_COPY_IMAGE", text);

        let text = command_type_text(CL_COMMAND_READ_BUFFER_RECT);
        assert_eq!("CL_COMMAND_READ_BUFFER_RECT", text);

        let text = command_type_text(CL_COMMAND_BARRIER);
        assert_eq!("CL_COMMAND_BARRIER", text);

        let text = command_type_text(CL_COMMAND_SVM_FREE);
        assert_eq!("CL_COMMAND_SVM_FREE", text);

        let text = command_type_text(CL_COMMAND_SVM_MIGRATE_MEM + 1);
        assert_eq!("UNKNOWN_COMMAND_TYPE", text);
    }
}
