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

//! OpenCL Memory Object API.

#![allow(non_camel_case_types)]

use super::error_codes::{CL_INVALID_VALUE, CL_SUCCESS};
use super::info_type::InfoType;
#[allow(unused_imports)]
use super::types::{
    cl_buffer_create_type, cl_context, cl_image_desc, cl_image_format, cl_image_info, cl_int,
    cl_map_flags, cl_mem, cl_mem_flags, cl_mem_info, cl_mem_object_type, cl_mem_properties,
    cl_pipe_info, cl_svm_mem_flags, cl_uint, cl_ulong,
};
#[allow(unused_imports)]
use cl_sys::{
    clCreateBuffer, clCreatePipe, clCreateSubBuffer, clGetImageInfo, clGetMemObjectInfo,
    clGetPipeInfo, clReleaseMemObject, clRetainMemObject, clSVMAlloc, clSVMFree,
    clSetMemObjectDestructorCallback,
};

use super::{api_info_size, api_info_value, api_info_vector};

use libc::{c_void, intptr_t, size_t};
use std::mem;
use std::ptr;

// clGetSupportedImageFormats and clCreateImage because cl_image_format does not
// derive the Debug trait.
// clCreateBufferWithProperties, clCreateImageWithProperties are CL_VERSION_3_0
#[cfg_attr(not(target_os = "macos"), link(name = "OpenCL"))]
#[cfg_attr(target_os = "macos", link(name = "OpenCL", kind = "framework"))]
extern "system" {
    pub fn clGetSupportedImageFormats(
        context: cl_context,
        flags: cl_mem_flags,
        image_type: cl_mem_object_type,
        num_entries: cl_uint,
        image_formats: *mut cl_image_format,
        num_image_formats: *mut cl_uint,
    ) -> cl_int;

    pub fn clCreateImage(
        context: cl_context,
        flags: cl_mem_flags,
        image_format: *const cl_image_format,
        image_desc: *const cl_image_desc,
        host_ptr: *mut c_void,
        errcode_ret: *mut cl_int,
    ) -> cl_mem;

    // #ifdef CL_VERSION_3_0
    pub fn clCreateBufferWithProperties(
        context: cl_context,
        properties: *const cl_mem_properties,
        flags: cl_mem_flags,
        size: size_t,
        host_ptr: *mut c_void,
        errcode_ret: *mut cl_int,
    ) -> cl_mem;

    pub fn clCreateImageWithProperties(
        context: cl_context,
        properties: *const cl_mem_properties,
        flags: cl_mem_flags,
        image_format: *const cl_image_format,
        image_desc: *const cl_image_desc,
        host_ptr: *mut c_void,
        errcode_ret: *mut cl_int,
    ) -> cl_mem;
// #endif
}

// cl_mem_flags and cl_svm_mem_flags - bitfield:
pub const CL_MEM_READ_WRITE: cl_mem_flags = 1 << 0;
pub const CL_MEM_WRITE_ONLY: cl_mem_flags = 1 << 1;
pub const CL_MEM_READ_ONLY: cl_mem_flags = 1 << 2;
pub const CL_MEM_USE_HOST_PTR: cl_mem_flags = 1 << 3;
pub const CL_MEM_ALLOC_HOST_PTR: cl_mem_flags = 1 << 4;
pub const CL_MEM_COPY_HOST_PTR: cl_mem_flags = 1 << 5;
// reserved                      cl_mem_flags = 1 << 6;
pub const CL_MEM_HOST_WRITE_ONLY: cl_mem_flags = 1 << 7;
pub const CL_MEM_HOST_READ_ONLY: cl_mem_flags = 1 << 8;
pub const CL_MEM_HOST_NO_ACCESS: cl_mem_flags = 1 << 9;
// #ifdef CL_VERSION_2_0
pub const CL_MEM_SVM_FINE_GRAIN_BUFFER: cl_svm_mem_flags = 1 << 10; // used by cl_svm_mem_flags only
pub const CL_MEM_SVM_ATOMICS: cl_svm_mem_flags = 1 << 11; // used by cl_svm_mem_flags only
pub const CL_MEM_KERNEL_READ_AND_WRITE: cl_mem_flags = 1 << 12;
// #endif

