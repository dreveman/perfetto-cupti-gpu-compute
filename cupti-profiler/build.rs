// Copyright (C) 2026 David Reveman.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::env;
use std::path::PathBuf;

fn main() {
    let cuda_path = env::var("CUDA_HOME").unwrap_or_else(|_| "/usr/local/cuda".to_string());
    let cuda_include = format!("{}/include", cuda_path);
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-env-changed=CUDA_HOME");
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", cuda_include))
        .clang_arg("-x")
        .clang_arg("c++")
        .allowlist_function("cupti.*")
        .allowlist_function("cuda.*")
        .allowlist_function("cu.*")
        .allowlist_type("CUpti.*")
        .allowlist_type("CU.*")
        .allowlist_type("cu.*")
        .allowlist_var("CUPTI.*")
        .generate_comments(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    let use_stubs = env::var("CARGO_FEATURE_STUBS").is_ok();
    if use_stubs {
        cc::Build::new()
            .file("stubs.cpp")
            .include(cuda_include)
            .cpp(true)
            .compile("cupti_stubs");
    } else {
        println!("cargo:rustc-link-search=native={}/lib64", cuda_path);
        println!("cargo:rustc-link-lib=cupti");
        println!("cargo:rustc-link-lib=cuda");
    }
}
