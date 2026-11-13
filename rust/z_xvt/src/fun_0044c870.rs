// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d9d60: u16;
    static mut DAT_0070ad9e: [u8; 0];
    static mut DAT_0070ad9f: [u8; 0];
    static mut DAT_0070ada0: [u8; 0];
    static mut DAT_0070ada1: [u8; 0];
    static mut DAT_0070ada2: [u8; 0];
    static mut DAT_0070ad98: [u8; 0];
    static mut DAT_0070ad99: [u8; 0];
    static mut DAT_0070ad9a: [u8; 0];
    static mut DAT_0070ad9b: [u8; 0];
    static mut DAT_0070ad9c: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0044c960(a: u16, b: u16, c: u16, d: u16, e: u16) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0044c870(param_1: u32) {
    // SAFETY: the globals and the callee are the game's own; the indexed
    // bytes lie within the game's tables, read where the original reads.
    unsafe {
        let i_var1 = ((param_1 & 0xffff) as i32).wrapping_mul(0x52);
        let mut i_var2 = i_var1.wrapping_add((DAT_006d9d60 as i32).wrapping_mul(0x566));
        let off = i_var2 as usize;
        i_var2 = FUN_0044c960(
            u16::from((&raw mut DAT_0070ad9e).cast::<u8>().add(off).read()),
            u16::from((&raw mut DAT_0070ad9f).cast::<u8>().add(off).read()),
            u16::from((&raw mut DAT_0070ada2).cast::<u8>().add(off).read()),
            u16::from((&raw mut DAT_0070ada0).cast::<u8>().add(off).read()),
            u16::from((&raw mut DAT_0070ada1).cast::<u8>().add(off).read()),
        );
        if (i_var2 as i16) == -1 {
            let i_var2 = i_var1.wrapping_add((DAT_006d9d60 as i32).wrapping_mul(0x566));
            let off = i_var2 as usize;
            FUN_0044c960(
                u16::from((&raw mut DAT_0070ad98).cast::<u8>().add(off).read()),
                u16::from((&raw mut DAT_0070ad9a).cast::<u8>().add(off).read()),
                u16::from((&raw mut DAT_0070ad9c).cast::<u8>().add(off).read()),
                u16::from((&raw mut DAT_0070ad99).cast::<u8>().add(off).read()),
                u16::from((&raw mut DAT_0070ad9b).cast::<u8>().add(off).read()),
            );
        }
    }
}
