// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d9d60: u16;
    static mut DAT_0070ad98: u8;
    static mut DAT_0070ad99: u8;
    static mut DAT_0070ad9a: u8;
    static mut DAT_0070ad9b: u8;
    static mut DAT_0070ad9c: u8;
    static mut DAT_0070ad9e: u8;
    static mut DAT_0070ad9f: u8;
    static mut DAT_0070ada0: u8;
    static mut DAT_0070ada1: u8;
    static mut DAT_0070ada2: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0044fe90(param_1: u16, param_2: u16, param_3: u16, param_4: u16, param_5: u16) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0044fd90(param_1: u32) -> i32 {
    // SAFETY: the globals and the callee are the game's own; the table
    // reads use the same offsets as the original.
    unsafe {
        let i_var2: i32 = ((param_1 & 0xffff) as i32).wrapping_mul(0x52);
        let i_var1: i32 = i_var2.wrapping_add((DAT_006d9d60 as u32).wrapping_mul(0x566) as i32);
        let u_var3 = FUN_0044fe90(
            (&raw mut DAT_0070ad9e).offset(i_var1 as isize).read() as u16,
            (&raw mut DAT_0070ad9f).offset(i_var1 as isize).read() as u16,
            (&raw mut DAT_0070ada2).offset(i_var1 as isize).read() as u16,
            (&raw mut DAT_0070ada0).offset(i_var1 as isize).read() as u16,
            (&raw mut DAT_0070ada1).offset(i_var1 as isize).read() as u16,
        );
        if (u_var3 as i16) != 0 {
            return 1;
        }
        let i_var1: i32 = i_var2.wrapping_add((DAT_006d9d60 as u32).wrapping_mul(0x566) as i32);
        let u_var3 = FUN_0044fe90(
            (&raw mut DAT_0070ad98).offset(i_var1 as isize).read() as u16,
            (&raw mut DAT_0070ad9a).offset(i_var1 as isize).read() as u16,
            (&raw mut DAT_0070ad9c).offset(i_var1 as isize).read() as u16,
            (&raw mut DAT_0070ad99).offset(i_var1 as isize).read() as u16,
            (&raw mut DAT_0070ad9b).offset(i_var1 as isize).read() as u16,
        );
        if (u_var3 as i16) == 0 { 0 } else { 1 }
    }
}
