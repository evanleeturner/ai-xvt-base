// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00533fd0: i8;
    static mut DAT_00533fdc: i8;
    static mut DAT_00533f6c: i8;
    static mut DAT_00533f18: i8;
    static mut DAT_00533f50: i8;
    static mut DAT_00533f34: i8;
    static mut DAT_00532a4c: i8;
    static mut DAT_00533f8c: i8;
    static mut DAT_00533fb0: i8;
    static mut DAT_004bee00: u8;

    static mut DAT_00533f00: i32;
    static mut DAT_006509e4: *mut c_void;
    static mut DAT_006509e0: u16;
    static mut DAT_006509d4: i32;
    static mut DAT_00650a08: i32;
    static mut DAT_00650a04: i32;
    static mut DAT_006509e8: i32;
    static mut DAT_006509c8: i32;
    static mut DAT_006509c0: i32;
    static mut DAT_006509ec: i32;
    static mut DAT_006509cc: i32;
    static mut DAT_006509c4: i32;
    static mut DAT_006509d0: i32;
    static mut DAT_006509f0: i32;
    static mut DAT_006509d8: i32;
    static mut DAT_006509b8: i32;
    static mut DAT_006509f4: i32;
    static mut DAT_006509dc: i32;
    static mut DAT_006509bc: i32;
    static mut DAT_006509f8: i32;
    static mut DAT_006509fc: i32;
    static mut DAT_00650a00: i32;
    static mut DAT_00650a0c: i32;
    static mut DAT_00650a10: i32;
    static mut DAT_00650a18: i8;
    static mut DAT_00652a18: i8;
    static mut DAT_00654a18: u16;
    static mut DAT_00654a58: u16;
    static mut DAT_00654a98: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dbea0();
    fn FUN_004be1f0(p: *mut c_void);
    fn FUN_004be160(a: *mut i8, b: *mut i8) -> *mut c_void;
    fn FUN_004f6ad0(a: *mut i32, b: *mut i32, c: *mut i32, d: *mut i32) -> i32;
    fn FUN_004db670(a: *mut c_void, b: *mut c_void);
    fn FUN_004db490(a: i32);
    fn FUN_004dbe00(a: i32) -> u8;
    fn FUN_004d0e60(a: *mut i8, b: i32, c: i32);
    fn FUN_004d52b0();
    fn FUN_004d1750(a: *mut i8, b: i32, c: i32);
    fn FUN_004d0580(a: *mut i8, b: i32, c: i32);
    fn FUN_004d5350(a: i32);
    fn FUN_004db4e0();
    fn FUN_004da7c0(a: i32, b: *mut i8, c: i32, d: i32, e: u32);
    fn FUN_004db500();
    fn FUN_004cff00();
    fn FUN_004d5420();
    fn FUN_004d00f0(a: i32, b: i32, c: i32);
    fn FUN_004dbe40() -> u32;
    fn FUN_004db570() -> u8;
    fn FUN_004db590() -> u8;
    fn FUN_004d50a0();
    fn FUN_004bef20();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004f663a(param_1: u32, param_2: i32) -> u32 {
    let _ = param_1;
    let mut unaff_retaddr: i32 = 0;
    let in_zf: u8 = 0;
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers are addresses of those globals, used as the original does.
    unsafe {
        if in_zf == 0 {
            FUN_004dbea0();
            if !DAT_006509e4.is_null() {
                FUN_004be1f0(DAT_006509e4);
            }
            DAT_006509e0 = 0xffff;
            DAT_00533f00 = 0;
            DAT_006509d4 = 0;
            DAT_00650a08 = 0;
            DAT_00650a04 = 0;
            DAT_006509e8 = 0;
            DAT_006509c8 = 0;
            DAT_006509c0 = 0;
            DAT_006509ec = 0;
            DAT_006509cc = 0;
            DAT_006509c4 = 0;
            DAT_006509e4 = FUN_004be160(&raw mut DAT_00533fd0, &raw mut DAT_00533fdc);
            DAT_006509d0 = 0;
            DAT_006509f0 = 0;
            DAT_006509d8 = 0;
            DAT_006509b8 = 0;
            DAT_006509f4 = 0;
            DAT_006509dc = 0;
            DAT_006509bc = 0;
            let i = FUN_004f6ad0(
                &raw mut DAT_00650a10,
                &raw mut DAT_006509d0,
                &raw mut DAT_006509f8,
                &raw mut DAT_006509fc,
            );
            if i == 0 {
                FUN_004db670(
                    FUN_004bef20 as unsafe extern "C" fn() as *mut c_void,
                    &raw mut DAT_004bee00 as *mut c_void,
                );
                return 0;
            }
            FUN_004db490(200);
        }
        if DAT_00533f00 != 0 {
            FUN_004db670(
                FUN_004bef20 as unsafe extern "C" fn() as *mut c_void,
                &raw mut DAT_004bee00 as *mut c_void,
            );
            return 0;
        }
        if FUN_004dbe00(0x10) != 0 && FUN_004dbe00(0x12) != 0 && FUN_004dbe00(0x7b) != 0 {
            let sel: Option<(i32, *mut i8)> = match DAT_006509d4 {
                0 => Some((0x9d, &raw mut DAT_00533f6c)),
                1 => Some((0xa3, &raw mut DAT_00533f18)),
                2 => Some((0x98, &raw mut DAT_00533f50)),
                4 => Some((0x8f, &raw mut DAT_00533f34)),
                _ => None,
            };
            if let Some((y, s)) = sel {
                FUN_004d0e60(s, 0xa0, y);
            }
        }
        let same = DAT_006509e8 == DAT_006509f0
            && DAT_006509b8 == DAT_006509c0
            && DAT_006509d8 == DAT_006509c8
            && DAT_006509ec == DAT_006509f4
            && DAT_006509bc == DAT_006509c4
            && DAT_006509dc == DAT_006509cc;
        if !same {
            DAT_006509f0 = DAT_006509e8;
            DAT_006509b8 = DAT_006509c0;
            DAT_006509d8 = DAT_006509c8;
            DAT_006509f4 = DAT_006509ec;
            DAT_006509bc = DAT_006509c4;
            DAT_006509dc = DAT_006509cc;
            FUN_004d52b0();
            FUN_004d1750(&raw mut DAT_00532a4c, 0, 0);
            if DAT_006509e8 == 1 {
                FUN_004d0580(&raw mut DAT_00533f8c, DAT_006509c0, DAT_006509c8);
            } else if DAT_006509e8 == 2 {
                FUN_004d0580(&raw mut DAT_00533fb0, DAT_006509c0, DAT_006509c8);
            }
            if DAT_006509ec == 1 {
                FUN_004d0580(&raw mut DAT_00533f8c, DAT_006509c4, DAT_006509cc);
            } else if DAT_006509ec == 2 {
                FUN_004d0580(&raw mut DAT_00533fb0, DAT_006509c4, DAT_006509cc);
            }
            FUN_004d5350(1);
        }
        if (DAT_00650a10 & 1) != 0 {
            FUN_004db4e0();
        }
        let mut i_var3 = DAT_00650a08;
        let mut p5: *mut u16 = &raw mut DAT_00654a18;
        let mut pc: *mut i8 = &raw mut DAT_00650a18;
        let mut i_var7 = DAT_00650a00;
        loop {
            let p6 = p5.add(1);
            FUN_004da7c0(0xf, pc, i_var3, i_var7, p5.read() as u32);
            p5 = p6;
            pc = pc.byte_add(0x100);
            i_var7 = i_var7.wrapping_add(0x13);
            if !(p6 < (&raw mut DAT_00654a58)) {
                break;
            }
        }
        if (DAT_00650a10 & 1) == 0 {
            FUN_004db4e0();
        } else {
            FUN_004db500();
        }
        i_var3 = DAT_00650a0c;
        p5 = &raw mut DAT_00654a58;
        pc = &raw mut DAT_00652a18;
        i_var7 = DAT_00650a04;
        loop {
            let p6 = p5.add(1);
            FUN_004da7c0(0xf, pc, i_var3, i_var7, p5.read() as u32);
            p5 = p6;
            pc = pc.byte_add(0x100);
            i_var7 = i_var7.wrapping_add(0x13);
            if !(p6 < (&raw mut DAT_00654a98)) {
                break;
            }
        }
        FUN_004db500();
        if DAT_006509d0 == 0 {
            if DAT_006509f8 <= param_2 {
                DAT_00533f00 = 1;
                FUN_004d5420();
                FUN_004d00f0(0x8000, 0x1000, 2000);
            }
        } else if DAT_006509f8 <= param_2 {
            FUN_004f6ad0(
                &raw mut DAT_00650a10,
                &raw mut DAT_006509d0,
                &raw mut unaff_retaddr,
                &raw mut DAT_006509fc,
            );
            DAT_006509f8 = DAT_006509f8.wrapping_add(unaff_retaddr);
            FUN_004db490(DAT_006509fc);
            DAT_006509d4 = DAT_006509d4.wrapping_add(1);
            if DAT_006509d0 == 0 {
                FUN_004cff00();
            }
        }
        let u_var4 = FUN_004dbe40() & 0xff;
        if FUN_004db570() != 0
            || FUN_004db590() != 0
            || u_var4 == 0x1b
            || u_var4 == 0xd
            || u_var4 == 0x20
        {
            DAT_00533f00 = 1;
            FUN_004d5420();
            FUN_004d50a0();
            FUN_004d00f0(0x8000, 0x1000, 2000);
        }
    }
    0
}
