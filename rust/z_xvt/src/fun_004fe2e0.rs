// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00654c60: [u32; 10];
    static mut DAT_00654b98: [u32; 10];
    static mut DAT_00654bd0: [u32; 10];
    static mut DAT_00654bb8: [u32; 0];
    static mut DAT_00654c18: [u32; 10];
    static mut DAT_00654c40: [u32; 0];
    static mut DAT_00654c88: [u32; 8];
    static mut DAT_00654ca8: [u32; 0];
    static mut DAT_00654cb8: [u32; 8];
    static mut DAT_00654cd8: [u32; 0];
    static mut DAT_00654cf0: [u32; 8];
    static mut DAT_00654d10: [u32; 10];
    static mut DAT_00654d38: [u32; 0];
    static mut DAT_00654bfc: i32;
    static mut DAT_00654bf8: i32;
    static mut DAT_00654d44: i32;

    static mut DAT_0081f7da: [u8; 0];
    static mut DAT_0081f7de: [u8; 0];
    static mut DAT_0081f7e6: [u8; 0];
    static mut DAT_0081fa9a: [u8; 0];
    static mut DAT_0081fa9e: [u8; 0];
    static mut DAT_0081fa7e: [u8; 0];
    static mut DAT_0081fa82: [u8; 0];
    static mut DAT_0081d32e: [u8; 0];
    static mut DAT_0081d332: [u8; 0];
    static mut DAT_0081e0a6: [u8; 0];
    static mut DAT_0081e0c6: [u8; 0];
    static mut DAT_0081e186: [u8; 0];
    static mut DAT_0081e266: [u8; 0];
    static mut DAT_0081e286: [u8; 0];
    static mut DAT_0081e346: [u8; 0];
    static mut DAT_0081bfba: [u8; 4];
    static mut DAT_0081bfb6: [u8; 4];
    static mut DAT_007438d8: [u8; 0];
    static mut DAT_007438e4: [u8; 0];
    static mut DAT_0081b850: i32;
    static mut DAT_0085e7c4: i8;
    static mut DAT_0075766c: i16;
    static mut DAT_00757a28: u32;
    static mut DAT_007579d0: [i32; 0];
}

macro_rules! u32p {
    ($n:ident) => {
        (&raw mut $n).cast::<u32>()
    };
}

macro_rules! u8p {
    ($n:ident) => {
        (&raw mut $n).cast::<u8>()
    };
}

unsafe fn ld<T>(p: *const u8) -> T {
    // SAFETY: the caller passes an address inside the game's own memory.
    unsafe { p.cast::<T>().read_unaligned() }
}

