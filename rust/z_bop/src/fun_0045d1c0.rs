// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a73c0: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0045d2b0(a: u16, b: u16, c: u16, d: u16, e: u16) -> u16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0045d1c0(param_1: u32) {
    // SAFETY: the game's globals and tables live at these fixed addresses in
    // its own memory; the callee is the game's own function.
    unsafe {
        let i_var2: u32 = (param_1 & 0xffff).wrapping_mul(0x52);
        let i_var1: u32 = i_var2.wrapping_add((DAT_009a73c0 as u32).wrapping_mul(0x566));
        let rd = |base: u32| -> u16 { u16::from((base.wrapping_add(i_var1) as *const u8).read()) };
        let u_var3 = FUN_0045d2b0(
            rd(0x009d8cde),
            rd(0x009d8cdf),
            rd(0x009d8ce2),
            rd(0x009d8ce0),
            rd(0x009d8ce1),
        );
        if u_var3 == 0xffff {
            FUN_0045d2b0(
                rd(0x009d8cd8),
                rd(0x009d8cda),
                rd(0x009d8cdc),
                rd(0x009d8cd9),
                rd(0x009d8cdb),
            );
        }
    }
}
