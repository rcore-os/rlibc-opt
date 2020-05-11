# rlibc-opt

An optimized version of [rlibc](https://github.com/alexcrichton/rlibc).

This crate used to provide a Rust implementation of some libc functions such as `memcpy` / `memmove` / `memset` / `memcmp`, required when developing freestanding applications.

The code is borrowed from [musl-libc](https://musl.libc.org) and [compiler_builtins](https://github.com/rust-lang/compiler-builtins).
