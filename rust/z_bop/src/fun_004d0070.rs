// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7b0d: i8;
    static mut DAT_00aa6cfc: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d4970();
    fn FUN_004d0540(param_1: *mut i32, param_2: *mut u32) -> *mut i32;
    fn FUN_004d00d0(param_1: i32, param_2: i32);
    fn FUN_004d48e0() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d0070(param_1: *mut i32, param_2: *mut u32) -> *mut i32 {
    // SAFETY: the callees and globals are the game's own; param_1 and
    // param_2 are the caller's pointers, and piVar2 is checked non-null
    // before it is read.
    unsafe {
        let c_var1 = DAT_00aa7b0d;
        FUN_004d4970();
        let mut pi_var2;
        loop {
            pi_var2 = FUN_004d0540(param_1, param_2);
            if pi_var2.is_null() || param_1.read_unaligned() == 0 {
                break;
            }
            FUN_004d00d0(pi_var2.read_unaligned(), pi_var2 as usize as i32);
        }
        if c_var1 != 0 {
            DAT_00aa6cfc = FUN_004d48e0();
        }
        pi_var2
    }
}
