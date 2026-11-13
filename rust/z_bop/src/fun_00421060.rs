// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00421060(param_1: *mut f32, param_2: i32) -> f64 {
    // SAFETY: param_1 points at three floats and param_2 is the address of a
    // structure holding floats at offsets 4, 0x10 and 0x1c, as in the original.
    unsafe {
        let b = (param_2.wrapping_add(0x1c) as *const f32).read_unaligned();
        let a = (param_2.wrapping_add(0x10) as *const f32).read_unaligned();
        let c = (param_2.wrapping_add(4) as *const f32).read_unaligned();
        let p2 = param_1.add(2).read_unaligned();
        let p1 = param_1.add(1).read_unaligned();
        let p0 = param_1.read_unaligned();
        let t1 = b * p2;
        let t2 = a * p1;
        let s = t1 + t2;
        let t3 = c * p0;
        (s + t3) as f64
    }
}