pub const CL_MAP_READ: cl_map_flags = 1 << 0;
pub const CL_MAP_WRITE: cl_map_flags = 1 << 1;
pub const CL_MAP_WRITE_INVALIDATE_REGION: cl_map_flags = 1 << 2;

/// Create an OpenCL buffer object for a context.  
/// Calls clCreateBuffer to create an OpenCL buffer object.  
///
/// * `context` - a valid OpenCL context.
/// * `flags` - a bit-field used to specify allocation and usage information
/// about the image memory object being created, see:
/// [Memory Flags](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#memory-flags-table).
/// * `size` - the size in bytes of the buffer memory object to be allocated.
/// * `host_ptr` - a pointer to the buffer data that may already be allocated
/// by the application.
///
/// returns a Result containing the new OpenCL buffer object
/// or the error code from the OpenCL C API function.
pub fn create_buffer(
    context: cl_context,
    flags: cl_mem_flags,
    size: size_t,
    host_ptr: *mut c_void,
) -> Result<cl_mem, cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    let mem: cl_mem = unsafe { clCreateBuffer(context, flags, size, host_ptr, &mut status) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(mem)
    }
}

/// Create an new OpenCL buffer object from an existing buffer object.  
/// Calls clCreateSubBuffer to create an OpenCL sub-buffer object.  
///
/// * `buffer` - a valid OpenCL buffer.
/// * `flags` - a bit-field used to specify allocation and usage information
/// about the sub-buffer memory object being created, see:
/// [Memory Flags](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#memory-flags-table).
/// * `buffer_create_type`,`buffer_create_info` - describe the type of
/// buffer object to be created, see:
/// [SubBuffer Attributes](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#subbuffer-create-info-table).
///
/// returns a Result containing the new OpenCL buffer object
/// or the error code from the OpenCL C API function.
pub fn create_sub_buffer(
    buffer: cl_mem,
    flags: cl_mem_flags,
    buffer_create_type: cl_buffer_create_type,
    buffer_create_info: *const c_void,
) -> Result<cl_mem, cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    let mem: cl_mem = unsafe {
        clCreateSubBuffer(
            buffer,
            flags,
            buffer_create_type,
            buffer_create_info,
            &mut status,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(mem)
    }
}

/// Create an OpenCL image object for a context.  
/// Calls clCreateImage to create an OpenCL image object.  
///
/// * `context` - a valid OpenCL context.
/// * `flags` - a bit-field used to specify allocation and usage information
/// about the image memory object being created, see:
/// [Memory Flags](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#memory-flags-table).
/// * `image_format` - a pointer to a structure that describes format properties
/// of the image to be allocated.
/// * `image_desc` - a pointer to a structure that describes type and dimensions
/// of the image to be allocated.
/// * `host_ptr` - a pointer to the image data that may already be allocated
/// by the application.
///
/// returns a Result containing the new OpenCL image object
/// or the error code from the OpenCL C API function.
pub fn create_image(
    context: cl_context,
    flags: cl_mem_flags,
    image_format: *const cl_image_format,
    image_desc: *const cl_image_desc,
    host_ptr: *mut c_void,
) -> Result<cl_mem, cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    let mem: cl_mem = unsafe {
        clCreateImage(
            context,
            flags,
            image_format,
            image_desc,
            host_ptr,
            &mut status,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(mem)
    }
}

