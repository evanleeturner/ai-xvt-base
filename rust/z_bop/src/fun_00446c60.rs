// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: *mut u8;
    static mut DAT_009ec5fe: u16;
    static mut DAT_009a73d8: u8;
    static mut DAT_00a08140: i32;
    static mut DAT_00a08144: i32;
    static mut DAT_00a08148: i32;
    static mut DAT_009d8cde: u8;
    static mut DAT_009d8cdf: u8;
    static mut DAT_009d8ce0: u8;
    static mut DAT_009d8ce1: u8;
    static mut DAT_009d8ce2: u8;
    static mut DAT_009d8cd8: u8;
    static mut DAT_009d8cd9: u8;
    static mut DAT_009d8cda: u8;
    static mut DAT_009d8cdb: u8;
    static mut DAT_009d8cdc: u8;
    static mut DAT_009d8c87: u8;
    static mut DAT_00a07cdc: i32;
    static mut DAT_00a07cd8: i32;
    static mut DAT_00a07cd4: i32;
    static mut DAT_009a8c00: u32;
    static mut DAT_009d8112: u16;
    static mut DAT_00a080f0: u16;
    static mut DAT_009a8070: u16;
    static mut s_starshipprotectpln_0051a2fc: u8;
    static mut s_formevadeldr1pln_0051a394: u8;
    static mut DAT_0051a194: u8;
    static mut DAT_009a8d84: i32;
    static mut DAT_009a8d88: i32;
    static mut DAT_009a8da4: i32;
    static mut DAT_009ecc34: i32;
    static mut DAT_009e9640: u16;
    static mut DAT_009a8e18: *mut u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00461690(a: u16, b: u16, c: u16, d: u16, e: u16, f: i32) -> u16;
    fn FUN_00458e20(a: u32, b: i32);
    fn FUN_0041f410(a: i32, b: i32, c: i32) -> u32;
    fn FUN_0046a8d0(a: i32, b: i32, c: i32);
    fn FUN_00425cb0() -> u16;
    fn FUN_00425b70(a: u32, b: u16) -> u32;
    fn FUN_00459750(a: i32) -> u16;
    fn FUN_00405860(a: i32) -> i16;
    fn FUN_00427bd0(a: u16, b: u16, c: *mut u8);
    fn FUN_0042e720(a: u32, b: i32);
}

#[inline(always)]
unsafe fn ld<T>(p: *const u8, o: i32) -> T {
    // SAFETY: caller supplies a pointer to the game's own memory.
    unsafe {
        p.wrapping_add(o as u32 as usize)
            .cast::<T>()
            .read_unaligned()
    }
}

#[inline(always)]
unsafe fn st<T>(p: *mut u8, o: i32, v: T) {
    // SAFETY: caller supplies a pointer to the game's own memory.
    unsafe {
        p.wrapping_add(o as u32 as usize)
            .cast::<T>()
            .write_unaligned(v)
    }
}

#[inline(always)]
fn at(p: *mut u8, o: i32) -> *mut u8 {
    p.wrapping_add(o as u32 as usize)
}

