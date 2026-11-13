// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dd000(param_1: *mut i32, param_2: *mut u32);
    fn FUN_004dd070(param_1: *mut i32, param_2: i32, param_3: i32);
    fn FUN_004f36b0() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004f2320(param_1: *mut u32, param_2: *mut u32, param_3: i16) -> i32 {
    let mut local_10: [i32; 4] = [0; 4];
    let mut local_20: [i32; 4] = [0; 4];
    // SAFETY: the callees are the game's own and receive pointers to local
    // buffers and the caller's pointers; the fixed address 0x5b is read as the
    // original does.
    unsafe {
        FUN_004dd000(local_10.as_mut_ptr(), param_1);
        FUN_004dd070(local_10.as_mut_ptr(), 1, 1);
        FUN_004dd000(local_20.as_mut_ptr(), param_2);
        FUN_004dd070(local_20.as_mut_ptr(), 1, 1);
        if param_3 != 0 {
            let v = (0x5b as *const u16).read_unaligned();
            return ((v as u32) << 0x10) as i32;
        }
        FUN_004f36b0()
    }
}
