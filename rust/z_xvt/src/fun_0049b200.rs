// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006db730: i32;
    static mut DAT_0071f720: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0049b200(param_1: i32) -> *mut i32 {
    // SAFETY: the globals are the game's own tables; the indices and the
    // stride walk match the original's pointer arithmetic.
    unsafe {
        let mut i_var2: i32 = (&raw mut DAT_006db730)
            .wrapping_offset(param_1 as isize)
            .read();
        let mut pi_var3: *mut i32 =
            (&raw mut DAT_0071f720).wrapping_offset(param_1.wrapping_mul(0x7e9) as isize);
        let mut pi_var1: *mut i32 = core::ptr::null_mut();
        if 0 < i_var2 {
            loop {
                if pi_var3.read_unaligned() != 0 {
                    pi_var1 = pi_var3;
                }
                pi_var3 = pi_var3.byte_add(0x12);
                i_var2 = i_var2.wrapping_sub(1);
                if i_var2 == 0 {
                    break;
                }
            }
        }
        pi_var1
    }
}
