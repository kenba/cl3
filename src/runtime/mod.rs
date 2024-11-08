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

#[macro_use]
#[cfg(feature = "static_runtime")]
mod static_runtime;

#[macro_use]
#[cfg(feature = "dynamic_runtime")]
pub(crate) mod dynamic_runtime;
#[cfg(feature = "dynamic_runtime")]
pub use dynamic_runtime::is_opencl_runtime_available;
