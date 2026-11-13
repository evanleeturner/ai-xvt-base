// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071ed00: u16;
    static mut DAT_006d4a42: u16;
    static mut DAT_00709ffc: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00413f20(param_1: u32, param_2: u32) -> i32;
    fn FUN_00478350(param_1: u32, param_2: u32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0040cad0(param_1: u32, param_2: u32) -> i32 {
    // SAFETY: the callees and globals are the game's own memory and code,
    // read and called in the original's order.
    unsafe {
        let mut u_var2: u32;
        if (param_1 as i32) < 0 {
            let i_var1 = FUN_00413f20(0u32.wrapping_sub(param_1), param_2);
            u_var2 = 0u32.wrapping_sub(i_var1 as u32);
        } else {
            u_var2 = FUN_00413f20(param_1, param_2) as u32;
        }
        if DAT_0071ed00 != 0 {
            if (u_var2 as i32) < 0 {
                u_var2 = FUN_00478350(0u32.wrapping_sub(u_var2), DAT_0071ed00 as u32) as u32;
                u_var2 = 0u32.wrapping_sub(u_var2);
            } else {
                u_var2 = FUN_00478350(u_var2, DAT_0071ed00 as u32) as u32;
            }
        }
        u_var2
            .wrapping_add(DAT_006d4a42 as u32)
            .wrapping_add(DAT_00709ffc) as i32
    }
}
