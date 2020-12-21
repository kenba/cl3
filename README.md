# cl3

A Rust implementation of the Khronos [OpenCL](https://www.khronos.org/registry/OpenCL/) 3.0 API.

# Description

The [ffi](src/ffi) modules provide unsafe Foreign Function Interfaces for the OpenCL
C functions, while [types](src/type.rs) contains all the OpenCL API data types.

Most of the other modules are named after their equivalent OpenCL objects
and provide simple, safe functions around the C API functions that return
Rust Result types. The exceptions are:

* [error_codes](src/error_codes.rs) - contains the OpenCL API error codes from
cl.h that are returned in the OpenCL API Result types.
* [info_type](src/info_type.rs) - contains a Rust enum (`InfoType`) to hold the
OpenCL types that can be returned from OpenCL "Info" functions, e.g.
clGetPlatformInfo, clGetDeviceInfo, clGetProgramInfo, etc.
* [macros](src/macros.rs) - contains Rust macros to call the OpenCL "Info"
functions and return the appropriate `InfoType` in a Rust Result.
