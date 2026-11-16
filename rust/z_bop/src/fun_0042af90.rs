// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009ecc34: i32;
    static mut DAT_009d7680: i32;
    static mut DAT_009e9a5b: i8;
    static mut DAT_005233a8: i32;
    static mut _DAT_005233b0: i32;
    static mut DAT_005233b8: i32;
    static mut DAT_005233bc: i32;
    static mut DAT_005233c4: i32;
    static mut DAT_005233c0: i32;
    static mut _DAT_005233b4: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0042af90() {
    // SAFETY: the globals are the game's own memory; the writes go to the
    // record at DAT_009e9a5b indexed by DAT_009ecc34, as the original does.
    unsafe {
        let i_var1 = DAT_009ecc34.wrapping_mul(0x5bd);
        DAT_009d7680 = 0;
        let base = (&raw mut DAT_009e9a5b).byte_offset(i_var1 as isize);
        base.cast::<u32>().write_unaligned(0);
        base.byte_add(4).cast::<u32>().write_unaligned(0xfffffb00);
        base.byte_add(8).cast::<u32>().write_unaligned(0);
        base.byte_add(14).cast::<u16>().write_unaligned(0);
        base.byte_add(10).cast::<u16>().write_unaligned(0x4000);
        base.byte_add(12).cast::<u16>().write_unaligned(0);
        DAT_005233a8 = 0x3f800000;
        _DAT_005233b0 = 0x3f800000;
        DAT_005233b8 = 1;
        DAT_005233bc = 1;
        DAT_005233c4 = 1;
        DAT_005233c0 = 1;
        _DAT_005233b4 = 1;
    }
}
