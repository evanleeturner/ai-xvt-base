// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006db7ac: i32;
    static mut DAT_006db7b0: i32;
    static mut DAT_006db7b4: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00478290(param_1: u32, param_2: i32, param_3: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0040c970(param_1: i32, param_2: i32, param_3: i32) {
    // SAFETY: the globals and the callee are the game's own memory and code;
    // they are read, written and called in the original's order.
    unsafe {
        let u_var2: u32 = 0u32.wrapping_sub(DAT_006db7b4 as u32);
        if DAT_006db7ac < param_1 {
            let i_var1 = FUN_00478290(
                u_var2,
                (param_1 as u32).wrapping_sub(DAT_006db7ac as u32) as i32,
                (param_3 as u32).wrapping_add(u_var2) as i32,
            );
            DAT_006db7ac = (DAT_006db7ac as u32).wrapping_add(i_var1 as u32) as i32;
        } else {
            let i_var1 = FUN_00478290(
                u_var2,
                (DAT_006db7ac as u32).wrapping_sub(param_1 as u32) as i32,
                (param_3 as u32).wrapping_add(u_var2) as i32,
            );
            DAT_006db7ac = (DAT_006db7ac as u32).wrapping_sub(i_var1 as u32) as i32;
        }
        if DAT_006db7b0 < param_2 {
            let i_var1 = FUN_00478290(
                u_var2,
                (param_2 as u32).wrapping_sub(DAT_006db7b0 as u32) as i32,
                (param_3 as u32).wrapping_add(u_var2) as i32,
            );
            DAT_006db7b0 = (DAT_006db7b0 as u32).wrapping_add(i_var1 as u32) as i32;
        } else {
            let i_var1 = FUN_00478290(
                u_var2,
                (DAT_006db7b0 as u32).wrapping_sub(param_2 as u32) as i32,
                (param_3 as u32).wrapping_add(u_var2) as i32,
            );
            DAT_006db7b0 = (DAT_006db7b0 as u32).wrapping_sub(i_var1 as u32) as i32;
        }
        DAT_006db7b4 = 1;
    }
}
