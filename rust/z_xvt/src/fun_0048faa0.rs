// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052a578: u32;
    static mut DAT_0052a57c: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a57a0(
        param_1: u32,
        param_2: *mut u32,
        param_3: *mut u32,
        param_4: *mut u32,
        param_5: *mut u32,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0048faa0(
    param_1: *mut u32,
    param_2: *mut u32,
    param_3: *mut u32,
) -> u32 {
    let mut local_4: u32 = 0;
    // SAFETY: the globals and the callee are the game's own; the pointer
    // parameters are the caller's and are written where the original does.
    unsafe {
        if DAT_0052a578 == 0 {
            FUN_004a57a0(DAT_0052a57c, param_1, param_2, param_3, &raw mut local_4);
            return local_4;
        }
        param_1.write_unaligned(0);
        param_2.write_unaligned(0);
        param_3.write_unaligned(0);
        0
    }
}
