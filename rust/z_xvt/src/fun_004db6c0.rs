// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0077dc52: u32;
    static mut DAT_0077dc56: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dd000(param_1: *mut u32, param_2: *mut u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004db6c0(param_1: u32, param_2: *mut u32) -> u32 {
    // SAFETY: the globals and the callee are the game's own; the unaligned
    // write matches the original's store at an odd address.
    unsafe {
        (&raw mut DAT_0077dc52).write_unaligned(param_1);
        FUN_004dd000((&raw mut DAT_0077dc56).cast::<u32>(), param_2);
    }
    1
}
