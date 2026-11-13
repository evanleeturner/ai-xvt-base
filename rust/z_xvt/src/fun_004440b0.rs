// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0072f508: [i8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004440b0(param_1: i32, param_2: i32) -> u8 {
    if param_1 == param_2 {
        return 0;
    }
    let idx = param_1.wrapping_add(param_2.wrapping_mul(0x1e5));
    // SAFETY: the global is the game's own table; the index is read where the original reads it.
    unsafe {
        let p = (&raw mut DAT_0072f508).cast::<i8>();
        (p.offset(idx as isize).read() == 0) as u8
    }
}
