// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b6a248: *mut c_void;
    static mut DAT_00b69d20: [i8; 256];
    static mut DAT_00b69d1e: i8;
    static mut DAT_00b69e34: u32;
    static mut DAT_0051c4bc: [i8; 4];
    static mut DAT_0052c164: [i8; 16];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn free(p: *mut c_void);
    fn malloc(n: u32) -> *mut c_void;
    fn fgets(buf: *mut i8, n: i32, f: *mut c_void) -> *mut i8;
    fn atoi(s: *const i8) -> i32;
    fn sscanf(s: *const i8, fmt: *const i8, ...) -> i32;
    fn FUN_004cc4a0(a: *mut i8, b: *mut i8) -> *mut c_void;
    fn FUN_004cc590(f: *mut c_void);
}

/// Reads lines until one that is not a "//" comment; false on EOF.
unsafe fn next_line(file: *mut c_void) -> bool {
    // SAFETY: the buffer and libc are the game's own.
    unsafe {
        loop {
            let p = fgets((&raw mut DAT_00b69d20).cast::<i8>(), 0xff, file);
            if p.is_null() {
                return false;
            }
            if !(*p == b'/' as i8 && *p.add(1) == b'/' as i8) {
                return true;
            }
        }
    }
}

unsafe fn chomp() {
    // SAFETY: operates on the game's line buffer as the original does.
    unsafe {
        let buf = (&raw mut DAT_00b69d20).cast::<i8>();
        let mut n: u32 = 0;
        let mut p = buf;
        loop {
            let c = *p;
            p = p.add(1);
            n = n.wrapping_add(1);
            if c == 0 {
                break;
            }
        }
        let q = (&raw mut DAT_00b69d1e).byte_add(n as usize);
        if *q == b'\n' as i8 {
            *q = 0;
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004df250(param_1: *mut i8) -> u32 {
    // SAFETY: globals, libc and callees are the game's own; record
    // pointers stay inside the block allocated here as in the original.
    unsafe {
        if !DAT_00b6a248.is_null() {
            free(DAT_00b6a248);
            DAT_00b6a248 = core::ptr::null_mut();
        }
        let file = FUN_004cc4a0(param_1, (&raw mut DAT_0051c4bc).cast::<i8>());
        if file.is_null() {
            return 0;
        }
        let buf = (&raw mut DAT_00b69d20).cast::<i8>();
        if fgets(buf, 0xff, file).is_null() {
            FUN_004cc590(file);
            return 0;
        }
        let count = atoi(buf) as u32;
        DAT_00b6a248 = malloc(count.wrapping_mul(300));
        if DAT_00b6a248.is_null() {
            return 0;
        }
        let mut z = DAT_00b6a248.cast::<u32>();
        let mut k = count.wrapping_mul(300) >> 2;
        while k != 0 {
            z.write_unaligned(0);
            z = z.add(1);
            k -= 1;
        }
        DAT_00b69e34 = 0;
        loop {
            if count <= DAT_00b69e34 {
                FUN_004cc590(file);
                return 1;
            }
            if !next_line(file) {
                FUN_004cc590(file);
                return 1;
            }
            chomp();
            let rec = |i: u32| -> *mut u32 {
                DAT_00b6a248
                    .cast::<u32>()
                    .wrapping_add(i.wrapping_mul(0x4b) as usize)
            };
            let mut src = buf.cast::<u32>();
            let mut dst = rec(DAT_00b69e34);
            for _ in 0..0x20 {
                dst.write_unaligned(src.read_unaligned());
                src = src.add(1);
                dst = dst.add(1);
            }
            if !next_line(file) {
                rec(DAT_00b69e34).cast::<i8>().write(0);
                FUN_004cc590(file);
                return 1;
            }
            let r = rec(DAT_00b69e34);
            let n = sscanf(
                buf,
                (&raw mut DAT_0052c164).cast::<i8>(),
                r.wrapping_add(0x48),
                r.wrapping_add(0x49),
                r.wrapping_add(0x4a),
            );
            if n != 3 {
                let mut p = rec(DAT_00b69e34);
                for _ in 0..0x4b {
                    p.write_unaligned(0);
                    p = p.add(1);
                }
                FUN_004cc590(file);
                return 1;
            }
            if !next_line(file) {
                FUN_004cc590(file);
                return 1;
            }
            chomp();
            let mut src = buf.cast::<u32>();
            let mut dst = rec(DAT_00b69e34).wrapping_add(0x20);
            for _ in 0..8 {
                dst.write_unaligned(src.read_unaligned());
                src = src.add(1);
                dst = dst.add(1);
            }
            if !next_line(file) {
                FUN_004cc590(file);
                return 1;
            }
            chomp();
            let mut src = buf.cast::<u32>();
            let mut dst = rec(DAT_00b69e34).wrapping_add(0x28);
            for _ in 0..0x20 {
                dst.write_unaligned(src.read_unaligned());
                src = src.add(1);
                dst = dst.add(1);
            }
            DAT_00b69e34 = DAT_00b69e34.wrapping_add(1);
        }
    }
}
