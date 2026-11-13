// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005233d8: i32;
    static mut DAT_00a081f4: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0040e1b0(param_1: i32, param_2: i32, param_3: i32, param_4: i32);
    fn FUN_004ab890(param_1: i32, param_2: i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0040e590() {
    let mut local_300: [i8; 768] = [0; 768];
    // SAFETY: the callees and globals are the game's own; local_300 is a
    // live stack buffer whose address is passed as a 32-bit integer.
    unsafe {
        FUN_0040e1b0(0x9a7bc0, local_300.as_mut_ptr() as usize as i32, 0, 0x100);
        if DAT_005233d8 != 1 {
            FUN_004ab890(local_300.as_mut_ptr() as usize as i32, 0, 0x100);
        }
        DAT_00a081f4 = (DAT_00a081f4 as u8 & 0xfe) as i8;
    }
}
