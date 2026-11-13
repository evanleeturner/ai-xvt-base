// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7fd3: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetTickCount() -> u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d34a0(param_1: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d3520(param_1: u32, param_2: u32, param_3: i32) -> u32 {
    let mut param_1 = param_1;
    // SAFETY: the global is the game's own memory, read unaligned by address.
    let flag = unsafe { (&raw const DAT_00aa7fd3).read_unaligned() };
    if flag == 0 {
        return 0;
    }
    let b_var5 = param_1 <= param_2;
    if param_2 != param_1 {
        let u_var4 = if b_var5 {
            param_2.wrapping_sub(param_1)
        } else {
            param_1.wrapping_sub(param_2)
        };
        // SAFETY: GetTickCount and FUN_004d34a0 are the game's/system's own
        // functions; u_var4 is nonzero since the parameters differ.
        unsafe {
            let mut d_var2 = GetTickCount();
            loop {
                loop {
                    let d_var3 = GetTickCount();
                    let t = ((param_3 as u32).wrapping_shl(8) / u_var4).wrapping_add(d_var2);
                    if (t as i32) < (d_var3 as i32) {
                        if b_var5 {
                            let u_var1 = param_1.wrapping_add(0x100);
                            param_1 = 0xFFFF;
                            if u_var1 < 0x10000 {
                                param_1 = u_var1;
                            }
                        } else if param_1 < 0x100 {
                            param_1 = 0;
                        } else {
                            param_1 = param_1.wrapping_sub(0x100);
                        }
                        FUN_004d34a0(param_1);
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
                    break;
                }
            }
        }
        return 1;
    }
    1
}
