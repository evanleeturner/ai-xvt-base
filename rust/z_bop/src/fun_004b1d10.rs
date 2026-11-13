// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00528c80: u32;
    static mut DAT_00528ca0: u32;
    static mut DAT_00664c80: *mut u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b1d10(param_1: *mut u32, param_2: u32) -> u32 {
    // SAFETY: the globals are the game's own memory; param_1 and the
    // destination buffer are the caller's, copied forward word by word
    // exactly as the original does.
    unsafe {
        if DAT_00528c80 < param_2.wrapping_add(DAT_00528ca0) {
            return 0;
        }
        let mut src = param_1;
        if DAT_00664c80 != src {
            let mut dst = DAT_00664c80;
            let mut count = (param_2 & 0x7ffffff) << 3;
            while count != 0 {
                dst.write_unaligned(src.read_unaligned());
                src = src.wrapping_add(1);
                dst = dst.wrapping_add(1);
                count -= 1;
            }
        }
        DAT_00528ca0 = DAT_00528ca0.wrapping_add(param_2);
        let off = (param_2.wrapping_mul(8) as usize).wrapping_mul(4);
        DAT_00664c80 = ((DAT_00664c80 as usize).wrapping_add(off)) as *mut u32;
        1
    }
}
