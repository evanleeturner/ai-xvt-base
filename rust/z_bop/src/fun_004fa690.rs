// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b6a5a6: i32;
    static mut DAT_00aa5ac0: [i32; 1];
    static mut DAT_00b69e30: i32;
    static mut DAT_00aa6af4: i32;
    static mut DAT_00aa6af0: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cef70(param1: i32, param2: *mut i32, param3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004fa690() -> i32 {
    // SAFETY: the globals and the callee are the game's own memory; the
    // array is indexed as the original does.
    unsafe {
        let i_var2 = DAT_00b6a5a6.wrapping_mul(8);
        let mut i_var3: i32 = 0;
        loop {
            let i_var1 = i_var2.wrapping_add(i_var3);
            i_var3 = i_var3.wrapping_add(1);
            (&raw mut DAT_00aa5ac0)
                .cast::<i32>()
                .offset(i_var1 as isize)
                .write_unaligned(-1);
            if i_var3 >= 8 {
                break;
            }
        }
        if DAT_00b69e30 != 2 {
            DAT_00aa6af4 = DAT_00b6a5a6;
            DAT_00aa6af0 = 0x56;
            FUN_004cef70(0, &raw mut DAT_00aa6af0, 8);
        }
    }
    1
}
