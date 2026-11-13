// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071eae4: u16;
    static mut DAT_0071ecf0: u16;
    static mut DAT_007092b0: u16;
    static mut DAT_006db592: u16;
    static mut DAT_0071e534: u16;
    static mut DAT_0071b6b0: u16;
    static mut DAT_006d9d88: u16;
    static mut DAT_006d9d92: u16;
    static mut DAT_0070acc6: u16;
    static mut DAT_00732f92: u16;
    static mut DAT_0073254c: u16;
    static mut DAT_00732554: u16;
    static mut DAT_00732f94: u16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004054e0() {
    // SAFETY: all globals are the game's own memory, read and written
    // in the same order as the original.
    unsafe {
        DAT_0071eae4 = 0;
        DAT_0071ecf0 = 0;
        DAT_007092b0 = DAT_006db592;
        DAT_0071e534 = 0;
        if DAT_0071b6b0 != 0 {
            DAT_0071ecf0 = ((DAT_006d9d88 as u32).wrapping_shl(7)) as u16;
            DAT_0071eae4 = ((DAT_006d9d92 as u32).wrapping_shl(6)) as u16;
            DAT_0071e534 = DAT_0070acc6;
        }
        if ((DAT_00732f92 as u8) | 1) != 0 {
            if DAT_0071ecf0 == 0 {
                DAT_0071ecf0 = ((DAT_0073254c as u32).wrapping_mul(0x78)) as u16;
            }
            if DAT_0071eae4 == 0 {
                DAT_0071eae4 = ((DAT_00732554 as u32).wrapping_mul(0x32)) as u16;
            }
            DAT_0071e534 = DAT_0071e534 | DAT_00732f94;
        }
    }
}
