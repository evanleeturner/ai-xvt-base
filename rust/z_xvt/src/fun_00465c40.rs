// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006a8820: u32;
    static mut DAT_006a8832: u32;
    static mut DAT_006a884a: u32;
    static mut DAT_006a8858: u32;
    static mut DAT_006a8866: u32;
    static mut DAT_006a8874: u32;
    static mut DAT_006a887e: u32;
    static mut _DAT_006a8888: u32;
    static mut DAT_006a888e: u32;
    static mut DAT_006a8840: u32;
    static mut DAT_006a8894: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_0048f510();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00465c40() {
    // SAFETY: the callee and the globals are the game's own memory;
    // each global is written where the original writes it.
    unsafe {
        FUN_0048f510();
        FUN_0048f510();
        FUN_0048f510();
        FUN_0048f510();
        FUN_0048f510();
        FUN_0048f510();
        FUN_0048f510();
        FUN_0048f510();
        FUN_0048f510();
        FUN_0048f510();
        FUN_0048f510();
        DAT_006a8820 = 0;
        DAT_006a8832 = 0;
        DAT_006a884a = 0;
        DAT_006a8858 = 0;
        DAT_006a8866 = 0;
        DAT_006a8874 = 0;
        DAT_006a887e = 0;
        _DAT_006a8888 = 0;
        DAT_006a888e = 0;
        DAT_006a8840 = 0;
        DAT_006a8894 = 0;
    }
}
