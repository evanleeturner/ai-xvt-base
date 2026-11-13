// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d9990: i16;
    static mut DAT_006d999c: u8;
    static mut DAT_00645ae0: i16;
    static mut DAT_00645aec: u8;
    static mut DAT_00645b38: i16;
    static mut DAT_00645b44: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004a2e30() {
    // SAFETY: the globals are the game's own memory, read and written
    // where the original does.
    unsafe {
        if DAT_006d9990 != -1 {
            DAT_006d999c = DAT_006d999c.wrapping_add(1);
        }
        if DAT_00645ae0 != -1 {
            DAT_00645aec = DAT_00645aec.wrapping_add(1);
        }
        if DAT_00645b38 != -1 {
            DAT_00645b44 = DAT_00645b44.wrapping_add(1);
        }
    }
}
