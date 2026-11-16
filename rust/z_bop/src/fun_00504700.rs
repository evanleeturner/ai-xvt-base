// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0066d968: [u32; 10];
    static mut DAT_0066d8a0: [u32; 10];
    static mut DAT_0066d8c0: [u32; 0];
    static mut DAT_0066d8d8: [u32; 10];
    static mut DAT_0066d900: u32;
    static mut DAT_0066d904: u32;
    static mut DAT_0066d920: [u32; 10];
    static mut DAT_0066d948: [u32; 0];
    static mut DAT_0066d990: [u32; 8];
    static mut DAT_0066d9b0: [u32; 0];
    static mut DAT_0066d9c0: [u32; 8];
    static mut DAT_0066d9e0: [u32; 0];
    static mut DAT_0066d9f8: [u32; 8];
    static mut DAT_0066da18: [u32; 10];
    static mut DAT_0066da40: [u32; 0];
    static mut DAT_0066da4c: u32;

    static mut DAT_00b6d5a6: [u8; 0];
    static mut DAT_00b6d5aa: [u8; 0];
    static mut DAT_00b6d5b2: [u8; 0];
    static mut DAT_00b6d866: [u8; 0];
    static mut DAT_00b6d86a: [u8; 0];
    static mut DAT_00b6d84a: [u8; 0];
    static mut DAT_00b6d84e: [u8; 0];
    static mut DAT_00b6bad2: [u8; 0];
    static mut DAT_00b6bad6: [u8; 0];
    static mut DAT_00b6bcc2: [u8; 0];
    static mut DAT_00b6bce2: [u8; 0];
    static mut DAT_00b6bda2: [u8; 0];
    static mut DAT_00b6be82: [u8; 0];
    static mut DAT_00b6bea2: [u8; 0];
    static mut DAT_00b6bf62: [u8; 0];
    static mut DAT_00b6a5aa: [u8; 4];
    static mut DAT_00b6a5a6: [u8; 4];

    static mut DAT_00a91d18: [u8; 0];
    static mut DAT_00aa5aac: i16;
    static mut DAT_00bb73a4: i8;
    static mut DAT_00b69e30: i32;
    static mut DAT_00aa5e10: [i32; 0];
    static mut DAT_00aa5e68: u32;
}

unsafe fn ld(base: *const u8, off: u32) -> u32 {
    // SAFETY: the game's own memory, read unaligned at a byte offset.
    unsafe {
        base.wrapping_add(off as usize)
            .cast::<u32>()
            .read_unaligned()
    }
}