unsafe fn fill(p: *mut u32, n: usize, v: u32) {
    let mut i = 0;
    while i < n {
        // SAFETY: p points at an array of at least n words.
        unsafe { p.add(i).write(v) };
        i += 1;
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case, unused_assignments, unused_mut, unused_variables)]
pub unsafe extern "stdcall" fn FUN_004fe2e0() -> u32 {
    // SAFETY: every global and address here is the game's own memory, accessed
    // at the offsets and widths the original uses.
    unsafe {
        let c60 = u32p!(DAT_00654c60);
        let b98 = u32p!(DAT_00654b98);
        let bd0 = u32p!(DAT_00654bd0);
        let bb8 = u32p!(DAT_00654bb8) as *const u32;
        let c18 = u32p!(DAT_00654c18);
        let c40 = u32p!(DAT_00654c40) as *const u32;
        let c88 = u32p!(DAT_00654c88);
        let ca8 = u32p!(DAT_00654ca8) as *const u32;
        let cb8 = u32p!(DAT_00654cb8);
        let cd8 = u32p!(DAT_00654cd8) as *const u32;
        let cf0 = u32p!(DAT_00654cf0);
        let d10 = u32p!(DAT_00654d10);
        let d38 = u32p!(DAT_00654d38) as *const u32;
        let f7da = u8p!(DAT_0081f7da);
        let f7de = u8p!(DAT_0081f7de);
        let f7e6 = u8p!(DAT_0081f7e6);
        let fa9a = u8p!(DAT_0081fa9a) as *const u8;
        let fa9e = u8p!(DAT_0081fa9e) as *const u8;
        let fa7e = u8p!(DAT_0081fa7e);
        let fa82 = u8p!(DAT_0081fa82);
        let d8 = u8p!(DAT_007438d8);
        let e4 = u8p!(DAT_007438e4);

        fill(c60, 10, 0);
        fill(b98, 10, 0);
        fill(bd0, 10, 0);
        DAT_00654bfc = 0;
        let mut i_var6: i32 = 0;
        let mut pb: *mut u8 = f7da;
        DAT_00654bf8 = 0;
        loop {
            let a: i32 = ld(pb.byte_add(4));
            if a != 0 {
                i_var6 = i_var6.wrapping_add(1);
                let slot: u32 = ld(pb);
                let idx = *d8.byte_add(slot.wrapping_mul(0x562) as usize) as usize;
                if *c60.add(idx) == 0 {
                    DAT_00654bfc = DAT_00654bfc.wrapping_add(1);
                }
                let slot: u32 = ld(pb);
                let idx = *d8.byte_add(slot.wrapping_mul(0x562) as usize) as usize;
                *c60.add(idx) = 1;
            }
            pb = pb.byte_add(0x58);
            if !((pb as *const u8) < fa9a) {
                break;
            }
        }

        let bfba: i32 = ld(u8p!(DAT_0081bfba));
        let bfba12 = bfba == 1 || bfba == 2;
        if bfba12 {
            if (DAT_0085e7c4 != 0 || (DAT_0081b850 == 2 || i_var6 == 1)) && {
                i_var6 = DAT_0075766c as i32;
                i_var6 != 0
            } {
                let mut pv: *mut u8 = d8;
                loop {
                    if *pv.add(0xc) != 0 {
                        let b = *pv as usize;
                        if *c60.add(b) == 0 {
                            DAT_00654bfc = DAT_00654bfc.wrapping_add(1);
                            *bd0.add(b) = 1;
                        }
                        *c60.add(b) = 1;
                    }
                    pv = pv.byte_add(0x562);
                    i_var6 = i_var6.wrapping_sub(1);
                    if i_var6 == 0 {
                        break;
                    }
                }
            }
            let mut pb: *mut u8 = u8p!(DAT_0081d32e);
            let mut p4: *mut u32 = b98;
            loop {
                let a: i32 = ld(pb);
                *p4 = (a != -1) as u32;
                pb = pb.byte_add(20);
                p4 = p4.add(1);
                if !((p4 as *const u32) < bb8) {
                    break;
                }
            }
        }
        fill(d10, 10, 0xffff_ffff);
        fill(c18, 10, 0xffff_ffff);
        fill(cf0, 8, 0xffff_ffff);
        fill(cb8, 8, 0xffff_ffff);
        fill(c88, 8, 0xffff_ffff);

        let mut u_stack_10: u32 = 0;
        loop {
            if *c60.add(u_stack_10 as usize) != 0 {
                let mut i6: usize = 0;
                let mut p4: *mut u32 = c18;
                let mut u17: u32 = u_stack_10;
                loop {
                    let u11 = *p4;
                    if u11 == 0xffff_ffff {
                        *c18.add(i6) = u17;
                        break;
                    }
                    let a: i32 = ld(fa7e.byte_add(u11.wrapping_mul(28) as usize));
                    let b: i32 = ld(fa7e.byte_add(u17.wrapping_mul(28) as usize));
                    let swap;
                    if bfba12 {
                        swap = a < b;
                    } else {
                        let ua: u32 = ld(fa82.byte_add(u11.wrapping_mul(28) as usize));
                        let ub: u32 = ld(fa82.byte_add(u17.wrapping_mul(28) as usize));
                        swap = (ua == 0 && ub == 1) || a < b;
                    }
                    if swap {
                        *p4 = u17;
                        u17 = u11;
                    }
                    p4 = p4.add(1);
                    i6 += 1;
                    if !((p4 as *const u32) < c40) {
                        break;
                    }
                }
            }
            u_stack_10 = u_stack_10.wrapping_add(1);
            if !(u_stack_10 < 10) {
                break;
            }
        }

        let mut i_var6: i32 = 0;
        let mut p15: *mut u32 = c18;
        let mut p9: *mut u32 = c60;
        let bfb6: i32 = ld(u8p!(DAT_0081bfb6));
        let mut i_var10: i32 = 0;
        loop {
            if *p9 != 0 {
                i_var10 = i_var6;
                if *p15 as i32 == bfb6 {
                    break;
                }
            }
            p15 = p15.add(1);
            p9 = p9.add(1);
            i_var6 = i_var6.wrapping_add(1);
            i_var10 = DAT_00654bf8;
            if !((p9 as *const u32) < (c88 as *const u32)) {
                break;
            }
        }
        DAT_00654bf8 = i_var10;

        if bfba12 {
            let mut u17: u32 = 0;
            loop {
                if *b98.add(u17 as usize) != 0 {
                    let mut i6: usize = 0;
                    let mut p4: *mut u32 = d10;
                    let mut u11: u32 = u17;
                    loop {
                        let u2 = *p4;
                        if u2 == 0xffff_ffff {
                            *d10.add(i6) = u11;
                            break;
                        }
                        let a: i32 = ld(u8p!(DAT_0081d332).byte_add(u2.wrapping_mul(20) as usize));
                        let b: i32 = ld(u8p!(DAT_0081d332).byte_add(u11.wrapping_mul(20) as usize));
                        if a < b {
                            *p4 = u11;
                            u11 = u2;
                        }
                        p4 = p4.add(1);
                        i6 += 1;
                        if !((p4 as *const u32) < d38) {
                            break;
                        }
                    }
                }
                u17 = u17.wrapping_add(1);
                if !(u17 < 10) {
                    break;
                }
            }
        }

        u_stack_10 = 0;
        let mut pb_stack_8: *mut u8 = f7de;
        loop {
            let a: i32 = ld(pb_stack_8);
            if a != 0 {
                let mut i_stack_4: usize = 0;
                let mut p15: *mut u32 = c88;
                let mut u17: u32 = u_stack_10;
                loop {
                    let i6 = *p15 as i32;
                    if i6 == -1 {
                        *c88.add(i_stack_4) = u17;
                        break;
                    }
                    let slot: u32 = ld(f7da.byte_add(u17.wrapping_mul(0x58) as usize));
                    let cls = *d8.byte_add(slot.wrapping_mul(0x562) as usize) as u32;
                    let ua: u32 = ld(fa82.byte_add(cls.wrapping_mul(28) as usize));
                    let slot: u32 = ld(f7da.byte_add((i6 as u32).wrapping_mul(0x58) as usize));
                    let cls = *d8.byte_add(slot.wrapping_mul(0x562) as usize) as u32;
                    let ub: u32 = ld(fa82.byte_add(cls.wrapping_mul(28) as usize));
                    let c: i32 = ld(f7e6.byte_add((i6 as u32).wrapping_mul(0x58) as usize));
                    let d: i32 = ld(f7e6.byte_add(u17.wrapping_mul(0x58) as usize));
                    if (ua == 1 && ub == 0) || ((ua != 0 || ub != 1) && c < d) {
                        *p15 = u17;
                        u17 = i6 as u32;
                    }
                    p15 = p15.add(1);
                    i_stack_4 += 1;
                    if !((p15 as *const u32) < ca8) {
                        break;
                    }
                }
            }
            pb_stack_8 = pb_stack_8.byte_add(0x58);
            u_stack_10 = u_stack_10.wrapping_add(1);
            if !((pb_stack_8 as *const u8) < fa9e) {
                break;
            }
        }

        let mut i_var6: i32 = 0;
        let mut pb: *mut u8 = f7de;
        loop {
            let a: i32 = ld(pb);
            if a != 0 {
                let mut i7: usize = 0;
                let mut p9: *mut u32 = cb8;
                let mut i10: i32 = i_var6;
                loop {
                    let i3 = *p9 as i32;
                    if i3 == -1 {
                        *cb8.add(i7) = i10 as u32;
                        break;
                    }
                    let ua: u32 =
                        ld(u8p!(DAT_0081e0a6).byte_add((i3 as u32).wrapping_mul(4) as usize));
                    let ub: u32 =
                        ld(u8p!(DAT_0081e0a6).byte_add((i10 as u32).wrapping_mul(4) as usize));
                    if ua < ub {
                        *p9 = i10 as u32;
                        i10 = i3;
                    }
                    p9 = p9.add(1);
                    i7 += 1;
                    if !((p9 as *const u32) < cd8) {
                        break;
                    }
                }
            }
            pb = pb.byte_add(0x58);
            i_var6 = i_var6.wrapping_add(1);
            if !((pb as *const u8) < fa9e) {
                break;
            }
        }

        if bfba12 {
            u_stack_10 = 0;
            let u17 = DAT_0075766c as i32 as u32;
            if u17 != 0 {
                pb_stack_8 = e4;
                loop {
                    let mut u11 = u_stack_10.wrapping_add(8);
                    if *pb_stack_8 != 0 {
                        let mut i6: i32 = 0;
                        let mut pb2: *mut u8 = f7de;
                        loop {
                            let a: i32 = ld(pb2);
                            let b: i32 = ld(pb2.byte_sub(4));
                            if a != 0 && b as u32 == u_stack_10 {
                                break;
                            }
                            pb2 = pb2.byte_add(0x58);
                            i6 = i6.wrapping_add(1);
                            if !((pb2 as *const u8) < fa9e) {
                                break;
                            }
                        }
                        if i6 == 8 {
                            let mut i_stack_4: usize = 0;
                            let mut p4: *mut u32 = cb8;
                            loop {
                                let u2 = *p4;
                                if u2 == 0xffff_ffff {
                                    *cb8.add(i_stack_4) = u11;
                                    break;
                                }
                                let u5: u32;
                                let u14: u32;
                                if u11 < 8 {
                                    u5 =
                                        ld(u8p!(DAT_0081e0a6)
                                            .byte_add(u11.wrapping_mul(4) as usize));
                                    u14 =
                                        ld(u8p!(DAT_0081e0c6)
                                            .byte_add(u11.wrapping_mul(4) as usize));
                                } else {
                                    u5 =
                                        ld(u8p!(DAT_0081e0c6)
                                            .byte_add(u11.wrapping_mul(4) as usize));
                                    u14 =
                                        ld(u8p!(DAT_0081e186)
                                            .byte_add(u11.wrapping_mul(4) as usize));
                                }
                                let u8_: u32;
                                let u12: u32;
                                if (u2 as i32) < 8 {
                                    u8_ =
                                        ld(u8p!(DAT_0081e0a6)
                                            .byte_add(u2.wrapping_mul(4) as usize));
                                    u12 =
                                        ld(u8p!(DAT_0081e0c6)
                                            .byte_add(u2.wrapping_mul(4) as usize));
                                } else {
                                    u8_ =
                                        ld(u8p!(DAT_0081e0c6)
                                            .byte_add(u2.wrapping_mul(4) as usize));
                                    u12 =
                                        ld(u8p!(DAT_0081e186)
                                            .byte_add(u2.wrapping_mul(4) as usize));
                                }
                                if u8_ < u5 || (u8_ == u5 && u12 < u14) {
                                    *p4 = u11;
                                    u11 = u2;
                                }
                                p4 = p4.add(1);
                                i_stack_4 += 1;
                                if !((p4 as *const u32) < cd8) {
                                    break;
                                }
                            }
                        }
                    }
                    pb_stack_8 = pb_stack_8.byte_add(0x562);
                    u_stack_10 = u_stack_10.wrapping_add(1);
                    if !(u_stack_10 < u17) {
                        break;
                    }
                }
            }
        }

        let mut i_var6: i32 = 0;
        let mut pb: *mut u8 = f7de;
        loop {
            let a: i32 = ld(pb);
            if a != 0 {
                let mut i7: usize = 0;
                let mut p9: *mut u32 = cf0;
                let mut i10: i32 = i_var6;
                loop {
                    let i3 = *p9 as i32;
                    if i3 == -1 {
                        *cf0.add(i7) = i10 as u32;
                        break;
                    }
                    let ua: u32 =
                        ld(u8p!(DAT_0081e266).byte_add((i3 as u32).wrapping_mul(4) as usize));
                    let ub: u32 =
                        ld(u8p!(DAT_0081e266).byte_add((i10 as u32).wrapping_mul(4) as usize));
                    if ua < ub {
                        *p9 = i10 as u32;
                        i10 = i3;
                    }
                    p9 = p9.add(1);
                    i7 += 1;
                    if !((p9 as *const u32) < (d10 as *const u32)) {
                        break;
                    }
                }
            }
            pb = pb.byte_add(0x58);
            i_var6 = i_var6.wrapping_add(1);
            if !((pb as *const u8) < fa9e) {
                break;
            }
        }

        if bfba12 {
            u_stack_10 = 0;
            let u17 = DAT_0075766c as i32 as u32;
            if u17 != 0 {
                pb_stack_8 = e4;
                loop {
                    let mut u11 = u_stack_10.wrapping_add(8);
                    if *pb_stack_8 != 0 {
                        let mut i6: i32 = 0;
                        let mut pb2: *mut u8 = f7de;
                        loop {
                            let a: i32 = ld(pb2);
                            let b: i32 = ld(pb2.byte_sub(4));
                            if a != 0 && b as u32 == u_stack_10 {
                                break;
                            }
                            pb2 = pb2.byte_add(0x58);
                            i6 = i6.wrapping_add(1);
                            if !((pb2 as *const u8) < fa9e) {
                                break;
                            }
                        }
                        if i6 == 8 {
                            let mut i_stack_4: usize = 0;
                            let mut p4: *mut u32 = cf0;
                            loop {
                                let u2 = *p4;
                                if u2 == 0xffff_ffff {
                                    *cf0.add(i_stack_4) = u11;
                                    break;
                                }
                                let u5: u32;
                                let u14: u32;
                                if u11 < 8 {
                                    u5 =
                                        ld(u8p!(DAT_0081e266)
                                            .byte_add(u11.wrapping_mul(4) as usize));
                                    u14 =
                                        ld(u8p!(DAT_0081e286)
                                            .byte_add(u11.wrapping_mul(4) as usize));
                                } else {
                                    u5 =
                                        ld(u8p!(DAT_0081e286)
                                            .byte_add(u11.wrapping_mul(4) as usize));
                                    u14 =
                                        ld(u8p!(DAT_0081e346)
                                            .byte_add(u11.wrapping_mul(4) as usize));
                                }
                                let u8_: u32;
                                let u12: u32;
                                if (u2 as i32) < 8 {
                                    u8_ =
                                        ld(u8p!(DAT_0081e266)
                                            .byte_add(u2.wrapping_mul(4) as usize));
                                    u12 =
                                        ld(u8p!(DAT_0081e286)
                                            .byte_add(u2.wrapping_mul(4) as usize));
                                } else {
                                    u8_ =
                                        ld(u8p!(DAT_0081e286)
                                            .byte_add(u2.wrapping_mul(4) as usize));
                                    u12 =
                                        ld(u8p!(DAT_0081e346)
                                            .byte_add(u2.wrapping_mul(4) as usize));
                                }
                                if u8_ < u5 || (u8_ == u5 && u12 < u14) {
                                    *p4 = u11;
                                    u11 = u2;
                                }
                                p4 = p4.add(1);
                                i_stack_4 += 1;
                                if !((p4 as *const u32) < (d10 as *const u32)) {
                                    break;
                                }
                            }
                        }
                    }
                    pb_stack_8 = pb_stack_8.byte_add(0x562);
                    u_stack_10 = u_stack_10.wrapping_add(1);
                    if !(u_stack_10 < u17) {
                        break;
                    }
                }
            }
        }

        let mut u17: u32 = 0;
        DAT_00654d44 = 0;
        if bfba == 1 {
            if DAT_00757a28 != 0 {
                let mut pi: *const i32 = (&raw mut DAT_007579d0).cast::<i32>();
                loop {
                    if 1 < *pi {
                        break;
                    }
                    pi = pi.add(1);
                    u17 = u17.wrapping_add(1);
                    if !(u17 < DAT_00757a28) {
                        break;
                    }
                }
                if u17 != DAT_00757a28 {
                    DAT_00654d44 = 0;
                    return 1;
                }
            }
            DAT_00654d44 = 1;
        }
        1
    }
}
