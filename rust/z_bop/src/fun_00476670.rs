// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004765b0(param_1: i32, param_2: i32, param_3: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00476670(param_1: i32, param_2: i32, param_3: i32) -> i32 {
    // SAFETY: the pointers are the game's own structures, read at the
    // offsets the original reads; the callee is the game's function.
    unsafe {
        let v6: i32 = ((param_2 as u32).wrapping_add(6) as *const i32).read_unaligned();
        let v10_2: i32 = ((param_2 as u32).wrapping_add(10) as *const i32).read_unaligned();
        let v10_1: i32 = ((param_1 as u32).wrapping_add(10) as *const i32).read_unaligned();

        let mut i_var2: i32 = 0;
        let mut i_var3: i32 = 0;
        if v6 > 0 {
            loop {
                let a = (v10_2.wrapping_add(i_var2) as u32 as *const i32).read_unaligned();
                let b = (v10_1.wrapping_add(i_var2) as u32 as *const i32).read_unaligned();
                let i_var1 = FUN_004765b0(a, b, param_3);
                if i_var1 != 0 {
                    return i_var1;
                }
                i_var2 = i_var2.wrapping_add(4);
                i_var3 = i_var3.wrapping_add(1);
                if i_var3 >= v6 {
                    break;
                }
            }
        }
        0
    }
}
