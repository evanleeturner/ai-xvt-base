// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732f94: u16;
    static mut DAT_0070acc6: u16;
    static mut DAT_007092b0: u16;
    static mut DAT_006db592: u16;
    static mut DAT_0071ecf0: u16;
    static mut DAT_0071e534: u16;
    static mut DAT_0071b6b0: u16;
    static mut DAT_006d9d88: u16;
    static mut DAT_006d9d92: u16;
    static mut DAT_00732f92: u16;
    static mut DAT_0073254c: u16;
    static mut DAT_00732554: u16;
    static mut DAT_0071eae4: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00405cf0(param_1: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004058c0() {
    // entry: 0 = return, 1 = loop condition check, 2 = loop body
    let mut entry: u32;
    // SAFETY: the callee and the globals are the game's own memory.
    unsafe {
        loop {
            let u_var1 = FUN_00405cf0(-2);
            if (u_var1 as i16) != 0 {
                entry = 3;
                break;
            }
            if (DAT_00732f94 & 0xf) != 0 {
                entry = 1;
                break;
            }
            if DAT_0070acc6 != 0 {
                entry = 3;
                break;
            }
        }
        if entry == 3 {
            if (DAT_00732f94 & 0xf) != 0 {
                entry = 2;
            } else if DAT_0070acc6 != 0 {
                entry = 1;
            } else {
                return;
            }
        }
        loop {
            if entry != 2 {
                if !((DAT_00732f94 & 0xf) != 0 || DAT_0070acc6 != 0) {
                    return;
                }
            }
            entry = 1;
            let mut u_var3: u16 = 0;
            FUN_00405cf0(-2);
            DAT_007092b0 = DAT_006db592;
            DAT_0071ecf0 = 0;
            DAT_0071e534 = 0;
            if DAT_0071b6b0 != 0 {
                DAT_0071ecf0 = DAT_006d9d88.wrapping_shl(7);
                u_var3 = DAT_006d9d92.wrapping_shl(6);
                DAT_0071e534 = DAT_0070acc6;
            }
            if ((DAT_00732f92 as u8) | 1) != 0 {
                if DAT_0071ecf0 == 0 {
                    DAT_0071ecf0 = DAT_0073254c.wrapping_mul(0x78);
                }
                if u_var3 == 0 {
                    u_var3 = DAT_00732554.wrapping_mul(0x32);
                }
                DAT_0071e534 |= DAT_00732f94;
            }
            let mut u_var2 = DAT_0071ecf0;
            if 0x7fff < DAT_0071ecf0 {
                u_var2 = DAT_0071ecf0.wrapping_neg();
            }
            if (u_var2 as i16) < 0x801 {
                DAT_0071ecf0 = 0;
            }
            u_var2 = u_var3;
            if 0x7fff < u_var3 {
                u_var2 = u_var3.wrapping_neg();
            }
            DAT_0071eae4 = u_var3;
            if (u_var2 as i16) < 0x601 {
                DAT_0071eae4 = 0;
            }
        }
    }
}
