// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006a8858: i32;
    static mut DAT_006a8866: u32;
    static mut DAT_006a884a: i32;
    static mut DAT_006a8874: i32;
    static mut DAT_006d4984: u32;
    static mut DAT_005f10cc: i32;
    static mut DAT_005f10dc: i32;
    static mut DAT_005f10b8: i32;
    static mut DAT_005f10d8: i32;
    static mut DAT_005f10e0: i32;
    static mut DAT_005f10c8: i32;
    static mut DAT_0052ae1c: i32;
    static mut DAT_005279f4: i32;
    static mut DAT_005f0fac: i32;
    static mut DAT_005f0fb0: i32;
    static mut DAT_005f102c: i32;
    static mut DAT_005f1030: i32;
    static mut DAT_005f1034: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0045ed70(param_1: i32, param_2: i32, param_3: i32);
    fn FUN_0045d7d0(param_1: i32, param_2: i32, param_3: i32);
    fn FUN_0045dd00(param_1: i32, param_2: i32, param_3: i32);
    fn FUN_0045e290(param_1: i32, param_2: i32, param_3: i32);
    fn FUN_0045e7d0(param_1: i32, param_2: i32, param_3: i32);
    fn FUN_0045f410(param_1: i32, param_2: i32) -> i32;
    fn FUN_0045c050(param_1: i32, param_2: i32, param_3: i32, param_4: *mut u8) -> *mut i32;
    fn _ftol() -> i64;
}

#[inline(always)]
unsafe fn rd(a: i32) -> i32 {
    // SAFETY: address is the game's own memory, read where the original reads.
    unsafe { (a as u32 as usize as *const i32).read_unaligned() }
}

#[inline(always)]
unsafe fn wr(a: i32, v: i32) {
    // SAFETY: address is the game's own memory, written where the original writes.
    unsafe { (a as u32 as usize as *mut i32).write_unaligned(v) }
}

