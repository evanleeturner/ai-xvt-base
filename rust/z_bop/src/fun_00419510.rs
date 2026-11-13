// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00523420: i32;
    static mut DAT_0052341c: i32;
    static mut DAT_00550b98: i32;
    static mut DAT_00550bd8: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0046e040() -> i32;
    fn FUN_004197b0();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00419510(param_1: i32) {
    // SAFETY: the callees and globals are the game's own; param_1 is an
    // address of a caller-owned structure, read where the original does.
    unsafe {
        let i_var2 = FUN_0046e040();
        if i_var2 == 0 && ((param_1.wrapping_add(4)) as *const i32).read_unaligned() == DAT_00523420
        {
            let mut pi_var4 = param_1.wrapping_add(8) as *const i32;
            let mut b_var1: i32 = 0;
            let mut pi_var3 = &raw mut DAT_00550b98 as *const i32;
            let end = &raw mut DAT_00550bd8 as *const i32;
            loop {
                if pi_var4.read_unaligned() != pi_var3.read_unaligned() {
                    b_var1 = 1;
                }
                pi_var3 = pi_var3.wrapping_add(1);
                pi_var4 = pi_var4.wrapping_add(1);
                if !(pi_var3 < end) {
                    break;
                }
            }
            if b_var1 == 0 {
                DAT_0052341c = 0;
                FUN_004197b0();
            }
        }
    }
}
