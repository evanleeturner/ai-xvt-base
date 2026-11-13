// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7a98: u32;
    static mut DAT_00aa7a9c: u32;
    static mut DAT_00aa7698: [u8; 0x400];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004dcad0() -> u32 {
    // SAFETY: the globals are the game's own memory; the index is kept
    // below 0x400 by the original's wraparound, and read as the C does.
    unsafe {
        if DAT_00aa7a98 == DAT_00aa7a9c {
            return DAT_00aa7a9c & 0xffffff00;
        }
        let idx = DAT_00aa7a9c;
        let byte = *(&raw mut DAT_00aa7698).cast::<u8>().add(idx as usize);
        let u_var2 = idx >> 8;
        DAT_00aa7a9c = idx.wrapping_add(1);
        if DAT_00aa7a9c == 0x400 {
            DAT_00aa7a9c = 0;
        }
        (u_var2 << 8) | u32::from(byte)
    }
}
