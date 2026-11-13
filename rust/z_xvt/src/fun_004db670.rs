// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0077e0a2: u32;
    static mut DAT_0077dc66: u32;
    static mut DAT_0077e0a6: u32;
    static mut DAT_007806f2: u32;
    static mut DAT_0077dc6a: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004db670(param_1: u32, param_2: u32) {
    // SAFETY: the globals are the game's own memory; the addresses are not
    // 4-byte aligned, so every access is unaligned.
    unsafe {
        let off = (&raw mut DAT_0077dc66).read_unaligned().wrapping_mul(0x440) as usize;
        (&raw mut DAT_0077e0a2)
            .cast::<u8>()
            .byte_add(off)
            .cast::<u32>()
            .write_unaligned(param_1);
        let off = (&raw mut DAT_0077dc66).read_unaligned().wrapping_mul(0x440) as usize;
        (&raw mut DAT_0077e0a6)
            .cast::<u8>()
            .byte_add(off)
            .cast::<u32>()
            .write_unaligned(param_2);
        (&raw mut DAT_007806f2).write_unaligned(0xffff_ffff);
        (&raw mut DAT_0077dc6a).write_unaligned(1);
    }
}
