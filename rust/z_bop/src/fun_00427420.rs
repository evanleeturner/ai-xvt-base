// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a00488: i32;
    static mut DAT_00a004a8: i32;
    static mut DAT_00a004cc: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00427420(param_1: i32, param_2: i32, param_3: i32) -> i32 {
    // SAFETY: the globals are the game's own memory, read by value.
    unsafe {
        let prod1: i64 = (DAT_00a004a8 as i64).wrapping_mul(param_1 as i64);
        let term1: u32 = ((prod1 as u32) >> 15) | (((prod1 >> 32) as u32).wrapping_shl(17));

        let prod2: i64 = (DAT_00a004cc as i64).wrapping_mul(param_2 as i64);
        let term2: u32 = ((prod2 as u32) >> 15) | (((prod2 >> 32) as u32).wrapping_shl(17));

        let sum: u32 = term1.wrapping_add(term2);

        let prod3: i64 = (DAT_00a00488 as i64).wrapping_mul(param_3 as i64);
        let term3: u32 = ((prod3 as u32) >> 15) | (((prod3 >> 32) as u32).wrapping_shl(17));

        term3.wrapping_add(sum) as i32
    }
}
