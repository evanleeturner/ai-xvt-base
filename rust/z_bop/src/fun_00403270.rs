// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a73c0: u16;
    static mut DAT_009d8cde: [u8; 1];
    static mut DAT_009d8cdf: [u8; 1];
    static mut DAT_009d8ce2: [u8; 1];
    static mut DAT_009d8ce0: [u8; 1];
    static mut DAT_009d8ce1: [u8; 1];
    static mut DAT_009d8cd8: [u8; 1];
    static mut DAT_009d8cda: [u8; 1];
    static mut DAT_009d8cdc: [u8; 1];
    static mut DAT_009d8cd9: [u8; 1];
    static mut DAT_009d8cdb: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004039e0(a: u16, b: u16, c: u16, d: u16, e: u16) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00403270(param_1: u32) {
    // SAFETY: the globals and the callee belong to the game; the byte
    // tables are indexed by offset exactly as the original does.
    unsafe {
        let i_var2: u32 = (param_1 & 0xffff).wrapping_mul(0x52);
        let mut i_var1: u32 = i_var2.wrapping_add((DAT_009a73c0 as u32).wrapping_mul(0x566));
        let u_var3 = FUN_004039e0(
            (&raw mut DAT_009d8cde)
                .cast::<u8>()
                .add(i_var1 as usize)
                .read() as u16,
            (&raw mut DAT_009d8cdf)
                .cast::<u8>()
                .add(i_var1 as usize)
                .read() as u16,
            (&raw mut DAT_009d8ce2)
                .cast::<u8>()
                .add(i_var1 as usize)
                .read() as u16,
            (&raw mut DAT_009d8ce0)
                .cast::<u8>()
                .add(i_var1 as usize)
                .read() as u16,
            (&raw mut DAT_009d8ce1)
                .cast::<u8>()
                .add(i_var1 as usize)
                .read() as u16,
        );
        if (u_var3 as u16 as i16) == -1 {
            i_var1 = i_var2.wrapping_add((DAT_009a73c0 as u32).wrapping_mul(0x566));
            FUN_004039e0(
                (&raw mut DAT_009d8cd8)
                    .cast::<u8>()
                    .add(i_var1 as usize)
                    .read() as u16,
                (&raw mut DAT_009d8cda)
                    .cast::<u8>()
                    .add(i_var1 as usize)
                    .read() as u16,
                (&raw mut DAT_009d8cdc)
                    .cast::<u8>()
                    .add(i_var1 as usize)
                    .read() as u16,
                (&raw mut DAT_009d8cd9)
                    .cast::<u8>()
                    .add(i_var1 as usize)
                    .read() as u16,
                (&raw mut DAT_009d8cdb)
                    .cast::<u8>()
                    .add(i_var1 as usize)
                    .read() as u16,
            );
        }
    }
}
