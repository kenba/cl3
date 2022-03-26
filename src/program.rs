// Copyright (c) 2020-2022 Via Technology Ltd. All Rights Reserved.
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

//! OpenCL Program Object API.

#![allow(non_camel_case_types)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![allow(clippy::wildcard_in_or_patterns)]

pub use cl_sys::{
    CL_BUILD_ERROR, CL_BUILD_IN_PROGRESS, CL_BUILD_NONE, CL_BUILD_SUCCESS, CL_PROGRAM_BINARIES,
    CL_PROGRAM_BINARY_SIZES, CL_PROGRAM_BINARY_TYPE, CL_PROGRAM_BINARY_TYPE_COMPILED_OBJECT,
    CL_PROGRAM_BINARY_TYPE_EXECUTABLE, CL_PROGRAM_BINARY_TYPE_LIBRARY, CL_PROGRAM_BINARY_TYPE_NONE,
    CL_PROGRAM_BUILD_GLOBAL_VARIABLE_TOTAL_SIZE, CL_PROGRAM_BUILD_LOG, CL_PROGRAM_BUILD_OPTIONS,
    CL_PROGRAM_BUILD_STATUS, CL_PROGRAM_CONTEXT, CL_PROGRAM_DEVICES, CL_PROGRAM_IL,
    CL_PROGRAM_KERNEL_NAMES, CL_PROGRAM_NUM_DEVICES, CL_PROGRAM_NUM_KERNELS,
    CL_PROGRAM_REFERENCE_COUNT, CL_PROGRAM_SOURCE,
};

use super::error_codes::{CL_INVALID_VALUE, CL_SUCCESS};
use super::info_type::InfoType;
#[allow(unused_imports)]
use super::types::{
    cl_context, cl_device_id, cl_int, cl_platform_id, cl_program, cl_program_build_info,
    cl_program_info, cl_uint,
};
use super::{
    api2_info_size, api2_info_value, api2_info_vector, api_info_size, api_info_value,
    api_info_vector,
};

#[cfg(feature = "CL_VERSION_2_1")]
use cl_sys::clCreateProgramWithIL;
use cl_sys::{
    clBuildProgram, clCreateProgramWithBinary, clCreateProgramWithSource, clGetProgramBuildInfo,
    clGetProgramInfo, clReleaseProgram, clRetainProgram,
};
#[cfg(feature = "CL_VERSION_1_2")]
use cl_sys::{clCompileProgram, clLinkProgram};

use libc::{c_char, c_uchar, c_void, intptr_t, size_t};
use std::ffi::CStr;
use std::mem;
use std::ptr;

// clUnloadPlatformCompiler disabled in cl_sys due to platform incompatibility.
// clCreateProgramWithBuiltInKernels kernel_names mutability incorrect in cl_sys
// clSetProgramReleaseCallback, clSetProgramSpecializationConstant, are
// CL_VERSION_2_2 and missing from cl_sys
#[cfg_attr(not(target_os = "macos"), link(name = "OpenCL"))]
#[cfg_attr(target_os = "macos", link(name = "OpenCL", kind = "framework"))]
extern "system" {
    #[cfg(feature = "CL_VERSION_1_2")]
    pub fn clUnloadPlatformCompiler(platform: cl_platform_id) -> cl_int;

    #[cfg(feature = "CL_VERSION_1_2")]
    pub fn clCreateProgramWithBuiltInKernels(
        context: cl_context,
        num_devices: cl_uint,
        device_list: *const cl_device_id,
        kernel_names: *const c_char,
        errcode_ret: *mut cl_int,
    ) -> cl_program;

    #[cfg(feature = "CL_VERSION_2_2")]
    pub fn clSetProgramReleaseCallback(
        program: cl_program,
        pfn_notify: Option<extern "C" fn(program: cl_program, user_data: *mut c_void)>,
        user_data: *mut c_void,
    ) -> cl_int;

    #[cfg(feature = "CL_VERSION_2_2")]
    pub fn clSetProgramSpecializationConstant(
        program: cl_program,
        spec_id: cl_uint,
        spec_size: size_t,
        spec_value: *const c_void,
    ) -> cl_int;
}

// Missing from cl_sys
pub const CL_PROGRAM_SCOPE_GLOBAL_CTORS_PRESENT: cl_program_info = 0x116A;
pub const CL_PROGRAM_SCOPE_GLOBAL_DTORS_PRESENT: cl_program_info = 0x116B;

