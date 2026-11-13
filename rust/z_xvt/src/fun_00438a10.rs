// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071f2d8: [u8; 4];
    static mut DAT_0071f2dc: u8;
    static mut DAT_0071f2dd: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00438a10() -> i32 {
    // SAFETY: the globals are the game's own memory; only bytes are read.
    unsafe {
        let a = u32::from((*(&raw const DAT_0071f2d8))[3]);
        let b = u32::from(DAT_0071f2dc);
        let c = u32::from(DAT_0071f2dd);
        c.wrapping_add(a.wrapping_mul(0x3c).wrapping_add(b).wrapping_mul(0x3c)) as i32
    }
}
