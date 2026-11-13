// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00449190(param_1: i32) -> i32 {
    if param_1 == 0 {
        return 0;
    }
    // SAFETY: param_1 is a pointer to the game's node structure; fields are
    // read at the same offsets as the original.
    unsafe {
        if (param_1.wrapping_add(4) as *const i32).read_unaligned() != 0x17 {
            let mut i_var3: i32 = 0;
            let mut i_var2: i32 = 0;
            if 0 < (param_1.wrapping_add(8) as *const i32).read_unaligned() {
                loop {
                    let arr = (param_1.wrapping_add(0xc) as *const i32).read_unaligned();
                    let child = (i_var3.wrapping_add(arr) as *const i32).read_unaligned();
                    if child != 0 {
                        let arr2 = (param_1.wrapping_add(0xc) as *const i32).read_unaligned();
                        let i_var1 = FUN_00449190(
                            (arr2.wrapping_add(i_var3) as *const i32).read_unaligned(),
                        );
                        if i_var1 != 0 {
                            return i_var1;
                        }
                    }
                    i_var3 = i_var3.wrapping_add(4);
                    i_var2 = i_var2.wrapping_add(1);
                    if i_var2 >= (param_1.wrapping_add(8) as *const i32).read_unaligned() {
                        break;
                    }
                }
            }
            return 0;
        }
    }
    param_1
}
