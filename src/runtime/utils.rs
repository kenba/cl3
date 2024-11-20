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

use std::sync::OnceLock;

use dlopen2::{wrapper::Container, Error};

use super::OpenCl;

/// `dlopen2` container with all loaded API functions.
pub type OpenClRuntime = Container<OpenCl>;

static OPENCL_RUNTIME: OnceLock<Result<OpenClRuntime, Error>> = OnceLock::new();

/// Utility function to load the `OpenCL` shared library (actual load will be performed only once).
///
/// Returns an error if the library is not found.
pub fn load_library() -> &'static Result<OpenClRuntime, Error> {
    const LIBRARY_NAME: &str = if cfg!(target_os = "windows") {
        "OpenCL.dll"
    } else if cfg!(target_os = "macos") {
        "/System/Library/Frameworks/OpenCL.framework/OpenCL"
    } else {
        "libOpenCL.so"
    };

    OPENCL_RUNTIME.get_or_init(|| {
        if let Ok(env_var) = std::env::var("OPENCL_DYLIB_PATH") {
            for library_path in env_var.split(';') {
                let library = unsafe { Container::load(library_path) };
                if library.is_ok() {
                    return library;
                }
            }
        }

        unsafe { Container::load(LIBRARY_NAME) }
    })
}

/// Utility function to check if the `OpenCL` shared library is loaded successfully.
#[must_use]
pub fn is_opencl_runtime_available() -> bool {
    load_library().is_ok()
}
