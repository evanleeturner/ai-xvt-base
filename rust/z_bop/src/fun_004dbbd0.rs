// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aabd11: [u8; 0];
    static mut DAT_00aa7f9b: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d4cf0(param_1: *mut core::ffi::c_void);
    fn FUN_004d4d20(param_1: *mut core::ffi::c_void);
    fn FUN_004dbfa0(param_1: *mut i8, param_2: i32) -> i32;
    fn FUN_004b6780(param_1: *mut i32, param_2: i32, param_3: i32, param_4: u32, param_5: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004dbbd0(
    param_1: i32,
    param_2: *mut i8,
    param_3: *mut i32,
    param_4: u32,
    param_5: i32,
    param_6: i32,
) -> i32 {
    // SAFETY: all pointers and globals are the game's own memory; reads and
    // writes occur in the same order and places as the original.
    unsafe {
        if param_2.is_null() {
            return 0;
        }
        if *param_2 == 0 {
            return 0;
        }
        if param_1 < 0 || 0xff < param_1 {
            return 0;
        }
        let pi_var2: *mut i32 = (&raw mut DAT_00aabd11)
            .cast::<u8>()
            .byte_add((param_1 as u32).wrapping_mul(4) as usize)
            .cast::<u32>()
            .read_unaligned() as usize as *mut i32;
        if pi_var2.is_null() {
            return 0;
        }
        let pb_base = pi_var2 as *mut u8;
        let mut local_534: i32 = 0;
        let mut i_var7: i32 = param_3.read();
        let mut i_var6: i32 = 0;
        let mut local_544: i32 = 0;
        let mut b_var3: i32 = 1;
        let mut local_52c: u32 = param_4;
        let mut local_540: i32 = param_3.add(1).read();
        let mut local_524: [u32; 4] = [0; 4];
        let mut local_514: [u8; 256] = [0; 256];
        let mut local_414: [u32; 5] = [0; 5];
        let p514 = local_514.as_mut_ptr();
        FUN_004d4cf0(local_524.as_mut_ptr().cast());
        FUN_004d4d20(param_3.cast());
        loop {
            let mut local_530: u32 = local_52c;
            let b_var1: u8 = *param_2.offset(local_534 as isize) as u8;
            if b_var1 == 1 {
                local_530 = param_4;
            } else if b_var1 < 2 || 7 < b_var1 {
                if b_var1 == 0x20
                    || b_var1 == 10
                    || *param_2.offset(local_534 as isize).add(1) == 0
                    || b_var1 == 0x24
                {
                    if b_var3 != 0 && b_var1 == 0x20 {
                        local_534 = local_534.wrapping_add(1);
                        continue;
                    }
                    if b_var1 != 10 && b_var1 != 0x24 {
                        *p514.offset(local_544 as isize) = b_var1;
                        local_544 = local_544.wrapping_add(1);
                    }
                    *p514.offset(local_544 as isize) = 0;
                    let i_var4 = FUN_004dbfa0(p514.cast::<i8>(), param_1);
                    if ((param_3.add(2).read() as u32).wrapping_sub(param_1 as u32) as i32)
                        < (i_var4 as u32).wrapping_add(i_var7 as u32) as i32
                    {
                        if param_6 <= i_var6 {
                            local_540 = local_540.wrapping_add(param_5).wrapping_add(param_1);
                        }
                        i_var6 = i_var6.wrapping_add(1);
                        b_var3 = 1;
                        i_var7 = param_3.read();
                    }
                    let mut local_53c: i32 = 0;
                    if 0 < local_544 {
                        loop {
                            let need_wrap: bool;
                            if i_var6 < param_6 {
                                let ch = *p514.offset(local_53c as isize) as usize;
                                i_var7 = (i_var7 as u32)
                                    .wrapping_add(*pb_base.add(ch + 0x504) as u32)
                                    .wrapping_add(*pb_base.add(0x609) as u32)
                                    as i32;
                                need_wrap = param_3.add(2).read() < i_var7;
                            } else {
                                let u_var5 = *p514.offset(local_53c as isize) as usize;
                                local_414[0] = *pb_base.add(u_var5 + 0x504) as u32;
                                local_414[1] = *pb_base.add(u_var5 + 0x404) as u32;
                                local_414[3] = 0;
                                local_414[2] = 1;
                                local_414[4] = (pi_var2.add(u_var5 + 1).read() as u32)
                                    .wrapping_add(pi_var2.read() as u32);
                                FUN_004b6780(
                                    local_414.as_mut_ptr().cast::<i32>(),
                                    i_var7,
                                    local_540,
                                    local_52c,
                                    1,
                                );
                                i_var7 = (i_var7 as u32)
                                    .wrapping_add(*pb_base.add(0x609) as u32)
                                    .wrapping_add(*pb_base.add(u_var5 + 0x504) as u32)
                                    as i32;
                                need_wrap = ((param_3.add(2).read() as u32)
                                    .wrapping_sub(param_1 as u32)
                                    as i32)
                                    < i_var7;
                            }
                            if need_wrap {
                                if param_6 <= i_var6 {
                                    local_540 =
                                        local_540.wrapping_add(param_5).wrapping_add(param_1);
                                }
                                i_var6 = i_var6.wrapping_add(1);
                                i_var7 = param_3.read();
                            }
                            local_53c = local_53c.wrapping_add(1);
                            if !(local_53c < local_544) {
                                break;
                            }
                        }
                    }
                    if b_var1 == 10 || b_var1 == 0x24 {
                        if param_6 <= i_var6 {
                            local_540 = local_540.wrapping_add(param_5).wrapping_add(param_1);
                        }
                        i_var6 = i_var6.wrapping_add(1);
                        b_var3 = 1;
                        i_var7 = param_3.read();
                    }
                    local_544 = 0;
                } else {
                    b_var3 = 0;
                    *p514.offset(local_544 as isize) = b_var1;
                    local_544 = local_544.wrapping_add(1);
                }
            } else {
                local_530 = (&raw mut DAT_00aa7f9b)
                    .cast::<u8>()
                    .byte_add((b_var1 as u32).wrapping_mul(4) as usize)
                    .cast::<u32>()
                    .read_unaligned();
            }
            local_52c = local_530;
            local_534 = local_534.wrapping_add(1);
            if *param_2.offset(local_534 as isize) == 0 {
                FUN_004d4d20(local_524.as_mut_ptr().cast());
                return i_var6;
            }
        }
    }
}
