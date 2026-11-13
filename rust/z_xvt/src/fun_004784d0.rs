// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0070a128: u16;
    static mut DAT_005f263c: u16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004784d0() {
    let mut s_var2: i16 = 0x10;
    // SAFETY: the globals are the game's own memory, read and written
    // where the original does.
    unsafe {
        loop {
            let u_var1: u16 = DAT_0070a128 & 0x8000;
            s_var2 = s_var2.wrapping_sub(1);
            let d: u16 = DAT_0070a128;
            let bit: u16 = ((((d >> 8) ^ (d & 0xff).wrapping_mul(2)) & 0x80) != 0) as u16;
            DAT_0070a128 = bit.wrapping_add(d.wrapping_mul(2));
            DAT_005f263c = ((u_var1 != 0) as u16).wrapping_add(DAT_005f263c.wrapping_mul(2));
            if s_var2 == 0 {
                break;
            }
        }
    }
}
