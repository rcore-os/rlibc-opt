#![no_std]
#![no_builtins]
#![deny(warnings)]
#![feature(global_asm)]

// copy from musl libc
#[cfg(target_arch = "x86_64")]
global_asm!(include_str!("x86_64_intel.S"));

// copy from compiler-builtins
#[no_mangle]
unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    let mut i = 0;
    while i < n {
        let a = *s1.add(i);
        let b = *s2.add(i);
        if a != b {
            return a as i32 - b as i32;
        }
        i += 1;
    }
    0
}
