// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00525518: i32;
    static mut DAT_00525534: i32;
    static mut DAT_005f10f0: i32;
    static mut DAT_005f1120: [i32; 0];
    static mut DAT_005f1d20: *mut i32;
    static mut DAT_005f1d24: *mut i32;
    static mut DAT_006fede0: [i32; 0];
    static mut DAT_0070acc8: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048f570() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00461f20(param_1: i32, param_2: i32, param_3: u32, param_4: i32) {
    // SAFETY: all globals and the callee are the game's own memory; the
    // arrays are indexed exactly as the original does.
    unsafe {
        let i_var1 = DAT_00525518;
        if param_1 == 0 {
            DAT_00525534 = FUN_0048f570();
            DAT_005f1d24 = &raw mut DAT_0070acc8;
            DAT_005f1d20 = (&raw mut DAT_006fede0).cast::<i32>();
            return;
        }
        DAT_00525534 = param_1;
        if param_4 != -1 {
            let mut s_var3: i16 = 0;
            if param_3 != 0 {
                loop {
                    let i_var2 = s_var3 as i32;
                    s_var3 = s_var3.wrapping_add(1);
                    let v = (i_var1 as u32)
                        .wrapping_mul(i_var2 as u32)
                        .wrapping_mul(param_2 as u32) as i32;
                    (&raw mut DAT_005f1120)
                        .cast::<i32>()
                        .offset(i_var2 as isize)
                        .write(v);
                    if !((s_var3 as i32 as u32) < param_3) {
                        break;
                    }
                }
            }
            DAT_005f10f0 = param_4;
            DAT_005f1d24 = &raw mut DAT_005f10f0;
            DAT_005f1d20 = (&raw mut DAT_005f1120).cast::<i32>();
            return;
        }
        let mut s_var3: i16 = 0;
        if param_3 != 0 {
            loop {
                let i_var1b = s_var3 as i32;
                s_var3 = s_var3.wrapping_add(1);
                let v = (DAT_0070acc8 as u32).wrapping_mul(i_var1b as u32) as i32;
                (&raw mut DAT_006fede0)
                    .cast::<i32>()
                    .offset(i_var1b as isize)
                    .write(v);
                if !((s_var3 as i32 as u32) < param_3) {
                    break;
                }
            }
        }
        DAT_005f1d24 = &raw mut DAT_0070acc8;
        DAT_005f1d20 = (&raw mut DAT_006fede0).cast::<i32>();
    }
}
