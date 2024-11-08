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

//! `OpenCL` Runtime.

#[cfg(feature = "dynamic_runtime")]
pub(crate) mod dynamic;

pub fn is_opencl_runtime_available() -> bool {
    if cfg!(feature = "static_runtime") {
        true
    } else if cfg!(feature = "dynamic_runtime") {
        crate::runtime::dynamic::load_runtime().is_ok()
    } else {
        false
    }
}

macro_rules! cl_call {
    ($func:ident($($arg:expr),* $(,)?)) => {{
        if cfg!(feature = "static_runtime") {
            opencl_sys::$func($($arg),*)
        } else if cfg!(feature = "dynamic_runtime") {
            crate::runtime::dynamic::load_runtime()?.$func($($arg),*)
        } else {
            $func($($arg),*)
        }
    }}
}
