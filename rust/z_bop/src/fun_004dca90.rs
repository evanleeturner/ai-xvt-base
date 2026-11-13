// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7498: [u8; 256];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004dca90(param_1: u8) -> u8 {
    // SAFETY: the table is the game's own memory; the index is a byte, so it
    // stays within the 256 entries read.
    unsafe {
        let p = &raw mut DAT_00aa7498;
        let v = p.cast::<u8>().add(param_1 as usize).read();
        ((v & 0x80) != 0) as u8
    }
}
