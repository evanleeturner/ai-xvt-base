// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b69cd8: *mut u32;
    static mut DAT_00b69cc8: u32;
    static mut DAT_00b69d1e: [i8; 1];
    static mut DAT_00b69d20: [i8; 1];
    static mut DAT_0051c4bc: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cc4a0(param_1: *mut i8, param_2: *mut i8) -> *mut c_void;
    fn FUN_004cc590(param_1: *mut c_void);
    fn _free(param_1: *mut c_void);
    fn _fgets(param_1: *mut i8, param_2: i32, param_3: *mut c_void) -> *mut i8;
    fn _atoi(param_1: *const i8) -> i32;
    fn _malloc(param_1: u32) -> *mut c_void;
}

/// Reads lines, skipping "//" comments. Returns false when fgets fails.
unsafe fn read_line(file: *mut c_void) -> bool {
    // SAFETY: the buffer and the stream are the game's own.
    unsafe {
        loop {
            let p = _fgets(&raw mut DAT_00b69d20 as *mut i8, 0xff, file);
            if p.is_null() {
                return false;
            }
            if !(*p == b'/' as i8 && *p.add(1) == b'/' as i8) {
                return true;
            }
        }
    }
}

/// Strips a trailing newline from the line buffer.
unsafe fn trim() {
    // SAFETY: operates on the game's line buffer, as the original does.
    unsafe {
        let buf = &raw mut DAT_00b69d20 as *mut i8;
        let mut len: usize = 0;
        while *buf.add(len) != 0 {
            len += 1;
        }
        let last = (&raw mut DAT_00b69d1e as *mut i8).wrapping_add(len.wrapping_add(1));
        if *last == b'\n' as i8 {
            *last = 0;
        }
    }
}

/// Copies 8 dwords from the line buffer to dst.
unsafe fn copy32(dst: *mut u8) {
    // SAFETY: the destination lies inside the allocated record array.
    unsafe {
        let src = &raw mut DAT_00b69d20 as *const u8;
        for i in 0..8usize {
            let v = src.add(i * 4).cast::<u32>().read_unaligned();
            dst.add(i * 4).cast::<u32>().write_unaligned(v);
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004cc060(param_1: *mut i8) -> u32 {
    // SAFETY: all globals and callees are the game's own; the record array
    // is allocated here with count * 0x424 bytes and indexed as the original.
    unsafe {
        if !DAT_00b69cd8.is_null() {
            _free(DAT_00b69cd8 as *mut c_void);
            DAT_00b69cd8 = core::ptr::null_mut();
        }
        let file = FUN_004cc4a0(param_1, &raw mut DAT_0051c4bc as *mut i8);
        if file.is_null() {
            return 0;
        }
        let p = _fgets(&raw mut DAT_00b69d20 as *mut i8, 0xff, file);
        if p.is_null() {
            FUN_004cc590(file);
            return 0;
        }
        let mut local_4: u32 = _atoi(&raw mut DAT_00b69d20 as *const i8) as u32;
        DAT_00b69cd8 = _malloc(local_4.wrapping_mul(0x424)) as *mut u32;
        if DAT_00b69cd8.is_null() {
            return 0;
        }
        let mut z = DAT_00b69cd8;
        let mut n = local_4.wrapping_mul(0x424) >> 2;
        while n != 0 {
            z.write_unaligned(0);
            z = z.add(1);
            n -= 1;
        }
        DAT_00b69cc8 = 0;
        loop {
            if local_4 <= DAT_00b69cc8 {
                FUN_004cc590(file);
                return 1;
            }
            let base = DAT_00b69cd8 as *mut u8;
            let rec = base.wrapping_add(DAT_00b69cc8.wrapping_mul(0x424) as usize);

            if !read_line(file) {
                FUN_004cc590(file);
                return 1;
            }
            trim();
            let v = _atoi(&raw mut DAT_00b69d20 as *const i8);
            rec.cast::<u32>().write_unaligned(v as u32);

            if !read_line(file) {
                rec.cast::<u32>().write_unaligned(0);
                FUN_004cc590(file);
                return 1;
            }
            trim();
            copy32(rec.wrapping_add(4));

            let mut uvar3: u32 = 0;
            *rec.wrapping_add(0x23) = 0;
            while uvar3 < 0x1e0 {
                if !read_line(file) {
                    rec.cast::<u32>().write_unaligned(0);
                    FUN_004cc590(file);
                    return 1;
                }
                trim();
                copy32(rec.wrapping_add(uvar3 as usize).wrapping_add(0x224));
                uvar3 = uvar3.wrapping_add(0x20);
                *rec.wrapping_add(uvar3 as usize).wrapping_add(0x223) = 0;
            }

            local_4 = 0;
            uvar3 = 0;
            while uvar3 < 0x1e0 {
                if !read_line(file) {
                    rec.cast::<u32>().write_unaligned(0);
                    FUN_004cc590(file);
                    return 1;
                }
                trim();
                copy32(rec.wrapping_add(uvar3 as usize).wrapping_add(0x24));
                uvar3 = uvar3.wrapping_add(0x20);
                local_4 = local_4.wrapping_add(1);
                *rec.wrapping_add(uvar3 as usize).wrapping_add(0x23) = 0;
            }
            DAT_00b69cc8 = DAT_00b69cc8.wrapping_add(1);
        }
    }
}
