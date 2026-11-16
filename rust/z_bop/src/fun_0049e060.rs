// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a20ae: u8;
    static mut DAT_009a8c14: i32;
    static mut DAT_009a8c34: i32;
    static mut DAT_005233d8: i32;
    static mut DAT_009a7bac: i32;
    static mut DAT_009d8b6c: unsafe extern "C" fn(i32, i32, i32, i32, u32);
    static mut DAT_00a07ce0: unsafe extern "C" fn(i32, i32, i32, i32, u32);
    static mut DAT_009d1300: unsafe extern "C" fn();

    static mut DAT_009a6fe0: u16;
    static mut DAT_009a807a: u8;
    static mut DAT_009cd270: u16;
    static mut DAT_009d80c8: i8;
    static mut DAT_009d8c00: u16;
    static mut DAT_009e8f52: u8;
    static mut DAT_009ec464: u8;
    static mut DAT_009fe7e4: u16;
    static mut DAT_00a07c64: u16;
    static mut DAT_00a07ce4: u16;
    static mut DAT_00a08100: u8;
    static mut DAT_00a08102: u16;
    static mut DAT_00a08108: u16;
    static mut DAT_00a0813c: u16;

    static mut DAT_009d8a60: [i8; 0];
    static mut DAT_0051bf3c: [i8; 0];
    static mut DAT_00a609d4: *mut i8;
    static mut DAT_00a609d8: *mut u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004aba50();
    fn FUN_004aba40() -> i32;
    fn FUN_004abe50();
    fn FUN_004ac260();
    fn FUN_004abee0();
    fn FUN_004aa870() -> u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a9640(param_1: i8);
    fn FUN_004a9520(a: i16, b: i16, c: i16, d: i16);
    fn FUN_004a9500(a: i16, b: i16);
    fn FUN_004a9a50(c: *mut u8);
}

/// Copies the NUL-terminated string at `src` (terminator included) to `dst`.
unsafe fn copy_cstr(mut dst: *mut u8, mut src: *const u8) {
    // SAFETY: caller supplies valid NUL-terminated source and writable dest.
    unsafe {
        loop {
            let b = *src;
            *dst = b;
            src = src.add(1);
            dst = dst.add(1);
            if b == 0 {
                break;
            }
        }
    }
}

