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
use std::fmt;

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

impl fmt::Display for InfoType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InfoType::VecUchar(a) => {
                let b = String::from_utf8_lossy(a).into_owned();
                write!(f, "{}", b)
            }

            InfoType::VecNameVersion(a) => {
                let mut s = String::default();
                for b in a.iter() {
                    s.push_str("\n");

                    s.push_str(&b.version.to_string());
                    s.push_str(": ");
                    s.push_str(&String::from_utf8_lossy(&b.name).into_owned());
                }

                write!(f, "{}", s)
            }

            InfoType::VecImageFormat(a) => {
                let mut s = String::default();

                for b in a.iter() {
                    s.push_str("\n");

                    s.push_str(&b.image_channel_order.to_string());
                    s.push_str(": ");
                    s.push_str(&b.image_channel_data_type.to_string());
                }

                write!(f, "{}", s)
            }

            InfoType::VecVecUchar(a) => {
                let mut s = String::default();
                for b in a.iter() {
                    s.push_str("\n");
                    s.push_str(&String::from_utf8_lossy(b).into_owned());
                }

                write!(f, "{}", s)
            }

            _ => panic!("not a Displayable type, use Debug instead"),
        }
    }
}

impl InfoType {
    /// Get a `Vec<cl_uchar>` aka `Vec<u8>` as a String.
    /// Note: it uses from_utf8_lossy to convert any invalid characters to
    /// std::char::REPLACEMENT_CHARACTER.
    ///
    /// returns a utf8 String.
    pub fn to_string(self) -> String {
        let mut a = self.to_vec_uchar();

        // remove all trailing nulls, if any
        while let Some(0) = a.last() {
            a.pop();
        }

        // convert invalid characters to std::char::REPLACEMENT_CHARACTER
        String::from_utf8_lossy(&a).into_owned()
    }

    #[deprecated(since = "0.1.8", note = "Please use the to_string function instead")]
    pub fn to_str(self) -> Result<CString, NulError> {
        let mut a = self.to_vec_uchar();

        // remove all trailing nulls if any
        while let Some(0) = a.last() {
            a.pop();
        }

        // convert remaining nulls (if any) to spaces
        const SPACE: u8 = 32;
        for elem in a.iter_mut() {
            if *elem == 0 {
                *elem = SPACE
            }
        }

        CString::new(a)
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

impl From<InfoType> for cl_int {
    fn from(info_type: InfoType) -> Self {
        info_type.to_int()
    }
}

impl From<InfoType> for cl_uint {
    fn from(info_type: InfoType) -> Self {
        info_type.to_uint()
    }
}

impl From<InfoType> for cl_ulong {
    fn from(info_type: InfoType) -> Self {
        info_type.to_ulong()
    }
}

impl From<InfoType> for size_t {
    fn from(info_type: InfoType) -> Self {
        info_type.to_size()
    }
}

#[cfg(test)]
mod tests {
    use crate::device::*;
    use crate::platform::*;

    #[test]
    fn test_debug_display_info() {
        let platform_ids = get_platform_ids().unwrap();
        println!("Number of platforms: {}", platform_ids.len());
        assert!(0 < platform_ids.len());

        // Choose the first platform
        let platform_id = platform_ids[0];

        // Test Display trait
        let value = get_platform_info(platform_id, PlatformInfo::CL_PLATFORM_NAME).unwrap();
        println!("CL_PLATFORM_NAME: {}", value);

        let value = get_platform_info(platform_id, PlatformInfo::CL_PLATFORM_VERSION).unwrap();
        println!("CL_PLATFORM_VERSION: {}", value);

        let device_ids = get_device_ids(platform_id, CL_DEVICE_TYPE_ALL).unwrap();
        println!("Platform[0]->number of devices: {}", device_ids.len());
        assert!(0 < device_ids.len());

        // Choose the first device
        let device_id = device_ids[0];

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_NAME).unwrap();
        println!("CL_DEVICE_NAME: {}", value);

        let value = get_device_info(device_id, DeviceInfo::CL_DRIVER_VERSION).unwrap();
        println!("CL_DRIVER_VERSION: {}", value);

        // Test Debug trait
        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_TYPE).unwrap();
        println!("CL_DEVICE_TYPE: {:?}", value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_VENDOR_ID).unwrap();
        println!("CL_DEVICE_VENDOR_ID: {:?}", value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_MAX_WORK_ITEM_SIZES).unwrap();
        println!("CL_DEVICE_MAX_WORK_ITEM_SIZES len: {:?}", value);

        let value = get_device_info(device_id, DeviceInfo::CL_DEVICE_PARTITION_PROPERTIES).unwrap();
        println!("CL_DEVICE_PARTITION_PROPERTIES: {:?}", value);
    }
}
