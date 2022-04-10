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
It is the foundation of the [opencl3](https://crates.io/crates/opencl3) crate
which provides a simpler, object based model of the OpenCL 3.0 API.

[OpenCL 3.0](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html)
is a unified specification that adds little new functionality to previous OpenCL versions.  
It specifies that all **OpenCL 1.2** features are **mandatory**, while all
OpenCL 2.x and 3.0 features are now optional.

OpenCL also has extensions that enable other features such as OpenGL and Direct X interoperability, see [OpenCL Extensions](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_Ext.html). This library includes FFI bindings to use OpenCL extensions.

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

## Use

Ensure that an OpenCL Installable Client Driver (ICD) and the appropriate OpenCL
hardware driver(s) are installed, see
[OpenCL Installation](https://github.com/kenba/cl3/tree/main/docs/opencl_installation.md).

`cl3` supports OpenCL 1.2 and 2.0 ICD loaders by default. If you have an
OpenCL 2.0 ICD loader then add the following to your project's `Cargo.toml`:

```toml
[dependencies]
cl3 = "0.7"
```

If your OpenCL ICD loader supports higher versions of OpenCL then add the
appropriate features to cl3, e.g. for an OpenCL 2.2 ICD loader add the
following to your project's `Cargo.toml` instead:

```toml
[dependencies.cl3]
version = "0.7"
features = ["CL_VERSION_2_1", "CL_VERSION_2_2"]
```

OpenCL extensions can also be enabled by adding their features, e.g.:

```toml
[dependencies.cl3]
version = "0.7"
features = ["cl_khr_gl_sharing", "cl_khr_dx9_media_sharing"]
```

Whichever version of OpenCL ICD loader you use, add the following to your
crate root (`lib.rs` or `main.rs`):

```rust
extern crate cl3;
```

## Tests

The crate contains unit, documentation and integration tests.  
The tests run the platform and device info functions (among others) so they
can provide useful information about OpenCL capabilities of the system.

It is recommended to run the tests in single-threaded mode, since some of
them can interfere with each other when run multi-threaded, e.g.:

```shell
cargo test -- --test-threads=1 --show-output
```

The integration tests are marked `ignore` so use the following command to
run them:

```shell
cargo test -- --test-threads=1 --show-output --ignored
```

## Examples

The tests provide examples of how the crate may be used, e.g. see:
[platform](https://github.com/kenba/cl3/tree/main/src/platform.rs),
[device](https://github.com/kenba/cl3/tree/main/src/device.rs),
[context](https://github.com/kenba/cl3/tree/main/src/context.rs) and
[integration_test](https://github.com/kenba/cl3/tree/main/tests/integration_test.rs).

## Contribution

If you want to contribute through code or documentation, the [Contributing](CONTRIBUTING.md) guide is the best place to start. If you have any questions, please feel free to ask.
Just please abide by our [Code of Conduct](CODE_OF_CONDUCT.md).

## License

Licensed under the Apache License, Version 2.0, as per Khronos Group OpenCL.  
You may obtain a copy of the License at: <http://www.apache.org/licenses/LICENSE-2.0>

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be licensed as above, without any additional terms or conditions.

OpenCL and the OpenCL logo are trademarks of Apple Inc. used by permission by Khronos.
