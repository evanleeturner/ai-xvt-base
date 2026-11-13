// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00664770: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b0b60(param_1: *mut u32, param_2: u32) {
    let mut src = param_1;
    // SAFETY: the global is the game's own memory; the destination runs past it
    // as in the original, and the caller guarantees the source is readable.
    unsafe {
        let mut dst: *mut u32 = &raw mut DAT_00664770;
        let mut count = (param_2 & 0x7fff_ffff) >> 1;
        while count != 0 {
            dst.write_unaligned(src.read_unaligned());
            src = src.wrapping_add(1);
            dst = dst.wrapping_add(1);
            count -= 1;
        }
        let mut bsrc = src as *const u8;
        let mut bdst = dst as *mut u8;
        let mut count = param_2.wrapping_mul(2) & 3;
        while count != 0 {
            bdst.write(bsrc.read());
            bsrc = bsrc.wrapping_add(1);
            bdst = bdst.wrapping_add(1);
            count -= 1;
        }
    }
}
