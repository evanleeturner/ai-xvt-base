// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009994fc: i32;
    static mut DAT_00999570: [u32; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0046d350(param_1: u32, param_2: *mut u32, param_3: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0046d300(param_1: *mut u32, param_2: u32) {
    // SAFETY: the globals and the callee are the game's own; the table
    // entries are 10 words apart and read as the original does.
    unsafe {
        let mut i_var1: i32 = 0;
        if DAT_009994fc > 0 {
            let mut pi_var2: *mut u32 = &raw mut DAT_00999570 as *mut u32;
            loop {
                let a = pi_var2.read_unaligned();
                let b = pi_var2.add(1).read_unaligned();
                if a != 0 && b != 0 {
                    FUN_0046d350(a, param_1, param_2);
                }
                pi_var2 = pi_var2.add(10);
                i_var1 = i_var1.wrapping_add(1);
                if !(i_var1 < DAT_009994fc) {
                    break;
                }
            }
        }
    }
}
