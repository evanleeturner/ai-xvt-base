// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d6480(param_1: *mut u32, param_2: *mut u32) {
    // SAFETY: both pointers point at the caller's 16-byte buffers; each
    // dword is read then written in the original's order.
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
