// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005ef2cc: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn timeGetTime() -> u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d0070(param_1: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004395b0(param_1: u32, param_2: u32, param_3: i32) -> u32 {
    let mut param_1 = param_1;
    // SAFETY: the global and the callees are the game's own.
    unsafe {
        if DAT_005ef2cc == 0 {
            return 0;
        }
        let b_var5 = param_1 <= param_2;
        if param_2 != param_1 {
            let u_var4: u32 = if b_var5 {
                param_2.wrapping_sub(param_1)
            } else {
                param_1.wrapping_sub(param_2)
            };
            let mut d_var2 = timeGetTime();
            loop {
                loop {
                    let d_var3 = timeGetTime();
                    let delay = ((param_3.wrapping_shl(8)) as u32) / u_var4;
                    if (delay.wrapping_add(d_var2) as i32) < (d_var3 as i32) {
                        if b_var5 {
                            let u_var1 = param_1.wrapping_add(0x100);
                            param_1 = 0xffff;
                            if u_var1 < 0x10000 {
                                param_1 = u_var1;
                            }
                        } else if param_1 < 0x100 {
                            param_1 = 0;
                        } else {
                            param_1 = param_1.wrapping_sub(0x100);
                        }
                        FUN_004d0070(param_1);
                        d_var2 = d_var3;
                    }
                    if !b_var5 {
                        break;
                    }
                    if param_2 <= param_1 {
                        return 1;
                    }
                }
                if !(param_2 < param_1) {
                    return 1;
                }
            }
        }
        1
    }
}
