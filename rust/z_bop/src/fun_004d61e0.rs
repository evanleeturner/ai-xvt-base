// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa73b0: [u8; 0];
    static mut DAT_00aa73f6: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d61e0(param_1: i32) -> u8 {
    if param_1 >= 2 {
        return 0;
    }
    // SAFETY: the globals are the game's own byte tables; they are read at
    // the same offsets the original uses.
    unsafe {
        let a = (&raw mut DAT_00aa73b0).cast::<u8>();
        if a.offset(param_1 as isize).read() != 0 {
            return 0;
        }
        let b = (&raw mut DAT_00aa73f6).cast::<u8>();
        b.offset(param_1.wrapping_mul(8) as isize).read()
    }
}
