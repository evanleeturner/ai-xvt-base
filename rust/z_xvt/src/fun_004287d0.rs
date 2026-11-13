// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00536f35: i32;
    static mut DAT_00536f25: i32;
    static mut DAT_00536f29: i32;
    static mut DAT_00536ef4: i32;
    static mut DAT_00536ef0: *mut core::ffi::c_void;
    static mut DAT_00536f21: *mut core::ffi::c_void;
    static mut DAT_00536ef8: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004288b0(a: u8, b: u8, c: i32, d: u8, e: u8, f: u8) -> *mut core::ffi::c_void;
    fn malloc(size: u32) -> *mut core::ffi::c_void;
    fn FUN_0047f010(a: u16);
    fn _exit(code: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004287d0(param_1: i32) {
    let mut param_1 = param_1;
    // SAFETY: the globals and callees are the game's own memory and
    // functions; the pointers written through are the ones the game
    // just allocated, checked non-null before use.
    unsafe {
        DAT_00536f35 = 0;
        DAT_00536f25 = 0;
        DAT_00536f29 = 0;
        if 8 < param_1 {
            param_1 = 8;
        }
        if param_1 < 2 {
            param_1 = 2;
        }
        DAT_00536ef4 = param_1;
        DAT_00536ef0 = FUN_004288b0(0, 0, 0, 0x80, 0x80, 0x80);
        DAT_00536f21 = malloc(0x7fc);
        if !DAT_00536ef0.is_null() && !DAT_00536f21.is_null() {
            let tmp_2e = DAT_00536ef0;
            DAT_00536ef0
                .byte_add(0x2e)
                .cast::<*mut core::ffi::c_void>()
                .write_unaligned(tmp_2e);
            let p_var1 = DAT_00536ef0;
            p_var1.byte_add(0xe).cast::<i32>().write_unaligned(0);
            p_var1.byte_add(0x12).cast::<i32>().write_unaligned(0);
            DAT_00536ef8 = 0;
            DAT_00536f21 = DAT_00536f21.byte_add(0x3fc);
            let mut i_var2: i32 = -0xff;
            loop {
                DAT_00536f21
                    .byte_offset((i_var2.wrapping_mul(4)) as isize)
                    .cast::<i32>()
                    .write_unaligned(i_var2.wrapping_mul(i_var2));
                i_var2 = i_var2.wrapping_add(1);
                if i_var2 >= 0x100 {
                    break;
                }
            }
            return;
        }
        FUN_0047f010(0);
        _exit(1);
    }
}
