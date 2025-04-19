// Copyright (c) 2024 Via Technology Ltd.
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

#![allow(
    non_snake_case,
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::wildcard_imports
)]

use dlopen2::wrapper::WrapperApi;

use libc::{c_char, c_int, c_uchar, c_void, size_t};

use opencl_sys::cl_dx9_media_sharing::*;
use opencl_sys::cl_egl::*;
use opencl_sys::cl_function_types::*;
use opencl_sys::cl_layer::*;
use opencl_sys::*;

mod utils;
pub use utils::{OpenClRuntime, is_opencl_runtime_available, load_library};

/// Wrapper for the `OpenCL` API functions.
///
/// These functions are marked as optional to avoid library load failure
/// if a function is not present in the library.
#[derive(WrapperApi)]
pub struct OpenCl {
    // Platform API
    clGetPlatformIDs: Option<
        fn(
            num_entries: cl_uint,
            platforms: *mut cl_platform_id,
            num_platforms: *mut cl_uint,
        ) -> cl_int,
    >,

    clGetPlatformInfo: Option<
        fn(
            platform: cl_platform_id,
            param_name: cl_platform_info,
            param_value_size: size_t,
            param_value: *mut c_void,
            param_value_size_ret: *mut size_t,
        ) -> cl_int,
    >,

    // Device APIs
    clGetDeviceIDs: Option<
        fn(
            platform: cl_platform_id,
            device_type: cl_device_type,
            num_entries: cl_uint,
            devices: *mut cl_device_id,
            num_devices: *mut cl_uint,
        ) -> cl_int,
    >,

    clGetDeviceInfo: Option<
        fn(
            device: cl_device_id,
            param_name: cl_device_info,
            param_value_size: size_t,
            param_value: *mut c_void,
            param_value_size_ret: *mut size_t,
        ) -> cl_int,
    >,

    clCreateSubDevices: Option<
        fn(
            in_device: cl_device_id,
            properties: *const cl_device_partition_property,
            num_devices: cl_uint,
            out_devices: *mut cl_device_id,
            num_devices_ret: *mut cl_uint,
        ) -> cl_int,
    >,

    clRetainDevice: Option<fn(device: cl_device_id) -> cl_int>,

    clReleaseDevice: Option<fn(device: cl_device_id) -> cl_int>,

    clSetDefaultDeviceCommandQueue: Option<
        fn(context: cl_context, device: cl_device_id, command_queue: cl_command_queue) -> cl_int,
    >,

    clGetDeviceAndHostTimer: Option<
        fn(
            device: cl_device_id,
            device_timestamp: *mut cl_ulong,
            host_timestamp: *mut cl_ulong,
        ) -> cl_int,
    >,

    clGetHostTimer: Option<fn(device: cl_device_id, host_timestamp: *mut cl_ulong) -> cl_int>,

    // Context APIs
    clCreateContext: Option<
        fn(
            properties: *const cl_context_properties,
            num_devices: cl_uint,
            devices: *const cl_device_id,
            pfn_notify: Option<
                unsafe extern "C" fn(
                    errinfo: *const c_char,
                    private_info: *const c_void,
                    cb: size_t,
                    user_data: *mut c_void,
                ),
            >,
            user_data: *mut c_void,
            errcode_ret: *mut cl_int,
        ) -> cl_context,
    >,

    clCreateContextFromType: Option<
        fn(
            properties: *const cl_context_properties,
            device_type: cl_device_type,
            pfn_notify: Option<
                unsafe extern "C" fn(
                    errinfo: *const c_char,
                    private_info: *const c_void,
                    cb: size_t,
                    user_data: *mut c_void,
                ),
            >,
            user_data: *mut c_void,
            errcode_ret: *mut cl_int,
        ) -> cl_context,
    >,

    clRetainContext: Option<fn(context: cl_context) -> cl_int>,

    clReleaseContext: Option<fn(context: cl_context) -> cl_int>,

    clGetContextInfo: Option<
        fn(
            context: cl_context,
            param_name: cl_context_info,
            param_value_size: size_t,
            param_value: *mut c_void,
            param_value_size_ret: *mut size_t,
        ) -> cl_int,
    >,

    clSetContextDestructorCallback: Option<
        fn(
            context: cl_context,
            pfn_notify: Option<unsafe extern "C" fn(context: cl_context, user_data: *mut c_void)>,
            user_data: *mut c_void,
        ) -> cl_int,
    >,

    // Command Queue APIs
    clCreateCommandQueueWithProperties: Option<
        fn(
            context: cl_context,
            device: cl_device_id,
            properties: *const cl_queue_properties,
            errcode_ret: *mut cl_int,
        ) -> cl_command_queue,
    >,

    clRetainCommandQueue: Option<fn(command_queue: cl_command_queue) -> cl_int>,

    clReleaseCommandQueue: Option<fn(command_queue: cl_command_queue) -> cl_int>,

    clGetCommandQueueInfo: Option<
        fn(
            command_queue: cl_command_queue,
            param_name: cl_command_queue_info,
            param_value_size: size_t,
            param_value: *mut c_void,
            param_value_size_ret: *mut size_t,
        ) -> cl_int,
    >,

    // Memory Object APIs
    clCreateBuffer: Option<
        fn(
            context: cl_context,
            flags: cl_mem_flags,
            size: size_t,
            host_ptr: *mut c_void,
            errcode_ret: *mut cl_int,
        ) -> cl_mem,
    >,

    clCreateSubBuffer: Option<
        fn(
            buffer: cl_mem,
            flags: cl_mem_flags,
            buffer_create_type: cl_buffer_create_type,
            buffer_create_info: *const c_void,
            errcode_ret: *mut cl_int,
        ) -> cl_mem,
    >,

    clCreateImage: Option<
        fn(
            context: cl_context,
            flags: cl_mem_flags,
            image_format: *const cl_image_format,
            image_desc: *const cl_image_desc,
            host_ptr: *mut c_void,
            errcode_ret: *mut cl_int,
        ) -> cl_mem,
    >,

    clCreatePipe: Option<
        fn(
            context: cl_context,
            flags: cl_mem_flags,
            pipe_packet_size: cl_uint,
            pipe_max_packets: cl_uint,
            properties: *const cl_pipe_properties,
            errcode_ret: *mut cl_int,
        ) -> cl_mem,
    >,

    clCreateBufferWithProperties: Option<
        fn(
            context: cl_context,
            properties: *const cl_mem_properties,
            flags: cl_mem_flags,
            size: size_t,
            host_ptr: *mut c_void,
            errcode_ret: *mut cl_int,
        ) -> cl_mem,
    >,

    clCreateImageWithProperties: Option<
        fn(
            context: cl_context,
            properties: *const cl_mem_properties,
            flags: cl_mem_flags,
            image_format: *const cl_image_format,
            image_desc: *const cl_image_desc,
            host_ptr: *mut c_void,
            errcode_ret: *mut cl_int,
        ) -> cl_mem,
    >,

