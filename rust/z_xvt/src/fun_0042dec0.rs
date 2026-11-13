// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005373d4: i32;
    static mut DAT_00525560: i32;
    static mut DAT_005373d8: i32;
    static mut DAT_005373d0: i32;
    static mut DAT_005373dc: i32;
    static mut DAT_006db5a8: i32;
    static mut DAT_005250d8: i32;
    static mut DAT_00536f70: i32;
    static mut DAT_00732a18: i32;
    static mut DAT_0071f2ec: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00431110(param_1: i32, param_2: *mut u32, param_3: u32);
    fn FUN_0048f530() -> i32;
    fn FUN_0042c710();
    fn FUN_004310c0(param_1: *mut u32, param_2: u32);
    fn FUN_00432c10(param_1: i32) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004a53f0() -> i32;
    fn FUN_004a5470() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042dec0(param_1: i32, param_2: u32) {
    // SAFETY: the globals and callees are the game's own memory and code;
    // the pointers passed are addresses of the game's globals.
    unsafe {
        DAT_005373d4 = DAT_00525560;
        DAT_005373d8 = param_2 as i32;
        let mut i_var5: i32 = 10;
        let mut i_var4: i32 = 0;
        DAT_005373d0 = 0x3e;
        DAT_005373dc = DAT_006db5a8;
        FUN_00431110(param_1, &raw mut DAT_005373d0 as *mut u32, 0x10);
        FUN_0048f530();
        loop {
            let i_var1 = DAT_006db5a8;
            DAT_005250d8 = 1;
            loop {
                if (DAT_006db5a8.wrapping_sub(i_var1) as u32) > 0xeb {
                    break;
                }
                let i_var2 = FUN_004a53f0();
                if i_var2 != 0 {
                    let u_var3 = FUN_004a5470();
                    if u_var3 == 0x1b {
                        i_var5 = 1;
                        DAT_006db5a8 = DAT_006db5a8.wrapping_add(0xec);
                        break;
                    }
                }
                FUN_0042c710();
                let i_var2 = FUN_0048f530();
                DAT_006db5a8 = DAT_006db5a8.wrapping_add(i_var2);
                if DAT_00536f70 != 0 {
                    DAT_00536f70 = 0;
                    DAT_006db5a8 = i_var1;
                }
                if DAT_005250d8 == 0 {
                    break;
                }
            }
            i_var4 = i_var4.wrapping_sub(i_var1).wrapping_add(DAT_006db5a8);
            if 0xeb < i_var4 {
                i_var4 = 0;
                DAT_005373d0 = 0x1d;
                FUN_004310c0(&raw mut DAT_005373d0 as *mut u32, 4);
            }
            let done = if DAT_005250d8 == 0 {
                true
            } else {
                i_var5 = i_var5.wrapping_sub(1);
                i_var5 == 0
            };
            if done {
                if DAT_005250d8 == 1 {
                    DAT_005373d4 = FUN_00432c10(param_1);
                    DAT_005373d0 = 0x15;
                    FUN_004310c0(&raw mut DAT_005373d0 as *mut u32, 8);
                    FUN_0048f530();
                    DAT_005250d8 = 0;
                    DAT_006db5a8 = DAT_00732a18.wrapping_add(DAT_0071f2ec);
                } else {
                    FUN_0048f530();
                    DAT_006db5a8 = DAT_00732a18.wrapping_add(DAT_0071f2ec);
                }
                DAT_005373d0 = 0x17;
                DAT_005373d4 = 0;
                FUN_004310c0(&raw mut DAT_005373d0 as *mut u32, 8);
                return;
            }
        }
    }
}
