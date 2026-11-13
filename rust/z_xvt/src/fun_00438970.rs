// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071b740: i8;
    static mut DAT_0071e528: i8;
    static mut DAT_0072f444: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00438970() {
    // SAFETY: the globals are the game's own memory; the pointer walks the
    // game's table in 0x5bd-byte steps, read exactly as the original does.
    unsafe {
        DAT_0072f444 = 0;
        let mut p_var1: *const i8 = &raw mut DAT_0071b740;
        let end: *const i8 = &raw mut DAT_0071e528;
        loop {
            let c = p_var1.read();
            if c == 1 || c == 2 {
                DAT_0072f444 = DAT_0072f444.wrapping_add(1);
            }
            p_var1 = p_var1.wrapping_byte_add(0x5bd);
            if !(p_var1 < end) {
                break;
            }
        }
    }
}
