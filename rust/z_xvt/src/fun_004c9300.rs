// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0081b740: i8;
    static mut DAT_005322f4: i8;
    static mut DAT_005322e4: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn sprintf(dst: *mut i8, fmt: *const i8, ...);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004c9300(param_1: u32) {
    // SAFETY: the globals and sprintf are the game's own; the format strings
    // and destination buffer are addressed, not copied.
    unsafe {
        if param_1 / 0xe10 == 0 {
            sprintf(&raw mut DAT_0081b740, (&raw mut DAT_005322f4).cast_const());
            return;
        }
        sprintf(
            &raw mut DAT_0081b740,
            (&raw mut DAT_005322e4).cast_const(),
            param_1 / 0xe10,
            (((param_1 as u64) / 0x3c) % 0x3c) as i32,
            param_1 % 0x3c,
        );
    }
}
