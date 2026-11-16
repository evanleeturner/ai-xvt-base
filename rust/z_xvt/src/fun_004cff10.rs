// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00759b08: i32;
    static mut DAT_00759b0c: i32;
    static mut DAT_00759b14: i32;
    static mut DAT_00759b18: i32;
    static mut DAT_00759b20: i32;
    static mut DAT_00759b24: i32;
    static mut DAT_00759b2c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cfe80(param_1: i32) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetTickCount() -> u32;
    fn mciSendCommandA(mci_id: u32, u_msg: u32, dw_param1: u32, dw_param2: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004cff10() -> i32 {
    let mut local_4: [u8; 4] = [0; 4];
    // SAFETY: the globals and callees are the game's own; local_4 lives for
    // the duration of the MCI call that receives its address.
    unsafe {
        let mut i_var2 = DAT_00759b18;
        if DAT_00759b08 == 0 {
            return 0;
        }
        if DAT_00759b2c == 0 {
            if DAT_00759b0c != 0 && DAT_00759b14 == 0 {
                let d_var1 = GetTickCount();
                DAT_00759b20 = i_var2.wrapping_sub(d_var1 as i32).wrapping_sub(2000);
                i_var2 = FUN_004cfe80(DAT_00759b0c);
                DAT_00759b24 = i_var2.wrapping_sub(DAT_00759b20);
                mciSendCommandA(DAT_00759b08 as u32, 0x808, 0, local_4.as_mut_ptr() as u32);
                DAT_00759b2c = 1;
                return 1;
            }
        } else if DAT_00759b2c == 2 {
            DAT_00759b2c = 1;
        }
        1
    }
}
