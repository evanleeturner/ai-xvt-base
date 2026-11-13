// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00759645: i8;
    static mut DAT_00758830: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cce90(param_1: *mut i32, param_2: *mut u32) -> *mut i32;
    fn FUN_004cca00(param_1: i32, param_2: i32);
    fn FUN_004d4d00() -> i32;
    fn FUN_004d4d90();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004cc9a0(param_1: *mut i32, param_2: *mut u32) -> *mut i32 {
    // SAFETY: the callees and globals are the game's own; param_1 and param_2
    // are the caller's pointers, passed through and read as the original does.
    unsafe {
        let c_var1 = DAT_00759645;
        FUN_004d4d90();
        let mut pi_var2: *mut i32;
        loop {
            pi_var2 = FUN_004cce90(param_1, param_2);
            if pi_var2.is_null() || param_1.read() == 0 {
                break;
            }
            FUN_004cca00(pi_var2.read(), pi_var2 as usize as i32);
        }
        if c_var1 != 0 {
            DAT_00758830 = FUN_004d4d00();
        }
        pi_var2
    }
}
