// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00759657: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d5520() -> i32 {
    // SAFETY: DAT_00759657 is the game's own global, read by value.
    unsafe {
        let v = DAT_00759657;
        let sum = (v as u32).wrapping_add(((v >> 0x1f) as u32) & 7);
        (sum as i32) >> 3
    }
}
