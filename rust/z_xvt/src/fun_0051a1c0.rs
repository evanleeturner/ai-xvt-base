// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00536700: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn timeGetTime() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0051a1c0() {
    // SAFETY: timeGetTime is the Windows multimedia API; the global is the
    // game's own memory.
    unsafe {
        DAT_00536700 = timeGetTime();
    }
}
