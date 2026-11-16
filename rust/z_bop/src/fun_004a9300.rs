// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a0adf0: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0042dbd0(param_1: *mut i8) -> u32;
    fn FUN_0042dba0(param_1: *mut i8) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a9300(param_1: i32, param_2: i32) -> u32 {
    if param_1 < 4 || param_1 > 0x345 {
        return 0;
    }
    // SAFETY: the global and the callees are the game's own; the pointer is
    // formed from the game's table base plus the record offset, as the original does.
    unsafe {
        if param_2 == 0x100 {
            return FUN_0042dbd0(DAT_00a0adf0.wrapping_add(param_1.wrapping_mul(24)) as *mut i8);
        }
        if param_2 != 0x500 {
            return 0;
        }
        FUN_0042dba0(DAT_00a0adf0.wrapping_add(param_1.wrapping_mul(24)) as *mut i8)
    }
}
