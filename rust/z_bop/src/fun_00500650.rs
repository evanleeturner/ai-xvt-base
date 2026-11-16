// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b6a606: [u8; 0];
    static mut DAT_00b6a5aa: [u8; 0];
    static mut DAT_00b6bace: [u8; 0];
    static mut DAT_00b6baca: [u8; 0];
    static mut DAT_00b69d20: [i8; 0];
    static mut DAT_00b6bba6: [u8; 0];
    static mut DAT_00b6bbb2: [u8; 0];
    static mut DAT_00b6a5a6: [u8; 0];
    static mut DAT_00b6bbae: [u8; 0];
    static mut DAT_00bb00ee: [u8; 0];
    static mut DAT_00bb00ea: [u8; 0];
    static mut DAT_00bb00f2: [u8; 0];
    static mut DAT_00b6a2c0: u32;
    static mut DAT_00b69ccc: u32;
    static mut DAT_00b69e30: i32;
    static mut DAT_0066d920: [u32; 0];
    static mut DAT_0066d968: [i32; 0];
    static mut DAT_0066d8d8: [i32; 0];
    static mut DAT_0066da4c: i32;
    static mut DAT_00b6d84a: [u8; 0];
    static mut DAT_00b6d85a: [u8; 0];
    static mut DAT_00b6d85e: [u8; 0];
    static mut DAT_00b6d862: [u8; 0];
    static mut DAT_00aa47ba: [i8; 0];
    static mut DAT_00aa5aac: i16;
    static mut DAT_00b6c042: [u8; 0];
    static mut DAT_00a91d24: [i8; 0];
    static mut DAT_00a91d18: [i8; 0];
    static mut DAT_00a91cc0: [i8; 0];
    static mut DAT_0066d990: [i32; 0];
    static mut DAT_00b6d5a6: [u8; 0];
    static mut DAT_00b6d598: [u32; 0];
    static mut DAT_00b6d5de: [u8; 0];
    static mut DAT_00b6d5ae: [u8; 0];
    static mut DAT_00b6d5aa: [u8; 0];
    static mut DAT_00b6d5b2: [u8; 0];
    static mut DAT_00b6d5b6: [u8; 0];
    static mut DAT_00b6d5ba: [u8; 0];
    static mut DAT_00b6d5c6: [u8; 0];
    static mut DAT_00b6a270: [u8; 0];
    static mut DAT_0066d9c0: [i32; 0];
    static mut DAT_00b6bcc2: [u8; 0];
    static mut DAT_00b6bce2: [u8; 0];
    static mut DAT_00b6bd02: [u8; 0];
    static mut DAT_00b6bdc2: [u8; 0];
    static mut DAT_00b6be82: [u8; 0];
    static mut DAT_00b6bea2: [u8; 0];
    static mut DAT_00b6bec2: [u8; 0];
    static mut DAT_00b6bf82: [u8; 0];

    static mut DAT_0052d4cc: [i8; 0];
    static mut DAT_0052b460: [i8; 0];
    static mut DAT_0052a6b0: [i8; 0];
    static mut DAT_0052d4b8: [i8; 0];
    static mut DAT_0052d4ac: [i8; 0];
    static mut DAT_0052d4a0: [i8; 0];
    static mut DAT_0052b464: [i8; 0];
    static mut DAT_0052d490: [i8; 0];
    static mut DAT_0052d488: [i8; 0];
    static mut DAT_0052d478: [i8; 0];
    static mut DAT_0052b450: [i8; 0];

    fn sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dca90(param_1: i32) -> u8;
    fn FUN_004b5570(param_1: *mut i8, param_2: i32, param_3: i32);
    fn FUN_004d6460(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_004ddc10(param_1: i32) -> *mut i8;
    fn FUN_004db640(param_1: i32, param_2: *mut i8, param_3: *mut i32, param_4: u32);
    fn FUN_004db4e0(param_1: i32, param_2: *mut i8, param_3: i32, param_4: i32, param_5: u32);
    fn FUN_004d4cf0(param_1: *mut u32);
    fn FUN_004d4d20(param_1: *mut core::ffi::c_void);
    fn FUN_004d11e0() -> i32;
}

macro_rules! adr {
    ($x:ident, $t:ty) => {
        (&raw mut $x).cast::<$t>()
    };
}

