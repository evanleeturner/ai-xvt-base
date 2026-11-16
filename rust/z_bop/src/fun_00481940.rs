// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009e967c: [u8; 0];
    static mut DAT_009e9670: [u8; 0];
    static mut _DAT_00a07cec: u32;
    static mut DAT_009a1fe0: u32;
    static mut DAT_009a1fe8: *mut u8;
    static mut DAT_00a07ce8: u32;
    static mut DAT_009d1308: u32;
    static mut DAT_009a8c00: u32;
    static mut DAT_009d8c88: [i8; 0];
    static mut DAT_009d8e25: [i8; 0];
    static mut DAT_009d8e29: [i8; 0];
    static mut DAT_009a2196: [i8; 0];
    static mut DAT_009a8080: [i8; 0];
    static mut DAT_009a8081: [i8; 0];
    static mut DAT_009a8082: [i8; 0];
    static mut DAT_009a8084: [i8; 0];
    static mut DAT_009a8085: [i8; 0];
    static mut DAT_009a8086: [i8; 0];
    static mut DAT_009a808b: [i8; 0];
    static mut DAT_009a808f: [i8; 0];
    static mut DAT_009a8090: [i8; 0];
    static mut DAT_009a8091: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00484f80(a: u16) -> u32;
    fn FUN_00433390(a: u32, b: u16, c: u16) -> u8;
    fn FUN_00485000(a: i32, b: u32);
    fn FUN_00451f50(a: u32, b: i32, c: i32, d: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00481940(param_1: i32, param_2: i32) -> u32 {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers are dereferenced at the offsets the original uses.
    unsafe {
        let iv3 = (param_2 as u32).wrapping_mul(0x5bd) as i32;
        let base_967c = (&raw mut DAT_009e967c) as *mut u8;
        let base_9670 = (&raw mut DAT_009e9670) as *mut u8;
        let p8c88 = (&raw mut DAT_009d8c88) as *mut i8;
        let p8e25 = (&raw mut DAT_009d8e25) as *mut i8;
        let p8e29 = (&raw mut DAT_009d8e29) as *mut i8;
        let p2196 = (&raw mut DAT_009a2196) as *mut i8;
        let p8080 = (&raw mut DAT_009a8080) as *mut i8;
        let p8081 = (&raw mut DAT_009a8081) as *mut i8;
        let p8082 = (&raw mut DAT_009a8082) as *mut i8;
        let p8084 = (&raw mut DAT_009a8084) as *mut i8;
        let p8085 = (&raw mut DAT_009a8085) as *mut i8;
        let p8086 = (&raw mut DAT_009a8086) as *mut i8;
        let p808b = (&raw mut DAT_009a808b) as *mut i8;
        let p808f = (&raw mut DAT_009a808f) as *mut i8;
        let p8090 = (&raw mut DAT_009a8090) as *mut i8;
        let p8091 = (&raw mut DAT_009a8091) as *mut i8;

        let u_var6: u32 = base_967c
            .offset(iv3 as isize)
            .cast::<u16>()
            .read_unaligned() as u32;
        let mut u_stack_1a: u16 = 0xffff;
        let mut local_1c: u16 = 0xffff;
        let mut local_8: u32 = 0xffff_ffff;
        let mut local_10: u32 = 0xffff_ffff;

        if _DAT_00a07cec < DAT_009a1fe0 {
            let mut iv9 = _DAT_00a07cec.wrapping_mul(0x23) as i32;
            let mut u_var13 = _DAT_00a07cec;
            loop {
                let tmp_d = base_9670
                    .offset(iv3 as isize)
                    .cast::<u32>()
                    .read_unaligned();
                let ent = DAT_009a1fe8;
                if *ent.offset(iv9 as isize + 3) != 0
                    && tmp_d != u_var13
                    && *ent.offset(iv9 as isize + 2) != 0x0d
                {
                    let u_var15 = u_var13 as u16;
                    let u_var7 = FUN_00484f80(u_var15);
                    if (u_var7 as u8) == 0 {
                        let u_var7 = *ent.offset(iv9 as isize + 0x16) as u32;
                        let iv1 = u_var7.wrapping_mul(0x566) as i32;
                        if (*p8c88.offset(iv1 as isize) as u8) as u32 != u_var6 {
                            let mut u_var12: u32 = 0;
                            let mut pc14 = p8e25.offset(iv1 as isize);
                            let mut b_var4: i32 = 0;
                            let mut pc11 = p8e29.offset(u_var6 as isize).offset(iv1 as isize);
                            loop {
                                if *pc11 != 0
                                    && *pc14 == param_1 as i8
                                    && -1 < (*pc14.offset(3)) as i32
                                    && *p2196.offset(
                                        u_var12
                                            .wrapping_add(u_var6.wrapping_mul(8))
                                            .wrapping_add(u_var7.wrapping_mul(0x126))
                                            as isize,
                                    ) == 4
                                {
                                    b_var4 = 1;
                                }
                                pc14 = pc14.offset(0x4e);
                                pc11 = pc11.offset(0x4e);
                                u_var12 = u_var12.wrapping_add(1);
                                if u_var12 >= 8 {
                                    break;
                                }
                            }
                            let iv8 = u_var6.wrapping_mul(7).wrapping_add(param_1 as u32);
                            let iv1 = iv8.wrapping_mul(0x2a) as i32 as isize;
                            let c = *p8080.offset(iv1);
                            if c != 10 && c != 0 {
                                let b5 = FUN_00433390(
                                    u_var13,
                                    (*p8081.offset(iv1) as u8) as u16,
                                    (*p8082.offset(iv1) as u8) as u16,
                                );
                                if b5 != 0 {
                                    b_var4 = 1;
                                }
                            }
                            let c = *p8084.offset(iv1);
                            if c != 10 && c != 0 {
                                let b5 = FUN_00433390(
                                    u_var13,
                                    (*p8085.offset(iv1) as u8) as u16,
                                    (*p8086.offset(iv1) as u8) as u16,
                                );
                                if b5 != 0 {
                                    b_var4 = 1;
                                }
                            }
                            let c = *p808b.offset(iv1);
                            if c != 10 && c != 0 {
                                let b5 = FUN_00433390(
                                    u_var13,
                                    (*p8081.offset(iv1) as u8) as u16,
                                    (*p8082.offset(iv1) as u8) as u16,
                                );
                                if b5 != 0 {
                                    b_var4 = 1;
                                }
                            }
                            let c = *p808f.offset(iv1);
                            if c != 10 && c != 0 {
                                let b5 = FUN_00433390(
                                    u_var13,
                                    (*p8090.offset(iv1) as u8) as u16,
                                    (*p8091.offset(iv1) as u8) as u16,
                                );
                                if b5 != 0 {
                                    b_var4 = 1;
                                }
                            }
                            if b_var4 != 0 {
                                let p_a = ent
                                    .offset(iv9 as isize + 0x1f)
                                    .cast::<*mut u8>()
                                    .read_unaligned();
                                let p_b = p_a.offset(0xa9).cast::<*mut u8>().read_unaligned();
                                let c2 = *p_b.offset(7) as i8;
                                if c2 == 0 || c2 == 6 {
                                    FUN_00485000(param_2, u_var13);
                                    let r = FUN_00451f50(u_var13, param_2, 0, 1);
                                    if r == 0 {
                                        if DAT_009a8c00 < local_10 {
                                            local_10 = DAT_009a8c00;
                                            local_1c = u_var15;
                                        }
                                    } else if DAT_009a8c00 < local_8 {
                                        local_8 = DAT_009a8c00;
                                        u_stack_1a = u_var15;
                                    }
                                }
                            }
                        }
                    }
                }
                u_var13 = u_var13.wrapping_add(1);
                iv9 = (iv9 as u32).wrapping_add(0x23) as i32;
                if u_var13 >= DAT_009a1fe0 {
                    break;
                }
            }
        }

        let mut u_stack_18: u16 = DAT_00a07ce8 as u16;
        if DAT_00a07ce8 < DAT_009d1308.wrapping_add(DAT_00a07ce8) {
            local_10 = DAT_00a07ce8.wrapping_mul(0x23);
            let mut u_var13 = DAT_00a07ce8;
            loop {
                u_stack_18 = u_var13 as u16;
                let ent = DAT_009a1fe8;
                if *ent.offset(local_10.wrapping_add(3) as isize) != 0 {
                    let u_var7 = *ent.offset(local_10.wrapping_add(0x16) as isize) as u32;
                    let iv9 = u_var7.wrapping_mul(0x566) as i32;
                    let tmp_w = base_967c
                        .offset(iv3 as isize)
                        .cast::<u16>()
                        .read_unaligned();
                    if (*p8c88.offset(iv9 as isize) as u8) as u16 != tmp_w {
                        let mut u_var12: u32 = 0;
                        let mut pc14 = p8e25.offset(iv9 as isize);
                        let mut pc11 = p8e29.offset(u_var6 as isize).offset(iv9 as isize);
                        loop {
                            if *pc11 != 0
                                && *pc14 == param_1 as i8
                                && -1 < (*pc14.offset(3)) as i32
                                && *p2196.offset(
                                    u_var12
                                        .wrapping_add(u_var6.wrapping_mul(8))
                                        .wrapping_add(u_var7.wrapping_mul(0x126))
                                        as isize,
                                ) == 4
                            {
                                FUN_00485000(param_2, u_var13);
                                if DAT_009a8c00 < local_8 {
                                    local_8 = DAT_009a8c00;
                                    u_stack_1a = u_stack_18;
                                }
                            }
                            pc14 = pc14.offset(0x4e);
                            pc11 = pc11.offset(0x4e);
                            u_var12 = u_var12.wrapping_add(1);
                            if u_var12 >= 8 {
                                break;
                            }
                        }
                    }
                }
                local_10 = local_10.wrapping_add(0x23);
                u_var13 = u_var13.wrapping_add(1);
                u_stack_18 = u_var13 as u16;
                if u_var13 >= DAT_009d1308.wrapping_add(DAT_00a07ce8) {
                    break;
                }
            }
        }

        let mut u_var10 = ((u_stack_18 as u32) << 16) | (u_stack_1a as u32);
        if u_stack_1a == 0xffff {
            u_var10 = 0xffff_0000u32 | (local_1c as u32);
        }
        u_var10
    }
}