/// Create an OpenCL pipe object for a context.  
/// Calls clCreatePipe to create an OpenCL pipe object.  
/// CL_VERSION_2_0
///
/// * `context` - a valid OpenCL context.
/// * `flags` - a bit-field used to specify allocation and usage information
/// about the image memory object being created, see:
/// [Memory Flags](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#memory-flags-table).
/// * `pipe_packet_size` - the size in bytes of a pipe packet.
/// * `pipe_max_packets` -the maximum number of packets the pipe can hold.
/// * `properties` - currently must be NULL.
///
/// returns a Result containing the new OpenCL pipe object
/// or the error code from the OpenCL C API function.
pub fn create_pipe(
    context: cl_context,
    flags: cl_mem_flags,
    pipe_packet_size: cl_uint,
    pipe_max_packets: cl_uint,
    // properties: *const cl_pipe_properties,
) -> Result<cl_mem, cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    let mem: cl_mem = unsafe {
        clCreatePipe(
            context,
            flags,
            pipe_packet_size,
            pipe_max_packets,
            ptr::null(),
            &mut status,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(mem)
    }
}

/// Create an OpenCL buffer object for a context.  
/// Calls clCreateBufferWithProperties to create an OpenCL buffer object.  
/// CL_VERSION_3_0
///
/// * `context` - a valid OpenCL context.
/// * `properties` - an optional null terminated list of properties.
/// * `flags` - a bit-field used to specify allocation and usage information
/// about the image memory object being created, see:
/// [Memory Flags](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#memory-flags-table).
/// * `size` - the size in bytes of the buffer memory object to be allocated.
/// * `host_ptr` - a pointer to the buffer data that may already be allocated
/// by the application.
///
/// returns a Result containing the new OpenCL buffer object
/// or the error code from the OpenCL C API function.
#[cfg(feature = "CL_VERSION_3_0")]
pub fn create_buffer_with_properties(
    context: cl_context,
    properties: *const cl_mem_properties,
    flags: cl_mem_flags,
    size: size_t,
    host_ptr: *mut c_void,
) -> Result<cl_mem, cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    let mem: cl_mem = unsafe {
        clCreateBufferWithProperties(context, properties, flags, size, host_ptr, &mut status)
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(mem)
    }
}

/// Create an OpenCL image object for a context.  
/// Calls clCreateImage to create an OpenCL image object.  
/// CL_VERSION_3_0
///
/// * `context` - a valid OpenCL context.
/// * `properties` - an optional null terminated list of properties.
/// * `flags` - a bit-field used to specify allocation and usage information
/// about the image memory object being created, see:
/// [Memory Flags](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#memory-flags-table).
/// * `image_format` - a pointer to a structure that describes format properties
/// of the image to be allocated.
/// * `image_desc` - a pointer to a structure that describes type and dimensions
/// of the image to be allocated.
/// * `host_ptr` - a pointer to the image data that may already be allocated
/// by the application.
///
/// returns a Result containing the new OpenCL image object
/// or the error code from the OpenCL C API function.
#[cfg(feature = "CL_VERSION_3_0")]
pub fn create_image_with_properties(
    context: cl_context,
    properties: *const cl_mem_properties,
    flags: cl_mem_flags,
    image_format: *const cl_image_format,
    image_desc: *const cl_image_desc,
    host_ptr: *mut c_void,
) -> Result<cl_mem, cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    let mem: cl_mem = unsafe {
        clCreateImageWithProperties(
            context,
            properties,
            flags,
            image_format,
            image_desc,
            host_ptr,
            &mut status,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(mem)
    }
}

/// Retain an OpenCL memory object.  
/// Calls clRetainMemObject to increment the memory object reference count.
///
/// * `memobj` - the OpenCL memory object.
///
/// returns an empty Result or the error code from the OpenCL C API function.
pub fn retain_mem_object(memobj: cl_mem) -> Result<(), cl_int> {
    let status: cl_int = unsafe { clRetainMemObject(memobj) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(())
    }
}

/// Release an OpenCL memory object.  
/// Calls clReleaseMemObject to decrement the memory object reference count.
///
/// * `memobj` - the OpenCL memory object.
///
/// returns an empty Result or the error code from the OpenCL C API function.
pub fn release_mem_object(memobj: cl_mem) -> Result<(), cl_int> {
    let status: cl_int = unsafe { clReleaseMemObject(memobj) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(())
    }
}

