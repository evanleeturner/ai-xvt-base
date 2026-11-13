// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d6460(
    param_1: *mut u32,
    param_2: u32,
    param_3: u32,
    param_4: u32,
    param_5: u32,
) {
    // SAFETY: param_1 points at the caller's buffer of four u32 values,
    // written where the original does.
    unsafe {
        param_1.write_unaligned(param_2);
        param_1.byte_add(4).write_unaligned(param_3);
        param_1.byte_add(8).write_unaligned(param_4);
        param_1.byte_add(12).write_unaligned(param_5);
    }
}