unsafe fn key(x: u32) -> u32 {
    // SAFETY: the game's own tables, indexed as the original does.
    unsafe {
        let a91d18 = (&raw mut DAT_00a91d18).cast::<u8>();
        let b5a6 = (&raw mut DAT_00b6d5a6).cast::<u8>();
        let b84e = (&raw mut DAT_00b6d84e).cast::<u8>();
        let slot = ld(b5a6, x.wrapping_mul(0x58));
        let b = *a91d18.wrapping_add(slot.wrapping_mul(0x562) as usize) as u32;
        ld(b84e, b.wrapping_mul(0x1c))
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00504700() -> u32 {
    // SAFETY: every global is the game's own memory; pointers are walked
    // exactly as the original walks them.
    unsafe {
        let p968 = (&raw mut DAT_0066d968).cast::<u32>();
        let p8a0 = (&raw mut DAT_0066d8a0).cast::<u32>();
        let p8c0 = (&raw mut DAT_0066d8c0).cast::<u32>();
        let p8d8 = (&raw mut DAT_0066d8d8).cast::<u32>();
        let p920 = (&raw mut DAT_0066d920).cast::<u32>();
        let p948 = (&raw mut DAT_0066d948).cast::<u32>();
        let p990 = (&raw mut DAT_0066d990).cast::<u32>();
        let p9b0 = (&raw mut DAT_0066d9b0).cast::<u32>();
        let p9c0 = (&raw mut DAT_0066d9c0).cast::<u32>();
        let p9e0 = (&raw mut DAT_0066d9e0).cast::<u32>();
        let p9f8 = (&raw mut DAT_0066d9f8).cast::<u32>();
        let pda18 = (&raw mut DAT_0066da18).cast::<u32>();
        let pda40 = (&raw mut DAT_0066da40).cast::<u32>();

        let b5a6 = (&raw mut DAT_00b6d5a6).cast::<u8>();
        let b5aa = (&raw mut DAT_00b6d5aa).cast::<u8>();
        let b5b2 = (&raw mut DAT_00b6d5b2).cast::<u8>();
        let b866 = (&raw mut DAT_00b6d866).cast::<u8>();
        let b86a = (&raw mut DAT_00b6d86a).cast::<u8>();
        let b84a = (&raw mut DAT_00b6d84a).cast::<u8>();
        let b84e = (&raw mut DAT_00b6d84e).cast::<u8>();
        let bad2 = (&raw mut DAT_00b6bad2).cast::<u8>();
        let bad6 = (&raw mut DAT_00b6bad6).cast::<u8>();
        let bcc2 = (&raw mut DAT_00b6bcc2).cast::<u8>();
        let bce2 = (&raw mut DAT_00b6bce2).cast::<u8>();
        let bda2 = (&raw mut DAT_00b6bda2).cast::<u8>();
        let be82 = (&raw mut DAT_00b6be82).cast::<u8>();
        let bea2 = (&raw mut DAT_00b6bea2).cast::<u8>();
        let bf62 = (&raw mut DAT_00b6bf62).cast::<u8>();
        let a5aa = (&raw mut DAT_00b6a5aa).cast::<u8>();
        let a5a6 = (&raw mut DAT_00b6a5a6).cast::<u8>();
        let a91d18 = (&raw mut DAT_00a91d18).cast::<u8>();

        let mut i: usize;

        i = 0;
        while i < 10 {
            *p968.add(i) = 0;
            i += 1;
        }
        i = 0;
        while i < 10 {
            *p8a0.add(i) = 0;
            i += 1;
        }
        i = 0;
        while i < 10 {
            *p8d8.add(i) = 0;
            i += 1;
        }
        DAT_0066d904 = 0;
        let mut i5: i32 = 0;
        let mut pb = b5a6;
        DAT_0066d900 = 0;
        loop {
            if ld(pb, 4) != 0 {
                i5 = i5.wrapping_add(1);
                let idx = *a91d18.wrapping_add(ld(pb, 0).wrapping_mul(0x562) as usize) as usize;
                if *p968.add(idx) == 0 {
                    DAT_0066d904 = DAT_0066d904.wrapping_add(1);
                }
                *p968.add(idx) = 1;
            }
            pb = pb.wrapping_add(0x58);
            if !(pb < b866) {
                break;
            }
        }
        let t = ld(a5aa, 0);
        if t == 1 || t == 2 {
            if (DAT_00bb73a4 != 0 || (DAT_00b69e30 == 2 || i5 == 1)) && {
                i5 = DAT_00aa5aac as i32;
                i5 != 0
            } {
                let mut pb17 = a91d18;
                loop {
                    if *pb17.wrapping_add(0xc) != 0 {
                        let b = *pb17 as usize;
                        if *p968.add(b) == 0 {
                            DAT_0066d904 = DAT_0066d904.wrapping_add(1);
                            *p8d8.add(b) = 1;
                        }
                        *p968.add(b) = 1;
                    }
                    pb17 = pb17.wrapping_add(0x562);
                    i5 = i5.wrapping_sub(1);
                    if i5 == 0 {
                        break;
                    }
                }
            }
            let mut pb = bad2;
            let mut p8 = p8a0;
            loop {
                let v = ld(pb, 0) as i32;
                pb = pb.wrapping_add(0x14);
                let p6 = p8.wrapping_add(1);
                *p8 = (v != -1) as u32;
                p8 = p6;
                if !(p6 < p8c0) {
                    break;
                }
            }
        }
        i = 0;
        while i < 10 {
            *pda18.add(i) = 0xffffffff;
            *p920.add(i) = 0xffffffff;
            i += 1;
        }
        i = 0;
        while i < 8 {
            *p9f8.add(i) = 0xffffffff;
            i += 1;
        }
        i = 0;
        while i < 8 {
            *p9c0.add(i) = 0xffffffff;
            i += 1;
        }
        i = 0;
        while i < 8 {
            *p990.add(i) = 0xffffffff;
            i += 1;
        }

        let mut local_10: u32 = 0;
        loop {
            if *p968.add(local_10 as usize) != 0 {
                let mut i5: usize = 0;
                let mut p8 = p920;
                let mut u10 = local_10;
                loop {
                    let u7 = *p8;
                    if u7 == 0xffffffff {
                        *p920.add(i5) = u10;
                        break;
                    }
                    let swap;
                    let t = ld(a5aa, 0);
                    if t == 1 || t == 2 {
                        swap = (ld(b84a, u7.wrapping_mul(0x1c)) as i32)
                            < (ld(b84a, u10.wrapping_mul(0x1c)) as i32);
                    } else {
                        swap = (ld(b84e, u7.wrapping_mul(0x1c)) == 0
                            && ld(b84e, u10.wrapping_mul(0x1c)) == 1)
                            || ((ld(b84a, u7.wrapping_mul(0x1c)) as i32)
                                < (ld(b84a, u10.wrapping_mul(0x1c)) as i32));
                    }
                    if swap {
                        *p8 = u10;
                        u10 = u7;
                    }
                    p8 = p8.wrapping_add(1);
                    i5 += 1;
                    if !(p8 < p948) {
                        break;
                    }
                }
            }
            local_10 += 1;
            if !(local_10 < 10) {
                break;
            }
        }
        let mut i5: i32 = 0;
        let mut p16 = p920 as *mut i32;
        let mut p9 = p968 as *mut i32;
        let mut i12: i32 = 0;
        loop {
            if *p9 != 0 && {
                i12 = i5;
                (*p16 as u32) == ld(a5a6, 0)
            } {
                break;
            }
            p16 = p16.wrapping_add(1);
            p9 = p9.wrapping_add(1);
            i5 = i5.wrapping_add(1);
            i12 = DAT_0066d900 as i32;
            if !((p9 as *mut u32) < p990) {
                break;
            }
        }
        DAT_0066d900 = i12 as u32;
        let t = ld(a5aa, 0);
        if t == 1 || t == 2 {
            let mut u10: u32 = 0;
            loop {
                if *p8a0.add(u10 as usize) != 0 {
                    let mut i5: usize = 0;
                    let mut p8 = pda18;
                    let mut u7 = u10;
                    loop {
                        let u2 = *p8;
                        if u2 == 0xffffffff {
                            *pda18.add(i5) = u7;
                            break;
                        }
                        if (ld(bad6, u2.wrapping_mul(0x14)) as i32)
                            < (ld(bad6, u7.wrapping_mul(0x14)) as i32)
                        {
                            *p8 = u7;
                            u7 = u2;
                        }
                        p8 = p8.wrapping_add(1);
                        i5 += 1;
                        if !(p8 < pda40) {
                            break;
                        }
                    }
                }
                u10 += 1;
                if !(u10 < 10) {
                    break;
                }
            }
        }
        local_10 = 0;
        let mut local_c = b5aa;
        loop {
            if ld(local_c, 0) != 0 {
                let mut local_4: usize = 0;
                let mut p16 = p990 as *mut i32;
                let mut u10 = local_10;
                loop {
                    let i5 = *p16;
                    if i5 == -1 {
                        *p990.add(local_4) = u10;
                        break;
                    }
                    if (key(u10) == 1 && key(i5 as u32) == 0)
                        || ((key(u10) != 0 || key(i5 as u32) != 1)
                            && ((ld(b5b2, (i5 as u32).wrapping_mul(0x58)) as i32)
                                < (ld(b5b2, u10.wrapping_mul(0x58)) as i32)))
                    {
                        *p16 = u10 as i32;
                        u10 = i5 as u32;
                    }
                    p16 = p16.wrapping_add(1);
                    local_4 += 1;
                    if !((p16 as *mut u32) < p9b0) {
                        break;
                    }
                }
            }
            local_c = local_c.wrapping_add(0x58);
            local_10 += 1;
            if !(local_c < b86a) {
                break;
            }
        }
        let mut i5: i32 = 0;
        let mut pb = b5aa;
        loop {
            if ld(pb, 0) != 0 {
                let mut i15: usize = 0;
                let mut p9 = p9c0 as *mut i32;
                let mut i12 = i5;
                loop {
                    let i3 = *p9;
                    if i3 == -1 {
                        *p9c0.add(i15) = i12 as u32;
                        break;
                    }
                    if ld(bcc2, (i3 as u32).wrapping_mul(4))
                        < ld(bcc2, (i12 as u32).wrapping_mul(4))
                    {
                        *p9 = i12;
                        i12 = i3;
                    }
                    p9 = p9.wrapping_add(1);
                    i15 += 1;
                    if !((p9 as *mut u32) < p9e0) {
                        break;
                    }
                }
            }
            pb = pb.wrapping_add(0x58);
            i5 = i5.wrapping_add(1);
            if !(pb < b86a) {
                break;
            }
        }
        let t = ld(a5aa, 0);
        if t == 1 || t == 2 {
            local_10 = 0;
            let u10 = DAT_00aa5aac as i32 as u32;
            if u10 != 0 {
                let mut local_8 = a91d18.wrapping_add(0xc);
                loop {
                    let mut u7 = local_10.wrapping_add(8);
                    if *local_8 != 0 {
                        let mut i5: i32 = 0;
                        let mut pb = b5aa;
                        loop {
                            if ld(pb, 0) != 0 && ld(pb.wrapping_sub(4), 0) == local_10 {
                                break;
                            }
                            pb = pb.wrapping_add(0x58);
                            i5 = i5.wrapping_add(1);
                            if !(pb < b86a) {
                                break;
                            }
                        }
                        if i5 == 8 {
                            let mut local_4: usize = 0;
                            let mut p8 = p9c0;
                            loop {
                                let u2 = *p8;
                                if u2 == 0xffffffff {
                                    *p9c0.add(local_4) = u7;
                                    break;
                                }
                                let (u11, u14);
                                if u7 < 8 {
                                    u11 = ld(bcc2, u7.wrapping_mul(4));
                                    u14 = ld(bce2, u7.wrapping_mul(4));
                                } else {
                                    u11 = ld(bce2, u7.wrapping_mul(4));
                                    u14 = ld(bda2, u7.wrapping_mul(4));
                                }
                                let (u13, u4);
                                if (u2 as i32) < 8 {
                                    u13 = ld(bcc2, u2.wrapping_mul(4));
                                    u4 = ld(bce2, u2.wrapping_mul(4));
                                } else {
                                    u13 = ld(bce2, u2.wrapping_mul(4));
                                    u4 = ld(bda2, u2.wrapping_mul(4));
                                }
                                if u13 < u11 || (u13 == u11 && u4 < u14) {
                                    *p8 = u7;
                                    u7 = u2;
                                }
                                p8 = p8.wrapping_add(1);
                                local_4 += 1;
                                if !(p8 < p9e0) {
                                    break;
                                }
                            }
                        }
                    }
                    local_8 = local_8.wrapping_add(0x562);
                    local_10 += 1;
                    if !(local_10 < u10) {
                        break;
                    }
                }
            }
        }
        let mut i5: i32 = 0;
        let mut pb = b5aa;
        loop {
            if ld(pb, 0) != 0 {
                let mut i15: usize = 0;
                let mut p9 = p9f8 as *mut i32;
                let mut i12 = i5;
                loop {
                    let i3 = *p9;
                    if i3 == -1 {
                        *p9f8.add(i15) = i12 as u32;
                        break;
                    }
                    if ld(be82, (i3 as u32).wrapping_mul(4))
                        < ld(be82, (i12 as u32).wrapping_mul(4))
                    {
                        *p9 = i12;
                        i12 = i3;
                    }
                    p9 = p9.wrapping_add(1);
                    i15 += 1;
                    if !((p9 as *mut u32) < pda18) {
                        break;
                    }
                }
            }
            pb = pb.wrapping_add(0x58);
            i5 = i5.wrapping_add(1);
            if !(pb < b86a) {
                break;
            }
        }
        let t = ld(a5aa, 0);
        if t == 1 || t == 2 {
            local_10 = 0;
            let u10 = DAT_00aa5aac as i32 as u32;
            if u10 != 0 {
                let mut local_8 = a91d18.wrapping_add(0xc);
                loop {
                    let mut u7 = local_10.wrapping_add(8);
                    if *local_8 != 0 {
                        let mut i5: i32 = 0;
                        let mut pb = b5aa;
                        loop {
                            if ld(pb, 0) != 0 && ld(pb.wrapping_sub(4), 0) == local_10 {
                                break;
                            }
                            pb = pb.wrapping_add(0x58);
                            i5 = i5.wrapping_add(1);
                            if !(pb < b86a) {
                                break;
                            }
                        }
                        if i5 == 8 {
                            let mut local_4: usize = 0;
                            let mut p8 = p9f8;
                            loop {
                                let u2 = *p8;
                                if u2 == 0xffffffff {
                                    *p9f8.add(local_4) = u7;
                                    break;
                                }
                                let (u11, u14);
                                if u7 < 8 {
                                    u11 = ld(be82, u7.wrapping_mul(4));
                                    u14 = ld(bea2, u7.wrapping_mul(4));
                                } else {
                                    u11 = ld(bea2, u7.wrapping_mul(4));
                                    u14 = ld(bf62, u7.wrapping_mul(4));
                                }
                                let (u13, u4);
                                if (u2 as i32) < 8 {
                                    u13 = ld(be82, u2.wrapping_mul(4));
                                    u4 = ld(bea2, u2.wrapping_mul(4));
                                } else {
                                    u13 = ld(bea2, u2.wrapping_mul(4));
                                    u4 = ld(bf62, u2.wrapping_mul(4));
                                }
                                if u13 < u11 || (u13 == u11 && u4 < u14) {
                                    *p8 = u7;
                                    u7 = u2;
                                }
                                p8 = p8.wrapping_add(1);
                                local_4 += 1;
                                if !(p8 < pda18) {
                                    break;
                                }
                            }
                        }
                    }
                    local_8 = local_8.wrapping_add(0x562);
                    local_10 += 1;
                    if !(local_10 < u10) {
                        break;
                    }
                }
            }
        }
        let mut u10: u32 = 0;
        DAT_0066da4c = 0;
        if ld(a5aa, 0) == 1 {
            if DAT_00aa5e68 != 0 {
                let mut p16 = (&raw mut DAT_00aa5e10).cast::<i32>();
                loop {
                    if 1 < *p16 {
                        break;
                    }
                    p16 = p16.wrapping_add(1);
                    u10 += 1;
                    if !(u10 < DAT_00aa5e68) {
                        break;
                    }
                }
                if DAT_00aa5e68 != u10 {
                    DAT_0066da4c = 0;
                    return 1;
                }
            }
            DAT_0066da4c = 1;
        }
        1
    }
}
