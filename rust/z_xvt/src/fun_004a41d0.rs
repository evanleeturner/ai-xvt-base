// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071ecf4: i32;
    static mut DAT_006d4988: i32;
    static mut DAT_00732fe2: u16;
    static mut _DAT_00732950: u16;
    static mut _DAT_00732e70: i32;
    static mut _DAT_00732952: u16;
    static mut _DAT_00732e74: i32;
    static mut _DAT_00732954: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ac6f0(param_1: u32, param_2: u32) -> u32;
    fn FUN_004a1d70(param_1: i32, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a41d0(param_1: i32) {
    // SAFETY: all addresses are the game's own memory, read and written in
    // the same order as the original; the callees and globals are the game's.
    unsafe {
        let i_var2: i32 = ((0x71b730i32.wrapping_add(DAT_0071ecf4.wrapping_mul(0x5bd))
            as *const i32)
            .read_unaligned())
        .wrapping_mul(0x23)
        .wrapping_add(DAT_006d4988);
        let v1f: i32 = (i_var2.wrapping_add(0x1f) as *const i32).read_unaligned();
        DAT_00732fe2 = u16::from((v1f.wrapping_add(0x86) as *const u8).read());
        _DAT_00732950 = 0x8000;
        let va7: i32 = (v1f.wrapping_add(0xa7) as *const i32).read_unaligned();
        _DAT_00732e70 = 0x52b070i32.wrapping_add(
            (u32::from((va7.wrapping_add(4) as *const u8).read()) as i32).wrapping_mul(0xef),
        );
        _DAT_00732952 = 0x8001;
        let b16: u32 = u32::from((i_var2.wrapping_add(0x16) as *const u8).read());
        _DAT_00732e74 = 0x70acf0i32.wrapping_add((b16 as i32).wrapping_mul(0x566));
        let u_var1 = FUN_004ac6f0(b16, va7 as u32);
        _DAT_00732954 = u_var1 as u16;
        FUN_004a1d70(param_1, DAT_0071ecf4);
    }
}
