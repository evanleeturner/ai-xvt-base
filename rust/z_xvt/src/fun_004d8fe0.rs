// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0064f708: u32;
    static mut DAT_0064f688: u32;
    static mut DAT_00532adc: u32;
    static mut DAT_00532ae0: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d8fe0() -> u32 {
    // SAFETY: the globals are the game's own memory; the copy reads 0x20
    // dwords from DAT_0064f708 and writes 0x20 dwords to DAT_0064f688,
    // in the same order as the original.
    unsafe {
        let mut src = &raw mut DAT_0064f708;
        let mut dst = &raw mut DAT_0064f688;
        let mut i: i32 = 0x20;
        while i != 0 {
            dst.write_unaligned(src.read_unaligned());
            src = src.add(1);
            dst = dst.add(1);
            i -= 1;
        }
        DAT_00532adc = DAT_00532ae0;
    }
    1
}
