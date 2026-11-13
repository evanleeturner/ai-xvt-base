// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a8e3c: i16;
    static mut DAT_009a7b60: i16;
    static mut DAT_009d682c: i16;
    static mut DAT_009a1fe4: i16;
    static mut DAT_0055635c: i32;
    static mut DAT_009ecc34: i32;
    static mut DAT_009e9a63: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a9520(a: i16, b: i16, c: i16, d: i16);
    fn FUN_00436780(a: i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_00436320();
    fn FUN_00436580();
    fn FUN_00437b20();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00435bc0() {
    // SAFETY: the callees and globals are the game's own memory; the indexed
    // read is at the same byte offset from DAT_009e9a63 as the original.
    unsafe {
        FUN_004a9520(
            DAT_009a8e3c,
            DAT_009a7b60,
            DAT_009d682c.wrapping_add(DAT_009a8e3c),
            DAT_009a1fe4.wrapping_add(DAT_009a7b60),
        );
        FUN_00436320();
        FUN_00436580();
        DAT_0055635c = 1;
        let local_val: i32 = (&raw mut DAT_009e9a63)
            .cast::<u8>()
            .byte_offset(DAT_009ecc34.wrapping_mul(0x5bd) as isize)
            .cast::<i32>()
            .read_unaligned();
        if local_val < -0x10000 {
            FUN_00436780(1);
            FUN_00437b20();
            FUN_00436780(0);
        } else {
            FUN_00436780(0);
            FUN_00437b20();
            FUN_00436780(1);
        }
    }
}
