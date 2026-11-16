// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7b0d: i8;
    static mut DAT_00acebc5: u32;
    static mut DAT_00aa6cfc: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004d4970();
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cefe0(param_1: i32, param_2: *mut u32, param_3: u32) -> i32;
    fn FUN_004d48e0() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004cef70(param_1: i32, param_2: *mut u32, param_3: u32) -> i32 {
    // SAFETY: the globals and callees are the game's own; param_2 is the
    // caller's pointer, passed through unchanged.
    unsafe {
        let c_var1 = DAT_00aa7b0d;
        if DAT_00acebc5 == 0 {
            return 1;
        }
        FUN_004d4970();
        let b_var2 = FUN_004cefe0(param_1, param_2, param_3);
        let mut local_4: u32 = 0x39;
        FUN_004cefe0(param_1, &raw mut local_4, 4);
        if c_var1 != 0 {
            DAT_00aa6cfc = FUN_004d48e0();
        }
        b_var2
    }
}
