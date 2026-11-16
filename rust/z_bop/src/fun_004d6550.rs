// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa6cfc: u32;
    static mut DAT_00aa7b13: [u8; 4];
    static mut DAT_00aa7b1f: [u8; 4];
    static mut DAT_00aa7b23: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d6480(param_1: *mut i32, param_2: *mut i32);
    fn FUN_004d64a0(param_1: *mut i32, param_2: i32, param_3: i32);
    fn FUN_004d64e0(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d6550(param_1: *mut i32, param_2: i32, param_3: i32, param_4: u32) {
    // SAFETY: the callees and globals are the game's own; param_1 points at a
    // rect of four ints and the surface pointer comes from the game's globals,
    // written where the original does.
    unsafe {
        let mut l: [i32; 4] = [0; 4];
        let lp = l.as_mut_ptr();
        if !(param_1.read() < param_1.add(2).read()
            && param_1.add(1).read() < param_1.add(3).read())
        {
            return;
        }
        FUN_004d6480(lp, param_1);
        FUN_004d64a0(lp, param_2, param_3);
        if !(lp.read() < 0x281
            && (-1 < lp.add(2).read() && lp.add(1).read() < 0x1e1)
            && -1 < lp.add(3).read())
        {
            return;
        }
        FUN_004d64e0(lp);
        let mut u_var2: u32 = (&raw const DAT_00aa7b13).cast::<u32>().read_unaligned();
        lp.add(3).write(lp.add(3).read().wrapping_add(1));
        let u_var1: u32 = (lp.add(2).read() as u32)
            .wrapping_sub(lp.read() as u32)
            .wrapping_add(1);
        let bpp: u32 = (&raw const DAT_00aa7b1f).cast::<u32>().read_unaligned();
        if bpp == 8 {
            let mut p4: u32 = u_var2
                .wrapping_mul(lp.add(1).read() as u32)
                .wrapping_add(DAT_00aa6cfc)
                .wrapping_add(lp.read() as u32);
            if lp.add(1).read() < lp.add(3).read() {
                lp.add(3)
                    .write(lp.add(3).read().wrapping_sub(lp.add(1).read()));
                loop {
                    let b = param_4 as u8;
                    let u_var3: u32 =
                        ((b as u32) << 24) | ((b as u32) << 16) | ((b as u32) << 8) | (b as u32);
                    let mut p7: u32 = p4;
                    let mut c = u_var1 >> 2;
                    while c != 0 {
                        (p7 as *mut u32).write_unaligned(u_var3);
                        p7 = p7.wrapping_add(4);
                        c -= 1;
                    }
                    let mut c = u_var1 & 3;
                    while c != 0 {
                        (p7 as *mut u8).write(b);
                        p7 = p7.wrapping_add(1);
                        c -= 1;
                    }
                    p4 = p4.wrapping_add(u_var2);
                    let n = lp.add(3).read().wrapping_sub(1);
                    lp.add(3).write(n);
                    if n == 0 {
                        break;
                    }
                }
            }
            return;
        }
        if bpp != 0x10 {
            return;
        }
        let mut local_14: u32 = u_var2
            .wrapping_mul(lp.add(1).read() as u32)
            .wrapping_add((lp.read() as u32).wrapping_mul(2))
            .wrapping_add(DAT_00aa6cfc);
        if DAT_00aa7b23 == 0 {
            if lp.add(1).read() < lp.add(3).read() {
                lp.add(3)
                    .write(lp.add(3).read().wrapping_sub(lp.add(1).read()));
                u_var2 &= 0xffff_fffe;
                loop {
                    let mut u_var6 = u_var1;
                    let mut p8 = local_14;
                    if 0 < (u_var1 as i32) {
                        loop {
                            let px = (p8 as *const u16).read_unaligned() as u32;
                            let a = (px & 0xf800)
                                .wrapping_mul(0x20)
                                .wrapping_add((px & 0x7e0).wrapping_mul(8))
                                .wrapping_add(px & 0x1f)
                                >> 1;
                            let b = (param_4 & 0xf800)
                                .wrapping_mul(0x20)
                                .wrapping_add((param_4 & 0x7e0).wrapping_mul(8))
                                .wrapping_add(param_4 & 0x1f)
                                >> 1;
                            let u_var3 = a.wrapping_add(b);
                            u_var6 = u_var6.wrapping_sub(1);
                            let r: u16 = (((u_var3 >> 3) as u16) & 0x7e0)
                                .wrapping_add(((u_var3 >> 5) as u16) & 0xf800)
                                .wrapping_add((u_var3 as u16) & 0x1f);
                            (p8 as *mut u16).write_unaligned(r);
                            p8 = p8.wrapping_add(2);
                            if u_var6 == 0 {
                                break;
                            }
                        }
                    }
                    local_14 = local_14.wrapping_add(u_var2);
                    let n = lp.add(3).read().wrapping_sub(1);
                    lp.add(3).write(n);
                    if n == 0 {
                        break;
                    }
                }
            }
        } else if lp.add(1).read() < lp.add(3).read() {
            lp.add(3)
                .write(lp.add(3).read().wrapping_sub(lp.add(1).read()));
            u_var2 &= 0xffff_fffe;
            loop {
                let mut u_var6 = u_var1;
                let mut p8 = local_14;
                if 0 < (u_var1 as i32) {
                    loop {
                        let px = (p8 as *const u16).read_unaligned() as u32;
                        let a = (px & 0x7c00)
                            .wrapping_mul(0x40)
                            .wrapping_add((px & 0x3e0).wrapping_mul(8))
                            .wrapping_add(px & 0x1f)
                            >> 1;
                        let b = (param_4 & 0x7c00)
                            .wrapping_mul(0x40)
                            .wrapping_add((param_4 & 0x3e0).wrapping_mul(8))
                            .wrapping_add(param_4 & 0x1f)
                            >> 1;
                        let u_var3 = a.wrapping_add(b);
                        u_var6 = u_var6.wrapping_sub(1);
                        let r: u16 = (((u_var3 >> 3) as u16) & 0x3e0)
                            .wrapping_add(((u_var3 >> 6) as u16) & 0x7c00)
                            .wrapping_add((u_var3 as u16) & 0x1f);
                        (p8 as *mut u16).write_unaligned(r);
                        p8 = p8.wrapping_add(2);
                        if u_var6 == 0 {
                            break;
                        }
                    }
                }
                local_14 = local_14.wrapping_add(u_var2);
                let n = lp.add(3).read().wrapping_sub(1);
                lp.add(3).write(n);
                if n == 0 {
                    break;
                }
            }
        }
    }
}
