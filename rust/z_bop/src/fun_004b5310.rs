// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa6cfc: i32;
    static mut DAT_00aa7b13: [u8; 4];
    static mut DAT_00aa7b1f: [u8; 4];
    static mut DAT_00aa7b23: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d64e0(param_1: *mut i32) -> u32;
    fn FUN_004d6480(param_1: *mut i32, param_2: *mut i32);
    fn FUN_004d64a0(param_1: *mut i32, param_2: i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b5310(
    param_1: *mut i32,
    param_2: *mut i32,
    param_3: i32,
    param_4: i32,
    param_5: u32,
) -> u32 {
    if param_1.is_null() {
        return 0;
    }
    let mut local_2c: [i32; 4] = [0; 4];
    let mut local_10: [i32; 4] = [0; 4];
    // SAFETY: the callees and globals are the game's own; param_1 and param_2
    // point at the caller's structures and surface/bitmap memory, accessed
    // exactly where the original does.
    unsafe {
        FUN_004d6480(local_2c.as_mut_ptr(), param_2);
        FUN_004d64a0(
            local_2c.as_mut_ptr(),
            param_3.wrapping_sub(*param_2),
            param_4.wrapping_sub(*param_2.add(1)),
        );
        FUN_004d6480(local_10.as_mut_ptr(), local_2c.as_mut_ptr());
        let u_var2 = FUN_004d64e0(local_2c.as_mut_ptr());
        if local_2c[0] <= local_2c[2] {
            if local_2c[3] < local_2c[1] {
                return u_var2;
            }
            local_2c[0] = local_2c[0].wrapping_sub(local_10[0]);
            local_2c[1] = local_2c[1].wrapping_sub(local_10[1]);
            let mut local_1c: i32 = *param_2;
            local_2c[2] = (*param_2.add(2))
                .wrapping_sub(local_1c)
                .wrapping_sub(local_2c[0])
                .wrapping_sub(local_10[2])
                .wrapping_add(1)
                .wrapping_add(local_2c[2]);
            let mut i_var6: i32 = *param_2.add(1);
            let mut local_14: i32 = (*param_2.add(3))
                .wrapping_sub(i_var6)
                .wrapping_sub(local_2c[1])
                .wrapping_sub(local_10[3])
                .wrapping_add(1)
                .wrapping_add(local_2c[3]);

            let t0 = u32::from_ne_bytes(DAT_00aa7b1f);
            if t0 == 8 {
                let t1 = u32::from_ne_bytes(DAT_00aa7b13) as i32;
                let mut i_var5: i32 = param_4
                    .wrapping_add(local_2c[1])
                    .wrapping_mul(t1)
                    .wrapping_add(local_2c[0])
                    .wrapping_add(param_3)
                    .wrapping_add(DAT_00aa6cfc);
                local_1c = local_1c
                    .wrapping_add(i_var6.wrapping_add(local_2c[1]).wrapping_mul(*param_1))
                    .wrapping_add(*param_1.add(4))
                    .wrapping_add(local_2c[0]);
                if 0 < local_14 {
                    loop {
                        i_var6 = 0;
                        if 0 < local_2c[2] {
                            loop {
                                if *((local_1c.wrapping_add(i_var6)) as *const i8) != 0 {
                                    *((i_var5.wrapping_add(i_var6)) as *mut u8) = param_5 as u8;
                                }
                                i_var6 = i_var6.wrapping_add(1);
                                if !(i_var6 < local_2c[2]) {
                                    break;
                                }
                            }
                        }
                        let t1 = u32::from_ne_bytes(DAT_00aa7b13) as i32;
                        i_var5 = i_var5.wrapping_add(t1);
                        local_1c = local_1c.wrapping_add(*param_1);
                        local_14 = local_14.wrapping_sub(1);
                        if local_14 == 0 {
                            break;
                        }
                    }
                }
            } else {
                let t0 = u32::from_ne_bytes(DAT_00aa7b1f);
                if t0 == 0x10 {
                    local_1c = i_var6
                        .wrapping_add(local_2c[1])
                        .wrapping_mul(*param_1)
                        .wrapping_add(*param_1.add(4))
                        .wrapping_add(local_2c[0])
                        .wrapping_add(local_1c);
                    let t1 = u32::from_ne_bytes(DAT_00aa7b13) as i32;
                    let mut local_18: i32 = param_4
                        .wrapping_add(local_2c[1])
                        .wrapping_mul(t1)
                        .wrapping_add(local_2c[0].wrapping_add(param_3).wrapping_mul(2))
                        .wrapping_add(DAT_00aa6cfc);
                    if 0 < local_14 {
                        loop {
                            i_var6 = 0;
                            if 0 < local_2c[2] {
                                loop {
                                    let b = *((local_1c.wrapping_add(i_var6)) as *const u8);
                                    if b != 0 {
                                        let u_var3 = (*param_1.add(b as usize + 5) as u32) & 0x1f;
                                        let i_var5: i32;
                                        let u_var4: u32;
                                        if DAT_00aa7b23 == 0 {
                                            i_var5 = 0xb;
                                            u_var4 = param_5 & 0x7e0;
                                        } else {
                                            i_var5 = 10;
                                            u_var4 = param_5 & 0x3e0;
                                        }
                                        let i_var1: i32 = if DAT_00aa7b23 == 0 { 6 } else { 5 };
                                        let x = ((param_5.wrapping_shr(i_var5 as u32))
                                            .wrapping_mul(u_var3)
                                            as i32
                                            / 0x1f)
                                            as u32;
                                        let a =
                                            (x.wrapping_shl(i_var1 as u32) as i16) as i32 as u32;
                                        let y = (((u_var4 >> 5).wrapping_mul(u_var3)) / 0x1f) as i16
                                            as i32
                                            as u32;
                                        let s = a.wrapping_add(y) as i16 as i32;
                                        let z = (((param_5 & 0x1f).wrapping_mul(u_var3)) / 0x1f)
                                            as i16
                                            as i32;
                                        let r = s.wrapping_mul(0x20).wrapping_add(z);
                                        (local_18.wrapping_add(i_var6.wrapping_mul(2)) as *mut i16)
                                            .write_unaligned(r as i16);
                                    }
                                    i_var6 = i_var6.wrapping_add(1);
                                    if !(i_var6 < local_2c[2]) {
                                        break;
                                    }
                                }
                            }
                            local_1c = local_1c.wrapping_add(*param_1);
                            let t1 = u32::from_ne_bytes(DAT_00aa7b13);
                            local_18 = local_18.wrapping_add((t1 & 0xffff_fffe) as i32);
                            local_14 = local_14.wrapping_sub(1);
                            if local_14 == 0 {
                                break;
                            }
                        }
                    }
                }
            }
        }
        u_var2
    }
}
