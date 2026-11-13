// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d6480(param_1: *mut u32, param_2: *mut u32);
    fn FUN_004f7e40(param_1: *mut u32, param_2: *mut u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004f6970(param_1: *mut u32, param_2: *mut u32) {
    let mut local_20: [u32; 4] = [0; 4];
    let mut local_10: [u32; 4] = [0; 4];
    // SAFETY: the callees are the game's own; the locals are valid 16-byte
    // buffers and the parameters are passed through unchanged.
    unsafe {
        FUN_004d6480(local_10.as_mut_ptr(), param_1);
        FUN_004d6480(local_20.as_mut_ptr(), param_2);
        FUN_004f7e40(local_20.as_mut_ptr(), local_10.as_mut_ptr());
    }
}
