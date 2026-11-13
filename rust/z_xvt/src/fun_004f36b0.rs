// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00650440: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dd000(param_1: *mut i32, param_2: *mut u32);
    fn FUN_004f32e0(param_1: *mut i32, param_2: i16, param_3: i16) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004f36b0(in_stack_00000014: i16, in_stack_00000018: i16) -> u32 {
    let mut local_10: [i32; 4] = [0; 4];
    // SAFETY: the callees and the global are the game's own; local_10 is a
    // live local buffer passed by address as the original does.
    unsafe {
        FUN_004dd000(local_10.as_mut_ptr(), &raw mut DAT_00650440);
        let u_var1 = FUN_004f32e0(local_10.as_mut_ptr(), in_stack_00000014, in_stack_00000018);
        (u_var1 & 0xFFFF0000) | 1
    }
}
