// SPDX-License-Identifier: MIT

unsafe extern "C" {
    fn FUN_004ae1a0(param_1: i32, param_2: i32) -> *mut i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004afa30(param_1: i32) -> i32 {
    let mut i_var3: i32 = 0;
    let mut i_var4: i32 = 0;
    // SAFETY: param_1 is the address of the game's own structure; fields are
    // read at the same offsets and order as the original, and the callee is
    // the game's own.
    unsafe {
        let mut count: i32 = ((param_1 as usize + 6) as *const i32).read_unaligned();
        if 0 < count {
            let p_array: i32 = ((param_1 as usize + 10) as *const i32).read_unaligned();
            let mut i_var5: i32 = 0;
            loop {
                let i_var1: i32 =
                    (p_array.wrapping_add(i_var5) as usize as *const i32).read_unaligned();
                if ((i_var1.wrapping_add(4)) as usize as *const i32).read_unaligned() != 0x14 {
                    let pi_var2 = FUN_004ae1a0(i_var1, param_1);
                    if !pi_var2.is_null() && pi_var2.read_unaligned() == 7 {
                        break;
                    }
                    i_var3 = i_var3.wrapping_add(1);
                }
                i_var5 = i_var5.wrapping_add(4);
                i_var4 = i_var4.wrapping_add(1);
                count = ((param_1 as usize + 6) as *const i32).read_unaligned();
                if !(i_var4 < count) {
                    break;
                }
            }
        }
        count = ((param_1 as usize + 6) as *const i32).read_unaligned();
        if count <= i_var4 {
            i_var3 = -1;
        }
    }
    i_var3
}
