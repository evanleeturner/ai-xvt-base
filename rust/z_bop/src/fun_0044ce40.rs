// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a806c: u32;
    static mut DAT_009a7bac: u32;
    static mut DAT_009a8c14: u32;
    static mut DAT_005233d8: i32;
    static mut DAT_009ecc34: i32;
    static mut DAT_009e967c: u8;
    static mut DAT_009e967e: u8;
    static mut DAT_009e9670: u8;
    static mut DAT_009e968a: u8;
    static mut DAT_009e9683: u8;
    static mut DAT_009e9a77: u8;
    static mut DAT_009a1fe8: i32;
    static mut DAT_009a1fe0: i32;
    static mut _DAT_00a07cec: i32;
    static mut DAT_009a2000: u16;
    static mut DAT_009fd458: u8;
    static mut DAT_009d8c88: u8;
    static mut DAT_00a08ba0: i16;
    static mut _DAT_00a07be0: u16;
    static mut _DAT_00a07be2: u16;
    static mut DAT_009a20ae: u8;
    static mut DAT_00a083bc: u16;
    static mut DAT_00a08376: u16;
    static mut DAT_00a08378: u16;
    static mut DAT_00a083b8: u16;
    static mut DAT_00a08606: u16;
    static mut DAT_00a08608: u16;
    static mut DAT_00a08a00: u16;
    static mut DAT_00a08604: u16;
    static mut DAT_00521548: u16;
    static mut DAT_0052154c: u16;
    static mut DAT_00a0a1e0: u8;
    static mut DAT_00a08374: u16;
    static mut DAT_009ed232: u8;
    static mut DAT_005233c8: u8;
    static mut DAT_005569d8: u16;
    static mut DAT_005569ec: u8;
    static mut DAT_009d8b8c: u8;
    static mut DAT_009d1300: unsafe extern "C" fn();
    static mut DAT_009eca20: unsafe extern "C" fn(u32, u32, u32, u32, i32);
    static mut DAT_009a6fe4: unsafe extern "C" fn(i32);
    static mut DAT_00a0a160: *mut c_void;
    static mut DAT_00a0a164: *mut c_void;
    static mut DAT_00a0a168: *mut c_void;
    static mut DAT_00a0a0e4: *mut c_void;
    static mut DAT_00a0a0e8: *mut c_void;
    static mut DAT_00524078: u8;
    static mut DAT_009a1ed0: u8;
    static mut DAT_009a1ed9: u8;
    static mut DAT_00a08108: u16;
    static mut DAT_005233ec: i32;
    static mut DAT_009d6930: u16;
    static mut DAT_00a08c70: *mut u8;
    static mut DAT_00a0a130: *mut u8;
    static mut DAT_00523f80: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0040e0c0(a: i32, b: i32, c: i32, d: i32) -> u32;
    fn FUN_004a9640(a: i8);
    fn FUN_00450650(a: i32, b: i32);
    fn FUN_004a95c0(a: u32);
    fn FUN_004a9520(a: i32, b: i32, c: i32, d: i32);
    fn FUN_00480530() -> u32;
    fn FUN_004a9630(a: i32);
    fn FUN_004a9590(a: u32);
    fn FUN_004a9500(a: i32, b: i32);
    fn FUN_004a9960(a: *const c_void);
    fn FUN_004a97f0(a: *const c_void);
    fn FUN_004a9820(a: *const c_void);
    fn FUN_004a9ac0(a: *const c_void);
    fn FUN_0044e0a0(a: u32);
    fn FUN_00405820(a: u32) -> u32;
    fn FUN_00425c60(a: u32, b: u32) -> u32;
    fn FUN_004277f0(a: u16, b: i32, c: i32);
    fn FUN_004a9890(a: u32) -> u32;
    fn FUN_0043c290(a: u16, b: i32);
    fn FUN_0044e170(a: u32) -> u32;
}

unsafe fn m5bd() -> i32 {
    // SAFETY: reads the game's own global.
    unsafe { (DAT_009ecc34 as u32).wrapping_mul(0x5bd) as i32 }
}

unsafe fn m10() -> i32 {
    // SAFETY: reads the game's own global.
    unsafe { (DAT_009ecc34 as u32).wrapping_mul(0x10) as i32 }
}

unsafe fn mula() -> i32 {
    // SAFETY: reads the game's own globals.
    unsafe { (DAT_005233d8 as u32).wrapping_mul(DAT_009a7bac) as i32 }
}

unsafe fn r8(a: i32) -> u8 {
    // SAFETY: the address is the game's own memory, read as the original does.
    unsafe { (a as usize as *const u8).read() }
}

