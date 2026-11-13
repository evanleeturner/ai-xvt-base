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
    fn FUN_0042a3f0(a: u16, b: u16, c: u16, d: u16, e: u16) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00429ed0(param_1: u32) {
    // SAFETY: the globals and the callee are the game's own; the tables are
    // indexed by byte offsets exactly as the original does.
    unsafe {
        let i_var2: i32 = ((param_1 & 0xffff) as i32).wrapping_mul(0x52);
        let i_var1: i32 = i_var2.wrapping_add((DAT_006d9d60 as u32).wrapping_mul(0x566) as i32);
        let o = i_var1 as isize;
        let u_var3 = FUN_0042a3f0(
            (&raw mut DAT_0070ad9e).byte_offset(o).read() as u16,
            (&raw mut DAT_0070ad9f).byte_offset(o).read() as u16,
            (&raw mut DAT_0070ada2).byte_offset(o).read() as u16,
            (&raw mut DAT_0070ada0).byte_offset(o).read() as u16,
            (&raw mut DAT_0070ada1).byte_offset(o).read() as u16,
        );
        if (u_var3 as i16) == -1 {
            let i_var1: i32 = i_var2.wrapping_add((DAT_006d9d60 as u32).wrapping_mul(0x566) as i32);
            let o = i_var1 as isize;
            FUN_0042a3f0(
                (&raw mut DAT_0070ad98).byte_offset(o).read() as u16,
                (&raw mut DAT_0070ad9a).byte_offset(o).read() as u16,
                (&raw mut DAT_0070ad9c).byte_offset(o).read() as u16,
                (&raw mut DAT_0070ad99).byte_offset(o).read() as u16,
                (&raw mut DAT_0070ad9b).byte_offset(o).read() as u16,
            );
        }
    }
}
