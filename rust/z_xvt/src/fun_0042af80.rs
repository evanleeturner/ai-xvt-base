// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d9d60: u16;
    static mut DAT_006d9d62: u16;
    static mut DAT_0070ad9e: [u8; 0];
    static mut DAT_0070ad9f: [u8; 0];
    static mut DAT_0070ada0: [u8; 0];
    static mut DAT_0070ada1: [u8; 0];
    static mut DAT_0070ada2: [u8; 0];
    static mut DAT_0070ad98: [u8; 0];
    static mut DAT_0070ad9a: [u8; 0];
    static mut DAT_0070ad99: [u8; 0];
    static mut DAT_0070ad9b: [u8; 0];
    static mut DAT_0070ad9c: [u8; 0];
    static mut unaff_EDI: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0049f010(param_1: u32, param_2: u16, param_3: u16) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042af80(param_1: u16) -> u32 {
    // SAFETY: the globals and the callee are the game's own memory; the
    // byte tables are indexed where the original indexes them.
    unsafe {
        let u_var6: u32 =
            (((unaff_EDI >> 16) as i16 as i32 as u32).wrapping_shl(16)) | (param_1 as u32);
        let idx: usize = (DAT_006d9d62 as u32)
            .wrapping_mul(0x52)
            .wrapping_add((DAT_006d9d60 as u32).wrapping_mul(0x566))
            as usize;

        let b = |p: *mut [u8; 0]| -> u16 { p.cast::<u8>().add(idx).read() as u16 };

        let mut ret = FUN_0049f010(u_var6, b(&raw mut DAT_0070ad9e), b(&raw mut DAT_0070ad9f));
        let mut u_var5 = ret as u16;

        ret = FUN_0049f010(u_var6, b(&raw mut DAT_0070ada0), b(&raw mut DAT_0070ada1));
        let mut u_var4 = ret as u16;

        if b(&raw mut DAT_0070ada2) == 1 {
            u_var5 |= u_var4;
        } else {
            u_var5 &= u_var4;
        }

        ret = FUN_0049f010(u_var6, b(&raw mut DAT_0070ad98), b(&raw mut DAT_0070ad9a));
        u_var4 = ret as u16;

        ret = FUN_0049f010(u_var6, b(&raw mut DAT_0070ad99), b(&raw mut DAT_0070ad9b));
        let mut u_var3 = ret as u16;

        if b(&raw mut DAT_0070ad9c) == 1 {
            u_var4 |= u_var3;
        } else {
            u_var4 &= u_var3;
        }

        u_var3 = (ret >> 16) as u16;

        if u_var5 == 0 && u_var4 == 0 {
            return (u_var3 as u32) << 16;
        }
        ((u_var3 as u32) << 16) | 1
    }
}
