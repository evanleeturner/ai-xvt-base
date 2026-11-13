// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b6d962: i32;
    static mut DAT_0052b1a8: *mut i8;
    static mut DAT_0052b19c: *mut i8;
    static mut DAT_0052a6d4: *mut i8;
    static mut DAT_00b6a250: i32;
    static mut DAT_0052b69c: *mut i8;
    static mut DAT_0052b6ac: *mut i8;
    static mut DAT_0052b690: *mut i8;
    static mut DAT_00b6a2b0: i32;
    static mut DAT_0052b684: *mut i8;
    static mut DAT_0052b674: *mut i8;
    static mut DAT_0052b668: *mut i8;
    static mut DAT_0052b65c: *mut i8;
    static mut DAT_0052b650: *mut i8;
    static mut DAT_0052b644: *mut i8;
    static mut DAT_0052b638: *mut i8;
    static mut DAT_0052b62c: *mut i8;
    static mut DAT_0052b620: *mut i8;
    static mut DAT_0052b614: *mut i8;
    static mut DAT_0052b600: *mut i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d4f80() -> u32;
    fn FUN_004d4e20() -> u32;
    fn FUN_004b5de0(param_1: *mut i8, param_2: i32, param_3: i32) -> u32;
    fn FUN_004b5570(param_1: *mut i8, param_2: i32, param_3: i32) -> u32;
    fn FUN_004b4d40(param_1: *mut i8, param_2: i32, param_3: i32) -> u32;
    fn FUN_004d4ec0(param_1: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004caf60() -> u32 {
    // SAFETY: the callees and globals are the game's own memory; the
    // string pointers are passed through unchanged as the original does.
    unsafe {
        FUN_004d4f80();
        FUN_004d4e20();
        let p = if DAT_00b6d962 == 0 {
            DAT_0052b1a8
        } else {
            DAT_0052b19c
        };
        FUN_004b5de0(p, 0, 0);
        FUN_004b5570(DAT_0052a6d4, 0, 0);
        let p = if DAT_00b6a250 == 0 {
            DAT_0052b69c
        } else {
            DAT_0052b6ac
        };
        FUN_004b5570(p, 0, 0);
        FUN_004b4d40(DAT_0052b690, 0, 0);
        match DAT_00b6a2b0 {
            0 | 3 | 4 => {
                FUN_004b4d40(DAT_0052b684, 0, 0);
            }
            1 => {
                FUN_004b4d40(DAT_0052b674, 0, 0);
                if DAT_00b6d962 == 0 {
                    FUN_004b5570(DAT_0052b668, 0, 0);
                    FUN_004b5570(DAT_0052b65c, 0, 0);
                    FUN_004b5570(DAT_0052b650, 0, 0);
                    FUN_004b5570(DAT_0052b644, 0, 0);
                } else {
                    FUN_004b5570(DAT_0052b638, 0, 0);
                    FUN_004b5570(DAT_0052b62c, 0, 0);
                    FUN_004b5570(DAT_0052b620, 0, 0);
                    FUN_004b5570(DAT_0052b614, 0, 0);
                }
            }
            2 => {
                FUN_004b4d40(DAT_0052b600, 0, 0);
            }
            _ => {}
        }
        FUN_004d4ec0(1);
    }
    1
}
