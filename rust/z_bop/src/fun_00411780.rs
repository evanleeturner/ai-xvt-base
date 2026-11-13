// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a08242: u16;
    static mut DAT_009e95f0: u16;
    static mut DAT_009d8c06: u16;
    static mut DAT_00a08244: u16;
    static mut _DAT_00a08130: u16;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004acac0() -> u32;
    fn FUN_004117f0();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00411780() {
    // SAFETY: the globals and callees are the game's own; accessed in the original's order.
    unsafe {
        DAT_00a08242 = 0;
        DAT_009e95f0 = 0;
        DAT_009d8c06 = 0;
        DAT_00a08244 = 0;
        let u_var1 = FUN_004acac0();
        _DAT_00a08130 = u_var1 as u16;
        DAT_00a08242 = (_DAT_00a08130 != 0) as u16;
        FUN_004117f0();
        DAT_009e95f0 = 0;
    }
}
