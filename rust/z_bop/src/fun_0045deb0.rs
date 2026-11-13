// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a73c0: u16;
    static mut DAT_009d8cde: [u8; 0];
    static mut DAT_009d8cdf: [u8; 0];
    static mut DAT_009d8ce2: [u8; 0];
    static mut DAT_009d8ce0: [u8; 0];
    static mut DAT_009d8ce1: [u8; 0];
    static mut DAT_009d8cd8: [u8; 0];
    static mut DAT_009d8cda: [u8; 0];
    static mut DAT_009d8cdc: [u8; 0];
    static mut DAT_009d8cd9: [u8; 0];
    static mut DAT_009d8cdb: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0045dfa0(a: u16, b: u16, c: u16, d: u16, e: u16) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0045deb0(param_1: u32) {
    // SAFETY: the globals are the game's own tables, indexed as the original
    // does; the callee is the game's function.
    unsafe {
        let i_var1: u32 = (param_1 & 0xffff).wrapping_mul(0x52);
        let mut i_var2: u32 = i_var1.wrapping_add((DAT_009a73c0 as u32).wrapping_mul(0x566));
        let a = (&raw mut DAT_009d8cde)
            .cast::<u8>()
            .add(i_var2 as usize)
            .read() as u16;
        let b = (&raw mut DAT_009d8cdf)
            .cast::<u8>()
            .add(i_var2 as usize)
            .read() as u16;
        let c = (&raw mut DAT_009d8ce2)
            .cast::<u8>()
            .add(i_var2 as usize)
            .read() as u16;
        let d = (&raw mut DAT_009d8ce0)
            .cast::<u8>()
            .add(i_var2 as usize)
            .read() as u16;
        let idx5 = (DAT_009a73c0 as u32)
            .wrapping_mul(0x566)
            .wrapping_add(i_var1);
        let e = (&raw mut DAT_009d8ce1)
            .cast::<u8>()
            .add(idx5 as usize)
            .read() as u16;
        i_var2 = FUN_0045dfa0(a, b, c, d, e) as u32;
        if (i_var2 as i16) == -1 {
            i_var2 = i_var1.wrapping_add((DAT_009a73c0 as u32).wrapping_mul(0x566));
            let a = (&raw mut DAT_009d8cd8)
                .cast::<u8>()
                .add(i_var2 as usize)
                .read() as u16;
            let b = (&raw mut DAT_009d8cda)
                .cast::<u8>()
                .add(i_var2 as usize)
                .read() as u16;
            let c = (&raw mut DAT_009d8cdc)
                .cast::<u8>()
                .add(i_var2 as usize)
                .read() as u16;
            let d = (&raw mut DAT_009d8cd9)
                .cast::<u8>()
                .add(i_var2 as usize)
                .read() as u16;
            let idx5 = (DAT_009a73c0 as u32)
                .wrapping_mul(0x566)
                .wrapping_add(i_var1);
            let e = (&raw mut DAT_009d8cdb)
                .cast::<u8>()
                .add(idx5 as usize)
                .read() as u16;
            FUN_0045dfa0(a, b, c, d, e);
        }
    }
}
