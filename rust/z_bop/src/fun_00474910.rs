// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00474830(param_1: *mut i8, param_2: i32, param_3: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00474910(param_1: *mut i8, param_2: i32, param_3: i32) -> i32 {
    // SAFETY: param_2 and the tables it points to are the game's own memory,
    // read where the original does; the callee is the game's.
    unsafe {
        let count: i32 = ((param_2 as u32).wrapping_add(6) as *const i32).read_unaligned();
        let base: i32 = ((param_2 as u32).wrapping_add(10) as *const i32).read_unaligned();

        let mut i_var2: i32 = 0;
        let mut i_var3: i32 = 0;
        if 0 < count {
            loop {
                let i_var1: i32 =
                    ((base.wrapping_add(i_var3)) as u32 as *const i32).read_unaligned();
                if param_3 == i_var1 {
                    return 0;
                }
                let r = FUN_00474830(param_1, i_var1, param_3);
                if r == 0 {
                    return r;
                }
                i_var3 = i_var3.wrapping_add(4);
                i_var2 = i_var2.wrapping_add(1);
                if !(i_var2 < count) {
                    break;
                }
            }
        }
        0
    }
}
