// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004dd030(param_1: *mut i32, param_2: i32, param_3: i32) {
    // SAFETY: param_1 points at the caller's array of four i32 values,
    // read and written where the original does.
    unsafe {
        param_1.write_unaligned(param_1.read_unaligned().wrapping_add(param_2));
        let p2 = param_1.byte_add(8);
        p2.write_unaligned(p2.read_unaligned().wrapping_add(param_2));
        let p1 = param_1.byte_add(4);
        p1.write_unaligned(p1.read_unaligned().wrapping_add(param_3));
        let p3 = param_1.byte_add(12);
        p3.write_unaligned(p3.read_unaligned().wrapping_add(param_3));
    }
}
