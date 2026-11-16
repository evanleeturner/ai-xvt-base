// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0075d846: [u8; 1024];
    static mut DAT_00759ad0: [u32; 8];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d2190(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: u32,
        param_5: i32,
    ) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004da7c0(
    param_1: i32,
    param_2: *mut i8,
    param_3: i32,
    param_4: i32,
    param_5: u32,
) -> u32 {
    let mut param_3 = param_3;
    if param_2.is_null() {
        return 0;
    }
    if -1 < param_1 && param_1 < 0x100 {
        // SAFETY: the table and the buffers are the game's own memory; the
        // reads and calls occur in the original's order.
        unsafe {
            let tbl = (&raw mut DAT_0075d846).cast::<u8>();
            let p3: u32 = tbl
                .byte_add((param_1 as u32).wrapping_mul(4) as usize)
                .cast::<u32>()
                .read_unaligned();
            if p3 != 0 {
                let mut i_var5: usize = 0;
                let mut c_var1: i8 = *param_2;
                let mut local_418: u32 = 0;
                let mut local_41c: u32 = param_5;
                let mut local_414: [u32; 5] = [0; 5];
                while c_var1 != 0 && param_3 < 0x280 {
                    let b_var2: u8 = *param_2.add(i_var5) as u8;
                    if b_var2 == 1 {
                        local_41c = param_5;
                    } else if b_var2 < 2 || 7 < b_var2 {
                        let u_var4 = b_var2 as u32;
                        local_414[0] =
                            *(p3.wrapping_add(u_var4).wrapping_add(0x504) as *const u8) as u32;
                        local_414[1] =
                            *(p3.wrapping_add(u_var4).wrapping_add(0x404) as *const u8) as u32;
                        local_414[3] = 0;
                        local_414[2] = 1;
                        let entry = (p3.wrapping_add(u_var4.wrapping_add(1).wrapping_mul(4))
                            as *const u32)
                            .read_unaligned();
                        let base = (p3 as *const u32).read_unaligned();
                        local_414[4] = entry.wrapping_add(base);
                        let r = FUN_004d2190(
                            local_414.as_mut_ptr().cast::<i32>(),
                            param_3,
                            param_4,
                            local_41c,
                            1,
                        );
                        local_418 |= r;
                        let b = *param_2.add(i_var5) as u8 as u32;
                        let w1 = *(p3.wrapping_add(b).wrapping_add(0x504) as *const u8) as u32;
                        let w2 = *(p3.wrapping_add(0x609) as *const u8) as u32;
                        param_3 = (param_3 as u32).wrapping_add(w1.wrapping_add(w2)) as i32;
                    } else {
                        local_41c = DAT_00759ad0[b_var2 as usize];
                    }
                    c_var1 = *param_2.add(i_var5 + 1);
                    i_var5 += 1;
                }
                return local_418;
            }
        }
        return 0;
    }
    0
}