    clRetainMemObject: Option<fn(memobj: cl_mem) -> cl_int>,

    clReleaseMemObject: Option<fn(memobj: cl_mem) -> cl_int>,

    clGetSupportedImageFormats: Option<
        fn(
            context: cl_context,
            flags: cl_mem_flags,
            image_type: cl_mem_object_type,
            num_entries: cl_uint,
            image_formats: *mut cl_image_format,
            num_image_formats: *mut cl_uint,
        ) -> cl_int,
    >,

    clGetMemObjectInfo: Option<
        fn(
            memobj: cl_mem,
            param_name: cl_mem_info,
            param_value_size: size_t,
            param_value: *mut c_void,
            param_value_size_ret: *mut size_t,
        ) -> cl_int,
    >,

    clGetImageInfo: Option<
        fn(
            image: cl_mem,
            param_name: cl_image_info,
            param_value_size: size_t,
            param_value: *mut c_void,
            param_value_size_ret: *mut size_t,
        ) -> cl_int,
    >,

    clGetPipeInfo: Option<
        fn(
            pipe: cl_mem,
            param_name: cl_pipe_info,
            param_value_size: size_t,
            param_value: *mut c_void,
            param_value_size_ret: *mut size_t,
        ) -> cl_int,
    >,

    clSetMemObjectDestructorCallback: Option<
        fn(
            memobj: cl_mem,
            pfn_notify: Option<unsafe extern "C" fn(memobj: cl_mem, user_data: *mut c_void)>,
            user_data: *mut c_void,
        ) -> cl_int,
    >,

    // SVM Allocation APIs
    clSVMAlloc: Option<
        fn(
            context: cl_context,
            flags: cl_svm_mem_flags,
            size: size_t,
            alignment: cl_uint,
        ) -> *mut c_void,
    >,

    clSVMFree: Option<fn(context: cl_context, svm_pointer: *mut c_void)>,

    // Sampler APIs
    clCreateSamplerWithProperties: Option<
        fn(
            context: cl_context,
            normalized_coords: *const cl_sampler_properties,
            errcode_ret: *mut cl_int,
        ) -> cl_sampler,
    >,

    clRetainSampler: Option<fn(sampler: cl_sampler) -> cl_int>,

    clReleaseSampler: Option<fn(sampler: cl_sampler) -> cl_int>,

    clGetSamplerInfo: Option<
        fn(
            sampler: cl_sampler,
            param_name: cl_sampler_info,
            param_value_size: size_t,
            param_value: *mut c_void,
            param_value_size_ret: *mut size_t,
        ) -> cl_int,
    >,

    // Program Object APIs
    clCreateProgramWithSource: Option<
        fn(
            context: cl_context,
            count: cl_uint,
            strings: *const *const c_char,
            lengths: *const size_t,
            errcode_ret: *mut cl_int,
        ) -> cl_program,
    >,

    clCreateProgramWithBinary: Option<
        fn(
            context: cl_context,
            num_devices: cl_uint,
            device_list: *const cl_device_id,
            lengths: *const size_t,
            binaries: *const *const c_uchar,
            binary_status: *mut cl_int,
            errcode_ret: *mut cl_int,
        ) -> cl_program,
    >,

    clCreateProgramWithBuiltInKernels: Option<
        fn(
            context: cl_context,
            num_devices: cl_uint,
            device_list: *const cl_device_id,
            kernel_names: *const c_char,
            errcode_ret: *mut cl_int,
        ) -> cl_program,
    >,

    clCreateProgramWithIL: Option<
        fn(
            context: cl_context,
            il: *const c_void,
            length: size_t,
            errcode_ret: *mut cl_int,
        ) -> cl_program,
    >,

    clRetainProgram: Option<fn(program: cl_program) -> cl_int>,

    clReleaseProgram: Option<fn(program: cl_program) -> cl_int>,

    clBuildProgram: Option<
        fn(
            program: cl_program,
            num_devices: cl_uint,
            device_list: *const cl_device_id,
            options: *const c_char,
            pfn_notify: Option<unsafe extern "C" fn(program: cl_program, user_data: *mut c_void)>,
            user_data: *mut c_void,
        ) -> cl_int,
    >,

    clCompileProgram: Option<
        fn(
            program: cl_program,
            num_devices: cl_uint,
            device_list: *const cl_device_id,
            options: *const c_char,
            num_input_headers: cl_uint,
            input_headers: *const cl_program,
            header_include_names: *const *const c_char,
            pfn_notify: Option<unsafe extern "C" fn(program: cl_program, user_data: *mut c_void)>,
            user_data: *mut c_void,
        ) -> cl_int,
    >,

    clLinkProgram: Option<
        fn(
            context: cl_context,
            num_devices: cl_uint,
            device_list: *const cl_device_id,
            options: *const c_char,
            num_input_programs: cl_uint,
            input_programs: *const cl_program,
            pfn_notify: Option<unsafe extern "C" fn(program: cl_program, user_data: *mut c_void)>,
            user_data: *mut c_void,
            errcode_ret: *mut cl_int,
        ) -> cl_program,
    >,

    clSetProgramReleaseCallback: Option<
        fn(
            program: cl_program,
            pfn_notify: Option<unsafe extern "C" fn(program: cl_program, user_data: *mut c_void)>,
            user_data: *mut c_void,
        ) -> cl_int,
    >,

    clSetProgramSpecializationConstant: Option<
        fn(
            program: cl_program,
            spec_id: cl_uint,
            spec_size: size_t,
            spec_value: *const c_void,
        ) -> cl_int,
    >,

    clUnloadPlatformCompiler: Option<fn(platform: cl_platform_id) -> cl_int>,

    clGetProgramInfo: Option<
        fn(
            program: cl_program,
            param_name: cl_program_info,
            param_value_size: size_t,
            param_value: *mut c_void,
            param_value_size_ret: *mut size_t,
        ) -> cl_int,
    >,

    clGetProgramBuildInfo: Option<
        fn(
            program: cl_program,
            device: cl_device_id,
            param_name: cl_program_build_info,
            param_value_size: size_t,
            param_value: *mut c_void,
            param_value_size_ret: *mut size_t,
        ) -> cl_int,
    >,

    // Kernel Object APIs
    clCreateKernel: Option<
        fn(program: cl_program, kernel_name: *const c_char, errcode_ret: *mut cl_int) -> cl_kernel,
    >,

    clCreateKernelsInProgram: Option<
        fn(
            program: cl_program,
            num_kernels: cl_uint,
            kernels: *mut cl_kernel,
            num_kernels_ret: *mut cl_uint,
        ) -> cl_int,
    >,

    clCloneKernel: Option<fn(source_kernel: cl_kernel, errcode_ret: *mut cl_int) -> cl_kernel>,

    clRetainKernel: Option<fn(kernel: cl_kernel) -> cl_int>,

    clReleaseKernel: Option<fn(kernel: cl_kernel) -> cl_int>,

