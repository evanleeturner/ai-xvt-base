// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a00870: i32;
    static mut DAT_00a07ba4: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004adc40(param_1: i32) -> i32;
    fn FUN_004ae2a0(param_1: i32, param_2: i32) -> i32;
    fn FUN_004ae200(param_1: i32, param_2: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004afa90() {
    // SAFETY: the globals and callees are the game's own; the pointer walks
    // the game's table in strides of 0x65 ints, as the original does.
    unsafe {
        let mut local_4: *mut i32 = &raw mut DAT_00a00870;
        let end: *mut i32 = &raw mut DAT_00a07ba4;
        let mut i_var4: i32 = 0;
        loop {
            let mut i_var5: i32 = 0;
            let i_var2 = FUN_004adc40(i_var4);
            local_4.write(i_var2);
            let pi_var1 = local_4;
            if i_var2 > 0 {
                loop {
                    let i_var3 = FUN_004ae2a0(i_var4, i_var5);
                    pi_var1.wrapping_add(1).write(i_var3);
                    i_var5 = i_var5.wrapping_add(1);
                    i_var5 = FUN_004ae200(i_var4, i_var5);
                    pi_var1.wrapping_add(0x33).write(i_var5);
                    if !(i_var5 < i_var2) {
                        break;
                    }
                }
            }
            i_var4 = i_var4.wrapping_add(1);
            local_4 = local_4.wrapping_add(0x65);
            if !(local_4 < end) {
                break;
            }
        }
    }
}