macro_rules! fm {
    ($x:ident) => {
        (&raw mut $x).cast::<i8>() as *const i8
    };
}

macro_rules! rdat {
    ($x:ident, $off:expr) => {
        rd(at(adr!($x, u8), $off))
    };
}

#[inline(always)]
unsafe fn rd(p: *const u8) -> i32 {
    // SAFETY: the caller passes an address inside the game's own memory.
    unsafe { p.cast::<i32>().read_unaligned() }
}

#[inline(always)]
fn at(p: *const u8, off: u32) -> *const u8 {
    p.wrapping_add(off as usize)
}

#[unsafe(no_mangle)]
#[allow(
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    unused_variables
)]
pub unsafe extern "C" fn FUN_00500650(param_1: i32) -> u32 {
    // SAFETY: all globals and callees are the game's own; pointers are
    // read and written exactly where the original does.
    unsafe {
        let buf = adr!(DAT_00b69d20, i8);
        let mut local_20 = [0i32; 4];
        let mut local_10 = [0u32; 4];
        let mut local_30 = [0u32; 3];
        let l20 = local_20.as_mut_ptr();
        let l10 = local_10.as_mut_ptr();

        if FUN_004dca90(0x12) != 0 && FUN_004dca90(0x10) != 0 && FUN_004dca90(0x11) != 0 {
            FUN_004b5570(adr!(DAT_0052d4cc, i8), 0xb8, 0x16a);
        }
        FUN_004d6460(l20, 0x54, 0x5a, 0x1b2, 0x6a);

        let pc3: *mut i8;
        let t0 = rd(adr!(DAT_00b6a606, u8));
        if t0 == 0 {
            pc3 = FUN_004ddc10(0x137);
        } else {
            let t0 = rd(adr!(DAT_00b6a5aa, u8));
            if t0 == 1 {
                let t1 = rd(adr!(DAT_00b6bace, u8));
                let t2 = rd(adr!(DAT_00b6baca, u8));
                if t1.wrapping_sub(t2) == 1 {
                    let pc3 = FUN_004ddc10(0xce);
                    let u14: u32 = 4;
                    let pc5 = FUN_004ddc10(0x2af);
                    sprintf(buf, fm!(DAT_0052a6b0), 1i32, pc5, u14, pc3);
                } else {
                    let pc3 = FUN_004ddc10(0x154);
                    let i10 = rd(adr!(DAT_00b6bace, u8));
                    let pc5 = FUN_004ddc10(0x14f);
                    let mut i12 = rd(adr!(DAT_00b6baca, u8));
                    i12 = i12.wrapping_add(1);
                    let u14: u32 = 4;
                    let pc6 = FUN_004ddc10(0x2af);
                    sprintf(buf, fm!(DAT_0052d4b8), 1i32, pc6, u14, i12, pc5, i10, pc3);
                }
            } else if t0 == 3 {
                local_30 = [0; 3];
                let t1 = rd(adr!(DAT_00b6bba6, u8));
                if -1 < t1 {
                    let mut p4 = adr!(DAT_00b6bbb2, u8) as *const u8;
                    let mut i10 = t1.wrapping_add(1);
                    loop {
                        let i12 = rd(p4);
                        p4 = p4.wrapping_add(4);
                        let q = local_30.as_mut_ptr().offset(i12 as isize);
                        *q = (*q).wrapping_add(1);
                        i10 = i10.wrapping_sub(1);
                        if i10 == 0 {
                            break;
                        }
                    }
                }
                let t1 = rd(adr!(DAT_00b6a5a6, u8));
                let i10: i32 = if t1 == 1 { 1 } else { 0 };
                let mut p11 = rd(adr!(DAT_00b6bbae, u8)) as u32;
                if p11 == local_30[0] || p11 == local_30[1] {
                    let pc3 = FUN_004ddc10(0xce);
                    let u14: u32 = 4;
                    let pc5 = FUN_004ddc10(0x2b1);
                    sprintf(buf, fm!(DAT_0052a6b0), 1i32, pc5, u14, pc3);
                } else {
                    let pc3 = FUN_004ddc10(0x2b2);
                    p11 = rd(adr!(DAT_00b6bbae, u8)) as u32;
                    let pc5 = FUN_004ddc10(0x14f);
                    let p4 = *local_30.as_mut_ptr().offset(i10 as isize);
                    let u14: u32 = 4;
                    let pc6 = FUN_004ddc10(0x2b1);
                    sprintf(buf, fm!(DAT_0052d4b8), 1i32, pc6, u14, p4, pc5, p11, pc3);
                }
            } else {
                let t1 = rd(adr!(DAT_00bb00ee, u8));
                let t2 = rd(adr!(DAT_00bb00ea, u8));
                let t3 = rd(adr!(DAT_00bb00f2, u8));
                if t1 == t2 && t3 == 1 {
                    let pc3 = FUN_004ddc10(0xce);
                    let u14: u32 = 4;
                    let pc5 = FUN_004ddc10(0x311);
                    sprintf(buf, fm!(DAT_0052a6b0), 1i32, pc5, u14, pc3);
                } else {
                    let i10 = t2.wrapping_add(1);
                    let pc3 = FUN_004ddc10(0x158);
                    let u14: u32 = 4;
                    let pc5 = FUN_004ddc10(0x311);
                    sprintf(buf, fm!(DAT_0052d4ac), 1i32, pc5, u14, pc3, i10);
                }
            }
            pc3 = buf;
        }
        FUN_004db640(0xf, pc3, l20, 0xffff);

        let mut i12: i32 = 0x72;
        let mut i10: i32 = 0xe4;
        let mut u16v = DAT_00b6a2c0;
        let pc3 = FUN_004ddc10(0x14a);
        FUN_004db4e0(0xc, pc3, i10, i12, u16v);
        i12 = 0x72;
        i10 = 0x12a;
        u16v = DAT_00b6a2c0;
        let pc3 = FUN_004ddc10(0x150);
        FUN_004db4e0(0xc, pc3, i10, i12, u16v);
        let mut i13: i32 = 0x72;
        i12 = 0x16b;
        i10 = 0x86;
        u16v = DAT_00b6a2c0;
        let pc3 = FUN_004ddc10(0x21b);
        FUN_004db4e0(0xc, pc3, i12, i13, u16v);

        let d8d8 = adr!(DAT_0066d8d8, i32);
        let d968 = adr!(DAT_0066d968, i32);
        let mut pu8 = adr!(DAT_0066d920, u32);
        let end8 = pu8.wrapping_add(10);
        let mut local_34: i32 = 0;
        let mut local_24: i32 = 0;
        let mut local_38: i32 = rdat!(DAT_00b6d84a, (*pu8).wrapping_mul(28));

        loop {
            i12 = 0x58;
            let uv = *pu8;
            if uv == 0xffff_ffff {
                break;
            }
            if *d968.add(uv as usize) != 0 {
                let t0 = rdat!(DAT_00b6d84a, uv.wrapping_mul(28));
                if local_38 != t0 {
                    local_34 = local_24;
                    local_38 = rdat!(DAT_00b6d84a, uv.wrapping_mul(28));
                }
                let u2: i32 = if 2 < local_34 { 1 } else { 2 };
                if DAT_0066da4c == 0 {
                    let t0 = rd(adr!(DAT_00b6a5aa, u8));
                    if t0 == 1 || t0 == 2 {
                        sprintf(
                            buf,
                            fm!(DAT_0052d4a0),
                            u2,
                            local_34.wrapping_add(1),
                            1i32,
                            at(adr!(DAT_00aa47ba, u8), uv.wrapping_mul(0x1e5)),
                        );
                        FUN_004db4e0(0xc, buf, 0x58, i10, 0xffff);
                        let t0 = rdat!(DAT_00b6d84a, (*pu8).wrapping_mul(28));
                        sprintf(buf, fm!(DAT_0052b460), t0);
                        FUN_004db4e0(0xc, buf, 0xe4, i10, 0xffff);
                        let t0 = rdat!(DAT_00b6d85a, (*pu8).wrapping_mul(28));
                        let t1 = rdat!(DAT_00b6d85e, (*pu8).wrapping_mul(28));
                        sprintf(buf, fm!(DAT_0052b464), t0, t1);
                        FUN_004db4e0(0xc, buf, 0x12a, i10, 0xffff);
                        let t0 = rdat!(DAT_00b6d862, (*pu8).wrapping_mul(28));
                        sprintf(buf, fm!(DAT_0052b460), t0);
                        i12 = 0x67;
                        FUN_004db4e0(0xc, buf, 0x16b, i10, 0xffff);
                        i10 = i10.wrapping_add(0xf);
                    } else if *d8d8.add(uv as usize) == 0 {
                        sprintf(
                            buf,
                            fm!(DAT_0052d4a0),
                            u2,
                            local_34.wrapping_add(1),
                            1i32,
                            at(adr!(DAT_00aa47ba, u8), uv.wrapping_mul(0x1e5)),
                        );
                        i12 = 0x67;
                        FUN_004db4e0(0xc, buf, 0x58, i10, 0xffff);
                        i10 = i10.wrapping_add(0xf);
                    } else {
                        sprintf(
                            buf,
                            fm!(DAT_0052d4a0),
                            u2,
                            local_34.wrapping_add(1),
                            1i32,
                            at(adr!(DAT_00aa47ba, u8), uv.wrapping_mul(0x1e5)),
                        );
                        FUN_004db4e0(0xc, buf, 0x58, i10, 0xffff);
                        let t0 = rdat!(DAT_00b6d84a, (*pu8).wrapping_mul(28));
                        sprintf(buf, fm!(DAT_0052b460), t0);
                        FUN_004db4e0(0xc, buf, 0xe4, i10, 0xffff);
                        let t0 = rdat!(DAT_00b6d85a, (*pu8).wrapping_mul(28));
                        let t1 = rdat!(DAT_00b6d85e, (*pu8).wrapping_mul(28));
                        sprintf(buf, fm!(DAT_0052b464), t0, t1);
                        FUN_004db4e0(0xc, buf, 0x12a, i10, 0xffff);
                        let t0 = rdat!(DAT_00b6d862, (*pu8).wrapping_mul(28));
                        sprintf(buf, fm!(DAT_0052b460), t0);
                        i12 = 0x67;
                        FUN_004db4e0(0xc, buf, 0x16b, i10, 0xffff);
                        i10 = i10.wrapping_add(0xf);
                    }
                } else if *d8d8.add(uv as usize) != 0 && (DAT_00aa5aac as i32) > 0 {
                    i13 = 0;
                    let mut p30 = adr!(DAT_00b6c042, u8) as *const u8;
                    let mut pc3 = adr!(DAT_00a91d24, i8) as *const i8;
                    let mut i9 = i10;
                    loop {
                        i10 = i9;
                        if *pc3 != 0 && (*pc3.wrapping_sub(0xc) as u8 as u32) == *pu8 {
                            let pc5 = pc3.wrapping_sub(100);
                            let u14: u32 = 1;
                            let t0 = rd(p30);
                            let pc6 = FUN_004ddc10(t0.wrapping_add(0x9a));
                            sprintf(
                                buf,
                                fm!(DAT_0052d490),
                                u2,
                                local_34.wrapping_add(1),
                                6i32,
                                pc6,
                                u14,
                                pc5,
                            );
                            FUN_004db4e0(0xc, buf, i12, i9, 0xffff);
                            let t0 = rdat!(DAT_00b6d84a, (*pu8).wrapping_mul(28));
                            sprintf(buf, fm!(DAT_0052b460), t0);
                            FUN_004db4e0(0xc, buf, 0xe4, i9, 0xffff);
                            let t0 = rdat!(DAT_00b6d85a, (*pu8).wrapping_mul(28));
                            let t1 = rdat!(DAT_00b6d85e, (*pu8).wrapping_mul(28));
                            sprintf(buf, fm!(DAT_0052b464), t0, t1);
                            i12 = 0x16b;
                            FUN_004db4e0(0xc, buf, 0x12a, i9, 0xffff);
                            let t0 = rdat!(DAT_00b6d862, (*pu8).wrapping_mul(28));
                            sprintf(buf, fm!(DAT_0052b460), t0);
                            i10 = i9.wrapping_add(0xf);
                            FUN_004db4e0(0xc, buf, 0x16b, i9, 0xffff);
                        }
                        p30 = p30.wrapping_add(4);
                        pc3 = pc3.wrapping_add(0x562);
                        i13 = i13.wrapping_add(1);
                        i9 = i10;
                        if !(i13 < DAT_00aa5aac as i32) {
                            break;
                        }
                    }
                }

                if *d8d8.add(*pu8 as usize) == 0 {
                    let mut p4 = adr!(DAT_0066d990, i32);
                    let end4 = p4.wrapping_add(8);
                    i13 = i10;
                    loop {
                        let i9 = *p4;
                        i10 = i13;
                        if i9 == -1 {
                            break;
                        }
                        let t0 = rdat!(DAT_00b6d5a6, (i9 as u32).wrapping_mul(0x58));
                        let ch =
                            *adr!(DAT_00a91d18, i8).add((t0 as u32).wrapping_mul(0x562) as usize);
                        if (ch as u8 as u32) == *pu8 {
                            FUN_004d6460(l20, i12, i13, 0xe1, i13.wrapping_add(0xe));
                            FUN_004d4cf0(l10);
                            FUN_004d4d20(l20.cast());
                            let puv17 = adr!(DAT_00b6d598, u32)
                                .wrapping_add((i9 as u32).wrapping_mul(0x16) as usize);
                            let t0 = rdat!(DAT_00b6d5de, (i9 as u32).wrapping_mul(0x58));
                            if DAT_0066da4c == 0 {
                                if t0 == 0 {
                                    let u14: u32 = 1;
                                    let t1 = rdat!(DAT_00b6d5ae, (i9 as u32).wrapping_mul(0x58));
                                    let pc3 = FUN_004ddc10(t1.wrapping_add(0x9a));
                                    sprintf(buf, fm!(DAT_0052a6b0), 6i32, pc3, u14, puv17);
                                } else {
                                    let t1 = rdat!(DAT_00b6d5ae, (i9 as u32).wrapping_mul(0x58));
                                    let pc3 = FUN_004ddc10(t1.wrapping_add(0x9a));
                                    sprintf(buf, fm!(DAT_0052d488), pc3);
                                }
                            } else if t0 == 0 {
                                let u14: u32 = 1;
                                let t1 = rdat!(DAT_00b6d5ae, (i9 as u32).wrapping_mul(0x58));
                                let pc3 = FUN_004ddc10(t1.wrapping_add(0x9a));
                                sprintf(
                                    buf,
                                    fm!(DAT_0052d490),
                                    u2,
                                    local_34.wrapping_add(1),
                                    6i32,
                                    pc3,
                                    u14,
                                    puv17,
                                );
                            } else {
                                let t1 = rdat!(DAT_00b6d5ae, (i9 as u32).wrapping_mul(0x58));
                                let pc3 = FUN_004ddc10(t1.wrapping_add(0x9a));
                                sprintf(
                                    buf,
                                    fm!(DAT_0052d478),
                                    u2,
                                    local_34.wrapping_add(1),
                                    1i32,
                                    pc3,
                                );
                            }
                            let mut uv16 = DAT_00b69ccc;
                            let t0 = rdat!(DAT_00b6d5de, (i9 as u32).wrapping_mul(0x58));
                            if t0 == 0 {
                                i10 = FUN_004d11e0();
                                let t1 = rdat!(DAT_00b6d5aa, (i9 as u32).wrapping_mul(0x58));
                                if i10 == t1 {
                                    uv16 = rd(at(
                                        adr!(DAT_00b6a270, u8),
                                        ((param_1 % 0x18) as u32 & 0xffff_fffe).wrapping_mul(2),
                                    )) as u32;
                                } else {
                                    uv16 = DAT_00b6a2c0;
                                    if DAT_00b69e30 == 2 {
                                        uv16 = rd(at(
                                            adr!(DAT_00b6a270, u8),
                                            ((param_1 % 0x18) as u32 & 0xffff_fffe).wrapping_mul(2),
                                        )) as u32;
                                    }
                                }
                            }
                            FUN_004db4e0(0xc, buf, i12, i13, uv16);
                            FUN_004d4d20(l10.cast());
                            let t0 = rd(adr!(DAT_00b6a5aa, u8));
                            let u14: u32;
                            if (t0 == 1 || t0 == 2) && DAT_0066da4c != 0 {
                                u14 = rdat!(DAT_00b6d84a, (*pu8).wrapping_mul(28)) as u32;
                            } else {
                                u14 = rdat!(DAT_00b6d5b2, (i9 as u32).wrapping_mul(0x58)) as u32;
                            }
                            sprintf(buf, fm!(DAT_0052b460), u14);
                            FUN_004db4e0(0xc, buf, 0xe4, i13, 0xffff);
                            let u14: u32;
                            if DAT_0066da4c == 0 {
                                let t0 = rdat!(DAT_00b6d5b6, (i9 as u32).wrapping_mul(0x58));
                                let t1 = rdat!(DAT_00b6d5ba, (i9 as u32).wrapping_mul(0x58));
                                sprintf(buf, fm!(DAT_0052b464), t0, t1);
                                FUN_004db4e0(0xc, buf, 0x12a, i13, 0xffff);
                                u14 = rdat!(DAT_00b6d5c6, (i9 as u32).wrapping_mul(0x58)) as u32;
                            } else {
                                let t0 = rdat!(DAT_00b6d85a, (*pu8).wrapping_mul(28));
                                let t1 = rdat!(DAT_00b6d85e, (*pu8).wrapping_mul(28));
                                sprintf(buf, fm!(DAT_0052b464), t0, t1);
                                FUN_004db4e0(0xc, buf, 0x12a, i13, 0xffff);
                                u14 = rdat!(DAT_00b6d862, (*pu8).wrapping_mul(28)) as u32;
                            }
                            sprintf(buf, fm!(DAT_0052b460), u14);
                            i10 = i13.wrapping_add(0xf);
                            FUN_004db4e0(0xc, buf, 0x16b, i13, 0xffff);
                            i12 = ((0u32.wrapping_sub((DAT_0066da4c == 0) as u32) & 0xf)
                                .wrapping_add(0x58)) as i32;
                        }
                        p4 = p4.wrapping_add(1);
                        i13 = i10;
                        if !(p4 < end4) {
                            break;
                        }
                    }
                }

                let t0 = rd(adr!(DAT_00b6a5aa, u8));
                if (t0 == 1 || t0 == 2) && DAT_0066da4c == 0 && (DAT_00aa5aac as i32) > 0 {
                    i13 = 0;
                    let mut p30 = adr!(DAT_00b6c042, u8) as *const u8;
                    let mut pc3 = adr!(DAT_00a91d24, i8) as *const i8;
                    let mut i9 = i10;
                    loop {
                        i10 = i9;
                        if *pc3 != 0 && (*pc3.wrapping_sub(0xc) as u8 as u32) == *pu8 {
                            let mut i7: i32 = 0;
                            loop {
                                let t0 = rdat!(DAT_00b6d5aa, (i7 as u32).wrapping_mul(0x58));
                                let t1 = rd(at(
                                    adr!(DAT_00b6d5aa, u8),
                                    (i7 as u32).wrapping_mul(0x58).wrapping_sub(4),
                                ));
                                if t0 != 0 && t1 == i13 {
                                    break;
                                }
                                i7 = i7.wrapping_add(1);
                                if !(i7 < 8) {
                                    break;
                                }
                            }
                            if i7 == 8 {
                                let pc5 = pc3.wrapping_sub(100);
                                let u14: u32 = 1;
                                let t0 = rd(p30);
                                let pc6 = FUN_004ddc10(t0.wrapping_add(0x9a));
                                sprintf(buf, fm!(DAT_0052a6b0), 6i32, pc6, u14, pc5);
                                i10 = i9.wrapping_add(0xf);
                                FUN_004db4e0(0xc, buf, i12, i9, 0xffff);
                            }
                        }
                        p30 = p30.wrapping_add(4);
                        pc3 = pc3.wrapping_add(0x562);
                        i13 = i13.wrapping_add(1);
                        i9 = i10;
                        if !(i13 < DAT_00aa5aac as i32) {
                            break;
                        }
                    }
                }
            }
            pu8 = pu8.wrapping_add(1);
            local_24 = local_24.wrapping_add(1);
            if !(pu8 < end8) {
                break;
            }
        }

        let i9 = i10.wrapping_add(0xf);
        i10 = i10.wrapping_add(0x1e);
        i13 = i9;
        u16v = DAT_00b6a2c0;
        let pc3 = FUN_004ddc10(0x21e);
        FUN_004db4e0(0xc, pc3, i12, i13, u16v);
        let mut p30 = adr!(DAT_0066d9c0, i32);
        let end30 = p30.wrapping_add(8);
        i12 = i10;
        loop {
            i13 = *p30;
            if i13 == -1 {
                break;
            }
            if i13 < 8 {
                let i7 = FUN_004d11e0();
                let t0 = rdat!(DAT_00b6d5aa, (i13 as u32).wrapping_mul(0x58));
                let t1 = rdat!(DAT_00b6bcc2, (i13 as u32).wrapping_mul(4));
                let t2 = rdat!(DAT_00b6bce2, (i13 as u32).wrapping_mul(4));
                if i7 != t0 && (t1 != 0 || t2 != 0) {
                    FUN_004d6460(l20, 0x58, i12, 0xce, i12.wrapping_add(0xe));
                    FUN_004d4cf0(l10);
                    FUN_004d4d20(l20.cast());
                    let t0 = rdat!(DAT_00b6d5de, (i13 as u32).wrapping_mul(0x58));
                    let uv16: u32;
                    let puv17 = adr!(DAT_00b6d598, u32)
                        .wrapping_add((i13 as u32).wrapping_mul(0x16) as usize);
                    if t0 == 0 {
                        let u14: u32 = 4;
                        let t1 = rdat!(DAT_00b6d5ae, (i13 as u32).wrapping_mul(0x58));
                        let pc3 = FUN_004ddc10(t1.wrapping_add(0x9a));
                        sprintf(buf, fm!(DAT_0052a6b0), 6i32, pc3, u14, puv17);
                        uv16 = 0xffff;
                    } else {
                        let t1 = rdat!(DAT_00b6d5ae, (i13 as u32).wrapping_mul(0x58));
                        let pc3 = FUN_004ddc10(t1.wrapping_add(0x9a));
                        sprintf(buf, fm!(DAT_0052d488), pc3, puv17);
                        uv16 = DAT_00b69ccc;
                    }
                    FUN_004db4e0(0xc, buf, 0x58, i12, uv16);
                    FUN_004d4d20(l10.cast());
                    let u14 = rdat!(DAT_00b6bce2, (i13 as u32).wrapping_mul(4)) as u32;
                    let u15 = rdat!(DAT_00b6bcc2, (i13 as u32).wrapping_mul(4)) as u32;
                    sprintf(buf, fm!(DAT_0052b450), u15, u14);
                    FUN_004db4e0(0xc, buf, 0xd0, i12, 0xffff);
                    i12 = i12.wrapping_add(0xf);
                }
            } else {
                let i7 = (i13 as u32).wrapping_sub(8);
                let idx = i7.wrapping_mul(0x52).wrapping_add(
                    ((i13 as u32).wrapping_mul(9).wrapping_sub(0x48)).wrapping_mul(0x90),
                );
                i13 = idx as i32;
                let t0 = rdat!(DAT_00b6bd02, i7.wrapping_mul(4));
                let t1 = rdat!(DAT_00b6bdc2, i7.wrapping_mul(4));
                let ch = *adr!(DAT_00a91d18, i8).add(idx as usize);
                if *d8d8.add(ch as u8 as usize) != 0 && (t0 != 0 || t1 != 0) {
                    let p13 = (adr!(DAT_00a91cc0, i8) as usize as u32).wrapping_add(i13 as u32);
                    let u14: u32 = 4;
                    let t2 = rdat!(DAT_00b6c042, i7.wrapping_mul(4));
                    let pc3 = FUN_004ddc10(t2.wrapping_add(0x9a));
                    sprintf(buf, fm!(DAT_0052a6b0), 6i32, pc3, u14, p13);
                    FUN_004db4e0(0xc, buf, 0x58, i12, 0xffff);
                    let u14 = rdat!(DAT_00b6bdc2, i7.wrapping_mul(4)) as u32;
                    let u15 = rdat!(DAT_00b6bd02, i7.wrapping_mul(4)) as u32;
                    sprintf(buf, fm!(DAT_0052b450), u15, u14);
                    FUN_004db4e0(0xc, buf, 0xd0, i12, 0xffff);
                    i12 = i12.wrapping_add(0xf);
                }
            }
            p30 = p30.wrapping_add(1);
            if !(p30 < end30) {
                break;
            }
        }

        i12 = 0x103;
        let mut pi4 = adr!(DAT_0066d9c0, i32);
        u16v = DAT_00b6a2c0;
        let pc3 = FUN_004ddc10(0x21d);
        FUN_004db4e0(0xc, pc3, i12, i9, u16v);
        let end4 = pi4.wrapping_add(8);
        loop {
            let e = *pi4;
            if e == -1 {
                return 1;
            }
            if e < 8 {
                let i13 = FUN_004d11e0();
                let t0 = rdat!(DAT_00b6d5aa, (e as u32).wrapping_mul(0x58));
                let t1 = rdat!(DAT_00b6be82, (e as u32).wrapping_mul(4));
                let t2 = rdat!(DAT_00b6bea2, (e as u32).wrapping_mul(4));
                if i13 != t0 && (t1 != 0 || t2 != 0) {
                    FUN_004d6460(l20, 0x103, i10, 0x179, i10.wrapping_add(0xe));
                    FUN_004d4cf0(l10);
                    FUN_004d4d20(l20.cast());
                    let puv17 = adr!(DAT_00b6d598, u32)
                        .wrapping_add((e as u32).wrapping_mul(0x16) as usize);
                    let t0 = rdat!(DAT_00b6d5de, (e as u32).wrapping_mul(0x58));
                    let uv16: u32;
                    if t0 == 0 {
                        let u14: u32 = 4;
                        let t1 = rdat!(DAT_00b6d5ae, (e as u32).wrapping_mul(0x58));
                        let pc3 = FUN_004ddc10(t1.wrapping_add(0x9a));
                        sprintf(buf, fm!(DAT_0052a6b0), 6i32, pc3, u14, puv17);
                        uv16 = 0xffff;
                    } else {
                        let t1 = rdat!(DAT_00b6d5ae, (e as u32).wrapping_mul(0x58));
                        let pc3 = FUN_004ddc10(t1.wrapping_add(0x9a));
                        sprintf(buf, fm!(DAT_0052d488), pc3);
                        uv16 = DAT_00b69ccc;
                    }
                    FUN_004db4e0(0xc, buf, 0x103, i10, uv16);
                    FUN_004d4d20(l10.cast());
                    let u14 = rdat!(DAT_00b6bea2, (e as u32).wrapping_mul(4)) as u32;
                    let u15 = rdat!(DAT_00b6be82, (e as u32).wrapping_mul(4)) as u32;
                    sprintf(buf, fm!(DAT_0052b450), u15, u14);
                    FUN_004db4e0(0xc, buf, 0x17b, i10, 0xffff);
                    i10 = i10.wrapping_add(0xf);
                }
            } else {
                let i13 = (e as u32).wrapping_sub(8);
                let idx = i13.wrapping_mul(0x52).wrapping_add(
                    ((e as u32).wrapping_mul(9).wrapping_sub(0x48)).wrapping_mul(0x90),
                );
                let t0 = rdat!(DAT_00b6bec2, i13.wrapping_mul(4));
                let t1 = rdat!(DAT_00b6bf82, i13.wrapping_mul(4));
                let ch = *adr!(DAT_00a91d18, i8).add(idx as usize);
                if *d8d8.add(ch as u8 as usize) != 0 && (t0 != 0 || t1 != 0) {
                    let p12 = (adr!(DAT_00a91cc0, i8) as usize as u32).wrapping_add(idx);
                    let u14: u32 = 4;
                    let t2 = rdat!(DAT_00b6c042, i13.wrapping_mul(4));
                    let pc3 = FUN_004ddc10(t2.wrapping_add(0x9a));
                    sprintf(buf, fm!(DAT_0052a6b0), 6i32, pc3, u14, p12);
                    FUN_004db4e0(0xc, buf, 0x103, i10, 0xffff);
                    let u14 = rdat!(DAT_00b6bf82, i13.wrapping_mul(4)) as u32;
                    let u15 = rdat!(DAT_00b6bec2, i13.wrapping_mul(4)) as u32;
                    sprintf(buf, fm!(DAT_0052b450), u15, u14);
                    FUN_004db4e0(0xc, buf, 0x17b, i10, 0xffff);
                    i10 = i10.wrapping_add(0xf);
                }
            }
            pi4 = pi4.wrapping_add(1);
            if end4 <= pi4 {
                return 1;
            }
        }
    }
}