    clSetKernelArg: Option<
        fn(
            kernel: cl_kernel,
            arg_index: cl_uint,
            arg_size: size_t,
            arg_value: *const c_void,
        ) -> cl_int,
    >,

    clSetKernelArgSVMPointer:
        Option<fn(kernel: cl_kernel, arg_index: cl_uint, arg_value: *const c_void) -> cl_int>,

    clSetKernelExecInfo: Option<
        fn(
            kernel: cl_kernel,
            param_name: cl_kernel_exec_info,
            param_value_size: size_t,
            param_value: *const c_void,
        ) -> cl_int,
    >,

    clGetKernelInfo: Option<
        fn(
            kernel: cl_kernel,
            param_name: cl_kernel_info,
            param_value_size: size_t,
            param_value: *mut c_void,
            param_value_size_ret: *mut size_t,
        ) -> cl_int,
    >,

    clGetKernelArgInfo: Option<
        fn(
            kernel: cl_kernel,
            arg_indx: cl_uint,
            param_name: cl_kernel_arg_info,
            param_value_size: size_t,
            param_value: *mut c_void,
            param_value_size_ret: *mut size_t,
        ) -> cl_int,
    >,

    clGetKernelWorkGroupInfo: Option<
        fn(
            kernel: cl_kernel,
            device: cl_device_id,
            param_name: cl_kernel_work_group_info,
            param_value_size: size_t,
            param_value: *mut c_void,
            param_value_size_ret: *mut size_t,
        ) -> cl_int,
    >,

    clGetKernelSubGroupInfo: Option<
        fn(
            kernel: cl_kernel,
            device: cl_device_id,
            param_name: cl_kernel_sub_group_info,
            input_value_size: size_t,
            input_value: *const c_void,
            param_value_size: size_t,
            param_value: *mut c_void,
            param_value_size_ret: *mut size_t,
        ) -> cl_int,
    >,

    // Event Object APIs
    clWaitForEvents: Option<fn(num_events: cl_uint, event_list: *const cl_event) -> cl_int>,

    clGetEventInfo: Option<
        fn(
            event: cl_event,
            param_name: cl_event_info,
            param_value_size: size_t,
            param_value: *mut c_void,
            param_value_size_ret: *mut size_t,
        ) -> cl_int,
    >,

    clCreateUserEvent: Option<fn(context: cl_context, errcode_ret: *mut cl_int) -> cl_event>,

    clRetainEvent: Option<fn(event: cl_event) -> cl_int>,

    clReleaseEvent: Option<fn(event: cl_event) -> cl_int>,

    clSetUserEventStatus: Option<fn(event: cl_event, execution_status: cl_int) -> cl_int>,

    clSetEventCallback: Option<
        fn(
            event: cl_event,
            command_exec_callback_type: cl_int,
            pfn_notify: Option<
                unsafe extern "C" fn(
                    event: cl_event,
                    event_command_status: cl_int,
                    user_data: *mut c_void,
                ),
            >,
            user_data: *mut c_void,
        ) -> cl_int,
    >,

    // Profiling APIs
    clGetEventProfilingInfo: Option<
        fn(
            event: cl_event,
            param_name: cl_profiling_info,
            param_value_size: size_t,
            param_value: *mut c_void,
            param_value_size_ret: *mut size_t,
        ) -> cl_int,
    >,

    // Flush and Finish APIs
    clFlush: Option<fn(command_queue: cl_command_queue) -> cl_int>,

    clFinish: Option<fn(command_queue: cl_command_queue) -> cl_int>,

