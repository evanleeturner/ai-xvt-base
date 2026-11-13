// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cc820(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004dc470() -> u8 {
    // SAFETY: the callee is the game's own function; it is called with the
    // same arguments in the same order as the original, stopping at the first
    // non-zero result.
    unsafe {
        if FUN_004cc820(0x45) != 0 {
            return 1;
        }
        if FUN_004cc820(0x46) != 0 {
            return 1;
        }
        if FUN_004cc820(0x47) != 0 {
            return 1;
        }
        if FUN_004cc820(0x48) != 0 {
            return 1;
        }
        if FUN_004cc820(0x49) != 0 {
            return 1;
        }
        if FUN_004cc820(0x4a) != 0 {
            return 1;
        }
        if FUN_004cc820(0x4b) != 0 {
            return 1;
        }
        if FUN_004cc820(0x4c) != 0 {
            return 1;
        }
        if FUN_004cc820(0x76) != 0 {
            return 1;
        }
        if FUN_004cc820(0x53) != 0 {
            return 1;
        }
        if FUN_004cc820(0x5a) != 0 {
            return 1;
        }
        if FUN_004cc820(0x5b) != 0 {
            return 1;
        }
        if FUN_004cc820(0x5c) != 0 {
            return 1;
        }
        if FUN_004cc820(0x61) != 0 {
            return 1;
        }
        (FUN_004cc820(0x66) != 0) as u8
    }
}
