// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004dd070(param_1: *mut i32, param_2: i32, param_3: i32) {
    // SAFETY: param_1 points at the caller's array of four i32 values,
    // read and written where the original does.
    unsafe {
        let p0 = param_1.read_unaligned();
        param_1.write_unaligned(p0.wrapping_add(param_2));
        let p2 = param_1.byte_add(8).read_unaligned();
        param_1
            .byte_add(8)
            .write_unaligned(p2.wrapping_sub(param_2));
        let p1 = param_1.byte_add(4).read_unaligned();
        param_1
            .byte_add(4)
            .write_unaligned(p1.wrapping_add(param_3));
        let p3 = param_1.byte_add(12).read_unaligned();
        param_1
            .byte_add(12)
            .write_unaligned(p3.wrapping_sub(param_3));
    }
}
