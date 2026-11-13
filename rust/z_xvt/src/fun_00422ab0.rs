// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00478470() -> u16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00422ab0(param_1: u16) -> i16 {
    if param_1 == 0 {
        return 0;
    }
    // SAFETY: the callee is the game's own function; param_1 is nonzero so
    // the division cannot fault.
    unsafe {
        let u_var1 = FUN_00478470();
        let q = u_var1 / param_1;
        (u32::from(u_var1).wrapping_sub(u32::from(q).wrapping_mul(u32::from(param_1))) as u16)
            as i16
    }
}
