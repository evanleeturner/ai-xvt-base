// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00525560: i32;
    static mut DAT_00645a90: i32;
    static mut DAT_00645ad0: i32;
    static mut DAT_0052555c: i32;
    static mut DAT_0052acc8: i32;
    static mut DAT_0052acc4: i32;
    static mut DAT_0052accc: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00431d30() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0049bf30(param_1: i32) {
    // SAFETY: the callee and the globals are the game's own memory; param_1
    // is an address of the caller's data, read where the original reads.
    unsafe {
        let i_var2 = FUN_00431d30();
        if i_var2 == 0 && ((param_1.wrapping_add(4)) as *const i32).read_unaligned() == DAT_00525560
        {
            let mut pi_var4 = param_1.wrapping_add(8) as *const i32;
            let mut b_var1: i32 = 0;
            let mut pi_var3 = &raw mut DAT_00645a90 as *const i32;
            let end = &raw mut DAT_00645ad0 as *const i32;
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
                DAT_0052555c = 0;
                DAT_0052acc8 = DAT_0052acc4;
                DAT_0052accc = 0;
            }
        }
    }
}
