// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009d113c: u16;
    static mut DAT_00555c84: u16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00425cb0() {
    let mut s_var2: i16 = 0x10;
    while s_var2 != 0 {
        s_var2 = s_var2.wrapping_sub(1);
        if s_var2 == 0 {
            break;
        }
        // SAFETY: the globals are the game's own memory, read and written
        // in the original's order.
        unsafe {
            let u_var1: u16 = DAT_009d113c & 0x8000;
            let d: u32 = DAT_009d113c as u32;
            let carry: u16 = ((((d >> 8) ^ ((d & 0xff).wrapping_mul(2))) & 0x80) != 0) as u16;
            DAT_009d113c = carry.wrapping_add(DAT_009d113c.wrapping_mul(2));
            DAT_00555c84 = ((u_var1 != 0) as u16).wrapping_add(DAT_00555c84.wrapping_mul(2));
        }
    }
}
