// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a2000: u16;
    static mut DAT_009d8120: u16;
    static mut DAT_009e8f60: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ac6e0(param_1: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0045c2d0() {
    // SAFETY: the globals and the callee are the game's own; the pointers
    // walk the game's contiguous u16 tables exactly as the original does.
    unsafe {
        let mut i_var4: i32 = 0;
        if 0 < DAT_009a2000 as i32 {
            let mut pu_var1: *mut u16 = &raw mut DAT_009d8120;
            loop {
                let mut i_var3: i32 = 8;
                loop {
                    let mut i_var2: i32 = 3;
                    loop {
                        if pu_var1.read() != 0 {
                            FUN_004ac6e0(pu_var1.read() as u32);
                        }
                        pu_var1 = pu_var1.add(1);
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
                i_var4 = i_var4.wrapping_add(1);
                if !(i_var4 < DAT_009a2000 as i32) {
                    break;
                }
            }
        }
        let mut local_4: i32 = 10;
        let mut pu_var1: *mut u16 = &raw mut DAT_009e8f60;
        loop {
            let mut i_var4: i32 = 7;
            loop {
                let mut i_var3: i32 = 4;
                loop {
                    let mut i_var2: i32 = 3;
                    loop {
                        if pu_var1.read() != 0 {
                            FUN_004ac6e0(pu_var1.read() as u32);
                        }
                        pu_var1 = pu_var1.add(1);
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
                i_var4 = i_var4.wrapping_sub(1);
                if i_var4 == 0 {
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
