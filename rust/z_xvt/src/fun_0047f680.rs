// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_007092c1: u8;
    static mut DAT_0071e6a4: u16;
    static mut DAT_00732a3c: u16;
    static mut DAT_00703c88: i32;
    static mut DAT_00732a38: i32;
    static mut DAT_006d4988: i32;
    static mut DAT_006d4980: u16;
    static mut DAT_006a89f4: u16;
    static mut DAT_006a89f8: *mut u8;
    static mut DAT_00526872: [u8; 0];
    static mut DAT_00528230: [u8; 0];
    static mut DAT_0052b0c3: [u8; 0];
    static mut DAT_00524948: [i8; 0];
    static mut DAT_00703ca0: [i8; 0];
    static mut DAT_00524934: [i8; 0];
    static mut DAT_00524920: [i8; 0];
    static mut DAT_00732e54: i32;
    static mut DAT_00732a2c: i32;
    static mut DAT_00732a28: i32;
    static mut DAT_00732a24: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00401c20();
    fn FUN_004490f0(a: u32) -> u32;
    fn FUN_0042b150(a: i32) -> i32;
    fn FUN_00449790(a: u32, b: u32) -> i32;
    fn FUN_0042b1e0(a: i32, b: i32) -> *mut u16;
    fn FUN_0044a780(a: u32, b: u32);
    fn FUN_00421c70(a: u32, b: i32);
    fn FUN_00483410(a: i16, b: i16, c: i32);
    fn FUN_00483540(a: i16, b: i32, c: i32);
    fn FUN_00413cc0(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32;
    fn FUN_00419100(a: u32, b: u32) -> u32;
    fn FUN_00478470() -> u16;
    fn FUN_00421580(a: i32);
    fn FUN_0044ad00(a: u32) -> i32;
    fn FUN_0041a750(a: u32, b: i32);
    fn FUN_00421df0(a: u32, b: i32);
    fn FUN_00422190(a: u32);
    fn FUN_00449cb0(a: i32, b: u32) -> i32;
    fn FUN_004a1d70(a: i32, b: i32);
    fn FUN_004223d0(a: u32);
}

unsafe fn rd32(p: *const u8) -> i32 {
    // SAFETY: the caller passes an address inside the game's own memory.
    unsafe { p.cast::<i32>().read_unaligned() }
}

unsafe fn rd16(p: *const u8) -> i16 {
    // SAFETY: the caller passes an address inside the game's own memory.
    unsafe { p.cast::<i16>().read_unaligned() }
}

unsafe fn wr16(p: *mut u8, v: i16) {
    // SAFETY: the caller passes an address inside the game's own memory.
    unsafe { p.cast::<i16>().write_unaligned(v) }
}

unsafe fn ent(i26: i32, o: u32) -> *mut u8 {
    // SAFETY: reads the game's entity table base global.
    unsafe {
        (DAT_006d4988 as u32)
            .wrapping_add(i26 as u32)
            .wrapping_add(o) as *mut u8
    }
}

unsafe fn sp(o: u32) -> *mut u8 {
    // SAFETY: reads the game's current-object base global.
    unsafe { (DAT_00732e54 as u32).wrapping_add(o) as *mut u8 }
}

unsafe fn repe_cmp(mut a: *const i8, mut b: *const i8, mut n: i32) -> bool {
    let mut eq = false;
    // SAFETY: both pointers address the game's NUL-padded strings.
    unsafe {
        loop {
            if n == 0 {
                break;
            }
            n = n.wrapping_sub(1);
            eq = *a == *b;
            a = a.add(1);
            b = b.add(1);
            if !eq {
                break;
            }
        }
    }
    eq
}

