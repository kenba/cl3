// Copyright (c) 2021-2024 Via Technology Ltd.
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

//! `OpenCL` extensions that don't have external (OpenGL, D3D) dependencies.
//! See: [OpenCL Extension Specification](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_Ext.html)

#![allow(unused_unsafe)]
#![allow(non_camel_case_types)]
#![allow(
    clippy::not_unsafe_ptr_arg_deref,
    clippy::wildcard_in_or_patterns,
    clippy::missing_safety_doc,
    clippy::too_many_arguments
)]

pub use opencl_sys::*;

#[allow(unused_imports)]
use super::info_type::InfoType;
#[allow(unused_imports)]
use super::{api_info_size, api_info_value, api_info_vector};
#[allow(unused_imports)]
use libc::{c_char, c_int, c_void, intptr_t, size_t};
#[allow(unused_imports)]
use std::mem;
#[allow(unused_imports)]
use std::ptr;

#[cfg(feature = "cl_khr_command_buffer")]
#[allow(clippy::cast_possible_truncation)]
pub fn create_command_buffer_khr(
    queues: &[cl_command_queue],
    properties: *const cl_command_buffer_properties_khr,
) -> Result<cl_command_buffer_khr, cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    let buffer = unsafe {
        cl_call!(clCreateCommandBufferKHR(
            queues.len() as cl_uint,
            queues.as_ptr(),
            properties,
            &mut status,
        ))
    };
    if CL_SUCCESS == status {
        Ok(buffer)
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_khr_command_buffer")]
pub fn finalize_command_buffer_khr(command_buffer: cl_command_buffer_khr) -> Result<(), cl_int> {
    let status: cl_int = unsafe { cl_call!(clFinalizeCommandBufferKHR(command_buffer)) };
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_khr_command_buffer")]
pub unsafe fn retain_command_buffer_khr(
    command_buffer: cl_command_buffer_khr,
) -> Result<(), cl_int> {
    let status: cl_int = cl_call!(clRetainCommandBufferKHR(command_buffer));
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_khr_command_buffer")]
pub unsafe fn release_command_buffer_khr(
    command_buffer: cl_command_buffer_khr,
) -> Result<(), cl_int> {
    let status: cl_int = cl_call!(clReleaseCommandBufferKHR(command_buffer));
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_khr_command_buffer")]
pub unsafe fn enqueue_command_buffer_khr(
    num_queues: cl_uint,
    queues: *mut cl_command_queue,
    command_buffer: cl_command_buffer_khr,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = cl_call!(clEnqueueCommandBufferKHR(
        num_queues,
        queues,
        command_buffer,
        num_events_in_wait_list,
        event_wait_list,
        &mut event,
    ));
    if CL_SUCCESS == status {
        Ok(event)
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_khr_command_buffer")]
#[allow(clippy::cast_possible_truncation)]
pub unsafe fn command_barrier_with_wait_list_khr(
    command_buffer: cl_command_buffer_khr,
    command_queue: cl_command_queue,
    sync_point_wait_list: &[cl_sync_point_khr],
    sync_point: *mut cl_sync_point_khr,
    mutable_handle: *mut cl_mutable_command_khr,
) -> Result<(), cl_int> {
    let status: cl_int = cl_call!(clCommandBarrierWithWaitListKHR(
        command_buffer,
        command_queue,
        sync_point_wait_list.len() as cl_uint,
        sync_point_wait_list.as_ptr(),
        sync_point,
        mutable_handle,
    ));
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_khr_command_buffer")]
#[allow(clippy::cast_possible_truncation)]
pub unsafe fn command_copy_buffer_khr(
    command_buffer: cl_command_buffer_khr,
    command_queue: cl_command_queue,
    src_buffer: cl_mem,
    dst_buffer: cl_mem,
    src_offset: size_t,
    dst_offset: size_t,
    size: size_t,
    sync_point_wait_list: &[cl_sync_point_khr],
    sync_point: *mut cl_sync_point_khr,
    mutable_handle: *mut cl_mutable_command_khr,
) -> Result<(), cl_int> {
    let status: cl_int = cl_call!(clCommandCopyBufferKHR(
        command_buffer,
        command_queue,
        src_buffer,
        dst_buffer,
        src_offset,
        dst_offset,
        size,
        sync_point_wait_list.len() as cl_uint,
        sync_point_wait_list.as_ptr(),
        sync_point,
        mutable_handle,
    ));
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_khr_command_buffer")]
#[allow(clippy::cast_possible_truncation)]
pub unsafe fn command_copy_buffer_rect_khr(
    command_buffer: cl_command_buffer_khr,
    command_queue: cl_command_queue,
    src_buffer: cl_mem,
    dst_buffer: cl_mem,
    src_origin: *const size_t,
    dst_origin: *const size_t,
    region: *const size_t,
    src_row_pitch: size_t,
    src_slice_pitch: size_t,
    dst_row_pitch: size_t,
    dst_slice_pitch: size_t,
    sync_point_wait_list: &[cl_sync_point_khr],
    sync_point: *mut cl_sync_point_khr,
    mutable_handle: *mut cl_mutable_command_khr,
) -> Result<(), cl_int> {
    let status: cl_int = cl_call!(clCommandCopyBufferRectKHR(
        command_buffer,
        command_queue,
        src_buffer,
        dst_buffer,
        src_origin,
        dst_origin,
        region,
        src_row_pitch,
        src_slice_pitch,
        dst_row_pitch,
        dst_slice_pitch,
        sync_point_wait_list.len() as cl_uint,
        sync_point_wait_list.as_ptr(),
        sync_point,
        mutable_handle,
    ));
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_khr_command_buffer")]
#[allow(clippy::cast_possible_truncation)]
pub unsafe fn command_copy_buffer_to_image_khr(
    command_buffer: cl_command_buffer_khr,
    command_queue: cl_command_queue,
    src_buffer: cl_mem,
    dst_image: cl_mem,
    src_offset: size_t,
    dst_origin: *const size_t,
    region: *const size_t,
    sync_point_wait_list: &[cl_sync_point_khr],
    sync_point: *mut cl_sync_point_khr,
    mutable_handle: *mut cl_mutable_command_khr,
) -> Result<(), cl_int> {
    let status: cl_int = cl_call!(clCommandCopyBufferToImageKHR(
        command_buffer,
        command_queue,
        src_buffer,
        dst_image,
        src_offset,
        dst_origin,
        region,
        sync_point_wait_list.len() as cl_uint,
        sync_point_wait_list.as_ptr(),
        sync_point,
        mutable_handle,
    ));
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_khr_command_buffer")]
#[allow(clippy::cast_possible_truncation)]
pub unsafe fn command_copy_image_khr(
    command_buffer: cl_command_buffer_khr,
    command_queue: cl_command_queue,
    src_image: cl_mem,
    dst_image: cl_mem,
    src_origin: *const size_t,
    dst_origin: *const size_t,
    region: *const size_t,
    sync_point_wait_list: &[cl_sync_point_khr],
    sync_point: *mut cl_sync_point_khr,
    mutable_handle: *mut cl_mutable_command_khr,
) -> Result<(), cl_int> {
    let status: cl_int = cl_call!(clCommandCopyImageKHR(
        command_buffer,
        command_queue,
        src_image,
        dst_image,
        src_origin,
        dst_origin,
        region,
        sync_point_wait_list.len() as cl_uint,
        sync_point_wait_list.as_ptr(),
        sync_point,
        mutable_handle,
    ));
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_khr_command_buffer")]
#[allow(clippy::cast_possible_truncation)]
pub unsafe fn command_copy_image_to_buffer_khr(
    command_buffer: cl_command_buffer_khr,
    command_queue: cl_command_queue,
    src_image: cl_mem,
    dst_buffer: cl_mem,
    src_origin: *const size_t,
    region: *const size_t,
    dst_offset: size_t,
    sync_point_wait_list: &[cl_sync_point_khr],
    sync_point: *mut cl_sync_point_khr,
    mutable_handle: *mut cl_mutable_command_khr,
) -> Result<(), cl_int> {
    let status: cl_int = cl_call!(clCommandCopyImageToBufferKHR(
        command_buffer,
        command_queue,
        src_image,
        dst_buffer,
        src_origin,
        region,
        dst_offset,
        sync_point_wait_list.len() as cl_uint,
        sync_point_wait_list.as_ptr(),
        sync_point,
        mutable_handle,
    ));
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_khr_command_buffer")]
#[allow(clippy::cast_possible_truncation)]
pub unsafe fn command_fill_buffer_khr(
    command_buffer: cl_command_buffer_khr,
    command_queue: cl_command_queue,
    buffer: cl_mem,
    pattern: *const c_void,
    pattern_size: size_t,
    offset: size_t,
    size: size_t,
    sync_point_wait_list: &[cl_sync_point_khr],
    sync_point: *mut cl_sync_point_khr,
    mutable_handle: *mut cl_mutable_command_khr,
) -> Result<(), cl_int> {
    let status: cl_int = cl_call!(clCommandFillBufferKHR(
        command_buffer,
        command_queue,
        buffer,
        pattern,
        pattern_size,
        offset,
        size,
        sync_point_wait_list.len() as cl_uint,
        sync_point_wait_list.as_ptr(),
        sync_point,
        mutable_handle,
    ));
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_khr_command_buffer")]
#[allow(clippy::cast_possible_truncation)]
pub unsafe fn command_fill_image_khr(
    command_buffer: cl_command_buffer_khr,
    command_queue: cl_command_queue,
    image: cl_mem,
    fill_color: *const c_void,
    origin: *const size_t,
    region: *const size_t,
    sync_point_wait_list: &[cl_sync_point_khr],
    sync_point: *mut cl_sync_point_khr,
    mutable_handle: *mut cl_mutable_command_khr,
) -> Result<(), cl_int> {
    let status: cl_int = cl_call!(clCommandFillImageKHR(
        command_buffer,
        command_queue,
        image,
        fill_color,
        origin,
        region,
        sync_point_wait_list.len() as cl_uint,
        sync_point_wait_list.as_ptr(),
        sync_point,
        mutable_handle,
    ));
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_khr_command_buffer")]
#[allow(clippy::cast_possible_truncation)]
pub unsafe fn command_nd_range_kernel_khr(
    command_buffer: cl_command_buffer_khr,
    command_queue: cl_command_queue,
    properties: *const cl_ndrange_kernel_command_properties_khr,
    kernel: cl_kernel,
    work_dim: cl_uint,
    global_work_offset: *const size_t,
    global_work_size: *const size_t,
    local_work_size: *const size_t,
    sync_point_wait_list: &[cl_sync_point_khr],
    sync_point: *mut cl_sync_point_khr,
    mutable_handle: *mut cl_mutable_command_khr,
) -> Result<(), cl_int> {
    let status: cl_int = cl_call!(clCommandNDRangeKernelKHR(
        command_buffer,
        command_queue,
        properties,
        kernel,
        work_dim,
        global_work_offset,
        global_work_size,
        local_work_size,
        sync_point_wait_list.len() as cl_uint,
        sync_point_wait_list.as_ptr(),
        sync_point,
        mutable_handle,
    ));
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_khr_command_buffer")]
#[allow(clippy::cast_possible_truncation)]
pub unsafe fn command_svm_memcpy_khr(
    command_buffer: cl_command_buffer_khr,
    command_queue: cl_command_queue,
    dst_ptr: *mut c_void,
    src_ptr: *const c_void,
    size: size_t,
    sync_point_wait_list: &[cl_sync_point_khr],
    sync_point: *mut cl_sync_point_khr,
    mutable_handle: *mut cl_mutable_command_khr,
) -> Result<(), cl_int> {
    let status: cl_int = cl_call!(clCommandSVMMemcpyKHR(
        command_buffer,
        command_queue,
        dst_ptr,
        src_ptr,
        size,
        sync_point_wait_list.len() as cl_uint,
        sync_point_wait_list.as_ptr(),
        sync_point,
        mutable_handle,
    ));
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_khr_command_buffer")]
#[allow(clippy::cast_possible_truncation)]
pub unsafe fn command_svm_mem_fill_khr(
    command_buffer: cl_command_buffer_khr,
    command_queue: cl_command_queue,
    svm_ptr: *mut c_void,
    pattern: *const c_void,
    pattern_size: size_t,
    size: size_t,
    sync_point_wait_list: &[cl_sync_point_khr],
    sync_point: *mut cl_sync_point_khr,
    mutable_handle: *mut cl_mutable_command_khr,
) -> Result<(), cl_int> {
    let status: cl_int = cl_call!(clCommandSVMMemFillKHR(
        command_buffer,
        command_queue,
        svm_ptr,
        pattern,
        pattern_size,
        size,
        sync_point_wait_list.len() as cl_uint,
        sync_point_wait_list.as_ptr(),
        sync_point,
        mutable_handle,
    ));
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_khr_command_buffer")]
pub fn get_command_buffer_data_khr(
    command_buffer: cl_command_buffer_khr,
    param_name: cl_command_buffer_info_khr,
) -> Result<Vec<u8>, cl_int> {
    api_info_size!(get_size, clGetCommandBufferInfoKHR);
    let size = get_size(command_buffer, param_name)?;
    api_info_vector!(get_vector, u8, clGetCommandBufferInfoKHR);
    get_vector(command_buffer, param_name, size)
}

#[cfg(feature = "cl_khr_command_buffer")]
pub fn get_command_buffer_info_khr(
    command_queue: cl_command_buffer_khr,
    param_name: cl_command_buffer_info_khr,
) -> Result<InfoType, cl_int> {
    match param_name {
        CL_COMMAND_BUFFER_NUM_QUEUES_KHR
        | CL_COMMAND_BUFFER_REFERENCE_COUNT_KHR
        | CL_COMMAND_BUFFER_STATE_KHR => {
            api_info_value!(get_value, cl_uint, clGetCommandBufferInfoKHR);
            Ok(InfoType::Uint(get_value(command_queue, param_name)?))
        }

        CL_COMMAND_BUFFER_PROPERTIES_ARRAY_KHR => {
            api_info_size!(get_size, clGetCommandBufferInfoKHR);
            api_info_vector!(
                get_vec,
                cl_command_buffer_properties_khr,
                clGetCommandBufferInfoKHR
            );
            let size = get_size(command_queue, param_name)?;
            Ok(InfoType::VecUlong(get_vec(
                command_queue,
                param_name,
                size,
            )?))
        }

        CL_COMMAND_BUFFER_QUEUES_KHR => {
            api_info_size!(get_size, clGetCommandBufferInfoKHR);
            api_info_vector!(get_vec, intptr_t, clGetCommandBufferInfoKHR);
            let size = get_size(command_queue, param_name)?;
            Ok(InfoType::VecIntPtr(get_vec(
                command_queue,
                param_name,
                size,
            )?))
        }

        _ => Ok(InfoType::VecUchar(get_command_buffer_data_khr(
            command_queue,
            param_name,
        )?)),
    }
}

#[cfg(feature = "cl_khr_command_buffer_multi_device")]
pub unsafe fn remap_command_buffer_khr(
    command_buffer: cl_command_buffer_khr,
    automatic: cl_bool,
    num_queues: cl_uint,
    queues: *const cl_command_queue,
    num_handles: cl_uint,
    handles: *const cl_mutable_command_khr,
    handles_ret: *mut cl_mutable_command_khr,
) -> Result<cl_command_buffer_khr, cl_int> {
    let mut errcode_ret: cl_int = CL_INVALID_VALUE;
    let cmd_buffer = cl_call!(clRemapCommandBufferKHR(
        command_buffer,
        automatic,
        num_queues,
        queues,
        num_handles,
        handles,
        handles_ret,
        &mut errcode_ret,
    ));
    if CL_SUCCESS == errcode_ret {
        Ok(cmd_buffer)
    } else {
        Err(errcode_ret)
    }
}

#[cfg(feature = "cl_khr_command_buffer_mutable_dispatch")]
pub unsafe fn update_mutable_commands_khr(
    command_buffer: cl_command_buffer_khr,
    mutable_config: *const cl_mutable_base_config_khr,
) -> Result<(), cl_int> {
    let status: cl_int = cl_call!(clUpdateMutableCommandsKHR(command_buffer, mutable_config));
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_khr_command_buffer_mutable_dispatch")]
pub fn get_command_buffer_mutable_dispatch_data(
    command: cl_mutable_command_khr,
    param_name: cl_mutable_command_info_khr,
) -> Result<Vec<u8>, cl_int> {
    api_info_size!(get_size, clGetMutableCommandInfoKHR);
    let size = get_size(command, param_name)?;
    api_info_vector!(get_vector, u8, clGetMutableCommandInfoKHR);
    get_vector(command, param_name, size)
}

#[cfg(feature = "cl_apple_setmemobjectdestructor")]
pub unsafe fn set_mem_object_destructor_apple(
    memobj: cl_mem,
    pfn_notify: Option<unsafe extern "C" fn(cl_context, *mut c_void)>,
    user_data: *mut c_void,
) -> Result<(), cl_int> {
    let status: cl_int = cl_call!(clSetMemObjectDestructorAPPLE(memobj, pfn_notify, user_data));
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_khr_icd")]
#[allow(clippy::uninit_vec)]
pub fn icd_get_platform_ids_khr() -> Result<Vec<cl_platform_id>, cl_int> {
    // Get the number of platforms
    let mut count: cl_uint = 0;
    let mut status = unsafe { cl_call!(clIcdGetPlatformIDsKHR(0, ptr::null_mut(), &mut count)) };

    if CL_SUCCESS != status {
        Err(status)
    } else if 0 < count {
        // Get the platform ids.
        let len = count as usize;
        let mut ids: Vec<cl_platform_id> = Vec::with_capacity(len);
        unsafe {
            ids.set_len(len);
            status = cl_call!(clIcdGetPlatformIDsKHR(
                count,
                ids.as_mut_ptr(),
                ptr::null_mut()
            ));
        };

        if CL_SUCCESS == status {
            Ok(ids)
        } else {
            Err(status)
        }
    } else {
        Ok(Vec::default())
    }
}

#[cfg(feature = "cl_khr_il_program")]
pub fn create_program_with_il_khr(context: cl_context, il: &[u8]) -> Result<cl_program, cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    let program = unsafe {
        cl_call!(clCreateProgramWithILKHR(
            context,
            il.as_ptr().cast::<c_void>(),
            il.len() as size_t,
            &mut status,
        ))
    };
    if CL_SUCCESS == status {
        Ok(program)
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_khr_terminate_context")]
pub unsafe fn terminate_context_khr(context: cl_context) -> Result<(), cl_int> {
    let status = cl_call!(clTerminateContextKHR(context));
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_khr_create_command_queue")]
pub fn create_command_queue_with_properties_khr(
    context: cl_context,
    device: cl_device_id,
    properties: *const cl_queue_properties_khr,
) -> Result<cl_command_queue, cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    let queue: cl_command_queue = unsafe {
        cl_call!(clCreateCommandQueueWithPropertiesKHR(
            context,
            device,
            properties,
            &mut status
        ))
    };
    if CL_SUCCESS == status {
        Ok(queue)
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_ext_device_fission")]
pub unsafe fn release_device_ext(device: cl_device_id) -> Result<(), cl_int> {
    let status = cl_call!(clReleaseDeviceEXT(device));
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_ext_device_fission")]
pub unsafe fn retain_device_ext(device: cl_device_id) -> Result<(), cl_int> {
    let status = cl_call!(clRetainDeviceEXT(device));
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}

// helper function for create_sub_devices_ext
#[cfg(feature = "cl_ext_device_fission")]
fn count_sub_devices_ext(
    in_device: cl_device_id,
    properties: &[cl_device_partition_property_ext],
) -> Result<cl_uint, cl_int> {
    let mut count: cl_uint = 0;
    let status: cl_int = unsafe {
        cl_call!(clCreateSubDevicesEXT(
            in_device,
            properties.as_ptr(),
            0,
            ptr::null_mut(),
            &mut count,
        ))
    };
    if CL_SUCCESS == status {
        Ok(count)
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_ext_device_fission")]
#[allow(clippy::cast_possible_truncation)]
pub fn create_sub_devices_ext(
    in_device: cl_device_id,
    properties: &[cl_device_partition_property_ext],
) -> Result<Vec<cl_device_id>, cl_int> {
    // get the number of partitions
    let num_devices: cl_uint = count_sub_devices_ext(in_device, properties)?;

    // partition in_device
    let mut ids: Vec<cl_device_id> = Vec::with_capacity(num_devices as usize);
    let status: cl_int = unsafe {
        ids.set_len(num_devices as usize);
        cl_call!(clCreateSubDevicesEXT(
            in_device,
            properties.as_ptr(),
            num_devices * mem::size_of::<cl_device_id>() as cl_uint,
            ids.as_mut_ptr(),
            ptr::null_mut(),
        ))
    };

    if CL_SUCCESS == status {
        Ok(ids)
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_ext_migrate_memobject")]
pub unsafe fn enqueue_migrate_mem_object_ext(
    command_queue: cl_command_queue,
    num_mem_objects: cl_uint,
    mem_objects: *const cl_mem,
    flags: cl_mem_migration_flags_ext,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = cl_call!(clEnqueueMigrateMemObjectEXT(
        command_queue,
        num_mem_objects,
        mem_objects,
        flags,
        num_events_in_wait_list,
        event_wait_list,
        &mut event,
    ));
    if CL_SUCCESS == status {
        Ok(event)
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_qcom_ext_host_ptr")]
pub fn get_device_image_info_qcom(
    device: cl_device_id,
    image_width: size_t,
    image_height: size_t,
    image_format: *const cl_image_format,
    param_name: cl_image_pitch_info_qcom,
) -> Result<cl_uint, cl_int> {
    let mut data: cl_uint = 0;
    let data_ptr: *mut cl_uint = &mut data;
    let status = unsafe {
        cl_call!(clGetDeviceImageInfoQCOM(
            device,
            image_width,
            image_height,
            image_format,
            param_name,
            mem::size_of::<cl_uint>(),
            data_ptr.cast::<c_void>(),
            ptr::null_mut(),
        ))
    };
    if CL_SUCCESS == status {
        Ok(data)
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_img_use_gralloc_ptr")]
pub unsafe fn enqueue_acquire_gralloc_objects_img(
    command_queue: cl_command_queue,
    num_objects: cl_uint,
    mem_objects: *const cl_mem,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = cl_call!(clEnqueueAcquireGrallocObjectsIMG(
        command_queue,
        num_objects,
        mem_objects,
        num_events_in_wait_list,
        event_wait_list,
        &mut event,
    ));
    if CL_SUCCESS == status {
        Ok(event)
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_img_use_gralloc_ptr")]
pub unsafe fn enqueue_release_gralloc_objects_img(
    command_queue: cl_command_queue,
    num_objects: cl_uint,
    mem_objects: *const cl_mem,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = cl_call!(clEnqueueReleaseGrallocObjectsIMG(
        command_queue,
        num_objects,
        mem_objects,
        num_events_in_wait_list,
        event_wait_list,
        &mut event,
    ));
    if CL_SUCCESS == status {
        Ok(event)
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_img_generate_mipmap")]
pub unsafe fn enqueue_generate_mipmap_img(
    command_queue: cl_command_queue,
    src_image: cl_mem,
    dst_image: cl_mem,
    mipmap_filter_mode: cl_mipmap_filter_mode_img,
    array_region: *const size_t,
    mip_region: *const size_t,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = cl_call!(clEnqueueGenerateMipmapIMG(
        command_queue,
        src_image,
        dst_image,
        mipmap_filter_mode,
        array_region,
        mip_region,
        num_events_in_wait_list,
        event_wait_list,
        &mut event,
    ));
    if CL_SUCCESS == status {
        Ok(event)
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_khr_subgroups")]
pub fn get_kernel_sub_group_info_khr(
    kernel: cl_kernel,
    device: cl_device_id,
    param_name: cl_kernel_sub_group_info,
    input_value_size: size_t,
    input_value: *const c_void,
) -> Result<size_t, cl_int> {
    match param_name {
        CL_KERNEL_MAX_SUB_GROUP_SIZE_FOR_NDRANGE_KHR
        | CL_KERNEL_SUB_GROUP_COUNT_FOR_NDRANGE_KHR
        | _ => {
            // Assumes other cl_kernel_sub_group_info values return a size_t
            // get the value
            let mut data: size_t = 0;
            let data_ptr: *mut size_t = &mut data;
            let status = unsafe {
                cl_call!(clGetKernelSubGroupInfoKHR(
                    kernel,
                    device,
                    param_name,
                    input_value_size,
                    input_value,
                    mem::size_of::<size_t>(),
                    data_ptr.cast::<c_void>(),
                    ptr::null_mut(),
                ))
            };
            if CL_SUCCESS == status {
                Ok(data)
            } else {
                Err(status)
            }
        }
    }
}

#[cfg(feature = "cl_khr_suggested_local_work_size")]
pub fn get_kernel_suggested_local_work_size_khr(
    command_queue: cl_command_queue,
    kernel: cl_kernel,
    work_dim: cl_uint,
    global_work_offset: *const size_t,
    global_work_size: *const size_t,
) -> Result<size_t, cl_int> {
    let mut suggested_local_work_size: size_t = 0;
    let status: cl_int = unsafe {
        cl_call!(clGetKernelSuggestedLocalWorkSizeKHR(
            command_queue,
            kernel,
            work_dim,
            global_work_offset,
            global_work_size,
            &mut suggested_local_work_size,
        ))
    };
    if CL_SUCCESS == status {
        Ok(suggested_local_work_size)
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_khr_external_memory")]
pub unsafe fn enqueue_acquire_external_mem_objects_khr(
    command_queue: cl_command_queue,
    num_mem_objects: cl_uint,
    mem_objects: *const cl_mem,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = cl_call!(clEnqueueAcquireExternalMemObjectsKHR(
        command_queue,
        num_mem_objects,
        mem_objects,
        num_events_in_wait_list,
        event_wait_list,
        &mut event,
    ));
    if CL_SUCCESS == status {
        Ok(event)
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_khr_external_memory")]
pub unsafe fn enqueue_release_external_mem_objects_khr(
    command_queue: cl_command_queue,
    num_mem_objects: cl_uint,
    mem_objects: *const cl_mem,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = cl_call!(clEnqueueReleaseExternalMemObjectsKHR(
        command_queue,
        num_mem_objects,
        mem_objects,
        num_events_in_wait_list,
        event_wait_list,
        &mut event,
    ));
    if CL_SUCCESS == status {
        Ok(event)
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_khr_external_semaphore")]
pub fn get_semaphore_handle_for_type_khr(
    sema_object: cl_semaphore_khr,
    device: cl_device_id,
    handle_type: cl_external_semaphore_handle_type_khr,
) -> Result<cl_semaphore_khr, cl_int> {
    // Get the size of the information.
    let mut size: size_t = 0;
    let status: cl_int = unsafe {
        cl_call!(clGetSemaphoreHandleForTypeKHR(
            sema_object,
            device,
            handle_type,
            0,
            ptr::null_mut(),
            &mut size,
        ))
    };
    if CL_SUCCESS == status {
        let mut data: cl_semaphore_khr = ptr::null_mut();
        let data_ptr: *mut cl_semaphore_khr = &mut data;
        let status: cl_int = unsafe {
            cl_call!(clGetSemaphoreHandleForTypeKHR(
                sema_object,
                device,
                handle_type,
                size,
                data_ptr.cast::<c_void>(),
                ptr::null_mut(),
            ))
        };
        if CL_SUCCESS == status {
            Ok(data)
        } else {
            Err(status)
        }
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_khr_external_semaphore_sync_fd")]
pub unsafe fn reimport_semaphore_sync_fd(
    sema_object: cl_semaphore_khr,
    reimport_props: *mut cl_semaphore_reimport_properties_khr,
    fd: c_int,
) -> Result<(), cl_int> {
    let status: cl_int = cl_call!(clReImportSemaphoreSyncFdKHR(
        sema_object,
        reimport_props,
        fd
    ));
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_khr_semaphore")]
pub fn create_semaphore_with_properties_khr(
    context: cl_context,
    sema_props: *const cl_semaphore_properties_khr,
) -> Result<cl_semaphore_khr, cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    let semaphore: cl_semaphore_khr = unsafe {
        cl_call!(clCreateSemaphoreWithPropertiesKHR(
            context,
            sema_props,
            &mut status
        ))
    };
    if CL_SUCCESS == status {
        Ok(semaphore)
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_khr_semaphore")]
pub unsafe fn enqueue_wait_semaphores_khr(
    command_queue: cl_command_queue,
    num_sema_objects: cl_uint,
    sema_objects: *const cl_semaphore_khr,
    sema_payload_list: *const cl_semaphore_payload_khr,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = cl_call!(clEnqueueWaitSemaphoresKHR(
        command_queue,
        num_sema_objects,
        sema_objects,
        sema_payload_list,
        num_events_in_wait_list,
        event_wait_list,
        &mut event,
    ));
    if CL_SUCCESS == status {
        Ok(event)
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_khr_semaphore")]
pub unsafe fn enqueue_signal_semaphores_khr(
    command_queue: cl_command_queue,
    num_sema_objects: cl_uint,
    sema_objects: *const cl_semaphore_khr,
    sema_payload_list: *const cl_semaphore_payload_khr,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = cl_call!(clEnqueueSignalSemaphoresKHR(
        command_queue,
        num_sema_objects,
        sema_objects,
        sema_payload_list,
        num_events_in_wait_list,
        event_wait_list,
        &mut event,
    ));
    if CL_SUCCESS == status {
        Ok(event)
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_khr_semaphore")]
pub fn get_semaphore_info_khr(
    sema_object: cl_semaphore_khr,
    param_name: cl_semaphore_info_khr,
) -> Result<Vec<u8>, cl_int> {
    api_info_size!(get_size, clGetSemaphoreInfoKHR);
    let size = get_size(sema_object, param_name)?;
    api_info_vector!(get_vector, u8, clGetSemaphoreInfoKHR);
    get_vector(sema_object, param_name, size)
}

#[cfg(feature = "cl_khr_semaphore")]
pub unsafe fn release_semaphore_khr(sema_object: cl_semaphore_khr) -> Result<(), cl_int> {
    let status: cl_int = cl_call!(clReleaseSemaphoreKHR(sema_object));
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_khr_semaphore")]
pub unsafe fn retain_semaphore_khr(sema_object: cl_semaphore_khr) -> Result<(), cl_int> {
    let status: cl_int = cl_call!(clRetainSemaphoreKHR(sema_object));
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_arm_import_memory")]
pub unsafe fn import_memory_arm(
    context: cl_context,
    flags: cl_mem_flags,
    properties: *const cl_import_properties_arm,
    memory: *mut c_void,
    size: size_t,
) -> Result<cl_mem, cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    let mem: cl_mem = cl_call!(clImportMemoryARM(
        context,
        flags,
        properties,
        memory,
        size,
        &mut status
    ));
    if CL_SUCCESS == status {
        Ok(mem)
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_arm_shared_virtual_memory")]
pub unsafe fn svm_alloc_arm(
    context: cl_context,
    flags: cl_svm_mem_flags_arm,
    size: size_t,
    alignment: cl_uint,
) -> Result<*mut c_void, cl_int> {
    let ptr = cl_call!(clSVMAllocARM(context, flags, size, alignment));
    if ptr.is_null() {
        Err(CL_INVALID_VALUE)
    } else {
        Ok(ptr)
    }
}

#[cfg(feature = "cl_arm_shared_virtual_memory")]
pub unsafe fn svm_free_arm(context: cl_context, svm_pointer: *mut c_void) -> Result<(), cl_int> {
    cl_call!(clSVMFreeARM(context, svm_pointer));
    Ok(())
}

#[cfg(feature = "cl_arm_shared_virtual_memory")]
pub unsafe fn enqueue_svm_free_arm(
    command_queue: cl_command_queue,
    num_svm_pointers: cl_uint,
    svm_pointers: *mut *mut c_void,
    pfn_free_func: Option<
        unsafe extern "C" fn(
            queue: cl_command_queue,
            num_svm_pointers: cl_uint,
            svm_pointers: *mut *mut c_void,
            user_data: *mut c_void,
        ),
    >,
    user_data: *mut c_void,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = cl_call!(clEnqueueSVMFreeARM(
        command_queue,
        num_svm_pointers,
        svm_pointers,
        pfn_free_func,
        user_data,
        num_events_in_wait_list,
        event_wait_list,
        &mut event,
    ));
    if CL_SUCCESS == status {
        Ok(event)
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_arm_shared_virtual_memory")]
pub unsafe fn enqueue_svm_mem_cpy_arm(
    command_queue: cl_command_queue,
    blocking_copy: cl_bool,
    dst_ptr: *mut c_void,
    src_ptr: *const c_void,
    size: size_t,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = cl_call!(clEnqueueSVMMemcpyARM(
        command_queue,
        blocking_copy,
        dst_ptr,
        src_ptr,
        size,
        num_events_in_wait_list,
        event_wait_list,
        &mut event,
    ));
    if CL_SUCCESS == status {
        Ok(event)
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_arm_shared_virtual_memory")]
pub unsafe fn enqueue_svm_mem_fill_arm(
    command_queue: cl_command_queue,
    svm_ptr: *mut c_void,
    pattern: *const c_void,
    pattern_size: size_t,
    size: size_t,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = cl_call!(clEnqueueSVMMemFillARM(
        command_queue,
        svm_ptr,
        pattern,
        pattern_size,
        size,
        num_events_in_wait_list,
        event_wait_list,
        &mut event,
    ));
    if CL_SUCCESS == status {
        Ok(event)
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_arm_shared_virtual_memory")]
pub unsafe fn enqueue_svm_map_arm(
    command_queue: cl_command_queue,
    blocking_map: cl_bool,
    flags: cl_map_flags,
    svm_ptr: *mut c_void,
    size: size_t,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = cl_call!(clEnqueueSVMMapARM(
        command_queue,
        blocking_map,
        flags,
        svm_ptr,
        size,
        num_events_in_wait_list,
        event_wait_list,
        &mut event,
    ));
    if CL_SUCCESS == status {
        Ok(event)
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_arm_shared_virtual_memory")]
pub unsafe fn enqueue_svm_unmap_arm(
    command_queue: cl_command_queue,
    svm_ptr: *mut c_void,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = cl_call!(clEnqueueSVMUnmapARM(
        command_queue,
        svm_ptr,
        num_events_in_wait_list,
        event_wait_list,
        &mut event,
    ));
    if CL_SUCCESS == status {
        Ok(event)
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_arm_shared_virtual_memory")]
pub fn set_kernel_arg_svm_pointer(
    kernel: cl_kernel,
    arg_index: cl_uint,
    arg_ptr: *const c_void,
) -> Result<(), cl_int> {
    let status: cl_int =
        unsafe { cl_call!(clSetKernelArgSVMPointerARM(kernel, arg_index, arg_ptr)) };
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_arm_shared_virtual_memory")]
pub fn set_kernel_exec_info_arm(
    kernel: cl_kernel,
    param_name: cl_kernel_exec_info_arm,
    param_value_size: size_t,
    param_value: *const c_void,
) -> Result<(), cl_int> {
    let status: cl_int = unsafe {
        cl_call!(clSetKernelExecInfoARM(
            kernel,
            param_name,
            param_value_size,
            param_value
        ))
    };
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_intel_accelerator")]
pub fn create_accelerator_intel(
    context: cl_context,
    accelerator_type: cl_accelerator_type_intel,
    descriptor_size: size_t,
    descriptor: *const c_void,
) -> Result<cl_accelerator_intel, cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    let ptr = unsafe {
        cl_call!(clCreateAcceleratorINTEL(
            context,
            accelerator_type,
            descriptor_size,
            descriptor,
            &mut status,
        ))
    };
    if CL_SUCCESS == status {
        Ok(ptr)
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_intel_accelerator")]
pub fn get_accelerator_data_intel(
    accelerator: cl_accelerator_intel,
    param_name: cl_accelerator_info_intel,
) -> Result<Vec<u8>, cl_int> {
    api_info_size!(get_size, clGetAcceleratorInfoINTEL);
    let size = get_size(accelerator, param_name)?;
    api_info_vector!(get_vector, u8, clGetAcceleratorInfoINTEL);
    get_vector(accelerator, param_name, size)
}

#[cfg(feature = "cl_intel_accelerator")]
pub fn get_accelerator_info_intel(
    accelerator: cl_accelerator_intel,
    param_name: cl_accelerator_info_intel,
) -> Result<InfoType, cl_int> {
    match param_name {
        CL_ACCELERATOR_REFERENCE_COUNT_INTEL | CL_ACCELERATOR_TYPE_INTEL => {
            api_info_value!(get_value, cl_uint, clGetAcceleratorInfoINTEL);
            Ok(InfoType::Uint(get_value(accelerator, param_name)?))
        }
        CL_ACCELERATOR_CONTEXT_INTEL => {
            api_info_value!(get_value, intptr_t, clGetAcceleratorInfoINTEL);
            Ok(InfoType::Ptr(get_value(accelerator, param_name)?))
        }
        CL_ACCELERATOR_DESCRIPTOR_INTEL
            // The complete descriptor structure supplied when the
            // accelerator was created.
        | _ => {
            Ok(InfoType::VecUchar(get_accelerator_data_intel(
                accelerator,
                param_name,
            )?))
        }
    }
}

#[cfg(feature = "cl_intel_accelerator")]
pub unsafe fn retain_accelerator_intel(accelerator: cl_accelerator_intel) -> Result<(), cl_int> {
    let status = cl_call!(clRetainAcceleratorINTEL(accelerator));
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_intel_accelerator")]
pub unsafe fn release_accelerator_intel(accelerator: cl_accelerator_intel) -> Result<(), cl_int> {
    let status = cl_call!(clReleaseAcceleratorINTEL(accelerator));
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_intel_unified_shared_memory")]
pub unsafe fn host_mem_alloc_intel(
    context: cl_context,
    properties: *const cl_mem_properties_intel,
    size: size_t,
    alignment: cl_uint,
) -> Result<(), cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    cl_call!(clHostMemAllocINTEL(
        context,
        properties,
        size,
        alignment,
        &mut status
    ));
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_intel_unified_shared_memory")]
pub unsafe fn device_mem_alloc_intel(
    context: cl_context,
    device: cl_device_id,
    properties: *const cl_mem_properties_intel,
    size: size_t,
    alignment: cl_uint,
) -> Result<(), cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    cl_call!(clDeviceMemAllocINTEL(
        context,
        device,
        properties,
        size,
        alignment,
        &mut status
    ));
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_intel_unified_shared_memory")]
pub unsafe fn shared_mem_alloc_intel(
    context: cl_context,
    device: cl_device_id,
    properties: *const cl_mem_properties_intel,
    size: size_t,
    alignment: cl_uint,
) -> Result<(), cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    cl_call!(clSharedMemAllocINTEL(
        context,
        device,
        properties,
        size,
        alignment,
        &mut status
    ));
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_intel_unified_shared_memory")]
pub unsafe fn mem_free_intel(context: cl_context, ptr: *mut c_void) -> Result<(), cl_int> {
    let status = cl_call!(clMemFreeINTEL(context, ptr));
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_intel_unified_shared_memory")]
pub unsafe fn mem_blocking_free_intel(context: cl_context, ptr: *mut c_void) -> Result<(), cl_int> {
    let status = cl_call!(clMemBlockingFreeINTEL(context, ptr));
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_intel_unified_shared_memory")]
fn mem_alloc_info_intel<T: Default>(
    context: cl_context,
    ptr: *const c_void,
    param_id: cl_mem_info_intel,
) -> Result<T, cl_int> {
    let mut data: T = T::default();
    let data_ptr: *mut T = &mut data;
    let status = unsafe {
        cl_call!(clGetMemAllocInfoINTEL(
            context,
            ptr,
            param_id,
            mem::size_of::<T>(),
            data_ptr.cast::<c_void>(),
            ptr::null_mut(),
        ))
    };
    if CL_SUCCESS == status {
        Ok(data)
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_intel_unified_shared_memory")]
pub fn get_mem_alloc_info_intel(
    context: cl_context,
    ptr: *const c_void,
    param_name: cl_mem_info_intel,
) -> Result<InfoType, cl_int> {
    match param_name {
        CL_MEM_ALLOC_TYPE_INTEL => Ok(InfoType::Uint(mem_alloc_info_intel::<
            cl_unified_shared_memory_type_intel,
        >(context, ptr, param_name)?)),

        CL_MEM_ALLOC_BASE_PTR_INTEL | CL_MEM_ALLOC_DEVICE_INTEL => Ok(InfoType::Ptr(
            mem_alloc_info_intel::<intptr_t>(context, ptr, param_name)?,
        )),

        CL_MEM_ALLOC_SIZE_INTEL => Ok(InfoType::Size(mem_alloc_info_intel::<size_t>(
            context, ptr, param_name,
        )?)),

        _ => {
            // values 0x419E-0x419F are reserved for future queries
            // get the size
            let mut size: size_t = 0;
            let status = unsafe {
                cl_call!(clGetMemAllocInfoINTEL(
                    context,
                    ptr,
                    param_name,
                    0,
                    ptr::null_mut(),
                    &mut size
                ))
            };
            if CL_SUCCESS != status {
                Err(status)
            } else if 0 < size {
                // Get the data.
                let mut data: Vec<u8> = Vec::with_capacity(size);
                let status = unsafe {
                    cl_call!(clGetMemAllocInfoINTEL(
                        context,
                        ptr,
                        param_name,
                        size,
                        data.as_mut_ptr().cast::<c_void>(),
                        ptr::null_mut(),
                    ))
                };
                if CL_SUCCESS == status {
                    Ok(InfoType::VecUchar(data))
                } else {
                    Err(status)
                }
            } else {
                Ok(InfoType::VecUchar(Vec::default()))
            }
        }
    }
}

#[cfg(feature = "cl_intel_unified_shared_memory")]
pub unsafe fn set_kernel_arg_mem_pointer_intel(
    kernel: cl_kernel,
    arg_index: cl_uint,
    arg_value: *const c_void,
) -> Result<(), cl_int> {
    let status = cl_call!(clSetKernelArgMemPointerINTEL(kernel, arg_index, arg_value));
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_intel_unified_shared_memory")]
pub unsafe fn enqueue_mem_set_intel(
    command_queue: cl_command_queue,
    dst_ptr: *mut c_void,
    value: cl_int,
    size: size_t,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = cl_call!(clEnqueueMemsetINTEL(
        command_queue,
        dst_ptr,
        value,
        size,
        num_events_in_wait_list,
        event_wait_list,
        &mut event,
    ));
    if CL_SUCCESS == status {
        Ok(event)
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_intel_unified_shared_memory")]
pub unsafe fn enqueue_mem_fill_intel(
    command_queue: cl_command_queue,
    dst_ptr: *mut c_void,
    pattern: *const c_void,
    pattern_size: size_t,
    size: size_t,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = cl_call!(clEnqueueMemFillINTEL(
        command_queue,
        dst_ptr,
        pattern,
        pattern_size,
        size,
        num_events_in_wait_list,
        event_wait_list,
        &mut event,
    ));
    if CL_SUCCESS == status {
        Ok(event)
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_intel_unified_shared_memory")]
pub unsafe fn enqueue_mem_copy_intel(
    command_queue: cl_command_queue,
    blocking: cl_bool,
    dst_ptr: *mut c_void,
    src_ptr: *const c_void,
    size: size_t,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = cl_call!(clEnqueueMemcpyINTEL(
        command_queue,
        blocking,
        dst_ptr,
        src_ptr,
        size,
        num_events_in_wait_list,
        event_wait_list,
        &mut event,
    ));
    if CL_SUCCESS == status {
        Ok(event)
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_intel_unified_shared_memory")]
pub unsafe fn enqueue_migrate_mem_intel(
    command_queue: cl_command_queue,
    ptr: *const c_void,
    size: size_t,
    flags: cl_mem_migration_flags,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = cl_call!(clEnqueueMigrateMemINTEL(
        command_queue,
        ptr,
        size,
        flags,
        num_events_in_wait_list,
        event_wait_list,
        &mut event,
    ));
    if CL_SUCCESS == status {
        Ok(event)
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_intel_unified_shared_memory")]
pub unsafe fn enqueue_mem_advise_intel(
    command_queue: cl_command_queue,
    ptr: *const c_void,
    size: size_t,
    advice: cl_mem_advice_intel,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = cl_call!(clEnqueueMemAdviseINTEL(
        command_queue,
        ptr,
        size,
        advice,
        num_events_in_wait_list,
        event_wait_list,
        &mut event,
    ));
    if CL_SUCCESS == status {
        Ok(event)
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_intel_create_buffer_with_properties")]
pub unsafe fn create_buffer_with_properties_intel(
    context: cl_context,
    properties: *const cl_mem_properties_intel,
    flags: cl_mem_flags,
    size: size_t,
    host_ptr: *mut c_void,
) -> Result<cl_mem, cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    let mem: cl_mem = cl_call!(clCreateBufferWithPropertiesINTEL(
        context,
        properties,
        flags,
        size,
        host_ptr,
        &mut status
    ));
    if CL_SUCCESS == status {
        Ok(mem)
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_intel_program_scope_host_pipe")]
pub unsafe fn enqueue_read_host_pipe_intel(
    command_queue: cl_command_queue,
    program: cl_program,
    pipe_symbol: *const c_char,
    blocking_read: cl_bool,
    ptr: *mut c_void,
    size: size_t,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = cl_call!(clEnqueueReadHostPipeINTEL(
        command_queue,
        program,
        pipe_symbol,
        blocking_read,
        ptr,
        size,
        num_events_in_wait_list,
        event_wait_list,
        &mut event,
    ));
    if CL_SUCCESS == status {
        Ok(event)
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_intel_program_scope_host_pipe")]
pub unsafe fn enqueue_write_host_pipe_intel(
    command_queue: cl_command_queue,
    program: cl_program,
    pipe_symbol: *const c_char,
    blocking_write: cl_bool,
    ptr: *const c_void,
    size: size_t,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = cl_call!(clEnqueueWriteHostPipeINTEL(
        command_queue,
        program,
        pipe_symbol,
        blocking_write,
        ptr,
        size,
        num_events_in_wait_list,
        event_wait_list,
        &mut event,
    ));
    if CL_SUCCESS == status {
        Ok(event)
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_ext_image_requirements_info")]
pub fn get_image_requirements_info_ext(
    context: cl_context,
    properties: *const cl_mem_properties,
    flags: cl_mem_flags,
    image_format: *const cl_image_format,
    image_desc: *const cl_image_desc,
    param_name: cl_image_requirements_info_ext,
) -> Result<Vec<u8>, cl_int> {
    // get the size
    let mut size: size_t = mem::size_of::<u8>();
    let status: cl_int = unsafe {
        cl_call!(clGetImageRequirementsInfoEXT(
            context,
            properties,
            flags,
            image_format,
            image_desc,
            param_name,
            0,
            ptr::null_mut(),
            &mut size,
        ))
    };
    if CL_SUCCESS == status {
        // Get the data.
        let mut data: Vec<u8> = Vec::with_capacity(size);
        let status = unsafe {
            data.set_len(size);
            cl_call!(clGetImageRequirementsInfoEXT(
                context,
                properties,
                flags,
                image_format,
                image_desc,
                param_name,
                size,
                data.as_mut_ptr().cast::<c_void>(),
                ptr::null_mut(),
            ))
        };
        if CL_SUCCESS == status {
            Ok(data)
        } else {
            Err(status)
        }
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_loader_info")]
pub fn get_icd_loader_info_oclicd(param_name: cl_icdl_info) -> Result<Vec<u8>, cl_int> {
    // get the size
    let mut size: size_t = 0;
    let status = unsafe {
        cl_call!(clGetICDLoaderInfoOCLICD(
            param_name,
            0,
            ptr::null_mut(),
            &mut size
        ))
    };
    if CL_SUCCESS == status {
        // Get the data.
        let mut data: Vec<u8> = Vec::with_capacity(size);
        let status = unsafe {
            cl_call!(clGetICDLoaderInfoOCLICD(
                param_name,
                size,
                data.as_mut_ptr().cast::<c_void>(),
                ptr::null_mut(),
            ))
        };
        if CL_SUCCESS == status {
            Ok(data)
        } else {
            Err(status)
        }
    } else {
        Err(status)
    }
}

#[cfg(feature = "cl_pocl_content_size")]
pub fn set_content_size_buffer_pocl(
    buffer: cl_mem,
    content_size_buffer: cl_mem,
) -> Result<(), cl_int> {
    let status = unsafe { cl_call!(clSetContentSizeBufferPoCL(buffer, content_size_buffer)) };
    if CL_SUCCESS == status {
        Ok(())
    } else {
        Err(status)
    }
}
