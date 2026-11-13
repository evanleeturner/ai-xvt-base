// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0073ba30: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00491960(param_1: *mut i8) -> u32;
    fn FUN_004918c0(param_1: *mut i8) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00439830(param_1: i32, param_2: i32) -> u32 {
    if param_1 < 4 || 0x342 < param_1 {
        return 0;
    }
    if param_2 == 0x100 {
        // SAFETY: the global and the callee are the game's own; the pointer
        // is an address inside the game's table.
        return unsafe {
            FUN_00491960((&raw mut DAT_0073ba30).byte_offset(param_1.wrapping_mul(0x18) as isize))
        };
    }
    if param_2 != 0x500 {
        return 0;
    }
    // SAFETY: the global and the callee are the game's own; the pointer
    // is an address inside the game's table.
    unsafe {
        FUN_004918c0((&raw mut DAT_0073ba30).byte_offset(param_1.wrapping_mul(0x18) as isize))
    }
}
