// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0042a920(param_1: *mut u32, param_2: i32, param_3: u32, param_4: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042ac50(param_1: i32, param_2: u32, param_3: u32) {
    // SAFETY: param_1 is the address of the game's own structure; the count
    // and array pointer are read from it at the original's offsets, each
    // iteration, and the callee is the game's own.
    unsafe {
        let count: i32 = ((param_1.wrapping_add(6)) as *const i32).read_unaligned();
        let mut idx: i32 = 0;
        if 0 < count {
            let mut i: i32 = 0;
            loop {
                i = i.wrapping_add(4);
                idx = idx.wrapping_add(1);
                let arr_ptr: i32 = ((param_1.wrapping_add(10)) as *const i32).read_unaligned();
                let elem: *mut u32 = (arr_ptr.wrapping_sub(4).wrapping_add(i) as *const i32)
                    .read_unaligned() as usize as *mut u32;
                FUN_0042a920(elem, param_1, param_2, param_3);
                if !(idx < count) {
                    break;
                }
            }
        }
    }
}
