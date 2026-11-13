// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00415080(param_1: *const f32, param_2: i32) -> f32 {
    // SAFETY: param_1 points at the caller's floats and param_2 is the
    // address of a float block; both are read where the original reads them.
    unsafe {
        let p2 = param_2 as u32 as *const u8;
        let a: f32 = p2.byte_add(8).cast::<f32>().read_unaligned();
        let b: f32 = p2.byte_add(0x20).cast::<f32>().read_unaligned();
        let c: f32 = p2.byte_add(0x14).cast::<f32>().read_unaligned();
        let p0: f32 = param_1.read_unaligned();
        let p2v: f32 = param_1.add(2).read_unaligned();
        let p1: f32 = param_1.add(1).read_unaligned();
        a * p0 + p2v * b + p1 * c
    }
}
