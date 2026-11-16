// SPDX-License-Identifier: MIT

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
    static mut DAT_0071f2eb: u8;
    static mut DAT_006db594: i32;
    static mut DAT_006da538: i32;
    static mut DAT_006d4a4e: u8;
    static mut DAT_006a89ec: i32;
    static mut DAT_00703c80: *const core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048ed00(param_1: i16, param_2: i16, param_3: i16, param_4: i16);
    fn FUN_004a6a70() -> i32;
    fn FUN_004a6f90();
    fn FUN_004a6a80();
    fn FUN_0048ee20(param_1: i8);
    fn FUN_0048ece0(param_1: i16, param_2: i16);
    fn FUN_0048f270(param_1: *mut u8);
    fn FUN_004a7130();
    fn FUN_004a7040();
    fn FUN_004a5470() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0047ea40(param_1: *mut i8) -> i32 {
    let mut ab_stack_100 = [0u8; 256];
    // SAFETY: all globals and callees are the game's own memory and code;
    // param_1 points at a NUL-terminated string that is copied as the original does.
    unsafe {
        let u_var16 = DAT_00732e8c;
        let u_var15 = DAT_00732e58;
        let u_var14 = DAT_00732e52;
        let u_var13 = DAT_00732e50;
        let u_var12 = DAT_00732a34;
        let u_var11 = DAT_007329c0;
        let u_var10 = DAT_00730894;
        let u_var8 = DAT_0071e524;
        let u_var7 = DAT_0071b012;
        let u_var6 = DAT_0070acc0;
        let c_var5 = DAT_0070a188;
        let u_var4 = DAT_006ffbf0;
        let u_var3 = DAT_006da9fa;
        let u_var2 = DAT_006d9980;
        let mut i_var18 = FUN_004a6a70();
        let mut i_var22 = i_var18;
        if 0 < i_var18 {
            loop {
                FUN_004a6f90();
                i_var22 = i_var22.wrapping_sub(1);
                if i_var22 == 0 {
                    break;
                }
            }
        }
        let u_var9 = DAT_0071f2eb;
        DAT_0071f2eb = 1;
        FUN_004a6a80();
        FUN_0048ee20(1);
        let neg2 = (DAT_006d4a4e as u32).wrapping_mul(0xfffffffe) as u16;
        let pos2 = (DAT_006d4a4e as u32).wrapping_mul(2) as u16;
        let mut s_var17 = (DAT_006db594 >> 1) as i16;
        let mut s_var19 = (DAT_006da538 >> 4) as i16;
        FUN_0048ed00(
            s_var19,
            (s_var17 as u16).wrapping_add(neg2) as i16,
            (DAT_006da538 as i16 as u16).wrapping_sub(s_var19 as u16) as i16,
            pos2.wrapping_add(s_var17 as u16) as i16,
        );
        DAT_00732e50 = 0xf9;
        let f: unsafe extern "C" fn() = core::mem::transmute(DAT_00703c80);
        f();
        s_var19 = (DAT_006db594 >> 1) as i16;
        s_var17 = (DAT_006da538 >> 4) as i16;
        let neg2 = (DAT_006d4a4e as u32).wrapping_mul(0xfffffffe) as u16;
        let pos2 = (DAT_006d4a4e as u32).wrapping_mul(2) as u16;
        FUN_0048ed00(
            (s_var17 as u16).wrapping_add(1) as i16,
            (s_var19 as u16).wrapping_add(neg2).wrapping_add(1) as i16,
            ((DAT_006da538 as i16 as u16).wrapping_sub(s_var17 as u16)).wrapping_add(0xffff) as i16,
            pos2.wrapping_add(s_var19 as u16).wrapping_add(0xffff) as i16,
        );
        DAT_00732e50 = 0;
        let f: unsafe extern "C" fn() = core::mem::transmute(DAT_00703c80);
        f();
        DAT_006da9fa = 0xf9;
        DAT_0071b012 = 0;
        DAT_0071e524 = 0;
        FUN_0048ece0(
            0,
            ((((DAT_006db594 >> 1) as i16) as u16).wrapping_sub(DAT_006d4a4e as u16))
                .wrapping_add(0xfffe) as i16,
        );
        let mut len: u32 = 0;
        while *param_1.byte_add(len as usize) != 0 {
            len = len.wrapping_add(1);
        }
        len = len.wrapping_add(1);
        let dst = ab_stack_100.as_mut_ptr();
        let mut k: u32 = 0;
        while k < len {
            *dst.byte_add(k as usize) = *param_1.byte_add(k as usize) as u8;
            k = k.wrapping_add(1);
        }
        FUN_0048f270(dst);
        FUN_0048ece0(
            0,
            (((DAT_006db594 >> 1) as i16) as u16).wrapping_add(2) as i16,
        );
        FUN_0048f270(DAT_006a89ec as usize as *mut u8);
        FUN_004a6f90();
        FUN_004a7130();
        FUN_004a7040();
        let u_var20 = FUN_004a5470();
        DAT_0071f2eb = u_var9;
        if 0 < i_var18 {
            let mut n = i_var18;
            loop {
                FUN_004a6a80();
                n = n.wrapping_sub(1);
                if n == 0 {
                    break;
                }
            }
        }
        FUN_0048ee20(c_var5);
        DAT_006d9980 = u_var2;
        DAT_006da9fa = u_var3;
        DAT_006ffbf0 = u_var4;
        DAT_0070acc0 = u_var6;
        DAT_0071b012 = u_var7;
        DAT_0071e524 = u_var8;
        DAT_00730894 = u_var10;
        DAT_007329c0 = u_var11;
        DAT_00732a34 = u_var12;
        DAT_00732e50 = u_var13;
        DAT_00732e52 = u_var14;
        DAT_00732e58 = u_var15;
        DAT_00732e8c = u_var16;
        i_var18 = FUN_004a6a70();
        i_var22 = i_var18;
        if 0 < i_var18 {
            loop {
                FUN_004a6f90();
                i_var22 = i_var22.wrapping_sub(1);
                if i_var22 == 0 {
                    break;
                }
            }
        }
        FUN_004a7130();
        FUN_004a7040();
        if 0 < i_var18 {
            loop {
                FUN_004a6a80();
                i_var18 = i_var18.wrapping_sub(1);
                if i_var18 == 0 {
                    break;
                }
            }
        }
        (u_var20 as i8) as i32
    }
}
