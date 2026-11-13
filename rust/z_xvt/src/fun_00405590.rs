// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071ecf0: u16;
    static mut DAT_0071eae4: u16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00405590() {
    // SAFETY: the globals are the game's own memory, read and written
    // where the original does.
    unsafe {
        let mut u_var1: u16 = DAT_0071ecf0;
        if 0x7fff < DAT_0071ecf0 {
            u_var1 = 0u32.wrapping_sub(DAT_0071ecf0 as u32) as u16;
        }
        if (u_var1 as i16) < 0x41 {
            DAT_0071ecf0 = 0;
        }

        u_var1 = DAT_0071eae4;
        if 0x7fff < DAT_0071eae4 {
            u_var1 = 0u32.wrapping_sub(DAT_0071eae4 as u32) as u16;
        }
        if (u_var1 as i16) < 0x19 {
            DAT_0071eae4 = 0;
        }
    }
}
