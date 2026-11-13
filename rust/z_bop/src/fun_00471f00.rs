// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00471f60(param_1: *mut i32, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00471f00(param_1: *mut i32) {
    // SAFETY: param_1 points at the caller's structure; reads and writes
    // happen at the same offsets and in the same order as the original.
    unsafe {
        let mut i_var2: i32 = 0;
        let i_var4: i32 = (param_1 as usize as i32).wrapping_sub(param_1.read_unaligned());
        param_1.write_unaligned(param_1.read_unaligned().wrapping_add(i_var4));

        let mut local_10: i32 = param_1.byte_add(10).cast::<i32>().read_unaligned();
        if local_10 != 0 {
            let mut i_var3: i32 = 0;
            local_10 = local_10.wrapping_add(i_var4);
            param_1.byte_add(10).cast::<i32>().write_unaligned(local_10);

            let mut local_6: i32 = param_1.byte_add(6).cast::<i32>().read_unaligned();
            if 0 < local_6 {
                loop {
                    local_10 = param_1.byte_add(10).cast::<i32>().read_unaligned();
                    let i_var1 = local_10;

                    let mut local_ptr: i32 = ((i_var1 as usize).wrapping_add(i_var2 as usize)
                        as *const i32)
                        .read_unaligned();
                    if local_ptr != 0 {
                        local_ptr = local_ptr.wrapping_add(i_var4);
                        ((i_var1 as usize).wrapping_add(i_var2 as usize) as *mut i32)
                            .write_unaligned(local_ptr);
                        local_10 = param_1.byte_add(10).cast::<i32>().read_unaligned();
                        FUN_00471f60(local_ptr as usize as *mut i32, i_var4);
                    }
                    i_var2 = i_var2.wrapping_add(4);
                    i_var3 = i_var3.wrapping_add(1);
                    local_6 = param_1.byte_add(6).cast::<i32>().read_unaligned();
                    if !(i_var3 < local_6) {
                        break;
                    }
                }
            }
        }
        let _ = local_10;
    }
}
