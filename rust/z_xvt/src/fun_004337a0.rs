// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4984: i16;
    static mut DAT_006da4fc: i16;
    static mut DAT_006db7bc: i16;
    static mut DAT_007091ae: i16;
    static mut DAT_0071ecf4: i32;
    static mut DAT_0071bb23: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048ed00(a: i16, b: i16, c: i16, d: i16);
    fn FUN_00433e80();
    fn FUN_004341f0();
    fn FUN_004343f0(a: i32);
    fn FUN_00435c70();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004337a0() {
    // SAFETY: the globals and callees are the game's own memory and code;
    // the read is at the byte offset the original computes.
    unsafe {
        FUN_0048ed00(
            DAT_006db7bc,
            DAT_006da4fc,
            DAT_007091ae.wrapping_add(DAT_006db7bc),
            DAT_006d4984.wrapping_add(DAT_006da4fc),
        );
        FUN_00433e80();
        FUN_004341f0();
        let local_val: i32 = (&raw mut DAT_0071bb23)
            .cast::<u8>()
            .byte_offset(DAT_0071ecf4.wrapping_mul(0x5bd) as isize)
            .cast::<i32>()
            .read_unaligned();
        if local_val < -0x10000 {
            FUN_004343f0(1);
            FUN_00435c70();
            FUN_004343f0(0);
            return;
        }
        FUN_004343f0(0);
        FUN_00435c70();
        FUN_004343f0(1);
    }
}
