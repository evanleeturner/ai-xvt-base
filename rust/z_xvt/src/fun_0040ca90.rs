// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732e44: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00413f20(param_1: u32, param_2: u32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0040ca90(param_1: u32, param_2: u32) -> i32 {
    // SAFETY: the callee and the global are the game's own; the arguments
    // are passed through as the original does.
    unsafe {
        let i_var1: i32;
        if (param_1 as i32) < 0 {
            let r = FUN_00413f20(0u32.wrapping_sub(param_1), param_2);
            i_var1 = 0u32.wrapping_sub(r as u32) as i32;
        } else {
            i_var1 = FUN_00413f20(param_1, param_2);
        }
        (i_var1 as u32).wrapping_add(u32::from(DAT_00732e44)) as i32
    }
}
