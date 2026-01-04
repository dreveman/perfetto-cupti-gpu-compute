# cupti-profiler-sys

Low-level FFI bindings for the NVIDIA CUPTI Profiler API.

## Bindings Generation

The bindings in `src/bindings.rs` are pre-generated to avoid a build-time dependency on `bindgen` and `libclang` for consumers.

To re-generate the bindings (e.g., after updating `wrapper.h` or upgrading CUDA):

1.  Ensure you have a valid CUDA installation (set `CUDA_HOME` if needed).
2.  Run the build with the `gen` feature enabled:

```bash
CUDA_HOME=/usr/local/cuda cargo build -p cupti-profiler-sys --features gen
```

This will run `bindgen` and overwrite `src/bindings.rs`.

## Stubs

This crate supports a `stubs` feature which compiles a dummy C++ implementation of the APIs. This allows building and testing on systems without CUDA installed (e.g., MacOS).

```bash
cargo build -p cupti-profiler-sys --features stubs
```
