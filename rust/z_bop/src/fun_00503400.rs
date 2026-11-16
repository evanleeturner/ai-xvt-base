// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0066da18: [u32; 0];
    static mut DAT_0066da3c: u32;
    static mut DAT_0066da4c: i32;
    static mut DAT_0066d8d8: [i32; 0];
    static mut DAT_0066d8a0: [u32; 0];
    static mut DAT_0066d968: [u32; 0];
    static mut DAT_0066d990: [i32; 0];
    static mut DAT_0066d9b0: i32;
    static mut DAT_00aa5aac: i16;
    static mut DAT_00aa47ba: [u8; 0];
    static mut DAT_00a91d18: [u8; 0];
    static mut DAT_00a91d24: [i8; 0];
    static mut DAT_00a91b90: [i8; 0];
    static mut DAT_00b69d20: [i8; 0];
    static mut DAT_00b69e30: i32;
    static mut DAT_00b6a2c0: u32;
    static mut DAT_00b6a270: [u8; 0];
    static mut DAT_00b6a5a6: u8;
    static mut DAT_00b6baca: u8;
    static mut DAT_00b6bace: u8;
    static mut DAT_00b6bad6: [u8; 0];
    static mut DAT_00b6bada: [u8; 0];
    static mut DAT_00b6bade: [u8; 0];
    static mut DAT_00b6bae2: [u8; 0];
    static mut DAT_00b6c042: [u8; 0];
    static mut DAT_00b6d598: [u32; 0];
    static mut DAT_00b6d5a6: [u8; 0];
    static mut DAT_00b6d5aa: [u8; 0];
    static mut DAT_00b6d5ae: [u8; 0];
    static mut DAT_00b6d84a: [u8; 0];
    static mut DAT_00b6d866: u8;
    static mut DAT_00b6d86a: u8;
    static mut DAT_00b6d962: u8;
    static mut DAT_00b6da16: [u8; 0];
    static mut DAT_0052b460: [i8; 0];
    static mut DAT_0052b4d4: [i8; 0];
    static mut DAT_0052adb0: [i8; 0];
    static mut DAT_0052d504: [i8; 0];
    static mut DAT_0052b49c: [i8; 0];
    static mut DAT_0052d530: [i8; 0];
    static mut DAT_0052d514: [i8; 0];
    static mut DAT_0052d4a0: [i8; 0];
    static mut DAT_0052d490: [i8; 0];
    static mut DAT_0052a6b0: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d6460(rect: *mut i32, a: i32, b: i32, c: i32, d: i32);
    fn FUN_004ddc10(id: i32) -> *mut i8;
    fn FUN_004db640(font: i32, text: *mut i8, rect: *mut i32, color: u32);
    fn FUN_004d64a0(rect: *mut i32, a: i32, b: i32);
    fn FUN_004dbfa0(text: *mut i8, font: i32) -> i32;
    fn FUN_004b4cc0(name: *mut i8, out: *mut i32);
    fn FUN_004db4e0(font: i32, text: *mut i8, x: i32, y: i32, color: u32);
    fn FUN_004b5570(name: *mut i8, x: i32, y: i32);
    fn FUN_004d11e0() -> i32;
    fn FUN_004d4cf0(p: *mut i32);
    fn FUN_004d4d20(p: *mut i32);
    fn sprintf(dst: *mut i8, fmt: *const i8, ...) -> i32;
}

macro_rules! p8 {
    ($x:ident) => {
        (&raw mut $x).cast::<u8>()
    };
}
macro_rules! pc {
    ($x:ident) => {
        (&raw mut $x).cast::<i8>()
    };
}
macro_rules! p32 {
    ($x:ident) => {
        (&raw mut $x).cast::<u32>()
    };
}

unsafe fn rd(p: *mut u8) -> i32 {
    // SAFETY: reads the game's own memory at the address the original reads.
    unsafe { p.cast::<i32>().read_unaligned() }
}

unsafe fn ru(p: *mut u8) -> u32 {
    // SAFETY: reads the game's own memory at the address the original reads.
    unsafe { p.cast::<u32>().read_unaligned() }
}

fn at(p: *mut u8, off: u32) -> *mut u8 {
    p.wrapping_add(off as usize)
}

unsafe fn d8d8(i: u32) -> i32 {
    // SAFETY: the game's own table, indexed as the original does.
    unsafe {
        (&raw mut DAT_0066d8d8)
            .cast::<i32>()
            .wrapping_add(i as usize)
            .read()
    }
}

