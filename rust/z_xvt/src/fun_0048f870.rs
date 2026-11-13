// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052a570: u32;
    static mut DAT_0052a574: u32;
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
    ) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0048f870() -> u32 {
    let mut local_10: u32 = 0;
    let mut local_c: u32 = 0;
    let mut local_8: u32 = 0;
    let mut local_4: u32 = 0;
    // SAFETY: the globals and the callee are the game's own; the locals
    // passed by address live for the whole call.
    unsafe {
        if DAT_0052a570 == 0 {
            DAT_0052a574 = 1;
            FUN_004a57a0(0, &mut local_10, &mut local_c, &mut local_4, &mut local_8);
            if local_10 == 32000 && local_c == 32000 {
                FUN_004a57a0(1, &mut local_10, &mut local_c, &mut local_4, &mut local_8);
                if local_10 == 32000 && local_c == 32000 {
                    DAT_0052a578 = 0;
                } else {
                    DAT_0052a57c = 1;
                    DAT_0052a578 = 1;
                }
            } else {
                DAT_0052a57c = 0;
                DAT_0052a578 = 1;
            }
            DAT_0052a570 = 1;
        }
        DAT_0052a574
    }
}
