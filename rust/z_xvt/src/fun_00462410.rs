// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00525518: i32;
    static mut DAT_00732f44: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00462010(param_1: i32, param_2: i32, param_3: i32, param_4: i32);
    fn FUN_004a68c0(param_1: i32, param_2: i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00462410() {
    let mut local_300 = [0u8; 768];
    // SAFETY: the callees and globals are the game's own; local_300 is a
    // live stack buffer whose 32-bit address is passed as the C does.
    unsafe {
        FUN_00462010(0x6da540, local_300.as_mut_ptr() as usize as i32, 0, 0x100);
        if DAT_00525518 == 1 {
            FUN_004a68c0(local_300.as_mut_ptr() as usize as i32, 0, 0x100);
        }
        DAT_00732f44 = DAT_00732f44 & 0xfe;
    }
}
