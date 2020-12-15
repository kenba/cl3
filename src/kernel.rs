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

//! OpenCL Kernel Object API.

use super::error_codes::{CL_INVALID_VALUE, CL_SUCCESS};
use super::ffi::cl::{
    clCloneKernel, clCreateKernel, clCreateKernelsInProgram, clGetKernelArgInfo, clGetKernelInfo,
    clGetKernelSubGroupInfo, clGetKernelWorkGroupInfo, clReleaseKernel, clRetainKernel,
    clSetKernelArg, clSetKernelArgSVMPointer, clSetKernelExecInfo,
};
use super::info_type::InfoType;
use super::types::{
    cl_device_id, cl_int, cl_kernel, cl_kernel_arg_info, cl_kernel_exec_info, cl_kernel_info,
    cl_kernel_sub_group_info, cl_kernel_work_group_info, cl_program, cl_uint, cl_ulong,
};
use super::{
    api2_info_size, api2_info_value, api2_info_vector, api_info_size, api_info_value,
    api_info_vector,
};

use libc::{c_void, intptr_t, size_t};
use std::ffi::CStr;
use std::mem;
use std::ptr;

/// Create an OpenCL kernel object for a program with a successfully built executable.  
/// Calls clCreateKernel to create an OpenCL kernel object.  
///
/// * `program` - a valid OpenCL program.
/// * `kernel_name` - a kernel function name in the program.
///
/// returns a Result containing the new OpenCL kernel object
/// or the error code from the OpenCL C API function.
pub fn create_kernel(program: cl_program, kernel_name: &CStr) -> Result<cl_kernel, cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    let kernel: cl_kernel = unsafe { clCreateKernel(program, kernel_name.as_ptr(), &mut status) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(kernel)
    }
}

fn count_kernels_in_program(program: cl_program) -> Result<cl_uint, cl_int> {
    let mut count: cl_uint = 0;
    let status: cl_int =
        unsafe { clCreateKernelsInProgram(program, 0, ptr::null_mut(), &mut count) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(count)
    }
}

/// Create OpenCL kernel objects for all kernel functions in a program.  
/// Calls clCreateKernelsInProgram to create OpenCL kernel objects.  
///
/// * `program` - a valid OpenCL program.
///
/// returns a Result containing the new OpenCL kernel objects
/// or the error code from the OpenCL C API function.
pub fn create_kernels_in_program(program: cl_program) -> Result<Vec<cl_kernel>, cl_int> {
    let count: cl_uint = count_kernels_in_program(program)?;
    let mut kernels: Vec<cl_kernel> = Vec::with_capacity(count as usize);
    let status: cl_int = unsafe {
        kernels.set_len(count as usize);
        clCreateKernelsInProgram(
            program,
            count,
            kernels.as_mut_ptr() as *mut cl_kernel,
            ptr::null_mut(),
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(kernels)
    }
}

/// Clone an OpenCL kernel object.  
/// Calls clCloneKernel to clone an OpenCL kernel object.  
/// CL_VERSION_2_1
///
/// * `source_kernel` - a valid OpenCL cl_kernel object that will be copied.
///
/// returns a Result containing the new OpenCL kernel object
/// or the error code from the OpenCL C API function.
pub fn clone_kernel(source_kernel: cl_kernel) -> Result<cl_kernel, cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    let kernel: cl_kernel = unsafe { clCloneKernel(source_kernel, &mut status) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(kernel)
    }
}

/// Retain an OpenCL kernel.  
/// Calls clRetainKernel to increment the kernel reference count.
///
/// * `program` - the OpenCL kernel.
///
/// returns an empty Result or the error code from the OpenCL C API function.
pub fn retain_kernel(kernel: cl_kernel) -> Result<(), cl_int> {
    let status: cl_int = unsafe { clRetainKernel(kernel) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(())
    }
}

/// Release an OpenCL kernel.  
/// Calls clReleaseKernel to decrement the kernel reference count.
///
/// * `kernel` - the OpenCL kernel.
///
/// returns an empty Result or the error code from the OpenCL C API function.
pub fn release_kernel(kernel: cl_kernel) -> Result<(), cl_int> {
    let status: cl_int = unsafe { clReleaseKernel(kernel) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(())
    }
}

/// Set the argument value for a specific argument of a kernel.  
/// Calls clSetKernelArg.  
///
/// * `kernel` - the OpenCL kernel.
/// * `arg_index` - the kernel argument index.
/// * `arg_ptr` - pointer to the data for the argument at arg_index.
///
/// returns an empty Result or the error code from the OpenCL C API function.
pub fn set_kernel_arg(
    kernel: cl_kernel,
    arg_index: cl_uint,
    arg_size: size_t,
    arg_value: *const c_void,
) -> Result<(), cl_int> {
    let status: cl_int = unsafe { clSetKernelArg(kernel, arg_index, arg_size, arg_value) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(())
    }
}

