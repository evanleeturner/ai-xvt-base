// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_00732a3c: u32;
    static mut DAT_00732904: i32;
    static mut DAT_006d4988: i32;
    static mut DAT_006d4980: i32;
    static mut DAT_0071b71c: i32;
    static mut DAT_006d4974: i32;
    static mut DAT_006d4978: i32;
    static mut DAT_006d4970: i32;
    static mut DAT_0071e6c4: i32;
    static mut DAT_00732f38: i32;
    static mut DAT_00732fe8: i32;
    static mut DAT_0073294c: i32;
    static mut DAT_007329b0: i32;
    static mut DAT_007329b4: i32;
    static mut DAT_0070a180: i32;
    static mut DAT_0070acd8: i32;
    static mut DAT_00703ad8: i32;
    static mut DAT_006d9d38: i32;
    static mut DAT_0071ecf4: i32;
    static mut DAT_00732e48: i32;
    static mut _DAT_00732950: i32;
    static mut DAT_00732a38: i32;
    static mut DAT_0070ab18: i32;
    static mut DAT_0070ab14: i32;
    static mut DAT_0070ac20: i32;
    static mut DAT_00732f7c: i32;
    static mut DAT_00732f8c: i32;
    static mut DAT_00732f5c: i32;
    static mut DAT_00655a84: u16;
    static mut DAT_0071e6be: u16;
    static mut DAT_006db6de: u16;
    static mut DAT_0071f2d6: u16;
    static mut DAT_00732576: u16;
    static mut DAT_007092d9: u8;
    static mut DAT_007092d5: u8;
    static mut DAT_007092c1: u8;
    static mut DAT_006d4a02: u8;
    static mut DAT_0071b744: u8;
    static mut DAT_0071bce9: u8;
    static mut DAT_0071b75d: u8;
    static mut DAT_0071b75f: u8;
    static mut DAT_0071b74a: u8;
    static mut DAT_0071b73c: u8;
    static mut DAT_0071b7c1: u8;
    static mut DAT_0071b7c3: u8;
    static mut DAT_0071b776: u8;
    static mut DAT_0071b777: u8;
    static mut DAT_0071bcdd: u8;
    static mut DAT_00709388: u8;
    static mut DAT_006d4a7a: u8;
    static mut DAT_006d4abe: u8;
    static mut DAT_006d4aaa: u8;
    static mut DAT_006d4ac8: u8;
    static mut DAT_0070ad40: u8;
    static mut DAT_0070aee5: u8;
    static mut DAT_0070aee9: u8;
    static mut DAT_0070ac4a: u8;
    static mut DAT_0052686c: u8;
    static mut DAT_0070ad8c: u8;
    static mut DAT_0070ad8d: u8;
    static mut DAT_0070ad8e: u8;
    static mut DAT_0070ad8f: u8;
    static mut DAT_0052b14b: u8;
    static mut DAT_0052b14d: u8;
    static mut DAT_0052b14f: u8;
    static mut DAT_00709d0a: u8;
    static mut DAT_00703ca0: u8;
    static mut DAT_00525bf7: u8;
    static mut DAT_0070ad44: u8;
    static mut DAT_0070b215: u8;
    static mut DAT_00523c08: i8;
    static mut DAT_00523bf4: i8;
    static mut DAT_00523be4: i8;
    static mut DAT_00523bd0: i8;
    static mut DAT_00523c18: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0041b530(a: u32, b: u32);
    fn FUN_004a1590(a: i32, b: u32, c: i32, d: u32, e: u32, f: u32) -> i32;
    fn FUN_004a1d70(a: i32, b: i32);
    fn FUN_004a34c0(a: u32, b: u32, c: i32);
    fn FUN_0042a230(a: i32, b: u32, c: i32, d: i16, e: u16, f: i16);
    fn FUN_0040cd90(a: *mut u8, b: u16);
    fn FUN_0040ed40(a: *mut u8, b: u16, c: u32);
    fn FUN_0042b150(a: i32) -> i32;
    fn FUN_0040f7e0(a: u16, b: u16) -> u32;
    fn FUN_00478470() -> u32;
    fn FUN_0040a1a0(a: i32, b: u32, c: i32);
    fn FUN_004187e0(a: u16, b: u16) -> u32;
    fn FUN_00483410(a: i16, b: i16, c: i32);
    fn FUN_00419100(a: i32, b: i32) -> u32;
    fn FUN_004184f0(a: u16) -> u32;
    fn FUN_00483540(a: i16, b: i32, c: i32);
    fn FUN_004110e0(a: u32, b: u32, c: u32, d: u32) -> u32;
    fn FUN_00413d00(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32;
    fn FUN_00413010(a: u16, b: u16) -> u32;
    fn FUN_0040f260(a: u16, b: u16);
    fn FUN_00413280(a: u16, b: u32);
    fn FUN_004a0ef0(a: u16);
    fn FUN_00412700(a: u32, b: u8);
    fn FUN_00410810(a: u32, b: u32, c: i32);
}

#[inline(always)]
fn ad<T: Into<i64>>(a: T) -> usize {
    a.into() as u32 as usize
}
#[inline(always)]
unsafe fn r8<T: Into<i64>>(a: T) -> u8 {
    // SAFETY: address inside the game's memory, as in the original.
    unsafe { (ad(a) as *const u8).read() }
}
#[inline(always)]
unsafe fn r16<T: Into<i64>>(a: T) -> u16 {
    // SAFETY: address inside the game's memory, as in the original.
    unsafe { (ad(a) as *const u16).read_unaligned() }
}
#[inline(always)]
unsafe fn r32<T: Into<i64>>(a: T) -> i32 {
    // SAFETY: address inside the game's memory, as in the original.
    unsafe { (ad(a) as *const i32).read_unaligned() }
}
#[inline(always)]
unsafe fn w8<T: Into<i64>>(a: T, v: u32) {
    // SAFETY: address inside the game's memory, as in the original.
    unsafe { (ad(a) as *mut u8).write(v as u8) }
}
#[inline(always)]
unsafe fn w16<T: Into<i64>>(a: T, v: u32) {
    // SAFETY: address inside the game's memory, as in the original.
    unsafe { (ad(a) as *mut u16).write_unaligned(v as u16) }
}
#[inline(always)]
unsafe fn w32<T: Into<i64>>(a: T, v: u32) {
    // SAFETY: address inside the game's memory, as in the original.
    unsafe { (ad(a) as *mut u32).write_unaligned(v) }
}
#[inline(always)]
fn o(a: i32, b: i32) -> i32 {
    a.wrapping_add(b)
}
#[inline(always)]
fn add3(a: i32, b: i32, c: i32) -> i32 {
    a.wrapping_add(b).wrapping_add(c)
}

macro_rules! adr {
    ($x:ident) => {
        ((&raw mut $x) as usize as i32)
    };
}

