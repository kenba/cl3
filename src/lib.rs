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
//! A functional, safe Rust interface to the Khronos OpenCL 3.0
//! [C API](https://github.com/KhronosGroup/OpenCL-Headers/blob/master/CL/cl.h)
//! based upon the [cl-sys](https://crates.io/crates/cl-sys) OpenCL FFI bindings.
//!
//! [OpenCL 3.0](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html)
//! is a unified specification that adds little new functionality to previous OpenCL versions.  
//! It specifies that all **OpenCL 1.2** features are **mandatory**, while all
//! OpenCL 2.x and 3.0 features are now optional.
//!
//! ## Design
//!
//! This crate applies the [adapter pattern](https://en.wikipedia.org/wiki/Adapter_pattern)
//! to convert OpenCL C API functions into Rust functions that return a
//! [Result](https://doc.rust-lang.org/std/result/) containing the desired result of
//! the C function or the OpenCL error code.
//! The exception is `svm_free`, which just provides a safe wrapper for the
//! `clSVMFree` C API function.
//!
//! Most of the modules are named after their equivalent OpenCL "API" sections in
//! [cl.h](https://github.com/KhronosGroup/OpenCL-Headers/blob/master/CL/cl.h).
//! They contain Rust adapter functions for the OpenCL API C functions defined
//! in those sections with their associated types and constants. The exceptions are:
//!
//! * [error_codes] - contains the OpenCL API error codes from cl.h and a function
//! (`error_text`) to convert an error code to it's enum name from cl.h.
//! * [info_type] - contains a Rust enum (`InfoType`) to hold the OpenCL types
//! that can be returned from OpenCL "Info" functions, e.g. clGetPlatformInfo,
//! clGetDeviceInfo, clGetProgramInfo, etc.
//! * [macros] - contains Rust macros to call the OpenCL "Info" functions and
//! return the appropriate `InfoType` in a Rust Result.
//!
//! It is vital to call the correct `InfoType` method type when decoding the
//! result of "Info" functions, since the methods will panic if called with the
//! wrong type, see [info_type].
//!
//! # Use
//!
//! Ensure that an OpenCL ICD and the appropriate OpenCL hardware driver(s)
//! are installed, see [cl3](https://github.com/kenba/cl3).  
//!
//! `cl3` supports OpenCL 1.2 and 2.0 ICD loaders by default. If you have an
//! OpenCL 2.0 ICD loader then add the following to your project's `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! cl3 = "0.1"
//! ```
//!
//! If your OpenCL ICD loader supports higher versions of OpenCL then add the
//! appropriate features to cl3, e.g. for an OpenCL 2.2 ICD loader add the
//! following to your project's `Cargo.toml` instead:
//!
//! ```toml
//! [dependencies.cl3]
//! version = "0.1"
//! features = ["CL_VERSION_2_1", "CL_VERSION_2_2"]
//! ```
//!
//! Whichever version of OpenCL ICD loader you use, add the following to your
//! crate root (`lib.rs` or `main.rs`):
//!
//! ```rust
//! extern crate cl3;
//! ```
//!
//! ## Tests
//!
//! The crate contains unit, documentation and integration tests.  
//! The tests run the platform and device info functions (among others) so they
//! can provide useful information about OpenCL capabilities of the system.
//!
//! It is recommended to run the tests in single-threaded mode, since some of
//! them can interfere with each other when run multi-threaded, e.g.:
//!
//! ```shell
//! cargo test -- --test-threads=1 --show-output
//! ```
//!
//! The integration tests are marked `ignore` so use the following command to
//! run them:
//!
//! ```shell
//! cargo test -- --test-threads=1 --show-output --ignored
//! ```
//!
//! ## Examples
//!
//! The tests provide examples of how the crate may be used, e.g. see:
//! [platform], [device], [context] and
//! [integration_test](https://github.com/kenba/cl3/tests/integration_test.rs).
//!
//! ## License
//! 
//! Licensed under the Apache License, Version 2.0, as per Khronos Group OpenCL.  
//! You may obtain a copy of the License at:
//! [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0)
//! 
//! OpenCL and the OpenCL logo are trademarks of Apple Inc. used under license by Khronos.

extern crate cl_sys;

pub mod command_queue;
pub mod context;
pub mod device;
pub mod error_codes;
pub mod event;
pub mod info_type;
pub mod kernel;
pub mod macros;
pub mod memory;
pub mod platform;
pub mod program;
pub mod sampler;
pub mod types;