/// Set set a SVM pointer as the argument value for a specific argument of a kernel.  
/// Calls clSetKernelArgSVMPointer.  
///
/// * `kernel` - the OpenCL kernel.
/// * `arg_index` - the kernel argument index.
/// * `arg_ptr` - the SVM pointer to the data for the argument at arg_index.
///
/// returns an empty Result or the error code from the OpenCL C API function.
pub fn set_kernel_arg_svm_pointer(
    kernel: cl_kernel,
    arg_index: cl_uint,
    arg_ptr: *const c_void,
) -> Result<(), cl_int> {
    let status: cl_int = unsafe { clSetKernelArgSVMPointer(kernel, arg_index, arg_ptr) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(())
    }
}

// cl_kernel_exec_info:
pub const CL_KERNEL_EXEC_INFO_SVM_PTRS: cl_kernel_exec_info = 0x11B6;
pub const CL_KERNEL_EXEC_INFO_SVM_FINE_GRAIN_SYSTEM: cl_kernel_exec_info = 0x11B7;

/// Pass additional information other than argument values to a kernel.  
/// Calls clSetKernelExecInfo.  
///
/// * `kernel` - the OpenCL kernel.
/// * `param_name` - the information to be passed to kernel, see:
/// [Kernel Execution Properties](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#kernel-exec-info-table).
/// * `param_ptr` - pointer to the data for the param_name.
///
/// returns an empty Result or the error code from the OpenCL C API function.
pub fn set_kernel_exec_info(
    kernel: cl_kernel,
    param_name: cl_kernel_exec_info,
    param_value_size: size_t,
    param_value: *const c_void,
) -> Result<(), cl_int> {
    let status: cl_int =
        unsafe { clSetKernelExecInfo(kernel, param_name, param_value_size, param_value) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(())
    }
}

// cl_kernel_info:
pub const CL_KERNEL_FUNCTION_NAME: cl_kernel_info = 0x1190;
pub const CL_KERNEL_NUM_ARGS: cl_kernel_info = 0x1191;
pub const CL_KERNEL_REFERENCE_COUNT: cl_kernel_info = 0x1192;
pub const CL_KERNEL_CONTEXT: cl_kernel_info = 0x1193;
pub const CL_KERNEL_PROGRAM: cl_kernel_info = 0x1194;
// #ifdef CL_VERSION_1_2
pub const CL_KERNEL_ATTRIBUTES: cl_kernel_info = 0x1195;
// #endif

/// Get specific information about an OpenCL kernel.  
/// Calls clGetKernelInfo to get the desired information about the kernel.
///
/// * `kernel` - the OpenCL kernel.
/// * `param_name` - the type of kernel information being queried, see:
/// [Kernel Object Queries](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#kernel-info-table).
///
/// returns a Result containing the desired information in an InfoType enum
/// or the error code from the OpenCL C API function.
pub fn get_kernel_info(kernel: cl_kernel, param_name: cl_kernel_info) -> Result<InfoType, cl_int> {
    api_info_size!(get_size, clGetKernelInfo);
    match param_name {
        CL_KERNEL_FUNCTION_NAME | CL_KERNEL_ATTRIBUTES => {
            api_info_vector!(get_string, u8, clGetKernelInfo);
            let size = get_size(kernel, param_name)?;
            Ok(InfoType::Str(get_string(kernel, param_name, size)?))
        }

        CL_KERNEL_NUM_ARGS | CL_KERNEL_REFERENCE_COUNT => {
            api_info_value!(get_value, cl_uint, clGetKernelInfo);
            Ok(InfoType::Uint(get_value(kernel, param_name)?))
        }

        CL_KERNEL_CONTEXT | CL_KERNEL_PROGRAM => {
            api_info_value!(get_value, intptr_t, clGetKernelInfo);
            Ok(InfoType::Ptr(get_value(kernel, param_name)?))
        }

        _ => Err(CL_INVALID_VALUE),
    }
}

// cl_kernel_arg_info
pub const CL_KERNEL_ARG_ADDRESS_QUALIFIER: cl_kernel_arg_info = 0x1196;
pub const CL_KERNEL_ARG_ACCESS_QUALIFIER: cl_kernel_arg_info = 0x1197;
pub const CL_KERNEL_ARG_TYPE_NAME: cl_kernel_arg_info = 0x1198;
pub const CL_KERNEL_ARG_TYPE_QUALIFIER: cl_kernel_arg_info = 0x1199;
pub const CL_KERNEL_ARG_NAME: cl_kernel_arg_info = 0x119A;

