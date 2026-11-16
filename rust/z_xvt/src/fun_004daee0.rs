// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0075d846: [u8; 0];
    static mut DAT_00759ad0: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d5180(param_1: *mut u32);
    fn FUN_004d51b0(param_1: *mut i32);
    fn FUN_004db2f0(param_1: *mut i8, param_2: i32) -> i32;
    fn FUN_004d2190(param_1: *mut i32, param_2: i32, param_3: i32, param_4: u32, param_5: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004daee0(
    param_1: i32,
    param_2: *mut i8,
    param_3: *mut i32,
    param_4: u32,
    param_5: i32,
    param_6: i32,
) -> i32 {
    // SAFETY: all pointers and globals belong to the game; accessed exactly as the original does.
    unsafe {
        if param_2.is_null() {
            return 0;
        }
        if *param_2 == 0 {
            return 0;
        }
        if !(0..=0xff).contains(&param_1) {
            return 0;
        }
        let pi_var2: *mut i32 = (&raw mut DAT_0075d846)
            .cast::<u8>()
            .byte_add((param_1 as u32).wrapping_mul(4) as usize)
            .cast::<*mut i32>()
            .read_unaligned();
        if pi_var2.is_null() {
            return 0;
        }
        let base = pi_var2.cast::<u8>();
        let mut i_var6: i32 = param_3.read_unaligned();
        let mut u_stack_52c: u32 = param_4;
        let mut i_stack_53c: i32 = 0;
        let mut i_stack_540: i32 = param_3.add(1).read_unaligned();
        let mut i_stack_544: i32 = 0;
        let mut b_var3: bool = true;
        let mut i_stack_548: i32 = 0;
        let mut au_stack_524: [u32; 4] = [0; 4];
        let mut ab_stack_514: [u8; 256] = [0; 256];
        let mut au_stack_414: [u32; 5] = [0; 5];
        let buf = ab_stack_514.as_mut_ptr();

        FUN_004d5180(au_stack_524.as_mut_ptr());
        FUN_004d51b0(param_3);
        loop {
            let b_var1: u8 = *param_2.offset(i_stack_53c as isize) as u8;
            let mut u_stack_530: u32 = u_stack_52c;
            if b_var1 == 1 {
                u_stack_530 = param_4;
            } else if b_var1 < 2 || 7 < b_var1 {
                if b_var1 == 0x20
                    || b_var1 == 10
                    || *param_2.offset(i_stack_53c as isize + 1) == 0
                    || b_var1 == 0x24
                {
                    if b_var3 && b_var1 == 0x20 {
                        i_stack_53c = i_stack_53c.wrapping_add(1);
                        continue;
                    }
                    if b_var1 != 10 && b_var1 != 0x24 {
                        *buf.offset(i_stack_544 as isize) = b_var1;
                        i_stack_544 = i_stack_544.wrapping_add(1);
                    }
                    *buf.offset(i_stack_544 as isize) = 0;
                    let i_var4 = FUN_004db2f0(buf.cast::<i8>(), param_1);
                    let lhs = (param_3.add(2).read_unaligned() as u32).wrapping_sub(param_1 as u32)
                        as i32;
                    let rhs = (i_var4 as u32).wrapping_add(i_var6 as u32) as i32;
                    if lhs < rhs {
                        if param_6 <= i_stack_548 {
                            i_stack_540 = i_stack_540.wrapping_add(param_5).wrapping_add(param_1);
                        }
                        i_var6 = param_3.read_unaligned();
                        b_var3 = true;
                        i_stack_548 = i_stack_548.wrapping_add(1);
                    }
                    let mut i_stack_538: i32 = 0;
                    if 0 < i_stack_544 {
                        loop {
                            let ch = *buf.offset(i_stack_538 as isize);
                            let overflow: bool;
                            if i_stack_548 < param_6 {
                                let w = *base.add(ch as usize + 0x504) as u32;
                                let s = *base.add(0x609) as u32;
                                i_var6 = (i_var6 as u32).wrapping_add(w).wrapping_add(s) as i32;
                                overflow = param_3.add(2).read_unaligned() < i_var6;
                            } else {
                                let u_var5 = ch as u32;
                                au_stack_414[0] = *base.add(u_var5 as usize + 0x504) as u32;
                                au_stack_414[1] = *base.add(u_var5 as usize + 0x404) as u32;
                                au_stack_414[3] = 0;
                                au_stack_414[2] = 1;
                                au_stack_414[4] =
                                    (pi_var2.add(u_var5 as usize + 1).read_unaligned() as u32)
                                        .wrapping_add(pi_var2.read_unaligned() as u32);
                                FUN_004d2190(
                                    au_stack_414.as_mut_ptr().cast::<i32>(),
                                    i_var6,
                                    i_stack_540,
                                    u_stack_52c,
                                    1,
                                );
                                i_var6 = (i_var6 as u32)
                                    .wrapping_add(*base.add(0x609) as u32)
                                    .wrapping_add(*base.add(u_var5 as usize + 0x504) as u32)
                                    as i32;
                                let lim = (param_3.add(2).read_unaligned() as u32)
                                    .wrapping_sub(param_1 as u32)
                                    as i32;
                                overflow = lim < i_var6;
                            }
                            if overflow {
                                if param_6 <= i_stack_548 {
                                    i_stack_540 =
                                        i_stack_540.wrapping_add(param_5).wrapping_add(param_1);
                                }
                                i_var6 = param_3.read_unaligned();
                                i_stack_548 = i_stack_548.wrapping_add(1);
                            }
                            i_stack_538 = i_stack_538.wrapping_add(1);
                            if i_stack_538 >= i_stack_544 {
                                break;
                            }
                        }
                    }
                    if b_var1 == 10 || b_var1 == 0x24 {
                        if param_6 <= i_stack_548 {
                            i_stack_540 = i_stack_540.wrapping_add(param_5).wrapping_add(param_1);
                        }
                        i_var6 = param_3.read_unaligned();
                        b_var3 = true;
                        i_stack_548 = i_stack_548.wrapping_add(1);
                    }
                    i_stack_544 = 0;
                } else {
                    b_var3 = false;
                    *buf.offset(i_stack_544 as isize) = b_var1;
                    i_stack_544 = i_stack_544.wrapping_add(1);
                }
            } else {
                u_stack_530 = (&raw mut DAT_00759ad0)
                    .cast::<u8>()
                    .byte_add((b_var1 as usize) * 4)
                    .cast::<u32>()
                    .read_unaligned();
            }
            u_stack_52c = u_stack_530;
            i_stack_53c = i_stack_53c.wrapping_add(1);
            if *param_2.offset(i_stack_53c as isize) == 0 {
                FUN_004d51b0(au_stack_524.as_mut_ptr().cast::<i32>());
                return i_stack_548;
            }
        }
    }
}
