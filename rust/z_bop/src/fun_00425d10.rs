// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009d77a8: u16;
    static mut DAT_00555c80: u16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00425d10() {
    let mut s_var2: i16 = 0x10;
    loop {
        // SAFETY: the globals are the game's own memory, read and written
        // in the same order as the original.
        unsafe {
            let d: u32 = DAT_009d77a8 as u32;
            let u_var1: u32 = d & 0x8000;
            s_var2 = s_var2.wrapping_sub(1);
            let t: u32 = (((d >> 8) ^ (d & 0xff).wrapping_mul(2)) & 0x80 != 0) as u32;
            let d_now: u32 = DAT_009d77a8 as u32;
            DAT_009d77a8 = t.wrapping_add(d_now.wrapping_mul(2)) as u16;
            let e: u32 = DAT_00555c80 as u32;
            DAT_00555c80 = ((u_var1 != 0) as u32).wrapping_add(e.wrapping_mul(2)) as u16;
        }
        if s_var2 == 0 {
            break;
        }
    }
}
