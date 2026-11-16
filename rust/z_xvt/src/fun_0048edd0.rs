// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00525508: u8;
    static mut DAT_0071b012: i8;
    static mut s_Rating_points_awarded___d_to_pla_0052ad48: [i8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0048edd0(param_1: u32) {
    // SAFETY: the globals are the game's own memory; the string table is
    // indexed at the offset the original uses.
    unsafe {
        if 0x3f < param_1 && (DAT_00525508 as u32) != param_1 {
            DAT_0071b012 = (&raw const s_Rating_points_awarded___d_to_pla_0052ad48)
                .cast::<i8>()
                .byte_add(param_1.wrapping_add(0x38) as usize)
                .read();
            return;
        }
        DAT_0071b012 = param_1 as i8;
    }
}
