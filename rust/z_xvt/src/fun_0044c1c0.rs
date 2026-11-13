// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d9d60: u16;
    static mut DAT_0070ad9e: u8;
    static mut DAT_0070ad9f: u8;
    static mut DAT_0070ada2: u8;
    static mut DAT_0070ada0: u8;
    static mut DAT_0070ada1: u8;
    static mut DAT_0070ad98: u8;
    static mut DAT_0070ad9a: u8;
    static mut DAT_0070ad9c: u8;
    static mut DAT_0070ad99: u8;
    static mut DAT_0070ad9b: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0044c2b0(a: u16, b: u16, c: u16, d: u16, e: u16) -> u16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0044c1c0(param_1: u32) {
    // SAFETY: the globals and the callee are the game's own; the arrays are
    // indexed by byte offset exactly as the original does.
    unsafe {
        let i_var2: u32 = (param_1 & 0xffff).wrapping_mul(0x52);
        let i_var1: u32 = i_var2.wrapping_add((DAT_006d9d60 as u32).wrapping_mul(0x566));
        let o = i_var1 as usize;
        let a = *(&raw mut DAT_0070ad9e).cast::<u8>().wrapping_add(o) as u16;
        let b = *(&raw mut DAT_0070ad9f).cast::<u8>().wrapping_add(o) as u16;
        let c = *(&raw mut DAT_0070ada2).cast::<u8>().wrapping_add(o) as u16;
        let d = *(&raw mut DAT_0070ada0).cast::<u8>().wrapping_add(o) as u16;
        let e = *(&raw mut DAT_0070ada1).cast::<u8>().wrapping_add(o) as u16;
        let u_var3 = FUN_0044c2b0(a, b, c, d, e);
        if u_var3 == 0xffff {
            let i_var1: u32 = i_var2.wrapping_add((DAT_006d9d60 as u32).wrapping_mul(0x566));
            let o = i_var1 as usize;
            let a = *(&raw mut DAT_0070ad98).cast::<u8>().wrapping_add(o) as u16;
            let b = *(&raw mut DAT_0070ad9a).cast::<u8>().wrapping_add(o) as u16;
            let c = *(&raw mut DAT_0070ad9c).cast::<u8>().wrapping_add(o) as u16;
            let d = *(&raw mut DAT_0070ad99).cast::<u8>().wrapping_add(o) as u16;
            let e = *(&raw mut DAT_0070ad9b).cast::<u8>().wrapping_add(o) as u16;
            FUN_0044c2b0(a, b, c, d, e);
        }
    }
}
