// Copyright (c) 2020-2021 Via Technology Ltd. All Rights Reserved.
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

use crate::types::{cl_image_format, cl_int, cl_name_version, cl_uchar, cl_uint, cl_ulong};
use libc::{intptr_t, size_t};
use std::ffi::{CString, NulError};

/// A Rust enum to handle OpenCL API "Info" function return types.  
/// It provides functions to extract each data type from the enum.  
/// The functions will panic if they are called for the incorrect type.
#[derive(Debug)]
pub enum InfoType {
    Int(cl_int),
    Uint(cl_uint),
    Ulong(cl_ulong),
    Size(size_t),
    Ptr(intptr_t),
    VecUchar(Vec<cl_uchar>),
    VecUlong(Vec<cl_ulong>),
    VecSize(Vec<size_t>),
    VecIntPtr(Vec<intptr_t>),
    VecNameVersion(Vec<cl_name_version>),
    VecImageFormat(Vec<cl_image_format>),
    VecVecUchar(Vec<Vec<cl_uchar>>),
}

impl InfoType {
    pub fn to_str(self) -> Result<CString, NulError> {
        let mut a = self.to_vec_uchar();

        // remove all trailing nulls if any
        while let Some(0) = a.last() {
            a.pop();
        }

        // convert remaining nulls (if any) to spaces
        const SPACE: u8 = 32;
        let b: Vec<u8> = a.iter().map(|x| if *x != 0 { *x } else { SPACE }).collect();

        CString::new(b)
    }

    pub fn to_int(self) -> cl_int {
        match self {
            InfoType::Int(a) => a,
            _ => panic!("not a cl_int"),
        }
    }

    pub fn to_uint(self) -> cl_uint {
        match self {
            InfoType::Uint(a) => a,
            _ => panic!("not a cl_uint"),
        }
    }

    pub fn to_ulong(self) -> cl_ulong {
        match self {
            InfoType::Ulong(a) => a,
            _ => panic!("not a cl_ulong"),
        }
    }

    pub fn to_size(self) -> size_t {
        match self {
            InfoType::Size(a) => a,
            _ => panic!("not a size_t"),
        }
    }

    pub fn to_ptr(self) -> intptr_t {
        match self {
            InfoType::Ptr(a) => a,
            _ => panic!("not a intptr_t"),
        }
    }

    pub fn to_vec_uchar(self) -> Vec<cl_uchar> {
        match self {
            InfoType::VecUchar(a) => a,
            _ => panic!("not a Vec<cl_uchar>"),
        }
    }

    pub fn to_vec_ulong(self) -> Vec<cl_ulong> {
        match self {
            InfoType::VecUlong(a) => a,
            _ => panic!("not a Vec<cl_ulong>"),
        }
    }

    pub fn to_vec_size(self) -> Vec<size_t> {
        match self {
            InfoType::VecSize(a) => a,
            _ => panic!("not a Vec<size_t>"),
        }
    }

    pub fn to_vec_intptr(self) -> Vec<intptr_t> {
        match self {
            InfoType::VecIntPtr(a) => a,
            _ => panic!("not a Vec<intptr_t>"),
        }
    }

    pub fn to_vec_name_version(self) -> Vec<cl_name_version> {
        match self {
            InfoType::VecNameVersion(a) => a,
            _ => panic!("not a Vec<cl_name_version>"),
        }
    }

    pub fn to_vec_image_format(self) -> Vec<cl_image_format> {
        match self {
            InfoType::VecImageFormat(a) => a,
            _ => panic!("not a Vec<cl_image_format>"),
        }
    }

    pub fn to_vec_vec_uchar(self) -> Vec<Vec<cl_uchar>> {
        match self {
            InfoType::VecVecUchar(a) => a,
            _ => panic!("not a Vec<Vec<cl_uchar>"),
        }
    }
}
