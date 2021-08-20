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

//! OpenCL OpenGl Interoperability API.

#![allow(non_camel_case_types)]

pub use cl_sys::{
    cl_command_queue, cl_context_properties, cl_event, cl_gl_context_info, cl_gl_object_type,
    cl_gl_platform_info, cl_gl_texture_info, CL_CGL_SHAREGROUP_KHR, CL_EGL_DISPLAY_KHR,
    CL_GLX_DISPLAY_KHR, CL_GL_CONTEXT_KHR, CL_GL_OBJECT_BUFFER, CL_GL_OBJECT_RENDERBUFFER,
    CL_GL_OBJECT_TEXTURE1D, CL_GL_OBJECT_TEXTURE1D_ARRAY, CL_GL_OBJECT_TEXTURE2D,
    CL_GL_OBJECT_TEXTURE2D_ARRAY, CL_GL_OBJECT_TEXTURE3D, CL_GL_OBJECT_TEXTURE_BUFFER,
    CL_KHR_GL_SHARING, CL_WGL_HDC_KHR,
};

use super::error_codes::{CL_INVALID_VALUE, CL_SUCCESS};
use super::info_type::InfoType;
#[allow(unused_imports)]
use super::types::{cl_context, cl_int, cl_mem, cl_mem_flags, cl_uint};

#[allow(unused_imports)]
use cl_sys::{
    clCreateFromGLBuffer, clCreateFromGLRenderbuffer, clCreateFromGLTexture,
    clCreateFromGLTexture2D, clCreateFromGLTexture3D, clEnqueueAcquireGLObjects,
    clEnqueueReleaseGLObjects, clGetGLContextInfoKHR, clGetGLObjectInfo, clGetGLTextureInfo,
};

use super::{api_info_size, api_info_value, api_info_vector};

#[allow(unused_imports)]
use libc::{c_void, intptr_t, size_t};
use std::mem;
use std::ptr;

pub type gl_uint = cl_uint;
pub type gl_int = cl_int;
pub type gl_enum = cl_uint;
pub type gl_sizei = gl_int;
pub type gl_sync = *mut c_void;

pub const CL_COMMAND_GL_FENCE_SYNC_OBJECT_KHR: cl_uint = 0x200D;

// clCreateEventFromGLsyncKHR is not in cl_sys
#[cfg_attr(not(target_os = "macos"), link(name = "OpenCL"))]
#[cfg_attr(target_os = "macos", link(name = "OpenCL", kind = "framework"))]
extern "system" {
    pub fn clCreateEventFromGLsyncKHR(
        context: cl_context,
        sync: gl_sync,
        errcode_ret: *mut cl_int,
    ) -> cl_event;
}

/// Create an OpenCL buffer object for a context from an OpenGL buffer.  
/// Calls clCreateFromGLBuffer to create an OpenCL buffer object.  
///
/// * `context` - a valid OpenCL context created from an OpenGL context.
/// * `flags` - a bit-field used to specify allocation and usage information
/// about the image memory object being created, see:
/// [Memory Flags](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#memory-flags-table).
/// * `bufobj` - the OpenGL buffer.  
///
/// returns a Result containing the new OpenCL buffer object
/// or the error code from the OpenCL C API function.
#[inline]
pub fn create_from_gl_buffer(
    context: cl_context,
    flags: cl_mem_flags,
    bufobj: gl_uint,
) -> Result<cl_mem, cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    let mem = unsafe { clCreateFromGLBuffer(context, flags, bufobj, &mut status) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(mem)
    }
}

