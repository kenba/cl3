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

#![allow(non_camel_case_types, unused_macros)]

#[macro_export]
macro_rules! api_info_size {
    ($func:ident, $api:ident) => {
        fn $func(object: *mut c_void, param_name: cl_uint) -> Result<size_t, cl_int> {
            // Get the size of the information.
            let mut size: size_t = 0;
            let status = unsafe { $api(object, param_name, 0, ptr::null_mut(), &mut size) };
            if CL_SUCCESS != status {
                Err(status)
            } else {
                Ok(size)
            }
        }
    };
}

#[macro_export]
macro_rules! api_info_value {
    ($func:ident, $ty:tt, $api:ident) => {
        fn $func(object: *mut c_void, param_name: cl_uint) -> Result<$ty, cl_int> {
            // Get the size of the data type.
            let size: size_t = mem::size_of::<$ty>();
            let mut data: $ty = $ty::default();
            let data_ptr: *mut $ty = &mut data;
            let status = unsafe {
                $api(
                    object,
                    param_name,
                    size,
                    data_ptr.cast::<c_void>(),
                    ptr::null_mut(),
                )
            };
            if CL_SUCCESS != status {
                Err(status)
            } else {
                Ok(data)
            }
        }
    };
}

#[allow(clippy::uninit_vec)]
#[macro_export]
macro_rules! api_info_vector {
    ($func:ident, $ty:tt, $api:ident) => {
        fn $func(
            object: *mut c_void,
            param_name: cl_uint,
            size: size_t,
        ) -> Result<Vec<$ty>, cl_int> {
            if 0 < size {
                let count = size / mem::size_of::<$ty>();
                let mut data: Vec<$ty> = Vec::with_capacity(count);
                let status = unsafe {
                    data.set_len(count);
                    $api(
                        object,
                        param_name,
                        size,
                        data.as_mut_ptr().cast::<c_void>(),
                        ptr::null_mut(),
                    )
                };
                if CL_SUCCESS != status {
                    Err(status)
                } else {
                    Ok(data)
                }
            } else {
                Ok(Vec::default())
            }
        }
    };
}

#[macro_export]
macro_rules! api2_info_size {
    ($func:ident, $type:tt, $api:ident) => {
        fn $func(object: *mut c_void, idx: $type, param_name: cl_uint) -> Result<size_t, cl_int> {
            // Get the size of the information.
            let mut size: size_t = 0;
            let status = unsafe { $api(object, idx, param_name, 0, ptr::null_mut(), &mut size) };
            if CL_SUCCESS != status {
                Err(status)
            } else {
                Ok(size)
            }
        }
    };
}

#[macro_export]
macro_rules! api2_info_value {
    ($func:ident, $type:tt, $ty:tt, $api:ident) => {
        fn $func(object: *mut c_void, idx: $type, param_name: cl_uint) -> Result<$ty, cl_int> {
            // Get the size of the data type.
            let size: size_t = mem::size_of::<$ty>();
            let mut data: $ty = $ty::default();
            let data_ptr: *mut $ty = &mut data;
            let status = unsafe {
                $api(
                    object,
                    idx,
                    param_name,
                    size,
                    data_ptr.cast::<c_void>(),
                    ptr::null_mut(),
                )
            };
            if CL_SUCCESS != status {
                Err(status)
            } else {
                Ok(data)
            }
        }
    };
}

#[allow(clippy::uninit_vec)]
#[macro_export]
macro_rules! api2_info_vector {
    ($func:ident, $type:tt, $ty:tt, $api:ident) => {
        fn $func(
            object: *mut c_void,
            idx: $type,
            param_name: cl_uint,
            size: size_t,
        ) -> Result<Vec<$ty>, cl_int> {
            if 0 < size {
                let count = size / mem::size_of::<$ty>();
                let mut data: Vec<$ty> = Vec::with_capacity(count);
                let status = unsafe {
                    data.set_len(count);
                    $api(
                        object,
                        idx,
                        param_name,
                        size,
                        data.as_mut_ptr().cast::<c_void>(),
                        ptr::null_mut(),
                    )
                };
                if CL_SUCCESS != status {
                    Err(status)
                } else {
                    Ok(data)
                }
            } else {
                Ok(Vec::default())
            }
        }
    };
}
