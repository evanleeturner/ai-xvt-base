// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009e9a5b: u8;
    static mut DAT_009e9a5f: u8;
    static mut DAT_009e9a63: u8;
    static mut DAT_009ecc34: i32;
    static mut DAT_00a07cd4: i32;
    static mut DAT_00a07cd8: i32;
    static mut DAT_00a07cdc: i32;
    static mut DAT_009fd39c: i32;
    static mut DAT_009fd3a0: i32;
    static mut DAT_009fd398: i32;
    static mut DAT_009a8e34: u32;
    static mut DAT_009a8e2c: u32;
    static mut DAT_009a8e30: u32;
    static mut DAT_00a004b0: u32;
    static mut DAT_00a004a8: u32;
    static mut DAT_00a004a4: u32;
    static mut DAT_009fe7f4: u32;
    static mut DAT_00a004cc: u32;
    static mut DAT_00a004b8: u32;
    static mut DAT_00a0813c: i16;
    static mut DAT_009a6fe0: i16;
    static mut DAT_009cc454: *const core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00427500(a: i32, b: i32, c: i32) -> u32;
    fn FUN_00427420(a: i32, b: i32, c: i32) -> u32;
    fn FUN_00427490(a: i32, b: i32, c: i32) -> u32;
    fn FUN_00427390(a: u32, b: u32, c: u32);
    fn FUN_004aba50();
    fn FUN_00427590(a: u32, b: u32) -> i32;
    fn FUN_00427570(a: u32, b: u32) -> i32;
    fn FUN_004abe50();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00437b20() {
    // SAFETY: all globals and callees are the game's own memory and code;
    // the byte reads stay at the offsets the original uses.
    unsafe {
        let mut u_var1: u32;
        let mut u_var2: u32;
        let mut i_var3: i32;
        let mut i_var4: i32;
        let mut i_var5: i32;
        let mut i_var6: i32;
        let mut u_var7: u32;
        let mut i_var8: i32;
        let mut i_var9: i32;
        let mut u_var10: u32;
        let mut u_var11: u32;
        let mut u_var12: u32;
        let mut local_c: i32;
        let mut local_8: i32;
        let mut local_4: i32;

        DAT_00a07cd4 = 0;
        DAT_00a07cd8 = 0;
        DAT_00a07cdc = 0;
        local_8 = -0x100000;
        local_c = 0x21;
        loop {
            let off = (DAT_009ecc34 as u32).wrapping_mul(0x5bd) as i32 as isize;
            let tmp = (&raw mut DAT_009e9a5b)
                .byte_offset(off)
                .cast::<i32>()
                .read_unaligned();
            DAT_009fd39c = 0xfff00000u32.wrapping_sub(tmp as u32) as i32;
            let off = (DAT_009ecc34 as u32).wrapping_mul(0x5bd) as i32 as isize;
            let tmp = (&raw mut DAT_009e9a5f)
                .byte_offset(off)
                .cast::<i32>()
                .read_unaligned();
            DAT_009fd3a0 = (local_8 as u32).wrapping_sub(tmp as u32) as i32;
            let off = (DAT_009ecc34 as u32).wrapping_mul(0x5bd) as i32 as isize;
            let tmp = (&raw mut DAT_009e9a63)
                .byte_offset(off)
                .cast::<i32>()
                .read_unaligned();
            DAT_009fd398 = 0xffff0000u32.wrapping_sub(tmp as u32) as i32;
            local_8 = (local_8 as u32).wrapping_add(0x10000) as i32;
            DAT_009a8e34 = FUN_00427500(DAT_009fd39c, DAT_009fd3a0, DAT_009fd398);
            u_var11 = DAT_00a004b0.wrapping_mul(0x40).wrapping_add(DAT_009a8e34);
            if (0 < u_var11 as i32) || (0 < DAT_009a8e34 as i32) {
                DAT_009a8e2c = FUN_00427420(DAT_009fd39c, DAT_009fd3a0, DAT_009fd398);
                DAT_009a8e30 = FUN_00427490(DAT_009fd39c, DAT_009fd3a0, DAT_009fd398);
                u_var1 = DAT_00a004a8.wrapping_mul(0x40).wrapping_add(DAT_009a8e2c);
                u_var2 = DAT_00a004a4.wrapping_mul(0x40).wrapping_add(DAT_009a8e30);
                u_var7 = DAT_009a8e34;
                u_var10 = u_var2;
                u_var12 = u_var1;
                if (u_var11 as i32) < 1 {
                    u_var7 = u_var11;
                    u_var10 = DAT_009a8e30;
                    u_var11 = DAT_009a8e34;
                    u_var12 = DAT_009a8e2c;
                    DAT_009a8e2c = u_var1;
                    DAT_009a8e30 = u_var2;
                }
                DAT_009a8e34 = u_var7;
                if (u_var7 as i32) < 1 {
                    FUN_00427390(u_var12, u_var10, u_var11);
                }
                FUN_004aba50();
                i_var8 = DAT_00a0813c as i32;
                i_var3 = DAT_009a6fe0 as i32;
                let u_var13: u32 = 0x31;
                i_var4 = FUN_00427590(u_var10, u_var11);
                i_var4 = i_var4.wrapping_add(i_var8);
                i_var5 = FUN_00427570(u_var12, u_var11);
                u_var11 = DAT_009a8e34;
                i_var5 = i_var5.wrapping_add(i_var3);
                i_var6 = FUN_00427590(DAT_009a8e30, DAT_009a8e34);
                i_var6 = i_var6.wrapping_add(i_var8);
                i_var8 = FUN_00427570(DAT_009a8e2c, u_var11);
                let f: unsafe extern "C" fn(i32, i32, i32, i32, u32) =
                    core::mem::transmute(DAT_009cc454);
                f(i_var8.wrapping_add(i_var3), i_var6, i_var5, i_var4, u_var13);
                FUN_004abe50();
            }
            local_c = local_c.wrapping_sub(1);
            if local_c == 0 {
                break;
            }
        }
        local_c = 0x21;
        i_var3 = (DAT_00a07cd8 as u32).wrapping_sub(0x100000) as i32;
        local_4 = (DAT_00a07cdc as u32).wrapping_sub(0x100000) as i32;
        loop {
            let off = (DAT_009ecc34 as u32).wrapping_mul(0x5bd) as i32 as isize;
            let tmp = (&raw mut DAT_009e9a63)
                .byte_offset(off)
                .cast::<i32>()
                .read_unaligned();
            DAT_009fd398 = 0xffff0000u32.wrapping_sub(tmp as u32) as i32;
            let off = (DAT_009ecc34 as u32).wrapping_mul(0x5bd) as i32 as isize;
            let tmp = (&raw mut DAT_009e9a5b)
                .byte_offset(off)
                .cast::<i32>()
                .read_unaligned();
            DAT_009fd39c = (local_4 as u32).wrapping_sub(tmp as u32) as i32;
            let off = (DAT_009ecc34 as u32).wrapping_mul(0x5bd) as i32 as isize;
            let tmp = (&raw mut DAT_009e9a5f)
                .byte_offset(off)
                .cast::<i32>()
                .read_unaligned();
            DAT_009fd3a0 = (i_var3 as u32).wrapping_sub(tmp as u32) as i32;
            DAT_009a8e34 = FUN_00427500(DAT_009fd39c, DAT_009fd3a0, DAT_009fd398);
            u_var11 = DAT_009fe7f4.wrapping_mul(0x40).wrapping_add(DAT_009a8e34);
            if (0 < u_var11 as i32) || (0 < DAT_009a8e34 as i32) {
                DAT_009a8e2c = FUN_00427420(DAT_009fd39c, DAT_009fd3a0, DAT_009fd398);
                DAT_009a8e30 = FUN_00427490(DAT_009fd39c, DAT_009fd3a0, DAT_009fd398);
                u_var1 = DAT_00a004cc.wrapping_mul(0x40).wrapping_add(DAT_009a8e2c);
                u_var2 = DAT_00a004b8.wrapping_mul(0x40).wrapping_add(DAT_009a8e30);
                u_var7 = DAT_009a8e34;
                u_var10 = u_var2;
                u_var12 = u_var1;
                if (u_var11 as i32) < 1 {
                    u_var7 = u_var11;
                    u_var10 = DAT_009a8e30;
                    u_var11 = DAT_009a8e34;
                    u_var12 = DAT_009a8e2c;
                    DAT_009a8e2c = u_var1;
                    DAT_009a8e30 = u_var2;
                }
                DAT_009a8e34 = u_var7;
                if (u_var7 as i32) < 1 {
                    FUN_00427390(u_var12, u_var10, u_var11);
                }
                FUN_004aba50();
                i_var9 = DAT_00a0813c as i32;
                i_var4 = DAT_009a6fe0 as i32;
                let u_var13: u32 = 0x31;
                i_var5 = FUN_00427590(u_var10, u_var11);
                i_var5 = i_var5.wrapping_add(i_var9);
                i_var6 = FUN_00427570(u_var12, u_var11);
                u_var11 = DAT_009a8e34;
                i_var6 = i_var6.wrapping_add(i_var4);
                i_var8 = FUN_00427590(DAT_009a8e30, DAT_009a8e34);
                i_var8 = i_var8.wrapping_add(i_var9);
                i_var9 = FUN_00427570(DAT_009a8e2c, u_var11);
                let f: unsafe extern "C" fn(i32, i32, i32, i32, u32) =
                    core::mem::transmute(DAT_009cc454);
                f(i_var9.wrapping_add(i_var4), i_var8, i_var6, i_var5, u_var13);
                FUN_004abe50();
            }
            local_c = local_c.wrapping_sub(1);
            local_4 = (local_4 as u32).wrapping_add(0x10000) as i32;
            if local_c == 0 {
                break;
            }
        }
    }
}
