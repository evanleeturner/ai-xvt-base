// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732e8c: u16;
    static mut DAT_00732e58: u16;
    static mut DAT_00732e52: u16;
    static mut DAT_00732e50: u8;
    static mut DAT_00732a34: u16;
    static mut DAT_007329c0: u16;
    static mut DAT_00730894: u16;
    static mut DAT_0071e524: u8;
    static mut DAT_0071b012: u8;
    static mut DAT_0070acc0: u16;
    static mut DAT_0070a188: i8;
    static mut DAT_006ffbf0: u16;
    static mut DAT_006da9fa: u8;
    static mut DAT_006d9980: u16;
    static mut DAT_006d4a4e: u8;
    static mut DAT_006db594: i32;
    static mut DAT_006db5b0: i32;
    static mut DAT_006da538: i32;
    static mut DAT_00525518: i32;
    static mut DAT_0070ab20: i8;
    static mut DAT_00525e04: i8;
    static mut DAT_006a8974: *mut i8;
    static mut DAT_006a8978: *mut i8;
    static mut DAT_0070ac2c: *const c_void;
    static mut DAT_00732a30: *const c_void;
    static mut DAT_00703c80: *const c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a6a80();
    fn FUN_0048ee20(param_1: i8);
    fn FUN_0048ed00(param_1: i16, param_2: i16, param_3: i16, param_4: i16);
    fn FUN_0048ece0(param_1: i16, param_2: i16);
    fn FUN_0048f270(param_1: *mut u8);
    fn FUN_004a6a70() -> i32;
    fn FUN_004a6f90();
    fn FUN_004a7130();
    fn FUN_004a7040();
    fn FUN_004a5470() -> u32;
}

