// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00757870: u32;
    static mut DAT_007579b0: u32;
    static mut DAT_0081b850: u32;
    static mut _DAT_00758620: u32;
    static mut DAT_00758620: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cb830(param_1: i32, param_2: *mut u32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004f0d60() -> u32 {
    // SAFETY: the globals and the callee are the game's own memory; the two
    // zeroed regions are the game's arrays starting at those addresses.
    unsafe {
        let mut p = &raw mut DAT_00757870;
        let mut n: i32 = 0x50;
        while n != 0 {
            p.write_unaligned(0);
            p = p.add(1);
            n -= 1;
        }
        p = &raw mut DAT_007579b0;
        n = 8;
        while n != 0 {
            p.write_unaligned(0);
            p = p.add(1);
            n -= 1;
        }
        if DAT_0081b850 != 2 {
            _DAT_00758620 = 0x54;
            FUN_004cb830(0, &raw mut DAT_00758620, 4);
        }
    }
    1
}
