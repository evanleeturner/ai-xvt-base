// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00414fc0(param_1: *mut f32, param_2: *mut f32) -> f32 {
    // SAFETY: the caller passes pointers to at least three floats each.
    unsafe {
        let a = param_2.read_unaligned() * param_1.read_unaligned();
        let b = param_2.add(2).read_unaligned() * param_1.add(2).read_unaligned();
        let c = param_2.add(1).read_unaligned() * param_1.add(1).read_unaligned();
        (a + b) + c
    }
}
