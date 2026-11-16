// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7201: *mut i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004b7a70(param_1: *mut i8) -> i32;
    fn FUN_004b4d90(param_1: *mut i32, param_2: i32, param_3: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b4d40(param_1: *mut i8, param_2: i32, param_3: i32) -> u32 {
    // SAFETY: the callees and the global are the game's own; the table pointer
    // is read at the same scaled offset the original computes, and the entry
    // it yields is read where the original reads it.
    unsafe {
        let i_var2 = FUN_004b7a70(param_1);
        if i_var2 == -1 {
            return 0;
        }
        let idx = (0x40i32).wrapping_add(i_var2.wrapping_mul(0x44));
        let slot = DAT_00aa7201.wrapping_offset(idx as isize);
        let pi_var1 = (slot as *const *mut i32).read_unaligned();
        if pi_var1.wrapping_add(2).read_unaligned() != 0 {
            return 0;
        }
        FUN_004b4d90(pi_var1, param_2, param_3)
    }
}
