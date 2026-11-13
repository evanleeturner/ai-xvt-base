// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00479de0(param_1: i32, param_2: *mut i8) -> *mut u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042aba0(param_1: *mut u32, param_2: i32, param_3: *mut f32) {
    let mut param_1 = param_1;
    if param_1.is_null() {
        return;
    }
    // SAFETY: the pointers describe the game's own node structures and the
    // vector; reads, writes and calls happen in the original's order.
    unsafe {
        while param_1.add(1).read_unaligned() == 7 {
            param_1 = FUN_00479de0(param_2, param_1.add(5).read_unaligned() as usize as *mut i8);
            if param_1.is_null() {
                return;
            }
        }
        if param_1.add(1).read_unaligned() == 3 {
            let mut i_var2 = param_1.add(4).read_unaligned() as i32;
            let mut pf_var3 = param_1.add(5).read_unaligned() as usize as *mut f32;
            while i_var2 > 0 {
                let a = pf_var3.read_unaligned() + param_3.read_unaligned();
                pf_var3.write_unaligned(a);
                let b = pf_var3.add(1).read_unaligned() + param_3.add(1).read_unaligned();
                pf_var3.add(1).write_unaligned(b);
                let c = pf_var3.add(2).read_unaligned() + param_3.add(2).read_unaligned();
                pf_var3.add(2).write_unaligned(c);
                pf_var3 = pf_var3.add(3);
                i_var2 -= 1;
            }
        }
        let mut i_var2: i32 = 0;
        let mut i_var4: i32 = 0;
        if (param_1.add(2).read_unaligned() as i32) > 0 {
            loop {
                let pu_var1 = (param_1.add(3).read_unaligned() as usize as *const u8)
                    .byte_add(i_var2 as usize)
                    .cast::<u32>();
                i_var2 = i_var2.wrapping_add(4);
                i_var4 = i_var4.wrapping_add(1);
                FUN_0042aba0(
                    pu_var1.read_unaligned() as usize as *mut u32,
                    param_2,
                    param_3,
                );
                if !(i_var4 < param_1.add(2).read_unaligned() as i32) {
                    break;
                }
            }
        }
    }
}
