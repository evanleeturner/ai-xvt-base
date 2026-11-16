// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00524798: [u8; 0];
    static mut DAT_0070ad92: [u8; 0];
    static mut DAT_006d4988: [u8; 0];
    static mut DAT_006d9d62: i16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004296b0(param_1: u16);
    fn FUN_0042af80(param_1: u16) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042b2a0(param_1: i32, param_2: u32, param_3: u16) -> u32 {
    if param_1 == -1 {
        return 0;
    }
    let mut s_var2: i16 = 0;
    let mut u_var3: u32 = 0;
    // SAFETY: the globals and callees are the game's own memory and code;
    // offsets index the game's tables exactly as the original does.
    unsafe {
        FUN_004296b0(param_1 as u16);
        loop {
            let idx = (&raw mut DAT_006d4988)
                .cast::<u8>()
                .byte_add(0x16)
                .byte_offset(param_1.wrapping_mul(0x23) as isize)
                .read() as u32;
            let off = idx.wrapping_mul(0x566).wrapping_add(u_var3);
            let b = (&raw mut DAT_0070ad92)
                .cast::<u8>()
                .byte_add(off as usize)
                .read();
            let v = (&raw mut DAT_00524798)
                .cast::<u8>()
                .byte_add(b as usize)
                .read();
            if v as u32 == param_2 {
                DAT_006d9d62 = s_var2;
                let u_var1 = FUN_0042af80(param_3);
                if (u_var1 as i16) != 0 {
                    return 1;
                }
            }
            u_var3 = u_var3.wrapping_add(0x52);
            s_var2 = s_var2.wrapping_add(1);
            if u_var3 >= 0xf6 {
                break;
            }
        }
    }
    0
}
