// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00528c84: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004b3160(param_1: i32);
    fn FUN_004b30f0(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b3280(param_1: i32) {
    // SAFETY: param_1 is an address of the game's own object; the global and
    // callees belong to the game, accessed in the original's order.
    unsafe {
        ((param_1.wrapping_add(0x8c)) as *mut i32).write_unaligned(DAT_00528c84);
        FUN_004b3160(param_1);
        FUN_004b30f0(param_1);
    }
}
