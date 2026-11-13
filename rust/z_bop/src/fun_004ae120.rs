// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00663658: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004add60(param_1: i32);
    fn __ftol() -> i64;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004ae120(param_1: i32) -> i64 {
    // SAFETY: the global table and callees are the game's own; the index is
    // used as the original does, and __ftol reads the x87 top-of-stack value
    // left by the preceding code, with no floating-point work in between.
    unsafe {
        let p = (&raw mut DAT_00663658)
            .cast::<u8>()
            .byte_add(param_1.wrapping_mul(4) as isize as usize)
            .cast::<i32>();
        if p.read_unaligned() == 0 {
            FUN_004add60(param_1);
        }
        __ftol()
    }
}
