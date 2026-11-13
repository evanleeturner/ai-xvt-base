// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009d77f8: u8;
    static mut DAT_009a6ff0: u16;
    static mut DAT_009d8b86: u8;
    static mut DAT_009ecc34: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00451560() {
    // SAFETY: the globals are the game's own memory; the indexed write lands
    // at the same byte offset from DAT_009d8b86 as the original.
    unsafe {
        DAT_009d77f8 = 0;
        DAT_009a6ff0 = 0xffff;
        let off = DAT_009ecc34.wrapping_mul(0x10) as usize;
        (&raw mut DAT_009d8b86)
            .byte_add(off)
            .cast::<u16>()
            .write_unaligned(0);
    }
}
