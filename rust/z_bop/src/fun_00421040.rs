// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00421040(param_1: *mut f32, param_2: *mut f32) -> f32 {
    // SAFETY: both pointers refer to the caller's float arrays, read at the
    // same offsets as the original.
    unsafe {
        let a = param_1.read_unaligned() * param_2.read_unaligned();
        let b = param_1.add(1).read_unaligned() * param_2.add(3).read_unaligned();
        let c = param_2.add(6).read_unaligned() * param_1.add(2).read_unaligned();
        (a + b) + c
    }
}
