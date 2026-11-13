// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_0065510c: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn timeGetTime() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0051c0c0() {
    // SAFETY: timeGetTime is the system's own call; the global is the game's
    // own memory, written once by value.
    unsafe {
        _DAT_0065510c = timeGetTime();
    }
}
