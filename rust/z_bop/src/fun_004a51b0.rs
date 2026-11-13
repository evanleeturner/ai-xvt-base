// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00622c4c: u32;
    static mut DAT_00622c48: u32;
    static mut _DAT_00622bc0: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn mciSendCommandA(param_1: u32, param_2: u32, param_3: u32, param_4: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004a51b0() -> u32 {
    let mut local_4: [u8; 4] = [0; 4];
    // SAFETY: the globals and the MCI call are the game's own; local_4 is a
    // live stack buffer whose address is passed as the command parameter.
    unsafe {
        if DAT_00622c4c == 0 {
            return 0;
        }
        if DAT_00622c48 == 0 {
            return 0;
        }
        mciSendCommandA(DAT_00622c4c, 0x808, 0, local_4.as_mut_ptr() as usize as u32);
        DAT_00622c48 = 0;
        _DAT_00622bc0 = 0;
        1
    }
}