/// Create an OpenCL program object for a context and load source code into that object.  
/// Calls clCreateProgramWithSource to create an OpenCL program object.  
///
/// * `context` - a valid OpenCL context.
/// * `sources` - an array of slices of source code strings.
///
/// returns a Result containing the new OpenCL program object
/// or the error code from the OpenCL C API function.
#[inline]
pub fn create_program_with_source(
    context: cl_context,
    sources: &[&str],
) -> Result<cl_program, cl_int> {
    let lengths: Vec<size_t> = sources.iter().map(|src| src.len()).collect();
    let mut status: cl_int = CL_INVALID_VALUE;
    let program: cl_program = unsafe {
        clCreateProgramWithSource(
            context,
            sources.len() as cl_uint,
            sources.as_ptr() as *const *const c_char,
            lengths.as_ptr(),
            &mut status,
        )
    };

    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(program)
    }
}

/// Create an OpenCL program object for a context and load binary bits into that object.  
/// Calls clCreateProgramWithBinary to create an OpenCL program object.  
///
/// * `context` - a valid OpenCL context.
/// * `devices` - a slice of devices that are in context.
/// * `binaries` - a slice of program binaries slices.
///
/// returns a Result containing the new OpenCL program object
/// or the error code from the OpenCL C API function.
pub fn create_program_with_binary(
    context: cl_context,
    devices: &[cl_device_id],
    binaries: &[&[u8]],
) -> Result<cl_program, cl_int> {
    let binaries_length = binaries.len();
    let lengths: Vec<size_t> = binaries.iter().map(|bin| bin.len()).collect();
    let mut binary_status: Vec<cl_int> = Vec::with_capacity(binaries_length);
    unsafe { binary_status.set_len(binaries_length) };
    let mut status: cl_int = CL_INVALID_VALUE;
    let program: cl_program = unsafe {
        clCreateProgramWithBinary(
            context,
            devices.len() as cl_uint,
            devices.as_ptr(),
            lengths.as_ptr(),
            binaries.as_ptr() as *const *const c_uchar,
            binary_status.as_mut_ptr(),
            &mut status,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(program)
    }
}

/// Create an OpenCL program object for a context and  loads the information
/// related to the built-in kernels into that object.  
/// Calls clCreateProgramWithBuiltInKernels to create an OpenCL program object.  
///
/// * `context` - a valid OpenCL context.
/// * `devices` - a slice of devices that are in context.
/// * `kernel_names` - a semi-colon separated list of built-in kernel names.
///
/// returns a Result containing the new OpenCL program object
/// or the error code from the OpenCL C API function.
#[cfg(feature = "CL_VERSION_1_2")]
#[inline]
pub fn create_program_with_builtin_kernels(
    context: cl_context,
    devices: &[cl_device_id],
    kernel_names: &CStr,
) -> Result<cl_program, cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    let program: cl_program = unsafe {
        clCreateProgramWithBuiltInKernels(
            context,
            devices.len() as cl_uint,
            devices.as_ptr(),
            kernel_names.as_ptr(),
            &mut status,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(program)
    }
}

/// Create an OpenCL program object for a context and load code in an intermediate
/// language into that object.  
/// Calls clCreateProgramWithIL to create an OpenCL program object.  
/// CL_VERSION_2_1
///
/// * `context` - a valid OpenCL context.
/// * `il` - a slice of program intermediate language code.
///
/// returns a Result containing the new OpenCL program object
/// or the error code from the OpenCL C API function.
#[cfg(feature = "CL_VERSION_2_1")]
#[inline]
pub fn create_program_with_il(context: cl_context, il: &[u8]) -> Result<cl_program, cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    let program: cl_program = unsafe {
        clCreateProgramWithIL(
            context,
            il.as_ptr() as *const c_void,
            il.len() as size_t,
            &mut status,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(program)
    }
}

/// Retain an OpenCL program.  
/// Calls clRetainProgram to increment the program reference count.
///
/// * `program` - the OpenCL program.
///
/// returns an empty Result or the error code from the OpenCL C API function.
#[inline]
pub fn retain_program(program: cl_program) -> Result<(), cl_int> {
    let status: cl_int = unsafe { clRetainProgram(program) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(())
    }
}

/// Release an OpenCL program.  
/// Calls clReleaseProgram to decrement the program reference count.
///
/// * `program` - the OpenCL program.
///
/// returns an empty Result or the error code from the OpenCL C API function.
#[inline]
pub fn release_program(program: cl_program) -> Result<(), cl_int> {
    let status: cl_int = unsafe { clReleaseProgram(program) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(())
    }
}

/// Build (compile & link) a program executable.  
/// Calls clBuildProgram to build an OpenCL program object.  
///
/// * `program` - a valid OpenCL program.
/// * `devices` - a slice of devices that are in context.
/// * `options` - the build options in a null-terminated string.
/// * `pfn_notify` - an optional function pointer to a notification routine.
/// * `user_data` - passed as an argument when pfn_notify is called, or ptr::null_mut().
///
/// returns a Result containing the new OpenCL program object
/// or the error code from the OpenCL C API function.
#[inline]
pub fn build_program(
    program: cl_program,
    devices: &[cl_device_id],
    options: &CStr,
    pfn_notify: Option<extern "C" fn(cl_program, *mut c_void)>,
    user_data: *mut c_void,
) -> Result<(), cl_int> {
    let status: cl_int = unsafe {
        clBuildProgram(
            program,
            devices.len() as cl_uint,
            devices.as_ptr(),
            options.as_ptr(),
            pfn_notify,
            user_data,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(())
    }
}

/// Compile a program’s source for the devices the OpenCL context associated
/// with the program.  
/// Calls clCompileProgram to compile an OpenCL program object.  
///
/// * `program` - a valid OpenCL program.
/// * `devices` - a slice of devices that are in context.
/// * `options` - the compilation options in a null-terminated string.
/// * `input_headers` - a slice of programs that describe headers in the input_headers.
/// * `header_include_names` - an array that has a one to one correspondence with
/// input_headers.
/// * `pfn_notify` - an optional function pointer to a notification routine.
/// * `user_data` - passed as an argument when pfn_notify is called, or ptr::null_mut().
///
/// returns a Result containing the new OpenCL program object
/// or the error code from the OpenCL C API function.
///
/// # Panics
///
/// Panics if `input_headers.len()` != `header_include_names.len()`.
#[cfg(feature = "CL_VERSION_1_2")]
#[inline]
pub fn compile_program(
    program: cl_program,
    devices: &[cl_device_id],
    options: &CStr,
    input_headers: &[cl_program],
    header_include_names: &[&CStr],
    pfn_notify: Option<extern "C" fn(program: cl_program, user_data: *mut c_void)>,
    user_data: *mut c_void,
) -> Result<(), cl_int> {
    assert!(input_headers.len() == header_include_names.len());
    let status: cl_int = unsafe {
        let input_headers_ptr = if input_headers.is_empty() {
            ptr::null()
        } else {
            input_headers.as_ptr()
        };
        let header_include_names_ptr = if header_include_names.is_empty() {
            ptr::null()
        } else {
            header_include_names.as_ptr()
        };
        clCompileProgram(
            program,
            devices.len() as cl_uint,
            devices.as_ptr(),
            options.as_ptr(),
            input_headers.len() as cl_uint,
            input_headers_ptr,
            header_include_names_ptr as *const *const c_char,
            pfn_notify,
            user_data,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(())
    }
}

/// Link a set of compiled program objects and libraries for the devices in the
/// OpenCL context associated with the program.  
/// Calls clLinkProgram to link an OpenCL program object.  
///
/// * `context` - a valid OpenCL context.
/// * `devices` - a slice of devices that are in context.
/// * `options` - the link options in a null-terminated string.
/// * `input_programs` - a slice of programs that are to be linked to create the program executable.
/// * `pfn_notify` - an optional function pointer to a notification routine.
/// * `user_data` - passed as an argument when pfn_notify is called, or ptr::null_mut().
///
/// returns a Result containing the new OpenCL program object
/// or the error code from the OpenCL C API function.
///
/// # Panics
///
/// Panics if `input_programs.is_empty()`.
#[cfg(feature = "CL_VERSION_1_2")]
#[inline]
pub fn link_program(
    context: cl_context,
    devices: &[cl_device_id],
    options: &CStr,
    input_programs: &[cl_program],
    pfn_notify: Option<extern "C" fn(program: cl_program, user_data: *mut c_void)>,
    user_data: *mut c_void,
) -> Result<cl_program, cl_int> {
    assert!(!input_programs.is_empty());
    let mut status: cl_int = CL_INVALID_VALUE;
    let programme: cl_program = unsafe {
        clLinkProgram(
            context,
            devices.len() as cl_uint,
            devices.as_ptr(),
            options.as_ptr(),
            input_programs.len() as cl_uint,
            input_programs.as_ptr(),
            pfn_notify,
            user_data,
            &mut status,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(programme)
    }
}

/// Register a callback function with a program object that is called when the
/// program object is destroyed.  
/// Calls clSetProgramReleaseCallback to register a callback function.  
/// CL_VERSION_2_2
///
/// * `program` - the program being deleted.
/// * `pfn_notify` - function pointer to the notification routine.
/// * `user_data` - passed as an argument when pfn_notify is called, or ptr::null_mut().
///
/// returns an empty Result or the error code from the OpenCL C API function.
#[cfg(feature = "CL_VERSION_2_2")]
#[inline]
pub fn set_program_release_callback(
    program: cl_program,
    pfn_notify: Option<extern "C" fn(program: cl_program, user_data: *mut c_void)>,
    user_data: *mut c_void,
) -> Result<(), cl_int> {
    let status: cl_int = unsafe { clSetProgramReleaseCallback(program, pfn_notify, user_data) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(())
    }
}

/// Set the value of a specialization constant.  
/// Calls clSetProgramSpecializationConstant.  
/// CL_VERSION_2_2  
///
/// * `program` - the program.
/// * `spec_id` - the specialization constant whose value will be set.
/// * `spec_size` - size in bytes of the data pointed to by spec_value.
/// * `spec_value` - pointer to the memory location that contains the value
/// of the specialization constant.
///
/// returns an empty Result or the error code from the OpenCL C API function.
#[cfg(feature = "CL_VERSION_2_2")]
#[inline]
pub fn set_program_specialization_constant(
    program: cl_program,
    spec_id: cl_uint,
    spec_size: size_t,
    spec_value: *const c_void,
) -> Result<(), cl_int> {
    let status: cl_int =
        unsafe { clSetProgramSpecializationConstant(program, spec_id, spec_size, spec_value) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(())
    }
}

/// Release the resources allocated by the OpenCL compiler for platform.  
/// Calls clUnloadPlatformCompiler.  
///
/// * `platform` - the platform.
///
/// returns an empty Result or the error code from the OpenCL C API function.
#[cfg(feature = "CL_VERSION_1_2")]
#[inline]
pub fn unload_platform_compiler(platform: cl_platform_id) -> Result<(), cl_int> {
    let status: cl_int = unsafe { clUnloadPlatformCompiler(platform) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(())
    }
}

/// Get data about an OpenCL program.
/// Calls clGetProgramInfo to get the desired data about the program.
pub fn get_program_data(
    program: cl_program,
    param_name: cl_program_info,
) -> Result<Vec<u8>, cl_int> {
    api_info_size!(get_size, clGetProgramInfo);
    let size = get_size(program, param_name)?;
    api_info_vector!(get_vector, u8, clGetProgramInfo);
    get_vector(program, param_name, size)
}

/// Get specific information about an OpenCL program.  
/// Calls clGetProgramInfo to get the desired information about the program.
///
/// * `program` - the OpenCL program.
/// * `param_name` - the type of program information being queried, see:
/// [Program Object Queries](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#program-info-table).
///
/// returns a Result containing the desired information in an InfoType enum
/// or the error code from the OpenCL C API function.
pub fn get_program_info(
    program: cl_program,
    param_name: cl_program_info,
) -> Result<InfoType, cl_int> {
    api_info_size!(get_size, clGetProgramInfo);

    match param_name {
        CL_PROGRAM_REFERENCE_COUNT
        | CL_PROGRAM_NUM_DEVICES
        | CL_PROGRAM_SCOPE_GLOBAL_CTORS_PRESENT // CL_VERSION_2_2 only
        | CL_PROGRAM_SCOPE_GLOBAL_DTORS_PRESENT // CL_VERSION_2_2 only
        => {
            api_info_value!(get_value, cl_uint, clGetProgramInfo);
            Ok(InfoType::Uint(get_value(program, param_name)?))
        }

        CL_PROGRAM_CONTEXT => {
            api_info_value!(get_value, intptr_t, clGetProgramInfo);
            Ok(InfoType::Ptr(get_value(program, param_name)?))
        }

        CL_PROGRAM_DEVICES => {
            api_info_vector!(get_vec, intptr_t, clGetProgramInfo);
            let size = get_size(program, param_name)?;
            Ok(InfoType::VecIntPtr(get_vec(program, param_name, size)?))
        }

        CL_PROGRAM_BINARY_SIZES => {
            api_info_vector!(get_vec, size_t, clGetProgramInfo);
            let size = get_size(program, param_name)?;
            Ok(InfoType::VecSize(get_vec(program, param_name, size)?))
        }

        CL_PROGRAM_BINARIES => {
            // Gets the binaries for all the devices in the context

            // get the binary sizes, as the case above
            api_info_vector!(get_size_vec, size_t, clGetProgramInfo);
            let size = get_size(program, CL_PROGRAM_BINARY_SIZES as cl_program_info)?;
            let binary_sizes = get_size_vec(program, CL_PROGRAM_BINARY_SIZES as cl_program_info, size)?;

            // A vector of vectors to hold the binaries of each device
            let binaries = binary_sizes.into_iter().map(|size| {
                vec![0u8; size]
            }).collect::<Vec<Vec<u8>>>();

            // Create a vector of pointers to the vectors in binaries
            let mut binary_ptrs = binaries.iter().map(|vec| {
                vec.as_ptr()
            }).collect::<Vec<_>>();

            let status = unsafe {
                clGetProgramInfo(
                    program,
                    param_name,
                    binary_ptrs.len() * mem::size_of::<*mut c_void>(),
                    binary_ptrs.as_mut_ptr() as *mut _ as *mut c_void,
                    ptr::null_mut(),
                )
            };
            if CL_SUCCESS != status {
                Err(status)
            } else {
                Ok(InfoType::VecVecUchar(binaries))
            }
        }

        CL_PROGRAM_NUM_KERNELS => {
            api_info_value!(get_value, size_t, clGetProgramInfo);
            Ok(InfoType::Size(get_value(program, param_name)?))
        }

        CL_PROGRAM_SOURCE
        | CL_PROGRAM_KERNEL_NAMES
        | CL_PROGRAM_IL
        | _ => {
            Ok(InfoType::VecUchar(get_program_data(program, param_name)?))
        }
    }
}

/// Get data about an OpenCL program build.
/// Calls clGetProgramBuildInfo to get the desired data about the program build.
pub fn get_program_build_data(
    program: cl_program,
    device: cl_device_id,
    param_name: cl_program_info,
) -> Result<Vec<u8>, cl_int> {
    api2_info_size!(get_size, cl_device_id, clGetProgramBuildInfo);
    let size = get_size(program, device, param_name)?;
    api2_info_vector!(get_vector, cl_device_id, u8, clGetProgramBuildInfo);
    get_vector(program, device, param_name, size)
}

/// Get specific information about an OpenCL program build.  
/// Calls clGetProgramBuildInfo to get the desired information about the program build.
///
/// * `program` - the OpenCL program.
/// * `device` - -the device for which build information is being queried.
/// * `param_name` - the type of program build information being queried, see:
/// [Program Build Queries](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#program-build-info-table).
///
/// returns a Result containing the desired information in an InfoType enum
/// or the error code from the OpenCL C API function.
pub fn get_program_build_info(
    program: cl_program,
    device: cl_device_id,
    param_name: cl_program_build_info,
) -> Result<InfoType, cl_int> {
    match param_name {
        CL_PROGRAM_BUILD_STATUS => {
            api2_info_value!(
                get_device_value,
                cl_device_id,
                cl_int,
                clGetProgramBuildInfo
            );
            Ok(InfoType::Int(get_device_value(
                program, device, param_name,
            )?))
        }

        CL_PROGRAM_BINARY_TYPE => {
            api2_info_value!(
                get_device_value,
                cl_device_id,
                cl_uint,
                clGetProgramBuildInfo
            );
            Ok(InfoType::Uint(get_device_value(
                program, device, param_name,
            )?))
        }

        // CL_VERSION_2_0
        CL_PROGRAM_BUILD_GLOBAL_VARIABLE_TOTAL_SIZE => {
            api2_info_value!(
                get_device_value,
                cl_device_id,
                size_t,
                clGetProgramBuildInfo
            );
            Ok(InfoType::Size(get_device_value(
                program, device, param_name,
            )?))
        }

        CL_PROGRAM_BUILD_OPTIONS | CL_PROGRAM_BUILD_LOG | _ => Ok(InfoType::VecUchar(
            get_program_build_data(program, device, param_name)?,
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::{create_context, release_context};
    use crate::device::{get_device_ids, CL_DEVICE_TYPE_ALL};
    #[allow(unused_imports)]
    use crate::error_codes::error_text;
    use crate::platform::get_platform_ids;
    use std::ffi::CString;

    #[test]
    fn test_program() {
        let platform_ids = get_platform_ids().unwrap();

        let mut platform_id = platform_ids[0];
        let mut device_count: usize = 0;

        // Search for a platform with the most devices
        for p in platform_ids {
            let ids = get_device_ids(p, CL_DEVICE_TYPE_ALL).unwrap();
            let count = ids.len();
            if device_count < count {
                device_count = count;
                platform_id = p;
            }
        }

        println!("Platform device_count: {}", device_count);

        let device_ids = get_device_ids(platform_id, CL_DEVICE_TYPE_ALL).unwrap();
        let device_id = device_ids[0];

        let context = create_context(&device_ids, ptr::null(), None, ptr::null_mut());
        let context = context.unwrap();

        let source = r#"
            kernel void saxpy_float (global float* z,
                global float const* x,
                global float const* y,
                float a)
            {
            size_t i = get_global_id(0);
            z[i] = a*x[i] + y[i];
            }
        "#;

        // Convert source to an array
        let sources = [source];
        let program = create_program_with_source(context, &sources).unwrap();

        let value = get_program_info(program, CL_PROGRAM_REFERENCE_COUNT).unwrap();
        let value = cl_uint::from(value);
        println!("CL_PROGRAM_REFERENCE_COUNT: {}", value);
        assert!(0 < value);

        let value = get_program_info(program, CL_PROGRAM_CONTEXT).unwrap();
        let value = intptr_t::from(value);
        println!("CL_PROGRAM_CONTEXT: {}", value);
        assert!(0 < value);

        let value = get_program_info(program, CL_PROGRAM_NUM_DEVICES).unwrap();
        let value = cl_uint::from(value);
        println!("CL_PROGRAM_NUM_DEVICES: {}", value);
        assert!(0 < value);

        let value = get_program_info(program, CL_PROGRAM_DEVICES).unwrap();
        let value = Vec::<intptr_t>::from(value);
        println!("CL_PROGRAM_DEVICES: {}", value.len());
        assert!(0 < value.len());

        let value = get_program_info(program, CL_PROGRAM_SOURCE).unwrap();
        let value = String::from(value);
        println!("CL_PROGRAM_SOURCE: {}", value);
        assert!(0 < value.len());

        let options = CString::default();
        build_program(program, &device_ids, &options, None, ptr::null_mut()).unwrap();

        let value = get_program_build_info(program, device_id, CL_PROGRAM_BUILD_STATUS).unwrap();
        let value: cl_int = From::from(value);
        println!("CL_PROGRAM_BUILD_STATUS: {}", value);
        assert_eq!(CL_BUILD_SUCCESS, value);

        let value = get_program_build_info(program, device_id, CL_PROGRAM_BUILD_OPTIONS).unwrap();
        let value = String::from(value);
        println!("CL_PROGRAM_BUILD_OPTIONS: {}", value);

        let value = get_program_build_info(program, device_id, CL_PROGRAM_BUILD_LOG).unwrap();
        let value = String::from(value);
        println!("CL_PROGRAM_BUILD_LOG: {}", value);

        let value = get_program_build_info(program, device_id, CL_PROGRAM_BINARY_TYPE).unwrap();
        let value = cl_uint::from(value);
        println!("CL_PROGRAM_BINARY_TYPE: {:?}", value);
        assert_eq!(CL_PROGRAM_BINARY_TYPE_EXECUTABLE as cl_uint, value);

        #[cfg(feature = "CL_VERSION_2_0")]
        match get_program_build_info(
            program,
            device_id,
            CL_PROGRAM_BUILD_GLOBAL_VARIABLE_TOTAL_SIZE,
        ) {
            Ok(value) => {
                let value = size_t::from(value);
                println!("CL_PROGRAM_BUILD_GLOBAL_VARIABLE_TOTAL_SIZE: {:?}", value)
            }
            Err(e) => println!(
                "OpenCL error, CL_PROGRAM_BUILD_GLOBAL_VARIABLE_TOTAL_SIZE: {}",
                error_text(e)
            ),
        }

        let value = get_program_info(program, CL_PROGRAM_BINARY_SIZES).unwrap();
        let value = Vec::<size_t>::from(value);
        println!("CL_PROGRAM_BINARY_SIZES: {}", value.len());
        println!("CL_PROGRAM_BINARY_SIZES: {:?}", value);
        assert!(0 < value.len());

        let value = get_program_info(program, CL_PROGRAM_BINARIES).unwrap();
        // println!("CL_PROGRAM_BINARIES: {:?}", value);
        let value = Vec::<Vec<u8>>::from(value);
        println!("CL_PROGRAM_BINARIES count: {}", value.len());
        println!("CL_PROGRAM_BINARIES length[0]: {}", value[0].len());
        assert!(0 < value.len());

        let value = get_program_info(program, CL_PROGRAM_NUM_KERNELS).unwrap();
        let value = size_t::from(value);
        println!("CL_PROGRAM_NUM_KERNELS: {}", value);
        assert!(0 < value);

        let value = get_program_info(program, CL_PROGRAM_KERNEL_NAMES).unwrap();
        let value = String::from(value);
        println!("CL_PROGRAM_KERNEL_NAMES: {}", value);
        assert!(0 < value.len());

        #[cfg(feature = "CL_VERSION_2_1")]
        match get_program_info(program, CL_PROGRAM_IL) {
            Ok(value) => {
                let value = String::from(value);
                println!("CL_PROGRAM_IL: {}", value)
            }
            Err(e) => println!("OpenCL error, CL_PROGRAM_IL: {}", error_text(e)),
        };

        #[cfg(feature = "CL_VERSION_2_2")]
        match get_program_info(program, CL_PROGRAM_SCOPE_GLOBAL_CTORS_PRESENT) {
            Ok(value) => {
                let value = cl_uint::from(value);
                println!("CL_PROGRAM_SCOPE_GLOBAL_CTORS_PRESENT: {}", value)
            }
            Err(e) => println!(
                "OpenCL error, CL_PROGRAM_SCOPE_GLOBAL_CTORS_PRESENT: {}",
                error_text(e)
            ),
        };

        #[cfg(feature = "CL_VERSION_2_2")]
        match get_program_info(program, CL_PROGRAM_SCOPE_GLOBAL_CTORS_PRESENT) {
            Ok(value) => {
                let value = cl_uint::from(value);
                println!("CL_PROGRAM_SCOPE_GLOBAL_DTORS_PRESENT: {}", value)
            }
            Err(e) => println!(
                "OpenCL error, CL_PROGRAM_SCOPE_GLOBAL_DTORS_PRESENT: {}",
                error_text(e)
            ),
        };

        #[cfg(feature = "CL_VERSION_1_2")]
        if let Err(e) = unload_platform_compiler(platform_id) {
            println!("OpenCL error, clUnloadPlatformCompiler: {}", error_text(e));
        }

        release_program(program).unwrap();

        release_context(context).unwrap();
    }

    #[test]
    fn test_compile_and_link_program() {
        let platform_ids = get_platform_ids().unwrap();

        let mut platform_id = platform_ids[0];
        let mut device_count: usize = 0;

        // Search for a platform with the most devices
        for p in platform_ids {
            let ids = get_device_ids(p, CL_DEVICE_TYPE_ALL).unwrap();
            let count = ids.len();
            if device_count < count {
                device_count = count;
                platform_id = p;
            }
        }

        println!("Platform device_count: {}", device_count);

        let device_ids = get_device_ids(platform_id, CL_DEVICE_TYPE_ALL).unwrap();

        let context = create_context(&device_ids, ptr::null(), None, ptr::null_mut());
        let context = context.unwrap();

        let source = r#"
            kernel void saxpy_float (global float* z,
                global float const* x,
                global float const* y,
                float a)
            {
            size_t i = get_global_id(0);
            z[i] = a*x[i] + y[i];
            }
        "#;

        // Convert source to an array
        let sources = [source];
        let program = create_program_with_source(context, &sources).unwrap();

        use std::ffi::CString;
        let no_options = CString::new("").unwrap();
        compile_program(
            program,
            &device_ids,
            &no_options,
            &[],
            &[],
            None,
            ptr::null_mut(),
        )
        .unwrap();

        let programs = [program];
        link_program(
            context,
            &device_ids,
            &no_options,
            &programs,
            None,
            ptr::null_mut(),
        )
        .unwrap();

        release_program(program).unwrap();

        release_context(context).unwrap();
    }
}
