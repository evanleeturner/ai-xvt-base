// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005233c8: u8;
    static mut DAT_009a807a: u8;
    static mut DAT_00524040: [u8; 1];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a9590(param_1: u32) {
    // SAFETY: the globals are the game's own memory; the table at
    // DAT_00524040 is indexed by param_1 exactly as the original does.
    unsafe {
        if param_1 > 0x3f && (DAT_005233c8 as u32) != param_1 {
            DAT_009a807a = (&raw mut DAT_00524040)
                .cast::<u8>()
                .add(param_1 as usize)
                .read();
            return;
        }
        DAT_009a807a = param_1 as u8;
    }
}
