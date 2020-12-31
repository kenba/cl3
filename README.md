# cl3

[![crates.io](https://img.shields.io/crates/v/cl3.svg)](https://crates.io/crates/cl3)
[![docs.io](https://docs.rs/cl3/badge.svg)](https://docs.rs/cl3/)
[![OpenCL 3.0](https://img.shields.io/badge/OpenCL-3.0-blue.svg)](https://www.khronos.org/registry/OpenCL/)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://github.com/kenba/cl3/workflows/Rust/badge.svg)](https://github.com/kenba/cl3/actions)

A Rust adapter for the Khronos [OpenCL](https://www.khronos.org/registry/OpenCL/) API.

# Description

A functional, safe Rust interface to the Khronos OpenCL 3.0
[C API](https://github.com/KhronosGroup/OpenCL-Headers/blob/master/CL/cl.h)
based upon the [cl-sys](https://crates.io/crates/cl-sys) OpenCL FFI bindings.

[OpenCL 3.0](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html)
is a unified specification that adds little new functionality to previous OpenCL versions.  
It specifies that all **OpenCL 1.2** features are **mandatory**, while all
OpenCL 2.x and 3.0 features are now optional.

## Design

This crate applies the [adapter pattern](https://en.wikipedia.org/wiki/Adapter_pattern)
to convert OpenCL C API functions into Rust functions that return a
[Result](https://doc.rust-lang.org/std/result/) containing the desired result of
the C function or the OpenCL error code.
The only exception is `svm_free`, which just provides a safe wrapper for the
`clSVMFree` C API function.

Most of the modules are named after their equivalent "API" sections in
[cl.h](https://github.com/KhronosGroup/OpenCL-Headers/blob/master/CL/cl.h).
They contain Rust adapter functions for the OpenCL API C functions defined
in those sections with their associated types and constants.  
For more information see the Rust [documentation](https://docs.rs/cl3/).

## OpenCL Installation

There are two parts of OpenCL to install: an OpenCL hardware driver and an
OpenCL Installable Client Driver (ICD).

OpenCL hardware drivers are normally bundled together with the graphics drivers
for the hardware, while there are several options for installing an OpenCL ICD.

AMD support OpenCL 2.2 through their Radeon Open Compute (ROCm) development
platform. Unfortunately, it only supports a limited number of Linux distributions,
see [AMD ROCm Platform](https://rocmdocs.amd.com/en/latest/).

AMD used to provide the de facto standard OpenCL ICD in AMD APP SDK 3.0 which
provided OpenCL 2.0 support on both Linux and Windows. Unfortunately, it is
no longer available from AMD, although it can be found elsewhere on the internet, see
[OpenCL AMD APP SDK 3.0 for windows and linux](https://stackoverflow.com/questions/53070673/download-opencl-amd-app-sdk-3-0-for-windows-and-linux).  

Note: on Windows 10 systems with both an AMD Radeon GPU and an Intel GPU,
OpenCL is often limited to the AMD GPU only, see
[How to Enable Intel OpenCL Support on Windows when AMD Radeon Graphics Driver is Installed](https://www.geeks3d.com/20181220/how-to-enable-intel-opencl-support-on-windows-when-amd-radeon-graphics-driver-is-installed/)
for a description of the issue and how to fix it.

The Intel OpenCL ICD currently provides good OpenCL support (version 2.1)
for both Linux and Windows, see [Intel SDK for OpenCL applications](https://software.intel.com/content/www/us/en/develop/tools/opencl-sdk/choose-download.html).
Intel also provides a useful guide to OpenCL development, see
[Get Started with Intel SDK for OpenCL](https://software.intel.com/content/www/us/en/develop/articles/sdk-for-opencl-2019-gsg.html).

Nvidia also provides an [OpenCL ICD](https://developer.nvidia.com/opencl).
However, Nvidia OpenCL support has lagged behind AMD and Intel in the past,
so it is recommended to install the Nvidia graphics drivers with an AMD or Intel
OpenCL ICD, depending on your CPU manufacturer and operating system.

Other OpenCL ICDs are available. For example, [cl-sys](https://crates.io/crates/cl-sys)
searches for the [OCLSDK_Light](https://github.com/GPUOpen-LibrariesAndSDKs/OCL-SDK/releases)
on Windows if it can't find any of the AMD, Intel or Nvidia OpenCL ICDs.

Finally, it's possible to build your own OpenCL ICD for Linux or Windows from the
[Khronos official OpenCL ICD Loader](https://github.com/KhronosGroup/OpenCL-ICD-Loader) source code.

## Use

See the Rust crate documentation: [cl3](https://docs.rs/cl3/).

## License

Licensed under the Apache License, Version 2.0, as per Khronos Group OpenCL.  
You may obtain a copy of the License at: http://www.apache.org/licenses/LICENSE-2.0

OpenCL and the OpenCL logo are trademarks of Apple Inc. used under license by Khronos.
