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

//! `OpenCL` API constants.

pub use crate::runtime::OpenClConstants::*;

pub mod cl_d3d11 {
    pub use crate::runtime::OpenClConstants::cl_d3d11::*;
}

pub mod cl_dx9_media_sharing {
    pub use crate::runtime::OpenClConstants::cl_dx9_media_sharing::*;
}

pub mod cl_egl {
    pub use crate::runtime::OpenClConstants::cl_egl::*;
}

pub mod cl_ext {
    #[cfg(not(feature = "dynamic_runtime"))]
    pub use crate::runtime::OpenClConstants::*;

    #[cfg(feature = "dynamic_runtime")]
    pub use crate::runtime::OpenClConstants::cl_ext::*;
}

pub mod cl_gl {
    #[cfg(not(feature = "dynamic_runtime"))]
    pub use crate::runtime::OpenClConstants::*;

    #[cfg(feature = "dynamic_runtime")]
    pub use crate::runtime::OpenClConstants::cl_gl::*;
}

pub mod cl_layer {
    pub use crate::runtime::OpenClConstants::cl_layer::*;
}
