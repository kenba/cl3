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

//! A Rust implementation of the Khronos [OpenCL](https://www.khronos.org/registry/OpenCL/) 3.0 API.
//!
//! # Description
//!
//! The [ffi] modules provide unsafe Foreign Function Interfaces for the OpenCL
//! C functions, while [types] contains all the OpenCL API data types.
//!
//! Most of the other modules are named after their equivalent OpenCL objects
//! and provide simple, safe functions around the C API functions that return
//! Rust Result types. The exceptions are:
//!
//! * [error_codes] - contains the OpenCL API error codes from cl.h that are
//! returned in the OpenCL API Result types.
//! * [info_type] - contains a Rust enum (`InfoType`) to hold the OpenCL types
//! that can be returned from OpenCL "Info" functions, e.g. clGetPlatformInfo,
//! clGetDeviceInfo, clGetProgramInfo, etc.
//! * [macros] - contains Rust macros to call the OpenCL "Info" functions and
//! return the appropriate `InfoType` in a Rust Result.

pub mod command_queue;
pub mod context;
pub mod device;
pub mod error_codes;
pub mod event;
pub mod ffi;
pub mod info_type;
pub mod kernel;
pub mod macros;
pub mod memory;
pub mod platform;
pub mod program;
pub mod sampler;
pub mod types;
