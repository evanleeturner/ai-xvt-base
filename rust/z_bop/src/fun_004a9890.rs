// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1ed0: u8;
    static mut DAT_009a1ed1: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a9890(param_1: i32) -> u32 {
    let mut param_1 = param_1;
    // SAFETY: the globals are the game's own buffer; writes go where the original does.
    unsafe {
        let mut local_8: *mut u8 = &raw mut DAT_009a1ed0;
        let b_var5 = param_1 < 0;
        let mut b_var6 = false;
        if b_var5 {
            param_1 = param_1.wrapping_neg();
            DAT_009a1ed0 = 0x2d;
            local_8 = &raw mut DAT_009a1ed1;
            b_var6 = param_1 < 0;
        }
        let mut u_var4: u16;
        if param_1 == 0 {
            u_var4 = 1;
            *local_8 = 0x30;
        } else {
            u_var4 = 0;
            let mut i_var1 = param_1;
            if !b_var6 {
                loop {
                    u_var4 = u_var4.wrapping_add(1);
                    i_var1 /= 10;
                    if !(0 < i_var1) {
                        break;
                    }
                }
            }
            if u_var4 != 0 {
                let mut i_var1: i32 = 0;
                loop {
                    let i_var2 = param_1 % 10;
                    param_1 /= 10;
                    let i_var3 = i_var1.wrapping_add(1);
                    let idx = (u_var4 as u32).wrapping_add((-1i32).wrapping_sub(i_var1) as u32);
                    *local_8.byte_add(idx as usize) = (i_var2 as u8).wrapping_add(0x30);
                    i_var1 = i_var3;
                    if !(i_var3 < u_var4 as i32) {
                        break;
                    }
                }
            }
        }
        *local_8.byte_add(u_var4 as usize) = 0;
        if b_var5 {
            u_var4 = u_var4.wrapping_add(1);
        }
        local_8.byte_add(u_var4 as usize) as usize as u32
    }
}
