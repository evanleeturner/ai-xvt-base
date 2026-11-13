// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a00480: i32;
    static mut DAT_00a004a4: i32;
    static mut DAT_00a004b8: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00427490(param_1: i32, param_2: i32, param_3: i32) -> i32 {
    // SAFETY: the globals are the game's own memory, read by value.
    unsafe {
        let v1: i64 = (DAT_00a004a4 as i64).wrapping_mul(param_1 as i64);
        let term1: i32 = (((v1 as u32) >> 15) as i32) | (((v1 >> 32) as i32).wrapping_shl(17));

        let v2: i64 = (DAT_00a004b8 as i64).wrapping_mul(param_2 as i64);
        let term2: i32 = (((v2 as u32) >> 15) as i32) | (((v2 >> 32) as i32).wrapping_shl(17));

        let v3: i64 = (DAT_00a00480 as i64).wrapping_mul(param_3 as i64);
        let term3: i32 = (((v3 as u32) >> 15) as i32) | (((v3 >> 32) as i32).wrapping_shl(17));

        term1.wrapping_add(term2).wrapping_add(term3)
    }
}
