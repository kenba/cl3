# Releases

## Version 0.5.0 (2021-10-02)

### Breaking Changes

* Issue [#11](https://github.com/kenba/cl3/issues/11) InfoType::to_string shadows the implementation of Display. Note: the Display::to_string implementation does not remove trailing nulls, see PR [#1](Remove all trailing nulls from OpenCL API info strings).

### New Features

* Issue [#12](https://github.com/kenba/cl3/issues/12) Use From trait for `conversions.

## Version 0.4.4 (2021-09-19)

### New Features

* Return `uints` and `ulongs` for `cl_intel_device_attribute_query` after testing on Intel hardware, for Issue [#10](https://github.com/kenba/cl3/issues/10).

## Version 0.4.3 (2021-09-18)

### New Features

* Issue [#10](https://github.com/kenba/cl3/issues/10) Update for latest OpenCL-Headers: `cl_khr_integer_dot_product` and `cl_intel_device_attribute_query`.
* Fix more `clippy` warnings.
* Add CONTRIBUTING and CODE_OF_CONDUCT documents.

## Version 0.4.2 (2021-08-20)

### Changes

* PR [#9](https://github.com/kenba/cl3/pull/9) Implement CL_VERSION_* features.
* Remove deprecated attribute for `create_command_queue`
* Fix `clippy` warnings and format with `fmt`.

## Version 0.4.1 (2021-08-06)

### Changes

* Add `cl_khr_integer_dot_product` extension.
* Use `CL_BLOCKING` and `CL_NON_BLOCKING` in enqueue calls.

## Version 0.4.0 (2021-05-30)

### New Features

* Add functions for Issue [#8](https://github.com/kenba/cl3/issues/8) Add generic functions to call `clGet*Info` functions.

## Version 0.3.1 (2021-05-22)

### New Features

* Add `cl_apple_setmemobjectdestructor` feature for Issue [#7](https://github.com/kenba/cl3/issues/7) Building on Debian Stretch fails.

## Version 0.3.0 (2021-05-16)

### Changes

* None.

## Version 0.2.4 (2021-05-16)

### New Features

* Add `cl_khr_gl_event` extension.
* Inline EGL functions.

## Version 0.2.3 (2021-05-15)

### New Features

* Issue [#6](https://github.com/kenba/cl3/issues/6) Make custom clGetDeviceInfo requests easier.
* Add undocumented `cl_nv_device_attribute_query` extension values.
* Add FFI interfaces for OpenCL extensions.
* Add OpenGL interoperability functions.
* Add dx9_media_sharing, d3d10 and d3d11.
* Add cl_khr_device_uuid values.
* Add DirectX error codes.

## Version 0.2.2 (2021-04-18)

### Changes

* Add Event types and OpenCL 3 constants.

## Version 0.2.1 (2021-04-11)

### Changes

* Add OpenCL memory constants.

## Version 0.2.0 (2021-03-28)

### Breaking Changes

* Change `create_program_with_source` to take an array of string slices for source code strings.

## Version 0.1.8 (2021-03-26)

### Breaking Changes

* Issue [#4](https://github.com/kenba/cl3/issues/4) Change the API to use String instead of ffi::CString.

### New Features

* PR [#3](https://github.com/kenba/cl3/pull/3) Add InfoType::to_string method.

## Version 0.1.7 (2021-03-19)

### Bug fixes

* Issue [#2](https://github.com/kenba/cl3/issues/2) Change InfoType to handle nulls in strings returned from OpenCL devices.

## Version 0.1.6 (2021-03-12)

### Changes

* Add `CSTRING_UTF8_CONVERSION_ERROR` to error_codes.

## Version 0.1.5 (2021-01-13)

### Bug fixes

* PR [#1](https://github.com/kenba/cl3/pull/1) Remove all trailing nulls from OpenCL API info strings.

## Version 0.1.4 (2021-01-10)

### Bug fixes

* create_sub_devices

## Version 0.1.3 (2020-12-31)

### Bug fixes

* Mark missing CL_VERSION_2_1 features.

## Version 0.1.2 (2020-12-30)

### New Features

* `error_text` function in [error_codes](src/error_codes.rs) to convert OpenCL API error codes to text.

### Changes

* Now gets OpenCL FFI bindings from [cl-sys](https://crates.io/crates/cl-sys) where possible.
* Small functions are now declared `inline`.
* Added more tests, including [integration_test](tests/integration_test.rs).
* Added `rust.yml` for [GitHub Actions](https://docs.github.com/en/actions).

### Bug fixes

* SVM mapping
* get_program_info CL_PROGRAM_BINARIES
* clEnqueueCopyBufferRect

## Version 0.1.1 (2020-12-21)

### Changes

* Fixes for `cargo` documentation.

## Version 0.1.0 (2020-12-21)

### Features

* `safe` Rust functions that call OpenCL C API functions and return Rust Result types.
* Foreign Function Interfaces for OpenCL C functions in [ffi](src/ffi).
* OpenCL API data types in [types](src/type.rs).
* OpenCL API error codes in [error_codes](src/error_codes.rs).
* A Rust enum ([info_type](src/info_type.rs)) to hold the OpenCL types that can be returned from OpenCL "Info" functions, e.g. clGetPlatformInfo, clGetDeviceInfo, clGetProgramInfo, etc.
* Rust macros to call the OpenCL "Info" functions and return the appropriate `InfoType` in a Rust Result in [macros](src/macros.rs).