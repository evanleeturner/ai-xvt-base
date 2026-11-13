// SPDX-License-Identifier: MIT

unsafe extern "C" {
    fn FUN_00488a90(param_1: *mut i8, param_2: i32, param_3: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00488b70(param_1: *mut i8, param_2: i32, param_3: i32) -> i32 {
    let mut i_var3: i32 = 0;
    let mut i_var2: i32 = 0;
    // SAFETY: param_2 and the structures it links to are the game's own
    // memory, read at the original's byte offsets; the callee is the game's.
    unsafe {
        if 0 < (param_2.wrapping_add(6) as *const i32).read_unaligned() {
            loop {
                let base = (param_2.wrapping_add(10) as *const i32).read_unaligned();
                let mut i_var1 = (base.wrapping_add(i_var3) as *const i32).read_unaligned();
                if param_3 == i_var1 {
                    return 0;
                }
                i_var1 = FUN_00488a90(param_1, i_var1, param_3);
                if i_var1 == 0 {
                    return 0;
                }
                i_var3 = i_var3.wrapping_add(4);
                i_var2 = i_var2.wrapping_add(1);
                if i_var2 >= (param_2.wrapping_add(6) as *const i32).read_unaligned() {
                    break;
                }
            }
        }
    }
    0
}
