// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals, non_camel_case_types)]
unsafe extern "C" {
    static mut DAT_00525508: u8;
    static mut DAT_006da9fa: u8;
    static mut s_Rating_points_awarded___d_to_pla_0052ad48: [u8; 1];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0048ed70(param_1: u32) {
    // SAFETY: the globals and the string table are the game's own memory;
    // the table is read at the same byte offset the original uses.
    unsafe {
        if param_1 >= 0x40 && (DAT_00525508 as u32) != param_1 {
            let idx = param_1.wrapping_add(0x38) as usize;
            DAT_006da9fa = (&raw const s_Rating_points_awarded___d_to_pla_0052ad48)
                .cast::<u8>()
                .byte_add(idx)
                .read();
        } else {
            DAT_006da9fa = param_1 as u8;
        }
    }
}
