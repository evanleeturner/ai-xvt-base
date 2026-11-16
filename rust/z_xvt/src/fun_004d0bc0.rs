// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00758830: i32;
    static mut DAT_0075964b: i32;
    static mut DAT_00759657: i32;
    static mut DAT_0075965b: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dd000(param_1: *mut i32, param_2: *mut i32);
    fn FUN_004dd030(param_1: *mut i32, param_2: i32, param_3: i32);
    fn FUN_004dd0b0(param_1: *mut i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d0bc0(
    param_1: *mut i32,
    param_2: *mut i32,
    param_3: i32,
    param_4: i32,
    param_5: u32,
) -> u32 {
    if param_1.is_null() {
        return 0;
    }
    let mut a24: [i32; 4] = [0; 4];
    let mut a10: [i32; 4] = [0; 4];
    // SAFETY: the pointers are the caller's and the game's own memory; the
    // globals are read where the original reads them (two are unaligned).
    unsafe {
        FUN_004dd000(a24.as_mut_ptr(), param_2);
        FUN_004dd030(
            a24.as_mut_ptr(),
            (param_3 as u32).wrapping_sub(*param_2 as u32) as i32,
            (param_4 as u32).wrapping_sub(*param_2.add(1) as u32) as i32,
        );
        FUN_004dd000(a10.as_mut_ptr(), a24.as_mut_ptr());
        let u_var3 = FUN_004dd0b0(a24.as_mut_ptr());
        if a24[0] <= a24[2] {
            if a24[3] < a24[1] {
                return u_var3;
            }
            a24[0] = (a24[0] as u32).wrapping_sub(a10[0] as u32) as i32;
            let mut i30: u32 = *param_2 as u32;
            a24[1] = (a24[1] as u32).wrapping_sub(a10[1] as u32) as i32;
            a24[2] = ((*param_2.add(2) as u32)
                .wrapping_sub(a24[0] as u32)
                .wrapping_sub(i30)
                .wrapping_sub(a10[2] as u32))
            .wrapping_add(1)
            .wrapping_add(a24[2] as u32) as i32;
            let i_var6 = *param_2.add(1);
            let mut i14: i32 = ((*param_2.add(3) as u32)
                .wrapping_sub(a24[1] as u32)
                .wrapping_sub(i_var6 as u32)
                .wrapping_sub(a10[3] as u32))
            .wrapping_add(1)
            .wrapping_add(a24[3] as u32) as i32;

            let g657: i32 = (&raw const DAT_00759657).read_unaligned();
            let g64b: i32 = (&raw const DAT_0075964b).read_unaligned();
            let g830: i32 = DAT_00758830;

            if g657 == 8 {
                i30 = (i_var6 as u32)
                    .wrapping_add(a24[1] as u32)
                    .wrapping_mul(*param_1 as u32)
                    .wrapping_add(*param_1.add(4) as u32)
                    .wrapping_add(a24[0] as u32)
                    .wrapping_add(i30);
                let mut i2c: u32 = (param_4 as u32)
                    .wrapping_add(a24[1] as u32)
                    .wrapping_mul(g64b as u32)
                    .wrapping_add(param_3 as u32)
                    .wrapping_add(g830 as u32)
                    .wrapping_add(a24[0] as u32);
                if 0 < i14 {
                    loop {
                        let mut i = 0i32;
                        if 0 < a24[2] {
                            loop {
                                if *((i30.wrapping_add(i as u32)) as *const i8) != 0 {
                                    *((i2c.wrapping_add(i as u32)) as *mut u8) = param_5 as u8;
                                }
                                i = i.wrapping_add(1);
                                if !(i < a24[2]) {
                                    break;
                                }
                            }
                        }
                        i2c = i2c.wrapping_add(g64b as u32);
                        i30 = i30.wrapping_add(*param_1 as u32);
                        i14 = i14.wrapping_sub(1);
                        if i14 == 0 {
                            break;
                        }
                    }
                }
            } else if g657 == 0x10 {
                i30 = (i_var6 as u32)
                    .wrapping_add(a24[1] as u32)
                    .wrapping_mul(*param_1 as u32)
                    .wrapping_add(*param_1.add(4) as u32)
                    .wrapping_add(a24[0] as u32)
                    .wrapping_add(i30);
                let mut i2c: u32 = (param_4 as u32)
                    .wrapping_add(a24[1] as u32)
                    .wrapping_mul(g64b as u32)
                    .wrapping_add((a24[0] as u32).wrapping_add(param_3 as u32).wrapping_mul(2))
                    .wrapping_add(g830 as u32);
                if 0 < i14 {
                    loop {
                        let mut i = 0i32;
                        if 0 < a24[2] {
                            loop {
                                let px = *((i30.wrapping_add(i as u32)) as *const u8);
                                if px != 0 {
                                    let u5 = (*param_1.add(px as usize + 5) as u32) & 0x1f;
                                    let i_var2: u32;
                                    let u4: u32;
                                    if DAT_0075965b == 0 {
                                        i_var2 = 0xb;
                                        u4 = param_5 & 0x7e0;
                                    } else {
                                        i_var2 = 10;
                                        u4 = param_5 & 0x3e0;
                                    }
                                    let i_var1: u32 = if DAT_0075965b == 0 { 6 } else { 5 };
                                    let t1 =
                                        ((u5.wrapping_mul(param_5 & 0x1f)) as u64 / 0x1f) as u32;
                                    let t2 = ((((param_5 >> i_var2).wrapping_mul(u5)) as i32 / 0x1f)
                                        as u32)
                                        .wrapping_shl(i_var1)
                                        as u16;
                                    let t3 = (((u4 >> 5).wrapping_mul(u5)) / 0x1f) as u16;
                                    let v = t1.wrapping_add(
                                        (t2 as u32).wrapping_add(t3 as u32).wrapping_mul(0x20),
                                    );
                                    ((i2c.wrapping_add((i as u32).wrapping_mul(2))) as *mut u16)
                                        .write_unaligned(v as u16);
                                }
                                i = i.wrapping_add(1);
                                if !(i < a24[2]) {
                                    break;
                                }
                            }
                        }
                        i30 = i30.wrapping_add(*param_1 as u32);
                        i2c = i2c.wrapping_add((g64b as u32) & 0xffff_fffe);
                        i14 = i14.wrapping_sub(1);
                        if i14 == 0 {
                            break;
                        }
                    }
                }
            }
        }
        u_var3
    }
}
