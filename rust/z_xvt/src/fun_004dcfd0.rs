// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004dcfd0(
    param_1: *mut u32,
    param_2: u32,
    param_3: u32,
    param_4: u32,
    param_5: u32,
) {
    // SAFETY: param_1 points at the caller's four-u32 buffer, written
    // in the original's order.
    unsafe {
        param_1.byte_add(4).write_unaligned(param_3);
        param_1.write_unaligned(param_2);
        param_1.byte_add(12).write_unaligned(param_5);
        param_1.byte_add(8).write_unaligned(param_4);
    }
}
