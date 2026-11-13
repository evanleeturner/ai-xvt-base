// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004add10(param_1: i32) -> i32 {
    if param_1 == 0 {
        return 0;
    }
    // SAFETY: param_1 is the address of the game's node structure; fields
    // are read at the same offsets the original reads them.
    unsafe {
        let p = param_1 as usize as *const u8;
        if p.byte_add(4).cast::<i32>().read_unaligned() != 0x17 {
            let mut i_var3: i32 = 0;
            let mut i_var2: i32 = 0;
            if 0 < p.byte_add(8).cast::<i32>().read_unaligned() {
                loop {
                    let arr = p.byte_add(0xc).cast::<i32>().read_unaligned();
                    let i_var1 =
                        ((arr.wrapping_add(i_var3)) as usize as *const i32).read_unaligned();
                    if i_var1 != 0 {
                        let r = FUN_004add10(i_var1);
                        if r != 0 {
                            return r;
                        }
                    }
                    i_var3 = i_var3.wrapping_add(4);
                    i_var2 = i_var2.wrapping_add(1);
                    if !(i_var2 < p.byte_add(8).cast::<i32>().read_unaligned()) {
                        break;
                    }
                }
            }
            return 0;
        }
    }
    param_1
}
