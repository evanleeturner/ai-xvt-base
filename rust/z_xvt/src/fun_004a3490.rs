// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732950: [u16; 0];
    static mut DAT_00732e70: [u32; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a3490(param_1: u16, param_2: u32) {
    // SAFETY: the globals are the game's own arrays, indexed by a u16 as the original does.
    unsafe {
        (&raw mut DAT_00732950)
            .cast::<u16>()
            .add(param_1 as usize)
            .write(param_1.wrapping_add(0x8000));
        (&raw mut DAT_00732e70)
            .cast::<u32>()
            .add(param_1 as usize)
            .write(param_2);
    }
}