#[inline(always)]
unsafe fn rdf(a: i32) -> f32 {
    // SAFETY: address is the game's own memory.
    unsafe { (a as u32 as usize as *const f32).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0045d2a0(param_1: i32) {
    // SAFETY: all globals, callees and pointers are the game's own memory;
    // accesses follow the original's order and offsets.
    unsafe {
        let puVar3: *mut u32 = (&raw mut DAT_006a8866).read_unaligned() as usize as *mut u32;
        let mut iVar8: i32 = 0;
        let mut local_8: i32 = 0;
        let iVar1: i32 =
            DAT_006a8858.wrapping_add(rd(param_1.wrapping_add(0xbc)).wrapping_mul(0x18));
        DAT_005f10cc = rd(param_1.wrapping_add(200)).wrapping_add(rd(param_1.wrapping_add(0xbc)));
        let iTmp: i32 = (&raw mut DAT_006a884a).read_unaligned();
        let mut local_14: i32 = rd(param_1.wrapping_add(0xb8))
            .wrapping_mul(0x70)
            .wrapping_add(iTmp);
        let mut puVar4: *mut u32 = (&raw mut DAT_006a8866).read_unaligned() as usize as *mut u32;
        DAT_005f10dc = DAT_005f10cc;
        if 0 < DAT_005f10cc {
            loop {
                puVar4.write_unaligned(0xffff_ffff);
                iVar8 = iVar8.wrapping_add(1);
                puVar4 = puVar4.add(1);
                if !(iVar8 < DAT_005f10cc) {
                    break;
                }
            }
        }
        let mut local_c: i32 = 0;
        let mut local_4: *mut i32 = core::ptr::null_mut();
        if 0 < rd(param_1.wrapping_add(0xc4)) {
            loop {
                iVar8 = rd(local_14)
                    .wrapping_mul(0x40)
                    .wrapping_add(rd(param_1.wrapping_add(0xa0)));
                DAT_005f10b8 = 4i32.wrapping_sub((rd(iVar8.wrapping_add(0x1c)) == -1) as i32);
                if 0 < DAT_005f10b8 {
                    let mut piVar9: i32 = iVar8.wrapping_add(0x20);
                    let mut piVar12: *mut i32 = &raw mut DAT_005f1030;
                    let mut local_18: i32 = DAT_005f10b8;
                    loop {
                        iVar8 =
                            rd(DAT_006a8874
                                .wrapping_add(rd(piVar9.wrapping_sub(32)).wrapping_mul(4)));
                        piVar12.write_unaligned(iVar8);
                        let iVar5 = DAT_005f10dc;
                        let pfVar2: i32 =
                            rd(param_1.wrapping_add(0x80)).wrapping_add(rd(piVar9).wrapping_mul(8));
                        let p4: i32 = iVar1.wrapping_add(iVar8.wrapping_mul(0x18));
                        if rdf(iVar1
                            .wrapping_add(0x10)
                            .wrapping_add(iVar8.wrapping_mul(0x18)))
                            != rdf(pfVar2)
                            || rdf(p4.wrapping_add(20)) != rdf(pfVar2.wrapping_add(4))
                        {
                            let t = DAT_005f10dc.wrapping_mul(0x18);
                            DAT_005f10dc = DAT_005f10dc.wrapping_add(1);
                            wr(iVar1.wrapping_add(t), rd(p4));
                            let b = iVar1.wrapping_add(iVar5.wrapping_mul(0x18));
                            wr(b.wrapping_add(4), rd(p4.wrapping_add(4)));
                            wr(b.wrapping_add(0xc), rd(p4.wrapping_add(12)));
                            wr(b.wrapping_add(8), rd(p4.wrapping_add(8)));
                            wr(
                                b.wrapping_add(0x10),
                                rd(rd(param_1.wrapping_add(0x80))
                                    .wrapping_add(rd(piVar9).wrapping_mul(8))),
                            );
                            wr(
                                b.wrapping_add(0x14),
                                rd(rd(param_1.wrapping_add(0x80))
                                    .wrapping_add(4)
                                    .wrapping_add(rd(piVar9).wrapping_mul(8))),
                            );
                            piVar12.write_unaligned(iVar5);
                        }
                        piVar9 = piVar9.wrapping_add(4);
                        piVar12 = piVar12.add(1);
                        local_18 = local_18.wrapping_sub(1);
                        if local_18 == 0 {
                            break;
                        }
                    }
                }
                if rd(local_14.wrapping_add(12)) == -1 {
                    if 0 < DAT_005f10b8 {
                        let mut s: *mut i32 = &raw mut DAT_005f1030;
                        let mut d: *mut i32 = &raw mut DAT_005f0fb0;
                        let mut n = DAT_005f10b8;
                        while n != 0 {
                            d.write_unaligned(s.read_unaligned());
                            s = s.add(1);
                            d = d.add(1);
                            n = n.wrapping_sub(1);
                        }
                    }
                    DAT_005f10d8 = DAT_005f10b8;
                    iVar8 = 0;
                    DAT_005f10b8 = 0;
                    if 0 < DAT_005f10d8 {
                        let mut p: *mut i32 = &raw mut DAT_005f0fb0;
                        let mut iVar5 = (&raw mut DAT_005f0fac)
                            .offset(DAT_005f10d8 as isize)
                            .read_unaligned();
                        loop {
                            let iVar15 = p.read_unaligned();
                            p = p.add(1);
                            iVar8 = iVar8.wrapping_add(1);
                            FUN_0045ed70(iVar5, iVar15, iVar1);
                            iVar5 = iVar15;
                            if !(iVar8 < DAT_005f10d8) {
                                break;
                            }
                        }
                    }
                }
                iVar8 = 0;
                wr(local_14.wrapping_add(12), DAT_006d4984 as i32);
                DAT_005f10d8 = 0;
                if 0 < DAT_005f10b8 {
                    let mut p: *mut i32 = &raw mut DAT_005f1030;
                    let mut iVar5 = (&raw mut DAT_005f102c)
                        .offset(DAT_005f10b8 as isize)
                        .read_unaligned();
                    loop {
                        let iVar15 = p.read_unaligned();
                        p = p.add(1);
                        iVar8 = iVar8.wrapping_add(1);
                        FUN_0045d7d0(iVar5, iVar15, iVar1);
                        iVar5 = iVar15;
                        if !(iVar8 < DAT_005f10b8) {
                            break;
                        }
                    }
                }
                iVar8 = 0;
                DAT_005f10b8 = 0;
                if 0 < DAT_005f10d8 {
                    let mut p: *mut i32 = &raw mut DAT_005f0fb0;
                    let mut iVar5 = (&raw mut DAT_005f0fac)
                        .offset(DAT_005f10d8 as isize)
                        .read_unaligned();
                    loop {
                        let iVar15 = p.read_unaligned();
                        p = p.add(1);
                        iVar8 = iVar8.wrapping_add(1);
                        FUN_0045dd00(iVar5, iVar15, iVar1);
                        iVar5 = iVar15;
                        if !(iVar8 < DAT_005f10d8) {
                            break;
                        }
                    }
                }
                iVar8 = 0;
                DAT_005f10d8 = 0;
                if 0 < DAT_005f10b8 {
                    let mut p: *mut i32 = &raw mut DAT_005f1030;
                    let mut iVar5 = (&raw mut DAT_005f102c)
                        .offset(DAT_005f10b8 as isize)
                        .read_unaligned();
                    loop {
                        let iVar15 = p.read_unaligned();
                        p = p.add(1);
                        iVar8 = iVar8.wrapping_add(1);
                        FUN_0045e290(iVar5, iVar15, iVar1);
                        iVar5 = iVar15;
                        if !(iVar8 < DAT_005f10b8) {
                            break;
                        }
                    }
                }
                iVar8 = 0;
                DAT_005f10b8 = 0;
                if 0 < DAT_005f10d8 {
                    let mut p: *mut i32 = &raw mut DAT_005f0fb0;
                    let mut iVar5 = (&raw mut DAT_005f0fac)
                        .offset(DAT_005f10d8 as isize)
                        .read_unaligned();
                    loop {
                        let iVar15 = p.read_unaligned();
                        p = p.add(1);
                        iVar8 = iVar8.wrapping_add(1);
                        FUN_0045e7d0(iVar5, iVar15, iVar1);
                        iVar5 = iVar15;
                        if !(iVar8 < DAT_005f10d8) {
                            break;
                        }
                    }
                }
                iVar8 = 0;
                if 0 < DAT_005f10b8 {
                    let mut piVar9: *mut i32 = &raw mut DAT_005f1030;
                    loop {
                        let iVar5 = piVar9.read_unaligned();
                        if iVar5 < DAT_005f10cc {
                            if puVar3.offset(iVar5 as isize).read_unaligned() == 0xffff_ffff {
                                let u = FUN_0045f410(iVar5, iVar1) as u32;
                                puVar3.offset(iVar5 as isize).write_unaligned(u);
                            }
                            let idx = piVar9.read_unaligned();
                            piVar9.write_unaligned(
                                puVar3.offset(idx as isize).read_unaligned() as i32
                            );
                        } else {
                            let r = FUN_0045f410(iVar5, iVar1);
                            piVar9.write_unaligned(r);
                        }
                        piVar9 = piVar9.add(1);
                        iVar8 = iVar8.wrapping_add(1);
                        if !(iVar8 < DAT_005f10b8) {
                            break;
                        }
                    }
                }
                if 2 < DAT_005f10b8 {
                    let mut iVar11: i32 = 0;
                    let iVar8b = rd(param_1.wrapping_add(0xac));
                    let iVar5 = rd(param_1.wrapping_add(0xa8));
                    let mut iVar15 = rd(iVar5.wrapping_add(0x14));
                    let mut iVar13 = rd(iVar5.wrapping_add(0x10));
                    if iVar15
                        .wrapping_mul(iVar13)
                        .wrapping_sub(rd(iVar5.wrapping_add(8)))
                        == 0
                    {
                        let mut iv5 = _ftol() as i32;
                        while 0x100 < iv5 && iVar13 != 8 && iVar15 != 8 {
                            iv5 >>= 2;
                            let iVar10 = iVar15.wrapping_mul(iVar13);
                            iVar13 >>= 1;
                            iVar11 = iVar11.wrapping_add(iVar10);
                            iVar15 >>= 1;
                        }
                    }
                    let pbVar7 = iVar8b.wrapping_add(iVar11);
                    if local_8 != pbVar7 {
                        local_4 = FUN_0045c050(
                            iVar13,
                            iVar15,
                            rd(param_1.wrapping_add(0xb4)).wrapping_add(0x1000),
                            pbVar7 as u32 as usize as *mut u8,
                        );
                        local_8 = pbVar7;
                    }
                }
                iVar8 = 2;
                if 2 < DAT_005f10b8 {
                    let mut p4: *mut i32 = &raw mut DAT_005f1034;
                    loop {
                        let base = DAT_005f10e0.wrapping_mul(0x14).wrapping_add(DAT_005f10c8);
                        wr(base, DAT_005f1030);
                        let base = DAT_005f10e0
                            .wrapping_mul(0x14)
                            .wrapping_add(4)
                            .wrapping_add(DAT_005f10c8);
                        wr(base, p4.read_unaligned());
                        let base = DAT_005f10e0
                            .wrapping_mul(0x14)
                            .wrapping_add(8)
                            .wrapping_add(DAT_005f10c8);
                        wr(base, p4.add(1).read_unaligned());
                        let base = DAT_005f10e0
                            .wrapping_mul(0x14)
                            .wrapping_add(0xc)
                            .wrapping_add(DAT_005f10c8);
                        wr(base, local_4 as usize as i32);
                        let base = DAT_005f10e0
                            .wrapping_mul(0x14)
                            .wrapping_add(0x10)
                            .wrapping_add(DAT_005f10c8);
                        wr(base, 0x3023);
                        if DAT_0052ae1c != 0 {
                            let a = DAT_005f10e0
                                .wrapping_mul(0x14)
                                .wrapping_add(0x10)
                                .wrapping_add(DAT_005f10c8);
                            wr(a, rd(a).wrapping_add(0x300));
                        }
                        if DAT_005279f4 != 0 {
                            let a = DAT_005f10e0
                                .wrapping_mul(0x14)
                                .wrapping_add(0x10)
                                .wrapping_add(DAT_005f10c8);
                            wr(a, rd(a).wrapping_add(0x400));
                            DAT_005279f4 = 0;
                        }
                        iVar8 = iVar8.wrapping_add(1);
                        DAT_005f10e0 = DAT_005f10e0.wrapping_add(1);
                        p4 = p4.add(1);
                        if !(iVar8 < DAT_005f10b8) {
                            break;
                        }
                    }
                }
                local_c = local_c.wrapping_add(1);
                local_14 = local_14.wrapping_add(0x70);
                if !(local_c < rd(param_1.wrapping_add(0xc4))) {
                    break;
                }
            }
        }
    }
}
