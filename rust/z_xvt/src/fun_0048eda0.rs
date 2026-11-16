// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00525508: u8;
    static mut DAT_00732e50: u8;
    static mut s_Rating_points_awarded___d_to_pla_0052ad48: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0048eda0(param_1: u32) {
    // SAFETY: the globals are the game's own memory; the string table is
    // indexed at the same byte offset the original uses.
    unsafe {
        if 0x3f < param_1 && u32::from(DAT_00525508) != param_1 {
            DAT_00732e50 = (&raw mut s_Rating_points_awarded___d_to_pla_0052ad48)
                .cast::<u8>()
                .byte_add(param_1.wrapping_add(0x38) as usize)
                .read();
            return;
        }
        DAT_00732e50 = param_1 as u8;
    }
}
