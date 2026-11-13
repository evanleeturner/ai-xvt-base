// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0042aba0(param_1: i32, param_2: i32, param_3: *mut f32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042acd0(param_1: i32, param_2: *mut f32) {
    // SAFETY: param_1 is the address of the game's own structure; the count
    // and the array base are read from it at the same offsets as the original,
    // and each element is passed on to the game's function.
    unsafe {
        let count: i32 = ((param_1 as usize).wrapping_add(6) as *const i32).read_unaligned();
        if count > 0 {
            let mut i_var1: i32 = 0;
            let mut i_var2: i32 = 0;
            loop {
                i_var2 = i_var2.wrapping_add(4);
                i_var1 = i_var1.wrapping_add(1);
                let base_addr: i32 =
                    ((param_1 as usize).wrapping_add(10) as *const i32).read_unaligned();
                let elem: i32 = ((base_addr as usize)
                    .wrapping_sub(4)
                    .wrapping_add(i_var2 as usize) as *const i32)
                    .read_unaligned();
                FUN_0042aba0(elem, param_1, param_2);
                if !(i_var1 < count) {
                    break;
                }
            }
        }
    }
}
