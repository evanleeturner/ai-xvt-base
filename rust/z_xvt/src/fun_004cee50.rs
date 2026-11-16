// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00759645: i8;
    static mut DAT_007806fa: u32;
    static mut DAT_00780732: u32;
    static mut DAT_00758830: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004d4d90();
    fn FUN_004ca450();
    fn FUN_004d4d00() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004cee50() -> u32 {
    // SAFETY: the globals and callees are the game's own; values are read
    // and written in the same order as the original.
    unsafe {
        let c_var1 = DAT_00759645;
        if DAT_007806fa == 0 {
            return 0;
        }
        FUN_004d4d90();
        DAT_00780732 = 1;
        FUN_004ca450();
        if c_var1 != 0 {
            DAT_00758830 = FUN_004d4d00();
        }
        1
    }
}
