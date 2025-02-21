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

use libc::{intptr_t, size_t};
use opencl_sys::{
    CL_LUID_SIZE_KHR, CL_UUID_SIZE_KHR, cl_image_format, cl_int, cl_name_version, cl_uchar,
    cl_uint, cl_ulong,
};
use std::fmt;

/// A Rust enum to handle `OpenCL` API "Info" function return types.
/// Each of the data types may be extracted from the enum using its associated
/// From trait or `to_*` function.
///
/// # Panics
///
/// The From traits and `to_*` functions will panic if they are called for the
/// incorrect data type.
#[derive(Debug)]
pub enum InfoType {
    Int(i32),
    Uint(u32),
    Ulong(u64),
    Size(usize),
    Ptr(isize),
    Luid([u8; CL_LUID_SIZE_KHR]),
    Uuid([u8; CL_UUID_SIZE_KHR]),
    VecUchar(Vec<u8>),
    VecUshort(Vec<u32>),
    VecUlong(Vec<u64>),
    VecSize(Vec<usize>),
    VecIntPtr(Vec<isize>),
    VecNameVersion(Vec<cl_name_version>),
    VecImageFormat(Vec<cl_image_format>),
    VecVecUchar(Vec<Vec<u8>>),
}

/// A macro to help create the `InfoType` From traits.
macro_rules! match_info_type {
    ($value:expr, $variant:path) => {
        match $value {
            $variant(x) => x,
            _ => panic!("value is not an {}", stringify!($variant)),
        }
    };
}

impl From<InfoType> for i32 {
    fn from(value: InfoType) -> Self {
        match_info_type!(value, InfoType::Int)
    }
}

impl From<InfoType> for u32 {
    fn from(value: InfoType) -> Self {
        match_info_type!(value, InfoType::Uint)
    }
}

impl From<InfoType> for u64 {
    fn from(value: InfoType) -> Self {
        match_info_type!(value, InfoType::Ulong)
    }
}

impl From<InfoType> for usize {
    fn from(value: InfoType) -> Self {
        match_info_type!(value, InfoType::Size)
    }
}

impl From<InfoType> for isize {
    fn from(value: InfoType) -> Self {
        match_info_type!(value, InfoType::Ptr)
    }
}

impl From<InfoType> for [u8; CL_LUID_SIZE_KHR] {
    fn from(value: InfoType) -> Self {
        match_info_type!(value, InfoType::Luid)
    }
}

impl From<InfoType> for [u8; CL_UUID_SIZE_KHR] {
    fn from(value: InfoType) -> Self {
        match_info_type!(value, InfoType::Uuid)
    }
}

impl From<InfoType> for Vec<u8> {
    fn from(value: InfoType) -> Self {
        match_info_type!(value, InfoType::VecUchar)
    }
}

impl From<InfoType> for Vec<u32> {
    fn from(value: InfoType) -> Self {
        match_info_type!(value, InfoType::VecUshort)
    }
}

impl From<InfoType> for Vec<u64> {
    fn from(value: InfoType) -> Self {
        match_info_type!(value, InfoType::VecUlong)
    }
}

impl From<InfoType> for Vec<usize> {
    fn from(value: InfoType) -> Self {
        match_info_type!(value, InfoType::VecSize)
    }
}

impl From<InfoType> for Vec<isize> {
    fn from(value: InfoType) -> Self {
        match_info_type!(value, InfoType::VecIntPtr)
    }
}

impl From<InfoType> for Vec<cl_name_version> {
    fn from(value: InfoType) -> Self {
        match_info_type!(value, InfoType::VecNameVersion)
    }
}

impl From<InfoType> for Vec<cl_image_format> {
    fn from(value: InfoType) -> Self {
        match_info_type!(value, InfoType::VecImageFormat)
    }
}

impl From<InfoType> for Vec<Vec<u8>> {
    fn from(value: InfoType) -> Self {
        match_info_type!(value, InfoType::VecVecUchar)
    }
}

impl From<InfoType> for String {
    /// Get a `Vec<cl_uchar>` aka `Vec<u8>` as a String.
    /// Note: it uses `from_utf8_lossy` to convert any invalid characters to
    /// `std::char::REPLACEMENT_CHARACTER`.
    ///
    /// returns a utf8 String.
    fn from(info_type: InfoType) -> Self {
        let mut a = Vec::<u8>::from(info_type);

        // remove all trailing nulls, if any
        while a.last() == Some(&0) {
            a.pop();
        }

        // convert invalid characters to std::char::REPLACEMENT_CHARACTER
        Self::from_utf8_lossy(&a).into_owned()
    }
}

