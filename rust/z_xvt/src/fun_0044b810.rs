// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d9d60: u16;
    static mut DAT_0070ad9e: [u8; 0];
    static mut DAT_0070ad9f: [u8; 0];
    static mut DAT_0070ada2: [u8; 0];
    static mut DAT_0070ada0: [u8; 0];
    static mut DAT_0070ada1: [u8; 0];
    static mut DAT_0070ad98: [u8; 0];
    static mut DAT_0070ad9a: [u8; 0];
    static mut DAT_0070ad9c: [u8; 0];
    static mut DAT_0070ad99: [u8; 0];
    static mut DAT_0070ad9b: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0044b900(a: u16, b: u16, c: u16, d: u16, e: u16) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0044b810(param_1: u32) {
    // SAFETY: the globals and callee are the game's own; the arrays are
    // indexed at the same offsets the original uses.
    unsafe {
        let i_var2: u32 = (param_1 & 0xffff).wrapping_mul(0x52);
        let mut i_var1: u32 = i_var2.wrapping_add((DAT_006d9d60 as u32).wrapping_mul(0x566));
        let u_var3: u32 = FUN_0044b900(
            u16::from(
                (&raw mut DAT_0070ad9e)
                    .cast::<u8>()
                    .add(i_var1 as usize)
                    .read(),
            ),
            u16::from(
                (&raw mut DAT_0070ad9f)
                    .cast::<u8>()
                    .add(i_var1 as usize)
                    .read(),
            ),
            u16::from(
                (&raw mut DAT_0070ada2)
                    .cast::<u8>()
                    .add(i_var1 as usize)
                    .read(),
            ),
            u16::from(
                (&raw mut DAT_0070ada0)
                    .cast::<u8>()
                    .add(i_var1 as usize)
                    .read(),
            ),
            u16::from(
                (&raw mut DAT_0070ada1)
                    .cast::<u8>()
                    .add(i_var1 as usize)
                    .read(),
            ),
        );
        if (u_var3 as u16 as i16) == -1 {
            i_var1 = i_var2.wrapping_add((DAT_006d9d60 as u32).wrapping_mul(0x566));
            FUN_0044b900(
                u16::from(
                    (&raw mut DAT_0070ad98)
                        .cast::<u8>()
                        .add(i_var1 as usize)
                        .read(),
                ),
                u16::from(
                    (&raw mut DAT_0070ad9a)
                        .cast::<u8>()
                        .add(i_var1 as usize)
                        .read(),
                ),
                u16::from(
                    (&raw mut DAT_0070ad9c)
                        .cast::<u8>()
                        .add(i_var1 as usize)
                        .read(),
                ),
                u16::from(
                    (&raw mut DAT_0070ad99)
                        .cast::<u8>()
                        .add(i_var1 as usize)
                        .read(),
                ),
                u16::from(
                    (&raw mut DAT_0070ad9b)
                        .cast::<u8>()
                        .add(i_var1 as usize)
                        .read(),
                ),
            );
        }
    }
}
