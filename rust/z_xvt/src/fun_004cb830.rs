// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00759645: i8;
    static mut DAT_007806fa: u32;
    static mut DAT_00758830: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004d4d90();
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cb8b0(param_1: i32, param_2: *mut u32, param_3: u32) -> u8;
    fn FUN_004d4d00() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004cb830(param_1: i32, param_2: *mut u32, param_3: u32) -> u8 {
    // SAFETY: the globals and callees are the game's own; local_4 is a live
    // local whose address is passed for the callee to read 4 bytes.
    unsafe {
        let c_var1 = DAT_00759645;
        if DAT_007806fa != 0 {
            return 1;
        }
        FUN_004d4d90();
        let b_var2 = FUN_004cb8b0(param_1, param_2, param_3);
        let mut local_4: u32 = 0x39;
        FUN_004cb8b0(param_1, &raw mut local_4, 4);
        if c_var1 != 0 {
            DAT_00758830 = FUN_004d4d00();
        }
        b_var2
    }
}
