// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a004b0: i32;
    static mut DAT_009fe7f4: i32;
    static mut DAT_00a00490: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00427500(param_1: i32, param_2: i32, param_3: i32) -> i32 {
    // SAFETY: the globals are the game's own memory, read by value.
    unsafe {
        let v1 = (DAT_00a004b0 as i64).wrapping_mul(param_1 as i64);
        let uv1 = v1 as u64;
        let t1 = ((uv1 as u32) >> 15) | (((uv1 >> 32) as u32).wrapping_shl(17));

        let v2 = (DAT_009fe7f4 as i64).wrapping_mul(param_2 as i64);
        let uv2 = v2 as u64;
        let t2 = ((uv2 as u32) >> 15) | (((uv2 >> 32) as u32).wrapping_shl(17));

        let v3 = (DAT_00a00490 as i64).wrapping_mul(param_3 as i64);
        let uv3 = v3 as u64;
        let t3 = ((uv3 as u32) >> 15) | (((uv3 >> 32) as u32).wrapping_shl(17));

        t1.wrapping_add(t2).wrapping_add(t3) as i32
    }
}
