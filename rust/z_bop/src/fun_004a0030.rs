// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a73b0: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a44d0(param_1: i16, param_2: i32);
    fn FUN_004a4d70(param_1: u32, param_2: u16) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004a0030() -> u32 {
    // SAFETY: the callees and the global are the game's own.
    unsafe {
        FUN_004a44d0(0x8000u16 as i16, 1);
        let u_var1 = FUN_004a4d70(DAT_009a73b0 as u32, 0xffff);
        u_var1 & 0xffff0000
    }
}