#[unsafe(no_mangle)]
#[allow(
    non_snake_case,
    unused_assignments,
    unused_mut,
    unused_variables,
    unused_unsafe,
    unused_labels,
    unused_macros
)]
pub unsafe extern "stdcall" fn FUN_0040d2d0() {
    let mut cVar2: i8 = 0;
    let mut cVar3: i8 = 0;
    let mut bVar4: u8 = 0;
    let mut iVar5: i32 = 0;
    let mut sVar6: i16 = 0;
    let mut uVar7: u16 = 0;
    let mut uVar8: u32 = 0;
    let mut iVar9: i32 = 0;
    let mut iVar10: i32 = 0;
    let mut iVar11: i32 = 0;
    let mut uVar13: u32 = 0;
    let mut pbVar14: *mut u8 = core::ptr::null_mut();
    let mut iVar15: i32 = 0;
    let mut iVar16: i32 = 0;
    let mut uVar17: u32 = 0;
    let mut puVar18: u32 = 0;
    let mut uVar19: u16 = 0;
    let mut iVar20: i32 = 0;
    let mut uVar22: u16 = 0;
    let mut iVar23: i32 = 0;
    let mut iVar24: i32 = 0;
    let mut uVar25: u32 = 0;
    let mut unaff_EBX: u32 = 0;
    let mut uVar26: u16 = 0;
    let mut uVar27: u16 = 0;
    let mut iVar28: i32 = 0;
    let mut iVar29: i32 = 0;
    let mut puVar30: u32 = 0;
    let mut puVar31: u32 = 0;
    let mut bVar32: i32 = 0;
    let mut bCond: i32 = 0;
    let mut uVar33: u8 = 0;
    let mut in_stack_ffffffc4: u32 = 0;
    let mut sVar35: i16 = 0;
    let mut uStack_38: u16 = 0;
    let mut local_36: u16 = 0;
    let mut uStack_34: u16 = 0;
    let mut local_32: u16 = 0;
    let mut bStack_30: u8 = 0;
    let mut local_1c: i8 = 0;
    let mut local_18: u32 = 0;
    let mut local_10: u32 = 0;

    macro_rules! cmps {
        ($a:expr, $b:expr, $n:expr) => {{
            let mut cnt: i32 = $n;
            let mut pa: *const i8 = ad($a) as *const i8;
            let mut pb: *const i8 = ad($b) as *const i8;
            loop {
                if cnt == 0 {
                    break;
                }
                cnt -= 1;
                bVar32 = (pa.read() == pb.read()) as i32;
                pa = pa.wrapping_add(1);
                pb = pb.wrapping_add(1);
                if bVar32 == 0 {
                    break;
                }
            }
        }};
    }

    macro_rules! remove_entry {
        () => {{
            uVar25 = 0;
            bStack_30 = pbVar14.read();
            if bStack_30 != 0 {
                puVar18 = (iVar9 as u32).wrapping_add(0x53);
                loop {
                    if r16(puVar18) == uVar26 {
                        break;
                    }
                    puVar18 = puVar18.wrapping_add(2);
                    uVar25 = uVar25.wrapping_add(1);
                    if !((uVar25 as i32) < (bStack_30 as i32)) {
                        break;
                    }
                }
            }
            if uVar25 != bStack_30 as u32 {
                iVar15 = (uVar25 as i32).wrapping_add(1);
                if iVar15 < bStack_30 as i32 {
                    puVar30 = (iVar9 as u32)
                        .wrapping_add(0x51)
                        .wrapping_add((iVar15 as u32).wrapping_mul(2));
                    puVar31 = (iVar9 as u32)
                        .wrapping_add(0xf)
                        .wrapping_add((iVar15 as u32).wrapping_mul(4));
                    loop {
                        w16(puVar30, r16(puVar30.wrapping_add(2)) as u32);
                        puVar30 = puVar30.wrapping_add(2);
                        iVar15 = iVar15.wrapping_add(1);
                        w32(puVar31, r32(puVar31.wrapping_add(4)) as u32);
                        puVar31 = puVar31.wrapping_add(4);
                        if !(iVar15 < pbVar14.read() as i32) {
                            break;
                        }
                    }
                }
                pbVar14.write(pbVar14.read().wrapping_sub(1));
            }
            local_36 = local_36.wrapping_sub(1);
        }};
    }

    // SAFETY: all globals, functions and pointers are the game's own memory,
    // accessed exactly where the original does.
    unsafe {
        local_32 = _DAT_00732a3c as u16;
        if (local_32 as i32) < DAT_00732904 {
            loop {
                'next: {
                    uVar8 = local_32 as u32;
                    iVar5 = (uVar8 as i32).wrapping_mul(0x23);
                    iVar9 = o(DAT_006d4988, iVar5);
                    if r8(o(iVar9, 3)) != 0 && r8(o(iVar9, 2)) != 0x0d {
                        iVar9 = r32(o(iVar9, 0x1b));
                        if iVar9 != -1 {
                            iVar15 = (iVar9 as u32).wrapping_mul(0x5bd) as i32;
                            if r8(o(adr!(DAT_0071b744), iVar15)) == 2 {
                                break 'next;
                            }
                            if r32(o(adr!(DAT_0071bce9), iVar15)) < DAT_0071b71c {
                                uVar26 = _DAT_00732a3c as u16;
                                iVar10 = DAT_0071b71c.wrapping_add(0x1d);
                                w16(o(adr!(DAT_0071b75d), iVar15), 0xffff);
                                w16(o(adr!(DAT_0071b75f), iVar15), 0);
                                iVar23 = DAT_006d4980;
                                w32(o(adr!(DAT_0071bce9), iVar15), iVar10 as u32);
                                if ((_DAT_00732a3c & 0xffff) as i32) < iVar23 {
                                    loop {
                                        iVar23 =
                                            o((uVar26 as i32).wrapping_mul(0x23), DAT_006d4988);
                                        if r8(o(iVar23, 3)) != 0 {
                                            let t = r32(o(r32(o(iVar23, 0x1f)), 0xa7));
                                            if t != 0
                                                && r16(o(t, 0x1df)) != 0
                                                && (2 < r8(o(iVar23, 2)) && r8(o(iVar23, 2)) < 5)
                                            {
                                                FUN_0041b530(uVar8, uVar26 as u32);
                                            }
                                        }
                                        uVar26 = uVar26.wrapping_add(1);
                                        if !((uVar26 as i32) < DAT_006d4980) {
                                            break;
                                        }
                                    }
                                }
                            }
                            iVar23 = o(DAT_006d4988, iVar5);
                            DAT_006d4974 = r32(o(iVar23, 4));
                            DAT_006d4978 = r32(o(iVar23, 8));
                            DAT_006d4970 = r32(o(iVar23, 0xc));
                            DAT_0071e6c4 = r32(o(r32(o(iVar23, 0x1f)), 6));
                            DAT_00732f38 = r32(o(r32(o(iVar23, 0x1f)), 10));
                            DAT_00732fe8 = r32(o(r32(o(iVar23, 0x1f)), 0xe));
                            local_10 =
                                (local_10 & 0xffff0000) | r16(o(adr!(DAT_0071b74a), iVar15)) as u32;
                            uVar25 = r16(o(adr!(DAT_0071b74a), iVar15)) as u32;
                            unaff_EBX = _DAT_00732a3c;
                            if (_DAT_00732a3c as i32) <= (uVar25 as i32)
                                && (uVar25 as i32) < DAT_006d4980
                            {
                                iVar10 = uVar25.wrapping_mul(0x23) as i32;
                                iVar20 = o(DAT_006d4988, iVar10);
                                unaff_EBX = ((_DAT_00732a3c >> 8) << 8) | r8(o(iVar20, 3)) as u32;
                                if r8(o(iVar20, 3)) != 0 && r8(o(iVar20, 2)) != 0x0d {
                                    DAT_0073294c = r32(o(iVar20, 4));
                                    DAT_007329b0 = r32(o(iVar20, 8));
                                    DAT_007329b4 = r32(o(iVar20, 0xc));
                                    DAT_0070a180 = r32(o(r32(o(iVar20, 0x1f)), 6));
                                    DAT_0070acd8 = r32(o(r32(o(iVar20, 0x1f)), 10));
                                    DAT_00703ad8 = r32(o(r32(o(iVar20, 0x1f)), 0xe));
                                    iVar24 = DAT_006d4970.wrapping_sub(DAT_007329b4);
                                    iVar28 = DAT_006d4978.wrapping_sub(DAT_007329b0);
                                    iVar11 = DAT_006d4974.wrapping_sub(DAT_0073294c);
                                    if iVar11 < 0 {
                                        iVar11 = 0i32.wrapping_sub(iVar11);
                                    }
                                    if iVar28 < 0 {
                                        iVar28 = 0i32.wrapping_sub(iVar28);
                                    }
                                    if iVar24 < 0 {
                                        iVar24 = 0i32.wrapping_sub(iVar24);
                                    }
                                    if iVar28 < iVar11 && iVar24 < iVar11 {
                                        DAT_006d9d38 = add3(iVar11, iVar28 >> 2, iVar24 >> 2);
                                    } else if iVar11 < iVar28 && iVar24 < iVar28 {
                                        DAT_006d9d38 = add3(iVar11 >> 2, iVar24 >> 2, iVar28);
                                    } else {
                                        DAT_006d9d38 = add3(iVar11 >> 2, iVar28 >> 2, iVar24);
                                    }
                                    unaff_EBX = r32(o(r32(o(iVar20, 0x1f)), 0xa7)) as u32;
                                    if r8(o(unaff_EBX as i32, 7)) != 3
                                        && r8(o(unaff_EBX as i32, 7)) != 4
                                        && r8(o(
                                            o(r8(o(r32(o(iVar23, 0x1f)), 0x87)) as i32, 0x1f0),
                                            unaff_EBX as i32,
                                        )) == 0
                                    {
                                        uVar26 = r16(o(
                                            adr!(DAT_0052686c),
                                            (r8(o(iVar20, 3)) as i32).wrapping_mul(0x16),
                                        ));
                                        if 3000 < uVar26 {
                                            uVar26 >>= 1;
                                        }
                                        if DAT_006d9d38 < (uVar26 as i32).wrapping_mul(4) {
                                            uVar26 = 0;
                                            uVar19 = 0;
                                            loop {
                                                uVar22 = r8(o(
                                                    o(uVar19 as i32, 0x1f0),
                                                    unaff_EBX as i32,
                                                ))
                                                    as u16;
                                                if uVar26 < uVar22 {
                                                    uVar26 = uVar22;
                                                }
                                                uVar19 = uVar19.wrapping_add(1);
                                                if !(uVar19 < 10) {
                                                    break;
                                                }
                                            }
                                            local_1c = (uVar26 as i8).wrapping_add(1);
                                            w8(
                                                o(
                                                    o(
                                                        r16(o(adr!(DAT_0071b73c), iVar15)) as i32,
                                                        0x1f0,
                                                    ),
                                                    unaff_EBX as i32,
                                                ),
                                                local_1c as u8 as u32,
                                            );
                                            uVar19 = r16(o(adr!(DAT_0071b73c), iVar15));
                                            pbVar14 =
                                                ad(o(o(DAT_006d4988, 0x16), iVar10)) as *mut u8;
                                            let idx = (pbVar14.read() as u32)
                                                .wrapping_add((uVar19 as u32).wrapping_mul(0x30))
                                                as i32;
                                            w8(
                                                o(adr!(DAT_00709388), idx),
                                                r8(o(adr!(DAT_00709388), idx)).wrapping_add(1)
                                                    as u32,
                                            );
                                            uVar25 = pbVar14.read() as u32;
                                            w16(
                                                o(adr!(DAT_0071b7c1), iVar15),
                                                r16(o(adr!(DAT_0071b7c1), iVar15)).wrapping_add(1)
                                                    as u32,
                                            );
                                            let k = uVar25.wrapping_mul(0x126) as i32;
                                            w16(
                                                o(adr!(DAT_006d4a7a), k),
                                                r16(o(adr!(DAT_006d4a7a), k)).wrapping_add(1)
                                                    as u32,
                                            );
                                            iVar23 = uVar25.wrapping_mul(0x126) as i32;
                                            let idx2 = o(uVar19 as i32, iVar23);
                                            w8(
                                                o(adr!(DAT_006d4abe), idx2),
                                                r8(o(adr!(DAT_006d4abe), idx2)).wrapping_add(1)
                                                    as u32,
                                            );
                                            cVar2 = r8(o(unaff_EBX as i32, 0xb0)) as i8;
                                            iVar20 = uVar25.wrapping_mul(0x566) as i32;
                                            cVar3 = r8(o(adr!(DAT_0070ad40), iVar20)) as i8;
                                            if cVar3 == cVar2 {
                                                w8(o(adr!(DAT_006d4aaa), iVar23), 1);
                                                w16(
                                                    o(adr!(DAT_0071b7c3), iVar15),
                                                    r16(o(adr!(DAT_0071b7c3), iVar15))
                                                        .wrapping_add(1)
                                                        as u32,
                                                );
                                                let idx3 = o(
                                                    iVar23,
                                                    r16(o(adr!(DAT_0071b73c), iVar15)) as i32,
                                                );
                                                w8(
                                                    o(adr!(DAT_006d4ac8), idx3),
                                                    r8(o(adr!(DAT_006d4ac8), idx3)).wrapping_add(1)
                                                        as u32,
                                                );
                                            }
                                            local_18 = (cVar3 == cVar2) as u32;
                                            iVar11 = FUN_004a1590(
                                                5,
                                                pbVar14.read() as u32,
                                                iVar9,
                                                uVar26 as u32,
                                                local_18,
                                                r16(o(adr!(DAT_0071b73c), iVar15)) as u32,
                                            );
                                            FUN_004a1590(
                                                5,
                                                (((DAT_006d4988 as u32) & 0xffffff00)
                                                    | r8(o(o(DAT_006d4988, 0x16), iVar10)) as u32)
                                                    & 0xffff00ff,
                                                -1,
                                                uVar26 as u32,
                                                local_18,
                                                r16(o(adr!(DAT_0071b73c), iVar15)) as u32,
                                            );
                                            let mut pcVar21: *const i8 =
                                                ad(o(adr!(DAT_0070aee5), iVar20)) as *const i8;
                                            bVar32 = 0;
                                            let mut pcVar12: *const i8 = ad(o(
                                                o(adr!(DAT_0070aee9), iVar20),
                                                r16(o(adr!(DAT_0071b73c), iVar15)) as i32,
                                            ))
                                                as *const i8;
                                            iVar20 = 8;
                                            loop {
                                                if pcVar12.read() != 0
                                                    && (pcVar21.wrapping_add(1).read() == 5
                                                        || (pcVar21.wrapping_add(2).read() == 6
                                                            && (local_18 != 0
                                                                || r8(o(
                                                                    adr!(DAT_006d4aaa),
                                                                    iVar23,
                                                                )) == 0)))
                                                    && (pcVar21.read() == 0 || pcVar21.read() == 2)
                                                {
                                                    bVar32 = 1;
                                                }
                                                pcVar21 = pcVar21.wrapping_add(0x4e);
                                                pcVar12 = pcVar12.wrapping_add(0x4e);
                                                iVar20 = iVar20.wrapping_sub(1);
                                                if iVar20 == 0 {
                                                    break;
                                                }
                                            }
                                            if bVar32 != 0 && iVar9 == DAT_0071ecf4 {
                                                if local_18 == 0 {
                                                    iVar23 = 0x15b;
                                                } else {
                                                    iVar23 = 0x15c;
                                                }
                                                FUN_004a1d70(iVar23, DAT_0071ecf4);
                                                w16(
                                                    o(
                                                        adr!(DAT_0070ac4a),
                                                        DAT_0071ecf4.wrapping_mul(0x10),
                                                    ),
                                                    0x3b0,
                                                );
                                            }
                                            DAT_00655a84 = 0xfffd;
                                            if (bVar32 != 0
                                                || r8(o(o(DAT_006d4988, iVar10), 2)) != 0
                                                || (DAT_007092d9 == 0
                                                    && r32(o(o(DAT_006d4988, iVar10), 0x1b)) != -1))
                                                && ((iVar9 == DAT_0071ecf4 && {
                                                    FUN_004a34c0(local_10, unaff_EBX, 0x8d);
                                                    iVar11 != 0
                                                }) && 1 < DAT_00732e48)
                                                && DAT_006d4a02 == 2
                                            {
                                                _DAT_00732950 = (uVar26 as i32).wrapping_add(0x11c);
                                                FUN_004a1d70(0x12a, DAT_0071ecf4);
                                            }
                                        }
                                    }
                                }
                            }
                            if 0x2c < r16(o(r32(o(o(DAT_006d4988, 0x1f), iVar5)), 0x81)) {
                                local_36 = 0;
                                loop {
                                    let mut run = false;
                                    if local_36 == 0 {
                                        uVar25 = r8(o(o(DAT_006d4988, 0x16), iVar5)) as u32;
                                        unaff_EBX = uVar25.wrapping_mul(0x159);
                                        let k = uVar25.wrapping_mul(0x566) as i32;
                                        if r8(o(adr!(DAT_0070ad8d), k)) != 0 {
                                            cVar2 = r8(o(adr!(DAT_0070ad8c), k)) as i8;
                                            run = true;
                                        }
                                    } else {
                                        uVar25 = r8(o(o(DAT_006d4988, 0x16), iVar5)) as u32;
                                        unaff_EBX = uVar25.wrapping_mul(0x159);
                                        let k = uVar25.wrapping_mul(0x566) as i32;
                                        if r8(o(adr!(DAT_0070ad8f), k)) != 0 {
                                            cVar2 = r8(o(adr!(DAT_0070ad8e), k)) as i8;
                                            run = true;
                                        }
                                    }
                                    if run {
                                        uVar26 = _DAT_00732a3c as u16;
                                        if ((_DAT_00732a3c & 0xffff) as i32) < DAT_006d4980 {
                                            loop {
                                                unaff_EBX = (uVar26 as u32).wrapping_mul(0x23);
                                                iVar23 = o(DAT_006d4988, unaff_EBX as i32);
                                                if r8(o(iVar23, 3)) != 0
                                                    && r8(o(iVar23, 2)) != 0x0d
                                                    && r8(o(iVar23, 0x16)) == cVar2 as u8
                                                {
                                                    DAT_0073294c = r32(o(iVar23, 4));
                                                    DAT_007329b0 = r32(o(iVar23, 8));
                                                    DAT_007329b4 = r32(o(iVar23, 0xc));
                                                    DAT_0070a180 = r32(o(r32(o(iVar23, 0x1f)), 6));
                                                    DAT_0070acd8 = r32(o(r32(o(iVar23, 0x1f)), 10));
                                                    DAT_00703ad8 =
                                                        r32(o(r32(o(iVar23, 0x1f)), 0xe));
                                                    iVar10 = r32(o(r32(o(iVar23, 0x1f)), 0xa7));
                                                    if r8(o(iVar10, 7)) == 0 {
                                                        iVar10 = (r8(o(iVar10, 4)) as u32)
                                                            .wrapping_mul(0xef)
                                                            as i32;
                                                        FUN_0042a230(
                                                            iVar23,
                                                            r16(o(adr!(DAT_0052b14d), iVar10))
                                                                as u32,
                                                            iVar23,
                                                            r16(o(adr!(DAT_0052b14b), iVar10))
                                                                as i16,
                                                            r16(o(adr!(DAT_0052b14d), iVar10)),
                                                            r16(o(adr!(DAT_0052b14f), iVar10))
                                                                as i16,
                                                        );
                                                        DAT_0073294c =
                                                            DAT_0073294c.wrapping_add(DAT_0070ab18);
                                                        DAT_007329b0 =
                                                            DAT_007329b0.wrapping_add(DAT_0070ab14);
                                                        DAT_007329b4 =
                                                            DAT_007329b4.wrapping_add(DAT_0070ac20);
                                                        if r8(o(
                                                            o(DAT_006d4988, 2),
                                                            unaff_EBX as i32,
                                                        )) == 4
                                                        {
                                                            let c = (r8(o(
                                                                adr!(DAT_00709d0a),
                                                                (r16(o(adr!(DAT_0071b73c), iVar15))
                                                                    as i32)
                                                                    .wrapping_mul(3),
                                                            )) == 1)
                                                                as u32;
                                                            uVar25 = (0u32.wrapping_sub(c)
                                                                & 0x2000)
                                                                .wrapping_add(0x2000);
                                                        } else {
                                                            uVar25 = 0x2000;
                                                        }
                                                        unaff_EBX = (DAT_007329b4 as u32)
                                                            .wrapping_sub(DAT_006d4970 as u32);
                                                        iVar23 =
                                                            DAT_007329b0.wrapping_sub(DAT_006d4978);
                                                        iVar10 =
                                                            DAT_0073294c.wrapping_sub(DAT_006d4974);
                                                        if iVar10 < 0 {
                                                            iVar10 = 0i32.wrapping_sub(iVar10);
                                                        }
                                                        if iVar23 < 0 {
                                                            iVar23 = 0i32.wrapping_sub(iVar23);
                                                        }
                                                        if (unaff_EBX as i32) < 0 {
                                                            unaff_EBX =
                                                                0u32.wrapping_sub(unaff_EBX);
                                                        }
                                                        puVar31 = ((unaff_EBX as i32) >> 2) as u32;
                                                        if iVar23 < iVar10
                                                            && (unaff_EBX as i32) < iVar10
                                                        {
                                                            uVar13 = add3(
                                                                puVar31 as i32,
                                                                iVar10,
                                                                iVar23 >> 2,
                                                            )
                                                                as u32;
                                                            unaff_EBX = puVar31;
                                                        } else if iVar10 < iVar23
                                                            && (unaff_EBX as i32) < iVar23
                                                        {
                                                            uVar13 = add3(
                                                                puVar31 as i32,
                                                                iVar23,
                                                                iVar10 >> 2,
                                                            )
                                                                as u32;
                                                            unaff_EBX = puVar31;
                                                        } else {
                                                            uVar13 = add3(
                                                                iVar10 >> 2,
                                                                iVar23 >> 2,
                                                                unaff_EBX as i32,
                                                            )
                                                                as u32;
                                                        }
                                                        if uVar13 < uVar25
                                                            && r8(o(adr!(DAT_0071b776), iVar15))
                                                                == 0
                                                        {
                                                            if iVar9 == DAT_0071ecf4 {
                                                                FUN_004a1d70(0xd5, DAT_0071ecf4);
                                                            }
                                                            w8(o(adr!(DAT_0071b776), iVar15), 2);
                                                            w16(
                                                                o(adr!(DAT_0071b777), iVar15),
                                                                uVar26 as u32,
                                                            );
                                                            w32(
                                                                o(adr!(DAT_0071bcdd), iVar15),
                                                                0x760,
                                                            );
                                                        }
                                                    }
                                                }
                                                uVar26 = uVar26.wrapping_add(1);
                                                if !((uVar26 as i32) < DAT_006d4980) {
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                    local_36 = local_36.wrapping_add(1);
                                    if !(local_36 < 2) {
                                        break;
                                    }
                                }
                            }
                        }
                        iVar9 = r32(o(o(DAT_006d4988, 0x1f), iVar5));
                        if iVar9 != 0 {
                            pbVar14 = ad(o(iVar9, 0x12)) as *mut u8;
                            iVar15 = r32(o(iVar9, 0x73)).wrapping_sub(DAT_0071e6be as i32);
                            w32(o(iVar9, 0x73), iVar15 as u32);
                            if iVar15 < 1 {
                                w32(o(iVar9, 0x73), 0x7fff);
                                FUN_0040cd90(pbVar14, local_32);
                            }
                            local_36 = 0;
                            if pbVar14.read() != 0 {
                                'inner: loop {
                                    uVar13 = local_36 as u32;
                                    uVar26 =
                                        r16(o(o(iVar9, 0x53), (uVar13.wrapping_mul(2)) as i32));
                                    unaff_EBX = (unaff_EBX & 0xffff0000) | uVar26 as u32;
                                    uVar25 = uVar26 as u32;
                                    iVar15 = uVar25.wrapping_mul(0x23) as i32;
                                    iVar23 = o(iVar15, DAT_006d4988);
                                    'ecd4: {
                                        if r8(o(iVar23, 3)) == 0 || r8(o(iVar23, 2)) == 0x0d {
                                            remove_entry!();
                                            break 'ecd4;
                                        }
                                        bCond = (r32(o(iVar23, 0x1b)) == -1
                                            || ({
                                                cVar2 = r8(o(o(DAT_006d4988, 2), iVar5)) as i8;
                                                cVar2 != 4 && cVar2 != 5
                                            } || 2 < r16(o(r32(o(iVar23, 0x1f)), 0x81))))
                                            as i32;
                                        if bCond != 0 {
                                            iVar23 = r32(o(
                                                o(iVar9, 0x13),
                                                (uVar13.wrapping_mul(4)) as i32,
                                            ))
                                            .wrapping_sub(DAT_0071e6be as i32);
                                            w32(
                                                o(o(iVar9, 0x13), (uVar13.wrapping_mul(4)) as i32),
                                                iVar23 as u32,
                                            );
                                            bCond = (iVar23 < 1) as i32;
                                        }
                                        if bCond != 0 {
                                            iVar23 = o(DAT_006d4988, iVar5);
                                            'ebc5: {
                                                if r32(o(iVar23, 0x1b)) == -1 {
                                                    bVar4 = r8(o(iVar23, 2));
                                                    if bVar4 < 8 {
                                                        match bVar4 {
                                                            3 | 4 | 5 => {
                                                                iVar23 = r32(o(
                                                                    r32(o(iVar23, 0x1f)),
                                                                    0xa7,
                                                                ));
                                                                iVar10 = FUN_0042b150(iVar23);
                                                                if r8(o(iVar10, 0x34)) == 0x1e {
                                                                    local_36 = 0x10;
                                                                } else if (r16(o(iVar23, 0x68))
                                                                    as i16)
                                                                    == -1
                                                                {
                                                                    if r16(o(iVar23, 0x66))
                                                                        == uVar26
                                                                    {
                                                                        FUN_0040ed40(
                                                                            pbVar14, local_32,
                                                                            unaff_EBX,
                                                                        );
                                                                    } else if r8(o(iVar10, 0x34))
                                                                        == 0x12
                                                                        && r16(o(iVar10, 0x18))
                                                                            == uVar26
                                                                    {
                                                                        FUN_0040ed40(
                                                                            pbVar14, local_32,
                                                                            unaff_EBX,
                                                                        );
                                                                    } else {
                                                                        local_10 = r32(o(
                                                                            r32(o(
                                                                                o(
                                                                                    DAT_006d4988,
                                                                                    0x1f,
                                                                                ),
                                                                                iVar15,
                                                                            )),
                                                                            0xa7,
                                                                        ))
                                                                            as u32;
                                                                        iVar23 = FUN_0042b150(
                                                                            local_10 as i32,
                                                                        );
                                                                        let mut to829 = false;
                                                                        let mut fin = true;
                                                                        cmps!(
                                                                            o(
                                                                                adr!(DAT_00703ca0),
                                                                                (r8(o(iVar23, 10))
                                                                                    as i32)
                                                                                    .wrapping_mul(
                                                                                        0x55
                                                                                    )
                                                                            ),
                                                                            adr!(DAT_00523c08),
                                                                            0xe
                                                                        );
                                                                        if bVar32 == 0 {
                                                                            cmps!(
                                                                                o(
                                                                                    adr!(DAT_00703ca0),
                                                                                    (r8(o(iVar23, 10)) as i32).wrapping_mul(0x55)
                                                                                ),
                                                                                adr!(DAT_00523bf4),
                                                                                0x13
                                                                            );
                                                                            if bVar32 == 0 {
                                                                                cmps!(
                                                                                    o(
                                                                                        adr!(DAT_00703ca0),
                                                                                        (r8(o(iVar23, 10)) as i32).wrapping_mul(0x55)
                                                                                    ),
                                                                                    adr!(DAT_00523be4),
                                                                                    0xf
                                                                                );
                                                                                if bVar32 == 0 {
                                                                                    fin = false;
                                                                                    cmps!(
                                                                                        o(
                                                                                            adr!(DAT_00703ca0),
                                                                                            (r8(o(iVar23, 10)) as i32).wrapping_mul(0x55)
                                                                                        ),
                                                                                        adr!(DAT_00523bd0),
                                                                                        0x13
                                                                                    );
                                                                                    if bVar32 != 0 {
                                                                                        iVar10 = FUN_0042b150(r32(o(
                                                                                            r32(o(
                                                                                                o(
                                                                                                    (r8(o(local_10 as i32, 5)) as i32).wrapping_mul(0x23),
                                                                                                    0x1f,
                                                                                                ),
                                                                                                DAT_006d4988,
                                                                                            )),
                                                                                            0xa7,
                                                                                        )));
                                                                                        cmps!(
                                                                                            o(
                                                                                                adr!(DAT_00703ca0),
                                                                                                (r8(o(iVar10, 10)) as i32).wrapping_mul(0x55)
                                                                                            ),
                                                                                            adr!(DAT_00523c08),
                                                                                            0xe
                                                                                        );
                                                                                        if bVar32
                                                                                            == 0
                                                                                        {
                                                                                            cmps!(
                                                                                                o(
                                                                                                    adr!(DAT_00703ca0),
                                                                                                    (r8(o(iVar10, 10)) as i32).wrapping_mul(0x55)
                                                                                                ),
                                                                                                adr!(DAT_00523be4),
                                                                                                0xf
                                                                                            );
                                                                                            if bVar32 == 0 {
                                                                                                to829 = true;
                                                                                            }
                                                                                        }
                                                                                        if !to829 {
                                                                                            FUN_0040ed40(pbVar14, local_32, unaff_EBX);
                                                                                        }
                                                                                    } else {
                                                                                        to829 =
                                                                                            true;
                                                                                    }
                                                                                    if to829 {
                                                                                        if (r16(o(local_10 as i32, 0x68)) as i16) == -1
                                                                                            && {
                                                                                                cVar2 = r8(o(iVar23, 0x34)) as i8;
                                                                                                cVar2 != 0x12
                                                                                            }
                                                                                            && cVar2 != 0x15
                                                                                            && cVar2 != 0x1e
                                                                                        {
                                                                                            iVar23 = o(DAT_006d4988, iVar5);
                                                                                            DAT_0073294c = r32(o(iVar23, 4));
                                                                                            DAT_007329b0 = r32(o(iVar23, 8));
                                                                                            DAT_007329b4 = r32(o(iVar23, 0xc));
                                                                                            DAT_0070a180 = r32(o(r32(o(iVar23, 0x1f)), 6));
                                                                                            DAT_0070acd8 = r32(o(r32(o(iVar23, 0x1f)), 10));
                                                                                            iVar10 = o(iVar15, DAT_006d4988);
                                                                                            DAT_00703ad8 = r32(o(r32(o(iVar23, 0x1f)), 0xe));
                                                                                            DAT_006d4974 = r32(o(iVar10, 4));
                                                                                            DAT_006d4978 = r32(o(iVar10, 8));
                                                                                            DAT_006d4970 = r32(o(iVar10, 0xc));
                                                                                            DAT_0071e6c4 = r32(o(r32(o(iVar10, 0x1f)), 6));
                                                                                            DAT_00732f38 = r32(o(r32(o(iVar10, 0x1f)), 10));
                                                                                            DAT_00732fe8 = r32(o(r32(o(iVar10, 0x1f)), 0xe));
                                                                                            uVar25 = FUN_0040f7e0(uVar26, local_32);
                                                                                            if (uVar25 as i16) != 0 {
                                                                                                FUN_004110e0(local_32 as u32, uVar25, unaff_EBX, 0);
                                                                                                FUN_004110e0(unaff_EBX, 0xffff, local_32 as u32, 0);
                                                                                            }
                                                                                            break 'ebc5;
                                                                                        }
                                                                                        FUN_0040ed40(pbVar14, local_32, unaff_EBX);
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        if fin {
                                                                            FUN_0040ed40(
                                                                                pbVar14, local_32,
                                                                                unaff_EBX,
                                                                            );
                                                                        }
                                                                    }
                                                                } else {
                                                                    local_36 = 0x10;
                                                                }
                                                                break 'ecd4;
                                                            }
                                                            6 | 7 => {
                                                                if r32(o(
                                                                    o(
                                                                        (r16(o(
                                                                            r32(o(iVar23, 0x1f)),
                                                                            0x83,
                                                                        ))
                                                                            as i32)
                                                                            .wrapping_mul(0x23),
                                                                        0x1b,
                                                                    ),
                                                                    DAT_006d4988,
                                                                )) != -1
                                                                    && (uVar25 as i32)
                                                                        < DAT_006d4980
                                                                {
                                                                    iVar23 = FUN_0042b150(r32(o(
                                                                        r32(o(
                                                                            o(DAT_006d4988, 0x1f),
                                                                            iVar15,
                                                                        )),
                                                                        0xa7,
                                                                    )));
                                                                    cmps!(
                                                                        o(
                                                                            adr!(DAT_00703ca0),
                                                                            (r8(o(iVar23, 0xb))
                                                                                as i32)
                                                                                .wrapping_mul(0x55)
                                                                        ),
                                                                        adr!(DAT_00523c18),
                                                                        0xf
                                                                    );
                                                                    if bVar32 != 0
                                                                        && r8(o(iVar23, 0x34))
                                                                            == 0x12
                                                                        && r8(o(iVar23, 0x35)) == 2
                                                                        && r16(o(iVar23, 0x18))
                                                                            == local_32
                                                                    {
                                                                        FUN_0040ed40(
                                                                            pbVar14, local_32,
                                                                            unaff_EBX,
                                                                        );
                                                                        break 'ecd4;
                                                                    }
                                                                }
                                                                iVar23 = o(DAT_006d4988, iVar5);
                                                                DAT_006d4974 = r32(o(iVar23, 4));
                                                                DAT_006d4978 = r32(o(iVar23, 8));
                                                                DAT_006d4970 = r32(o(iVar23, 0xc));
                                                                DAT_0071e6c4 =
                                                                    r32(o(r32(o(iVar23, 0x1f)), 6));
                                                                DAT_00732f38 = r32(o(
                                                                    r32(o(iVar23, 0x1f)),
                                                                    10,
                                                                ));
                                                                iVar10 = o(iVar15, DAT_006d4988);
                                                                DAT_00732fe8 = r32(o(
                                                                    r32(o(iVar23, 0x1f)),
                                                                    0xe,
                                                                ));
                                                                if r32(o(iVar10, 0x1f)) == 0 {
                                                                    if DAT_007092c1 == 0 && {
                                                                        uVar25 = FUN_00413010(
                                                                            local_32, uVar26,
                                                                        );
                                                                        (uVar25 as i16) != 0
                                                                    } {
                                                                        FUN_00413280(
                                                                            local_32, unaff_EBX,
                                                                        );
                                                                        FUN_004a0ef0(local_32);
                                                                    }
                                                                } else {
                                                                    DAT_0073294c =
                                                                        r32(o(iVar10, 4));
                                                                    DAT_007329b0 =
                                                                        r32(o(iVar10, 8));
                                                                    DAT_007329b4 =
                                                                        r32(o(iVar10, 0xc));
                                                                    DAT_0070a180 = r32(o(
                                                                        r32(o(iVar10, 0x1f)),
                                                                        6,
                                                                    ));
                                                                    uVar17 = r32(o(
                                                                        r32(o(iVar10, 0x1f)),
                                                                        10,
                                                                    ))
                                                                        as u32;
                                                                    DAT_00703ad8 = r32(o(
                                                                        r32(o(iVar10, 0x1f)),
                                                                        0xe,
                                                                    ));
                                                                    DAT_0070acd8 = uVar17 as i32;
                                                                    uVar13 = FUN_0040f7e0(
                                                                        local_32, uVar26,
                                                                    );
                                                                    if (uVar13 as i16) != 0 {
                                                                        if (uVar25 as i32)
                                                                            < DAT_006d4980
                                                                        {
                                                                            FUN_004a0ef0(local_32);
                                                                            FUN_00410810(
                                                                                local_32 as u32,
                                                                                unaff_EBX,
                                                                                ((((uVar17 >> 0x10)
                                                                                    as u16
                                                                                    as u32)
                                                                                    << 16)
                                                                                    | (uVar13 as u16
                                                                                        as u32))
                                                                                    as i32,
                                                                            );
                                                                        } else {
                                                                            if r8(o(
                                                                                adr!(DAT_00525bf7),
                                                                                r8(o(o(DAT_006d4988, iVar5), 3)) as i32,
                                                                            )) == 0
                                                                            {
                                                                                FUN_004a0ef0(local_32);
                                                                                cVar2 = r8(o(o(DAT_006d4988, 3), iVar5)) as i8;
                                                                                if cVar2 == -0x73 || cVar2 == -0x72 {
                                                                                    uVar33 = 0x84;
                                                                                } else {
                                                                                    uVar33 = 0x83;
                                                                                }
                                                                            } else {
                                                                                puVar31 = unaff_EBX;
                                                                                if r8(o(
                                                                                    adr!(DAT_00525bf7),
                                                                                    r8(o(o(DAT_006d4988, iVar15), 3)) as i32,
                                                                                )) == 0
                                                                                {
                                                                                    FUN_004a0ef0(puVar31 as u16);
                                                                                } else {
                                                                                    if r16(o(
                                                                                        r32(o(r32(o(o(DAT_006d4988, iVar5), 0x1f)), 0xa3)),
                                                                                        4,
                                                                                    )) == uVar26
                                                                                    {
                                                                                        puVar31 = local_32 as u32;
                                                                                        FUN_004a0ef0(puVar31 as u16);
                                                                                    } else if r16(o(
                                                                                        r32(o(r32(o(o(DAT_006d4988, iVar15), 0x1f)), 0xa3)),
                                                                                        4,
                                                                                    )) == local_32
                                                                                    {
                                                                                        FUN_004a0ef0(puVar31 as u16);
                                                                                    }
                                                                                }
                                                                                uVar33 = 0x81;
                                                                            }
                                                                            FUN_00412700(
                                                                                uVar8, uVar33,
                                                                            );
                                                                            FUN_00412700(
                                                                                uVar25, 0x81,
                                                                            );
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            _ => {
                                                                DAT_006d4974 = r32(o(iVar23, 4));
                                                                DAT_006d4978 = r32(o(iVar23, 8));
                                                                DAT_006d4970 = r32(o(iVar23, 0xc));
                                                                DAT_0071e6c4 =
                                                                    r32(o(r32(o(iVar23, 0x1f)), 6));
                                                                DAT_00732f38 = r32(o(
                                                                    r32(o(iVar23, 0x1f)),
                                                                    10,
                                                                ));
                                                                DAT_00732fe8 = r32(o(
                                                                    r32(o(iVar23, 0x1f)),
                                                                    0xe,
                                                                ));
                                                                if DAT_00732a38 <= (uVar25 as i32)
                                                                    && {
                                                                        uVar25 = FUN_00413010(
                                                                            local_32, uVar26,
                                                                        );
                                                                        (uVar25 as i16) != 0
                                                                    }
                                                                {
                                                                    if DAT_007092d5 == 0 {
                                                                        FUN_0040f260(
                                                                            local_32, uVar26,
                                                                        );
                                                                    } else {
                                                                        FUN_004110e0(
                                                                            local_32 as u32,
                                                                            0xffff,
                                                                            unaff_EBX,
                                                                            0,
                                                                        );
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if 2 < r16(o(r32(o(iVar23, 0x1f)), 0x81)) || {
                                                        cVar2 =
                                                            r8(o(o(DAT_006d4988, 2), iVar15)) as i8;
                                                        cVar2 != 4 && cVar2 != 5
                                                    } {
                                                        iVar10 = r32(o(r32(o(iVar23, 0x1f)), 0xa7));
                                                        if r16(o(iVar10, 0x70)) != uVar26 {
                                                            DAT_006d4974 = r32(o(iVar23, 4));
                                                            DAT_006d4978 = r32(o(iVar23, 8));
                                                            DAT_006d4970 = r32(o(iVar23, 0xc));
                                                            DAT_0071e6c4 =
                                                                r32(o(r32(o(iVar23, 0x1f)), 6));
                                                            DAT_00732f38 =
                                                                r32(o(r32(o(iVar23, 0x1f)), 10));
                                                            DAT_00732fe8 =
                                                                r32(o(r32(o(iVar23, 0x1f)), 0xe));
                                                            iVar23 = r32(o(
                                                                o(DAT_006d4988, 0x1f),
                                                                iVar15,
                                                            ));
                                                            if iVar23 == 0 {
                                                                if DAT_007092c1 == 0
                                                                    && {
                                                                        uVar25 = FUN_00413010(
                                                                            local_32, uVar26,
                                                                        );
                                                                        (uVar25 as i16) != 0
                                                                    }
                                                                    && {
                                                                        FUN_0040f260(
                                                                            local_32, uVar26,
                                                                        );
                                                                        DAT_007092d5 != 0
                                                                    }
                                                                {
                                                                    iVar23 = (r8(o(
                                                                        o(DAT_006d4988, 0x16),
                                                                        iVar15,
                                                                    ))
                                                                        as i32)
                                                                        .wrapping_mul(0x566);
                                                                    if r8(o(
                                                                        adr!(DAT_0070ad44),
                                                                        iVar23,
                                                                    )) != 0x14
                                                                        && r8(o(
                                                                            adr!(DAT_0070b215),
                                                                            iVar23,
                                                                        )) != 0x14
                                                                        && {
                                                                            uVar25 = r8(o(
                                                                                o(
                                                                                    DAT_006d4988,
                                                                                    0x16,
                                                                                ),
                                                                                iVar5,
                                                                            ))
                                                                                as u32;
                                                                            r8(o(
                                                                                adr!(DAT_0070ad44),
                                                                                uVar25.wrapping_mul(
                                                                                    0x566,
                                                                                )
                                                                                    as i32,
                                                                            )) != 0x14
                                                                        }
                                                                        && r8(o(
                                                                            adr!(DAT_0070b215),
                                                                            uVar25
                                                                                .wrapping_mul(0x566)
                                                                                as i32,
                                                                        )) != 0x14
                                                                    {
                                                                        FUN_00413280(
                                                                            local_32, unaff_EBX,
                                                                        );
                                                                    }
                                                                    FUN_004110e0(
                                                                        local_32 as u32,
                                                                        0xffff,
                                                                        unaff_EBX,
                                                                        0,
                                                                    );
                                                                }
                                                            } else {
                                                                local_10 =
                                                                    r32(o(iVar23, 0xa7)) as u32;
                                                                iVar23 =
                                                                    FUN_0042b150(local_10 as i32);
                                                                cmps!(
                                                                    o(
                                                                        adr!(DAT_00703ca0),
                                                                        (r8(o(iVar23, 0xb)) as i32)
                                                                            .wrapping_mul(0x55)
                                                                    ),
                                                                    adr!(DAT_00523c18),
                                                                    0xf
                                                                );
                                                                if bVar32 != 0
                                                                    && r16(o(iVar23, 0x18))
                                                                        == local_32
                                                                {
                                                                    FUN_0040ed40(
                                                                        pbVar14, local_32,
                                                                        unaff_EBX,
                                                                    );
                                                                    break 'ecd4;
                                                                }
                                                                iVar23 = o(iVar15, DAT_006d4988);
                                                                DAT_0073294c = r32(o(iVar23, 4));
                                                                DAT_007329b0 = r32(o(iVar23, 8));
                                                                DAT_007329b4 = r32(o(iVar23, 0xc));
                                                                DAT_0070a180 =
                                                                    r32(o(r32(o(iVar23, 0x1f)), 6));
                                                                DAT_0070acd8 = r32(o(
                                                                    r32(o(iVar23, 0x1f)),
                                                                    10,
                                                                ));
                                                                DAT_00703ad8 = r32(o(
                                                                    r32(o(iVar23, 0x1f)),
                                                                    0xe,
                                                                ));
                                                                uVar13 =
                                                                    FUN_0040f7e0(local_32, uVar26);
                                                                sVar6 = uVar13 as i16;
                                                                in_stack_ffffffc4 =
                                                                    (in_stack_ffffffc4
                                                                        & 0xffff0000)
                                                                        | (sVar6 as u16 as u32);
                                                                if sVar6 != 0 {
                                                                    if DAT_007092c1 == 0 {
                                                                        cVar2 = r8(o(
                                                                            o(DAT_006d4988, 2),
                                                                            iVar15,
                                                                        ))
                                                                            as i8;
                                                                        if cVar2 == 0
                                                                            || cVar2 == 1
                                                                            || cVar2 == 2
                                                                        {
                                                                            w16(
                                                                                o(iVar10, 0x70),
                                                                                uVar26 as u32,
                                                                            );
                                                                            sVar35 = r16(o(r32(o(o(DAT_006d4988, 0x1f), iVar5)), 0x79)) as i16;
                                                                            uVar19 = r16(o(
                                                                                o(
                                                                                    DAT_006d4988,
                                                                                    iVar15,
                                                                                ),
                                                                                0x10,
                                                                            ))
                                                                            .wrapping_sub(r16(o(
                                                                                o(
                                                                                    DAT_006d4988,
                                                                                    iVar5,
                                                                                ),
                                                                                0x10,
                                                                            )));
                                                                            if 0x7fff < uVar19 {
                                                                                uVar19 = 0u16
                                                                                    .wrapping_sub(
                                                                                        uVar19,
                                                                                    );
                                                                            }
                                                                            if 0x4000 < uVar19 {
                                                                                uVar19 = 0x8000u16
                                                                                    .wrapping_sub(
                                                                                        uVar19,
                                                                                    );
                                                                            }
                                                                            uVar22 = r16(o(r32(o(o(DAT_006d4988, iVar15), 0x1f)), 0x79));
                                                                            if uVar19 < 0x4000 {
                                                                                uVar13 =
                                                                                    FUN_004187e0(
                                                                                        uVar22,
                                                                                        uVar19,
                                                                                    );
                                                                                in_stack_ffffffc4 = (((sVar35.wrapping_sub(uVar13 as i16)) as u16 as u32) << 16)
                                                                                    | (sVar6 as u16 as u32);
                                                                            } else {
                                                                                uVar13 =
                                                                                    FUN_004187e0(
                                                                                        uVar22,
                                                                                        uVar19,
                                                                                    );
                                                                                in_stack_ffffffc4 = (((sVar35.wrapping_add(uVar13 as i16)) as u16 as u32) << 16)
                                                                                    | (sVar6 as u16 as u32);
                                                                            }
                                                                            if (in_stack_ffffffc4
                                                                                as i32)
                                                                                < 0
                                                                            {
                                                                                in_stack_ffffffc4 = ((((in_stack_ffffffc4 >> 16) as i16).wrapping_neg() as u16 as u32) << 16)
                                                                                    | (in_stack_ffffffc4 as u16 as u32);
                                                                            }
                                                                            iVar29 = o(
                                                                                DAT_006d4988,
                                                                                iVar5,
                                                                            );
                                                                            iVar20 = o(
                                                                                iVar15,
                                                                                DAT_006d4988,
                                                                            );
                                                                            local_18 =
                                                                                (r32(o(iVar29, 4))
                                                                                    as u32)
                                                                                    .wrapping_sub(
                                                                                        r32(o(
                                                                                            iVar20,
                                                                                            4,
                                                                                        ))
                                                                                            as u32,
                                                                                    );
                                                                            iVar23 =
                                                                                r32(o(iVar29, 8))
                                                                                    .wrapping_sub(
                                                                                        r32(o(
                                                                                            iVar20,
                                                                                            8,
                                                                                        )),
                                                                                    );
                                                                            iVar20 =
                                                                                r32(o(iVar29, 0xc))
                                                                                    .wrapping_sub(
                                                                                        r32(o(
                                                                                            iVar20,
                                                                                            0xc,
                                                                                        )),
                                                                                    );
                                                                            iVar20 = iVar20
                                                                                .wrapping_mul(
                                                                                    iVar20,
                                                                                );
                                                                            iVar11 = iVar23
                                                                                .wrapping_mul(
                                                                                    iVar23,
                                                                                );
                                                                            iVar24 = local_18
                                                                                .wrapping_mul(
                                                                                    local_18,
                                                                                )
                                                                                as i32;
                                                                            iVar28 = iVar11
                                                                                .wrapping_add(
                                                                                    iVar20,
                                                                                )
                                                                                .wrapping_add(
                                                                                    iVar24,
                                                                                );
                                                                            if iVar28 < 0x33 {
                                                                                uStack_34 = 0;
                                                                                uStack_38 = 100;
                                                                            } else {
                                                                                iVar16 = (in_stack_ffffffc4 >> 16) as i16 as i32;
                                                                                local_18 = (iVar16
                                                                                    as u32)
                                                                                    .wrapping_mul(
                                                                                        local_18,
                                                                                    )
                                                                                    .wrapping_mul(
                                                                                        1000,
                                                                                    );
                                                                                uStack_34 =
                                                                                    ((local_18
                                                                                        as i32)
                                                                                        / iVar28)
                                                                                        as u16;
                                                                                iVar23 = (iVar23
                                                                                    as u32)
                                                                                    .wrapping_mul(
                                                                                        iVar16
                                                                                            as u32,
                                                                                    )
                                                                                    .wrapping_mul(
                                                                                        1000,
                                                                                    )
                                                                                    as i32;
                                                                                uStack_38 = (iVar23
                                                                                    / iVar28)
                                                                                    as u16;
                                                                            }
                                                                            if r8(o(
                                                                                r32(o(
                                                                                    iVar29, 0x1f,
                                                                                )),
                                                                                0x89,
                                                                            )) != 0
                                                                            {
                                                                                FUN_00483410(
                                                                                    r16(o(
                                                                                        iVar29,
                                                                                        0x12,
                                                                                    ))
                                                                                        as i16,
                                                                                    r16(o(
                                                                                        iVar29,
                                                                                        0x10,
                                                                                    ))
                                                                                        as i16,
                                                                                    iVar29,
                                                                                );
                                                                            }
                                                                            iVar29 = r32(o(
                                                                                o(
                                                                                    DAT_006d4988,
                                                                                    0x1f,
                                                                                ),
                                                                                iVar5,
                                                                            ));
                                                                            iVar28 = o(
                                                                                DAT_006d4988,
                                                                                iVar5,
                                                                            );
                                                                            uVar19 = r16(o(
                                                                                iVar29, 0x8a,
                                                                            ));
                                                                            uStack_34 = uStack_34
                                                                                .wrapping_add(
                                                                                    uVar19,
                                                                                );
                                                                            if ((uStack_34
                                                                                ^ uVar19)
                                                                                & 0x8000)
                                                                                != 0
                                                                            {
                                                                                uStack_34 = uVar19;
                                                                            }
                                                                            uVar19 = r16(o(
                                                                                iVar29, 0x8c,
                                                                            ));
                                                                            uStack_38 = uStack_38
                                                                                .wrapping_add(
                                                                                    uVar19,
                                                                                );
                                                                            if ((uStack_38
                                                                                ^ uVar19)
                                                                                & 0x8000)
                                                                                != 0
                                                                            {
                                                                                uStack_38 = uVar19;
                                                                            }
                                                                            uVar19 = r16(o(
                                                                                iVar28, 0x10,
                                                                            ));
                                                                            uVar17 = FUN_00419100(
                                                                                uStack_34 as i16
                                                                                    as i32,
                                                                                uStack_38 as i16
                                                                                    as i32,
                                                                            );
                                                                            w16(
                                                                                o(iVar28, 0x10),
                                                                                uVar17 as i16
                                                                                    as u32,
                                                                            );
                                                                            uVar7 = FUN_004184f0(
                                                                                uStack_34,
                                                                            )
                                                                                as u16;
                                                                            w16(o(o(DAT_006d4988, 0x12), iVar15), uVar7 as u32);
                                                                            uVar27 =
                                                                                ((in_stack_ffffffc4
                                                                                    >> 16)
                                                                                    as i16)
                                                                                    .wrapping_mul(
                                                                                        100,
                                                                                    )
                                                                                    as u16;
                                                                            w16(o(local_10 as i32, 0xd), r16(o(o(DAT_006d4988, 0x12), iVar15)) as u32);
                                                                            uVar22 = uVar27;
                                                                            if 0x7fff < uVar27 {
                                                                                uVar22 = 0x7fff;
                                                                            }
                                                                            if uVar19 < r16(o(o(iVar15, DAT_006d4988), 0x10)) {
                                                                                uVar22 = 0u16.wrapping_sub(uVar22);
                                                                            }
                                                                            w16(o(r32(o(o(iVar15, DAT_006d4988), 0x1f)), 0x77), uVar22 as u32);
                                                                            iVar28 = o(
                                                                                DAT_006d4988,
                                                                                iVar15,
                                                                            );
                                                                            FUN_00483410(
                                                                                r16(o(iVar28, 0x12))
                                                                                    as i16,
                                                                                r16(o(iVar28, 0x10))
                                                                                    as i16,
                                                                                iVar28,
                                                                            );
                                                                            FUN_00483540(r16(o(o(DAT_006d4988, iVar15), 0x14)) as i16, 0, o(DAT_006d4988, iVar15));
                                                                            w16(o(r32(o(r32(o(o(DAT_006d4988, 0x1f), iVar15)), 0xa7)), 0x70), local_32 as u32);
                                                                            iVar11 = iVar20
                                                                                .wrapping_add(
                                                                                    iVar24,
                                                                                )
                                                                                .wrapping_add(
                                                                                    iVar11,
                                                                                );
                                                                            if iVar11 < 0x33 {
                                                                                uStack_34 = 0;
                                                                                uStack_38 = 100;
                                                                            } else {
                                                                                sVar6 = (in_stack_ffffffc4 >> 16) as i16;
                                                                                uStack_34 = (((sVar6 as i32 as u32).wrapping_mul(local_18).wrapping_mul(1000)) as i32 / iVar11) as u16;
                                                                                uStack_38 = (((iVar23 as u32).wrapping_mul(sVar6 as i32 as u32).wrapping_mul(1000)) as i32 / iVar11) as u16;
                                                                            }
                                                                            iVar23 = o(
                                                                                iVar15,
                                                                                DAT_006d4988,
                                                                            );
                                                                            if r8(o(
                                                                                r32(o(
                                                                                    iVar23, 0x1f,
                                                                                )),
                                                                                0x89,
                                                                            )) != 0
                                                                            {
                                                                                FUN_00483410(
                                                                                    r16(o(
                                                                                        iVar23,
                                                                                        0x12,
                                                                                    ))
                                                                                        as i16,
                                                                                    r16(o(
                                                                                        iVar23,
                                                                                        0x10,
                                                                                    ))
                                                                                        as i16,
                                                                                    iVar23,
                                                                                );
                                                                            }
                                                                            iVar23 = o(
                                                                                iVar15,
                                                                                DAT_006d4988,
                                                                            );
                                                                            uVar19 = r16(o(
                                                                                r32(o(
                                                                                    iVar23, 0x1f,
                                                                                )),
                                                                                0x8a,
                                                                            ));
                                                                            uStack_34 = uStack_34
                                                                                .wrapping_add(
                                                                                    uVar19,
                                                                                );
                                                                            if ((uStack_34
                                                                                ^ uVar19)
                                                                                & 0x8000)
                                                                                != 0
                                                                            {
                                                                                uStack_34 = uVar19;
                                                                            }
                                                                            uVar19 = r16(o(
                                                                                r32(o(
                                                                                    iVar23, 0x1f,
                                                                                )),
                                                                                0x8c,
                                                                            ));
                                                                            uStack_38 = uStack_38
                                                                                .wrapping_add(
                                                                                    uVar19,
                                                                                );
                                                                            if ((uStack_38
                                                                                ^ uVar19)
                                                                                & 0x8000)
                                                                                != 0
                                                                            {
                                                                                uStack_38 = uVar19;
                                                                            }
                                                                            uVar19 = r16(o(
                                                                                iVar23, 0x10,
                                                                            ));
                                                                            uVar17 = FUN_00419100(
                                                                                uStack_34 as i16
                                                                                    as i32,
                                                                                uStack_38 as i16
                                                                                    as i32,
                                                                            );
                                                                            w16(
                                                                                o(iVar23, 0x10),
                                                                                uVar17 as i16
                                                                                    as u32,
                                                                            );
                                                                            uVar7 = FUN_004184f0(
                                                                                uStack_34,
                                                                            )
                                                                                as u16;
                                                                            w16(o(o(DAT_006d4988, 0x12), iVar5), uVar7 as u32);
                                                                            w16(o(iVar10, 0xd), r16(o(o(DAT_006d4988, 0x12), iVar5)) as u32);
                                                                            w8(o(r32(o(o(DAT_006d4988, 0x1f), iVar15)), 0x90), 1);
                                                                            w8(o(r32(o(o(DAT_006d4988, 0x1f), iVar15)), 0x89), 1);
                                                                            if 0x7fff < uVar27 {
                                                                                uVar27 = 0x7fff;
                                                                            }
                                                                            if uVar19 < r16(o(o(DAT_006d4988, iVar5), 0x10)) {
                                                                                uVar27 = 0u16.wrapping_sub(uVar27);
                                                                            }
                                                                            w16(o(r32(o(o(DAT_006d4988, iVar5), 0x1f)), 0x77), uVar27 as u32);
                                                                            iVar23 = o(
                                                                                DAT_006d4988,
                                                                                iVar5,
                                                                            );
                                                                            FUN_00483410(
                                                                                r16(o(iVar23, 0x12))
                                                                                    as i16,
                                                                                r16(o(iVar23, 0x10))
                                                                                    as i16,
                                                                                iVar23,
                                                                            );
                                                                            FUN_00483540(r16(o(o(DAT_006d4988, iVar5), 0x14)) as i16, 0, o(DAT_006d4988, iVar5));
                                                                            FUN_004a1d70(0xdb, r32(o(o(DAT_006d4988, 0x1b), iVar5)));
                                                                            FUN_0040a1a0(0x1f, uVar8, r32(o(o(DAT_006d4988, 0x1b), iVar5)));
                                                                            FUN_0040a1a0(0x20, uVar25, r32(o(o(DAT_006d4988, 0x1b), iVar5)));
                                                                        }
                                                                        if DAT_007092d5 != 0
                                                                            || {
                                                                                cVar2 = r8(o(o(DAT_006d4988, 2), iVar15)) as i8;
                                                                                cVar2 == 4
                                                                            }
                                                                            || (cVar2 == 3
                                                                                || cVar2 == 5)
                                                                        {
                                                                            FUN_004110e0(
                                                                                unaff_EBX,
                                                                                in_stack_ffffffc4,
                                                                                local_32 as u32,
                                                                                0,
                                                                            );
                                                                            iVar23 = o(
                                                                                DAT_006d4988,
                                                                                iVar5,
                                                                            );
                                                                            if r8(o(r32(o(o(DAT_006d4988, 0x1f), iVar5)), 0x90)) != 0 {
                                                                                FUN_00483410(r16(o(iVar23, 0x12)) as i16, r16(o(iVar23, 0x10)) as i16, iVar23);
                                                                                FUN_00483540(r16(o(o(DAT_006d4988, iVar5), 0x14)) as i16, 0, o(DAT_006d4988, iVar5));
                                                                            }
                                                                            iVar23 = r32(o(
                                                                                o(
                                                                                    DAT_006d4988,
                                                                                    0x1f,
                                                                                ),
                                                                                iVar5,
                                                                            ));
                                                                            iVar23 = FUN_00413d00(
                                                                                r16(o(iVar23, 0x91))
                                                                                    as i16
                                                                                    as i32,
                                                                                r16(o(iVar23, 0x93))
                                                                                    as i16
                                                                                    as i32,
                                                                                r16(o(iVar23, 0x95))
                                                                                    as i16
                                                                                    as i32,
                                                                                (DAT_0073294c
                                                                                    as i16)
                                                                                    .wrapping_sub(
                                                                                        DAT_0070a180
                                                                                            as i16,
                                                                                    )
                                                                                    as i32,
                                                                                (DAT_007329b0
                                                                                    as i16)
                                                                                    .wrapping_sub(
                                                                                        DAT_0070acd8
                                                                                            as i16,
                                                                                    )
                                                                                    as i32,
                                                                                (DAT_007329b4
                                                                                    as i16)
                                                                                    .wrapping_sub(
                                                                                        DAT_00703ad8
                                                                                            as i16,
                                                                                    )
                                                                                    as i32,
                                                                            );
                                                                            FUN_004110e0(
                                                                                local_32 as u32,
                                                                                0xffff,
                                                                                unaff_EBX,
                                                                                ((iVar23 as i16)
                                                                                    < 0)
                                                                                    as u32,
                                                                            );
                                                                        }
                                                                    } else {
                                                                        w16(o(r32(o(o(DAT_006d4988, 0x1f), iVar5)), 0x79), 0);
                                                                        w16(o(iVar10, 0xbd), 0);
                                                                        w32(
                                                                            o(
                                                                                o(DAT_006d4988, 4),
                                                                                iVar5,
                                                                            ),
                                                                            DAT_00732f7c as u32,
                                                                        );
                                                                        w32(o(r32(o(o(DAT_006d4988, iVar5), 0x1f)), 6), r32(o(o(DAT_006d4988, iVar5), 4)) as u32);
                                                                        w32(
                                                                            o(
                                                                                o(DAT_006d4988, 8),
                                                                                iVar5,
                                                                            ),
                                                                            DAT_00732f8c as u32,
                                                                        );
                                                                        w32(o(r32(o(o(DAT_006d4988, iVar5), 0x1f)), 10), r32(o(o(DAT_006d4988, iVar5), 8)) as u32);
                                                                        w32(
                                                                            o(
                                                                                o(
                                                                                    DAT_006d4988,
                                                                                    0xc,
                                                                                ),
                                                                                iVar5,
                                                                            ),
                                                                            DAT_00732f5c as u32,
                                                                        );
                                                                        w32(o(r32(o(o(DAT_006d4988, iVar5), 0x1f)), 0xe), r32(o(o(DAT_006d4988, iVar5), 0xc)) as u32);
                                                                        w16(
                                                                            o(
                                                                                o(
                                                                                    DAT_006d4988,
                                                                                    0x14,
                                                                                ),
                                                                                iVar5,
                                                                            ),
                                                                            DAT_006db6de as u32,
                                                                        );
                                                                        w16(
                                                                            o(
                                                                                o(
                                                                                    DAT_006d4988,
                                                                                    0x12,
                                                                                ),
                                                                                iVar5,
                                                                            ),
                                                                            DAT_0071f2d6 as u32,
                                                                        );
                                                                        w16(
                                                                            o(iVar10, 0xd),
                                                                            r16(o(
                                                                                o(
                                                                                    DAT_006d4988,
                                                                                    0x12,
                                                                                ),
                                                                                iVar5,
                                                                            ))
                                                                                as u32,
                                                                        );
                                                                        w16(
                                                                            o(
                                                                                o(
                                                                                    DAT_006d4988,
                                                                                    0x10,
                                                                                ),
                                                                                iVar5,
                                                                            ),
                                                                            DAT_00732576 as u32,
                                                                        );
                                                                        w8(o(r32(o(o(DAT_006d4988, 0x1f), iVar5)), 0x89), 1);
                                                                        iVar23 = r32(o(
                                                                            o(DAT_006d4988, 0x1f),
                                                                            iVar5,
                                                                        ));
                                                                        w8(
                                                                            o(iVar23, 0x90),
                                                                            r8(o(iVar23, 0x89))
                                                                                as u32,
                                                                        );
                                                                        uVar25 = FUN_00478470();
                                                                        iVar23 = r32(o(
                                                                            o(DAT_006d4988, 0x1b),
                                                                            iVar5,
                                                                        ));
                                                                        if (uVar25 & 0x8000) == 0 {
                                                                            FUN_0040a1a0(
                                                                                0x20, uVar8, iVar23,
                                                                            );
                                                                        } else {
                                                                            FUN_0040a1a0(
                                                                                0x1f, uVar8, iVar23,
                                                                            );
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            break 'ebc5;
                                                        }
                                                        FUN_0040ed40(pbVar14, local_32, unaff_EBX);
                                                    }
                                                    break 'ecd4;
                                                }
                                            }
                                            // LAB_0040ebc5
                                            if r8(o(o(DAT_006d4988, iVar5), 3)) == 0
                                                || r8(o(o(DAT_006d4988, iVar5), 2)) == 0x0d
                                            {
                                                w32(o(iVar9, 0x73), 0);
                                                pbVar14.write(0);
                                                break 'inner;
                                            }
                                            if r8(o(o(DAT_006d4988, iVar15), 3)) == 0
                                                || r8(o(o(DAT_006d4988, iVar15), 2)) == 0x0d
                                            {
                                                remove_entry!();
                                            }
                                        }
                                    }
                                    local_36 = local_36.wrapping_add(1);
                                    if !((local_36 as i32) < pbVar14.read() as i32) {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
                local_32 = local_32.wrapping_add(1);
                if !((local_32 as i32) < DAT_00732904) {
                    break;
                }
            }
        }
    }
}
