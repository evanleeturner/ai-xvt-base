// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005250d4: i32;
    static mut DAT_0070aaf0: [i32; 0];
    static mut DAT_0071b740: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00432ca0() -> i32;
    fn FUN_00432c10(param_1: i32) -> i32;
    fn FUN_00432cc0(param_1: i32) -> *mut i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0042b340() {
    // SAFETY: the globals and callees are the game's own; the table indices
    // are used exactly as the original does.
    unsafe {
        if DAT_005250d4 == 0 {
            DAT_005250d4 = FUN_00432ca0();
        }
        let mut i_var1 = FUN_00432c10(DAT_005250d4);
        if (&raw mut DAT_0070aaf0)
            .cast::<i32>()
            .offset(i_var1 as isize)
            .read()
            != 0
        {
            DAT_005250d4 = FUN_00432ca0();
            i_var1 = FUN_00432ca0();
            i_var1 = FUN_00432c10(i_var1);
        }
        if (&raw mut DAT_0071b740)
            .cast::<i8>()
            .offset(i_var1.wrapping_mul(0x5bd) as isize)
            .read()
            == 0
        {
            DAT_005250d4 = FUN_00432ca0();
            i_var1 = FUN_00432ca0();
            i_var1 = FUN_00432c10(i_var1);
        }
        FUN_00432cc0(i_var1);
    }
}