/// Get specific information about arguments of an OpenCL kernel.  
/// Calls clGetKernelArgInfo to get the desired information about the kernel.
///
/// * `kernel` - the OpenCL kernel.
/// * `arg_index` - the kernel argument index.
/// * `param_name` - the type of kernel information being queried, see:
/// [Kernel Argument Queries](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#kernel-argument-info-table).
///
/// returns a Result containing the desired information in an InfoType enum
/// or the error code from the OpenCL C API function.
pub fn get_kernel_arg_info(
    kernel: cl_kernel,
    arg_indx: cl_uint,
    param_name: cl_kernel_arg_info,
) -> Result<InfoType, cl_int> {
    match param_name {
        CL_KERNEL_ARG_ADDRESS_QUALIFIER
        | CL_KERNEL_ARG_ACCESS_QUALIFIER
        | CL_KERNEL_ARG_TYPE_QUALIFIER => {
            api2_info_value!(get_index_value, cl_uint, cl_uint, clGetKernelArgInfo);
            Ok(InfoType::Uint(get_index_value(
                kernel, arg_indx, param_name,
            )?))
        }

        CL_KERNEL_ARG_TYPE_NAME | CL_KERNEL_ARG_NAME => {
            api2_info_size!(get_device_size, cl_uint, clGetKernelArgInfo);
            api2_info_vector!(get_device_string, cl_uint, u8, clGetKernelArgInfo);
            let size = get_device_size(kernel, arg_indx, param_name)?;
            Ok(InfoType::Str(get_device_string(
                kernel, arg_indx, param_name, size,
            )?))
        }

        _ => Err(CL_INVALID_VALUE),
    }
}

// cl_kernel_work_group_info
pub const CL_KERNEL_WORK_GROUP_SIZE: cl_kernel_work_group_info = 0x11B0;
pub const CL_KERNEL_COMPILE_WORK_GROUP_SIZE: cl_kernel_work_group_info = 0x11B1;
pub const CL_KERNEL_LOCAL_MEM_SIZE: cl_kernel_work_group_info = 0x11B2;
pub const CL_KERNEL_PREFERRED_WORK_GROUP_SIZE_MULTIPLE: cl_kernel_work_group_info = 0x11B3;
pub const CL_KERNEL_PRIVATE_MEM_SIZE: cl_kernel_work_group_info = 0x11B4;
pub const CL_KERNEL_GLOBAL_WORK_SIZE: cl_kernel_work_group_info = 0x11B5;

/// Get specific information about work groups of an OpenCL kernel.  
/// Calls clGetKernelWorkGroupInfo to get the desired information about the kernel.
///
/// * `kernel` - the OpenCL kernel.
/// * `device` - a specific device in the list of devices associated with kernel.
/// * `param_name` - the type of kernel information being queried, see:
/// [Kernel Object Device Queries](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#kernel-workgroup-info-table).
///
/// returns a Result containing the desired information in an InfoType enum
/// or the error code from the OpenCL C API function.
pub fn get_kernel_work_group_info(
    kernel: cl_kernel,
    device: cl_device_id,
    param_name: cl_kernel_work_group_info,
) -> Result<InfoType, cl_int> {
    match param_name {
        CL_KERNEL_WORK_GROUP_SIZE | CL_KERNEL_PREFERRED_WORK_GROUP_SIZE_MULTIPLE => {
            api2_info_value!(
                get_index_value,
                cl_device_id,
                size_t,
                clGetKernelWorkGroupInfo
            );
            Ok(InfoType::Size(get_index_value(kernel, device, param_name)?))
        }

        CL_KERNEL_COMPILE_WORK_GROUP_SIZE | CL_KERNEL_GLOBAL_WORK_SIZE => {
            api2_info_size!(get_device_size, cl_device_id, clGetKernelWorkGroupInfo);
            api2_info_vector!(
                get_device_vec,
                cl_device_id,
                size_t,
                clGetKernelWorkGroupInfo
            );
            let size = get_device_size(kernel, device, param_name)?;
            Ok(InfoType::VecSize(get_device_vec(
                kernel, device, param_name, size,
            )?))
        }

        CL_KERNEL_LOCAL_MEM_SIZE | CL_KERNEL_PRIVATE_MEM_SIZE => {
            api2_info_value!(
                get_index_value,
                cl_device_id,
                cl_ulong,
                clGetKernelWorkGroupInfo
            );
            Ok(InfoType::Ulong(get_index_value(
                kernel, device, param_name,
            )?))
        }

        _ => Err(CL_INVALID_VALUE),
    }
}

