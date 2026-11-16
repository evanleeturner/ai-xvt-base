// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00650406: i16;
    static mut _DAT_00650408: i16;
    static mut DAT_00650424: i16;
    static mut DAT_00650426: i16;
    static mut DAT_00650428: i16;
    static mut DAT_0065042a: i16;
    static mut DAT_00650434: i32;
    static mut DAT_0065042c: i16;
    static mut DAT_00650438: i32;
    static mut DAT_0065042e: i16;
    static mut DAT_00650430: i16;
    static mut DAT_00650432: i16;
    static mut DAT_0065043c: i32;
    static mut DAT_00743880: i32;
    static mut DAT_0075766c: i32;
    static mut DAT_007562e1: i8;
    static mut DAT_007562de: i8;
    static mut DAT_006507a2: *mut core::ffi::c_void;
    static mut DAT_00650822: *mut core::ffi::c_void;
    static mut DAT_006508a2: *mut core::ffi::c_void;
    static mut DAT_006508fa: i32;
    static mut DAT_00650904: i32;
    static mut DAT_00650936: i32;
    static mut DAT_00650440: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004f2bd0();
    fn FUN_004f2c10();
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _malloc(size: u32) -> *mut core::ffi::c_void;
    fn FUN_004dcfd0(p1: *mut i32, p2: i32, p3: i32, p4: i32, p5: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004f2410() {
    // SAFETY: all globals and callees are the game's own memory and code;
    // array writes go to the game's tables at the offsets the original uses.
    unsafe {
        DAT_00650406 = 1;
        let mut s_var3: i16 = 0;
        _DAT_00650408 = 0;
        DAT_00650424 = 0;
        DAT_00650426 = 0;
        DAT_00650428 = 0;
        DAT_0065042a = 0;
        DAT_00650434 = 0;
        DAT_0065042c = 0x20;
        DAT_00650438 = 0;
        DAT_0065042e = 0x20;
        DAT_00650430 = 0x20;
        DAT_00650432 = 0x20;
        DAT_0065043c = 0;
        let mut p = &raw mut DAT_00743880;
        let mut n: i32 = 0x4f7d;
        while n != 0 {
            p.write_unaligned(0);
            p = p.add(1);
            n -= 1;
        }
        DAT_0075766c = 1;
        DAT_007562e1 = 0;
        DAT_007562de = 1;
        FUN_004f2bd0();
        FUN_004f2c10();
        loop {
            let pv = _malloc(0x28);
            let i = s_var3 as isize;
            s_var3 = s_var3.wrapping_add(1);
            (&raw mut DAT_006507a2).offset(i).write_unaligned(pv);
            if s_var3 >= 0x20 {
                break;
            }
        }
        s_var3 = 0;
        loop {
            let pv = _malloc(0x140);
            let i = s_var3 as isize;
            s_var3 = s_var3.wrapping_add(1);
            (&raw mut DAT_00650822).offset(i).write_unaligned(pv);
            if s_var3 >= 0x20 {
                break;
            }
        }
        s_var3 = 0;
        loop {
            let pv = _malloc(0x400);
            let i = s_var3 as isize;
            s_var3 = s_var3.wrapping_add(1);
            (&raw mut DAT_006508a2).offset(i).write_unaligned(pv);
            if s_var3 >= 0x14 {
                break;
            }
        }
        s_var3 = 0;
        loop {
            let i = s_var3 as isize;
            s_var3 = s_var3.wrapping_add(1);
            (&raw mut DAT_006508fa).offset(i).write_unaligned(0);
            if s_var3 >= 2 {
                break;
            }
        }
        s_var3 = 0;
        loop {
            let i = s_var3 as isize;
            s_var3 = s_var3.wrapping_add(1);
            (&raw mut DAT_00650904).offset(i).write_unaligned(0);
            if s_var3 >= 8 {
                break;
            }
        }
        s_var3 = 0;
        loop {
            let i = s_var3 as isize;
            s_var3 = s_var3.wrapping_add(1);
            (&raw mut DAT_00650936).offset(i).write_unaligned(0);
            if s_var3 >= 8 {
                break;
            }
        }
        FUN_004dcfd0(&raw mut DAT_00650440, 0, 0, 0x168, 0xec);
    }
}