#[inline(always)]
unsafe fn obj(o: i32) -> *mut u8 {
    // SAFETY: the entry table and its object pointers are the game's own.
    unsafe { ld::<*mut u8>(DAT_009a1fe8, 0x1f + o) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00446c60(param_1: u16) {
    // SAFETY: all globals, callees and entry pointers belong to the game;
    // accesses mirror the original's order and offsets.
    unsafe {
        let mut iVar13: i32 = (param_1 as u32).wrapping_mul(0x23) as i32;
        let mut pEnt = at(DAT_009a1fe8, iVar13);
        if ld::<u16>(pEnt, 0x17) == 0 {
            return;
        }
        if (DAT_009ec5fe >> 1) < (*at(pEnt, 0x1a) as u16) {
            let v = *at(pEnt, 0x1a);
            *at(pEnt, 0x1a) = v.wrapping_sub((DAT_009ec5fe >> 1) as u8);
            return;
        }
        *at(pEnt, 0x1a) = 0xec;
        pEnt = at(DAT_009a1fe8, iVar13);
        let mut iVar14: i32 = ld::<i32>(pEnt, 4);
        let mut local_14: i32 = ld::<i32>(pEnt, 8);
        let mut local_10: i32 = ld::<i32>(pEnt, 0xc);
        DAT_009a73d8 = (*at(pEnt, 3) == b'L') as u8;
        let iVar8: i32 = (*at(pEnt, 0x16) as u32).wrapping_mul(0x566) as i32;
        DAT_00a08140 = local_14;
        DAT_00a08144 = iVar14;
        DAT_00a08148 = local_10;
        let ix = iVar8 as u32 as usize;
        let mut uStack_8: u16 = FUN_00461690(
            (&raw mut DAT_009d8cde).wrapping_add(ix).read() as u16,
            (&raw mut DAT_009d8cdf).wrapping_add(ix).read() as u16,
            (&raw mut DAT_009d8ce2).wrapping_add(ix).read() as u16,
            (&raw mut DAT_009d8ce0).wrapping_add(ix).read() as u16,
            (&raw mut DAT_009d8ce1).wrapping_add(ix).read() as u16,
            0,
        );
        if uStack_8 == 0xffff {
            uStack_8 = FUN_00461690(
                (&raw mut DAT_009d8cd8).wrapping_add(ix).read() as u16,
                (&raw mut DAT_009d8cda).wrapping_add(ix).read() as u16,
                (&raw mut DAT_009d8cdc).wrapping_add(ix).read() as u16,
                (&raw mut DAT_009d8cd9).wrapping_add(ix).read() as u16,
                (&raw mut DAT_009d8cdb).wrapping_add(ix).read() as u16,
                0,
            );
        }
        if uStack_8 == 0xffff {
            return;
        }
        FUN_00458e20(uStack_8 as u32, 0);
        let mut local_38: i32 = DAT_00a07cdc;
        let mut local_3c: i32 = DAT_00a07cd8;
        let mut iVar9: i32 = DAT_00a07cd4;
        let mut uVar10: u32 = FUN_0041f410(
            (DAT_00a07cdc as u32).wrapping_sub(iVar14 as u32) as i32,
            (DAT_00a07cd8 as u32).wrapping_sub(local_14 as u32) as i32,
            (DAT_00a07cd4 as u32).wrapping_sub(local_10 as u32) as i32,
        );
        if 0xffff < uVar10 {
            return;
        }
        let iVar4: i32 = (uStack_8 as u32).wrapping_mul(0x23) as i32;
        let mut pTgt = at(DAT_009a1fe8, iVar4);
        if ld::<i32>(pTgt, 0x1f) != 0 {
            FUN_0046a8d0(
                (ld::<i32>(pTgt, 4) as u32).wrapping_sub(iVar14 as u32) as i32,
                (ld::<i32>(pTgt, 8) as u32).wrapping_sub(local_14 as u32) as i32,
                (ld::<i32>(pTgt, 0xc) as u32).wrapping_sub(local_10 as u32) as i32,
            );
            if *at(DAT_009a1fe8, 3i32.wrapping_add(iVar13)) == b'L' {
                DAT_009a8c00 =
                    ((DAT_009d8112 as u32).wrapping_mul(DAT_009a8c00) as i32 >> 0xf) as u32;
            } else {
                DAT_009a8c00 =
                    ((DAT_009d8112 as u32).wrapping_mul(DAT_009a8c00) as i32 >> 0xe) as u32;
            }
            let sVar7: i16 = DAT_009a8c00 as u16 as i16;
            let uVar5 = FUN_00425cb0();
            pTgt = at(DAT_009a1fe8, iVar4);
            uVar10 = ((sVar7 as i32 + (uVar5 & 3) as i32) - 1) as u16 as u32;
            let p = ld::<*mut u8>(pTgt, 0x1f);
            local_38 = ld::<u32>(pTgt, 4).wrapping_add(
                ld::<u32>(pTgt, 4)
                    .wrapping_sub(ld::<u32>(p, 6))
                    .wrapping_mul(uVar10),
            ) as i32;
            let p = ld::<*mut u8>(pTgt, 0x1f);
            local_3c = ld::<u32>(pTgt, 8).wrapping_add(
                ld::<u32>(pTgt, 8)
                    .wrapping_sub(ld::<u32>(p, 10))
                    .wrapping_mul(uVar10),
            ) as i32;
            let p = ld::<*mut u8>(pTgt, 0x1f);
            iVar9 = ld::<u32>(pTgt, 0xc).wrapping_add(
                ld::<u32>(pTgt, 0xc)
                    .wrapping_sub(ld::<u32>(p, 0xe))
                    .wrapping_mul(uVar10),
            ) as i32;
        }
        FUN_0046a8d0(
            (local_38 as u32).wrapping_sub(iVar14 as u32) as i32,
            (local_3c as u32).wrapping_sub(local_14 as u32) as i32,
            (iVar9 as u32).wrapping_sub(local_10 as u32) as i32,
        );
        let mut uVar5: u16 = DAT_00a080f0;
        let mut local_c: u16 = DAT_009a8070;
        let bVar1: u8 = *at(DAT_009a1fe8, 3i32.wrapping_add(iVar13));
        iVar9 = ((iVar13 as u32 & 0xffff_ff00) | bVar1 as u32) as i32;
        uVar10 = if bVar1 > 0x4c { 0x14 + 0x96 } else { 0x96 };
        'lab: {
            if DAT_00a080f0 < 0x2000 {
                iVar9 = (local_10 as u32).wrapping_add(uVar10) as i32;
                local_10 = iVar9;
            } else if DAT_00a080f0 < 0x6001 {
                if 0x1fff < DAT_009a8070 && DAT_009a8070 < 0xe001 {
                    if DAT_009a8070 < 0x6000 {
                        iVar14 = (iVar14 as u32).wrapping_add(uVar10) as i32;
                        break 'lab;
                    }
                    if 0x9fff < DAT_009a8070 {
                        iVar14 = (iVar14 as u32).wrapping_sub(uVar10) as i32;
                        break 'lab;
                    }
                    uVar10 = 0u32.wrapping_sub(uVar10);
                }
                iVar9 = (local_14 as u32).wrapping_add(uVar10) as i32;
                local_14 = iVar9;
            } else {
                if 0x4c < bVar1 {
                    return;
                }
                iVar9 = (local_10 as u32).wrapping_sub(uVar10) as i32;
                local_10 = iVar9;
            }
        }
        let mut uVar6: u16 = 0xffff;
        if DAT_009a8c00 < 0x10000 {
            uVar6 = DAT_009a8c00 as u16;
        }
        let iVar11: i32 = ld::<i32>(DAT_009a1fe8, 0x1fi32.wrapping_add(iVar4));
        let uVar12: u16;
        if iVar11 == 0 {
            uVar12 = 0xffff;
        } else {
            let uVar15: u16 = ld::<u16>(iVar11 as u32 as usize as *const u8, 0x79);
            let mut t: u16 = 0xffff;
            if 0xbb < uVar15 {
                t = (uVar15 as i32).wrapping_mul(-0x80).wrapping_add(0x5dff) as u16;
            }
            uVar12 = t;
        }
        uVar10 = FUN_00425b70(((iVar9 as u32 >> 0x10) << 0x10) | (!uVar6) as u32, uVar12);
        uVar6 = FUN_00425cb0();
        if (uVar10 as u16) < uVar6 {
            let sVar7 = FUN_00425cb0() as i16;
            uVar6 = FUN_00425cb0();
            let mut uVar15: u16 = ((sVar7 as i32 as u32).wrapping_sub(0x100) & 0x3ff) as u16;
            if 0x7fff < uVar6 {
                uVar15 = (uVar15 as i32).wrapping_neg() as u16;
            }
            local_c = local_c.wrapping_add(uVar15);
            let sVar7 = FUN_00425cb0() as i16;
            uVar6 = FUN_00425cb0();
            let uVar15: u16 = ((sVar7 as i32 as u32).wrapping_sub(0x100) & 0x3ff) as u16;
            if uVar6 < 0x8000 {
                uVar5 = uVar5.wrapping_add(uVar15);
                if (uVar5 & 0x8000) != 0 {
                    uVar5 = 0x7fff;
                }
            } else {
                uVar5 = uVar5.wrapping_sub(uVar15);
                if (uVar5 & 0x8000) != 0 {
                    uVar5 = 0;
                }
            }
        }
        uVar6 = FUN_00459750(7);
        if uVar6 != 0xffff {
            let iVar9: i32 = (uVar6 as u32).wrapping_mul(0x23) as i32;
            *obj(iVar9) = 1;
            *at(DAT_009a1fe8, 2i32.wrapping_add(iVar9)) = 7;
            let mut iVar11: i32;
            if *at(DAT_009a1fe8, 3i32.wrapping_add(iVar13)) == b'L' {
                iVar11 = 0x8e;
            } else {
                iVar11 = 0x8a;
                let v = (&raw mut DAT_009d8c87)
                    .wrapping_add(iVar8 as u32 as usize)
                    .read();
                if v == 1 || v == 4 {
                    iVar11 = 0x8c;
                }
            }
            *at(DAT_009a1fe8, 3i32.wrapping_add(iVar9)) = iVar11 as u8;
            st::<u16>(obj(iVar9), 0x83, 1);
            st::<u16>(obj(iVar9), 0x85, param_1);
            *at(obj(iVar9), 0x87) = 0;
            let k = (*at(DAT_009a1fe8, 0x16i32.wrapping_add(iVar13)) as u32).wrapping_mul(0x566)
                as usize;
            *at(obj(iVar9), 0x88) = (&raw mut DAT_009d8c87).wrapping_add(k).read();
            st::<u16>(DAT_009a1fe8, 0x12i32.wrapping_add(iVar9), uVar5);
            st::<u16>(DAT_009a1fe8, 0x14i32.wrapping_add(iVar9), 0);
            st::<u16>(DAT_009a1fe8, 0x10i32.wrapping_add(iVar9), local_c);
            *at(obj(iVar9), 0x92) = 1;
            let pObj = obj(iVar9);
            *at(pObj, 0x8b) = *at(pObj, 0x92);
            let a = (&raw mut s_starshipprotectpln_0051a2fc)
                .wrapping_add(iVar11.wrapping_mul(2).wrapping_add(10) as u32 as usize);
            st::<u16>(obj(iVar9), 0x79, ld::<u16>(a, 0) >> 1);
            let sVar7 = FUN_00405860(iVar11);
            st::<i16>(obj(iVar9), 0x81, sVar7.wrapping_mul(2));
            let a = (&raw mut DAT_0051a194).wrapping_add(iVar11.wrapping_mul(4) as u32 as usize);
            st::<u32>(obj(iVar9), 0x7d, ld::<u32>(a, 0));
            FUN_00427bd0(uVar5, local_c, at(DAT_009a1fe8, iVar9));
            st::<i32>(obj(iVar9), 6, iVar14);
            st::<i32>(obj(iVar9), 10, local_14);
            st::<i32>(obj(iVar9), 0xe, local_10);
            let a = (&raw mut s_formevadeldr1pln_0051a394)
                .wrapping_add(iVar11.wrapping_mul(2).wrapping_add(2) as u32 as usize);
            iVar13 = ld::<i16>(a, 0) as i32;
            let lVar2: i64 = DAT_009a8d84 as i64;
            let lVar3: i64 = DAT_009a8d88 as i64;
            let mut lVar5: i64 = (DAT_009a8da4 as i64).wrapping_mul(iVar13 as i64);
            st::<u32>(
                DAT_009a1fe8,
                4i32.wrapping_add(iVar9),
                (iVar14 as u32).wrapping_add(
                    (lVar5 as u32) >> 0xf | (((lVar5 as u64) >> 0x20) as u32) << 0x11,
                ),
            );
            lVar5 = lVar2.wrapping_mul(iVar13 as i64);
            st::<u32>(
                DAT_009a1fe8,
                8i32.wrapping_add(iVar9),
                (local_14 as u32).wrapping_add(
                    (lVar5 as u32) >> 0xf | (((lVar5 as u64) >> 0x20) as u32) << 0x11,
                ),
            );
            lVar5 = lVar3.wrapping_mul(iVar13 as i64);
            st::<u32>(
                DAT_009a1fe8,
                0xci32.wrapping_add(iVar9),
                (local_10 as u32).wrapping_add(
                    (lVar5 as u32) >> 0xf | (((lVar5 as u64) >> 0x20) as u32) << 0x11,
                ),
            );
            FUN_0042e720(uVar6 as u32, DAT_009ecc34);
            iVar13 = (uVar6.wrapping_sub(DAT_009e9640) as u32).wrapping_mul(10) as i32;
            *at(DAT_009a8e18, 1i32.wrapping_add(iVar13)) = 0;
            st::<u16>(DAT_009a8e18, 4i32.wrapping_add(iVar13), uStack_8);
            if uStack_8 == 0xffff || 0x7fff < uStack_8 {
                st::<u16>(DAT_009a8e18, 6i32.wrapping_add(iVar13), 0);
            } else {
                let v = ld::<u16>(DAT_009a1fe8, iVar4);
                st::<u16>(DAT_009a8e18, 6i32.wrapping_add(iVar13), v);
            }
            *at(DAT_009a8e18, iVar13) = 0xff;
            st::<*mut u8>(obj(iVar9), 0xa5, at(DAT_009a8e18, iVar13));
        }
    }
}
