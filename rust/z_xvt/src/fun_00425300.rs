// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d49a0: i32;
    static mut DAT_0070a1e0: u16;
    static mut DAT_0071b020: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048f4b0(param_1: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00425300() {
    // SAFETY: the globals and the callee are the game's own; the pointer
    // walks the game's tables exactly as the original does.
    unsafe {
        let mut i_var1: i32 = 0;
        let mut pu_var4: *const u16;
        if 0 < DAT_006d49a0 {
            pu_var4 = &raw mut DAT_0070a1e0 as *const u16;
            loop {
                let mut i_var3: i32 = 8;
                loop {
                    let mut i_var2: i32 = 3;
                    loop {
                        let v = pu_var4.read_unaligned();
                        if v != 0 {
                            FUN_0048f4b0(v as u32);
                        }
                        pu_var4 = pu_var4.add(1);
                        i_var2 = i_var2.wrapping_sub(1);
                        if i_var2 == 0 {
                            break;
                        }
                    }
                    i_var3 = i_var3.wrapping_sub(1);
                    if i_var3 == 0 {
                        break;
                    }
                }
                i_var1 = i_var1.wrapping_add(1);
                if !(i_var1 < DAT_006d49a0) {
                    break;
                }
            }
        }
        let mut local_4: i32 = 10;
        pu_var4 = &raw mut DAT_0071b020 as *const u16;
        loop {
            let mut i_var1: i32 = 7;
            loop {
                let mut i_var3: i32 = 4;
                loop {
                    let mut i_var2: i32 = 3;
                    loop {
                        let v = pu_var4.read_unaligned();
                        if v != 0 {
                            FUN_0048f4b0(v as u32);
                        }
                        pu_var4 = pu_var4.add(1);
                        i_var2 = i_var2.wrapping_sub(1);
                        if i_var2 == 0 {
                            break;
                        }
                    }
                    i_var3 = i_var3.wrapping_sub(1);
                    if i_var3 == 0 {
                        break;
                    }
                }
                i_var1 = i_var1.wrapping_sub(1);
                if i_var1 == 0 {
                    break;
                }
            }
            local_4 = local_4.wrapping_sub(1);
            if local_4 == 0 {
                break;
            }
        }
    }
}