unsafe fn r16(a: i32) -> u16 {
    // SAFETY: the address is the game's own memory, read as the original does.
    unsafe { (a as usize as *const u16).read_unaligned() }
}

unsafe fn r32(a: i32) -> i32 {
    // SAFETY: the address is the game's own memory, read as the original does.
    unsafe { (a as usize as *const i32).read_unaligned() }
}

unsafe fn g9683(i: i32) -> u8 {
    // SAFETY: indexes the game's own byte table.
    unsafe { (&raw mut DAT_009e9683).wrapping_offset(i as isize).read() }
}

unsafe fn g9a77(i: i32) -> u8 {
    // SAFETY: indexes the game's own byte table.
    unsafe { (&raw mut DAT_009e9a77).wrapping_offset(i as isize).read() }
}

unsafe fn g458(i: u32) -> u8 {
    // SAFETY: indexes the game's own byte table.
    unsafe { (&raw mut DAT_009fd458).wrapping_add(i as usize).read() }
}

unsafe fn a08ba0(i: u16) -> *mut i16 {
    // SAFETY: address arithmetic into the game's own table.
    unsafe { (&raw mut DAT_00a08ba0).wrapping_add(i as usize) }
}

unsafe fn sel(i: i32, d: u32) -> u32 {
    // SAFETY: reads the game's own globals.
    unsafe {
        if g9683(i) == 0 {
            if g9a77(i) == 0 {
                d
            } else {
                DAT_005233c8 as u32
            }
        } else {
            0x34
        }
    }
}

