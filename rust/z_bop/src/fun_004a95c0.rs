// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005233c8: u8;
    static mut DAT_00a08100: u8;
    static mut DAT_00524040: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a95c0(param_1: u32) {
    // SAFETY: the globals are the game's own memory; the table is indexed
    // by param_1 exactly as the original does.
    unsafe {
        if param_1 > 0x3f && DAT_005233c8 as u32 != param_1 {
            DAT_00a08100 = (&raw const DAT_00524040)
                .cast::<u8>()
                .add(param_1 as usize)
                .read();
            return;
        }
        DAT_00a08100 = param_1 as u8;
    }
}
