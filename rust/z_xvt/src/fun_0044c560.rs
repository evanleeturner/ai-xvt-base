// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732a3c: u16;
    static mut DAT_006d4980: i32;
    static mut DAT_006d4988: i32;
    static mut DAT_007092d4: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0042b150(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0044c560(param_1: u16, param_2: i16) -> u8 {
    // SAFETY: all addresses are the game's own memory structures, read
    // exactly where the original reads them.
    unsafe {
        let mut u_var5: u16 = 0;
        let mut u_var4: u16 = DAT_00732a3c;
        if (u_var4 as i32) < DAT_006d4980 {
            loop {
                let i_var2 = (u_var4 as i32)
                    .wrapping_mul(0x23)
                    .wrapping_add(DAT_006d4988);
                if *((i_var2.wrapping_add(3)) as u32 as usize as *const i8) != 0 {
                    let p = *((i_var2.wrapping_add(0x1f)) as u32 as usize as *const i32);
                    let i_var2 =
                        ((p.wrapping_add(0xa7)) as u32 as usize as *const i32).read_unaligned();
                    let i_var3 = FUN_0042b150(i_var2);
                    let w18 = ((i_var3.wrapping_add(0x18)) as u32 as usize as *const u16)
                        .read_unaligned();
                    if param_1 == w18
                        && param_1 != u_var4
                        && ((i_var2.wrapping_add(0x342)) as u32 as usize as *const u16)
                            .read_unaligned()
                            != w18
                    {
                        let c_var1 = *((i_var3.wrapping_add(0x34)) as u32 as usize as *const i8);
                        if c_var1 == 0x0b || c_var1 == 0x0c || c_var1 == 0x17 {
                            u_var5 = u_var5.wrapping_add(1);
                        }
                    }
                }
                u_var4 = u_var4.wrapping_add(1);
                if !((u_var4 as i32) < DAT_006d4980) {
                    break;
                }
            }
        }
        let u_var6: u32 = param_1 as u32;
        if (u_var6 as i32) < DAT_006d4980 {
            if param_2 == 1 {
                u_var4 = 100;
            } else {
                let addr = u_var6
                    .wrapping_mul(0x23)
                    .wrapping_add(2)
                    .wrapping_add(DAT_006d4988 as u32);
                let c_var1 = *(addr as usize as *const i8);
                if c_var1 == 4 || c_var1 == 5 {
                    u_var4 = 6;
                } else {
                    u_var4 = if c_var1 == 3 { 4 } else { 2 };
                }
            }
        } else {
            u_var4 = 2;
        }
        let addr = u_var6
            .wrapping_mul(0x23)
            .wrapping_add(0x1b)
            .wrapping_add(DAT_006d4988 as u32);
        if (addr as usize as *const i32).read_unaligned() != -1 {
            let d = DAT_007092d4;
            if d == 0 {
                u_var4 = if param_2 == 1 { 4 } else { 2 };
            } else if d == 1 {
                u_var4 = if param_2 == 1 { 8 } else { 3 };
            } else if d == 2 {
                u_var4 = if param_2 == 1 { 0x64 } else { 4 };
            }
        }
        (u_var5 < u_var4) as u8
    }
}
