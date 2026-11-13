// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00759b2c: u32;
    static mut DAT_00759b28: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetTickCount() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004cffc0() -> u32 {
    // SAFETY: the globals are the game's own memory and GetTickCount is the
    // Win32 API; both are used exactly where the original does.
    unsafe {
        if DAT_00759b2c == 1 {
            DAT_00759b2c = 2;
            let d_var1 = GetTickCount();
            DAT_00759b28 = d_var1.wrapping_add(1000);
        }
    }
    1
}
