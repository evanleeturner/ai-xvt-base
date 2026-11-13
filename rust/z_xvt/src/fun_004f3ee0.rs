// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006509a4: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dcfd0(param_1: *mut u32, param_2: u32, param_3: u32, param_4: u32, param_5: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004f3ee0() {
    let mut local_10: [u32; 4] = [0; 4];
    // SAFETY: the global and the callee are the game's own; local_10 is a
    // valid buffer for the callee to fill.
    unsafe {
        if DAT_006509a4 != 0 {
            FUN_004dcfd0(local_10.as_mut_ptr(), 0, 0, 0x140, 0x96);
        }
    }
}
