// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00525520: i32;
    static mut DAT_0052551c: i32;
    static mut DAT_006d4a4e: u8;
    static mut DAT_00525528: i32;
    static mut DAT_00525524: i32;
    static mut DAT_005257e0: i32;
    static mut DAT_0071e524: u8;
    static mut DAT_0052ae54: i32;
    static mut DAT_00703c80: unsafe extern "C" fn();
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048ee20(param_1: i8);
    fn FUN_0048ed70(param_1: u32);
    fn FUN_0048eda0(param_1: u32);
    fn FUN_0048edd0(param_1: u32);
    fn FUN_004a7040();
    fn FUN_004a6a80();
    fn FUN_0048ed00(param_1: i16, param_2: i16, param_3: i16, param_4: i16);
    fn FUN_0048ece0(param_1: i16, param_2: i16);
    fn FUN_0048f270(param_2: *mut u8);
    fn FUN_004a6f90();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00438c30(param_1: i32, param_2: *mut u8, param_3: u8) {
    let mut param_1 = param_1;
    // SAFETY: the callees and globals are the game's own memory; calls and
    // accesses happen in the original's order.
    unsafe {
        FUN_0048ee20(0);
        let u_var2: u32 = (DAT_00525520.wrapping_sub(DAT_0052551c) as u32) >> 1;
        let s_var3: i16 = (((DAT_0052551c.wrapping_add(DAT_00525520) as u32) >> 1) as i16)
            .wrapping_sub((u_var2 / 2) as i16);
        let u_var1: u32 = DAT_006d4a4e as u32;
        let s_var4: i16 = (((DAT_00525528.wrapping_add(DAT_00525524) as u32) >> 1) as i16)
            .wrapping_sub((u_var1.wrapping_mul(5) / 2) as i16);
        if DAT_005257e0 != 0 {
            FUN_0048ed70(0x2f);
            FUN_0048eda0((param_3 as u32).wrapping_add(2));
            DAT_0071e524 = 1;
            FUN_0048edd0(0x2c);
            FUN_004a7040();
            DAT_0052ae54 = 0;
            FUN_004a6a80();
            let local_8: i16 = u_var1.wrapping_mul(5) as i16;
            let local_4: i16 = u_var2 as i16;
            if param_1 == 1 {
                FUN_0048ed00(
                    s_var3.wrapping_sub(1),
                    s_var4.wrapping_sub(1),
                    local_4.wrapping_add(s_var3).wrapping_add(1),
                    local_8.wrapping_add(s_var4).wrapping_add(1),
                );
                (DAT_00703c80)();
                param_1 = 0;
            }
            FUN_0048eda0(param_3 as u32);
            let t = (DAT_006d4a4e as i32).wrapping_mul(param_1 as i16 as i32);
            FUN_0048ed00(
                s_var3,
                (t as i16).wrapping_add(s_var4),
                local_4.wrapping_add(s_var3),
                local_8.wrapping_add(s_var4),
            );
            (DAT_00703c80)();
            if param_1 == 0 {
                param_1 = 1;
            }
            let a = (DAT_006d4a4e as i32).wrapping_add(s_var3 as i32) as i16;
            let b = (param_1 as i16 as i32)
                .wrapping_mul(DAT_006d4a4e as i32)
                .wrapping_add(s_var4 as i32) as i16;
            FUN_0048ece0(a, b);
            FUN_0048f270(param_2);
            FUN_004a6f90();
            DAT_0052ae54 = 1;
            FUN_004a7040();
        }
    }
}
