// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00758ee8: [u8; 0];
    static mut DAT_00758f2e: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004df290(param_1: i32) -> u8 {
    if param_1 > 1 {
        return 0;
    }
    // SAFETY: the globals are the game's own arrays; the indices are read
    // exactly as the original computes them.
    unsafe {
        let a = (&raw mut DAT_00758ee8).cast::<u8>();
        if a.offset(param_1 as isize).read() == 0 {
            let b = (&raw mut DAT_00758f2e).cast::<u8>();
            return b
                .offset(param_1.wrapping_mul(0x20).wrapping_add(1) as isize)
                .read();
        }
    }
    0
}
