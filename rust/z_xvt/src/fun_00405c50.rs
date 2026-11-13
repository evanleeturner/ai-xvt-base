// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732f92: u16;
    static mut DAT_0071b6b0: u16;
    static mut DAT_0070acc6: u16;
    static mut DAT_00732f94: u16;
    static mut DAT_00732e80: u16;
    static mut DAT_00536920: u32;
    static mut DAT_00536924: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_0048f940() -> u16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00405c50() {
    // SAFETY: the globals and the callee are the game's own; they are
    // written and called in the original's order.
    unsafe {
        DAT_00732f92 = 0;
        DAT_0071b6b0 = 0;
        DAT_0070acc6 = 0;
        DAT_00732f94 = 0;
        DAT_00732e80 = FUN_0048f940();
        DAT_00732f92 = (DAT_00732e80 != 0) as u16;
        DAT_0071b6b0 = 0;
        DAT_00536920 = 0;
        DAT_00536924 = 0xFFFF_FFFF;
    }
}