/// Create an OpenCL image object, image array object, or image buffer object
/// for a context from an OpenGL texture object, texture array object,
/// texture buffer object, or a single face of an OpenGL cubemap texture object.  
/// Calls clCreateFromGLTexture to create an OpenCL memory object.  
///
/// * `context` - a valid OpenCL context created from an OpenGL context.
/// * `flags` - a bit-field used to specify allocation and usage information
/// about the image memory object being created, see:
/// [Memory Flags](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#memory-flags-table).
/// * `texture_target` - used to define the image type of texture.  
/// * `miplevel ` - used to define the mipmap level.  
/// * `texture  ` - the name of a GL buffer texture object.  
///
/// returns a Result containing the new OpenCL image object
/// or the error code from the OpenCL C API function.
#[inline]
pub fn create_from_gl_texture(
    context: cl_context,
    flags: cl_mem_flags,
    texture_target: gl_enum,
    miplevel: gl_int,
    texture: gl_uint,
) -> Result<cl_mem, cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    let mem = unsafe {
        clCreateFromGLTexture(
            context,
            flags,
            texture_target,
            miplevel,
            texture,
            &mut status,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(mem)
    }
}

/// Create an OpenCL 2D image object from an OpenGL renderbuffer object.  
/// Calls clCreateFromGLRenderbuffer to create an OpenCL buffer object.  
///
/// * `context` - a valid OpenCL context created from an OpenGL context.
/// * `flags` - a bit-field used to specify allocation and usage information
/// about the image memory object being created, see:
/// [Memory Flags](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#memory-flags-table).
/// * `renderbuffer`  - a GL renderbuffer object.  
///
/// returns a Result containing the new OpenCL image object
/// or the error code from the OpenCL C API function.
#[inline]
pub fn create_from_gl_render_buffer(
    context: cl_context,
    flags: cl_mem_flags,
    renderbuffer: gl_uint,
) -> Result<cl_mem, cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    let mem = unsafe { clCreateFromGLRenderbuffer(context, flags, renderbuffer, &mut status) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(mem)
    }
}

/// Query an OpenGL object used to create an OpenCL memory object.  
/// Calls clGetGLObjectInfo to get the object type and name.  
///
/// * `memobj` - a valid OpenCL memory object handle.
///
/// returns a Result containing the OpenGL object type and name
/// or the error code from the OpenCL C API function.
#[inline]
pub fn get_gl_object_info(memobj: cl_mem) -> Result<(gl_uint, gl_uint), cl_int> {
    let mut object_type: cl_uint = CL_GL_OBJECT_BUFFER;
    let mut object_name: cl_uint = 0;
    let status = unsafe { clGetGLObjectInfo(memobj, &mut object_type, &mut object_name) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok((object_type, object_name))
    }
}


/// Get data about an OpenGL texture object.
/// Calls clGetGLTextureInfo to get the desired data about the texture object.
pub fn get_gl_texture_data(
    memobj: cl_mem,
    param_name: cl_gl_texture_info,
) -> Result<Vec<u8>, cl_int> {
    api_info_size!(get_size, clGetGLTextureInfo);
    let size = get_size(memobj, param_name)?;
    api_info_vector!(get_vector, u8, clGetGLTextureInfo);
    get_vector(memobj, param_name, size)
}

// cl_gl_texture_info
#[derive(Clone, Copy, Debug)]
pub enum TextureInfo {
    CL_GL_TEXTURE_TARGET = 0x2004,
    CL_GL_MIPMAP_LEVEL = 0x2005,
    CL_GL_NUM_SAMPLES = 0x2012,
}

/// Get information about the GL texture object associated with a memory object.
/// Calls clGetGLTextureInfo to get the desired information.
///
/// * `memobj` - the OpenCL memory object.
/// * `param_name` - the type of memory object information being queried, see:
/// [Texture Info](https://www.khronos.org/registry/OpenCL//sdk/2.2/docs/man/html/clGetGLTextureInfo.html).
///
/// returns a Result containing the desired information in an InfoType enum
/// or the error code from the OpenCL C API function.
pub fn get_gl_texture_info(memobj: cl_mem, param_name: TextureInfo) -> Result<InfoType, cl_int> {
    let param_id = param_name as cl_gl_texture_info;

    match param_name {
        TextureInfo::CL_GL_TEXTURE_TARGET => {
            api_info_value!(get_value, gl_enum, clGetGLTextureInfo);
            Ok(InfoType::Uint(get_value(memobj, param_id)?))
        }
        TextureInfo::CL_GL_MIPMAP_LEVEL | TextureInfo::CL_GL_NUM_SAMPLES => {
            api_info_value!(get_value, gl_int, clGetGLTextureInfo);
            Ok(InfoType::Int(get_value(memobj, param_id)?))
        }
    }
}

