// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0075dc4e: u32;
    static mut DAT_0075dc46: u32;
    static mut DAT_0075dc4a: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004db490(param_1: u32) -> u32 {
    // SAFETY: the globals are the game's own memory; the fill covers the
    // 0x8000 dwords starting at DAT_0075dc4e, as the original does.
    unsafe {
        let mut p = &raw mut DAT_0075dc4e;
        let mut i: i32 = 0x8000;
        while i != 0 {
            p.write_unaligned(0);
            p = p.add(1);
            i -= 1;
        }
        DAT_0075dc46 = param_1;
        DAT_0075dc4a = param_1;
    }
    1
}