unsafe fn cur(i6: i32) -> i32 {
    // SAFETY: the game's own table, read at a byte offset as the original does.
    unsafe { rd(at(p8!(DAT_0066da18), i6 as u32)) }
}

unsafe fn rdidx(base: *mut u8, i6: i32) -> i32 {
    // SAFETY: the game's own tables, read as the original does.
    unsafe {
        let c = cur(i6) as u32;
        rd(at(base, c.wrapping_mul(20)))
    }
}

unsafe fn col(param_1: i32) -> u32 {
    // SAFETY: the game's own colour table, read as the original does.
    unsafe {
        ru(at(
            p8!(DAT_00b6a270),
            ((param_1 % 0x18) as u32 & 0xfffffffe).wrapping_mul(2),
        ))
    }
}

unsafe fn pr(t0: i32) {
    // SAFETY: the game's own buffer and format strings.
    unsafe {
        if t0 == 0 {
            sprintf(pc!(DAT_00b69d20), pc!(DAT_0052b4d4));
        } else {
            sprintf(pc!(DAT_00b69d20), pc!(DAT_0052b460), t0);
        }
    }
}

unsafe fn blk(p: *mut u32, l84: *mut i32) {
    // SAFETY: the game's own globals and functions; locals live in this frame.
    unsafe {
        let mut local_64 = [0i32; 4];
        let mut local_34 = [0i8; 52];
        let ut = ru(p8!(DAT_00b6a5a6));
        let t0 = rd(p8!(DAT_00b6d962));
        let t1 = rd(at(p8!(DAT_00b6da16), (t0 as u32).wrapping_mul(0x109e0)));
        if *p == ut && t1 != 0 {
            let t0 = rd(p8!(DAT_00b6d962));
            let t1 = rd(at(p8!(DAT_00b6da16), (t0 as u32).wrapping_mul(0x109e0)));
            let pc2 = FUN_004ddc10(t1.wrapping_add(0x17d));
            let pc5 = FUN_004ddc10(0x17b);
            let pc7 = FUN_004ddc10(0x179);
            sprintf(pc!(DAT_00b69d20), pc!(DAT_0052d504), pc7, pc5, pc2);
            let i12 = FUN_004dbfa0(pc!(DAT_00b69d20), 0xc);
            let t0 = rd(p8!(DAT_00b6d962));
            let t1 = rd(at(p8!(DAT_00b6da16), (t0 as u32).wrapping_mul(0x109e0)));
            sprintf(local_34.as_mut_ptr(), pc!(DAT_0052b49c), t1);
            FUN_004b4cc0(local_34.as_mut_ptr(), local_64.as_mut_ptr());
            let a = l84.add(2).read().wrapping_sub(l84.read()) >> 1;
            let b = local_64[2]
                .wrapping_sub(local_64[0])
                .wrapping_add(i12)
                .wrapping_add(10)
                .wrapping_add(0xb)
                >> 1;
            let i6 = a.wrapping_sub(b).wrapping_add(l84.read());
            FUN_004db4e0(0xc, pc!(DAT_00b69d20), i6, l84.add(1).read(), 0xffff);
            FUN_004b5570(
                local_34.as_mut_ptr(),
                i12.wrapping_add(10).wrapping_add(i6).wrapping_add(10),
                l84.add(1).read(),
            );
            FUN_004d64a0(l84, 0, 0xf);
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00503400(param_1: i32) -> u32 {
    // SAFETY: the callees and globals are the game's own; all pointers are
    // either locals of this frame or game memory used as the original does.
    unsafe {
        let mut local_84 = [0i32; 4];
        let mut local_54 = [0i32; 4];
        let mut local_44 = [0i32; 4];
        let l84 = local_84.as_mut_ptr();
        let bufp = pc!(DAT_00b69d20);
        let mut u19: u32;
        let mut i12: i32;
        let mut i6: i32;

        FUN_004d6460(l84, 0x54, 0x5a, 0x1b2, 0x6a);
        let pc2 = FUN_004ddc10(0x13b);
        FUN_004db640(0xf, pc2, l84, 0xffff);
        FUN_004d6460(l84, 0x54, 0x6e, 0x1b2, 0x73);
        FUN_004db640(0xc, pc!(DAT_00a91b90), l84, DAT_00b6a2c0);
        FUN_004d64a0(l84, 0, 0x14);
        let t0 = rd(p8!(DAT_00b6bace));
        let t1 = rd(p8!(DAT_00b6baca));
        if t0.wrapping_sub(t1) == 1 {
            if DAT_0066da4c != 0 {
                let mut pv11 = p32!(DAT_0066da18);
                loop {
                    u19 = *pv11;
                    if u19 == 0xffffffff {
                        break;
                    }
                    if d8d8(u19) == 0 {
                        i12 = 0;
                        let mut pb21 = p8!(DAT_00b6d5aa);
                        loop {
                            let t0 = rd(pb21);
                            if t0 != 0 {
                                let t1 = rd(pb21.wrapping_sub(4));
                                let v =
                                    at(p8!(DAT_00a91d18), (t1 as u32).wrapping_mul(0x562)).read();
                                if v as u32 == u19 {
                                    let pu20 = p32!(DAT_00b6d598)
                                        .wrapping_add((i12 as u32).wrapping_mul(0x16) as usize);
                                    let u14: u32 = 1;
                                    let t2 = rd(at(
                                        p8!(DAT_00b6d5ae),
                                        (*p32!(DAT_0066da18)).wrapping_mul(0x58),
                                    ));
                                    let pc2 = FUN_004ddc10(t2.wrapping_add(0x9a));
                                    let u17: u32 = 6;
                                    let pc5 = FUN_004ddc10(0x2ae);
                                    sprintf(
                                        bufp,
                                        pc!(DAT_0052d530),
                                        2i32,
                                        pc5,
                                        u17,
                                        pc2,
                                        u14,
                                        pu20,
                                    );
                                    break;
                                }
                            }
                            pb21 = pb21.wrapping_add(0x58);
                            i12 = i12.wrapping_add(1);
                            if !(pb21 < p8!(DAT_00b6d86a)) {
                                break;
                            }
                        }
                        if i12 == 8 {
                            let p8_ = at(p8!(DAT_00aa47ba), (*pv11).wrapping_mul(0x1e5));
                            let u17: u32 = 1;
                            let pc2 = FUN_004ddc10(0x2ae);
                            sprintf(bufp, pc!(DAT_0052adb0), 2i32, pc2, u17, p8_);
                            FUN_004db640(0xf, bufp, l84, 0xffff);
                            FUN_004d64a0(l84, 0, 0x11);
                        } else {
                            u19 = DAT_00b6a2c0;
                            let ut = ru(p8!(DAT_00b6a5a6));
                            if *pv11 == ut {
                                u19 = col(param_1);
                            }
                            FUN_004db640(0xf, bufp, l84, u19);
                            FUN_004d64a0(l84, 0, 0x11);
                            blk(pv11, l84);
                        }
                    } else {
                        i12 = 0;
                        if 0 < DAT_00aa5aac {
                            let mut pb21 = p8!(DAT_00b6c042);
                            let mut pc2 = pc!(DAT_00a91d24);
                            loop {
                                if pc2.read() != 0
                                    && (pc2.wrapping_sub(0xc).read() as u8 as u32) == *pv11
                                {
                                    let pc5 = pc2.wrapping_sub(100);
                                    let u14: u32 = 1;
                                    let t0 = rd(pb21);
                                    let pc7 = FUN_004ddc10(t0.wrapping_add(0x9a));
                                    let u17: u32 = 6;
                                    let pc4 = FUN_004ddc10(0x2ae);
                                    sprintf(bufp, pc!(DAT_0052d530), 2i32, pc4, u17, pc7, u14, pc5);
                                    FUN_004db640(0xf, bufp, l84, 0xffff);
                                    FUN_004d64a0(l84, 0, 0x11);
                                }
                                pb21 = pb21.wrapping_add(4);
                                pc2 = pc2.wrapping_add(0x562);
                                i12 = i12.wrapping_add(1);
                                if !(i12 < DAT_00aa5aac as i32) {
                                    break;
                                }
                            }
                        }
                    }
                    let pv1 = pv11.wrapping_add(1);
                    let a = rd(at(p8!(DAT_00b6bad6), (*pv1).wrapping_mul(20)));
                    let b = rd(at(p8!(DAT_00b6bad6), (*pv11).wrapping_mul(20)));
                    if a < b {
                        break;
                    }
                    pv11 = pv1;
                    if (pv1 as usize) >= ((&raw mut DAT_0066da3c) as usize) {
                        break;
                    }
                }
            } else {
                let mut pv11 = p32!(DAT_0066da18);
                while *pv11 != 0xffffffff {
                    let p8_ = at(p8!(DAT_00aa47ba), (*pv11).wrapping_mul(0x1e5));
                    let u17: u32 = 1;
                    let pc2 = FUN_004ddc10(0x2ae);
                    sprintf(bufp, pc!(DAT_0052adb0), 2i32, pc2, u17, p8_);
                    FUN_004db640(0xf, bufp, l84, 0xffff);
                    FUN_004d64a0(l84, 0, 0x11);
                    blk(pv11, l84);
                    let pv1 = pv11.wrapping_add(1);
                    let a = rd(at(p8!(DAT_00b6bad6), (*pv1).wrapping_mul(20)));
                    let b = rd(at(p8!(DAT_00b6bad6), (*pv11).wrapping_mul(20)));
                    if a < b {
                        break;
                    }
                    pv11 = pv1;
                    if (pv1 as usize) > 0x66da3b {
                        break;
                    }
                }
            }
        }
        FUN_004d64a0(l84, 0, 8);
        u19 = DAT_00b6a2c0;
        let pc2 = FUN_004ddc10(0x152);
        FUN_004db640(0xc, pc2, l84, u19);
        FUN_004d64a0(l84, 0, 0xf);
        let pc2 = FUN_004ddc10(0x154);
        let u18: u32 = 4;
        let u15: u32 = 1;
        i12 = rd(p8!(DAT_00b6bace));
        let pc5 = FUN_004ddc10(0x14f);
        let u14: u32 = 4;
        let t0 = rd(p8!(DAT_00b6baca));
        i6 = t0.wrapping_add(1);
        let u17: u32 = 1;
        let pc7 = FUN_004ddc10(0x153);
        sprintf(
            bufp,
            pc!(DAT_0052d514),
            4i32,
            pc7,
            u17,
            i6,
            u14,
            pc5,
            u15,
            i12,
            u18,
            pc2,
        );
        FUN_004db640(0xc, bufp, l84, 0xffff);
        i6 = local_84[1].wrapping_add(0xf);
        u19 = DAT_00b6a2c0;
        let pc2 = FUN_004ddc10(0x148);
        FUN_004db4e0(0xc, pc2, 0x58, i6, u19);
        u19 = DAT_00b6a2c0;
        let pc2 = FUN_004ddc10(699);
        FUN_004db4e0(0xc, pc2, 0xee, i6, u19);
        u19 = DAT_00b6a2c0;
        let pc2 = FUN_004ddc10(0x13e);
        FUN_004db4e0(0xc, pc2, 0x134, i6, u19);
        u19 = DAT_00b6a2c0;
        let pc2 = FUN_004ddc10(0x13f);
        FUN_004db4e0(0xc, pc2, 0x15c, i6, u19);
        i12 = local_84[1].wrapping_add(0x1e);
        u19 = DAT_00b6a2c0;
        let pc2 = FUN_004ddc10(0x140);
        FUN_004db4e0(0xc, pc2, 0x184, i6, u19);
        let mut local_74: i32 = 0;
        let mut local_68: i32 = 0;
        let mut local_6c = rd(at(
            p8!(DAT_00b6d84a),
            (*p32!(DAT_0066da18)).wrapping_mul(0x1c),
        ));
        let mut local_70: i32;
        i6 = 0;
        loop {
            let mut i13: i32 = 0x58;
            let mut i16 = cur(i6);
            if i16 == -1 {
                return 1;
            }
            if ru(at(p8!(DAT_0066d8a0), i6 as u32)) != 0 {
                let t0 = rd(at(p8!(DAT_00b6bad6), (i16 as u32).wrapping_mul(20)));
                if t0 != local_6c {
                    local_74 = local_68;
                    local_6c = rd(at(p8!(DAT_00b6bad6), (i16 as u32).wrapping_mul(20)));
                }
                local_70 = 2;
                if 2 < local_74 {
                    local_70 = 1;
                }
                if DAT_0066da4c == 0 {
                    sprintf(
                        bufp,
                        pc!(DAT_0052d4a0),
                        local_70,
                        local_74.wrapping_add(1),
                        1i32,
                        at(p8!(DAT_00aa47ba), (i16 as u32).wrapping_mul(0x1e5)),
                    );
                    FUN_004db4e0(0xc, bufp, 0x58, i12, 0xffff);
                    let t0 = rdidx(p8!(DAT_00b6bad6), i6);
                    sprintf(bufp, pc!(DAT_0052b460), t0);
                    FUN_004db4e0(0xc, bufp, 0xee, i12, 0xffff);
                    let t0 = rdidx(p8!(DAT_00b6bada), i6);
                    pr(t0);
                    FUN_004db4e0(0xc, bufp, 0x134, i12, 0xffff);
                    let t0 = rdidx(p8!(DAT_00b6bade), i6);
                    pr(t0);
                    FUN_004db4e0(0xc, bufp, 0x15c, i12, 0xffff);
                    let t0 = rdidx(p8!(DAT_00b6bae2), i6);
                    pr(t0);
                    i13 = 0x67;
                    FUN_004db4e0(0xc, bufp, 0x184, i12, 0xffff);
                    i12 = i12.wrapping_add(0xf);
                } else if d8d8(i16 as u32) != 0 && 0 < DAT_00aa5aac {
                    let mut cnt: i32 = 0;
                    let mut pb22 = p8!(DAT_00b6c042);
                    let mut pc2 = pc!(DAT_00a91d24);
                    i16 = i12;
                    loop {
                        i12 = i16;
                        if pc2.read() != 0
                            && (pc2.wrapping_sub(0xc).read() as u8 as u32) == (cur(i6) as u32)
                        {
                            let pc5 = pc2.wrapping_sub(100);
                            let u17: u32 = 1;
                            let t0 = rd(pb22);
                            let pc7 = FUN_004ddc10(t0.wrapping_add(0x9a));
                            sprintf(
                                bufp,
                                pc!(DAT_0052d490),
                                local_70,
                                local_74.wrapping_add(1),
                                6i32,
                                pc7,
                                u17,
                                pc5,
                            );
                            FUN_004db4e0(0xc, bufp, i13, i16, 0xffff);
                            let t0 = rdidx(p8!(DAT_00b6bad6), i6);
                            sprintf(bufp, pc!(DAT_0052b460), t0);
                            FUN_004db4e0(0xc, bufp, i13.wrapping_add(0x96), i16, 0xffff);
                            let t0 = rdidx(p8!(DAT_00b6bada), i6);
                            pr(t0);
                            i12 = i13.wrapping_add(0x104);
                            FUN_004db4e0(0xc, bufp, i13.wrapping_add(0xdc), i16, 0xffff);
                            let t0 = rdidx(p8!(DAT_00b6bade), i6);
                            pr(t0);
                            i13 = i13.wrapping_add(300);
                            FUN_004db4e0(0xc, bufp, i12, i16, 0xffff);
                            let t0 = rdidx(p8!(DAT_00b6bae2), i6);
                            pr(t0);
                            i12 = i16.wrapping_add(0xf);
                            FUN_004db4e0(0xc, bufp, i13, i16, 0xffff);
                        }
                        pb22 = pb22.wrapping_add(4);
                        pc2 = pc2.wrapping_add(0x562);
                        cnt = cnt.wrapping_add(1);
                        i16 = i12;
                        if !(cnt < DAT_00aa5aac as i32) {
                            break;
                        }
                    }
                }
                if d8d8(cur(i6) as u32) == 0 {
                    let mut p8c = (&raw mut DAT_0066d990).cast::<i32>();
                    i16 = i12;
                    loop {
                        let i10 = p8c.read();
                        i12 = i16;
                        if i10 == -1 {
                            break;
                        }
                        let t0 = rd(at(p8!(DAT_00b6d5a6), (i10 as u32).wrapping_mul(0x58)));
                        let v = at(p8!(DAT_00a91d18), (t0 as u32).wrapping_mul(0x562)).read();
                        if (v as u32) == (cur(i6) as u32) {
                            let pu20 = p32!(DAT_00b6d598)
                                .wrapping_add((i10 as u32).wrapping_mul(0x16) as usize);
                            let u17: u32 = 1;
                            if DAT_0066da4c == 0 {
                                let t1 = rd(at(p8!(DAT_00b6d5ae), (i10 as u32).wrapping_mul(0x58)));
                                let pc2 = FUN_004ddc10(t1.wrapping_add(0x9a));
                                sprintf(bufp, pc!(DAT_0052a6b0), 6i32, pc2);
                                i12 = FUN_004d11e0();
                                let t2 = rd(at(p8!(DAT_00b6d5aa), (i10 as u32).wrapping_mul(0x58)));
                                if i12 == t2 || {
                                    u19 = DAT_00b6a2c0;
                                    DAT_00b69e30 == 2
                                } {
                                    u19 = col(param_1);
                                }
                            } else {
                                let t1 = rd(at(p8!(DAT_00b6d5ae), (i10 as u32).wrapping_mul(0x58)));
                                let pc2 = FUN_004ddc10(t1.wrapping_add(0x9a));
                                sprintf(
                                    bufp,
                                    pc!(DAT_0052d490),
                                    local_70,
                                    local_74.wrapping_add(1),
                                    6i32,
                                    pc2,
                                    u17,
                                    pu20,
                                );
                                FUN_004d6460(
                                    local_54.as_mut_ptr(),
                                    i13,
                                    i16,
                                    i13.wrapping_add(0x94),
                                    i16.wrapping_add(0xf),
                                );
                                FUN_004d4cf0(local_44.as_mut_ptr());
                                FUN_004d4d20(local_54.as_mut_ptr());
                                i12 = FUN_004d11e0();
                                let t2 = rd(at(p8!(DAT_00b6d5aa), (i10 as u32).wrapping_mul(0x58)));
                                if i12 == t2 || {
                                    u19 = DAT_00b6a2c0;
                                    DAT_00b69e30 == 2
                                } {
                                    u19 = col(param_1);
                                }
                                FUN_004db4e0(0xc, bufp, i13, i16, u19);
                                FUN_004d4d20(local_44.as_mut_ptr());
                                let t0 = rdidx(p8!(DAT_00b6bad6), i6);
                                sprintf(bufp, pc!(DAT_0052b460), t0);
                                FUN_004db4e0(0xc, bufp, i13.wrapping_add(0x96), i16, 0xffff);
                                let t0 = rdidx(p8!(DAT_00b6bada), i6);
                                pr(t0);
                                i12 = i13.wrapping_add(0x104);
                                FUN_004db4e0(0xc, bufp, i13.wrapping_add(0xdc), i16, 0xffff);
                                let t0 = rdidx(p8!(DAT_00b6bade), i6);
                                pr(t0);
                                i13 = i13.wrapping_add(300);
                                FUN_004db4e0(0xc, bufp, i12, i16, 0xffff);
                                let t0 = rdidx(p8!(DAT_00b6bae2), i6);
                                pr(t0);
                                u19 = 0xffff;
                            }
                            i12 = i16.wrapping_add(0xf);
                            FUN_004db4e0(0xc, bufp, i13, i16, u19);
                            i13 = ((0u32.wrapping_sub((DAT_0066da4c == 0) as u32)) & 0xf)
                                .wrapping_add(0x58) as i32;
                        }
                        p8c = p8c.wrapping_add(1);
                        i16 = i12;
                        if !(p8c < (&raw mut DAT_0066d9b0)) {
                            break;
                        }
                    }
                }
                if ru(at(p8!(DAT_0066d968), i6 as u32)) != 0 && DAT_0066da4c == 0 && {
                    i16 = 0;
                    0 < DAT_00aa5aac
                } {
                    let mut pb22 = p8!(DAT_00b6c042);
                    let mut l88 = pc!(DAT_00a91d24);
                    let mut i10 = i12;
                    loop {
                        i12 = i10;
                        if l88.read() != 0
                            && (l88.wrapping_sub(0xc).read() as u8 as u32) == (cur(i6) as u32)
                        {
                            let mut i9: i32 = 0;
                            let mut pb23 = p8!(DAT_00b6d5a6);
                            loop {
                                let t0 = rd(pb23);
                                if t0 == i16 {
                                    let t1 = rd(pb23.wrapping_add(4));
                                    if t1 != 0 {
                                        break;
                                    }
                                }
                                pb23 = pb23.wrapping_add(0x58);
                                i9 = i9.wrapping_add(1);
                                if !(pb23 < p8!(DAT_00b6d866)) {
                                    break;
                                }
                            }
                            if i9 == 8 {
                                let pc2 = l88.wrapping_sub(100);
                                let u17: u32 = 1;
                                let t0 = rd(pb22);
                                let pc5 = FUN_004ddc10(t0.wrapping_add(0x9a));
                                sprintf(bufp, pc!(DAT_0052a6b0), 6i32, pc5, u17, pc2);
                                i12 = i10.wrapping_add(0xf);
                                FUN_004db4e0(0xc, bufp, i13, i10, 0xffff);
                            }
                        }
                        pb22 = pb22.wrapping_add(4);
                        l88 = l88.wrapping_add(0x562);
                        i16 = i16.wrapping_add(1);
                        i10 = i12;
                        if !(i16 < DAT_00aa5aac as i32) {
                            break;
                        }
                    }
                }
            }
            i6 = i6.wrapping_add(4);
            local_68 = local_68.wrapping_add(1);
            if 0x1f < i6 {
                return 1;
            }
        }
    }
}
