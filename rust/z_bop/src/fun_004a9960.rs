// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009cd270: u16;
    static mut DAT_009d8c00: i16;
    static mut DAT_00a08108: i16;
    static mut DAT_00523d40: u8;
    static mut DAT_009a6fe4: unsafe extern "C" fn(u8);
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a9590(param_1: u32);
    fn FUN_004278c0(param_1: *mut u8) -> u64;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a9960(param_1: *mut u8) {
    let mut param_1 = param_1;
    let mut local_50 = [0u8; 80];
    // SAFETY: the string pointer, callees and globals belong to the game;
    // the local buffer is written through a raw pointer without bounds
    // checks, as the original does.
    unsafe {
        let mut b_var5: u8 = *param_1;
        while b_var5 != 0 {
            b_var5 = *param_1;
            if b_var5 == 0xfe {
                param_1 = param_1.add(1);
                FUN_004a9590(*param_1 as u32);
            } else if b_var5 < 0x10 {
                FUN_004a9590(b_var5 as u32);
            } else {
                if b_var5 == 0x20 && DAT_009cd270 != 0 {
                    b_var5 = *param_1.add(1);
                    let mut pbvar2 = param_1.add(1);
                    let mut u_var1: u16 = 0;
                    let buf = local_50.as_mut_ptr();
                    while b_var5 != 0x20 {
                        b_var5 = *pbvar2;
                        if b_var5 == 0 {
                            break;
                        }
                        pbvar2 = pbvar2.add(1);
                        *buf.add(u_var1 as usize) = b_var5;
                        b_var5 = *pbvar2;
                        u_var1 = u_var1.wrapping_add(1);
                    }
                    *buf.add(u_var1 as usize) = 0;
                    let u_var3 = FUN_004278c0(buf);
                    let u_var4 = FUN_004278c0(&raw mut DAT_00523d40);
                    let lhs = (DAT_009d8c00 as i32 as u32).wrapping_sub(2) as i32;
                    let rhs = ((u_var3 as u32) & 0xffff)
                        .wrapping_add((u_var4 as u32) & 0xffff)
                        .wrapping_add(DAT_00a08108 as i32 as u32)
                        as i32;
                    if lhs < rhs {
                        b_var5 = 10;
                    } else {
                        b_var5 = *param_1;
                    }
                }
                let f = DAT_009a6fe4;
                f(b_var5);
            }
            let pbvar2 = param_1.add(1);
            param_1 = param_1.add(1);
            b_var5 = *pbvar2;
        }
    }
}