fn count_supported_image_formats(
    context: cl_context,
    flags: cl_mem_flags,
    image_type: cl_mem_object_type,
) -> Result<cl_uint, cl_int> {
    let mut count: cl_uint = 0;
    let status: cl_int = unsafe {
        clGetSupportedImageFormats(context, flags, image_type, 0, ptr::null_mut(), &mut count)
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(count)
    }
}

/// Get the list of image formats supported by an OpenCL implementation for a
/// specified context, image type, and allocation information.  
/// Calls clGetSupportedImageFormats to get the desired information about the program.
///
/// * `context` - a valid OpenCL context on which the image object(s) will be created.
/// * `flags` - a bit-field used to specify allocation and usage information
/// about the image memory object being created, see:
/// [Memory Flags](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#memory-flags-table).
/// * `image_type` - describes the image type.
///
/// returns a Result containing the desired information in an InfoType enum
/// or the error code from the OpenCL C API function.
pub fn get_supported_image_formats(
    context: cl_context,
    flags: cl_mem_flags,
    image_type: cl_mem_object_type,
) -> Result<Vec<cl_image_format>, cl_int> {
    let count: cl_uint = count_supported_image_formats(context, flags, image_type)?;
    let mut image_formats: Vec<cl_image_format> = Vec::with_capacity(count as usize);
    let status: cl_int = unsafe {
        image_formats.set_len(count as usize);
        clGetSupportedImageFormats(
            context,
            flags,
            image_type,
            count,
            image_formats.as_mut_ptr() as *mut cl_image_format,
            ptr::null_mut(),
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(image_formats)
    }
}

// cl_mem_info
#[derive(Clone, Copy, Debug)]
pub enum MemInfo {
    CL_MEM_TYPE = 0x1100,
    CL_MEM_FLAGS = 0x1101,
    CL_MEM_SIZE = 0x1102,
    CL_MEM_HOST_PTR = 0x1103,
    CL_MEM_MAP_COUNT = 0x1104,
    CL_MEM_REFERENCE_COUNT = 0x1105,
    CL_MEM_CONTEXT = 0x1106,
    // CL_VERSION_1_1
    CL_MEM_ASSOCIATED_MEMOBJECT = 0x1107,
    CL_MEM_OFFSET = 0x1108,
    // CL_VERSION_2_0
    CL_MEM_USES_SVM_POINTER = 0x1109,
    // CL_VERSION_3_0
    CL_MEM_PROPERTIES = 0x110A,
}

/// Get information common to all OpenCL memory objects (buffer and image objects).  
/// Calls clGetMemObjectInfo to get the desired information about the memory objects.
///
/// * `memobj` - the OpenCL memory objects.
/// * `param_name` - the type of memory object information being queried, see:
/// [Memory Object Info](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#mem-info-table).
///
/// returns a Result containing the desired information in an InfoType enum
/// or the error code from the OpenCL C API function.
pub fn get_mem_object_info(memobj: cl_mem, param_name: MemInfo) -> Result<InfoType, cl_int> {
    let param_id = param_name as cl_mem_info;
    match param_name {
        MemInfo::CL_MEM_TYPE
        | MemInfo::CL_MEM_MAP_COUNT
        | MemInfo::CL_MEM_REFERENCE_COUNT
        | MemInfo::CL_MEM_USES_SVM_POINTER // CL_VERSION_2_0
        => {
            api_info_value!(get_value, cl_uint, clGetMemObjectInfo);
            Ok(InfoType::Uint(get_value(memobj, param_id)?))
        }

        MemInfo::CL_MEM_FLAGS => {
            api_info_value!(get_value, cl_ulong, clGetMemObjectInfo);
            Ok(InfoType::Ulong(get_value(memobj, param_id)?))
        }

        MemInfo::CL_MEM_SIZE | MemInfo::CL_MEM_OFFSET => {
            api_info_value!(get_value, size_t, clGetMemObjectInfo);
            Ok(InfoType::Size(get_value(memobj, param_id)?))
        }

        MemInfo::CL_MEM_HOST_PTR | MemInfo::CL_MEM_CONTEXT | MemInfo::CL_MEM_ASSOCIATED_MEMOBJECT => {
            api_info_value!(get_value, intptr_t, clGetMemObjectInfo);
            Ok(InfoType::Ptr(get_value(memobj, param_id)?))
        }

        MemInfo::CL_MEM_PROPERTIES // CL_VERSION_3_0
        => {
            api_info_size!(get_size, clGetMemObjectInfo);
            api_info_vector!(get_vec, cl_ulong, clGetMemObjectInfo);
            let size = get_size(memobj, param_id)?;
            Ok(InfoType::VecUlong(get_vec(memobj, param_id, size,)?))
        }
    }
}

// cl_image_info
#[derive(Clone, Copy, Debug)]
pub enum ImageInfo {
    CL_IMAGE_FORMAT = 0x1110,
    CL_IMAGE_ELEMENT_SIZE = 0x1111,
    CL_IMAGE_ROW_PITCH = 0x1112,
    CL_IMAGE_SLICE_PITCH = 0x1113,
    CL_IMAGE_WIDTH = 0x1114,
    CL_IMAGE_HEIGHT = 0x1115,
    CL_IMAGE_DEPTH = 0x1116,
    // CL_VERSION_1_2
    CL_IMAGE_ARRAY_SIZE = 0x1117,
    CL_IMAGE_BUFFER = 0x1118,
    CL_IMAGE_NUM_MIP_LEVELS = 0x1119,
    CL_IMAGE_NUM_SAMPLES = 0x111A,
}

/// Get information specific to an OpenCL image object.  
/// Calls clGetImageInfo to get the desired information about the image object.
///
/// * `image` - the OpenCL image object.
/// * `param_name` - the type of memory object information being queried, see:
/// [Image Object Info](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#image-info-table).
///
/// returns a Result containing the desired information in an InfoType enum
/// or the error code from the OpenCL C API function.
pub fn get_image_info(image: cl_mem, param_name: ImageInfo) -> Result<InfoType, cl_int> {
    let param_id = param_name as cl_image_info;
    match param_name {
        ImageInfo::CL_IMAGE_FORMAT => {
            api_info_size!(get_size, clGetImageInfo);
            api_info_vector!(get_vec, cl_image_format, clGetImageInfo);
            let size = get_size(image, param_id)?;
            Ok(InfoType::VecImageFormat(get_vec(image, param_id, size)?))
        }

        ImageInfo::CL_IMAGE_ELEMENT_SIZE
        | ImageInfo::CL_IMAGE_ROW_PITCH
        | ImageInfo::CL_IMAGE_SLICE_PITCH
        | ImageInfo::CL_IMAGE_WIDTH
        | ImageInfo::CL_IMAGE_HEIGHT
        | ImageInfo::CL_IMAGE_DEPTH
        | ImageInfo::CL_IMAGE_ARRAY_SIZE => {
            api_info_value!(get_value, size_t, clGetImageInfo);
            Ok(InfoType::Size(get_value(image, param_id)?))
        }

        ImageInfo::CL_IMAGE_BUFFER => {
            api_info_value!(get_value, intptr_t, clGetImageInfo);
            Ok(InfoType::Ptr(get_value(image, param_id)?))
        }

        ImageInfo::CL_IMAGE_NUM_MIP_LEVELS | ImageInfo::CL_IMAGE_NUM_SAMPLES => {
            api_info_value!(get_value, cl_uint, clGetImageInfo);
            Ok(InfoType::Uint(get_value(image, param_id)?))
        }
    }
}

// cl_pipe_info
#[derive(Clone, Copy, Debug)]
pub enum PipeInfo {
    // CL_VERSION_2_0
    CL_PIPE_PACKET_SIZE = 0x1120,
    CL_PIPE_MAX_PACKETS = 0x1121,
    // CL_VERSION_3_0
    CL_PIPE_PROPERTIES = 0x1122,
}

/// Get information specific to an OpenCL pipe object.  
/// Calls clGetPipeInfo to get the desired information about the pipe object.
/// CL_VERSION_2_0
///
/// * `pipe` - the OpenCL pipe object.
/// * `param_name` - the type of pipe object information being queried, see:
/// [Pipe Object Queries](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#pipe-info-table).
///
/// returns a Result containing the desired information in an InfoType enum
/// or the error code from the OpenCL C API function.
pub fn get_pipe_info(pipe: cl_mem, param_name: PipeInfo) -> Result<InfoType, cl_int> {
    let param_id = param_name as cl_pipe_info;
    match param_name {
        PipeInfo::CL_PIPE_PACKET_SIZE | PipeInfo::CL_PIPE_MAX_PACKETS => {
            api_info_value!(get_value, cl_uint, clGetPipeInfo);
            Ok(InfoType::Uint(get_value(pipe, param_id)?))
        }
        // CL_VERSION_3_0
        PipeInfo::CL_PIPE_PROPERTIES => {
            api_info_size!(get_size, clGetPipeInfo);
            api_info_vector!(get_vec, intptr_t, clGetPipeInfo);
            let size = get_size(pipe, param_id)?;
            Ok(InfoType::VecIntPtr(get_vec(pipe, param_id, size)?))
        }
    }
}

/// Register a callback function with an OpenCL memory object that is called when the
/// memory object is destroyed.  
/// Calls clSetMemObjectDestructorCallback.  
///
/// * `memobj` - the OpenCL memory object.
/// * `pfn_notify` - callback function to be registered by the application.
/// * `user_data` - passed as the user_data argument when pfn_notify is called.
///
/// returns an empty Result or the error code from the OpenCL C API function.
pub fn set_mem_object_destructor_callback(
    memobj: cl_mem,
    pfn_notify: extern "C" fn(cl_mem, *mut c_void),
    user_data: *mut c_void,
) -> Result<(), cl_int> {
    let status: cl_int =
        unsafe { clSetMemObjectDestructorCallback(memobj, Some(pfn_notify), user_data) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(())
    }
}

/// Allocate a shared virtual memory (SVM) buffer that can be shared by the
/// host and all devices in an OpenCL context.  
/// Calls clSVMAlloc.  
/// CL_VERSION_2_0
///
/// * `context` - a valid OpenCL context.
/// * `flags` - a bit-field used to specify allocation and usage information, see:
/// [SVM Memory Flags](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#svm-flags-table).
/// * `size` - the size in bytes of the SVM buffer to be allocated.
/// * `alignment` - the minimum alignment in bytes that is required for the
/// newly created buffers memory region.
///
/// returns Result containing the address of the SVM buffer
/// or the error code: CL_INVALID_VALUE if the address is NULL..
pub fn svm_alloc(
    context: cl_context,
    flags: cl_svm_mem_flags,
    size: size_t,
    alignment: cl_uint,
) -> Result<*mut c_void, cl_int> {
    let ptr = unsafe { clSVMAlloc(context, flags, size, alignment) };
    if ptr::null_mut() == ptr {
        Err(CL_INVALID_VALUE)
    } else {
        Ok(ptr)
    }
}

/// Free a shared virtual memory (SVM) buffer allocated using clSVMAlloc.  
/// Calls clSVMFree.  
/// CL_VERSION_2_0
///
/// * `context` - the valid OpenCL context used to create the SVM buffer.
/// * `svm_pointer` - the value returned by a call to clSVMAlloc.
///
pub fn svm_free(context: cl_context, svm_pointer: *mut c_void) {
    unsafe { clSVMFree(context, svm_pointer) };
}