/// Returns pointer to the NUL terminator.
unsafe fn find_nul(mut p: *const u8) -> *mut u8 {
    // SAFETY: caller supplies a NUL-terminated string.
    unsafe {
        while *p != 0 {
            p = p.add(1);
        }
    }
    p as *mut u8
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0049e060() -> u8 {
    // SAFETY: all globals and callees are the game's own; string pointers
    // reference the game's NUL-terminated strings, copied into a local buffer
    // exactly as the original does.
    unsafe {
        let u_var16 = DAT_00a0813c;
        let u_var15 = DAT_00a08108;
        let u_var14 = DAT_00a08102;
        let u_var13 = DAT_00a08100;
        let u_var12 = DAT_00a07ce4;
        let u_var11 = DAT_00a07c64;
        let u_var10 = DAT_009fe7e4;
        let u_var9 = DAT_009ec464;
        let u_var8 = DAT_009e8f52;
        let u_var7 = DAT_009d8c00;
        let c_var6 = DAT_009d80c8;
        let u_var5 = DAT_009cd270;
        let u_var4 = DAT_009a807a;
        let u_var3 = DAT_009a6fe0;

        let mut abstack: [u8; 256] = [0u8; 256];

        FUN_004aba50();
        FUN_004a9640(1);
        let mut u_var19: u32 = DAT_009a20ae as u32;
        let i_var23: i32 = (DAT_009a8c34 as u32).wrapping_add(
            ((DAT_009a8c14 as u32)
                .wrapping_add(u_var19.wrapping_mul(0xFFFF_FFFC))
                .wrapping_add(0xFFFF_FFFF))
            .wrapping_mul(DAT_005233d8 as u32)
            .wrapping_mul(DAT_009a7bac as u32),
        ) as i32;
        let f = DAT_009d8b6c;
        f(
            i_var23,
            0,
            ((DAT_009a8c14 >> 1) as u32).wrapping_add(u_var19.wrapping_mul(0xFFFF_FFFE)) as i32,
            DAT_009a7bac,
            u_var19.wrapping_mul(4).wrapping_add(1),
        );
        let mut s_var22: i16 = (DAT_009a8c14 >> 1) as i16;
        let mut s_var17: i16 = (DAT_009a7bac >> 4) as i16;
        FUN_004a9520(
            s_var17,
            (s_var22 as i32 as u32).wrapping_add((DAT_009a20ae as u32).wrapping_mul(0xFFFF_FFFE))
                as i16,
            (DAT_009a7bac as i16).wrapping_sub(s_var17),
            ((DAT_009a20ae as u32)
                .wrapping_mul(2)
                .wrapping_add(s_var22 as i32 as u32)) as i16,
        );
        DAT_00a08100 = 0xf9;
        let g = DAT_009d1300;
        g();
        s_var22 = (DAT_009a8c14 >> 1) as i16;
        s_var17 = (DAT_009a7bac >> 4) as i16;
        FUN_004a9520(
            s_var17.wrapping_add(1),
            (s_var22 as i32 as u32)
                .wrapping_add((DAT_009a20ae as u32).wrapping_mul(0xFFFF_FFFE))
                .wrapping_add(1) as i16,
            (DAT_009a7bac as i16).wrapping_sub(s_var17).wrapping_add(-1),
            ((DAT_009a20ae as u32)
                .wrapping_mul(2)
                .wrapping_add(s_var22 as i32 as u32)
                .wrapping_add(0xFFFF_FFFF)) as i16,
        );
        DAT_00a08100 = 0;
        let g = DAT_009d1300;
        g();
        DAT_009a807a = 0xf9;
        DAT_009e8f52 = 0;
        DAT_009ec464 = 0;
        FUN_004a9500(
            0,
            (((DAT_009a8c14 >> 1) as i16 as i32).wrapping_sub(DAT_009a20ae as i32)).wrapping_add(-2)
                as i16,
        );

        // strcpy(buf, DAT_009d8a60)
        copy_cstr(abstack.as_mut_ptr(), (&raw mut DAT_009d8a60) as *const u8);
        // strcat(buf, DAT_0051bf3c)
        let end = find_nul(abstack.as_ptr());
        copy_cstr(end, (&raw mut DAT_0051bf3c) as *const u8);
        // strcat(buf, DAT_00a609d4)
        let end = find_nul(abstack.as_ptr());
        copy_cstr(end, DAT_00a609d4 as *const u8);
        FUN_004a9a50(abstack.as_mut_ptr());

        FUN_004a9500(0, ((DAT_009a8c14 >> 1) as i16).wrapping_add(2));
        FUN_004a9a50(DAT_00a609d8);
        let mut i_var18 = FUN_004aba40();
        let mut i_var21 = i_var18;
        if 0 < i_var18 {
            loop {
                FUN_004abe50();
                i_var21 = i_var21.wrapping_sub(1);
                if i_var21 == 0 {
                    break;
                }
            }
        }
        FUN_004ac260();
        FUN_004abee0();
        u_var19 = FUN_004aa870();
        if 0 < i_var18 {
            loop {
                FUN_004aba50();
                i_var18 = i_var18.wrapping_sub(1);
                if i_var18 == 0 {
                    break;
                }
            }
        }
        let h = DAT_00a07ce0;
        h(
            i_var23,
            0,
            ((DAT_009a8c14 >> 1) as u32)
                .wrapping_add((DAT_009a20ae as u32).wrapping_mul(0xFFFF_FFFE)) as i32,
            DAT_009a7bac,
            (DAT_009a20ae as u32).wrapping_mul(4).wrapping_add(1),
        );
        FUN_004a9640(c_var6);
        DAT_009a6fe0 = u_var3;
        DAT_009a807a = u_var4;
        DAT_009cd270 = u_var5;
        DAT_009d8c00 = u_var7;
        DAT_009e8f52 = u_var8;
        DAT_009ec464 = u_var9;
        DAT_009fe7e4 = u_var10;
        DAT_00a07c64 = u_var11;
        DAT_00a07ce4 = u_var12;
        DAT_00a08100 = u_var13;
        DAT_00a08102 = u_var14;
        DAT_00a08108 = u_var15;
        DAT_00a0813c = u_var16;
        FUN_004abe50();
        let mut i_var21 = FUN_004aba40();
        let mut i_var23b = i_var21;
        if 0 < i_var21 {
            loop {
                FUN_004abe50();
                i_var23b = i_var23b.wrapping_sub(1);
                if i_var23b == 0 {
                    break;
                }
            }
        }
        FUN_004ac260();
        FUN_004abee0();
        if 0 < i_var21 {
            loop {
                FUN_004aba50();
                i_var21 = i_var21.wrapping_sub(1);
                if i_var21 == 0 {
                    break;
                }
            }
        }
        u_var19 as u8
    }
}
