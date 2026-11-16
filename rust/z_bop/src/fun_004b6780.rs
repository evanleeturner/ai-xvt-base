// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa6cfc: i32;
    static mut DAT_00aa7b13: [u8; 4];
    static mut DAT_00aa7b1f: [u8; 4];
    static mut DAT_00aac111: [u8; 4];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d6480(param_1: *mut i32, param_2: *mut i32);
    fn FUN_004d64e0(param_1: *mut i32) -> u32;
    fn FUN_004d6f50(param_1: u32) -> u32;
    fn FUN_004b69b0(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
        param_8: i8,
    );
    fn FUN_004b6ca0(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
        param_8: i16,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b6780(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: u32,
    param_5: i32,
) -> u32 {
    let mut param_4 = param_4;
    if param_1.is_null() {
        return 0;
    }
    // SAFETY: param_1 points at the caller's image record; the globals and
    // callees are the game's own; the pixel pointers are computed as the
    // original does.
    unsafe {
        // l[0]=local_20, l[1]=local_1c, l[2]=local_18, l[3]=local_14,
        // l[4]=local_10, l[5]=local_c, l[6]=local_8, l[7]=local_4
        let mut l: [i32; 8] = [0; 8];
        let lp = l.as_mut_ptr();
        let w = param_1.read_unaligned();
        let h = param_1.add(1).read_unaligned();
        l[0] = param_2;
        l[1] = param_3;
        l[2] = (w as u32).wrapping_add(param_2 as u32).wrapping_sub(1) as i32;
        l[3] = (h as u32).wrapping_add(param_3 as u32).wrapping_sub(1) as i32;
        FUN_004d6480(lp.add(4), lp);
        let u_var2 = FUN_004d64e0(lp);
        let l20 = lp.read_volatile();
        let l18 = lp.add(2).read_volatile();
        if l20 <= l18 {
            let l14 = lp.add(3).read_volatile();
            let l1c = lp.add(1).read_volatile();
            if l14 < l1c {
                return u_var2;
            }
            let l10 = (l20 as u32).wrapping_sub(lp.add(4).read_volatile() as u32) as i32;
            lp.add(4).write_volatile(l10);
            let lc = (l1c as u32).wrapping_sub(lp.add(5).read_volatile() as u32) as i32;
            lp.add(5).write_volatile(lc);
            let l8 = lp.add(6).read_volatile();
            let l4 = lp.add(7).read_volatile();
            let w = param_1.read_unaligned();
            let h = param_1.add(1).read_unaligned();
            let mut i_var5 = (h as u32)
                .wrapping_sub(l4 as u32)
                .wrapping_sub(lc as u32)
                .wrapping_add(l14 as u32) as i32;
            let i_var1 = (l18 as u32)
                .wrapping_sub(l10 as u32)
                .wrapping_sub(l8 as u32)
                .wrapping_add(w as u32) as i32;
            let bpp = (&raw const DAT_00aa7b1f).cast::<i32>().read_unaligned();
            let pitch = (&raw const DAT_00aa7b13).cast::<i32>().read_unaligned();
            let mode = param_1.add(2).read_unaligned();
            if mode == 0 {
                if bpp == 8 {
                    let mut i_var3 = (lc as u32)
                        .wrapping_mul(w as u32)
                        .wrapping_add(param_1.add(4).read_unaligned() as u32)
                        .wrapping_add(l10 as u32) as i32;
                    let mut i_var4 = (param_3 as u32)
                        .wrapping_add(lc as u32)
                        .wrapping_mul(pitch as u32)
                        .wrapping_add(l10 as u32)
                        .wrapping_add(DAT_00aa6cfc as u32)
                        .wrapping_add(param_2 as u32) as i32;
                    if 0 < i_var5 {
                        loop {
                            let mut i_var6: i32 = 0;
                            if 0 < i_var1 {
                                loop {
                                    let src =
                                        i_var3.wrapping_add(i_var6) as u32 as usize as *const u8;
                                    if src.read() != 0 {
                                        let dst =
                                            i_var4.wrapping_add(i_var6) as u32 as usize as *mut u8;
                                        dst.write(param_4 as u8);
                                    }
                                    i_var6 = i_var6.wrapping_add(1);
                                    if i_var6 >= i_var1 {
                                        break;
                                    }
                                }
                            }
                            i_var3 = (i_var3 as u32).wrapping_add(param_1.read_unaligned() as u32)
                                as i32;
                            i_var4 = (i_var4 as u32).wrapping_add(pitch as u32) as i32;
                            i_var5 = i_var5.wrapping_sub(1);
                            if i_var5 == 0 {
                                break;
                            }
                        }
                    }
                } else if bpp == 0x10 {
                    let flag = (&raw const DAT_00aac111).cast::<i32>().read_unaligned();
                    if param_5 != 0 && flag != 0 {
                        param_4 = FUN_004d6f50(param_4);
                    }
                    let mut i_var3 = (param_1.read_unaligned() as u32)
                        .wrapping_mul(lc as u32)
                        .wrapping_add(param_1.add(4).read_unaligned() as u32)
                        .wrapping_add(l10 as u32) as i32;
                    let mut i_var4 = (param_3 as u32)
                        .wrapping_add(lc as u32)
                        .wrapping_mul(pitch as u32)
                        .wrapping_add((l10 as u32).wrapping_add(param_2 as u32).wrapping_mul(2))
                        .wrapping_add(DAT_00aa6cfc as u32)
                        as i32;
                    if 0 < i_var5 {
                        loop {
                            let mut i_var6: i32 = 0;
                            if 0 < i_var1 {
                                loop {
                                    let src =
                                        i_var3.wrapping_add(i_var6) as u32 as usize as *const u8;
                                    if src.read() != 0 {
                                        let dst = i_var4.wrapping_add(i_var6.wrapping_mul(2)) as u32
                                            as usize
                                            as *mut u8;
                                        dst.cast::<i16>().write_unaligned(param_4 as i16);
                                    }
                                    i_var6 = i_var6.wrapping_add(1);
                                    if i_var6 >= i_var1 {
                                        break;
                                    }
                                }
                            }
                            i_var3 = (i_var3 as u32).wrapping_add(param_1.read_unaligned() as u32)
                                as i32;
                            i_var4 =
                                (i_var4 as u32).wrapping_add((pitch as u32) & 0xffff_fffe) as i32;
                            i_var5 = i_var5.wrapping_sub(1);
                            if i_var5 == 0 {
                                break;
                            }
                        }
                    }
                }
            } else if bpp == 8 {
                FUN_004b69b0(
                    param_1,
                    (l10 as u32).wrapping_add(param_2 as u32) as i32,
                    (param_3 as u32).wrapping_add(lc as u32) as i32,
                    l10,
                    lc,
                    i_var1,
                    i_var5,
                    param_4 as i8,
                );
            } else if bpp == 0x10 {
                let flag = (&raw const DAT_00aac111).cast::<i32>().read_unaligned();
                if param_5 != 0 && flag != 0 {
                    param_4 = FUN_004d6f50(param_4);
                }
                FUN_004b6ca0(
                    param_1,
                    (l10 as u32).wrapping_add(param_2 as u32) as i32,
                    (param_3 as u32).wrapping_add(lc as u32) as i32,
                    l10,
                    lc,
                    i_var1,
                    i_var5,
                    param_4 as i16,
                );
            }
        }
        u_var2
    }
}
