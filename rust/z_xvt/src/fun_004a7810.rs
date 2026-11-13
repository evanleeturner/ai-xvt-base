// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052b050: i32;
    static mut DAT_0052b05c: i32;
    static mut DAT_0052b060: i32;
    static mut DAT_0052b064: i32;
    static mut DAT_0052b068: i32;
    static mut DAT_00645bd8: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_00490490();
    fn FUN_004904e0() -> *mut i32;
    fn FUN_00490510();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004a7810() {
    // SAFETY: the globals and callees are the game's own; they are read and
    // written in the order the original does.
    unsafe {
        if DAT_0052b050 != 0 {
            if DAT_0052b060 == 0 {
                if (DAT_00645bd8 >> 1) < DAT_0052b05c {
                    FUN_00490490();
                    DAT_0052b060 = 1;
                    return;
                }
            } else if DAT_0052b060 == 2 {
                let pi_var1 = FUN_004904e0();
                if DAT_0052b064 < (pi_var1 as i32) && (pi_var1 as i32) < DAT_0052b068 {
                    FUN_00490510();
                    DAT_0052b060 = 0;
                    DAT_0052b05c = 0;
                }
            }
        }
    }
}
