// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005233c8: u8;
    static mut DAT_009e8f52: u8;
    static mut DAT_00524040: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a95f0(param_1: u32) {
    // SAFETY: the globals and the table are the game's own memory; the index
    // is read from the table exactly as the original does.
    unsafe {
        if param_1 > 0x3f && (DAT_005233c8 as u32) != param_1 {
            let table = (&raw mut DAT_00524040).cast::<u8>();
            DAT_009e8f52 = table.add(param_1 as usize).read();
            return;
        }
        DAT_009e8f52 = param_1 as u8;
    }
}
