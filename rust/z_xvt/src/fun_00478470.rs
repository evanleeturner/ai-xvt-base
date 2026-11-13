// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00703abc: u16;
    static mut DAT_005f2640: u16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00478470() {
    let mut s_var2: i16 = 0x10;
    // SAFETY: the globals are the game's own memory, read and written
    // where the original does.
    unsafe {
        loop {
            let u_var1: u16 = DAT_00703abc & 0x8000;
            s_var2 = s_var2.wrapping_sub(1);
            let d: u32 = DAT_00703abc as u32;
            let bit: u16 = ((((d >> 8) ^ ((d & 0xff).wrapping_mul(2))) & 0x80) != 0) as u16;
            DAT_00703abc = bit.wrapping_add((d.wrapping_mul(2)) as u16);
            let e: u32 = DAT_005f2640 as u32;
            DAT_005f2640 = ((u_var1 != 0) as u16).wrapping_add((e.wrapping_mul(2)) as u16);
            if s_var2 == 0 {
                break;
            }
        }
    }
}
