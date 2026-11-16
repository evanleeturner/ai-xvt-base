// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00415060(param_1: *mut f32, param_2: i32) -> f32 {
    // SAFETY: param_1 points at three floats and param_2 is the address of a
    // matrix-like float block; reads happen in the original's order.
    unsafe {
        let m4 = ((param_2.wrapping_add(4)) as *const f32).read_unaligned();
        let a0 = param_1.read_unaligned();
        let a1 = param_1.add(1).read_unaligned();
        let m10 = ((param_2.wrapping_add(0x10)) as *const f32).read_unaligned();
        let a2 = param_1.add(2).read_unaligned();
        let m1c = ((param_2.wrapping_add(0x1c)) as *const f32).read_unaligned();
        ((m4 * a0) + (a1 * m10)) + (a2 * m1c)
    }
}
