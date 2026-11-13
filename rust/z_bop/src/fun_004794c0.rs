// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0060f1d4: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00479510(param_1: *mut f32, param_2: i32, param_3: i32, param_4: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004794c0(param_1: i32, param_2: i32, param_3: i32) -> *mut f32 {
    // SAFETY: param_1 points at the game's object; the callee and the global
    // are the game's own; reads and the call happen in the original's order.
    unsafe {
        let idx = ((param_1.wrapping_add(0x10)) as *const i32).read_unaligned();
        let p_var1 = param_2.wrapping_add(idx.wrapping_mul(0x40)) as *mut f32;
        let a = ((param_1.wrapping_add(0x14)) as *const i32).read_unaligned();
        let b = ((param_1.wrapping_add(0x10)) as *const i32).read_unaligned();
        FUN_00479510(p_var1, a, b, param_3);
        let idx2 = ((param_1.wrapping_add(0x10)) as *const i32).read_unaligned();
        let count = idx2
            .wrapping_mul(9)
            .wrapping_add(DAT_0060f1d4.wrapping_mul(3));
        p_var1.wrapping_offset(count as isize)
    }
}
