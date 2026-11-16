// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00759b0c: u32;
    static mut DAT_00759b18: u32;
    static mut DAT_00759b20: u32;
    static mut DAT_00759b24: u32;
    static mut DAT_00759b2c: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cfbc0(param_1: u32, param_2: u32, param_3: u8) -> u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetTickCount() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004cfff0() -> u32 {
    // SAFETY: the callees and the globals are the game's own memory; the
    // divisors are non-zero constants.
    unsafe {
        FUN_004cfbc0(
            DAT_00759b0c,
            DAT_00759b24 / 60000,
            ((DAT_00759b24 % 60000) / 1000) as u8,
        );
        let d_var1 = GetTickCount();
        DAT_00759b18 = DAT_00759b20.wrapping_add(d_var1).wrapping_add(2000);
        DAT_00759b2c = 0;
    }
    1
}
