// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009d1308: i32;
    static mut DAT_009a2000: i16;
    static mut DAT_00a07ce8: i32;
    static mut DAT_009a7ba4: i32;
    static mut DAT_009fd430: i32;
    static mut DAT_009eca28: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004173d0() -> i32 {
    // SAFETY: the globals are the game's own memory, read by value.
    unsafe {
        let a = DAT_009d1308.wrapping_mul(0x24);
        let b = (DAT_009a2000 as i32).wrapping_mul(0x68c);
        let c = DAT_00a07ce8.wrapping_mul(0xd5);
        let d = DAT_009a7ba4.wrapping_mul(0x4c);
        let e = DAT_009fd430.wrapping_mul(0xa);
        let f = DAT_009eca28.wrapping_mul(0x462);
        a.wrapping_add(b)
            .wrapping_add(c)
            .wrapping_add(d)
            .wrapping_add(e)
            .wrapping_add(0x9251)
            .wrapping_add(f)
    }
}
