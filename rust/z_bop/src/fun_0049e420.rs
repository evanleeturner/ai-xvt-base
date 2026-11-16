// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a20ae: u8;
    static mut DAT_009a6fe0: u16;
    static mut DAT_009a7bac: i32;
    static mut DAT_009a807a: u8;
    static mut DAT_009a8c14: i32;
    static mut DAT_009cd270: u16;
    static mut DAT_009d1300: *const core::ffi::c_void;
    static mut DAT_009d80c8: i8;
    static mut DAT_009d8c00: u16;
    static mut DAT_009e8f52: u8;
    static mut DAT_009ec464: u8;
    static mut DAT_009ed23b: u8;
    static mut DAT_009fe7e4: u16;
    static mut DAT_00a07c64: u16;
    static mut DAT_00a07ce4: u16;
    static mut DAT_00a08100: u8;
    static mut DAT_00a08102: u16;
    static mut DAT_00a08108: u16;
    static mut DAT_00a0813c: u16;
    static mut DAT_00a60a4c: *mut u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004aba40() -> i32;
    fn FUN_004abe50();
    fn FUN_004aba50();
    fn FUN_004a9640(param_1: i8);
    fn FUN_004a9520(a: i16, b: i16, c: i16, d: i16);
    fn FUN_004a9500(a: i16, b: i16);
    fn FUN_004a9a50(a: *mut u8);
    fn FUN_004ac260();
    fn FUN_004abee0();
    fn FUN_004aa870() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0049e420(param_1: *mut i8) -> i32 {
    let mut abstack: [u8; 256] = [0; 256];
    // SAFETY: all globals and callees are the game's own; param_1 is a
    // NUL-terminated string copied into the local buffer as the original does.
    unsafe {
        let u_var16 = DAT_00a0813c;
        let u_var15 = DAT_00a08108;
        let u_var14 = DAT_00a08102;
        let u_var13 = DAT_00a08100;
        let u_var12 = DAT_00a07ce4;
        let u_var11 = DAT_00a07c64;
        let u_var10 = DAT_009fe7e4;
        let u_var8 = DAT_009ec464;
        let u_var7 = DAT_009e8f52;
        let u_var6 = DAT_009d8c00;
        let c_var5 = DAT_009d80c8;
        let u_var4 = DAT_009cd270;
        let u_var3 = DAT_009a807a;
        let u_var2 = DAT_009a6fe0;
        let mut i_var17 = FUN_004aba40();
        let mut i_var22 = i_var17;
        while i_var22 > 0 {
            FUN_004abe50();
            i_var22 -= 1;
        }
        let u_var9 = DAT_009ed23b;
        DAT_009ed23b = 1;
        FUN_004aba50();
        FUN_004a9640(1);
        let mut s18 = (DAT_009a8c14 >> 1) as i16;
        let mut s21 = (DAT_009a7bac >> 4) as i16;
        let a = DAT_009a20ae as i32;
        FUN_004a9520(
            s21,
            (s18 as i32).wrapping_add(a.wrapping_mul(-2)) as i16,
            ((DAT_009a7bac as i16 as i32).wrapping_sub(s21 as i32)) as i16,
            a.wrapping_mul(2).wrapping_add(s18 as i32) as i16,
        );
        DAT_00a08100 = 0xf9;
        let f: unsafe extern "C" fn() = core::mem::transmute(DAT_009d1300);
        f();
        s21 = (DAT_009a8c14 >> 1) as i16;
        s18 = (DAT_009a7bac >> 4) as i16;
        let a = DAT_009a20ae as i32;
        FUN_004a9520(
            s18.wrapping_add(1),
            (s21 as i32)
                .wrapping_add(a.wrapping_mul(-2))
                .wrapping_add(1) as i16,
            ((DAT_009a7bac as i16 as i32).wrapping_sub(s18 as i32)).wrapping_sub(1) as i16,
            a.wrapping_mul(2).wrapping_add(s21 as i32).wrapping_sub(1) as i16,
        );
        DAT_00a08100 = 0;
        let f: unsafe extern "C" fn() = core::mem::transmute(DAT_009d1300);
        f();
        DAT_009a807a = 0xf9;
        DAT_009e8f52 = 0;
        DAT_009ec464 = 0;
        FUN_004a9500(
            0,
            ((((DAT_009a8c14 >> 1) as i16) as i32)
                .wrapping_sub(DAT_009a20ae as i32)
                .wrapping_sub(2)) as i16,
        );
        let mut len: usize = 0;
        while *param_1.add(len) != 0 {
            len += 1;
        }
        let total = len + 1;
        let dst = abstack.as_mut_ptr();
        let mut k = 0;
        while k < total {
            *dst.add(k) = *param_1.add(k) as u8;
            k += 1;
        }
        FUN_004a9a50(dst);
        FUN_004a9500(
            0,
            ((((DAT_009a8c14 >> 1) as i16) as i32).wrapping_add(2)) as i16,
        );
        FUN_004a9a50(DAT_00a60a4c);
        FUN_004abe50();
        FUN_004ac260();
        FUN_004abee0();
        let u_var19 = FUN_004aa870();
        DAT_009ed23b = u_var9;
        while i_var17 > 0 {
            FUN_004aba50();
            i_var17 -= 1;
        }
        FUN_004a9640(c_var5);
        DAT_009a6fe0 = u_var2;
        DAT_009a807a = u_var3;
        DAT_009cd270 = u_var4;
        DAT_009d8c00 = u_var6;
        DAT_009e8f52 = u_var7;
        DAT_009ec464 = u_var8;
        DAT_009fe7e4 = u_var10;
        DAT_00a07c64 = u_var11;
        DAT_00a07ce4 = u_var12;
        DAT_00a08100 = u_var13;
        DAT_00a08102 = u_var14;
        DAT_00a08108 = u_var15;
        DAT_00a0813c = u_var16;
        i_var17 = FUN_004aba40();
        i_var22 = i_var17;
        while i_var22 > 0 {
            FUN_004abe50();
            i_var22 -= 1;
        }
        FUN_004ac260();
        FUN_004abee0();
        while i_var17 > 0 {
            FUN_004aba50();
            i_var17 -= 1;
        }
        (u_var19 as u8 as i8) as i32
    }
}
