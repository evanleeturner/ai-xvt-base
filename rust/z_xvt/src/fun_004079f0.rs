// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00703c60: u16;
    static mut DAT_006ffbe6: i16;
    static mut DAT_00730884: i32;
    static mut DAT_0071e522: i16;
}

#[inline(always)]
unsafe fn rd(addr: i32) -> i16 {
    // SAFETY: the address is a field of the game's own structure.
    unsafe { (addr as usize as *const i16).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004079f0() {
    // SAFETY: the globals are the game's own memory, and DAT_00730884 holds
    // the address of the game's structure that is read at the offsets below.
    unsafe {
        let u_var2: u16 = DAT_00703c60;
        let mut s_var3: i16 = 1;
        if DAT_006ffbe6 != 0 {
            DAT_006ffbe6 = 0;
            return;
        }
        DAT_00703c60 = DAT_00703c60.wrapping_add(rd(DAT_00730884.wrapping_add(0x1c)) as u16);
        if rd(DAT_00730884.wrapping_add(0x1e)) == 0 {
            if u_var2 <= DAT_00703c60 {
                return;
            }
        } else if DAT_00703c60 < u_var2 {
            s_var3 = 2;
        }
        let mut s_var4: i16 = DAT_0071e522;
        if DAT_0071e522 < 0 {
            loop {
                s_var4 = s_var4.wrapping_add(rd(DAT_00730884.wrapping_add(0xe)));
                if !(s_var4 < 0) {
                    break;
                }
            }
        }
        while (rd(DAT_00730884.wrapping_add(0xe)) as u16 as i32) <= (s_var4 as i32) {
            s_var4 = s_var4.wrapping_sub(rd(DAT_00730884.wrapping_add(0xe)));
        }
        if rd(DAT_00730884.wrapping_add(0x18)) == 0 {
            let i_var1: i32 = DAT_00730884.wrapping_add((s_var4 as i32).wrapping_mul(4));
            if rd(DAT_00730884.wrapping_add(0x14)) == 1 {
                DAT_0071e522 = DAT_0071e522.wrapping_add(s_var3);
                if rd(i_var1.wrapping_add(0x36)) != rd(i_var1.wrapping_add(0x32)) {
                    DAT_006ffbe6 = 1;
                    return;
                }
            } else {
                DAT_0071e522 = DAT_0071e522.wrapping_sub(s_var3);
                if rd(i_var1.wrapping_add(0x2e)) != rd(i_var1.wrapping_add(0x32)) {
                    DAT_006ffbe6 = 1;
                    return;
                }
            }
        } else {
            let i_var1: i32 = DAT_00730884.wrapping_add((s_var4 as i32).wrapping_mul(4));
            if rd(DAT_00730884.wrapping_add(0x14)) == 1 {
                DAT_0071e522 = DAT_0071e522.wrapping_sub(s_var3);
                if rd(i_var1.wrapping_add(0x2c)) != rd(i_var1.wrapping_add(0x30)) {
                    DAT_006ffbe6 = 1;
                }
            } else {
                DAT_0071e522 = DAT_0071e522.wrapping_add(s_var3);
                if rd(i_var1.wrapping_add(0x34)) != rd(i_var1.wrapping_add(0x30)) {
                    DAT_006ffbe6 = 1;
                    return;
                }
            }
        }
    }
}