#[unsafe(no_mangle)]
#[allow(
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_variables,
    unused_mut,
    unused_labels
)]
pub unsafe extern "C" fn FUN_0044ce40(param_1: u16) -> u32 {
    // SAFETY: the globals, callees and the memory they point to are the
    // game's own; reads and writes happen where the original does them.
    unsafe {
        let mut local_60 = [0u32; 24];
        let mut local_c0 = [0u32; 24];
        let ps60 = local_60.as_mut_ptr().cast::<u16>();
        let pc0 = local_c0.as_mut_ptr();
        let ps_c0 = pc0.cast::<u16>();

        let mut uCnt: u16;
        let mut uVar8: u16;
        let mut uVar18: u16;
        let mut uVar9: u16;
        let mut uVar15: u32;
        let mut uVar26: u32;
        let mut uVar13: u32;
        let mut uVar14: u32;
        let mut uVar16: u32;
        let mut iVar5: i32;
        let mut iVar6: i32;
        let mut iVar19: i32;
        let mut iVar20: i32;
        let mut iTmp: i32;
        let mut iTmp2: i32;
        let mut bVar3: u8;
        let mut bVar25: i32 = 0;
        let mut sVar10: i16;
        let mut sVar17: i16;
        let mut sVar21: i16 = 0;
        let mut sStack_e2: i16 = 0;
        let mut sStack_e4: i16 = 0;
        let mut local_e6: i16 = 0;
        let mut local_e8: u16 = 0;
        let mut local_d4: u16 = 0;
        let mut uStack_e0: u16 = 0;
        let mut uStack_ec: u16;
        let mut uStack_ee: u16;
        let mut uStack_cc: u32;
        let extraout_var: u16 = 0;
        let uVar24: u16 = 0;
        let mut pbVar12: *mut u8;

        FUN_0040e0c0(
            DAT_009a806c as i32,
            DAT_009a7bac as i32,
            DAT_009a8c14 as i32,
            mula(),
        );
        FUN_004a9640(0);
        uCnt = 1;
        iVar6 = m5bd();
        uVar8 = (&raw mut DAT_009e967c)
            .wrapping_offset(iVar6 as isize)
            .cast::<u16>()
            .read_unaligned();
        let sTmp: u16 = (&raw mut DAT_009e967e)
            .wrapping_offset(iVar6 as isize)
            .cast::<u16>()
            .read_unaligned();
        ps60.write(sTmp);
        ps_c0.write(sTmp);
        let mut uVar7: u16 = 5u16.wrapping_sub(param_1);
        iVar5 = (&raw mut DAT_009e9670)
            .wrapping_offset(iVar6 as isize)
            .cast::<i32>()
            .read_unaligned();

        'end: {
            if iVar5 != -1 {
                iTmp = r32((iVar5 as u32)
                    .wrapping_mul(0x23)
                    .wrapping_add(0x1f)
                    .wrapping_add(DAT_009a1fe8 as u32) as i32);
                iTmp2 = r32(iTmp.wrapping_add(0xa9));
                if r16(iTmp2.wrapping_add(0x2b4)) == 0 && a08ba0(uVar7).read() != -2 {
                    _DAT_00a07be0 = 0x60;
                    _DAT_00a07be2 = 0x57;
                    a08ba0(uVar7).write(-2);
                    FUN_00450650(0x56, DAT_009ecc34);
                    break 'end;
                }
            }
            if param_1 == 1 {
                uStack_e0 = 0;
                if _DAT_00a07cec < DAT_009a1fe0 {
                    let mut p = (_DAT_00a07cec as u32)
                        .wrapping_mul(0x23)
                        .wrapping_add(0x1f)
                        .wrapping_add(DAT_009a1fe8 as u32) as i32;
                    iVar19 = _DAT_00a07cec;
                    loop {
                        iVar5 = r32(p);
                        if (r8(p.wrapping_sub(28)) as i8) != 0 && iVar5 != 0 {
                            iTmp = r32(iVar5.wrapping_add(0xa9));
                            if iTmp != 0 {
                                bVar3 = r8(iVar5.wrapping_add(0x89));
                                if (bVar3 as u32) != (uVar8 as u32)
                                    && g458(
                                        (bVar3 as u32)
                                            .wrapping_add((uVar8 as u32).wrapping_mul(0x1e5)),
                                    ) == 0
                                {
                                    let cVar4 = r8(iTmp.wrapping_add(7)) as i8;
                                    if cVar4 != 3 && cVar4 != 4 {
                                        ps60.add(uStack_e0 as usize)
                                            .write(((bVar3 as u16) << 8) | (iVar19 as u16));
                                        uStack_e0 = uStack_e0.wrapping_add(1);
                                    }
                                }
                            }
                        }
                        p = p.wrapping_add(0x23);
                        iVar19 = iVar19.wrapping_add(1);
                        if !(iVar19 < DAT_009a1fe0) {
                            break;
                        }
                    }
                }
            } else {
                uVar15 = 0;
                if 0 < (DAT_009a2000 as i32) {
                    pbVar12 = &raw mut DAT_009d8c88;
                    loop {
                        let b = pbVar12.read();
                        if (b as u32) == (uVar8 as u32) {
                            bVar25 = 0;
                        } else {
                            bVar25 =
                                (g458((b as u32).wrapping_add((uVar8 as u32).wrapping_mul(0x1e5)))
                                    == 0) as i32;
                        }
                        if bVar25 == 0 && (ps60.read() as u32) != uVar15 {
                            ps_c0.add(uCnt as usize).write(uVar15 as u16);
                            uCnt = uCnt.wrapping_add(1);
                        }
                        pbVar12 = pbVar12.wrapping_add(0x566);
                        uVar15 = uVar15.wrapping_add(1);
                        if !((uVar15 as i32) < (DAT_009a2000 as i32)) {
                            break;
                        }
                    }
                }
                uStack_e0 = 0;
                if uCnt != 0 {
                    let mut puVar11 = ps_c0;
                    uVar15 = uCnt as u32;
                    loop {
                        if _DAT_00a07cec < DAT_009a1fe0 {
                            let mut p = (_DAT_00a07cec as u32)
                                .wrapping_mul(0x23)
                                .wrapping_add(0x1f)
                                .wrapping_add(DAT_009a1fe8 as u32)
                                as i32;
                            uVar26 = puVar11.read() as u32;
                            iVar19 = _DAT_00a07cec;
                            loop {
                                if (r8(p.wrapping_sub(9)) as u16) == (uVar26 as u16)
                                    && (r8(p.wrapping_sub(28)) as i8) != 0
                                {
                                    iVar5 = r32(p);
                                    if iVar5 != 0 {
                                        iTmp = r32(iVar5.wrapping_add(0xa9));
                                        if iTmp != 0 {
                                            let cVar4 = r8(iTmp.wrapping_add(7)) as i8;
                                            if cVar4 != 3 && cVar4 != 4 {
                                                ps60.add(uStack_e0 as usize).write(
                                                    ((r8(iVar5.wrapping_add(0x89)) as u16) << 8)
                                                        | (iVar19 as u16),
                                                );
                                                uStack_e0 = uStack_e0.wrapping_add(1);
                                            }
                                        }
                                    }
                                }
                                p = p.wrapping_add(0x23);
                                iVar19 = iVar19.wrapping_add(1);
                                if !(iVar19 < DAT_009a1fe0) {
                                    break;
                                }
                            }
                        }
                        puVar11 = puVar11.add(1);
                        uVar15 = uVar15.wrapping_sub(1);
                        if uVar15 == 0 {
                            break;
                        }
                    }
                }
            }
            uVar15 = uStack_e0 as u32;
            iVar19 = 0;
            loop {
                iVar5 = iVar19 + 1;
                iVar20 = iVar5;
                while iVar20 < (uVar15 as i32) {
                    uVar8 = ps60.offset(iVar20 as isize).read();
                    uVar18 = ps60.offset(iVar19 as isize).read();
                    if ((uVar8 & 0xff00) as u32) < ((uVar18 as u32) & 0xffffff00) {
                        ps60.offset(iVar19 as isize).write(uVar8);
                        ps60.offset(iVar20 as isize).write(uVar18);
                        iVar20 = iVar5;
                    }
                    iVar20 += 1;
                }
                iVar19 = iVar5;
                if !(iVar5 < (uVar15 as i32)) {
                    break;
                }
            }
            if uStack_e0 == 0 {
                let psVar1 = a08ba0(uVar7);
                if psVar1.read() != -2 {
                    if psVar1.read() == 0 {
                        psVar1.write(0);
                    } else {
                        psVar1.write(-2);
                    }
                    break 'end;
                }
            }
            uVar8 = (DAT_009a20ae as u16).wrapping_add(2);
            if param_1 == 1 && g9683(iVar6) != 0 {
                sStack_e2 = (DAT_00a083bc as i32 + 2) as i16;
                sVar21 = (DAT_00a08376 as i32 + 2) as i16;
                local_e6 = (DAT_00a08378 as i32 - 2 + DAT_00a083bc as i32) as i16;
                sStack_e4 = ((DAT_00a083b8 as i32 - 2) + DAT_00a08376 as i32) as i16;
            } else {
                sStack_e2 = (DAT_00a08606 as i32 + 2) as i16;
                sVar21 = (DAT_00a08608 as i32 + 2) as i16;
                local_e6 = (DAT_00a08a00 as i32 - 2 + DAT_00a08606 as i32) as i16;
                'l311: {
                    if a08ba0(uVar7).read() != -2 && g9683(iVar6) == 0 {
                        sStack_e4 = ((uStack_e0 as i32 + 1)
                            .wrapping_mul(uVar8 as i32)
                            .wrapping_add(DAT_00a08608 as i32))
                            as i16;
                        if (sStack_e4 as i32)
                            <= ((DAT_00a08604 as u32)
                                .wrapping_add(DAT_00a08608 as u32)
                                .wrapping_sub(2) as i32)
                            && (DAT_00a08608 as i32) < (sStack_e4 as i32)
                        {
                            break 'l311;
                        }
                    }
                    sStack_e4 = ((DAT_00a08604 as i32 - 2) + DAT_00a08608 as i32) as i16;
                }
            }
            if g9683(iVar6) == 0 && DAT_00521548 == 0xffff {
                DAT_00521548 = uVar7;
            }
            sVar17 = a08ba0_dummy(uVar7);
            let off = (DAT_00a08374 as u32)
                .wrapping_add(param_1.wrapping_add(0x82) as u32)
                .wrapping_mul(2);
            if (&raw mut DAT_00a0a1e0)
                .wrapping_add(off as usize)
                .cast::<i16>()
                .read_unaligned()
                == sVar17
            {
                sVar17 = (uVar8 as i32 + sVar21 as i32) as i16;
                local_d4 = DAT_009ed232 as u16;
                FUN_004a9630(1);
                FUN_004a9590(0x46);
                FUN_004a9640(0);
                FUN_004a9520(
                    sStack_e2 as i32,
                    sVar21 as i32,
                    local_e6 as i32,
                    sStack_e4 as i32,
                );
                uVar26 = sel(m5bd(), 0x2c);
                FUN_004a95c0(uVar26);
                if uStack_e0 == DAT_005569d8 {
                    bVar25 = 0;
                } else {
                    if g9a77(m5bd()) == 0 {
                        FUN_004a95c0(DAT_005233c8 as u32);
                    }
                    if param_1 == 1 && g9683(m5bd()) != 0 {
                        sVar10 = ((DAT_00a083b8 as i32 - 2) + DAT_00a08376 as i32) as i16;
                    } else {
                        sVar10 = ((DAT_00a08604 as i32 - 2) + DAT_00a08608 as i32) as i16;
                    }
                    FUN_004a9520(
                        sStack_e2 as i32 - 2,
                        sVar17 as i32,
                        local_e6 as i32 + 2,
                        sVar10 as i32 + 2,
                    );
                    DAT_009d1300();
                    if g9a77(m5bd()) == 0 {
                        FUN_004a95c0(0x2c);
                        sVar10 =
                            ((uStack_e0 as i32).wrapping_mul(uVar8 as i32) + sVar17 as i32) as i16;
                        if ((DAT_00a08604 as u32)
                            .wrapping_add(DAT_00a08608 as u32)
                            .wrapping_sub(2) as i32)
                            < (sVar10 as i32)
                        {
                            sVar10 = sStack_e4;
                        }
                        FUN_004a9520(
                            sStack_e2 as i32 - 2,
                            sVar17 as i32,
                            local_e6 as i32 + 2,
                            sVar10 as i32 + 2,
                        );
                        DAT_009d1300();
                    }
                    bVar25 = 1;
                    DAT_005569d8 = uStack_e0;
                }
            } else {
                if g9683(iVar6) == 0 {
                    if sVar17 == -2 || g9a77(iVar6) != 0 {
                        uVar26 = DAT_005233c8 as u32;
                    } else {
                        uVar26 = 0x2c;
                    }
                } else {
                    uVar26 = 0x34;
                }
                FUN_004a95c0(uVar26);
                sVar17 = sStack_e4;
                if g9683(m5bd()) != 0 {
                    uVar18 = DAT_00a08608;
                    uVar9 = DAT_00a08604;
                    if param_1 == 1 {
                        uVar18 = DAT_00a08376;
                        uVar9 = DAT_00a083b8;
                    }
                    sVar17 = ((uVar9 as i32 - 2) + uVar18 as i32) as i16;
                }
                FUN_004a9520(
                    sStack_e2 as i32 - 2,
                    sVar21 as i32 - 2,
                    local_e6 as i32 + 2,
                    sVar17 as i32 + 2,
                );
                DAT_009d1300();
                FUN_004a9520(
                    sStack_e2 as i32 - 2,
                    sVar21 as i32 - 2,
                    local_e6 as i32 + 2,
                    sStack_e4 as i32 + 2,
                );
                if a08ba0(uVar7).read() == -2 {
                    if DAT_00521548 == uVar7 {
                        DAT_00521548 = DAT_0052154c;
                        uVar15 = FUN_00480530();
                        DAT_0052154c = uVar15 as u16;
                    }
                    FUN_0040e0c0(0, 0x140, 200, 0);
                    return ((extraout_var as u32) << 16) | (ps_c0.read() as u32);
                }
                FUN_004a9630(1);
                FUN_004a9590(0x46);
                FUN_004a9640(0);
                FUN_004a9520(
                    sStack_e2 as i32,
                    sVar21 as i32,
                    local_e6 as i32,
                    sStack_e4 as i32,
                );
                uVar26 = sel(m5bd(), 0x2c);
                FUN_004a95c0(uVar26);
                bVar3 = DAT_009ed232;
                uVar18 = bVar3 as u16;
                FUN_004a9500(sStack_e2 as i32, sVar21 as i32);
                FUN_004a9960(DAT_00a0a160 as *const c_void);
                local_d4 = bVar3 as u16;
                FUN_004a97f0(DAT_00a0a0e4 as *const c_void);
                FUN_004a9820((&raw const DAT_00524078).cast::<c_void>());
                FUN_004a9820(DAT_00a0a0e8 as *const c_void);
                FUN_004a9500(sStack_e2 as i32 + local_d4 as i32 * 0xb, sVar21 as i32);
                FUN_004a9960((&raw const DAT_009a1ed0).cast::<c_void>());
                sVar17 = (DAT_00a08108 as i32 + local_d4 as i32) as i16;
                if param_1 == 1 {
                    FUN_004a9500(sVar17 as i32, sVar21 as i32);
                    FUN_004a9960(DAT_00a0a164 as *const c_void);
                    if DAT_005233ec != 0x13 {
                        FUN_004a9500(sVar17 as i32 + local_d4 as i32 * 0x11, sVar21 as i32);
                        FUN_004a9960(DAT_00a0a168 as *const c_void);
                    }
                } else if g9683(m5bd()) == 0 || DAT_005233ec != 0x13 {
                    FUN_004a9500(sVar17 as i32, sVar21 as i32);
                    FUN_004a9ac0(DAT_00a0a164 as *const c_void);
                }
                bVar25 = 1;
                (&raw mut DAT_005569ec)
                    .wrapping_add((param_1 as usize) * 2)
                    .cast::<u16>()
                    .write_unaligned(0);
                (&raw mut DAT_009d8b8c)
                    .wrapping_offset(m10() as isize)
                    .cast::<u16>()
                    .write_unaligned(0x1d8);
                FUN_004a9520(
                    sStack_e2 as i32,
                    uVar8 as i32 + sVar21 as i32,
                    local_e6 as i32,
                    sStack_e4 as i32,
                );
                DAT_005569d8 = uStack_e0;
                local_d4 = uVar18;
            }
            local_e8 = (uVar8 as i32 + sVar21 as i32) as u16;
            let mut do_call = false;
            let mut ca: u32 = 0;
            let mut cb: u32 = 0;
            let mut cc: u32 = 0;
            let mut cd: u32 = 0;
            if DAT_00521548 == uVar7 {
                FUN_004a95c0(0x46);
                uVar14 = ((uVar24 as u32) << 16) | ((sVar21 as i32 - 2) as u16 as u32);
                ps_c0.write((sStack_e4 as i32 + 2) as u16);
                FUN_004a9520(
                    sStack_e2 as i32 - 2,
                    sVar21 as i32 - 2,
                    local_e6 as i32 + 2,
                    sStack_e4 as i32 + 2,
                );
                uVar26 = (sStack_e2 as i32 - 2) as u16 as u32;
                uVar13 = (local_e6 as i32 + 2) as u16 as u32;
                uVar16 = pc0.read();
                do_call = true;
                ca = uVar26;
                cb = uVar14;
                cc = uVar13;
                cd = uVar16;
            } else {
                if DAT_0052154c == uVar7 {
                    uVar26 = sel(m5bd(), 0x4e);
                    FUN_004a95c0(uVar26);
                    uVar16 = (sStack_e4 as i32 + 2) as u16 as u32;
                    uVar14 = ((uVar24 as u32) << 16) | ((sVar21 as i32 - 2) as u16 as u32);
                    ps_c0.write((local_e6 as i32 + 2) as u16);
                    FUN_004a9520(
                        sStack_e2 as i32 - 2,
                        sVar21 as i32 - 2,
                        local_e6 as i32 + 2,
                        sStack_e4 as i32 + 2,
                    );
                    uVar26 = (sStack_e2 as i32 - 2) as u16 as u32;
                    uVar13 = pc0.read();
                    do_call = true;
                    ca = uVar26;
                    cb = uVar14;
                    cc = uVar13;
                    cd = uVar16;
                } else if g9a77(m5bd()) == 0 {
                    FUN_004a95c0(0x4e);
                    uVar16 = (sStack_e4 as i32 + 2) as u16 as u32;
                    uVar13 = (local_e6 as i32 + 2) as u16 as u32;
                    uVar14 = ((uVar24 as u32) << 16) | ((sVar21 as i32 - 2) as u16 as u32);
                    ps_c0.write((sStack_e2 as i32 - 2) as u16);
                    FUN_004a9520(
                        sStack_e2 as i32 - 2,
                        sVar21 as i32 - 2,
                        local_e6 as i32 + 2,
                        sStack_e4 as i32 + 2,
                    );
                    uVar26 = pc0.read();
                    do_call = true;
                    ca = uVar26;
                    cb = uVar14;
                    cc = uVar13;
                    cd = uVar16;
                }
            }
            if do_call {
                DAT_009eca20(ca, cb, cc, cd, 1);
            }
            uVar26 = sel(m5bd(), 0x2c);
            FUN_004a95c0(uVar26);
            let puVar2 = (&raw mut DAT_005569ec)
                .wrapping_add((param_1 as usize) * 2)
                .cast::<u16>();
            if DAT_00521548 == uVar7 {
                if DAT_009d6930 == 0xa6 {
                    if puVar2.read_unaligned() != 0 {
                        puVar2.write_unaligned(puVar2.read_unaligned().wrapping_sub(1));
                        bVar25 = 1;
                    }
                } else if DAT_009d6930 == 0xa7 && 1 < uStack_e0 {
                    let q = ((local_e8 as u32).wrapping_sub(sStack_e4 as i32 as u32) as i32)
                        / (uVar8 as i32);
                    if (puVar2.read_unaligned() as i32) <= ((q as u32).wrapping_add(uVar15) as i32)
                    {
                        puVar2.write_unaligned(puVar2.read_unaligned().wrapping_add(1));
                        bVar25 = 1;
                    }
                }
            }
            let p8b = (&raw mut DAT_009d8b8c)
                .wrapping_offset(m10() as isize)
                .cast::<i16>();
            if p8b.read_unaligned() < 1 {
                bVar25 = 1;
                p8b.write_unaligned(0x1d8);
            }
            uVar7 = puVar2.read_unaligned();
            sVar21 = ((((sStack_e4 as i32) as u32).wrapping_sub(local_e8 as u32) as i32)
                / (uVar8 as i32)) as i16;
            uStack_ec = (uVar7 as i32 + sVar21 as i32) as u16;
            if uStack_e0 < uStack_ec {
                if uVar7 != 0 {
                    puVar2.write_unaligned(uVar7.wrapping_sub(1));
                }
                uStack_ec = (puVar2.read_unaligned() as i32 + sVar21 as i32) as u16;
            }
            uStack_ee = 0xffff;
            if bVar25 != 0 {
                FUN_004a9520(
                    sStack_e2 as i32,
                    local_e8 as i32,
                    local_e6 as i32,
                    sStack_e4 as i32,
                );
                DAT_009d1300();
                uStack_cc = 0;
                if uStack_e0 != 0 {
                    let mut local_c8 = ps60;
                    loop {
                        FUN_004a9590(0x43);
                        if (puVar2.read_unaligned() as i32) <= (uStack_cc as i32)
                            && (uStack_cc as i32) <= (uStack_ec as i32)
                        {
                            uVar7 = local_c8.read();
                            uVar18 = uVar7 >> 8;
                            pbVar12 = (&raw mut DAT_00523f80)
                                .cast::<u8>()
                                .wrapping_add(uVar18 as usize);
                            FUN_004a9590(pbVar12.read() as u32);
                            if uStack_ee != uVar18 && (puVar2.read_unaligned() as u32) == uStack_cc
                            {
                                if g9683(m5bd()) != 0 {
                                    FUN_0040e0c0(0, 0x140, 200, 0);
                                    FUN_004a95c0(0x2c);
                                    FUN_004a95c0(0x34);
                                    FUN_0040e0c0(
                                        DAT_009a806c as i32,
                                        DAT_009a7bac as i32,
                                        DAT_009a8c14 as i32,
                                        mula(),
                                    );
                                }
                                FUN_004a9520(
                                    sStack_e2 as i32,
                                    local_e8 as i32,
                                    local_e6 as i32,
                                    sStack_e4 as i32,
                                );
                                uStack_ee = uVar18;
                            }
                            let key = (((local_e8 as u32) << 16) | (uVar7 as u32)) & 0xffff00ff;
                            FUN_0044e0a0(key);
                            FUN_004a9500(sStack_e2 as i32, local_e8 as i32);
                            DAT_009a1ed9 = 0;
                            FUN_004a9960((&raw const DAT_009a1ed0).cast::<c_void>());
                            sVar21 = (sStack_e2 as i32 + local_d4 as i32 * 0xb) as i16;
                            pc0.write(((uVar7 & 0xff) as u32).wrapping_mul(0x23));
                            iVar6 = r32((DAT_009a1fe8 as u32)
                                .wrapping_add(0x1f)
                                .wrapping_add(pc0.read())
                                as i32);
                            if iVar6 != 0 {
                                iVar6 = r32(iVar6.wrapping_add(0xa9));
                                let mut st: i32;
                                if iVar6 == 0 {
                                    st = 1;
                                } else if (r8(iVar6.wrapping_add(7)) as i8) != 3 {
                                    if (r8(iVar6.wrapping_add(7)) as i8) != 4 {
                                        iTmp = r32(iVar6.wrapping_add(0x215));
                                        iTmp2 = r32(iVar6.wrapping_add(0x211));
                                        uVar26 = (iTmp.wrapping_add(iTmp2) / 2) as u32;
                                        uVar13 = FUN_00405820(key);
                                        if uVar26 == 0 || uVar13 == 0 {
                                            uVar26 = 0;
                                        } else {
                                            uVar26 = FUN_00425c60(uVar26, uVar13);
                                            uVar26 = ((uVar26 & 0xffff) / 0x28f).wrapping_mul(2);
                                        }
                                        if uVar26 < 0x15 {
                                            uVar13 = 0x4a;
                                        } else if uVar26 < 0x33 {
                                            uVar13 = 0x4e;
                                        } else {
                                            uVar13 = 0x46;
                                        }
                                        FUN_004a9590(uVar13);
                                        FUN_004a9500(
                                            sVar21 as i32 + local_d4 as i32,
                                            local_e8 as i32,
                                        );
                                        FUN_004277f0(uVar26 as u16, 3, 1);
                                        FUN_004a9590(pbVar12.read() as u32);
                                        DAT_009a6fe4(0x2f);
                                    }
                                    st = 1;
                                } else {
                                    st = 2;
                                }
                                loop {
                                    match st {
                                        1 => {
                                            let t = r8(iVar6.wrapping_add(7)) as i8;
                                            if t == 3 || t == 4 {
                                                st = 2;
                                                continue;
                                            }
                                            uVar26 = r32(iVar6.wrapping_add(0xcb)) as u32;
                                            let uTmp = r32(iVar6.wrapping_add(0xc3)) as u32;
                                            if uVar26 < uTmp {
                                                uStack_e0 = 1;
                                                st = 3;
                                                continue;
                                            }
                                            uVar26 =
                                                FUN_00425c60(uVar26.wrapping_sub(uTmp), uVar26);
                                            uVar26 = (uVar26 & 0xffff) / 0x28f;
                                            if uVar26 == 0 {
                                                uStack_e0 = 100;
                                                st = 3;
                                                continue;
                                            }
                                            st = 4;
                                        }
                                        2 => {
                                            uStack_e0 = 0;
                                            st = 3;
                                        }
                                        3 => {
                                            uVar26 = uStack_e0 as u32;
                                            if uVar26 != 0 {
                                                st = 4;
                                            } else {
                                                break;
                                            }
                                        }
                                        _ => {
                                            uVar14 = FUN_004a9890(uVar26);
                                            FUN_004a9500(
                                                (DAT_00a08108 as i32).wrapping_add(
                                                    (3i32.wrapping_sub(uVar14 as i16 as i32))
                                                        .wrapping_mul(local_d4 as i32),
                                                ),
                                                local_e8 as i32,
                                            );
                                            if uVar26 < 0x15 {
                                                uVar26 = 0x4a;
                                            } else if uVar26 < 0x33 {
                                                uVar26 = 0x4e;
                                            } else {
                                                uVar26 = 0x46;
                                            }
                                            FUN_004a9590(uVar26);
                                            FUN_004a9960(
                                                (&raw const DAT_009a1ed0).cast::<c_void>(),
                                            );
                                            break;
                                        }
                                    }
                                }
                                FUN_004a9590(pbVar12.read() as u32);
                                sVar21 = (local_d4 as i32 * 2 + DAT_00a08108 as i32) as i16;
                                if g9683(m5bd()) == 0 || param_1 != 0 || DAT_005233ec != 0x13 {
                                    FUN_004a9500(sVar21 as i32, local_e8 as i32);
                                    iVar19 = r32((DAT_009a1fe8 as u32)
                                        .wrapping_add(0x1b)
                                        .wrapping_add(pc0.read())
                                        as i32);
                                    let use83: bool;
                                    if iVar19 == -1 {
                                        uVar18 = r16(iVar6.wrapping_add(0x42));
                                        use83 = !(uVar18 == 0xff
                                            || uVar18 == 0xffff
                                            || 0x7fff < uVar18);
                                    } else {
                                        uVar18 = (&raw mut DAT_009e968a)
                                            .wrapping_offset(
                                                ((iVar19 as u32).wrapping_mul(0x5bd) as i32)
                                                    as isize,
                                            )
                                            .cast::<u16>()
                                            .read_unaligned();
                                        let mut u = false;
                                        if uVar18 != 0xffff && (uVar18 as i32) < DAT_009a1fe0 {
                                            iTmp = r32((uVar18 as u32)
                                                .wrapping_mul(0x23)
                                                .wrapping_add(0x1f)
                                                .wrapping_add(DAT_009a1fe8 as u32)
                                                as i32);
                                            if iTmp != 0 {
                                                u = true;
                                            }
                                        }
                                        use83 = u;
                                    }
                                    if use83 {
                                        FUN_0043c290(uVar18, 7);
                                    } else {
                                        FUN_004a97f0(DAT_00a08c70 as *const c_void);
                                    }
                                    if param_1 == 0 && DAT_005233ec != 0x13 && g9683(m5bd()) == 0 {
                                        FUN_004a9ac0((&raw const DAT_009a1ed0).cast::<c_void>());
                                    } else {
                                        FUN_004a9960((&raw const DAT_009a1ed0).cast::<c_void>());
                                    }
                                }
                            }
                            if param_1 != 0 && DAT_005233ec != 0x13 {
                                FUN_004a9500(
                                    sVar21 as i32 + local_d4 as i32 * 0xf,
                                    local_e8 as i32,
                                );
                                uVar26 = FUN_0044e170(key);
                                pbVar12 = DAT_00a08c70;
                                if (uVar26 & 0xffff) != 0 {
                                    pbVar12 = (&raw mut DAT_00a0a130)
                                        .wrapping_add((uVar26 & 0xffff) as usize)
                                        .read();
                                }
                                FUN_004a9960(pbVar12 as *const c_void);
                            }
                            local_e8 = local_e8.wrapping_add(uVar8);
                        }
                        local_c8 = local_c8.add(1);
                        uStack_cc = uStack_cc.wrapping_add(1);
                        if !((uStack_cc as i32) < (uVar15 as i32)) {
                            break;
                        }
                    }
                }
            }
        }
        FUN_0040e0c0(0, 0x140, 200, 0)
    }
}

unsafe fn a08ba0_dummy(i: u16) -> i16 {
    // SAFETY: reads the game's own table.
    unsafe { a08ba0(i).read() }
}