/// Appends the NUL-terminated string `src` at the NUL of `dst` (or at `dst`
/// when `at_end` is false), copying the terminator too.
unsafe fn append(dst: *mut u8, src: *const i8, at_end: bool) {
    // SAFETY: the callers pass a 256-byte buffer holding a NUL-terminated
    // string (when appending) and NUL-terminated game strings as sources.
    unsafe {
        let mut d = dst;
        if at_end {
            while d.read() != 0 {
                d = d.add(1);
            }
        }
        let mut s = src as *const u8;
        loop {
            let b = s.read();
            d.write(b);
            if b == 0 {
                break;
            }
            s = s.add(1);
            d = d.add(1);
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0047e680() -> u8 {
    let mut abstack_100 = [0u8; 256];
    // SAFETY: all globals and callees are the game's own memory and code;
    // the string buffer is local and large enough for the original's use.
    unsafe {
        let u_var16 = DAT_00732e8c;
        let u_var15 = DAT_00732e58;
        let u_var14 = DAT_00732e52;
        let u_var13 = DAT_00732e50;
        let u_var12 = DAT_00732a34;
        let u_var11 = DAT_007329c0;
        let u_var10 = DAT_00730894;
        let u_var9 = DAT_0071e524;
        let u_var8 = DAT_0071b012;
        let u_var7 = DAT_0070acc0;
        let c_var6 = DAT_0070a188;
        let u_var5 = DAT_006ffbf0;
        let u_var4 = DAT_006da9fa;
        let u_var3 = DAT_006d9980;

        FUN_004a6a80();
        FUN_0048ee20(1);

        let d = DAT_006d4a4e;
        let u_var19 = d as u32;
        let i_var23 = (DAT_006db5b0 as u32).wrapping_add(
            (DAT_006db594 as u32)
                .wrapping_add(u_var19.wrapping_mul(4u32.wrapping_neg()))
                .wrapping_add(u32::MAX)
                .wrapping_mul(DAT_00525518 as u32)
                .wrapping_mul(DAT_006da538 as u32),
        ) as i32;
        let f5: unsafe extern "C" fn(i32, i32, i32, i32, i32) = core::mem::transmute(DAT_0070ac2c);
        f5(
            i_var23,
            0,
            ((DAT_006db594 >> 1) as u32).wrapping_add(u_var19.wrapping_mul(2u32.wrapping_neg()))
                as i32,
            DAT_006da538,
            u_var19.wrapping_mul(4).wrapping_add(1) as i32,
        );

        let di = DAT_006d4a4e as i32;
        let mut s_var18 = (DAT_006db594 >> 1) as i16;
        let mut s_var22 = (DAT_006da538 >> 4) as i16;
        FUN_0048ed00(
            s_var22,
            (s_var18 as i32).wrapping_add(di.wrapping_mul(-2)) as i16,
            ((DAT_006da538 as i16) as i32).wrapping_sub(s_var22 as i32) as i16,
            di.wrapping_mul(2).wrapping_add(s_var18 as i32) as i16,
        );
        DAT_00732e50 = 0xf9;
        let f0: unsafe extern "C" fn() = core::mem::transmute(DAT_00703c80);
        f0();

        s_var22 = (DAT_006db594 >> 1) as i16;
        s_var18 = (DAT_006da538 >> 4) as i16;
        let di = DAT_006d4a4e as i32;
        FUN_0048ed00(
            (s_var18 as i32).wrapping_add(1) as i16,
            (s_var22 as i32)
                .wrapping_add(di.wrapping_mul(-2))
                .wrapping_add(1) as i16,
            (((DAT_006da538 as i16) as i32).wrapping_sub(s_var18 as i32)).wrapping_add(-1) as i16,
            di.wrapping_mul(2)
                .wrapping_add(s_var22 as i32)
                .wrapping_add(-1) as i16,
        );
        DAT_00732e50 = 0;
        let f0: unsafe extern "C" fn() = core::mem::transmute(DAT_00703c80);
        f0();

        DAT_006da9fa = 0xf9;
        DAT_0071b012 = 0;
        DAT_0071e524 = 0;
        let di = DAT_006d4a4e as i32;
        FUN_0048ece0(
            0,
            ((((DAT_006db594 >> 1) as i16) as i32).wrapping_sub(di)).wrapping_add(-2) as i16,
        );

        let buf = abstack_100.as_mut_ptr();
        append(buf, &raw mut DAT_0070ab20 as *const i8, false);
        append(buf, &raw mut DAT_00525e04 as *const i8, true);
        append(buf, DAT_006a8974 as *const i8, true);

        FUN_0048f270(buf);
        FUN_0048ece0(
            0,
            (((DAT_006db594 >> 1) as i16) as i32).wrapping_add(2) as i16,
        );
        FUN_0048f270(DAT_006a8978 as *mut u8);

        let mut i_var17 = FUN_004a6a70();
        let mut i_var21 = i_var17;
        if 0 < i_var17 {
            loop {
                FUN_004a6f90();
                i_var21 = i_var21.wrapping_sub(1);
                if i_var21 == 0 {
                    break;
                }
            }
        }
        FUN_004a7130();
        FUN_004a7040();
        let u_ret = FUN_004a5470();
        if 0 < i_var17 {
            loop {
                FUN_004a6a80();
                i_var17 = i_var17.wrapping_sub(1);
                if i_var17 == 0 {
                    break;
                }
            }
        }

        let g5: unsafe extern "C" fn(i32, i32, i32, i32, i32) = core::mem::transmute(DAT_00732a30);
        let u_d = DAT_006d4a4e as u32;
        g5(
            i_var23,
            0,
            ((DAT_006db594 >> 1) as u32).wrapping_add(u_d.wrapping_mul(2u32.wrapping_neg())) as i32,
            DAT_006da538,
            u_d.wrapping_mul(4).wrapping_add(1) as i32,
        );
        FUN_0048ee20(c_var6);

        DAT_006d9980 = u_var3;
        DAT_006da9fa = u_var4;
        DAT_006ffbf0 = u_var5;
        DAT_0070acc0 = u_var7;
        DAT_0071b012 = u_var8;
        DAT_0071e524 = u_var9;
        DAT_00730894 = u_var10;
        DAT_007329c0 = u_var11;
        DAT_00732a34 = u_var12;
        DAT_00732e50 = u_var13;
        DAT_00732e52 = u_var14;
        DAT_00732e58 = u_var15;
        DAT_00732e8c = u_var16;

        FUN_004a6f90();
        let mut i_var21 = FUN_004a6a70();
        let mut i_var23 = i_var21;
        if 0 < i_var21 {
            loop {
                FUN_004a6f90();
                i_var23 = i_var23.wrapping_sub(1);
                if i_var23 == 0 {
                    break;
                }
            }
        }
        FUN_004a7130();
        FUN_004a7040();
        if 0 < i_var21 {
            loop {
                FUN_004a6a80();
                i_var21 = i_var21.wrapping_sub(1);
                if i_var21 == 0 {
                    break;
                }
            }
        }
        u_ret as u8
    }
}
