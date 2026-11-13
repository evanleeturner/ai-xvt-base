// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00758fd0: [u8; 256];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004dbe00(param_1: u8) -> u8 {
    // SAFETY: the table is the game's own memory; param_1 is a byte, so the
    // index stays within the 256 entries read.
    unsafe {
        let p = (&raw mut DAT_00758fd0).cast::<u8>();
        ((p.add(param_1 as usize).read() & 0x80) != 0) as u8
    }
}
