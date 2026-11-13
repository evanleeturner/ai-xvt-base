// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00449670(param_1: i32, param_2: u32) -> i32 {
    if param_1 == 0 {
        return 0;
    }
    // SAFETY: param_1 points at the game's own node structure; its fields and
    // the child pointer array are read at the offsets the original uses.
    unsafe {
        let base = param_1 as usize as *const u8;
        if base.byte_add(4).cast::<i32>().read_unaligned() == 0x19 {
            let mut i_var3: i32 = 0;
            let mut i_var2: i32 = 0;
            if 0 < base.byte_add(8).cast::<i32>().read_unaligned() {
                loop {
                    let arr = base.byte_add(0xc).cast::<i32>().read_unaligned();
                    let child = (i_var3.wrapping_add(arr) as usize as *const i32).read_unaligned();
                    if child != 0 {
                        let arr2 = base.byte_add(0xc).cast::<i32>().read_unaligned();
                        let i_var1 = FUN_00449670(
                            (arr2.wrapping_add(i_var3) as usize as *const i32).read_unaligned(),
                            param_2,
                        );
                        if i_var1 != 0 {
                            return i_var1;
                        }
                    }
                    i_var3 = i_var3.wrapping_add(4);
                    i_var2 = i_var2.wrapping_add(1);
                    if i_var2 >= base.byte_add(8).cast::<i32>().read_unaligned() {
                        break;
                    }
                }
            }
            return 0;
        }
        base.byte_add(0x14).cast::<i32>().read_unaligned()
    }
}
