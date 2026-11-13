// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0073ba30: [u32; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00409b60() {
    // SAFETY: DAT_0073ba30 is the game's own array of at least 0x1392 words;
    // each word is written to zero in order, as the original does.
    unsafe {
        let mut p = (&raw mut DAT_0073ba30).cast::<u32>();
        let mut i: i32 = 0x1392;
        while i != 0 {
            p.write_unaligned(0);
            p = p.add(1);
            i -= 1;
        }
    }
}
