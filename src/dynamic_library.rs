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

//! `OpenCL` dynamic library function call.

use crate::error_codes::DLOPEN_RUNTIME_LOAD_FAILED;
use crate::runtime::{load_library, OpenClRuntime};

pub fn load_dynamic_runtime() -> Result<&'static OpenClRuntime, i32> {
    load_library()
        .as_ref()
        .map_err(|_| DLOPEN_RUNTIME_LOAD_FAILED)
}

macro_rules! cl_call {
    ($func:ident($($arg:expr),* $(,)?)) => {{
        if let Some(result) = $crate::dynamic_library::load_dynamic_runtime()?.$func($($arg),*) {
            result
        } else {
            return Err($crate::error_codes::DLOPEN_FUNCTION_NOT_AVAILABLE)
        }
    }};
    ($namespace:ident::$func:ident($($arg:expr),* $(,)?)) => {{
        cl_call!($func($($arg),*))
    }}
}
