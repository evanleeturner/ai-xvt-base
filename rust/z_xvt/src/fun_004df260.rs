// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00758ee8: [u8; 1];
    static mut DAT_00758f2e: [u8; 1];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004df260(param_1: i32) -> u8 {
    if param_1 >= 2 {
        return 0;
    }
    // SAFETY: both arrays are the game's own memory; the index is used
    // as the original does, without a lower bound check.
    unsafe {
        let a = (&raw mut DAT_00758ee8).cast::<u8>();
        if a.offset(param_1 as isize).read() != 0 {
            return 0;
        }
        let b = (&raw mut DAT_00758f2e).cast::<u8>();
        b.offset(param_1.wrapping_mul(0x20) as isize).read()
    }
}
