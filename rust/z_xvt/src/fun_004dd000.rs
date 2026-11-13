// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004dd000(param_1: *mut u32, param_2: *mut u32) {
    // SAFETY: both pointers refer to the caller's 4-dword buffers; reads and
    // writes happen in the original's order, unaligned-tolerant.
    unsafe {
        param_1.write_unaligned(param_2.read_unaligned());
        param_1
            .byte_add(4)
            .write_unaligned(param_2.byte_add(4).read_unaligned());
        param_1
            .byte_add(8)
            .write_unaligned(param_2.byte_add(8).read_unaligned());
        param_1
            .byte_add(12)
            .write_unaligned(param_2.byte_add(12).read_unaligned());
    }
}
