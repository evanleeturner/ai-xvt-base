// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7fd3: u32;
    static mut DAT_00aa7fd7: u32;
    static mut DAT_00aa7fdf: u32;
    static mut DAT_00aa7ff7: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn mciSendCommandA(param_1: u32, param_2: u32, param_3: u32, param_4: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d3140() -> u32 {
    let mut local_4: [u8; 4] = [0; 4];
    // SAFETY: the globals and mciSendCommandA belong to the game and the
    // system; local_4 outlives the call that receives its address.
    unsafe {
        if DAT_00aa7fd3 == 0 {
            return 0;
        }
        if DAT_00aa7fd7 == 0 {
            return 0;
        }
        mciSendCommandA(DAT_00aa7fd3, 0x808, 0, local_4.as_mut_ptr() as usize as u32);
        DAT_00aa7fd7 = 0;
        DAT_00aa7fdf = 0;
        DAT_00aa7ff7 = 0;
        1
    }
}
