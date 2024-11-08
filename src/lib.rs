// Copyright (c) 2020-2024 Via Technology Ltd.
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

//! [![crates.io](https://img.shields.io/crates/v/cl3.svg)](https://crates.io/crates/cl3)
//! [![docs.io](https://docs.rs/cl3/badge.svg)](https://docs.rs/cl3/)
//! [![OpenCL 3.0](https://img.shields.io/badge/OpenCL-3.0-blue.svg)](https://www.khronos.org/registry/OpenCL/)
//! [![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
//! ![Rust](https://github.com/kenba/cl3/workflows/Rust/badge.svg)
//!
//! A Rust adapter for the Khronos [OpenCL](https://www.khronos.org/registry/OpenCL/) API.
//!
//! # Description
//!
//! A functional, safe Rust interface to the Khronos `OpenCL 3.0`
//! [C API](https://github.com/KhronosGroup/OpenCL-Headers/blob/master/CL/cl.h)
//! based upon the [opencl-sys](https://crates.io/crates/opencl-sys) `OpenCL` FFI bindings.
//!
//! [OpenCL 3.0](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html)
//! is a unified specification that adds little new functionality to previous `OpenCL` versions.
//! It specifies that all `OpenCL 1.2` features are **mandatory**, while all
//! `OpenCL 2.x` and `OpenCL 3.0` features are now optional.
//!
//! ## Design
//!
//! This crate applies the [adapter pattern](https://en.wikipedia.org/wiki/Adapter_pattern)
//! to convert `OpenCL` C API functions into Rust functions that return a
//! [Result](https://doc.rust-lang.org/std/result/) containing the desired result of
//! the C function or the `OpenCL` error code.
//! The exception is `svm_free`, which just provides a safe wrapper for the
//! `clSVMFree` C API function.
//!
//! The API for `OpenCL` versions and extensions are controlled by Rust features such as
//! "`CL_VERSION_2_0`" and "`cl_khr_gl_sharing`". To enable an `OpenCL` version, the feature
//! for that version and **all** previous `OpenCL` versions must be enabled,
//! e.g. for "`CL_VERSION_2_0`"; "`CL_VERSION_1_1`" and "`CL_VERSION_1_2`" must also be enabled.
//!
//! The default features are "`CL_VERSION_1_1`" and "`CL_VERSION_1_2`".
//!
//! Rust deprecation warnings are given for `OpenCL` API functions that are
//! deprecated by an enabled `OpenCL` version e.g., `clCreateCommandQueue` is
//! deprecated whenever "`CL_VERSION_2_0`" is enabled.
//!
//! Most of the modules are named after their equivalent `OpenCL` "API" sections in
//! [cl.h](https://github.com/KhronosGroup/OpenCL-Headers/blob/master/CL/cl.h).
//! They contain Rust adapter functions for the `OpenCL` API C functions defined
//! in those sections with their associated types and constants. The exceptions are:
//!
//! * [`error_codes`] - contains the `OpenCL` API error codes from cl.h and a function
//! (`error_text`) to convert an error code to it's enum name from cl.h.
//! * [`info_type`] - contains a Rust enum (`InfoType`) to hold the `OpenCL` types
//! that can be returned from `OpenCL` "Info" functions, e.g. clGetPlatformInfo,
//! clGetDeviceInfo, clGetProgramInfo, etc.
//! * [`macros`] - contains Rust macros to call the `OpenCL` "Info" functions and
//! return the appropriate `InfoType` in a Rust Result.
//!
//! It is vital to call the correct `InfoType` method type when decoding the
//! result of "Info" functions, since the methods will panic if called with the
//! wrong type, see [`info_type`].
//!
//! # Use
//!
//! See [cl3](https://crates.io/crates/cl3).
//!
//! ## License
//!
//! Licensed under the Apache License, Version 2.0, as per Khronos Group `OpenCL`.
//! You may obtain a copy of the License at: <http://www.apache.org/licenses/LICENSE-2.0>
//!
//! `OpenCL` and the `OpenCL` logo are trademarks of Apple Inc. used under license by Khronos.

extern crate opencl_sys;

#[macro_use]
mod runtime;
pub use runtime::is_opencl_runtime_available;

pub mod command_queue;
pub mod context;
pub mod d3d10;
pub mod d3d11;
pub mod device;
pub mod dx9_media_sharing;
pub mod egl;
pub mod error_codes;
pub mod event;
pub mod ext;
pub mod gl;
pub mod info_type;
pub mod kernel;
#[cfg(feature = "cl_loader_layers")]
pub mod layer;
pub mod macros;
pub mod memory;
pub mod platform;
pub mod program;
pub mod sampler;
pub mod types;
