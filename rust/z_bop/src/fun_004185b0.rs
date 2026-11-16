// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00523428: i32;
    static mut DAT_009a8db0: i32;
    static mut DAT_009ed670: i32;
    static mut DAT_009e9680: i8;
    static mut DAT_009ecc34: i32;
    static mut DAT_009ec468: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004186e0(param_1: i32, param_2: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004185b0() {
    // SAFETY: all globals and the callee are the game's own memory; the
    // pointers walk the game's tables exactly as the original does.
    unsafe {
        if DAT_00523428 == 0 {
            let mut i_var1: i32 = 0;
            let mut pi_var4: *mut i32 = &raw mut DAT_009a8db0;
            let mut pi_stack_4: *mut i32 = &raw mut DAT_009ed670;
            let mut pc_stack_8: *mut i8 = &raw mut DAT_009e9680;
            let end: *mut i8 = &raw mut DAT_009ec468;
            loop {
                if pc_stack_8.read() != 0 && i_var1 != DAT_009ecc34 {
                    let mut i_var3: i32 = 0;
                    let mut pi_var2: *mut i32 = pi_stack_4;
                    if 0 < pi_var4.read_unaligned() {
                        loop {
                            if pi_var2.read_unaligned() == 0
                                && pi_var2.byte_add(4).read_unaligned() == 2
                            {
                                FUN_004186e0(i_var1, pi_var2);
                            } else {
                                pi_var2 = pi_var2.byte_add(0x12);
                                i_var3 = i_var3.wrapping_add(1);
                            }
                            if !(i_var3 < pi_var4.read_unaligned()) {
                                break;
                            }
                        }
                    }
                }
                pi_var4 = pi_var4.wrapping_add(1);
                pi_stack_4 = pi_stack_4.wrapping_add(0x7e9);
                pc_stack_8 = pc_stack_8.wrapping_add(0x5bd);
                i_var1 = i_var1.wrapping_add(1);
                if !(pc_stack_8 < end) {
                    break;
                }
            }
        }
    }
}
