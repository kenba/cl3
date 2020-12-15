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

//! A Rust implementation of the [Khronos OpenCL 3.0 API](https://www.khronos.org/registry/OpenCL/).
//!
//! # Description
//!
//! The `types` and `functions` modules provide unsafe Foreign Function Interfaces
//! (ffi) for the OpenCL C API types and functions.  
//! The other modules wrap the ffi in a functional, safe, idiomatic Rust API.

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
