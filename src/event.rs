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

//! OpenCL Event Object API.

use super::error_codes::{CL_INVALID_VALUE, CL_SUCCESS};
use super::ffi::cl::{
    clCreateUserEvent, clGetEventInfo, clGetEventProfilingInfo, clReleaseEvent, clRetainEvent,
    clSetEventCallback, clSetUserEventStatus, clWaitForEvents,
};
use super::info_type::InfoType;
use super::types::{
    cl_context, cl_event, cl_event_info, cl_int, cl_profiling_info, cl_uint, cl_ulong,
};

use super::api_info_value;

use libc::{c_void, intptr_t, size_t};
use std::mem;
use std::ptr;

/// Wait for OpenCL events to complete.  
/// Calls clWaitForEvents.
///
/// * `events` - a slice of OpenCL events.
///
/// returns an empty Result or the error code from the OpenCL C API function.
pub fn wait_for_events(events: &[cl_event]) -> Result<(), cl_int> {
    let status: cl_int = unsafe { clWaitForEvents(events.len() as cl_uint, events.as_ptr()) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(())
    }
}

// cl_event_info:
pub const CL_EVENT_COMMAND_QUEUE: cl_event_info = 0x11D0;
pub const CL_EVENT_COMMAND_TYPE: cl_event_info = 0x11D1;
pub const CL_EVENT_REFERENCE_COUNT: cl_event_info = 0x11D2;
pub const CL_EVENT_COMMAND_EXECUTION_STATUS: cl_event_info = 0x11D3;
// #ifdef CL_VERSION_1_1
pub const CL_EVENT_CONTEXT: cl_event_info = 0x11D4;
// #endif

/// Get specific information about an OpenCL event.  
/// Calls clGetEventInfo to get the desired information about the event.
///
/// * `event` - the OpenCL event.
/// * `param_name` - the type of program information being queried, see:
/// [Event Object Queries](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#event-info-table).
///
/// returns a Result containing the desired information in an InfoType enum
/// or the error code from the OpenCL C API function.
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

        _ => Err(CL_INVALID_VALUE),
    }
}

/// Create an OpenCL user event object.  
/// Calls clCreateUserEvent to create an OpenCL event.  
///
/// * `context` - a valid OpenCL context.
///
/// returns a Result containing the new OpenCL event object
/// or the error code from the OpenCL C API function.
pub fn create_user_event(context: cl_context) -> Result<cl_event, cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    let event: cl_event = unsafe { clCreateUserEvent(context, &mut status) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(event)
    }
}

/// Retain an OpenCL event.  
/// Calls clRetainEvent to increment the event reference count.
///
/// * `event` - the OpenCL event.
///
/// returns an empty Result or the error code from the OpenCL C API function.
pub fn retain_event(event: cl_event) -> Result<(), cl_int> {
    let status: cl_int = unsafe { clRetainEvent(event) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(())
    }
}

/// Release an OpenCL event.  
/// Calls clReleaseEvent to decrement the event reference count.
///
/// * `event` - the OpenCL event.
///
/// returns an empty Result or the error code from the OpenCL C API function.
pub fn release_event(event: cl_event) -> Result<(), cl_int> {
    let status: cl_int = unsafe { clReleaseEvent(event) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(())
    }
}

/// Set the execution status of a user event object.  
/// Calls clSetUserEventStatus to set the execution status.
///
/// * `event` - the OpenCL event.
/// * `execution_status` - the OpenCL execution_status.
///
/// returns an empty Result or the error code from the OpenCL C API function.
pub fn set_user_event_status(event: cl_event, execution_status: cl_int) -> Result<(), cl_int> {
    let status: cl_int = unsafe { clSetUserEventStatus(event, execution_status) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(())
    }
}

/// Register a user callback function for a specific command execution status,
/// Calls clSetEventCallback to register a callback function.  
///
/// * `event` - the OpenCL event.
/// * `pfn_notify` - function pointer to the callback function.
/// * `user_data` - passed as an argument when pfn_notify is called, or ptr::null_mut().
///
/// returns an empty Result or the error code from the OpenCL C API function.
pub fn set_event_callback(
    event: cl_event,
    command_exec_callback_type: cl_int,
    pfn_notify: extern "C" fn(cl_event, cl_int, *mut c_void),
    user_data: *mut c_void,
) -> Result<(), cl_int> {
    let status: cl_int =
        unsafe { clSetEventCallback(event, command_exec_callback_type, pfn_notify, user_data) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(())
    }
}

// cl_profiling_info
pub const CL_PROFILING_COMMAND_QUEUED: cl_profiling_info = 0x1280;
pub const CL_PROFILING_COMMAND_SUBMIT: cl_profiling_info = 0x1281;
pub const CL_PROFILING_COMMAND_START: cl_profiling_info = 0x1282;
pub const CL_PROFILING_COMMAND_END: cl_profiling_info = 0x1283;
// #ifdef CL_VERSION_2_0
pub const CL_PROFILING_COMMAND_COMPLETE: cl_profiling_info = 0x1284;
//#endif

/// Get profiling information for a command associated with an event when
/// profiling is enabled.  
/// Calls clGetEventProfilingInfo to get the desired information.
///
/// * `event` - the OpenCL event.
/// * `param_name` - the type of event profiling information being queried, see:
/// [Event Profiling Queries](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#event-profiling-info-table).
///
/// returns a Result containing the desired information in an InfoType enum
/// or the error code from the OpenCL C API function.
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

        _ => Err(CL_INVALID_VALUE),
    }
}
