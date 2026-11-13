// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732e54: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00498430(param_1: u32, param_2: u16) {
    let _ = param_1;
    // SAFETY: DAT_00732e54 is the game's own global holding an address; the
    // 2 bytes at offset 0xbd from it are written as the original does.
    unsafe {
        ((DAT_00732e54.wrapping_add(0xbd)) as *mut u16).write_unaligned(param_2);
    }
}
