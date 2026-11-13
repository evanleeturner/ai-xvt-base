// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006a8826: u32;
    static mut DAT_006a8824: u16;
    static mut DAT_006a8838: u32;
    static mut DAT_006a8836: u16;
    static mut DAT_006a8854: u32;
    static mut DAT_006a884e: u16;
    static mut DAT_006a8862: u32;
    static mut DAT_006a885c: u16;
    static mut DAT_006a8870: u32;
    static mut DAT_006a886a: u16;
    static mut DAT_006a8878: u16;
    static mut DAT_006a887a: u32;
    static mut DAT_006a8882: u16;
    static mut DAT_006a8884: u32;
    static mut DAT_006a888c: u16;
    static mut DAT_006a8892: u16;
    static mut DAT_006a8898: u16;
    static mut DAT_006a889a: u32;
    static mut DAT_006a8844: u16;
    static mut DAT_005f1d3c: u32;
    static mut DAT_005f1d68: u32;
    static mut DAT_005f262c: u32;
    static mut DAT_005f1d34: u32;
    static mut DAT_005f2600: u32;
    static mut DAT_0052a390: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048f3c0(param_1: u32, param_2: u32) -> u16;
    fn FUN_0047f010(param_1: u16);
    fn FUN_004a7340(param_1: *mut core::ffi::c_void, param_2: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00465740() {
    // SAFETY: the globals and callees are the game's own memory and code;
    // accesses happen in the same order and widths as the original.
    unsafe {
        DAT_006a8826 = 20000;
        DAT_006a8824 = FUN_0048f3c0(480000, 0);
        if DAT_006a8824 == 0 {
            FUN_0047f010(0);
        }
        DAT_006a8838 = 20000;
        DAT_006a8836 = FUN_0048f3c0(80000, 0);
        if DAT_006a8836 == 0 {
            FUN_0047f010(0);
        }
        DAT_006a8854 = 5000;
        DAT_006a884e = FUN_0048f3c0(560000, 0);
        if DAT_006a884e == 0 {
            FUN_0047f010(0);
        }
        DAT_006a8862 = DAT_006a887a.wrapping_mul(2);
        DAT_006a885c = FUN_0048f3c0(DAT_006a887a.wrapping_mul(0x30), 0);
        if DAT_006a885c == 0 {
            FUN_0047f010(0);
        }
        DAT_006a8870 = DAT_006a8884.wrapping_mul(2);
        DAT_006a886a = FUN_0048f3c0(DAT_006a8884.wrapping_mul(0x38), 0);
        if DAT_006a886a == 0 {
            FUN_0047f010(0);
        }
        DAT_006a8878 = FUN_0048f3c0(DAT_006a887a.wrapping_shl(2), 0);
        if DAT_006a8878 == 0 {
            FUN_0047f010(0);
        }
        DAT_006a8882 = FUN_0048f3c0(DAT_006a8884.wrapping_shl(2), 0);
        if DAT_006a8882 == 0 {
            FUN_0047f010(0);
        }
        DAT_006a888c = FUN_0048f3c0(0xc00, 0);
        if DAT_006a888c == 0 {
            FUN_0047f010(0);
        }
        DAT_006a8892 = FUN_0048f3c0(0xc00, 0);
        if DAT_006a8892 == 0 {
            FUN_0047f010(0);
        }
        DAT_005f1d3c = 0x10;
        DAT_005f1d68 = DAT_0052a390;
        DAT_005f262c = 4;
        DAT_005f1d34 = 0xf;
        DAT_005f2600 = 0x41800000;
        DAT_006a8844 = FUN_0048f3c0(0x25800, 0);
        if DAT_006a8844 == 0 {
            FUN_0047f010(0);
        }
        DAT_006a889a = 500;
        DAT_006a8898 = FUN_0048f3c0(0x19640, 0);
        if DAT_006a8898 == 0 {
            FUN_0047f010(0);
        }
        FUN_004a7340(0x465957 as *mut core::ffi::c_void, 0x80000);
    }
}