// cl_kernel_sub_group_info
pub const CL_KERNEL_MAX_SUB_GROUP_SIZE_FOR_NDRANGE: cl_kernel_sub_group_info = 0x2033;
pub const CL_KERNEL_SUB_GROUP_COUNT_FOR_NDRANGE: cl_kernel_sub_group_info = 0x2034;
pub const CL_KERNEL_LOCAL_SIZE_FOR_SUB_GROUP_COUNT: cl_kernel_sub_group_info = 0x11B8;
pub const CL_KERNEL_MAX_NUM_SUB_GROUPS: cl_kernel_sub_group_info = 0x11B9;
pub const CL_KERNEL_COMPILE_NUM_SUB_GROUPS: cl_kernel_sub_group_info = 0x11BA;

/// Get specific information about sub groups of an OpenCL kernel.  
/// Calls clGetKernelSubGroupInfo to get the desired information about the kernel.  
/// CL_VERSION_2_1
///
/// * `kernel` - the OpenCL kernel.
/// * `device` - a specific device in the list of devices associated with kernel.
/// * `param_name` - the type of kernel information being queried, see:
/// [Kernel Object Subgroup Queries](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#kernel-subgroup-info-table).
/// * `input_value_size` - the size in bytes of memory pointed to by input_value.
/// * `input_value` -  pointer to memory where the appropriate parameterization
/// of the query is passed from.
///
/// returns a Result containing the desired information in an InfoType enum
/// or the error code from the OpenCL C API function.
pub fn get_kernel_sub_group_info(
    kernel: cl_kernel,
    device: cl_device_id,
    param_name: cl_kernel_sub_group_info,
    input_value_size: size_t,
    input_value: *const c_void,
) -> Result<InfoType, cl_int> {
    // Get the size of the information.
    let mut size: size_t = 0;
    let status: cl_int = unsafe {
        clGetKernelSubGroupInfo(
            kernel,
            device,
            param_name,
            input_value_size,
            input_value,
            0,
            ptr::null_mut(),
            &mut size,
        )
    };

    if CL_SUCCESS != status {
        Err(status)
    } else {
        // Get the information.
        let count = size / mem::size_of::<size_t>();
        let mut data: Vec<size_t> = Vec::with_capacity(count);
        let status = unsafe {
            data.set_len(count);
            clGetKernelSubGroupInfo(
                kernel,
                device,
                param_name,
                input_value_size,
                input_value,
                size,
                data.as_mut_ptr() as *mut c_void,
                ptr::null_mut(),
            )
        };
        if CL_SUCCESS != status {
            Err(status)
        } else {
            match param_name {
                CL_KERNEL_MAX_SUB_GROUP_SIZE_FOR_NDRANGE
                | CL_KERNEL_SUB_GROUP_COUNT_FOR_NDRANGE
                | CL_KERNEL_MAX_NUM_SUB_GROUPS
                | CL_KERNEL_COMPILE_NUM_SUB_GROUPS => Ok(InfoType::Size(data[0])),

                CL_KERNEL_LOCAL_SIZE_FOR_SUB_GROUP_COUNT => Ok(InfoType::VecSize(data)),

                _ => Err(CL_INVALID_VALUE),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::{create_context, release_context};
    use crate::device::{get_device_ids, CL_DEVICE_TYPE_GPU};
    use crate::platform::get_platform_ids;
    use crate::program::{build_program, create_program_with_source, release_program};
    use std::ffi::CString;

    #[test]
    fn test_kernel() {
        let platform_ids = get_platform_ids().unwrap();

        // Choose the platform with the most compliant GPU
        let platform_id = platform_ids[1];

        let device_ids = get_device_ids(platform_id, CL_DEVICE_TYPE_GPU).unwrap();
        assert!(0 < device_ids.len());

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

        // Convert source to a C string
        let src = CString::new(source).unwrap();
        let char_ptrs: [*const _; 1] = [src.as_ptr()];
        let program =
            create_program_with_source(context, 1, char_ptrs.as_ptr(), ptr::null()).unwrap();

        let options = CString::default();
        build_program(program, &device_ids, &options, None, ptr::null_mut()).unwrap();

        let kernel_name = "saxpy_float";
        let name = CString::new(kernel_name).unwrap();
        let kernel = create_kernel(program, &name).unwrap();

        let value = get_kernel_info(kernel, CL_KERNEL_FUNCTION_NAME).unwrap();
        let value = value.to_str().unwrap();
        println!("CL_KERNEL_FUNCTION_NAME: {:?}", value);
        let value = value.into_string().unwrap();
        assert!(0 < value.len());

        release_kernel(kernel).unwrap();
        release_program(program).unwrap();
        release_context(context).unwrap();
    }
}