    // Enqueued Commands APIs
    clEnqueueReadBuffer: Option<
        fn(
            command_queue: cl_command_queue,
            buffer: cl_mem,
            blocking_read: cl_bool,
            offset: size_t,
            cb: size_t,
            ptr: *mut c_void,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clEnqueueReadBufferRect: Option<
        fn(
            command_queue: cl_command_queue,
            buffer: cl_mem,
            blocking_read: cl_bool,
            buffer_origin: *const size_t,
            host_origin: *const size_t,
            region: *const size_t,
            buffer_row_pitch: size_t,
            buffer_slc_pitch: size_t,
            host_row_pitch: size_t,
            host_slc_pitch: size_t,
            ptr: *mut c_void,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clEnqueueWriteBuffer: Option<
        fn(
            command_queue: cl_command_queue,
            buffer: cl_mem,
            blocking_write: cl_bool,
            offset: size_t,
            cb: size_t,
            ptr: *const c_void,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clEnqueueWriteBufferRect: Option<
        fn(
            command_queue: cl_command_queue,
            buffer: cl_mem,
            blocking_write: cl_bool,
            buffer_origin: *const size_t,
            host_origin: *const size_t,
            region: *const size_t,
            buffer_row_pitch: size_t,
            buffer_slc_pitch: size_t,
            host_row_pitch: size_t,
            host_slc_pitch: size_t,
            ptr: *const c_void,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clEnqueueFillBuffer: Option<
        fn(
            command_queue: cl_command_queue,
            buffer: cl_mem,
            pattern: *const c_void,
            pattern_size: size_t,
            offset: size_t,
            size: size_t,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clEnqueueCopyBuffer: Option<
        fn(
            command_queue: cl_command_queue,
            src_buffer: cl_mem,
            dst_buffer: cl_mem,
            src_offset: size_t,
            dst_offset: size_t,
            cb: size_t,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clEnqueueCopyBufferRect: Option<
        fn(
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
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clEnqueueReadImage: Option<
        fn(
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
        ) -> cl_int,
    >,

    clEnqueueWriteImage: Option<
        fn(
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
        ) -> cl_int,
    >,

    clEnqueueFillImage: Option<
        fn(
            command_queue: cl_command_queue,
            image: cl_mem,
            fill_color: *const c_void,
            origin: *const size_t,
            region: *const size_t,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clEnqueueCopyImage: Option<
        fn(
            command_queue: cl_command_queue,
            src_image: cl_mem,
            dst_image: cl_mem,
            src_origin: *const size_t,
            dst_origin: *const size_t,
            region: *const size_t,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clEnqueueCopyImageToBuffer: Option<
        fn(
            command_queue: cl_command_queue,
            src_image: cl_mem,
            dst_buffer: cl_mem,
            src_origin: *const size_t,
            region: *const size_t,
            dst_offset: size_t,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clEnqueueCopyBufferToImage: Option<
        fn(
            command_queue: cl_command_queue,
            src_buffer: cl_mem,
            dst_image: cl_mem,
            src_offset: size_t,
            dst_origin: *const size_t,
            region: *const size_t,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clEnqueueMapBuffer: Option<
        fn(
            command_queue: cl_command_queue,
            buffer: cl_mem,
            blocking_map: cl_bool,
            map_flags: cl_map_flags,
            offset: size_t,
            size: size_t,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
            errcode_ret: *mut cl_int,
        ) -> *mut c_void,
    >,

    clEnqueueMapImage: Option<
        fn(
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
            errcode_ret: *mut cl_int,
        ) -> *mut c_void,
    >,

    clEnqueueUnmapMemObject: Option<
        fn(
            command_queue: cl_command_queue,
            memobj: cl_mem,
            mapped_ptr: *mut c_void,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clEnqueueMigrateMemObjects: Option<
        fn(
            command_queue: cl_command_queue,
            num_mem_objects: cl_uint,
            mem_objects: *const cl_mem,
            flags: cl_mem_migration_flags,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clEnqueueNDRangeKernel: Option<
        fn(
            command_queue: cl_command_queue,
            kernel: cl_kernel,
            work_dim: cl_uint,
            global_work_offset: *const size_t,
            global_work_dims: *const size_t,
            local_work_dims: *const size_t,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clEnqueueNativeKernel: Option<
        fn(
            command_queue: cl_command_queue,
            user_func: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
            args: *mut c_void,
            cb_args: size_t,
            num_mem_objects: cl_uint,
            mem_list: *const cl_mem,
            args_mem_loc: *const *const c_void,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clEnqueueMarkerWithWaitList: Option<
        fn(
            command_queue: cl_command_queue,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clEnqueueBarrierWithWaitList: Option<
        fn(
            command_queue: cl_command_queue,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clEnqueueSVMFree: Option<
        fn(
            command_queue: cl_command_queue,
            num_svm_pointers: cl_uint,
            svm_pointers: *const *const c_void,
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
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clEnqueueSVMMemcpy: Option<
        fn(
            command_queue: cl_command_queue,
            blocking_copy: cl_bool,
            dst_ptr: *mut c_void,
            src_ptr: *const c_void,
            size: size_t,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clEnqueueSVMMemFill: Option<
        fn(
            command_queue: cl_command_queue,
            svm_ptr: *mut c_void,
            pattern: *const c_void,
            pattern_size: size_t,
            size: size_t,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clEnqueueSVMMap: Option<
        fn(
            command_queue: cl_command_queue,
            blocking_map: cl_bool,
            flags: cl_map_flags,
            svm_ptr: *mut c_void,
            size: size_t,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clEnqueueSVMUnmap: Option<
        fn(
            command_queue: cl_command_queue,
            svm_ptr: *mut c_void,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clEnqueueSVMMigrateMem: Option<
        fn(
            command_queue: cl_command_queue,
            num_svm_pointers: cl_uint,
            svm_pointers: *const *const c_void,
            sizes: *const size_t,
            flags: cl_mem_migration_flags,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clGetExtensionFunctionAddressForPlatform:
        Option<fn(platform: cl_platform_id, func_name: *const c_char) -> *mut c_void>,

    // Deprecated OpenCL 1.1 APIs
    clCreateImage2D: Option<
        fn(
            context: cl_context,
            flags: cl_mem_flags,
            image_format: *mut cl_image_format,
            image_width: size_t,
            image_depth: size_t,
            image_row_pitch: size_t,
            host_ptr: *mut c_void,
            errcode_ret: *mut cl_int,
        ) -> cl_mem,
    >,

    clCreateImage3D: Option<
        fn(
            context: cl_context,
            flags: cl_mem_flags,
            image_format: *mut cl_image_format,
            image_width: size_t,
            image_height: size_t,
            image_depth: size_t,
            image_row_pitch: size_t,
            image_slice_pitch: size_t,
            host_ptr: *mut c_void,
            errcode_ret: *mut cl_int,
        ) -> cl_mem,
    >,

    clEnqueueMarker: Option<fn(command_queue: cl_command_queue, event: *mut cl_event) -> cl_int>,

    clEnqueueWaitForEvents: Option<
        fn(
            command_queue: cl_command_queue,
            num_events: cl_uint,
            event_list: *mut cl_event,
        ) -> cl_int,
    >,

    clEnqueueBarrier: Option<fn(command_queue: cl_command_queue) -> cl_int>,

    clUnloadCompiler: Option<fn() -> cl_int>,

    clGetExtensionFunctionAddress: Option<fn(func_name: *const c_char)>,

    // Deprecated OpenCL 2.0 APIs
    clCreateCommandQueue: Option<
        fn(
            context: cl_context,
            device: cl_device_id,
            properties: cl_command_queue_properties,
            errcode_ret: *mut cl_int,
        ) -> cl_command_queue,
    >,

    clCreateSampler: Option<
        fn(
            context: cl_context,
            normalize_coords: cl_bool,
            addressing_mode: cl_addressing_mode,
            filter_mode: cl_filter_mode,
            errcode_ret: *mut cl_int,
        ) -> cl_sampler,
    >,

    // Deprecated 1.2
    clEnqueueTask: Option<
        fn(
            command_queue: cl_command_queue,
            kernel: cl_kernel,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    // Direct3D 10 APIs
    clGetSupportedD3D10TextureFormatsINTEL: Option<
        fn(
            context: cl_context,
            flags: cl_mem_flags,
            image_type: cl_mem_object_type,
            num_entries: cl_uint,
            d3d10_formats: *mut DXGI_FORMAT,
            num_surface_formats: *mut cl_uint,
        ) -> cl_int,
    >,

    // Direct3D 11 APIs
    clGetSupportedD3D11TextureFormatsINTEL: Option<
        fn(
            context: cl_context,
            flags: cl_mem_flags,
            image_type: cl_mem_object_type,
            plane: cl_uint,
            num_entries: cl_uint,
            d3d11_formats: *mut DXGI_FORMAT,
            num_surface_formats: *mut cl_uint,
        ) -> cl_int,
    >,

    // DirectX9 Media Sharing APIs
    clGetDeviceIDsFromDX9INTEL: Option<
        fn(
            platform: cl_platform_id,
            dx9_device_source: cl_dx9_device_source_intel,
            dx9_object: *mut c_void,
            dx9_device_set: cl_dx9_device_set_intel,
            num_entries: cl_uint,
            devices: *mut cl_device_id,
            num_devices: *mut cl_uint,
        ) -> cl_int,
    >,

    clCreateFromDX9MediaSurfaceINTEL: Option<
        fn(
            context: cl_context,
            flags: cl_mem_flags,
            resource: IDirect3DSurface9_ptr,
            sharedHandle: HANDLE,
            plane: cl_uint,
            errcode_ret: *mut cl_int,
        ) -> cl_mem,
    >,

    clEnqueueAcquireDX9ObjectsINTEL: Option<
        fn(
            command_queue: cl_command_queue,
            num_objects: cl_uint,
            mem_objects: *const cl_mem,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clEnqueueReleaseDX9ObjectsINTEL: Option<
        fn(
            command_queue: cl_command_queue,
            num_objects: cl_uint,
            mem_objects: *const cl_mem,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clGetSupportedDX9MediaSurfaceFormatsINTEL: Option<
        fn(
            context: cl_context,
            flags: cl_mem_flags,
            image_type: cl_mem_object_type,
            plane: cl_uint,
            num_entries: cl_uint,
            dx9_formats: *mut D3DFORMAT,
            num_surface_formats: *mut cl_uint,
        ) -> cl_int,
    >,

    // EGL APIs
    clCreateFromEGLImageKHR: Option<
        fn(
            context: cl_context,
            display: CLeglDisplayKHR,
            image: CLeglImageKHR,
            flags: cl_mem_flags,
            properties: *const cl_egl_image_properties_khr,
            errcode_ret: *mut cl_int,
        ) -> cl_mem,
    >,

    clEnqueueAcquireEGLObjectsKHR: Option<
        fn(
            command_queue: cl_command_queue,
            num_objects: cl_uint,
            mem_objects: *const cl_mem,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clEnqueueReleaseEGLObjectsKHR: Option<
        fn(
            command_queue: cl_command_queue,
            num_objects: cl_uint,
            mem_objects: *const cl_mem,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clCreateEventFromEGLSyncKHR: Option<
        fn(
            context: cl_context,
            sync: CLeglSyncKHR,
            display: CLeglDisplayKHR,
            errcode_ret: *mut cl_int,
        ) -> cl_event,
    >,

    // Extensions APIs
    clCreateCommandBufferKHR: Option<
        fn(
            num_queues: cl_uint,
            queues: *const cl_command_queue,
            properties: *const cl_command_buffer_properties_khr,
            errcode_ret: *mut cl_int,
        ) -> cl_command_buffer_khr,
    >,

    clFinalizeCommandBufferKHR: Option<fn(command_buffer: cl_command_buffer_khr) -> cl_int>,

    clRetainCommandBufferKHR: Option<fn(command_buffer: cl_command_buffer_khr) -> cl_int>,

    clReleaseCommandBufferKHR: Option<fn(command_buffer: cl_command_buffer_khr) -> cl_int>,

    clEnqueueCommandBufferKHR: Option<
        fn(
            num_queues: cl_uint,
            queues: *mut cl_command_queue,
            command_buffer: cl_command_buffer_khr,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clCommandBarrierWithWaitListKHR: Option<
        fn(
            command_buffer: cl_command_buffer_khr,
            command_queue: cl_command_queue,
            properties: *const cl_command_properties_khr,
            num_sync_points_in_wait_list: cl_uint,
            sync_point_wait_list: *const cl_sync_point_khr,
            sync_point: *mut cl_sync_point_khr,
            mutable_handle: *mut cl_mutable_command_khr,
        ) -> cl_int,
    >,

    clCommandCopyBufferKHR: Option<
        fn(
            command_buffer: cl_command_buffer_khr,
            command_queue: cl_command_queue,
            properties: *const cl_command_properties_khr,
            src_buffer: cl_mem,
            dst_buffer: cl_mem,
            src_offset: size_t,
            dst_offset: size_t,
            size: size_t,
            num_sync_points_in_wait_list: cl_uint,
            sync_point_wait_list: *const cl_sync_point_khr,
            sync_point: *mut cl_sync_point_khr,
            mutable_handle: *mut cl_mutable_command_khr,
        ) -> cl_int,
    >,

    clCommandCopyBufferRectKHR: Option<
        fn(
            command_buffer: cl_command_buffer_khr,
            command_queue: cl_command_queue,
            properties: *const cl_command_properties_khr,
            src_buffer: cl_mem,
            dst_buffer: cl_mem,
            src_origin: *const size_t,
            dst_origin: *const size_t,
            region: *const size_t,
            src_row_pitch: size_t,
            src_slice_pitch: size_t,
            dst_row_pitch: size_t,
            dst_slice_pitch: size_t,
            num_sync_points_in_wait_list: cl_uint,
            sync_point_wait_list: *const cl_sync_point_khr,
            sync_point: *mut cl_sync_point_khr,
            mutable_handle: *mut cl_mutable_command_khr,
        ) -> cl_int,
    >,

    clCommandCopyBufferToImageKHR: Option<
        fn(
            command_buffer: cl_command_buffer_khr,
            command_queue: cl_command_queue,
            properties: *const cl_command_properties_khr,
            src_buffer: cl_mem,
            dst_image: cl_mem,
            src_offset: size_t,
            dst_origin: *const size_t,
            region: *const size_t,
            num_sync_points_in_wait_list: cl_uint,
            sync_point_wait_list: *const cl_sync_point_khr,
            sync_point: *mut cl_sync_point_khr,
            mutable_handle: *mut cl_mutable_command_khr,
        ) -> cl_int,
    >,

    clCommandCopyImageKHR: Option<
        fn(
            command_buffer: cl_command_buffer_khr,
            command_queue: cl_command_queue,
            properties: *const cl_command_properties_khr,
            src_image: cl_mem,
            dst_image: cl_mem,
            src_origin: *const size_t,
            dst_origin: *const size_t,
            region: *const size_t,
            num_sync_points_in_wait_list: cl_uint,
            sync_point_wait_list: *const cl_sync_point_khr,
            sync_point: *mut cl_sync_point_khr,
            mutable_handle: *mut cl_mutable_command_khr,
        ) -> cl_int,
    >,

    clCommandCopyImageToBufferKHR: Option<
        fn(
            command_buffer: cl_command_buffer_khr,
            command_queue: cl_command_queue,
            properties: *const cl_command_properties_khr,
            src_image: cl_mem,
            dst_buffer: cl_mem,
            src_origin: *const size_t,
            region: *const size_t,
            dst_offset: size_t,
            num_sync_points_in_wait_list: cl_uint,
            sync_point_wait_list: *const cl_sync_point_khr,
            sync_point: *mut cl_sync_point_khr,
            mutable_handle: *mut cl_mutable_command_khr,
        ) -> cl_int,
    >,

    clCommandFillBufferKHR: Option<
        fn(
            command_buffer: cl_command_buffer_khr,
            command_queue: cl_command_queue,
            properties: *const cl_command_properties_khr,
            buffer: cl_mem,
            pattern: *const c_void,
            pattern_size: size_t,
            offset: size_t,
            size: size_t,
            num_sync_points_in_wait_list: cl_uint,
            sync_point_wait_list: *const cl_sync_point_khr,
            sync_point: *mut cl_sync_point_khr,
            mutable_handle: *mut cl_mutable_command_khr,
        ) -> cl_int,
    >,

    clCommandFillImageKHR: Option<
        fn(
            command_buffer: cl_command_buffer_khr,
            command_queue: cl_command_queue,
            properties: *const cl_command_properties_khr,
            image: cl_mem,
            fill_color: *const c_void,
            origin: *const size_t,
            region: *const size_t,
            num_sync_points_in_wait_list: cl_uint,
            sync_point_wait_list: *const cl_sync_point_khr,
            sync_point: *mut cl_sync_point_khr,
            mutable_handle: *mut cl_mutable_command_khr,
        ) -> cl_int,
    >,

    clCommandNDRangeKernelKHR: Option<
        fn(
            command_buffer: cl_command_buffer_khr,
            command_queue: cl_command_queue,
            properties: *const cl_command_properties_khr,
            kernel: cl_kernel,
            work_dim: cl_uint,
            global_work_offset: *const size_t,
            global_work_size: *const size_t,
            local_work_size: *const size_t,
            num_sync_points_in_wait_list: cl_uint,
            sync_point_wait_list: *const cl_sync_point_khr,
            sync_point: *mut cl_sync_point_khr,
            mutable_handle: *mut cl_mutable_command_khr,
        ) -> cl_int,
    >,

    clCommandSVMMemcpyKHR: Option<
        fn(
            command_buffer: cl_command_buffer_khr,
            command_queue: cl_command_queue,
            properties: *const cl_command_properties_khr,
            dst_ptr: *mut c_void,
            src_ptr: *const c_void,
            size: size_t,
            num_sync_points_in_wait_list: cl_uint,
            sync_point_wait_list: *const cl_sync_point_khr,
            sync_point: *mut cl_sync_point_khr,
            mutable_handle: *mut cl_mutable_command_khr,
        ) -> cl_int,
    >,

    clCommandSVMMemFillKHR: Option<
        fn(
            command_buffer: cl_command_buffer_khr,
            command_queue: cl_command_queue,
            properties: *const cl_command_properties_khr,
            svm_ptr: *mut c_void,
            pattern: *const c_void,
            pattern_size: size_t,
            size: size_t,
            num_sync_points_in_wait_list: cl_uint,
            sync_point_wait_list: *const cl_sync_point_khr,
            sync_point: *mut cl_sync_point_khr,
            mutable_handle: *mut cl_mutable_command_khr,
        ) -> cl_int,
    >,

    clGetCommandBufferInfoKHR: Option<
        fn(
            command_buffer: cl_command_buffer_khr,
            param_name: cl_command_buffer_info_khr,
            param_value_size: size_t,
            param_value: *mut c_void,
            param_value_size_ret: *mut size_t,
        ) -> cl_int,
    >,

    clRemapCommandBufferKHR: Option<
        fn(
            command_buffer: cl_command_buffer_khr,
            automatic: cl_bool,
            num_queues: cl_uint,
            queues: *const cl_command_queue,
            num_handles: cl_uint,
            handles: *const cl_mutable_command_khr,
            handles_ret: *mut cl_mutable_command_khr,
            errcode_ret: *mut cl_int,
        ) -> cl_command_buffer_khr,
    >,

    clUpdateMutableCommandsKHR: Option<
        fn(
            command_buffer: cl_command_buffer_khr,
            num_configs: cl_uint,
            config_types: *const cl_command_buffer_update_type_khr,
            configs: *mut *const c_void,
        ) -> cl_int,
    >,

    clGetMutableCommandInfoKHR: Option<
        fn(
            command: cl_mutable_command_khr,
            param_name: cl_mutable_command_info_khr,
            param_value_size: size_t,
            param_value: *mut c_void,
            param_value_size_ret: *mut size_t,
        ) -> cl_int,
    >,

    clSetMemObjectDestructorAPPLE: Option<
        fn(
            memobj: cl_mem,
            pfn_notify: Option<unsafe extern "C" fn(memobj: cl_mem, user_data: *mut c_void)>,
            user_data: *mut c_void,
        ) -> cl_int,
    >,

    clLogMessagesToSystemLogAPPLE: Option<
        fn(errstr: *const c_char, private_info: *const c_void, cb: size_t, user_data: *mut c_void),
    >,

    clLogMessagesToStdoutAPPLE: Option<
        fn(errstr: *const c_char, private_info: *const c_void, cb: size_t, user_data: *mut c_void),
    >,

    clLogMessagesToStderrAPPLE: Option<
        fn(errstr: *const c_char, private_info: *const c_void, cb: size_t, user_data: *mut c_void),
    >,

    clIcdGetPlatformIDsKHR: Option<
        fn(
            num_entries: cl_uint,
            platforms: *mut cl_platform_id,
            num_platforms: *mut cl_uint,
        ) -> cl_int,
    >,

    clIcdGetFunctionAddressForPlatformKHR: Option<
        fn(
            platform: cl_platform_id,
            func_name: *const c_char,
        ) -> *mut c_void,
    >,

    clIcdSetPlatformDispatchDataKHR: Option<
        fn(
            platform: cl_platform_id,
            dispatch_data: *mut c_void,
        ) -> cl_int,
    >,

    clCreateProgramWithILKHR: Option<
        fn(
            context: cl_context,
            il: *const c_void,
            length: size_t,
            errcode_ret: *mut cl_int,
        ) -> cl_program,
    >,

    clTerminateContextKHR: Option<fn(context: cl_context) -> cl_int>,

    clCreateCommandQueueWithPropertiesKHR: Option<
        fn(
            context: cl_context,
            device: cl_device_id,
            properties: *const cl_queue_properties_khr,
            errcode_ret: *mut cl_int,
        ) -> cl_command_queue,
    >,

    clReleaseDeviceEXT: Option<fn(device: cl_device_id) -> cl_int>,

    clRetainDeviceEXT: Option<fn(device: cl_device_id) -> cl_int>,

    clCreateSubDevicesEXT: Option<
        fn(
            in_device: cl_device_id,
            properties: *const cl_device_partition_property_ext,
            num_entries: cl_uint,
            out_devices: *mut cl_device_id,
            num_devices: *mut cl_uint,
        ) -> cl_int,
    >,

    clEnqueueMigrateMemObjectEXT: Option<
        fn(
            command_queue: cl_command_queue,
            num_mem_objects: cl_uint,
            mem_objects: *const cl_mem,
            flags: cl_mem_migration_flags_ext,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clGetDeviceImageInfoQCOM: Option<
        fn(
            device: cl_device_id,
            image_width: size_t,
            image_height: size_t,
            image_format: *const cl_image_format,
            param_name: cl_image_pitch_info_qcom,
            param_value_size: size_t,
            param_value: *mut c_void,
            param_value_size_ret: *mut size_t,
        ) -> cl_int,
    >,

    clEnqueueAcquireGrallocObjectsIMG: Option<
        fn(
            command_queue: cl_command_queue,
            num_objects: cl_uint,
            mem_objects: *const cl_mem,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clEnqueueReleaseGrallocObjectsIMG: Option<
        fn(
            command_queue: cl_command_queue,
            num_objects: cl_uint,
            mem_objects: *const cl_mem,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clEnqueueGenerateMipmapIMG: Option<
        fn(
            command_queue: cl_command_queue,
            src_image: cl_mem,
            dst_image: cl_mem,
            mipmap_filter_mode: cl_mipmap_filter_mode_img,
            array_region: *const size_t,
            mip_region: *const size_t,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clGetKernelSubGroupInfoKHR: Option<
        fn(
            in_kernel: cl_kernel,
            in_device: cl_device_id,
            param_name: cl_kernel_sub_group_info,
            input_value_size: size_t,
            input_value: *const c_void,
            param_value_size: size_t,
            param_value: *mut c_void,
            param_value_size_ret: *mut size_t,
        ) -> cl_int,
    >,

    clGetKernelSuggestedLocalWorkSizeKHR: Option<
        fn(
            command_queue: cl_command_queue,
            kernel: cl_kernel,
            work_dim: cl_uint,
            global_work_offset: *const size_t,
            global_work_size: *const size_t,
            suggested_local_work_size: *mut size_t,
        ) -> cl_int,
    >,

    clEnqueueAcquireExternalMemObjectsKHR: Option<
        fn(
            command_queue: cl_command_queue,
            num_mem_objects: cl_uint,
            mem_objects: *const cl_mem,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clEnqueueReleaseExternalMemObjectsKHR: Option<
        fn(
            command_queue: cl_command_queue,
            num_mem_objects: cl_uint,
            mem_objects: *const cl_mem,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clGetSemaphoreHandleForTypeKHR: Option<
        fn(
            sema_object: cl_semaphore_khr,
            device: cl_device_id,
            handle_type: cl_external_semaphore_handle_type_khr,
            handle_size: size_t,
            handle_ptr: *mut c_void,
            handle_size_ret: *mut size_t,
        ) -> cl_int,
    >,

    clReImportSemaphoreSyncFdKHR: Option<
        fn(
            sema_object: cl_semaphore_khr,
            reimport_props: *mut cl_semaphore_reimport_properties_khr,
            fd: c_int,
        ) -> cl_int,
    >,

    clCreateSemaphoreWithPropertiesKHR: Option<
        fn(
            context: cl_context,
            sema_props: *const cl_semaphore_properties_khr,
            errcode_ret: *mut cl_int,
        ) -> cl_semaphore_khr,
    >,

    clEnqueueWaitSemaphoresKHR: Option<
        fn(
            command_queue: cl_command_queue,
            num_sema_objects: cl_uint,
            sema_objects: *const cl_semaphore_khr,
            sema_payload_list: *const cl_semaphore_payload_khr,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clEnqueueSignalSemaphoresKHR: Option<
        fn(
            command_queue: cl_command_queue,
            num_sema_objects: cl_uint,
            sema_objects: *const cl_semaphore_khr,
            sema_payload_list: *const cl_semaphore_payload_khr,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clGetSemaphoreInfoKHR: Option<
        fn(
            sema_object: cl_semaphore_khr,
            param_name: cl_semaphore_info_khr,
            param_value_size: size_t,
            param_value: *mut c_void,
            param_value_size_ret: *mut size_t,
        ) -> cl_int,
    >,

    clReleaseSemaphoreKHR: Option<fn(sema_object: cl_semaphore_khr) -> cl_int>,

    clRetainSemaphoreKHR: Option<fn(sema_object: cl_semaphore_khr) -> cl_int>,

    clImportMemoryARM: Option<
        fn(
            context: cl_context,
            flags: cl_mem_flags,
            properties: *const cl_import_properties_arm,
            memory: *mut c_void,
            size: size_t,
            errcode_ret: *mut cl_int,
        ) -> cl_mem,
    >,

    clSVMAllocARM: Option<
        fn(
            context: cl_context,
            flags: cl_svm_mem_flags_arm,
            size: size_t,
            alignment: cl_uint,
        ) -> *mut c_void,
    >,

    clSVMFreeARM: Option<fn(context: cl_context, svm_pointer: *mut c_void)>,

    clEnqueueSVMFreeARM: Option<
        fn(
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
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clEnqueueSVMMemcpyARM: Option<
        fn(
            command_queue: cl_command_queue,
            blocking_copy: cl_bool,
            dst_ptr: *mut c_void,
            src_ptr: *const c_void,
            size: size_t,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clEnqueueSVMMemFillARM: Option<
        fn(
            command_queue: cl_command_queue,
            svm_ptr: *mut c_void,
            pattern: *const c_void,
            pattern_size: size_t,
            size: size_t,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clEnqueueSVMMapARM: Option<
        fn(
            command_queue: cl_command_queue,
            blocking_map: cl_bool,
            flags: cl_map_flags,
            svm_ptr: *mut c_void,
            size: size_t,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clEnqueueSVMUnmapARM: Option<
        fn(
            command_queue: cl_command_queue,
            svm_ptr: *mut c_void,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clSetKernelArgSVMPointerARM:
        Option<fn(kernel: cl_kernel, arg_index: cl_uint, arg_value: *const c_void) -> cl_int>,

    clSetKernelExecInfoARM: Option<
        fn(
            kernel: cl_kernel,
            param_name: cl_kernel_exec_info_arm,
            param_value_size: size_t,
            param_value: *const c_void,
        ) -> cl_int,
    >,

    clCreateAcceleratorINTEL: Option<
        fn(
            context: cl_context,
            accelerator_type: cl_accelerator_type_intel,
            descriptor_size: size_t,
            descriptor: *const c_void,
            errcode_ret: *mut cl_int,
        ) -> cl_accelerator_intel,
    >,

    clGetAcceleratorInfoINTEL: Option<
        fn(
            accelerator: cl_accelerator_intel,
            param_name: cl_accelerator_info_intel,
            param_value_size: size_t,
            param_value: *mut c_void,
            param_value_size_ret: *mut size_t,
        ) -> cl_int,
    >,

    clRetainAcceleratorINTEL: Option<fn(accelerator: cl_accelerator_intel) -> cl_int>,

    clReleaseAcceleratorINTEL: Option<fn(accelerator: cl_accelerator_intel) -> cl_int>,

    clHostMemAllocINTEL: Option<
        fn(
            context: cl_context,
            properties: *const cl_mem_properties_intel,
            size: size_t,
            alignment: cl_uint,
            errcode_ret: *mut cl_int,
        ) -> *mut c_void,
    >,

    clDeviceMemAllocINTEL: Option<
        fn(
            context: cl_context,
            device: cl_device_id,
            properties: *const cl_mem_properties_intel,
            size: size_t,
            alignment: cl_uint,
            errcode_ret: *mut cl_int,
        ) -> *mut c_void,
    >,

    clSharedMemAllocINTEL: Option<
        fn(
            context: cl_context,
            device: cl_device_id,
            properties: *const cl_mem_properties_intel,
            size: size_t,
            alignment: cl_uint,
            errcode_ret: *mut cl_int,
        ) -> *mut c_void,
    >,

    clMemFreeINTEL: Option<fn(context: cl_context, ptr: *mut c_void) -> cl_int>,

    clMemBlockingFreeINTEL: Option<fn(context: cl_context, ptr: *mut c_void) -> cl_int>,

    clGetMemAllocInfoINTEL: Option<
        fn(
            context: cl_context,
            ptr: *const c_void,
            param_name: cl_mem_info_intel,
            param_value_size: size_t,
            param_value: *mut c_void,
            param_value_size_ret: *mut size_t,
        ) -> cl_int,
    >,

    clSetKernelArgMemPointerINTEL:
        Option<fn(kernel: cl_kernel, arg_index: cl_uint, arg_value: *const c_void) -> cl_int>,

    clEnqueueMemFillINTEL: Option<
        fn(
            command_queue: cl_command_queue,
            dst_ptr: *mut c_void,
            pattern: *const c_void,
            pattern_size: size_t,
            size: size_t,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clEnqueueMemcpyINTEL: Option<
        fn(
            command_queue: cl_command_queue,
            blocking: cl_bool,
            dst_ptr: *mut c_void,
            src_ptr: *const c_void,
            size: size_t,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clEnqueueMemAdviseINTEL: Option<
        fn(
            command_queue: cl_command_queue,
            ptr: *const c_void,
            size: size_t,
            advice: cl_mem_advice_intel,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clEnqueueMigrateMemINTEL: Option<
        fn(
            command_queue: cl_command_queue,
            ptr: *const c_void,
            size: size_t,
            flags: cl_mem_migration_flags,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clEnqueueMemsetINTEL: Option<
        fn(
            command_queue: cl_command_queue,
            dst_ptr: *mut c_void,
            value: cl_int,
            size: size_t,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clCreateBufferWithPropertiesINTEL: Option<
        fn(
            context: cl_context,
            properties: *const cl_mem_properties_intel,
            flags: cl_mem_flags,
            size: size_t,
            host_ptr: *mut c_void,
            errcode_ret: *mut cl_int,
        ) -> cl_mem,
    >,

    clEnqueueReadHostPipeINTEL: Option<
        fn(
            queue: cl_command_queue,
            program: cl_program,
            pipe_symbol: *const c_char,
            blocking_read: cl_bool,
            ptr: *mut c_void,
            size: size_t,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clEnqueueWriteHostPipeINTEL: Option<
        fn(
            queue: cl_command_queue,
            program: cl_program,
            pipe_symbol: *const c_char,
            blocking_write: cl_bool,
            ptr: *const c_void,
            size: size_t,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clGetImageRequirementsInfoEXT: Option<
        fn(
            context: cl_context,
            properties: *const cl_mem_properties,
            flags: cl_mem_flags,
            image_format: *const cl_image_format,
            image_desc: *const cl_image_desc,
            param_name: cl_image_requirements_info_ext,
            param_value_size: size_t,
            param_value: *mut c_void,
            param_value_size_ret: *mut size_t,
        ) -> cl_int,
    >,

    clGetICDLoaderInfoOCLICD: Option<
        fn(
            param_name: cl_icdl_info,
            param_value_size: size_t,
            param_value: *mut c_void,
            param_value_size_ret: *mut size_t,
        ) -> cl_int,
    >,

    clSetContentSizeBufferPoCL: Option<fn(buffer: cl_mem, content_size_buffer: cl_mem) -> cl_int>,

    clCancelCommandsIMG:
        Option<fn(event_list: *const cl_event, num_events_in_list: cl_uint) -> cl_int>,

    clSetPerfHintQCOM:
        Option<fn(context: cl_context, perf_hint: cl_perf_hint_qcom) -> cl_int>,

    // OpenGL APIs
    clCreateFromGLBuffer: Option<
        fn(
            context: cl_context,
            flags: cl_mem_flags,
            bufobj: cl_GLuint,
            errcode_ret: *mut cl_int,
        ) -> cl_mem,
    >,

    clCreateFromGLTexture: Option<
        fn(
            context: cl_context,
            flags: cl_mem_flags,
            target: cl_GLenum,
            miplevel: cl_GLint,
            texture: cl_GLuint,
            errcode_ret: *mut cl_int,
        ) -> cl_mem,
    >,

    clCreateFromGLRenderbuffer: Option<
        fn(
            context: cl_context,
            flags: cl_mem_flags,
            renderbuffer: cl_GLuint,
            errcode_ret: *mut cl_int,
        ) -> cl_mem,
    >,

    clGetGLObjectInfo: Option<
        fn(
            memobj: cl_mem,
            gl_object_type: *mut cl_gl_object_type,
            gl_object_name: *mut cl_GLuint,
        ) -> cl_int,
    >,

    clGetGLTextureInfo: Option<
        fn(
            memobj: cl_mem,
            param_name: cl_gl_texture_info,
            param_value_size: size_t,
            param_value: *mut c_void,
            param_value_size_ret: *mut size_t,
        ) -> cl_int,
    >,

    clEnqueueAcquireGLObjects: Option<
        fn(
            command_queue: cl_command_queue,
            num_objects: cl_uint,
            mem_objects: *const cl_mem,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    clEnqueueReleaseGLObjects: Option<
        fn(
            command_queue: cl_command_queue,
            num_objects: cl_uint,
            mem_objects: *const cl_mem,
            num_events_in_wait_list: cl_uint,
            event_wait_list: *const cl_event,
            event: *mut cl_event,
        ) -> cl_int,
    >,

    // Deprecated OpenCL 1.1 APIs
    clCreateFromGLTexture2D: Option<
        fn(
            context: cl_context,
            flags: cl_mem_flags,
            texture_target: cl_GLenum,
            miplevel: cl_GLint,
            texture: cl_GLuint,
            errcode_ret: *mut cl_int,
        ) -> cl_mem,
    >,

    clCreateFromGLTexture3D: Option<
        fn(
            context: cl_context,
            flags: cl_mem_flags,
            texture_target: cl_GLenum,
            miplevel: cl_GLint,
            texture: cl_GLuint,
            errcode_ret: *mut cl_int,
        ) -> cl_mem,
    >,

    clGetGLContextInfoKHR: Option<
        fn(
            properties: *const cl_context_properties,
            param_name: cl_gl_context_info,
            param_value_size: size_t,
            param_value: *mut c_void,
            param_value_size_ret: *mut size_t,
        ) -> cl_int,
    >,

    clCreateEventFromGLsyncKHR:
        Option<fn(context: cl_context, sync: cl_GLsync, errcode_ret: *mut cl_int) -> cl_event>,

    clGetSupportedGLTextureFormatsINTEL: Option<
        fn(
            context: cl_context,
            flags: cl_mem_flags,
            image_type: cl_mem_object_type,
            num_entries: cl_uint,
            gl_formats: *mut cl_GLenum,
            num_texture_formats: *mut cl_uint,
        ) -> cl_int,
    >,

    // Layer APIs
    clGetLayerInfo: Option<
        fn(
            param_name: cl_layer_info,
            param_value_size: size_t,
            param_value: *mut c_void,
            param_value_size_ret: *mut size_t,
        ) -> cl_int,
    >,

    clInitLayer: Option<
        fn(
            num_entries: cl_uint,
            target_dispatch: *const cl_icd_dispatch,
            num_entries_ret: *mut cl_uint,
            layer_dispatch_ret: *mut *const cl_icd_dispatch,
        ) -> cl_int,
    >,
}