unsafe fn ftol() -> i64 {
    let mut r: i64 = 0;
    // SAFETY: pops the x87 top of stack and stores it truncated, as _ftol does.
    unsafe {
        core::arch::asm!(
            "fisttp qword ptr [{0}]",
            in(reg) &raw mut r,
            out("st(0)") _,
            out("st(1)") _,
            out("st(2)") _,
            out("st(3)") _,
            out("st(4)") _,
            out("st(5)") _,
            out("st(6)") _,
            out("st(7)") _,
            options(nostack)
        );
    }
    r
}

unsafe fn anim_step(us: u16) {
    // SAFETY: the globals and the animation table are the game's own memory.
    unsafe {
        let v = DAT_006a89f8
            .byte_add((DAT_006a89f4 as usize).wrapping_mul(2))
            .cast::<u16>()
            .read_unaligned();
        if v == 0xffff {
            let i18 = (us as u32)
                .wrapping_mul(8)
                .wrapping_sub(us as u32)
                .wrapping_mul(5);
            ((DAT_006d4988 as u32).wrapping_add(3).wrapping_add(i18) as *mut u8).write(0);
            if us < DAT_006d4980 {
                let i_tmp =
                    rd32((DAT_006d4988 as u32).wrapping_add(0x1f).wrapping_add(i18) as *const u8);
                let i = rd32((i_tmp as u32).wrapping_add(0xa7) as *const u8);
                if i != 0 {
                    FUN_00421580(i);
                }
            }
        } else if v == 0xfffd {
            DAT_006a89f4 = 0;
        } else if 0xfeff < v && v != 0xfffe {
            DAT_006a89f4 = v.wrapping_add(0x100);
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
#[allow(unused_assignments)]
pub unsafe extern "stdcall" fn FUN_0047f680() {
    // SAFETY: every global, table and callee is the game's own memory and code.
    unsafe {
        if DAT_007092c1 != 0 {
            FUN_00401c20();
        }
        if DAT_0071e6a4 != 0 {
            return;
        }
        DAT_0071e6a4 = 0x1d;
        let mut us: u16 = DAT_00732a3c;
        if (us as i32) >= (DAT_00703c88 as u32).wrapping_add(DAT_00732a38 as u32) as i32 {
            return;
        }
        loop {
            let mut u_var4: u32 = us as u32;
            let mut i26: i32 = u_var4.wrapping_mul(0x23) as i32;
            let b17 = *ent(i26, 3);
            if 99 < b17 && b17 < 0x6a {
                let d = u_var4.wrapping_sub(DAT_00703c88 as u32) as i32;
                let i18 = ((d >> 4) as u32).wrapping_mul(0x1d) as i32;
                let adj = ((i18 as u32).wrapping_add(((i18 >> 0x1f) as u32) & 0xf) as i32) >> 4;
                let p = ent(i26, 0x14);
                wr16(p, (rd16(p) as u16).wrapping_add(adj as u16) as i16);

                let d = u_var4.wrapping_sub(DAT_00703c88 as u32) as i32;
                let i18 = ((d >> 3) as u32).wrapping_mul(0x1d) as i32;
                let adj = ((i18 as u32).wrapping_add(((i18 >> 0x1f) as u32) & 0x1f) as i32) >> 5;
                let p = ent(i26, 0x12);
                wr16(p, (rd16(p) as u16).wrapping_add(adj as u16) as i16);

                let d = u_var4.wrapping_sub(DAT_00703c88 as u32) as i32;
                let i18 = (4u32.wrapping_sub((d >> 4) as u32)).wrapping_mul(0x1d) as i32;
                let adj = ((i18 as u32).wrapping_add(((i18 >> 0x1f) as u32) & 0xf) as i32) >> 4;
                let p = ent(i26, 0x10);
                wr16(p, (rd16(p) as u16).wrapping_add(adj as u16) as i16);
            }
            let pent = ent(i26, 0);
            let i_tmp = rd32(pent.byte_add(0x1f));
            if i_tmp == 0 {
                if *pent.byte_add(3) != 0 {
                    DAT_006a89f8 = (&raw mut DAT_00526872)
                        .cast::<u8>()
                        .byte_add((*pent.byte_add(3) as usize).wrapping_mul(0x16))
                        .cast::<*mut u8>()
                        .read_unaligned();
                    if !DAT_006a89f8.is_null() {
                        DAT_006a89f4 = *pent.byte_add(0x19) as u16;
                        if !DAT_006a89f8.is_null() {
                            DAT_006a89f4 = DAT_006a89f4.wrapping_add(1);
                            anim_step(us);
                        }
                        *ent(i26, 0x19) = DAT_006a89f4 as u8;
                    }
                }
            } else {
                let u_var8: u16 = *pent.byte_add(3) as u16;
                if u_var8 != 0 {
                    u_var4 = u_var8 as u32;
                    DAT_006a89f8 = (&raw mut DAT_00526872)
                        .cast::<u8>()
                        .byte_add((u_var4 as usize).wrapping_mul(0x16))
                        .cast::<*mut u8>()
                        .read_unaligned();
                    let i_tmp = rd32(pent.byte_add(0x1f));
                    let i_var3 = rd32((i_tmp as u32).wrapping_add(0xa7) as *const u8);
                    match *pent.byte_add(2) {
                        0..=5 => {
                            let u_var9 = FUN_004490f0(u_var4);
                            let mut b5: i32 = 0;
                            let i_tmp = rd32(ent(i26, 0x1f));
                            DAT_00732e54 = rd32((i_tmp as u32).wrapping_add(0xa7) as *const u8);
                            let i18 = FUN_0042b150(DAT_00732e54);
                            let s_tmp = rd16(sp(0x1df));
                            let mut b24: i32 = 0;
                            'a: {
                                if s_tmp == 0 {
                                    break 'a;
                                }
                                let pc20 = (&raw mut DAT_00703ca0).cast::<i8>().byte_add(
                                    (*((i18 as u32).wrapping_add(10) as *const u8) as usize)
                                        .wrapping_mul(0x55),
                                );
                                if repe_cmp(pc20, (&raw mut DAT_00524948).cast::<i8>(), 8) {
                                    break 'a;
                                }
                                if repe_cmp(pc20, (&raw mut DAT_00524934).cast::<i8>(), 0x11) {
                                    break 'a;
                                }
                                if repe_cmp(pc20, (&raw mut DAT_00524920).cast::<i8>(), 0x11) {
                                    break 'a;
                                }
                                b24 = 1;
                            }
                            if b24 != 0 {
                                let mut i_stack_1c: i32 = 0;
                                if *sp(0x213) != 0 {
                                    let mut i_stack_18: i32 = 0;
                                    let mut i_stack_14: i32 = 0;
                                    loop {
                                        if *sp(0x344u32.wrapping_add(i_stack_14 as u32)) == 2 {
                                            let u_var21: u32 = *(&raw mut DAT_0052b0c3)
                                                .cast::<u8>()
                                                .byte_add((i_stack_18 as u32).wrapping_add(
                                                    (*sp(4) as u32).wrapping_mul(0xef),
                                                )
                                                    as usize)
                                                as u32;
                                            if *sp(0x31au32.wrapping_add(u_var21)) != 0 {
                                                let i18 = FUN_00449790(u_var4, u_var21);
                                                if i18 == 0x15 {
                                                    let puvar10 =
                                                        FUN_0042b1e0(DAT_00732e54, i_stack_1c);
                                                    if *puvar10 == 0xffff {
                                                        let b = *sp(0x2f2u32.wrapping_add(u_var21));
                                                        let c = if (b & 1) == 0 {
                                                            b.wrapping_sub(4)
                                                        } else {
                                                            b.wrapping_add(4)
                                                        };
                                                        *sp(0x2f2u32.wrapping_add(u_var21)) = c;
                                                        let u22 = FUN_00478470();
                                                        if u22 < 0x600 {
                                                            let p =
                                                                sp(0x2f2u32.wrapping_add(u_var21));
                                                            *p ^= 1;
                                                        }
                                                    } else {
                                                        FUN_0044a780(u_var4, u_var21);
                                                        let i18 = (DAT_006d4988 as u32)
                                                            .wrapping_add(i26 as u32)
                                                            as i32;
                                                        let pe = i18 as u32;
                                                        FUN_00421c70(*puvar10 as u32, 0);
                                                        let t =
                                                            rd32(pe.wrapping_add(4) as *const u8);
                                                        DAT_00732a2c = DAT_00732a2c.wrapping_sub(t);
                                                        let t =
                                                            rd32(pe.wrapping_add(8) as *const u8);
                                                        DAT_00732a28 = DAT_00732a28.wrapping_sub(t);
                                                        let t =
                                                            rd32(pe.wrapping_add(0xc) as *const u8);
                                                        DAT_00732a24 = DAT_00732a24.wrapping_sub(t);
                                                        let t = rd32(
                                                            pe.wrapping_add(0x1f) as *const u8
                                                        );
                                                        if *((t as u32).wrapping_add(0x90)
                                                            as *const u8)
                                                            != 0
                                                        {
                                                            let s12 =
                                                                rd16(pe.wrapping_add(0x12)
                                                                    as *const u8);
                                                            let s10 =
                                                                rd16(pe.wrapping_add(0x10)
                                                                    as *const u8);
                                                            FUN_00483410(s12, s10, i18);
                                                            let s14 =
                                                                rd16(pe.wrapping_add(0x14)
                                                                    as *const u8);
                                                            FUN_00483540(s14, 0, i18);
                                                        }
                                                        let i12 = rd32(
                                                            pe.wrapping_add(0x1f) as *const u8
                                                        )
                                                            as u32;
                                                        let s_a = rd16(
                                                            i12.wrapping_add(0x97) as *const u8
                                                        );
                                                        let s_b = rd16(
                                                            i12.wrapping_add(0x99) as *const u8
                                                        );
                                                        let s_c = rd16(
                                                            i12.wrapping_add(0x9b) as *const u8
                                                        );
                                                        let i11 = FUN_00413cc0(
                                                            DAT_00732a2c,
                                                            DAT_00732a28,
                                                            DAT_00732a24,
                                                            s_a as i32,
                                                            s_b as i32,
                                                            s_c as i32,
                                                        );
                                                        let i12 = rd32(
                                                            pe.wrapping_add(0x1f) as *const u8
                                                        )
                                                            as u32;
                                                        let s_a = rd16(
                                                            i12.wrapping_add(0x91) as *const u8
                                                        );
                                                        let s_b = rd16(
                                                            i12.wrapping_add(0x93) as *const u8
                                                        );
                                                        let s_c = rd16(
                                                            i12.wrapping_add(0x95) as *const u8
                                                        );
                                                        let mut i12 = FUN_00413cc0(
                                                            DAT_00732a2c,
                                                            DAT_00732a28,
                                                            DAT_00732a24,
                                                            s_a as i32,
                                                            s_b as i32,
                                                            s_c as i32,
                                                        );
                                                        let i18b = rd32(
                                                            pe.wrapping_add(0x1f) as *const u8
                                                        )
                                                            as u32;
                                                        let s_a =
                                                            rd16(i18b.wrapping_add(0x9d)
                                                                as *const u8);
                                                        let s_b =
                                                            rd16(i18b.wrapping_add(0x9f)
                                                                as *const u8);
                                                        let s_c =
                                                            rd16(i18b.wrapping_add(0xa1)
                                                                as *const u8);
                                                        let mut i18c = FUN_00413cc0(
                                                            DAT_00732a2c,
                                                            DAT_00732a28,
                                                            DAT_00732a24,
                                                            s_a as i32,
                                                            s_b as i32,
                                                            s_c as i32,
                                                        );
                                                        let mut l = ftol();
                                                        DAT_00732a2c = (i11 as u32)
                                                            .wrapping_sub((l as i32) as u32)
                                                            as i32;
                                                        l = ftol();
                                                        DAT_00732a28 = (0u32
                                                            .wrapping_sub((l as i32) as u32))
                                                        .wrapping_sub(i12 as u32)
                                                            as i32;
                                                        l = ftol();
                                                        DAT_00732a24 = (i18c as u32)
                                                            .wrapping_sub((l as i32) as u32)
                                                            as i32;
                                                        l = ftol();
                                                        i18c = l as i32;
                                                        l = ftol();
                                                        i12 = l as i32;
                                                        l = ftol();
                                                        FUN_00413cc0(
                                                            DAT_00732a2c,
                                                            DAT_00732a28,
                                                            DAT_00732a24,
                                                            l as i32,
                                                            i12,
                                                            i18c,
                                                        );
                                                        l = ftol();
                                                        i18c = l as i32;
                                                        l = ftol();
                                                        i12 = l as i32;
                                                        l = ftol();
                                                        let u_var13 = FUN_00413cc0(
                                                            DAT_00732a2c,
                                                            DAT_00732a28,
                                                            DAT_00732a24,
                                                            l as i32,
                                                            i12,
                                                            i18c,
                                                        )
                                                            as u32;
                                                        l = ftol();
                                                        i18c = l as i32;
                                                        l = ftol();
                                                        i12 = l as i32;
                                                        l = ftol();
                                                        let u_var14 = FUN_00413cc0(
                                                            DAT_00732a2c,
                                                            DAT_00732a28,
                                                            DAT_00732a24,
                                                            l as i32,
                                                            i12,
                                                            i18c,
                                                        )
                                                            as u32;
                                                        let u_var15 =
                                                            FUN_00419100(u_var14, u_var13);
                                                        *sp(0x2f2u32.wrapping_add(u_var21)) =
                                                            (u_var15 >> 8) as u8;
                                                    }
                                                }
                                            }
                                        }
                                        i_stack_18 = i_stack_18.wrapping_add(8);
                                        i_stack_14 = i_stack_14.wrapping_add(4);
                                        i_stack_1c = i_stack_1c.wrapping_add(1);
                                        if !(i_stack_1c < *sp(0x213) as i32) {
                                            break;
                                        }
                                    }
                                }
                            }
                            let mut u22: u16 = 0;
                            if (u_var9 as u16) != 0 {
                                loop {
                                    let mut i18 = FUN_00449790(u_var4, u22 as u32);
                                    if i18 == 3 {
                                        DAT_006a89f8 = (&raw mut DAT_00528230).cast::<u8>();
                                        let addr = (u_var9 & 0xffff)
                                            .wrapping_add(0x2ca)
                                            .wrapping_add(i_var3 as u32);
                                        DAT_006a89f4 =
                                            ((*(addr as *const u8) as u32).wrapping_add(1)) as u16;
                                        anim_step(us);
                                        *(addr as *mut u8) = DAT_006a89f4 as u8;
                                    }
                                    if *sp(7) == 3 {
                                        let i12 = FUN_0044ad00(u_var4);
                                        if i12 == 0 {
                                            u22 = u22.wrapping_add(3);
                                            FUN_0041a750(((us as u32) << 16) | (us as u32), 0);
                                        } else {
                                            FUN_00421df0(((us as u32) << 16) | (us as u32), 0);
                                            let u7 = FUN_00478470();
                                            if u7 < 0x1800 {
                                                FUN_00422190(((us as u32) << 16) | (us as u32));
                                            }
                                        }
                                    }
                                    if b24 != 0
                                        && (i18 == 0xb
                                            || i18 == 0x17
                                            || i18 == 0xc
                                            || i18 == 0x18
                                            || i18 == 0xd
                                            || i18 == 0x19)
                                    {
                                        let u_var21 = u22 as u32;
                                        let b = *sp(0x2f2u32.wrapping_add(u_var21));
                                        let c = if (b & 1) == 0 {
                                            b.wrapping_sub(4)
                                        } else {
                                            b.wrapping_add(4)
                                        };
                                        *sp(0x2f2u32.wrapping_add(u_var21)) = c;
                                        let u7 = FUN_00478470();
                                        if u7 < 0x200 {
                                            let p = sp(0x2f2u32.wrapping_add(u_var21));
                                            *p ^= 1;
                                        }
                                    }
                                    if i18 == 0x14 && (*sp(0x29) & 1) != 0 {
                                        if (*sp(0x29) & 2) == 0 {
                                            if u_var8 == 1 {
                                                let p = sp((u22 as u32).wrapping_add(0x2f2));
                                                if *p != 0 {
                                                    *p = (*p).wrapping_sub(1);
                                                    b5 = 1;
                                                }
                                            } else if u_var8 == 4 {
                                                let p = sp((u22 as u32).wrapping_add(0x2f2));
                                                if *p != 0 {
                                                    *p = (*p).wrapping_sub(3);
                                                    b5 = 1;
                                                }
                                            }
                                        } else if u_var8 == 1 {
                                            i18 = FUN_00449cb0(1, u22 as u32);
                                            let mut b17: u8 = 8;
                                            if -1 < i18 {
                                                b17 = 0xc;
                                            }
                                            let p = sp(0x2f2u32.wrapping_add(u22 as u32));
                                            if *p < b17 {
                                                *p = (*p).wrapping_add(1);
                                                b5 = 1;
                                            }
                                        } else if u_var8 == 4 {
                                            let p = sp((u22 as u32).wrapping_add(0x2f2));
                                            if *p < 0x40 {
                                                *p = (*p).wrapping_add(3);
                                                b5 = 1;
                                            }
                                        }
                                    }
                                    u22 = u22.wrapping_add(1);
                                    if !(u22 < (u_var9 as u16)) {
                                        break;
                                    }
                                }
                            }
                            if (*sp(0x29) & 1) != 0 {
                                if (*sp(0x29) & 2) == 0 {
                                    if b5 == 0 {
                                        *sp(0x29) = 0;
                                        i26 = rd32(
                                            (DAT_006d4988 as u32)
                                                .wrapping_add(0x1b)
                                                .wrapping_add(i26 as u32)
                                                as *const u8,
                                        );
                                        FUN_004a1d70(0x7f, i26);
                                    }
                                } else if b5 == 0 {
                                    *sp(0x29) = 2;
                                    i26 = rd32(
                                        (DAT_006d4988 as u32)
                                            .wrapping_add(0x1b)
                                            .wrapping_add(i26 as u32)
                                            as *const u8,
                                    );
                                    FUN_004a1d70(0x80, i26);
                                }
                            }
                            if *sp(7) == 0 && *sp(0x23c) == 1 && rd16(sp(0x23e)) != 0 {
                                FUN_004223d0(((us as u32) << 16) | (us as u32));
                                FUN_004223d0(((us as u32) << 16) | (us as u32));
                                FUN_004223d0(((us as u32) << 16) | (us as u32));
                            }
                        }
                        0xb | 0xd => {
                            if *pent.byte_add(3) != b'Y' {
                                DAT_006a89f4 = *pent.byte_add(0x19) as u16;
                                if !DAT_006a89f8.is_null() {
                                    DAT_006a89f4 = DAT_006a89f4.wrapping_add(1);
                                    anim_step(us);
                                }
                                *ent(i26, 0x19) = DAT_006a89f4 as u8;
                            } else {
                                *pent.byte_add(0x1a) = 0;
                                let u8v = FUN_00478470();
                                if u8v < 0x800 {
                                    FUN_00422190(((us as u32) << 16) | (us as u32));
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            us = us.wrapping_add(1);
            if !((us as i32) < (DAT_00703c88 as u32).wrapping_add(DAT_00732a38 as u32) as i32) {
                break;
            }
        }
    }
}
