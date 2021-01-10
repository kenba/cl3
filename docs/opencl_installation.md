# OpenCL Installation

There are two parts of OpenCL to install: an OpenCL hardware driver and an
OpenCL Installable Client Driver (ICD).

OpenCL hardware drivers are normally bundled together with the graphics drivers
for the hardware, while there are several options for installing an OpenCL ICD.

Note: you do not need to install an OpenCL ICD from the same manufacturer as your hardware. In general, the more up to date the OpenCL ICD, the better.

## AMD ROCm

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

## Intel oneAPI

The Intel OpenCL ICD currently provides good OpenCL support (version 2.1)
for both Linux and Windows, see [Intel SDK for OpenCL applications](https://software.intel.com/content/www/us/en/develop/tools/opencl-sdk/choose-download.html).
Intel also provides a useful guide to OpenCL development, see
[Get Started with Intel SDK for OpenCL](https://software.intel.com/content/www/us/en/develop/articles/sdk-for-opencl-2019-gsg.html).

## Nvidia

Nvidia also provides an [OpenCL ICD](https://developer.nvidia.com/opencl).
However, Nvidia OpenCL support has lagged behind AMD and Intel in the past,
so it is recommended to install the Nvidia graphics drivers with an AMD or Intel
OpenCL ICD, depending on your CPU manufacturer and operating system.

## Others

Other OpenCL ICDs are available. For example, [cl-sys](https://crates.io/crates/cl-sys)
searches for the [OCLSDK_Light](https://github.com/GPUOpen-LibrariesAndSDKs/OCL-SDK/releases)
on Windows if it can't find any of the AMD, Intel or Nvidia OpenCL ICDs.

Finally, it's possible to build your own OpenCL ICD for Linux or Windows from the
[Khronos official OpenCL ICD Loader](https://github.com/KhronosGroup/OpenCL-ICD-Loader) source code.