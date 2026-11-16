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
    fn FUN_004dd0b0(param_1: *mut i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d05d0(param_1: *mut i32, param_2: i32, param_3: i32) -> u32 {
    if param_1.is_null() {
        return 0;
    }
    // SAFETY: the globals and callees are the game's own; param_1 points at the
    // caller's image header and the pixel buffers it and the screen describe.
    unsafe {
        let mut ai20: [i32; 4] = [0; 4];
        let mut ai10: [i32; 4] = [0; 4];
        ai20[0] = param_2;
        ai20[1] = param_3;
        ai20[2] = (param_1.read() as u32)
            .wrapping_add(param_2 as u32)
            .wrapping_sub(1) as i32;
        ai20[3] = (param_1.add(1).read() as u32)
            .wrapping_add(param_3 as u32)
            .wrapping_sub(1) as i32;
        FUN_004dd000(ai10.as_mut_ptr(), ai20.as_mut_ptr());
        let u_var2 = FUN_004dd0b0(ai20.as_mut_ptr());
        if ai20[0] <= ai20[2] && ai20[1] <= ai20[3] {
            ai20[0] = ai20[0].wrapping_sub(ai10[0]);
            ai20[1] = ai20[1].wrapping_sub(ai10[1]);
            let mut i_var8: u32 = param_1.read() as u32;
            let i_var5: i32 = (ai20[2] as u32)
                .wrapping_sub(ai20[0] as u32)
                .wrapping_sub(ai10[2] as u32)
                .wrapping_add(i_var8) as i32;
            let mut i_stack_24: i32 = (param_1.add(1).read() as u32)
                .wrapping_sub(ai20[1] as u32)
                .wrapping_sub(ai10[3] as u32)
                .wrapping_add(ai20[3] as u32) as i32;
            let l_bpp: i32 = (&raw const DAT_00759657).read_unaligned();
            let l_pitch: i32 = (&raw const DAT_0075964b).read_unaligned();
            if l_bpp == 8 {
                i_var8 = (ai20[1] as u32)
                    .wrapping_mul(i_var8)
                    .wrapping_add(param_1.add(4).read() as u32)
                    .wrapping_add(ai20[0] as u32);
                let mut i_var6: u32 = (param_2 as u32)
                    .wrapping_add(
                        (ai20[1] as u32)
                            .wrapping_add(param_3 as u32)
                            .wrapping_mul(l_pitch as u32),
                    )
                    .wrapping_add(DAT_00758830 as u32)
                    .wrapping_add(ai20[0] as u32);
                if 0 < i_stack_24 {
                    loop {
                        let mut i_var7: u32 = 0;
                        if 0 < i_var5 {
                            loop {
                                let s = i_var8.wrapping_add(i_var7) as *const u8;
                                if s.read() != 0 {
                                    (i_var6.wrapping_add(i_var7) as *mut u8).write(s.read());
                                }
                                i_var7 = i_var7.wrapping_add(1);
                                if (i_var7 as i32) >= i_var5 {
                                    break;
                                }
                            }
                        }
                        i_var6 = i_var6.wrapping_add(l_pitch as u32);
                        i_var8 = i_var8.wrapping_add(param_1.read() as u32);
                        i_stack_24 = i_stack_24.wrapping_sub(1);
                        if i_stack_24 == 0 {
                            break;
                        }
                    }
                }
            } else if l_bpp == 0x10 {
                let mut i_stack_2c: u32 = (ai20[1] as u32)
                    .wrapping_mul(i_var8)
                    .wrapping_add(param_1.add(4).read() as u32)
                    .wrapping_add(ai20[0] as u32);
                let mut p_stack_28: u32 = (ai20[1] as u32)
                    .wrapping_add(param_3 as u32)
                    .wrapping_mul(l_pitch as u32)
                    .wrapping_add(
                        (ai20[0] as u32)
                            .wrapping_add(param_2 as u32)
                            .wrapping_mul(2),
                    )
                    .wrapping_add(DAT_00758830 as u32);
                let is565 = DAT_0075965b == 0;
                if 0 < i_stack_24 {
                    loop {
                        let mut i_var8b: i32 = 0;
                        let mut pv4: *mut u16 = p_stack_28 as *mut u16;
                        if 0 < i_var5 {
                            loop {
                                let idx =
                                    (i_stack_2c.wrapping_add(i_var8b as u32) as *const u8).read();
                                if idx != 0 {
                                    let pal = param_1.add(idx as usize + 5).read() as u32;
                                    let u1 = pv4.read_unaligned() as u32;
                                    if is565 {
                                        let a = (u1 & 0x1f).wrapping_add(
                                            (u1 & 0x7e0)
                                                .wrapping_add((u1 & 0xf800).wrapping_mul(4))
                                                .wrapping_mul(8),
                                        ) >> 1;
                                        let b = (pal & 0xf800)
                                            .wrapping_mul(0x20)
                                            .wrapping_add((pal & 0x7e0).wrapping_mul(8))
                                            .wrapping_add(pal & 0x1f)
                                            >> 1;
                                        let u3 = a.wrapping_add(b);
                                        let r = (((u3 >> 5) as u16) & 0xf800)
                                            .wrapping_add(((u3 >> 3) as u16) & 0x7e0)
                                            .wrapping_add((u3 as u16) & 0x1f);
                                        pv4.write_unaligned(r);
                                    } else {
                                        let b = (pal & 0x7c00)
                                            .wrapping_mul(0x40)
                                            .wrapping_add((pal & 0x3e0).wrapping_mul(8))
                                            .wrapping_add(pal & 0x1f)
                                            >> 1;
                                        let a = (u1 & 0x1f).wrapping_add(
                                            (u1 & 0x3e0)
                                                .wrapping_add((u1 & 0x7c00).wrapping_mul(8))
                                                .wrapping_mul(8),
                                        ) >> 1;
                                        let u3 = b.wrapping_add(a);
                                        let r = (((u3 >> 3) as u16) & 0x3e0)
                                            .wrapping_add(((u3 >> 6) as u16) & 0x7c00)
                                            .wrapping_add((u3 as u16) & 0x1f);
                                        pv4.write_unaligned(r);
                                    }
                                }
                                i_var8b = i_var8b.wrapping_add(1);
                                pv4 = pv4.wrapping_add(1);
                                if i_var8b >= i_var5 {
                                    break;
                                }
                            }
                        }
                        i_stack_2c = i_stack_2c.wrapping_add(param_1.read() as u32);
                        p_stack_28 = p_stack_28.wrapping_add((l_pitch as u32) & 0xffff_fffe);
                        i_stack_24 = i_stack_24.wrapping_sub(1);
                        if i_stack_24 == 0 {
                            break;
                        }
                    }
                }
            }
        }
        u_var2
    }
}
