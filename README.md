# rlibc-opt

[![CI](https://github.com/rcore-os/rlibc-opt/workflows/CI/badge.svg?branch=master)](https://github.com/rcore-os/rlibc-opt/actions)

An optimized version of [rlibc](https://github.com/alexcrichton/rlibc).

This crate used to provide a Rust implementation of some libc functions such as `memcpy` / `memmove` / `memset` / `memcmp`, required when developing freestanding applications.

The code is borrowed from [musl-libc](https://musl.libc.org) and [compiler_builtins](https://github.com/rust-lang/compiler-builtins).
