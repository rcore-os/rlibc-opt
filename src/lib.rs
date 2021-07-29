#![no_std]
#![no_builtins]
#![deny(warnings)]
#![feature(global_asm)]

// copy from musl libc
#[cfg(target_arch = "x86_64")]
global_asm!(include_str!("x86_64_intel.S"));

// in x86_64_intel.S
#[allow(dead_code)]
extern "sysv64" {
    fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8;
    fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8;
    fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8;
}

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

#[cfg(test)]
mod test {
    use super::{memcmp, memcpy, memmove, memset};

    #[test]
    fn memcmp_single_byte_pointers() {
        unsafe {
            assert_eq!(memcmp(&0xFAu8, &0xFAu8, 1), 0x00);
            assert!(memcmp(&0xEFu8, &0xFEu8, 1) < 0x00);
        }
    }

    #[test]
    fn memcmp_strings() {
        {
            let (x, z) = ("Hello!", "Good Bye.");
            let l = x.len();
            unsafe {
                assert_eq!(memcmp(x.as_ptr(), x.as_ptr(), l), 0);
                assert!(memcmp(x.as_ptr(), z.as_ptr(), l) > 0);
                assert!(memcmp(z.as_ptr(), x.as_ptr(), l) < 0);
            }
        }
        {
            let (x, z) = ("hey!", "hey.");
            let l = x.len();
            unsafe {
                assert!(memcmp(x.as_ptr(), z.as_ptr(), l) < 0);
            }
        }
    }

    #[test]
    fn memset_single_byte_pointers() {
        let mut x: u8 = 0xFF;
        unsafe {
            memset(&mut x, 0xAA, 1);
            assert_eq!(x, 0xAA);
            memset(&mut x, 0x00, 1);
            assert_eq!(x, 0x00);
            x = 0x01;
            memset(&mut x, 0x12, 0);
            assert_eq!(x, 0x01);
        }
    }

    #[test]
    fn memset_array() {
        let mut buffer = [b'X'; 100];
        unsafe {
            memset(buffer.as_mut_ptr(), b'#' as i32, buffer.len());
        }
        for byte in buffer.iter() {
            assert_eq!(*byte, b'#');
        }
    }

    #[test]
    fn memcpy_and_memcmp_arrays() {
        let (src, mut dst) = ([b'X'; 100], [b'Y'; 100]);
        unsafe {
            assert!(memcmp(src.as_ptr(), dst.as_ptr(), 100) != 0);
            let _ = memcpy(dst.as_mut_ptr(), src.as_ptr(), 100);
            assert_eq!(memcmp(src.as_ptr(), dst.as_ptr(), 100), 0);
        }
    }

    #[test]
    fn memmove_overlapping() {
        {
            let mut buffer = [b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9'];
            unsafe {
                memmove(&mut buffer[4], &buffer[0], 6);
                let mut i = 0;
                for byte in b"0123012345".iter() {
                    assert_eq!(buffer[i], *byte);
                    i += 1;
                }
            }
        }
        {
            let mut buffer = [b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9'];
            unsafe {
                memmove(&mut buffer[0], &buffer[4], 6);
                let mut i = 0;
                for byte in b"4567896789".iter() {
                    assert_eq!(buffer[i], *byte);
                    i += 1;
                }
            }
        }
    }
}
