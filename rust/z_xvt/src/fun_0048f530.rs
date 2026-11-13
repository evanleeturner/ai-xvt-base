// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052a564: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn timeGetTime() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0048f530() {
    // SAFETY: timeGetTime and the global are the game's own; the global is
    // read and written where the original does.
    unsafe {
        let d_var1 = timeGetTime();
        if DAT_0052a564 == 0 {
            DAT_0052a564 = d_var1;
        }
        DAT_0052a564 = DAT_0052a564.wrapping_add(d_var1.wrapping_sub(DAT_0052a564) & 0xfffffffc);
    }
}
