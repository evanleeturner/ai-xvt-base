// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052552c: i32;
    static mut DAT_006d4998: u8;
    static mut DAT_006d4a4e: u8;
    static mut DAT_006da1a0: i32;
    static mut DAT_00709ff4: i32;
    static mut DAT_0070a188: i8;
    static mut DAT_0070acc2: i16;
    static mut DAT_0071b720: i32;
    static mut DAT_0071f2e2: u8;
    static mut DAT_00732a10: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0048ee20(param_1: i8) {
    // SAFETY: all globals are the game's own memory, accessed by value
    // in the same order as the original.
    unsafe {
        DAT_0070a188 = param_1;
        if param_1 == 0 {
            if DAT_0052552c != 0x13 {
                if DAT_0052552c == 0x101 {
                    DAT_006d4a4e = 8;
                    DAT_006d4998 = 1;
                    DAT_0071f2e2 = 4;
                    DAT_0070acc2 = 0x42;
                    DAT_00709ff4 = DAT_006da1a0;
                    return;
                }
                if DAT_0052552c != 0x112 {
                    return;
                }
            }
            DAT_006d4a4e = 5;
            DAT_006d4998 = 0;
            DAT_0071f2e2 = 3;
            DAT_0070acc2 = 0x2a;
            DAT_00709ff4 = DAT_00732a10;
            return;
        }
        if param_1 == 1 {
            if DAT_0052552c == 0x13 {
                DAT_006d4a4e = 5;
                DAT_006d4998 = 0;
                DAT_0071f2e2 = 3;
                DAT_0070acc2 = 0x2a;
                DAT_00709ff4 = DAT_00732a10;
                return;
            }
            if DAT_0052552c == 0x101 {
                DAT_006d4a4e = 10;
                DAT_006d4998 = 1;
                DAT_0071f2e2 = 5;
                DAT_0070acc2 = 0x52;
                DAT_00709ff4 = DAT_0071b720;
                return;
            }
            if DAT_0052552c == 0x112 {
                DAT_006d4a4e = 8;
                DAT_0071f2e2 = 4;
                DAT_0070acc2 = 0x42;
                DAT_00709ff4 = DAT_006da1a0;
                return;
            }
            return;
        }
        if param_1 != 2 {
            return;
        }
        if DAT_0052552c != 0x13 {
            if DAT_0052552c == 0x101 {
                DAT_006d4a4e = 10;
                DAT_006d4998 = 1;
                DAT_0071f2e2 = 5;
                DAT_0070acc2 = 0x52;
                DAT_00709ff4 = DAT_0071b720;
                return;
            }
            if DAT_0052552c == 0x112 {
                DAT_0071f2e2 = 4;
                DAT_0070acc2 = 0x42;
                DAT_00709ff4 = DAT_006da1a0;
                DAT_006d4a4e = 8;
                return;
            }
            return;
        }
        DAT_006d4a4e = 5;
        DAT_006d4998 = 0;
        DAT_0071f2e2 = 3;
        DAT_0070acc2 = 0x2a;
        DAT_00709ff4 = DAT_00732a10;
    }
}