impl fmt::Display for InfoType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::VecUchar(a) => {
                let b = String::from_utf8_lossy(a).into_owned();
                write!(f, "{b}")
            }

            // Formats a LUID the same way as `clinfo`.
            // See: https://github.com/Oblomov/clinfo/blob/master/src/clinfo.c
            Self::Luid(a) => {
                write!(
                    f,
                    "{:x}{:x}-{:x}{:x}{:x}{:x}{:x}{:x}",
                    a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7]
                )
            }

            // Formats a UUID according to RFC4122.
            Self::Uuid(a) => {
                write!(
                    f,
                    "{:x}{:x}{:x}{:x}-{:x}{:x}-{:x}{:x}-{:x}{:x}-{:x}{:x}{:x}{:x}{:x}{:x}",
                    a[0],
                    a[1],
                    a[2],
                    a[3],
                    a[4],
                    a[5],
                    a[6],
                    a[7],
                    a[8],
                    a[9],
                    a[10],
                    a[11],
                    a[12],
                    a[13],
                    a[14],
                    a[15],
                )
            }

            Self::VecNameVersion(a) => {
                let mut s = String::default();
                for b in a {
                    s.push('\n');

                    s.push_str(&b.version.to_string());
                    s.push_str(": ");
                    s.push_str(&String::from_utf8_lossy(&b.name));
                }

                write!(f, "{s}")
            }

            Self::VecImageFormat(a) => {
                let mut s = String::default();

                for b in a {
                    s.push('\n');

                    s.push_str(&b.image_channel_order.to_string());
                    s.push_str(": ");
                    s.push_str(&b.image_channel_data_type.to_string());
                }

                write!(f, "{s}")
            }

            // Note: underlying type may not be a vector of Strings.
            // If so use Debug trait instead
            Self::VecVecUchar(a) => {
                let mut s = String::default();
                for b in a {
                    s.push('\n');
                    s.push_str(&String::from_utf8_lossy(b));
                }

                write!(f, "{s}")
            }

            _ => panic!("not a Displayable type, use Debug instead"),
        }
    }
}

impl InfoType {
    #[must_use]
    pub fn to_int(self) -> cl_int {
        i32::from(self)
    }

    #[must_use]
    pub fn to_uint(self) -> cl_uint {
        u32::from(self)
    }

    #[must_use]
    pub fn to_ulong(self) -> cl_ulong {
        u64::from(self)
    }

    #[must_use]
    pub fn to_size(self) -> size_t {
        usize::from(self)
    }

    #[must_use]
    pub fn to_ptr(self) -> intptr_t {
        isize::from(self)
    }

    #[must_use]
    pub fn to_luid(self) -> [u8; CL_LUID_SIZE_KHR] {
        self.into()
    }

    #[must_use]
    pub fn to_uuid(self) -> [u8; CL_UUID_SIZE_KHR] {
        self.into()
    }

    #[must_use]
    pub fn to_vec_uchar(self) -> Vec<cl_uchar> {
        Vec::<u8>::from(self)
    }

    #[must_use]
    pub fn to_vec_ulong(self) -> Vec<cl_ulong> {
        Vec::<u64>::from(self)
    }

    #[must_use]
    pub fn to_vec_size(self) -> Vec<size_t> {
        Vec::<usize>::from(self)
    }

    #[must_use]
    pub fn to_vec_intptr(self) -> Vec<intptr_t> {
        Vec::<isize>::from(self)
    }

    #[must_use]
    pub fn to_vec_name_version(self) -> Vec<cl_name_version> {
        Vec::<cl_name_version>::from(self)
    }

    #[must_use]
    pub fn to_vec_image_format(self) -> Vec<cl_image_format> {
        Vec::<cl_image_format>::from(self)
    }

    #[must_use]
    pub fn to_vec_vec_uchar(self) -> Vec<Vec<cl_uchar>> {
        Vec::<Vec<u8>>::from(self)
    }
}
#[cfg(test)]
mod tests {
    use crate::device::{
        CL_DEVICE_MAX_WORK_ITEM_SIZES, CL_DEVICE_NAME, CL_DEVICE_PARTITION_PROPERTIES,
        CL_DEVICE_TYPE, CL_DEVICE_TYPE_ALL, CL_DEVICE_VENDOR_ID, CL_DRIVER_VERSION, get_device_ids,
        get_device_info,
    };
    use crate::platform::{
        CL_PLATFORM_NAME, CL_PLATFORM_VERSION, get_platform_ids, get_platform_info,
    };

    #[test]
    fn test_debug_display_info() {
        let platform_ids = get_platform_ids().unwrap();
        println!("Number of platforms: {}", platform_ids.len());
        assert!(0 < platform_ids.len());

        // Choose the first platform
        let platform_id = platform_ids[0];

        // Test Display trait
        let value = get_platform_info(platform_id, CL_PLATFORM_NAME).unwrap();
        println!("CL_PLATFORM_NAME: {}", value);

        let value = get_platform_info(platform_id, CL_PLATFORM_VERSION).unwrap();
        println!("CL_PLATFORM_VERSION: {}", value);

        let device_ids = get_device_ids(platform_id, CL_DEVICE_TYPE_ALL).unwrap();
        println!("Platform[0]->number of devices: {}", device_ids.len());
        assert!(0 < device_ids.len());

        // Choose the first device
        let device_id = device_ids[0];

        let value = get_device_info(device_id, CL_DEVICE_NAME).unwrap();
        println!("CL_DEVICE_NAME: {}", value);

        let value = get_device_info(device_id, CL_DRIVER_VERSION).unwrap();
        println!("CL_DRIVER_VERSION: {}", value);

        // Test Debug trait
        let value = get_device_info(device_id, CL_DEVICE_TYPE).unwrap();
        println!("CL_DEVICE_TYPE: {:?}", value);

        let value = get_device_info(device_id, CL_DEVICE_VENDOR_ID).unwrap();
        println!("CL_DEVICE_VENDOR_ID: {:?}", value);

        let value = get_device_info(device_id, CL_DEVICE_MAX_WORK_ITEM_SIZES).unwrap();
        println!("CL_DEVICE_MAX_WORK_ITEM_SIZES len: {:?}", value);

        let value = get_device_info(device_id, CL_DEVICE_PARTITION_PROPERTIES).unwrap();
        println!("CL_DEVICE_PARTITION_PROPERTIES: {:?}", value);
    }
}
