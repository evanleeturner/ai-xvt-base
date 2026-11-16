// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00758830: u32;
    static mut DAT_0075964b: [u8; 4];
    static mut DAT_00759657: [u8; 4];
    static mut DAT_0075965b: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dd000(param_1: *mut i32, param_2: *mut i32);
    fn FUN_004dd030(param_1: *mut i32, param_2: i32, param_3: i32);
    fn FUN_004dd0b0(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004dd130(param_1: *mut i32, param_2: i32, param_3: i32, param_4: u32) {
    // SAFETY: the game's globals and callees are its own memory; param_1 points
    // at a rect of four ints, and the framebuffer address comes from the game.
    unsafe {
        let mut rect: [i32; 4] = [0; 4];
        if !(param_1.read() < param_1.add(2).read()
            && param_1.add(1).read() < param_1.add(3).read())
        {
            return;
        }
        FUN_004dd000(rect.as_mut_ptr(), param_1);
        FUN_004dd030(rect.as_mut_ptr(), param_2, param_3);
        let mut local_10 = rect[0];
        let mut local_c = rect[1];
        let mut local_8 = rect[2];
        let mut local_4 = rect[3];
        if !(local_10 < 0x281 && (-1 < local_8 && local_c < 0x1e1 && -1 < local_4)) {
            return;
        }
        FUN_004dd0b0(rect.as_mut_ptr());
        local_10 = rect[0];
        local_c = rect[1];
        local_8 = rect[2];
        local_4 = rect[3];

        let pitch: u32 = (&raw const DAT_0075964b).cast::<u32>().read_unaligned();
        let bpp: u32 = (&raw const DAT_00759657).cast::<u32>().read_unaligned();
        let mut u_var2 = pitch;
        local_4 = (local_4 as u32).wrapping_add(1) as i32;
        let u_var1: u32 = (local_8 as u32)
            .wrapping_sub(local_10 as u32)
            .wrapping_add(1);

        if bpp == 8 {
            let mut puvar4 = (local_c as u32)
                .wrapping_mul(pitch)
                .wrapping_add(local_10 as u32)
                .wrapping_add(DAT_00758830) as *mut u8;
            if local_c < local_4 {
                local_4 = (local_4 as u32).wrapping_sub(local_c as u32) as i32;
                loop {
                    let u_var5 = param_4 as u8;
                    let fill: u32 = ((u_var5 as u32) << 24)
                        | ((u_var5 as u32) << 16)
                        | ((u_var5 as u32) << 8)
                        | (u_var5 as u32);
                    let mut puvar7 = puvar4;
                    let mut u_var6 = u_var1 >> 2;
                    while u_var6 != 0 {
                        puvar7.cast::<u32>().write_unaligned(fill);
                        puvar7 = puvar7.wrapping_add(4);
                        u_var6 = u_var6.wrapping_sub(1);
                    }
                    u_var6 = u_var1 & 3;
                    while u_var6 != 0 {
                        puvar7.write(u_var5);
                        puvar7 = puvar7.wrapping_add(1);
                        u_var6 = u_var6.wrapping_sub(1);
                    }
                    puvar4 = puvar4.wrapping_add(u_var2 as usize);
                    local_4 = local_4.wrapping_sub(1);
                    if local_4 == 0 {
                        break;
                    }
                }
                return;
            }
        } else {
            if bpp != 0x10 {
                return;
            }
            let mut local_18 = pitch
                .wrapping_mul(local_c as u32)
                .wrapping_add((local_10 as u32).wrapping_mul(2))
                .wrapping_add(DAT_00758830) as *mut u16;
            if DAT_0075965b == 0 {
                if local_c < local_4 {
                    local_4 = (local_4 as u32).wrapping_sub(local_c as u32) as i32;
                    u_var2 = pitch & 0xffff_fffe;
                    loop {
                        let mut u_var6 = u_var1;
                        let mut puvar8 = local_18;
                        if 0 < (u_var1 as i32) {
                            loop {
                                let s = puvar8.read_unaligned() as u32;
                                let a = (s & 0xf800)
                                    .wrapping_mul(0x20)
                                    .wrapping_add((s & 0x7e0).wrapping_mul(8))
                                    .wrapping_add(s & 0x1f)
                                    >> 1;
                                let b = (param_4 & 0xf800)
                                    .wrapping_mul(0x20)
                                    .wrapping_add((param_4 & 0x7e0).wrapping_mul(8))
                                    .wrapping_add(param_4 & 0x1f)
                                    >> 1;
                                let u_var3 = a.wrapping_add(b);
                                u_var6 = u_var6.wrapping_sub(1);
                                let r = ((u_var3 >> 5) as u16 & 0xf800)
                                    .wrapping_add((u_var3 >> 3) as u16 & 0x7e0)
                                    .wrapping_add(u_var3 as u16 & 0x1f);
                                puvar8.write_unaligned(r);
                                puvar8 = puvar8.wrapping_add(1);
                                if u_var6 == 0 {
                                    break;
                                }
                            }
                        }
                        local_18 = local_18.byte_add(u_var2 as usize);
                        local_4 = local_4.wrapping_sub(1);
                        if local_4 == 0 {
                            break;
                        }
                    }
                }
            } else if local_c < local_4 {
                u_var2 = pitch & 0xffff_fffe;
                local_4 = (local_4 as u32).wrapping_sub(local_c as u32) as i32;
                loop {
                    let mut u_var6 = u_var1;
                    let mut puvar8 = local_18;
                    if 0 < (u_var1 as i32) {
                        loop {
                            let s = puvar8.read_unaligned() as u32;
                            let a = (s & 0x7c00)
                                .wrapping_mul(0x40)
                                .wrapping_add((s & 0x3e0).wrapping_mul(8))
                                .wrapping_add(s & 0x1f)
                                >> 1;
                            let b = (param_4 & 0x7c00)
                                .wrapping_mul(0x40)
                                .wrapping_add((param_4 & 0x3e0).wrapping_mul(8))
                                .wrapping_add(param_4 & 0x1f)
                                >> 1;
                            let u_var3 = a.wrapping_add(b);
                            u_var6 = u_var6.wrapping_sub(1);
                            let r = ((u_var3 >> 6) as u16 & 0x7c00)
                                .wrapping_add((u_var3 >> 3) as u16 & 0x3e0)
                                .wrapping_add(u_var3 as u16 & 0x1f);
                            puvar8.write_unaligned(r);
                            puvar8 = puvar8.wrapping_add(1);
                            if u_var6 == 0 {
                                break;
                            }
                        }
                    }
                    local_18 = local_18.byte_add(u_var2 as usize);
                    local_4 = local_4.wrapping_sub(1);
                    if local_4 == 0 {
                        break;
                    }
                }
                return;
            }
        }
    }
}
