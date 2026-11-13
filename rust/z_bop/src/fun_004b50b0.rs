// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7201: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004b7a70(param_1: *mut i8) -> i32;
    fn FUN_004b5100(param_1: *mut i32, param_2: *mut i32, param_3: i32, param_4: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b50b0(
    param_1: *mut i8,
    param_2: *mut i32,
    param_3: i32,
    param_4: i32,
) -> u32 {
    // SAFETY: the callees and the global are the game's own; the table entry
    // pointer is read where the original reads it and passed on unchanged.
    unsafe {
        let i_var2 = FUN_004b7a70(param_1);
        if i_var2 == -1 {
            return 0;
        }
        let addr = DAT_00aa7201
            .wrapping_add(0x40)
            .wrapping_add(i_var2.wrapping_mul(0x44));
        let pi_var1 = (addr as *const *mut i32).read_unaligned();
        if pi_var1.add(2).read_unaligned() != 0 {
            return 0;
        }
        FUN_004b5100(pi_var1, param_2, param_3, param_4)
    }
}
