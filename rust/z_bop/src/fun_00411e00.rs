// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005506f8: [i8; 0];
    static mut DAT_0051a854: [i8; 0];
    static mut DAT_0051bd9c: [i8; 0];
    static mut DAT_00a07bd8: *mut c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn fclose(f: *mut c_void) -> i32;
    fn fscanf(f: *mut c_void, fmt: *const i8, ...) -> i32;
    fn _strnicmp(a: *const i8, b: *const i8, n: u32) -> i32;

    fn FUN_0049e720(a: *mut i8, b: *mut i8, c: i32, d: i32);
    fn FUN_004742a0(a: *mut i8) -> u32;
    fn FUN_00449130();
    fn FUN_004762f0(a: u32) -> i16;
    fn FUN_00412040(f: *mut c_void) -> u16;
    fn FUN_00412030() -> u16;
    fn FUN_00477950(a: u32) -> u16;
    fn FUN_004766c0(a: *mut i8, b: u32);
}

/// Copies the NUL-terminated string at `src` (terminator included) to `dst`.
unsafe fn copy_str(mut src: *const i8, mut dst: *mut i8) {
    // SAFETY: caller guarantees both pointers address valid string memory.
    unsafe {
        loop {
            let c = *src;
            *dst = c;
            src = src.add(1);
            dst = dst.add(1);
            if c == 0 {
                break;
            }
        }
    }
}

unsafe fn str_end(mut p: *mut i8) -> *mut i8 {
    // SAFETY: caller guarantees a NUL-terminated string.
    unsafe {
        while *p != 0 {
            p = p.add(1);
        }
    }
    p
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00411e00(param_1: *mut i8) -> i16 {
    // SAFETY: globals and callees are the game's own; param_1 is a
    // NUL-terminated string, and the buffers are the game's own memory.
    unsafe {
        let buf = &raw mut DAT_005506f8 as *mut i8;
        let a854 = &raw mut DAT_0051a854 as *mut i8;
        let bd9c = &raw mut DAT_0051bd9c as *mut i8;

        copy_str(param_1, buf);
        FUN_0049e720(buf, a854, 0, 0);
        if !DAT_00a07bd8.is_null() {
            fclose(DAT_00a07bd8);
            let u_var6 = FUN_004742a0(buf);
            FUN_00449130();
            let s = FUN_004762f0(u_var6 & 0xffff);
            FUN_00449130();
            return s;
        }
        let mut p = buf;
        while *p != b'.' as i8 {
            p = p.add(1);
        }
        *p = 0;
        let end = str_end(buf);
        copy_str(bd9c, end);
        FUN_0049e720(buf, a854, 1, 0);
        let file = DAT_00a07bd8;
        if file.is_null() {
            return 0;
        }
        if fscanf(file, b"%256s\0".as_ptr() as *const i8, buf) != 1 {
            return 0;
        }
        if _strnicmp(buf, b"#inventor\0".as_ptr() as *const i8, 9) != 0 {
            return 0;
        }
        if fscanf(file, b" %256s\0".as_ptr() as *const i8, buf) != 1 {
            return 0;
        }
        if fscanf(file, b" %256s\0".as_ptr() as *const i8, buf) != 1 {
            return 0;
        }
        let u_var2: u16;
        if _strnicmp(buf, b"ascii\0".as_ptr() as *const i8, 5) == 0 {
            u_var2 = FUN_00412040(file);
        } else {
            if _strnicmp(buf, b"binary\0".as_ptr() as *const i8, 6) != 0 {
                fclose(file);
                return 0;
            }
            u_var2 = FUN_00412030();
        }
        fclose(file);
        let u_var3 = FUN_00477950(u_var2 as u32);
        copy_str(param_1, buf);
        FUN_004766c0(buf, u_var3 as u32);
        FUN_004762f0(u_var3 as u32)
    }
}
