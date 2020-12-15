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

//! OpenCL API Foreign Function Interfaces (ffi) for the C functions declared in
//! [cl.h](https://github.com/KhronosGroup/OpenCL-Headers/blob/master/CL/cl.h).

use crate::types::*;
use libc::{c_char, c_uchar, c_void, size_t};

#[cfg_attr(not(target_os = "macos"), link(name = "OpenCL"))]
#[cfg_attr(target_os = "macos", link(name = "OpenCL", kind = "framework"))]
extern "system" {

    // Platform API
    pub fn clGetPlatformIDs(
        num_entries: cl_uint,
        platforms: *mut cl_platform_id,
        num_platforms: *mut cl_uint,
    ) -> cl_int;

    pub fn clGetPlatformInfo(
        platform: cl_platform_id,
        param_name: cl_platform_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;

    // Device API
    pub fn clGetDeviceIDs(
        platform: cl_platform_id,
        device_type: cl_device_type,
        num_entries: cl_uint,
        devices: *mut cl_device_id,
        num_devices: *mut cl_uint,
    ) -> cl_int;

    pub fn clGetDeviceInfo(
        device: cl_device_id,
        param_name: cl_device_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;

    // #ifdef CL_VERSION_1_2
    pub fn clCreateSubDevices(
        in_device: cl_device_id,
        properties: *const cl_device_partition_property,
        num_devices: cl_uint,
        out_devices: *mut cl_device_id,
        num_devices_ret: *mut cl_uint,
    ) -> cl_int;

    pub fn clRetainDevice(device: cl_device_id) -> cl_int;

    pub fn clReleaseDevice(device: cl_device_id) -> cl_int;
    // #endif

    // #ifdef CL_VERSION_2_1
    pub fn clSetDefaultDeviceCommandQueue(
        context: cl_context,
        device: cl_device_id,
        command_queue: cl_command_queue,
    ) -> cl_int;

    pub fn clGetDeviceAndHostTimer(
        device: cl_device_id,
        device_timestamp: *mut cl_ulong,
        host_timestamp: *mut cl_ulong,
    ) -> cl_int;

    pub fn clGetHostTimer(device: cl_device_id, host_timestamp: *mut cl_ulong) -> cl_int;
    // #endif

    // Context APIs
    pub fn clCreateContext(
        properties: *const cl_context_properties,
        num_devices: cl_uint,
        devices: *const cl_device_id,
        pfn_notify: Option<extern "C" fn(*const c_char, *const c_void, size_t, *mut c_void)>,
        user_data: *mut c_void,
        errcode_ret: *mut cl_int,
    ) -> cl_context;

    pub fn clCreateContextFromType(
        properties: *const cl_context_properties,
        device_type: cl_device_type,
        pfn_notify: Option<extern "C" fn(*const c_char, *const c_void, size_t, *mut c_void)>,
        user_data: *mut c_void,
        errcode_ret: *mut cl_int,
    ) -> cl_context;

    pub fn clRetainContext(context: cl_context) -> cl_int;

    pub fn clReleaseContext(context: cl_context) -> cl_int;

    pub fn clGetContextInfo(
        context: cl_context,
        param_name: cl_context_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;

    // #ifdef CL_VERSION_3_0
    pub fn clSetContextDestructorCallback(
        context: cl_context,
        pfn_notify: extern "C" fn(cl_context, *const c_void),
        user_data: *mut c_void,
    ) -> cl_int;
    // #endif

    // Command Queue API

    // #ifdef CL_VERSION_2_0
    pub fn clCreateCommandQueueWithProperties(
        context: cl_context,
        device: cl_device_id,
        properties: *const cl_queue_properties,
        errcode_ret: *mut cl_int,
    ) -> cl_command_queue;
    // #endif

    pub fn clRetainCommandQueue(command_queue: cl_command_queue) -> cl_int;

    pub fn clReleaseCommandQueue(command_queue: cl_command_queue) -> cl_int;

    pub fn clGetCommandQueueInfo(
        command_queue: cl_command_queue,
        param_name: cl_command_queue_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;

    // Memory Object APIs

    pub fn clCreateBuffer(
        context: cl_context,
        flags: cl_mem_flags,
        size: size_t,
        host_ptr: *mut c_void,
        errcode_ret: *mut cl_int,
    ) -> cl_mem;

    // #ifdef CL_VERSION_1_1
    pub fn clCreateSubBuffer(
        buffer: cl_mem,
        flags: cl_mem_flags,
        buffer_create_type: cl_buffer_create_type,
        buffer_create_info: *const c_void,
        errcode_ret: *mut cl_int,
    ) -> cl_mem;
    // #endif

    // #ifdef CL_VERSION_1_2
    pub fn clCreateImage(
        context: cl_context,
        flags: cl_mem_flags,
        image_format: *const cl_image_format,
        image_desc: *const cl_image_desc,
        host_ptr: *mut c_void,
        errcode_ret: *mut cl_int,
    ) -> cl_mem;
    // #endif

    // #ifdef CL_VERSION_2_0
    pub fn clCreatePipe(
        context: cl_context,
        flags: cl_mem_flags,
        pipe_packet_size: cl_uint,
        pipe_max_packets: cl_uint,
        properties: *const cl_pipe_properties,
        errcode_ret: *mut cl_int,
    ) -> cl_mem;
    // #endif

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

    pub fn clRetainMemObject(memobj: cl_mem) -> cl_int;

    pub fn clReleaseMemObject(memobj: cl_mem) -> cl_int;

    pub fn clGetSupportedImageFormats(
        context: cl_context,
        flags: cl_mem_flags,
        image_type: cl_mem_object_type,
        num_entries: cl_uint,
        image_formats: *mut cl_image_format,
        num_image_formats: *mut cl_uint,
    ) -> cl_int;

    pub fn clGetMemObjectInfo(
        memobj: cl_mem,
        param_name: cl_mem_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;

    pub fn clGetImageInfo(
        image: cl_mem,
        param_name: cl_image_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;

    // #ifdef CL_VERSION_2_0
    pub fn clGetPipeInfo(
        pipe: cl_mem,
        param_name: cl_pipe_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;
    // #endif

    // #ifdef CL_VERSION_1_1
    pub fn clSetMemObjectDestructorCallback(
        memobj: cl_mem,
        pfn_notify: extern "C" fn(cl_mem, *mut c_void),
        user_data: *mut c_void,
    ) -> cl_int;
    // #endif

    // SVM Allocation APIs

    // #ifdef CL_VERSION_2_0
    pub fn clSVMAlloc(
        context: cl_context,
        flags: cl_svm_mem_flags,
        size: size_t,
        alignment: cl_uint,
    ) -> *mut c_void;

    pub fn clSVMFree(context: cl_context, svm_pointer: *mut c_void);
    // #endif

    // Sampler APIs

    // #ifdef CL_VERSION_2_0
    pub fn clCreateSamplerWithProperties(
        context: cl_context,
        sampler_properties: *const cl_sampler_properties,
        errcode_ret: *mut cl_int,
    ) -> cl_sampler;
    // #endif

    pub fn clRetainSampler(sampler: cl_sampler) -> cl_int;

    pub fn clReleaseSampler(sampler: cl_sampler) -> cl_int;

    pub fn clGetSamplerInfo(
        sampler: cl_sampler,
        param_name: cl_sampler_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;

    // Program Object APIs

    pub fn clCreateProgramWithSource(
        context: cl_context,
        count: cl_uint,
        strings: *const *const c_char,
        lengths: *const size_t,
        errcode_ret: *mut cl_int,
    ) -> cl_program;

    pub fn clCreateProgramWithBinary(
        context: cl_context,
        num_devices: cl_uint,
        device_list: *const cl_device_id,
        lengths: *const size_t,
        binaries: *const *const c_uchar,
        binary_status: *mut cl_int,
        errcode_ret: *mut cl_int,
    ) -> cl_program;

    // #ifdef CL_VERSION_1_2
    pub fn clCreateProgramWithBuiltInKernels(
        context: cl_context,
        num_devices: cl_uint,
        device_list: *const cl_device_id,
        kernel_names: *const c_char,
        errcode_ret: *mut cl_int,
    ) -> cl_program;
    // endif

    // #ifdef CL_VERSION_2_1
    pub fn clCreateProgramWithIL(
        context: cl_context,
        il: *const c_void,
        length: size_t,
        errcode_ret: *mut cl_int,
    ) -> cl_program;
    // endif

    pub fn clRetainProgram(program: cl_program) -> cl_int;

    pub fn clReleaseProgram(program: cl_program) -> cl_int;

    pub fn clBuildProgram(
        program: cl_program,
        num_devices: cl_uint,
        device_list: *const cl_device_id,
        options: *const c_char,
        pfn_notify: Option<extern "C" fn(cl_program, *mut c_void)>,
        user_data: *mut c_void,
    ) -> cl_int;

    // #ifdef CL_VERSION_1_2
    pub fn clCompileProgram(
        program: cl_program,
        num_devices: cl_uint,
        device_list: *const cl_device_id,
        options: *const c_char,
        num_input_headers: cl_uint,
        input_headers: *const cl_program,
        header_include_names: *const *const c_char,
        pfn_notify: Option<extern "C" fn(program: cl_program, user_data: *mut c_void)>,
        user_data: *mut c_void,
    ) -> cl_int;

    pub fn clLinkProgram(
        context: cl_context,
        num_devices: cl_uint,
        device_list: *const cl_device_id,
        options: *const c_char,
        num_input_programs: cl_uint,
        input_programs: *const cl_program,
        pfn_notify: Option<extern "C" fn(program: cl_program, user_data: *mut c_void)>,
        user_data: *mut c_void,
        errcode_ret: *mut cl_int,
    ) -> cl_program;
    // #endif

    // #ifdef CL_VERSION_2_2
    // CL_EXT_PREFIX__VERSION_2_2_DEPRECATED
    pub fn clSetProgramReleaseCallback(
        program: cl_program,
        pfn_notify: Option<extern "C" fn(program: cl_program, user_data: *mut c_void)>,
        user_data: *mut c_void,
    ) -> cl_int;

    pub fn clSetProgramSpecializationConstant(
        program: cl_program,
        spec_id: cl_uint,
        spec_size: size_t,
        spec_value: *const c_void,
    ) -> cl_int;
    // #endif

    // #ifdef CL_VERSION_1_2
    pub fn clUnloadPlatformCompiler(platform: cl_platform_id) -> cl_int;
    // #endif

    pub fn clGetProgramInfo(
        program: cl_program,
        param_name: cl_program_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;

    pub fn clGetProgramBuildInfo(
        program: cl_program,
        device: cl_device_id,
        param_name: cl_program_build_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;

    // Kernel Object APIs

    pub fn clCreateKernel(
        program: cl_program,
        kernel_name: *const c_char,
        errcode_ret: *mut cl_int,
    ) -> cl_kernel;

    pub fn clCreateKernelsInProgram(
        program: cl_program,
        num_kernels: cl_uint,
        kernels: *mut cl_kernel,
        num_kernels_ret: *mut cl_uint,
    ) -> cl_int;

    // #ifdef CL_VERSION_2_1
    pub fn clCloneKernel(source_kernel: cl_kernel, errcode_ret: *mut cl_int) -> cl_kernel;
    // #endif

    pub fn clRetainKernel(kernel: cl_kernel) -> cl_int;

    pub fn clReleaseKernel(kernel: cl_kernel) -> cl_int;

    pub fn clSetKernelArg(
        kernel: cl_kernel,
        arg_index: cl_uint,
        arg_size: size_t,
        arg_value: *const c_void,
    ) -> cl_int;

    // #ifdef CL_VERSION_2_0
    pub fn clSetKernelArgSVMPointer(
        kernel: cl_kernel,
        arg_index: cl_uint,
        arg_value: *const c_void,
    ) -> cl_int;

    pub fn clSetKernelExecInfo(
        kernel: cl_kernel,
        param_name: cl_kernel_exec_info,
        param_value_size: size_t,
        param_value: *const c_void,
    ) -> cl_int;
    // #endif

    pub fn clGetKernelInfo(
        kernel: cl_kernel,
        param_name: cl_kernel_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;

    // #ifdef CL_VERSION_1_2
    pub fn clGetKernelArgInfo(
        kernel: cl_kernel,
        arg_indx: cl_uint,
        param_name: cl_kernel_arg_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;
    // #endif

    pub fn clGetKernelWorkGroupInfo(
        kernel: cl_kernel,
        device: cl_device_id,
        param_name: cl_kernel_work_group_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;

    // #ifdef CL_VERSION_2_1
    pub fn clGetKernelSubGroupInfo(
        kernel: cl_kernel,
        device: cl_device_id,
        param_name: cl_kernel_sub_group_info,
        input_value_size: size_t,
        input_value: *const c_void,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;
    // #endif

    // Event Object APIs

    pub fn clWaitForEvents(num_events: cl_uint, event_list: *const cl_event) -> cl_int;

    pub fn clGetEventInfo(
        event: cl_event,
        param_name: cl_event_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;

    // #ifdef CL_VERSION_1_1
    pub fn clCreateUserEvent(context: cl_context, errcode_ret: *mut cl_int) -> cl_event;
    // #endif

    pub fn clRetainEvent(event: cl_event) -> cl_int;

    pub fn clReleaseEvent(event: cl_event) -> cl_int;

    // #ifdef CL_VERSION_1_1
    pub fn clSetUserEventStatus(event: cl_event, execution_status: cl_int) -> cl_int;

    pub fn clSetEventCallback(
        event: cl_event,
        command_exec_callback_type: cl_int,
        pfn_notify: extern "C" fn(cl_event, cl_int, *mut c_void),
        user_data: *mut c_void,
    ) -> cl_int;
    // #endif

    // Profiling APIs

    pub fn clGetEventProfilingInfo(
        event: cl_event,
        param_name: cl_profiling_info,
        param_value_size: size_t,
        param_value: *mut c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;

    // Flush and Finish APIs
    pub fn clFlush(command_queue: cl_command_queue) -> cl_int;

    pub fn clFinish(command_queue: cl_command_queue) -> cl_int;

    // Enqueued Commands APIs

    pub fn clEnqueueReadBuffer(
        command_queue: cl_command_queue,
        buffer: cl_mem,
        blocking_read: cl_bool,
        offset: size_t,
        size: size_t,
        ptr: *mut c_void,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    // #ifdef CL_VERSION_1_1
    pub fn clEnqueueReadBufferRect(
        command_queue: cl_command_queue,
        buffer: cl_mem,
        blocking_read: cl_bool,
        buffer_origin: *const size_t,
        host_origin: *const size_t,
        region: *const size_t,
        buffer_row_pitch: size_t,
        buffer_slice_pitch: size_t,
        host_row_pitch: size_t,
        host_slice_pitch: size_t,
        ptr: *mut c_void,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;
    // #endif

    pub fn clEnqueueWriteBuffer(
        command_queue: cl_command_queue,
        buffer: cl_mem,
        blocking_write: cl_bool,
        offset: size_t,
        size: size_t,
        ptr: *const c_void,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    // #ifdef CL_VERSION_1_1
    pub fn clEnqueueWriteBufferRect(
        command_queue: cl_command_queue,
        buffer: cl_mem,
        blocking_write: cl_bool,
        buffer_origin: *const size_t,
        host_origin: *const size_t,
        region: *const size_t,
        buffer_row_pitch: size_t,
        buffer_slice_pitch: size_t,
        host_row_pitch: size_t,
        host_slice_pitch: size_t,
        ptr: *const c_void,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;
    // #endif

    // #ifdef CL_VERSION_1_2
    pub fn clEnqueueFillBuffer(
        command_queue: cl_command_queue,
        buffer: cl_mem,
        pattern: *const c_void,
        pattern_size: size_t,
        offset: size_t,
        size: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;
    // #endif

    pub fn clEnqueueCopyBuffer(
        command_queue: cl_command_queue,
        src_buffer: cl_mem,
        dst_buffer: cl_mem,
        src_offset: size_t,
        dst_offset: size_t,
        size: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    // #ifdef CL_VERSION_1_1
    pub fn clEnqueueCopyBufferRect(
        command_queue: cl_command_queue,
        src_buffer: cl_mem,
        dst_buffer: cl_mem,
        src_origin: *const size_t,
        dst_origin: *const size_t,
        src_row_pitch: size_t,
        src_slice_pitch: size_t,
        dst_row_pitch: size_t,
        dst_slice_pitch: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;
    // #endif

    pub fn clEnqueueReadImage(
        command_queue: cl_command_queue,
        image: cl_mem,
        blocking_read: cl_bool,
        origin: *const size_t,
        region: *const size_t,
        row_pitch: size_t,
        slice_pitch: size_t,
        ptr: *mut c_void,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueWriteImage(
        command_queue: cl_command_queue,
        image: cl_mem,
        blocking_write: cl_bool,
        origin: *const size_t,
        region: *const size_t,
        input_row_pitch: size_t,
        input_slice_pitch: size_t,
        ptr: *const c_void,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    // #ifdef CL_VERSION_1_2
    pub fn clEnqueueFillImage(
        command_queue: cl_command_queue,
        image: cl_mem,
        fill_color: *const c_void,
        origin: *const size_t,
        region: *const size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;
    // #endif

    pub fn clEnqueueCopyImage(
        command_queue: cl_command_queue,
        src_image: cl_mem,
        dst_image: cl_mem,
        src_origin: *const size_t,
        dst_origin: *const size_t,
        region: *const size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueCopyImageToBuffer(
        command_queue: cl_command_queue,
        src_image: cl_mem,
        dst_buffer: cl_mem,
        src_origin: *const size_t,
        region: *const size_t,
        dst_offset: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueCopyBufferToImage(
        command_queue: cl_command_queue,
        src_buffer: cl_mem,
        dst_image: cl_mem,
        src_offset: size_t,
        dst_origin: *const size_t,
        region: *const size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueMapBuffer(
        command_queue: cl_command_queue,
        buffer: cl_mem,
        blocking_map: cl_bool,
        map_flags: cl_map_flags,
        offset: size_t,
        size: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
        errorcode_ret: *mut cl_int,
    ) -> *mut c_void;

    pub fn clEnqueueMapImage(
        command_queue: cl_command_queue,
        image: cl_mem,
        blocking_map: cl_bool,
        map_flags: cl_map_flags,
        origin: *const size_t,
        region: *const size_t,
        image_row_pitch: *mut size_t,
        image_slice_pitch: *mut size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
        errorcode_ret: *mut cl_int,
    ) -> *mut c_void;

    pub fn clEnqueueUnmapMemObject(
        command_queue: cl_command_queue,
        memobj: cl_mem,
        mapped_ptr: *mut c_void,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    // #ifdef CL_VERSION_1_2
    pub fn clEnqueueMigrateMemObjects(
        command_queue: cl_command_queue,
        num_mem_objects: cl_uint,
        mem_objects: *const cl_mem,
        flags: cl_mem_migration_flags,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;
    // #endif

    pub fn clEnqueueNDRangeKernel(
        command_queue: cl_command_queue,
        kernel: cl_kernel,
        work_dim: cl_uint,
        global_work_offset: *const size_t,
        global_work_dims: *const size_t,
        local_work_dims: *const size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueNativeKernel(
        command_queue: cl_command_queue,
        user_func: Option<extern "C" fn(*mut c_void)>,
        args: *mut c_void,
        cb_args: size_t,
        num_mem_objects: cl_uint,
        mem_list: *const cl_mem,
        args_mem_loc: *const *const c_void,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    // #ifdef CL_VERSION_1_2
    pub fn clEnqueueMarkerWithWaitList(
        command_queue: cl_command_queue,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueBarrierWithWaitList(
        command_queue: cl_command_queue,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;
    // #endif

    // #ifdef CL_VERSION_2_0
    pub fn clEnqueueSVMFree(
        command_queue: cl_command_queue,
        num_svm_pointers: cl_uint,
        svm_pointers: *const *const c_void,
        pfn_free_func: Option<
            extern "C" fn(
                queue: cl_command_queue,
                num_svm_pointers: cl_uint,
                svm_pointers: *const *const c_void,
                user_data: *mut c_void,
            ),
        >,
        user_data: *mut c_void,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueSVMMemcpy(
        command_queue: cl_command_queue,
        blocking_copy: cl_bool,
        dst_ptr: *mut c_void,
        src_ptr: *const c_void,
        size: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueSVMMemFill(
        command_queue: cl_command_queue,
        svm_ptr: *mut c_void,
        pattern: *const c_void,
        pattern_size: size_t,
        size: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueSVMMap(
        command_queue: cl_command_queue,
        blocking_map: cl_bool,
        flags: cl_map_flags,
        svm_ptr: *mut c_void,
        size: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueSVMUnmap(
        command_queue: cl_command_queue,
        svm_ptr: *mut c_void,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;
    // #endif

    // #ifdef CL_VERSION_2_1
    pub fn clEnqueueSVMMigrateMem(
        command_queue: cl_command_queue,
        num_svm_pointers: cl_uint,
        svm_pointers: *const *const c_void,
        sizes: *const size_t,
        flags: cl_mem_migration_flags,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;
    // #endif

    // #ifdef CL_VERSION_1_2
    // Extension function access
    //
    // Returns the extension function address for the given function name,
    // or NULL if a valid function can not be found.  The client must
    // check to make sure the address is not NULL, before using or
    // calling the returned function address.
    pub fn clGetExtensionFunctionAddressForPlatform(
        platform: cl_platform_id,
        func_name: *const c_char,
    ) -> *mut c_void;
    // #endif

    // Deprecated OpenCL 1.1 APIs
    // CL_EXT_PREFIX__VERSION_1_1_DEPRECATED
    pub fn clCreateImage2D(
        context: cl_context,
        flags: cl_mem_flags,
        image_format: *mut cl_image_format,
        image_width: size_t,
        image_depth: size_t,
        image_slc_pitch: size_t,
        host_ptr: *mut c_void,
        errcode_ret: *mut cl_int,
    ) -> cl_mem;

    pub fn clCreateImage3D(
        context: cl_context,
        flags: cl_mem_flags,
        image_format: *mut cl_image_format,
        image_width: size_t,
        image_height: size_t,
        image_depth: size_t,
        image_row_pitch: size_t,
        image_slc_pitch: size_t,
        host_ptr: *mut c_void,
        errcode_ret: *mut cl_int,
    ) -> cl_mem;

    pub fn clEnqueueMarker(command_queue: cl_command_queue, event: *mut cl_event) -> cl_int;

    pub fn clEnqueueWaitForEvents(
        command_queue: cl_command_queue,
        num_events: cl_uint,
        event_list: *mut cl_event,
    ) -> cl_int;

    pub fn clEnqueueBarrier(command_queue: cl_command_queue) -> cl_int;

    pub fn clUnloadCompiler() -> cl_int;

    pub fn clGetExtensionFunctionAddress(func_name: *mut c_char);

    // Deprecated OpenCL 2.0 APIs
    // CL_EXT_PREFIX__VERSION_1_2_DEPRECATED
    pub fn clCreateCommandQueue(
        context: cl_context,
        device: cl_device_id,
        properties: cl_command_queue_properties,
        errcode_ret: *mut cl_int,
    ) -> cl_command_queue;

    pub fn clCreateSampler(
        context: cl_context,
        normalize_coords: cl_bool,
        addressing_mode: cl_addressing_mode,
        filter_mode: cl_filter_mode,
        errcode_ret: *mut cl_int,
    ) -> cl_sampler;

    pub fn clEnqueueTask(
        command_queue: cl_command_queue,
        kernel: cl_kernel,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;
}
