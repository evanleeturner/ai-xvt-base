// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009ecc34: i32;
    static mut DAT_009e9681: i8;
    static mut DAT_009d6940: i8;
    static mut DAT_009e9683: i8;
    static mut DAT_009e9a77: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00439900();
    fn FUN_004abe50();
    fn FUN_0042ec80(param_1: i32);
    fn FUN_004aba50();
    fn FUN_0043ec40();
    fn FUN_00439710();
    fn FUN_004398b0();
    fn FUN_00439c60();
    fn FUN_004419d0();
    fn FUN_0042f030(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00438df0(param_1: i32) {
    // SAFETY: the globals and callees are the game's own; the indexed
    // globals are read at the same offsets as the original.
    unsafe {
        if param_1 == DAT_009ecc34 {
            let base = (DAT_009ecc34 as isize).wrapping_mul(0x5bd);
            if (&raw mut DAT_009e9681).offset(base).read() == 0 && DAT_009d6940 == 0 {
                let off3 = (param_1 as isize).wrapping_mul(0x5bd);
                if (&raw mut DAT_009e9683).offset(off3).read() != 0 {
                    FUN_00439900();
                    FUN_004abe50();
                    FUN_0042ec80(0);
                    FUN_004aba50();
                    FUN_0043ec40();
                    return;
                }
                let base2 = (DAT_009ecc34 as isize).wrapping_mul(0x5bd);
                let c_var1 = (&raw mut DAT_009e9a77).offset(base2).read();
                if c_var1 == 0 {
                    FUN_00439710();
                    FUN_0043ec40();
                    return;
                }
                if c_var1 == 0x13 {
                    FUN_004398b0();
                    FUN_0043ec40();
                    return;
                }
                if c_var1 == 0x14 {
                    FUN_00439c60();
                    FUN_004abe50();
                    FUN_0042ec80(0);
                    FUN_004aba50();
                    FUN_0043ec40();
                    return;
                }
                FUN_004abe50();
                FUN_0042ec80(0);
                FUN_004419d0();
                FUN_004aba50();
                FUN_0043ec40();
                return;
            }
            FUN_004abe50();
            FUN_0042f030(0);
            FUN_004aba50();
        }
    }
}
