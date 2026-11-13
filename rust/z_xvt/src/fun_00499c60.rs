// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00703c88: i32;
    static mut DAT_006d49a0: i16;
    static mut DAT_00732a38: i32;
    static mut DAT_0072f4e0: i32;
    static mut DAT_006da530: i32;
    static mut DAT_0071eae8: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00499c60() -> i32 {
    // SAFETY: the globals are the game's own memory, read by value in the
    // original's order.
    unsafe {
        let mut acc: u32 = (DAT_00703c88 as u32)
            .wrapping_mul(0x24)
            .wrapping_add((DAT_006d49a0 as i32 as u32).wrapping_mul(0x68c));
        acc = acc.wrapping_add((DAT_00732a38 as u32).wrapping_mul(0xd3));
        acc = acc.wrapping_add((DAT_0072f4e0 as u32).wrapping_mul(8));
        acc = acc.wrapping_add((DAT_006da530 as u32).wrapping_mul(0x4c));
        acc = acc.wrapping_add(0x924f);
        acc = acc.wrapping_add((DAT_0071eae8 as u32).wrapping_mul(0x436));
        acc as i32
    }
}