/// Acquire OpenCL memory objects that have been created from OpenGL objects.  
/// Calls clEnqueueAcquireGLObjects.  
///
/// * `command_queue` - a valid OpenCL command_queue.
/// * `num_objects` - the number of memory objects to acquire.
/// * `mem_objects` - the memory objects to acquire.
/// * `num_events_in_wait_list` - the number of events in the wait list.
/// * `event_wait_list` - the wait list events.
///
/// returns a Result containing the new OpenCL event
/// or the error code from the OpenCL C API function.
#[inline]
pub fn enqueue_acquire_gl_objects(
    command_queue: cl_command_queue,
    num_objects: cl_uint,
    mem_objects: *const cl_mem,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = unsafe {
        clEnqueueAcquireGLObjects(
            command_queue,
            num_objects,
            mem_objects,
            num_events_in_wait_list,
            event_wait_list,
            &mut event,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(event)
    }
}

/// Release OpenCL memory objects that have been created from OpenGL objects.  
/// Calls clEnqueueReleaseGLObjects.  
///
/// * `command_queue` - a valid OpenCL command_queue.
/// * `num_objects` - the number of memory objects to acquire.
/// * `mem_objects` - the memory objects to acquire.
/// * `num_events_in_wait_list` - the number of events in the wait list.
/// * `event_wait_list` - the wait list events.
///
/// returns a Result containing the new OpenCL event
/// or the error code from the OpenCL C API function.
#[inline]
pub fn enqueue_release_gl_objects(
    command_queue: cl_command_queue,
    num_objects: cl_uint,
    mem_objects: *const cl_mem,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = unsafe {
        clEnqueueReleaseGLObjects(
            command_queue,
            num_objects,
            mem_objects,
            num_events_in_wait_list,
            event_wait_list,
            &mut event,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(event)
    }
}

/// Create an OpenCL 2D image object from an OpenGL 2D texture object,
/// or a single face of an OpenGL cubemap texture object.  
/// Calls clCreateFromGLTexture2D to create an OpenCL memory object.  
/// Deprecated in CL_VERSION_1_2, use create_from_gl_texture.
///
/// * `context` - a valid OpenCL context created from an OpenGL context.
/// * `flags` - a bit-field used to specify allocation and usage information
/// about the image memory object being created, see:
/// [Memory Flags](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#memory-flags-table).
/// * `texture_target` - used to define the image type of texture.  
/// * `miplevel ` - used to define the mipmap level.  
/// * `texture  ` - the name of a GL 2D, cubemap or rectangle texture object.  
///
/// returns a Result containing the new OpenCL image object
/// or the error code from the OpenCL C API function.
#[deprecated(since = "CL_VERSION_1_2", note = "Use create_from_gl_texture instead.")]
#[inline]
pub fn create_from_gl_texture_2d(
    context: cl_context,
    flags: cl_mem_flags,
    texture_target: gl_enum,
    miplevel: gl_int,
    texture: gl_uint,
) -> Result<cl_mem, cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    let mem = unsafe {
        clCreateFromGLTexture2D(
            context,
            flags,
            texture_target,
            miplevel,
            texture,
            &mut status,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(mem)
    }
}

/// Create an OpenCL 3D image object from an OpenGL 3D texture object.  
/// Calls clCreateFromGLTexture3D to create an OpenCL memory object.  
/// Deprecated in CL_VERSION_1_2, use create_from_gl_texture.
///
/// * `context` - a valid OpenCL context created from an OpenGL context.
/// * `flags` - a bit-field used to specify allocation and usage information
/// about the image memory object being created, see:
/// [Memory Flags](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#memory-flags-table).
/// * `texture_target` - used to define the image type of texture.  
/// * `miplevel ` - used to define the mipmap level.  
/// * `texture  ` - the name of a GL 2D, cubemap or rectangle texture object.  
///
/// returns a Result containing the new OpenCL image object
/// or the error code from the OpenCL C API function.
#[deprecated(since = "CL_VERSION_1_2", note = "Use create_from_gl_texture instead.")]
#[inline]
pub fn create_from_gl_texture_3d(
    context: cl_context,
    flags: cl_mem_flags,
    texture_target: gl_enum,
    miplevel: gl_int,
    texture: gl_uint,
) -> Result<cl_mem, cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    let mem = unsafe {
        clCreateFromGLTexture3D(
            context,
            flags,
            texture_target,
            miplevel,
            texture,
            &mut status,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(mem)
    }
}

// cl_gl_context_info
#[derive(Clone, Copy, Debug)]
pub enum GlContextInfo {
    CL_CURRENT_DEVICE_FOR_GL_CONTEXT_KHR = 0x2006,
    CL_DEVICES_FOR_GL_CONTEXT_KHR = 0x2007,
}

/// Get OpenGL context information.
/// Calls clGetGLContextInfoKHR to get the desired information.
///
/// * `properties` - the OpenCL context properties.
/// * `param_name` - the type of memory object information being queried, see:
/// [Context Info](https://www.khronos.org/registry/OpenCL//sdk/2.2/docs/man/html/clGetGLContextInfoKHR.html).
///
/// returns a Result containing the desired information in an InfoType enum
/// or the error code from the OpenCL C API function.
#[cfg(feature = "cl_khr_gl_sharing")]
pub fn get_gl_context_info_khr(
    properties: *mut cl_context_properties,
    param_name: GlContextInfo,
) -> Result<InfoType, cl_int> {
    let param_id = param_name as cl_gl_context_info;

    match param_name {
        GlContextInfo::CL_CURRENT_DEVICE_FOR_GL_CONTEXT_KHR => {
            let mut data: intptr_t = 0;
            let data_ptr: *mut intptr_t = &mut data;
            let status = unsafe {
                clGetGLContextInfoKHR(
                    properties,
                    param_id,
                    mem::size_of::<intptr_t>(),
                    data_ptr as *mut c_void,
                    ptr::null_mut(),
                )
            };
            if CL_SUCCESS != status {
                Err(status)
            } else {
                Ok(InfoType::Ptr(data))
            }
        }

        GlContextInfo::CL_DEVICES_FOR_GL_CONTEXT_KHR => {
            // Get the size
            let mut size: size_t = 0;
            let status = unsafe {
                clGetGLContextInfoKHR(properties, param_id, 0, ptr::null_mut(), &mut size)
            };
            if CL_SUCCESS != status {
                Err(status)
            } else {
                if 0 < size {
                    // Get the data
                    let count = size / mem::size_of::<intptr_t>();
                    let mut data: Vec<intptr_t> = Vec::with_capacity(count);
                    let status = unsafe {
                        clGetGLContextInfoKHR(
                            properties,
                            param_id,
                            size,
                            data.as_mut_ptr() as *mut c_void,
                            ptr::null_mut(),
                        )
                    };
                    if CL_SUCCESS != status {
                        Err(status)
                    } else {
                        Ok(InfoType::VecIntPtr(data))
                    }
                } else {
                    Ok(InfoType::VecIntPtr(Vec::default()))
                }
            }
        }
    }
}

/// Create an event object linked to an OpenGL sync object.  
/// Requires the cl_khr_gl_event extension
/// Calls clCreateEventFromGLsyncKHR.  
///
/// * `context` - a valid OpenCL context.
/// * `sync` - the sync object in the GL share group associated with context.  
///
/// returns a Result containing the new OpenCL event
/// or the error code from the OpenCL C API function.
#[cfg(feature = "cl_khr_gl_event")]
#[inline]
pub fn create_event_from_gl_sync_khr(
    context: cl_context,
    sync: gl_sync,
) -> Result<cl_event, cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    let event: cl_event = unsafe { clCreateEventFromGLsyncKHR(context, sync, &mut status) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(event)
    }
}
