// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00421080(param_1: *mut f32, param_2: *mut core::ffi::c_void) -> f32 {
    // SAFETY: the caller passes pointers to valid float arrays; reads happen in the original's order.
    unsafe {
        let a: f32 = param_2.byte_add(8).cast::<f32>().read_unaligned() * param_1.read_unaligned();
        let b: f32 =
            param_1.add(2).read_unaligned() * param_2.byte_add(0x20).cast::<f32>().read_unaligned();
        let c: f32 =
            param_2.byte_add(0x14).cast::<f32>().read_unaligned() * param_1.add(1).read_unaligned();
        (a + b) + c
    }
}
