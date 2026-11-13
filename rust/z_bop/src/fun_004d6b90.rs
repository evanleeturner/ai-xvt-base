// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d6b90(param_1: *mut i32, param_2: i32, param_3: i32) -> i32 {
    // SAFETY: param_1 points at the caller's rectangle of four i32 values.
    unsafe {
        if param_2 < param_1.read_unaligned() || param_1.add(2).read_unaligned() < param_2 {
            return 0;
        }
        if param_1.add(1).read_unaligned() <= param_3 && param_3 <= param_1.add(3).read_unaligned()
        {
            return 1;
        }
        0
    }
}
