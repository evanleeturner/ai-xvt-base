// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00557364: i32;
    static mut DAT_00523420: i32;
    static mut DAT_00557368: i32;
    static mut DAT_009a8c2c: i32;
    static mut _DAT_00557360: i32;
    static mut DAT_0055736c: i32;
    static mut DAT_005242e0: i32;
    static mut DAT_00556f00: i32;
    static mut DAT_00a07cc8: i32;
    static mut _DAT_009ed23c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0046d350(param_1: i32, param_2: *mut u32, param_3: i32);
    fn FUN_004ac760() -> i32;
    fn FUN_004aa7f0() -> i32;
    fn FUN_004aa870() -> u32;
    fn FUN_00463d00();
    fn FUN_0046d300(param_1: *mut u32, param_2: i32);
    fn FUN_0046f660(param_1: i32) -> i32;
    fn FUN_00463c50(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004651f0(param_1: i32, param_2: u32) {
    // SAFETY: the globals and callees are the game's own memory and code;
    // pointers passed are addresses of the game's globals.
    unsafe {
        DAT_00557364 = DAT_00523420;
        DAT_00557368 = param_2 as i32;
        let mut i_var5: i32 = 10;
        let mut i_var4: i32 = 0;
        _DAT_00557360 = 0x3e;
        DAT_0055736c = DAT_009a8c2c;
        FUN_0046d350(param_1, (&raw mut _DAT_00557360).cast::<u32>(), 0x10);
        FUN_004ac760();
        loop {
            let i_var1 = DAT_009a8c2c;
            DAT_005242e0 = 1;
            loop {
                if 0xeb < (DAT_009a8c2c.wrapping_sub(i_var1) as u32) {
                    break;
                }
                let i_var2 = FUN_004aa7f0();
                if i_var2 != 0 {
                    let u_var3 = FUN_004aa870();
                    if u_var3 == 0x1b {
                        i_var5 = 1;
                        DAT_009a8c2c = DAT_009a8c2c.wrapping_add(0xec);
                        break;
                    }
                }
                FUN_00463d00();
                let i_var2 = FUN_004ac760();
                DAT_009a8c2c = DAT_009a8c2c.wrapping_add(i_var2);
                if DAT_00556f00 != 0 {
                    DAT_00556f00 = 0;
                    DAT_009a8c2c = i_var1;
                }
                if DAT_005242e0 == 0 {
                    break;
                }
            }
            i_var4 = i_var4.wrapping_sub(i_var1).wrapping_add(DAT_009a8c2c);
            if 0xeb < i_var4 {
                i_var4 = 0;
                _DAT_00557360 = 0x1d;
                FUN_0046d300((&raw mut _DAT_00557360).cast::<u32>(), 4);
            }
            let done = if DAT_005242e0 == 0 {
                true
            } else {
                i_var5 = i_var5.wrapping_sub(1);
                i_var5 == 0
            };
            if done {
                if DAT_005242e0 == 1 {
                    i_var4 = FUN_0046f660(param_1);
                    FUN_00463c50(i_var4);
                    FUN_004ac760();
                    DAT_005242e0 = 0;
                    DAT_009a8c2c = DAT_00a07cc8.wrapping_add(_DAT_009ed23c);
                } else {
                    FUN_004ac760();
                    DAT_009a8c2c = DAT_00a07cc8.wrapping_add(_DAT_009ed23c);
                }
                _DAT_00557360 = 0x17;
                DAT_00557364 = 0;
                FUN_0046d300((&raw mut _DAT_00557360).cast::<u32>(), 8);
                return;
            }
        }
    }
}
