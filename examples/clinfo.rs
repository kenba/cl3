// Copyright (c) 2021 Via Technology Ltd. All Rights Reserved.
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

use cl3::device::{
    device_type_text, get_device_ids, get_device_info, vendor_id_text, CL_DEVICE_BUILT_IN_KERNELS,
    CL_DEVICE_EXTENSIONS, CL_DEVICE_NAME, CL_DEVICE_OPENCL_C_VERSION, CL_DEVICE_PROFILE,
    CL_DEVICE_SVM_CAPABILITIES, CL_DEVICE_TYPE, CL_DEVICE_TYPE_ALL, CL_DEVICE_VENDOR,
    CL_DEVICE_VENDOR_ID, CL_DEVICE_VERSION,
};
use cl3::platform::{
    get_platform_ids, get_platform_info, CL_PLATFORM_EXTENSIONS, CL_PLATFORM_NAME,
    CL_PLATFORM_PROFILE, CL_PLATFORM_VENDOR, CL_PLATFORM_VERSION,
};
use cl3::types::{cl_int, cl_uint, cl_ulong};

/// Finds all the OpenCL platforms and devices on a system.
///
/// It displays OpenCL platform information from `clGetPlatformInfo` and
/// OpenCL device information from `clGetDeviceInfo` for all the platforms and
/// devices.
fn main() -> Result<(), cl_int> {
    let platforms = get_platform_ids()?;
    println!("Number of platforms: {}", platforms.len());

    for platform_id in platforms {
        println!(
            "CL_PLATFORM_VENDOR: {}",
            String::from(get_platform_info(platform_id, CL_PLATFORM_VENDOR)?)
        );
        println!(
            "CL_PLATFORM_NAME: {}",
            String::from(get_platform_info(platform_id, CL_PLATFORM_NAME)?)
        );
        println!(
            "CL_PLATFORM_VERSION: {}",
            String::from(get_platform_info(platform_id, CL_PLATFORM_VERSION)?)
        );
        println!(
            "CL_PLATFORM_PROFILE: {}",
            String::from(get_platform_info(platform_id, CL_PLATFORM_PROFILE)?)
        );
        println!(
            "CL_PLATFORM_EXTENSIONS: {}",
            String::from(get_platform_info(platform_id, CL_PLATFORM_EXTENSIONS)?)
        );

        let devices = get_device_ids(platform_id, CL_DEVICE_TYPE_ALL)?;
        println!("Number of devices: {}", devices.len());
        println!();
        for device_id in devices {
            println!(
                "\tCL_DEVICE_VENDOR: {}",
                String::from(get_device_info(device_id, CL_DEVICE_VENDOR)?)
            );
            let vendor_id: cl_uint = get_device_info(device_id, CL_DEVICE_VENDOR_ID)?.into();
            println!(
                "\tCL_DEVICE_VENDOR_ID: {:X}, {}",
                vendor_id,
                vendor_id_text(vendor_id)
            );
            println!(
                "\tCL_DEVICE_NAME: {}",
                String::from(get_device_info(device_id, CL_DEVICE_NAME)?)
            );
            println!(
                "\tCL_DEVICE_VERSION: {}",
                String::from(get_device_info(device_id, CL_DEVICE_VERSION)?)
            );
            let device_type: cl_ulong = get_device_info(device_id, CL_DEVICE_TYPE)?.into();
            println!(
                "\tCL_DEVICE_TYPE: {:X}, {}",
                device_type,
                device_type_text(device_type)
            );
            println!(
                "\tCL_DEVICE_PROFILE: {}",
                String::from(get_device_info(device_id, CL_DEVICE_PROFILE)?)
            );
            println!(
                "\tCL_DEVICE_EXTENSIONS: {}",
                String::from(get_device_info(device_id, CL_DEVICE_EXTENSIONS)?)
            );
            println!(
                "\tCL_DEVICE_OPENCL_C_VERSION: {:?}",
                String::from(get_device_info(device_id, CL_DEVICE_OPENCL_C_VERSION)?)
            );

            println!(
                "\tCL_DEVICE_BUILT_IN_KERNELS: {}",
                String::from(get_device_info(device_id, CL_DEVICE_BUILT_IN_KERNELS)?)
            );
            println!(
                "\tCL_DEVICE_SVM_CAPABILITIES: {:X}",
                cl_ulong::from(get_device_info(device_id, CL_DEVICE_SVM_CAPABILITIES)?)
            );

            println!();
        }
    }

    Ok(())
}
