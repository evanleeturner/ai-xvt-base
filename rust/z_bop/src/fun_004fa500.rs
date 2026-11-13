// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa5e68: i32;
    static mut DAT_00b6a5a6: i32;
    static mut DAT_00aa5e10: [i32; 0];
    static mut DAT_00aa5cb0: [i32; 0];
    static mut DAT_00aa5ac0: [i32; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004fa500() -> i32 {
    // SAFETY: the globals are the game's own memory; arrays are indexed
    // exactly as the original does.
    unsafe {
        let mut i_var3: i32 = 0;
        if 0 < DAT_00aa5e68 {
            loop {
                let mut i_var2: i32 = 0;
                let p10 = (&raw mut DAT_00aa5e10).cast::<i32>();
                let p_cb0 = (&raw mut DAT_00aa5cb0).cast::<i32>();
                let p_ac0 = (&raw mut DAT_00aa5ac0).cast::<i32>();
                if 0 < p10.offset(DAT_00b6a5a6 as isize).read() {
                    loop {
                        let i_var1 = DAT_00b6a5a6.wrapping_mul(8).wrapping_add(i_var2);
                        if p_cb0.offset(i_var1 as isize).read() != 0
                            && p_ac0.offset(i_var1 as isize).read() == -1
                        {
                            return 0;
                        }
                        i_var2 = i_var2.wrapping_add(1);
                        if !(i_var2 < p10.offset(DAT_00b6a5a6 as isize).read()) {
                            break;
                        }
                    }
                }
                i_var3 = i_var3.wrapping_add(1);
                if !(i_var3 < DAT_00aa5e68) {
                    break;
                }
            }
        }
        1
    }
}
