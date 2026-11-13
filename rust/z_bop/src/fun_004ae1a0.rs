// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FUN_004ae1a0(param_1: i32, param_2: u32) -> i32 {
    if param_1 == 0 {
        return 0;
    }
    // SAFETY: param_1 is an address of the game's own node structure; fields
    // are read at the same offsets and in the same order as the original.
    unsafe {
        if ((param_1.wrapping_add(4)) as *const i32).read_unaligned() != 0x19 {
            let mut i_var3: i32 = 0;
            let mut i_var2: i32 = 0;
            if 0 < ((param_1.wrapping_add(8)) as *const i32).read_unaligned() {
                loop {
                    let base = ((param_1.wrapping_add(0xc)) as *const i32).read_unaligned();
                    let i_var1 = (base.wrapping_add(i_var3) as *const i32).read_unaligned();
                    if i_var1 != 0 {
                        let r = FUN_004ae1a0(i_var1, param_2);
                        if r != 0 {
                            return r;
                        }
                    }
                    i_var3 = i_var3.wrapping_add(4);
                    i_var2 = i_var2.wrapping_add(1);
                    if !(i_var2 < ((param_1.wrapping_add(8)) as *const i32).read_unaligned()) {
                        break;
                    }
                }
            }
            return 0;
        }
        ((param_1.wrapping_add(0x14)) as *const i32).read_unaligned()
    }
}
