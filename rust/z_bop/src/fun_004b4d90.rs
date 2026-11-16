// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa6cfc: i32;
    static mut DAT_00aa7b13: i32;
    static mut DAT_00aa7b1f: i32;
    static mut DAT_00aa7b23: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d64e0(param_1: *mut i32) -> u32;
    fn FUN_004d6480(param_1: *mut i32, param_2: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b4d90(param_1: *mut i32, param_2: i32, param_3: i32) -> u32 {
    if param_1.is_null() {
        return 0;
    }
    // SAFETY: param_1 points at the caller's image header and pixel data; the
    // globals and callees are the game's own; the packed globals are read unaligned.
    unsafe {
        let mut rect_a: [i32; 4] = [0; 4];
        let mut rect_b: [i32; 4] = [0; 4];
        rect_a[0] = param_2;
        rect_a[1] = param_3;
        rect_a[2] = param_1.read().wrapping_add(param_2).wrapping_sub(1);
        rect_a[3] = param_1.add(1).read().wrapping_add(param_3).wrapping_sub(1);
        FUN_004d6480(rect_b.as_mut_ptr(), rect_a.as_mut_ptr());
        let u_var2 = FUN_004d64e0(rect_a.as_mut_ptr());
        if rect_a[0] <= rect_a[2] && rect_a[1] <= rect_a[3] {
            rect_a[0] = rect_a[0].wrapping_sub(rect_b[0]);
            rect_a[1] = rect_a[1].wrapping_sub(rect_b[1]);
            let mut i_var6 = param_1.read();
            let i_var3 = rect_a[2]
                .wrapping_sub(rect_a[0])
                .wrapping_sub(rect_b[2])
                .wrapping_add(i_var6);
            let mut local_24 = param_1
                .add(1)
                .read()
                .wrapping_sub(rect_b[3])
                .wrapping_sub(rect_a[1])
                .wrapping_add(rect_a[3]);
            let tmp_a = (&raw const DAT_00aa7b1f).read_unaligned();
            if tmp_a == 8 {
                let tmp_b = (&raw const DAT_00aa7b13).read_unaligned();
                i_var6 = rect_a[1]
                    .wrapping_mul(i_var6)
                    .wrapping_add(param_1.add(4).read())
                    .wrapping_add(rect_a[0]);
                let mut i_var7 = rect_a[1]
                    .wrapping_add(param_3)
                    .wrapping_mul(tmp_b)
                    .wrapping_add(rect_a[0])
                    .wrapping_add(param_2)
                    .wrapping_add(DAT_00aa6cfc);
                if 0 < local_24 {
                    loop {
                        let mut i_var8: i32 = 0;
                        if 0 < i_var3 {
                            loop {
                                let s = i_var6.wrapping_add(i_var8) as *const u8;
                                if s.read() != 0 {
                                    let d = i_var7.wrapping_add(i_var8) as *mut u8;
                                    d.write(s.read());
                                }
                                i_var8 = i_var8.wrapping_add(1);
                                if i_var8 >= i_var3 {
                                    break;
                                }
                            }
                        }
                        let t = (&raw const DAT_00aa7b13).read_unaligned();
                        i_var7 = i_var7.wrapping_add(t);
                        i_var6 = i_var6.wrapping_add(param_1.read());
                        local_24 = local_24.wrapping_sub(1);
                        if local_24 == 0 {
                            break;
                        }
                    }
                    return u_var2;
                }
            } else {
                if tmp_a != 0x10 {
                    return u_var2;
                }
                let tmp_b = (&raw const DAT_00aa7b13).read_unaligned();
                let mut local_2c = rect_a[1]
                    .wrapping_mul(i_var6)
                    .wrapping_add(param_1.add(4).read())
                    .wrapping_add(rect_a[0]);
                let mut local_28: u32 = (rect_a[1].wrapping_add(param_3).wrapping_mul(tmp_b)
                    as u32)
                    .wrapping_add(rect_a[0].wrapping_add(param_2).wrapping_mul(2) as u32)
                    .wrapping_add(DAT_00aa6cfc as u32);
                if DAT_00aa7b23 == 0 {
                    if 0 < local_24 {
                        loop {
                            let mut i6: i32 = 0;
                            let mut p5 = local_28 as *mut u16;
                            if 0 < i_var3 {
                                loop {
                                    let px = (local_2c.wrapping_add(i6) as *const u8).read();
                                    if px != 0 {
                                        let u_var1 = u32::from(p5.read_unaligned());
                                        let pal = param_1.add(px as usize + 5).read() as u32;
                                        let a = (u_var1 & 0x1f).wrapping_add(
                                            ((u_var1 & 0x7e0)
                                                .wrapping_add((u_var1 & 0xf800).wrapping_mul(4)))
                                            .wrapping_mul(8),
                                        ) >> 1;
                                        let b = ((pal & 0xf800).wrapping_mul(0x20))
                                            .wrapping_add((pal & 0x7e0).wrapping_mul(8))
                                            .wrapping_add(pal & 0x1f)
                                            >> 1;
                                        let u_var4 = a.wrapping_add(b);
                                        let r = (((u_var4 >> 5) as u16) & 0xf800)
                                            .wrapping_add(((u_var4 >> 3) as u16) & 0x7e0)
                                            .wrapping_add((u_var4 as u16) & 0x1f);
                                        p5.write_unaligned(r);
                                    }
                                    i6 = i6.wrapping_add(1);
                                    p5 = p5.add(1);
                                    if i6 >= i_var3 {
                                        break;
                                    }
                                }
                            }
                            let t = (&raw const DAT_00aa7b13).read_unaligned() as u32;
                            local_2c = local_2c.wrapping_add(param_1.read());
                            local_28 = local_28.wrapping_add(t & 0xffff_fffe);
                            local_24 = local_24.wrapping_sub(1);
                            if local_24 == 0 {
                                break;
                            }
                        }
                    }
                } else if 0 < local_24 {
                    loop {
                        let mut i6: i32 = 0;
                        let mut p5 = local_28 as *mut u16;
                        if 0 < i_var3 {
                            loop {
                                let px = (local_2c.wrapping_add(i6) as *const u8).read();
                                if px != 0 {
                                    let u_var1 = u32::from(p5.read_unaligned());
                                    let pal = param_1.add(px as usize + 5).read() as u32;
                                    let b = ((pal & 0x7c00).wrapping_mul(0x40))
                                        .wrapping_add((pal & 0x3e0).wrapping_mul(8))
                                        .wrapping_add(pal & 0x1f)
                                        >> 1;
                                    let a = (u_var1 & 0x1f).wrapping_add(
                                        ((u_var1 & 0x3e0)
                                            .wrapping_add((u_var1 & 0x7c00).wrapping_mul(8)))
                                        .wrapping_mul(8),
                                    ) >> 1;
                                    let u_var4 = b.wrapping_add(a);
                                    let r = (((u_var4 >> 3) as u16) & 0x3e0)
                                        .wrapping_add(((u_var4 >> 6) as u16) & 0x7c00)
                                        .wrapping_add((u_var4 as u16) & 0x1f);
                                    p5.write_unaligned(r);
                                }
                                i6 = i6.wrapping_add(1);
                                p5 = p5.add(1);
                                if i6 >= i_var3 {
                                    break;
                                }
                            }
                        }
                        let t = (&raw const DAT_00aa7b13).read_unaligned() as u32;
                        local_2c = local_2c.wrapping_add(param_1.read());
                        local_28 = local_28.wrapping_add(t & 0xffff_fffe);
                        local_24 = local_24.wrapping_sub(1);
                        if local_24 == 0 {
                            break;
                        }
                    }
                    return u_var2;
                }
            }
        }
        u_var2
    }
}
