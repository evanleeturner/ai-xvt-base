// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0051e000(
        param_1: *mut u32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: *mut i32,
        param_6: i32,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0051e3b0(
    param_1: *mut u32,
    param_2: i32,
    mut param_3: i32,
    param_4: i32,
    param_5: *mut i32,
    param_6: i32,
) {
    // SAFETY: all pointers are the caller's image buffers and the game's own
    // routine; reads and writes happen in the original's order and sizes.
    unsafe {
        if param_3 < 3 || param_2 < 3 {
            FUN_0051e000(param_1, param_2, param_3, param_4, param_5, param_6);
        }
        let i_var15 = param_4.wrapping_sub(param_2);
        let i_var16 = param_6.wrapping_sub(param_2);
        let mut local_1c: *const u8 = (param_1 as *const u8).wrapping_offset(param_4 as isize);
        let mut local_18: *const u8 = param_1 as *const u8;
        let b_var4: u32 = local_18.add(1).read() as u32;
        let b_var6: u32 = local_18.read() as u32;
        let mut b_var5 = b_var6;
        if local_1c.read() as u32 == b_var4 {
            b_var5 = b_var4;
        }
        let mut p5: *mut u8 = param_5 as *mut u8;
        p5.cast::<u16>()
            .write_unaligned(((b_var6 << 8) | b_var6) as u16);
        let mut psvar21: *mut i16 = p5.wrapping_offset(param_6 as isize).cast::<i16>();
        psvar21.write_unaligned(((b_var6 << 8) | b_var5) as i16);
        let mut i_var18 = param_2.wrapping_sub(3);
        let mut local_24: *const u8 = local_18.add(2);
        let mut i_var11 = i_var18;
        let mut i_var10 = param_2.wrapping_sub(2);
        let mut u_var14: u32 = b_var4;
        let mut u_var19: u32 = b_var6;
        let mut u_var17: u32;
        let mut psvar8: *mut i16;
        let mut pivar7: *mut u8;
        loop {
            u_var17 = u_var14;
            let i_var9 = i_var11;
            psvar8 = psvar21.wrapping_add(1);
            pivar7 = p5.wrapping_add(2);
            local_1c = local_1c.wrapping_add(1);
            if i_var10 < 1 {
                break;
            }
            u_var14 = local_24.read() as u32;
            let s23: i16;
            let l1c = local_1c.read() as u32;
            if l1c == u_var19 {
                if u_var19 == u_var14 {
                    s23 = (u_var17 * 0x101) as i16;
                } else {
                    s23 = (u_var19 * 0x100 + u_var17) as i16;
                }
            } else if l1c != u_var14 {
                s23 = (u_var17 * 0x101) as i16;
            } else {
                s23 = (u_var17 * 0x100 + local_24.read() as u32) as i16;
            }
            pivar7
                .cast::<u16>()
                .write_unaligned((u_var17 * 0x101) as u16);
            psvar8.write_unaligned(s23);
            local_24 = local_24.wrapping_add(1);
            i_var11 = i_var9.wrapping_sub(1);
            p5 = pivar7;
            psvar21 = psvar8;
            i_var10 = i_var9;
            u_var19 = u_var17;
        }
        let s23: i16 = if local_1c.read() as u32 == u_var19 {
            (u_var19 * 0x100 + u_var17) as i16
        } else {
            (u_var17 * 0x101) as i16
        };
        pivar7
            .cast::<u16>()
            .write_unaligned((u_var17 * 0x101) as u16);
        psvar8.write_unaligned(s23);
        local_24 = local_24.wrapping_offset(i_var15 as isize);
        psvar21 = psvar21.wrapping_offset(i_var16.wrapping_add(2) as isize);
        let mut local_20: *mut i16 = p5
            .wrapping_offset((i_var16 as isize).wrapping_mul(2))
            .wrapping_add(4)
            .cast::<i16>();
        let mut b_var24 = param_3 != 2;
        param_3 = param_3.wrapping_sub(3);
        let mut psvar22: *mut i16 = psvar21;
        let mut u_var19: u32 = u_var19;
        if b_var24 {
            loop {
                local_1c = local_1c.wrapping_offset(i_var15.wrapping_add(1) as isize);
                let b4 = local_24.read() as u32;
                u_var14 = b4;
                let pbvar1 = local_18.wrapping_add(1);
                let pbvar2 = local_24.wrapping_add(2);
                let b6 = local_24.add(1).read() as u32;
                u_var19 = b6;
                let pbvar3 = local_1c.wrapping_add(1);
                let l18 = local_18.read() as u32;
                let l1c = local_1c.read() as u32;
                let (s23, s13): (i16, i16);
                if u_var19 == l18 {
                    if l1c == l18 {
                        s23 = (b4 * 0x101) as i16;
                        s13 = s23;
                    } else {
                        s23 = ((b4 << 8) | b6) as i16;
                        s13 = ((b4 << 8) | b4) as i16;
                    }
                } else if l1c != u_var19 {
                    s23 = (b4 * 0x101) as i16;
                    s13 = s23;
                } else {
                    s23 = ((b4 << 8) | b4) as i16;
                    s13 = ((b4 << 8) | b6) as i16;
                }
                psvar22 = psvar21.wrapping_add(1);
                psvar8 = local_20.wrapping_add(1);
                local_20.write_unaligned(s23);
                psvar21.write_unaligned(s13);
                psvar21 = psvar22;
                local_24 = pbvar2;
                local_20 = psvar8;
                local_1c = pbvar3;
                local_18 = pbvar1;
                let mut local_4 = i_var18;
                u_var17 = u_var14;
                if param_2 != 2 {
                    loop {
                        u_var14 = u_var19;
                        let u_var12 = local_18.read() as u32;
                        local_18 = local_18.wrapping_add(1);
                        let pb1 = local_24.wrapping_add(1);
                        let b4 = local_24.read() as u32;
                        u_var19 = b4;
                        let pb2 = local_1c.wrapping_add(1);
                        let u_var20 = local_1c.read() as u32;
                        let both = (u_var14 * 0x101) as i16;
                        let (s23, s13): (i16, i16);
                        if u_var19 == u_var12 {
                            if u_var17 == u_var12 || u_var20 == u_var12 {
                                s23 = both;
                                s13 = both;
                            } else if u_var20 == u_var17 {
                                s23 = (u_var14 * 0x100 + b4) as i16;
                                s13 = (u_var17 * 0x100 + u_var14) as i16;
                            } else {
                                s23 = (u_var14 * 0x100 + b4) as i16;
                                s13 = both;
                            }
                        } else if u_var17 == u_var12 {
                            if u_var20 == u_var12 {
                                s23 = both;
                                s13 = both;
                            } else if u_var20 == u_var19 {
                                s23 = (u_var17 * 0x100 + u_var14) as i16;
                                s13 = (u_var14 * 0x100 + b4) as i16;
                            } else {
                                s23 = (u_var17 * 0x100 + u_var14) as i16;
                                s13 = both;
                            }
                        } else if u_var20 == u_var17 {
                            if u_var17 == u_var19 {
                                s23 = both;
                                s13 = both;
                            } else {
                                s23 = both;
                                s13 = (u_var17 * 0x100 + u_var14) as i16;
                            }
                        } else if u_var20 != u_var19 {
                            s23 = both;
                            s13 = both;
                        } else {
                            s23 = both;
                            s13 = (u_var14 * 0x100 + b4) as i16;
                        }
                        psvar22 = psvar21.wrapping_add(1);
                        local_20.write_unaligned(s23);
                        local_20 = local_20.wrapping_add(1);
                        psvar21.write_unaligned(s13);
                        b_var24 = local_4 != 0;
                        psvar21 = psvar22;
                        local_24 = pb1;
                        local_1c = pb2;
                        local_4 = local_4.wrapping_sub(1);
                        u_var17 = u_var14;
                        if !b_var24 {
                            break;
                        }
                    }
                }
                let l18 = local_18.read() as u32;
                let l1c = local_1c.read() as u32;
                let both = (u_var19 * 0x101) as i16;
                let (s23, s13): (i16, i16);
                if u_var14 == l18 {
                    if l1c == l18 {
                        s23 = both;
                        s13 = both;
                    } else {
                        s23 = (u_var14 * 0x100 + u_var19) as i16;
                        s13 = both;
                    }
                } else if l1c != u_var14 {
                    s23 = both;
                    s13 = both;
                } else {
                    s23 = both;
                    s13 = (u_var14 * 0x100 + u_var19) as i16;
                }
                local_20.write_unaligned(s23);
                psvar22.write_unaligned(s13);
                local_20 = local_20.wrapping_offset(i_var16.wrapping_add(1) as isize);
                psvar21 = psvar22.wrapping_offset(i_var16.wrapping_add(1) as isize);
                local_18 = local_18.wrapping_offset(i_var15.wrapping_add(1) as isize);
                local_24 = local_24.wrapping_offset(i_var15 as isize);
                b_var24 = param_3 != 0;
                param_3 = param_3.wrapping_sub(1);
                if !b_var24 {
                    break;
                }
            }
        }
        let b4 = local_24.read() as u32;
        u_var14 = b4;
        let pbvar1 = local_18.wrapping_add(1);
        u_var19 = local_24.add(1).read() as u32;
        let mut b6 = b4;
        if u_var19 == local_18.read() as u32 {
            b6 = local_24.add(1).read() as u32;
        }
        psvar22 = psvar21.wrapping_add(1);
        psvar8 = local_20.wrapping_add(1);
        local_20.write_unaligned(((b4 << 8) | b6) as i16);
        psvar21.write_unaligned(((b4 << 8) | b4) as i16);
        psvar21 = psvar22;
        local_24 = local_24.wrapping_add(2);
        local_20 = psvar8;
        local_18 = pbvar1;
        u_var17 = u_var14;
        if param_2 != 2 {
            loop {
                u_var14 = u_var19;
                let pb1 = local_18.wrapping_add(1);
                let u_var12 = local_18.read() as u32;
                u_var19 = local_24.read() as u32;
                let s23: i16;
                if u_var19 == u_var12 {
                    if u_var17 == u_var12 {
                        s23 = (u_var14 * 0x101) as i16;
                    } else {
                        s23 = (u_var14 * 0x100 + local_24.read() as u32) as i16;
                    }
                } else if u_var17 != u_var12 {
                    s23 = (u_var14 * 0x101) as i16;
                } else {
                    s23 = (u_var17 * 0x100 + u_var14) as i16;
                }
                psvar22 = psvar21.wrapping_add(1);
                local_20.write_unaligned(s23);
                local_20 = local_20.wrapping_add(1);
                psvar21.write_unaligned((u_var14 * 0x101) as i16);
                b_var24 = i_var18 != 0;
                i_var18 = i_var18.wrapping_sub(1);
                psvar21 = psvar22;
                local_24 = local_24.wrapping_add(1);
                local_18 = pb1;
                u_var17 = u_var14;
                if !b_var24 {
                    break;
                }
            }
        }
        let s23: i16 = if local_18.read() as u32 == u_var14 {
            (u_var14 * 0x100 + u_var19) as i16
        } else {
            (u_var19 * 0x101) as i16
        };
        local_20.write_unaligned(s23);
        psvar22.write_unaligned((u_var19 * 0x101) as i16);
        let _ = u_var17;
    }
}
