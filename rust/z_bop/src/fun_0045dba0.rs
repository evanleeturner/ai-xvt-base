// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a07cec: u16;
    static mut DAT_009a1fe0: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_009d6954: i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0045dba0(param_1: u16, param_2: i16) -> u8 {
    // SAFETY: the globals are the game's own; the table addresses computed
    // from them are read where the original reads them.
    unsafe {
        let mut u_var3: u16 = 0;
        let mut u_var5: u16 = DAT_00a07cec;
        while (u_var5 as i32) < DAT_009a1fe0 {
            let base = (u_var5 as i32)
                .wrapping_mul(0x23)
                .wrapping_add(DAT_009a1fe8);
            if *((base.wrapping_add(3)) as *const i8) != 0 {
                let p = ((base.wrapping_add(0x1f)) as *const i32).read_unaligned();
                let i_var2 = (p.wrapping_add(0xa9) as *const i32).read_unaligned();
                if param_1 == (i_var2.wrapping_add(0x42) as *const u16).read_unaligned()
                    && u_var5 != param_1
                    && (i_var2.wrapping_add(0x36e) as *const u16).read_unaligned()
                        != (i_var2.wrapping_add(0x42) as *const u16).read_unaligned()
                {
                    let c_var1 = *((i_var2.wrapping_add(0x5e)) as *const i8);
                    if c_var1 == 0x0b || c_var1 == 0x0c || c_var1 == 0x17 {
                        u_var3 = u_var3.wrapping_add(1);
                    }
                }
            }
            u_var5 = u_var5.wrapping_add(1);
        }
        let u_var4 = param_1 as i32;
        let row = u_var4.wrapping_mul(0x23).wrapping_add(DAT_009a1fe8);
        if u_var4 < DAT_009a1fe0 {
            if param_2 == 1 {
                u_var5 = 100;
            } else {
                let c_var1 = *((row.wrapping_add(2)) as *const i8);
                if c_var1 == 4 || c_var1 == 5 {
                    u_var5 = 6;
                } else {
                    u_var5 = (0u16.wrapping_sub((c_var1 == 3) as u16) & 2).wrapping_add(2);
                }
            }
        } else {
            u_var5 = 2;
        }
        if ((row.wrapping_add(0x1b)) as *const i32).read_unaligned() != -1 {
            if DAT_009d6954 == 0 {
                u_var5 = (0u16.wrapping_sub((param_2 == 1) as u16) & 2).wrapping_add(2);
            } else if DAT_009d6954 == 1 {
                u_var5 = (0u16.wrapping_sub((param_2 == 1) as u16) & 5).wrapping_add(3);
            } else if DAT_009d6954 == 2 {
                u_var5 = (0u16.wrapping_sub((param_2 == 1) as u16) & 0x60).wrapping_add(4);
            }
        }
        (u_var3 < u_var5) as u8
    }
}
