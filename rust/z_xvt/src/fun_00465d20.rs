// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006a8824: u16;
    static mut DAT_006a8836: u16;
    static mut DAT_006a884e: u16;
    static mut DAT_006a885c: u16;
    static mut DAT_006a886a: u16;
    static mut DAT_006a8878: u16;
    static mut DAT_006a8882: u16;
    static mut DAT_006a888c: u16;
    static mut DAT_006a8892: u16;
    static mut DAT_006a8844: u16;
    static mut DAT_006a8898: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048f4b0(param_1: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00465d20() {
    // SAFETY: the globals and the callee are the game's own; each handle
    // is released if nonzero, then cleared, in the original's order.
    unsafe {
        if DAT_006a8824 != 0 {
            FUN_0048f4b0(DAT_006a8824 as u32);
        }
        DAT_006a8824 = 0;
        if DAT_006a8836 != 0 {
            FUN_0048f4b0(DAT_006a8836 as u32);
        }
        DAT_006a8836 = 0;
        if DAT_006a884e != 0 {
            FUN_0048f4b0(DAT_006a884e as u32);
        }
        DAT_006a884e = 0;
        if DAT_006a885c != 0 {
            FUN_0048f4b0(DAT_006a885c as u32);
        }
        DAT_006a885c = 0;
        if DAT_006a886a != 0 {
            FUN_0048f4b0(DAT_006a886a as u32);
        }
        DAT_006a886a = 0;
        if DAT_006a8878 != 0 {
            FUN_0048f4b0(DAT_006a8878 as u32);
        }
        DAT_006a8878 = 0;
        if DAT_006a8882 != 0 {
            FUN_0048f4b0(DAT_006a8882 as u32);
        }
        DAT_006a8882 = 0;
        if DAT_006a888c != 0 {
            FUN_0048f4b0(DAT_006a888c as u32);
        }
        DAT_006a888c = 0;
        if DAT_006a8892 != 0 {
            FUN_0048f4b0(DAT_006a8892 as u32);
        }
        DAT_006a8892 = 0;
        if DAT_006a8844 != 0 {
            FUN_0048f4b0(DAT_006a8844 as u32);
        }
        DAT_006a8844 = 0;
        if DAT_006a8898 != 0 {
            FUN_0048f4b0(DAT_006a8898 as u32);
        }
        DAT_006a8898 = 0;
    }
}
