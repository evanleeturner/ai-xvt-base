// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a210a: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00415a40(param_1: u32) -> u32 {
    let v1: u32 = param_1 & 0xffff;
    let idx: u32 = v1.wrapping_mul(0x126);
    // SAFETY: DAT_009a210a is the game's own table; the original reads the
    // byte at offset v1 * 0x126 from its start.
    let b: u8 = unsafe {
        (&raw mut DAT_009a210a)
            .cast::<u8>()
            .byte_add(idx as usize)
            .read()
    };
    ((v1.wrapping_mul(0x49) >> 8).wrapping_shl(8) | u32::from(b)) & 0xffff00ff
}
