// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00449140(param_1: i32) -> i32 {
    if param_1 == 0 {
        return 0;
    }
    // SAFETY: param_1 is a pointer-sized integer addressing the game's own
    // node structure; fields are read at the offsets the original uses.
    unsafe {
        if ((param_1.wrapping_add(4)) as *const i32).read_unaligned() != 3 {
            let mut i_var2: i32 = 0;
            let mut i_var3: i32 = 0;
            if 0 < ((param_1.wrapping_add(8)) as *const i32).read_unaligned() {
                loop {
                    let base = ((param_1.wrapping_add(0xc)) as *const i32).read_unaligned();
                    let mut i_var1 = (base.wrapping_add(i_var2) as *const i32).read_unaligned();
                    if i_var1 != 0 {
                        i_var1 = FUN_00449140(i_var1);
                        if i_var1 != 0 {
                            return i_var1;
                        }
                    }
                    i_var2 = i_var2.wrapping_add(4);
                    i_var3 = i_var3.wrapping_add(1);
                    if !(i_var3 < ((param_1.wrapping_add(8)) as *const i32).read_unaligned()) {
                        break;
                    }
                }
            }
            return 0;
        }
    }
    param_1
}
