# cupti-profiler

A safe Rust wrapper for the NVIDIA CUPTI Profiler API.

## Overview

This crate generates safe Rust bindings for the NVIDIA CUPTI (CUDA Profiling Tools Interface) library, specifically focusing on the Profiler and Activity APIs. It allows Rust applications to control profiling sessions, collect metrics, and inspect kernel execution data.

## Features

- **Safe Wrappers**: Encapsulates raw C bindings with safe Rust types and error handling.
- **Range Profiling**: Supports the CUPTI Range Profiler API for metric collection over specific code regions.
- **Activity API**: Provides access to asynchronous activity records (e.g., kernel launches).
- **Metric Evaluation**: Helper structs to evaluate and decode profiling metrics.

## Requirements

- **CUDA Toolkit**: Must be installed on the system (and `CUDA_HOME` set if not in default locations).
- **CUPTI**: Part of the CUDA Toolkit.

## Usage

This crate is primarily used as an internal dependency for profiling tools. It provides low-level checking of CUPTI results and higher-level abstractions like `RangeProfiler` and `ProfilerHost`.
