// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: *mut u8;
    static mut DAT_009a8e2c: u32;
    static mut DAT_009a8e30: u32;
    static mut DAT_009a8e34: u32;
    static mut DAT_009a6fe0: i16;
    static mut DAT_00a0813c: i16;
    static mut DAT_009ecc34: i32;
    static mut DAT_009eca28: i32;
    static mut DAT_009e968a: u8;
    static mut DAT_009e9a5b: u8;
    static mut DAT_009e9a5f: u8;
    static mut DAT_009e9a63: u8;
    static mut DAT_009e9a67: u8;
    static mut DAT_009d8c87: u8;
    static mut DAT_00a07cdc: i32;
    static mut DAT_00a07cd8: i32;
    static mut DAT_00a07cd4: i32;
    static mut DAT_009fd39c: i32;
    static mut DAT_009fd3a0: i32;
    static mut DAT_00a08100: u8;
    static mut DAT_00a081ec: i32;
    static mut DAT_009a7bac: u32;
    static mut DAT_009a1ed0: u8;
    static mut DAT_009a20ae: u8;
    static mut DAT_009a8c00: u32;
    static mut DAT_00521540: u8;
    static mut DAT_009cc454: *const core::ffi::c_void;
    static mut DAT_009a6fe4: *const core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00427570(a: u32, b: u32) -> i32;
    fn FUN_00427590(a: u32, b: u32) -> i32;
    fn FUN_00458e20(a: u32, b: u32);
    fn FUN_00427420(a: i32, b: i32, c: i32) -> u32;
    fn FUN_00427490(a: i32, b: i32, c: i32) -> u32;
    fn FUN_00427500(a: i32, b: i32, c: i32) -> u32;
    fn FUN_00427390(a: u32, b: u32, c: u32);
    fn FUN_004a95c0(a: u32);
    fn FUN_004a9640(a: i8);
    fn FUN_00427bd0(a: i16, b: i16, c: *mut u8);
    fn FUN_00427d30(a: i16, b: i32, c: *mut u8);
    fn FUN_00483030(a: u32) -> i32;
    fn FUN_0043c290(a: u16, b: i32);
    fn FUN_004278c0(a: i32, b: i32, c: *mut u8) -> u64;
    fn FUN_004a9500(a: i16, b: i16);
    fn FUN_004a9960(a: *mut u8);
    fn FUN_004034e0(a: u32, b: u32);
    fn FUN_004277f0(a: u16, b: i32, c: i32);
}

#[unsafe(no_mangle)]
#[allow(
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_variables,
    unused_mut
)]
pub unsafe extern "C" fn FUN_00436d00(param_1: u32) {
    // SAFETY: all globals, callees and entity pointers belong to the game;
    // reads and writes happen at the offsets and in the order of the original.
    unsafe {
        let mut uVar21: u32 = DAT_009a8e34;
        let mut uVar11: u32 = DAT_009a8e30;
        let mut uVar23: u32 = DAT_009a8e2c;
        let iVar9: i32 = param_1.wrapping_mul(0x23) as i32;
        let extraout_ECX: u32 = 0;
        let extraout_EDX: u32 = 0;

        macro_rules! ent {
            ($off:expr) => {
                DAT_009a1fe8.byte_offset((iVar9 as isize) + ($off as isize))
            };
        }
        macro_rules! rec {
            ($sym:ident) => {
                (&raw mut $sym).byte_offset(DAT_009ecc34.wrapping_mul(0x5bd) as isize)
            };
        }

        let cVar2 = ent!(2).read() as i8;
        if cVar2 != 13 && cVar2 != 11 && (DAT_009a8e34 as i32) > 0 {
            let mut iVar13 = FUN_00427570(DAT_009a8e2c, DAT_009a8e34);
            let mut iVar14 = FUN_00427590(uVar11, uVar21);
            iVar13 = iVar13.wrapping_add(DAT_009a6fe0 as i32);
            iVar14 = iVar14.wrapping_add(DAT_00a0813c as i32);
            let us: u16 = rec!(DAT_009e968a).cast::<u16>().read_unaligned();
            if us as u32 == param_1 {
                let mut uVar22: u32 = 0xffff;
                if (param_1 as i32) < DAT_009eca28 {
                    let p_ent: *mut u8 = ent!(0x1f).cast::<*mut u8>().read_unaligned();
                    let p_sub: *mut u8 = p_ent.byte_add(0xa9).cast::<*mut u8>().read_unaligned();
                    uVar22 = p_sub.byte_add(0x42).cast::<u16>().read_unaligned() as u32;
                } else {
                    let p_ent: *mut u8 = ent!(0x1f).cast::<*mut u8>().read_unaligned();
                    if !p_ent.is_null() {
                        let p_sub: *mut u8 =
                            p_ent.byte_add(0xa9).cast::<*mut u8>().read_unaligned();
                        if !p_sub.is_null() {
                            uVar22 = p_sub.byte_add(4).cast::<u16>().read_unaligned() as u32;
                        }
                    }
                }
                if uVar22 != 0xffff {
                    FUN_00458e20(uVar22, ent!(0x16).read() as u32);
                    let iv = rec!(DAT_009e9a5b).cast::<i32>().read_unaligned();
                    DAT_00a07cdc = DAT_00a07cdc.wrapping_sub(iv);
                    let iv = rec!(DAT_009e9a5f).cast::<i32>().read_unaligned();
                    DAT_00a07cd8 = DAT_00a07cd8.wrapping_sub(iv);
                    let iv = rec!(DAT_009e9a63).cast::<i32>().read_unaligned();
                    DAT_00a07cd4 = DAT_00a07cd4.wrapping_sub(iv);
                    DAT_009a8e2c = FUN_00427420(DAT_00a07cdc, DAT_00a07cd8, DAT_00a07cd4);
                    DAT_009a8e30 = FUN_00427490(DAT_00a07cdc, DAT_00a07cd8, DAT_00a07cd4);
                    DAT_009a8e34 = FUN_00427500(DAT_00a07cdc, DAT_00a07cd8, DAT_00a07cd4);
                    if (DAT_009a8e34 as i32) < 1 {
                        FUN_00427390(uVar23, uVar11, uVar21);
                    }
                    let a17 = iVar13;
                    let a18 = iVar14;
                    let mut iVar15 = FUN_00427590(DAT_009a8e30, DAT_009a8e34);
                    iVar15 = iVar15.wrapping_add(DAT_00a0813c as i32);
                    let iVar16 = FUN_00427570(DAT_009a8e2c, DAT_009a8e34);
                    let f: unsafe extern "C" fn(i32, i32, i32, i32, u32) =
                        core::mem::transmute(DAT_009cc454);
                    f(
                        iVar16.wrapping_add(DAT_009a6fe0 as i32),
                        iVar15,
                        a17,
                        a18,
                        0x36,
                    );
                }
            }
            let iVar17: i32 = ent!(0x1f).cast::<i32>().read_unaligned();
            let uVar3: u8 = if iVar17 == 0 {
                (&raw const DAT_009d8c87)
                    .byte_add((ent!(0x16).read() as u32).wrapping_mul(0x566) as usize)
                    .read()
            } else {
                (iVar17 as usize as *const u8).byte_add(0x88).read()
            };
            let uVar22: u32 = match uVar3 {
                0 => 0x3f,
                1 | 4 => 0x37,
                2 => 0x33,
                _ => 0x3b,
            };
            FUN_004a95c0(uVar22);
            FUN_004a9640(0);
            let mut bVar10 = false;
            if (param_1 as i32) < DAT_009eca28 {
                bVar10 = true;
            } else {
                let p_ent: *mut u8 = ent!(0x1f).cast::<*mut u8>().read_unaligned();
                if p_ent.is_null() {
                    bVar10 = true;
                } else {
                    let iv = p_ent.byte_add(0xa9).cast::<i32>().read_unaligned();
                    if iv != 0 {
                        bVar10 = true;
                    }
                }
            }
            if bVar10 {
                let iv = rec!(DAT_009e9a63).cast::<i32>().read_unaligned();
                DAT_009a8e2c = FUN_00427420(
                    DAT_009fd39c,
                    DAT_009fd3a0,
                    0xffff0000u32.wrapping_sub(iv as u32) as i32,
                );
                let iv = rec!(DAT_009e9a63).cast::<i32>().read_unaligned();
                DAT_009a8e30 = FUN_00427490(
                    DAT_009fd39c,
                    DAT_009fd3a0,
                    0xffff0000u32.wrapping_sub(iv as u32) as i32,
                );
                let iv = rec!(DAT_009e9a63).cast::<i32>().read_unaligned();
                DAT_009a8e34 = FUN_00427500(
                    DAT_009fd39c,
                    DAT_009fd3a0,
                    0xffff0000u32.wrapping_sub(iv as u32) as i32,
                );
                if (DAT_009a8e34 as i32) < 1 {
                    FUN_00427390(uVar23, uVar11, uVar21);
                }
                let iVar17 = FUN_00427570(DAT_009a8e2c, DAT_009a8e34);
                let iVar18 = FUN_00427590(DAT_009a8e30, DAT_009a8e34);
                let iVar17 = iVar17.wrapping_add(DAT_009a6fe0 as i32);
                let iVar18 = iVar18.wrapping_add(DAT_00a0813c as i32);
                let f: unsafe extern "C" fn(i32, i32, i32, i32, u32) =
                    core::mem::transmute(DAT_009cc454);
                f(
                    iVar17,
                    iVar18,
                    iVar13,
                    iVar14,
                    (iVar17 as u32 & 0xffffff00) | DAT_00a08100 as u32,
                );
                let pc5: *mut u8 = DAT_009a1fe8.byte_offset(iVar9 as isize);
                let p_ent: *mut u8 = pc5.byte_add(0x1f).cast::<*mut u8>().read_unaligned();
                if !p_ent.is_null() && p_ent.read() == 0 {
                    if p_ent.byte_add(0x92).read() != 0 {
                        let s16 = pc5.byte_add(0x12).cast::<i16>().read_unaligned();
                        let s16b = pc5.byte_add(0x10).cast::<i16>().read_unaligned();
                        FUN_00427bd0(s16, s16b, pc5);
                        let s16 = ent!(0x14).cast::<i16>().read_unaligned();
                        FUN_00427d30(s16, 0, ent!(0));
                    }
                    let uVar22b = DAT_009a8e34;
                    uVar11 = DAT_009a8e30;
                    uVar23 = DAT_009a8e2c;
                    let p_ent: *mut u8 = ent!(0x1f).cast::<*mut u8>().read_unaligned();
                    let s16 = p_ent.byte_add(0x8c).cast::<i16>().read_unaligned();
                    let lVar6: i64 = (s16 as i32 as i64).wrapping_mul(0x100);
                    let s16b = p_ent.byte_add(0x8e).cast::<i16>().read_unaligned();
                    let lVar7: i64 = (s16b as i32 as i64).wrapping_mul(0x100);
                    let s16 = p_ent.byte_add(0x8c).cast::<i16>().read_unaligned();
                    let mut uVar19: u32 = s16 as i32 as u32;
                    let us = p_ent.byte_add(0x79).cast::<u16>().read_unaligned();
                    let uVar20: u32;
                    if us < 0x400 {
                        let lVar8: i64 = ((us as u32) << 5) as i32 as i64 * (uVar19 as i32 as i64);
                        uVar19 = (lVar8 as u32) >> 15 | (((lVar8 as u64) >> 32) as u32) << 17;
                        let us = p_ent.byte_add(0x79).cast::<u16>().read_unaligned();
                        let s16b = p_ent.byte_add(0x8e).cast::<i16>().read_unaligned();
                        let lVar8: i64 = (((us as u32) << 5) as i32 as i64) * (s16b as i32 as i64);
                        uVar20 = (lVar8 as u32) >> 15 | (((lVar8 as u64) >> 32) as u32) << 17;
                    } else {
                        let s16b = p_ent.byte_add(0x8e).cast::<i16>().read_unaligned();
                        uVar20 = s16b as i32 as u32;
                    }
                    let t6 = (lVar6 as u32) >> 15 | (((lVar6 as u64) >> 32) as u32) << 17;
                    let t7 = (lVar7 as u32) >> 15 | (((lVar7 as u64) >> 32) as u32) << 17;
                    DAT_009fd39c =
                        (DAT_009fd39c as u32).wrapping_add(t6).wrapping_add(uVar19) as i32;
                    DAT_009fd3a0 =
                        (DAT_009fd3a0 as u32).wrapping_add(t7).wrapping_add(uVar20) as i32;
                    let iv = rec!(DAT_009e9a63).cast::<i32>().read_unaligned();
                    DAT_009a8e2c = FUN_00427420(
                        DAT_009fd39c,
                        DAT_009fd3a0,
                        0xffff0000u32.wrapping_sub(iv as u32) as i32,
                    );
                    let iv = rec!(DAT_009e9a63).cast::<i32>().read_unaligned();
                    DAT_009a8e30 = FUN_00427490(
                        DAT_009fd39c,
                        DAT_009fd3a0,
                        0xffff0000u32.wrapping_sub(iv as u32) as i32,
                    );
                    let iv = rec!(DAT_009e9a63).cast::<i32>().read_unaligned();
                    DAT_009a8e34 = FUN_00427500(
                        DAT_009fd39c,
                        DAT_009fd3a0,
                        0xffff0000u32.wrapping_sub(iv as u32) as i32,
                    );
                    if (DAT_009a8e34 as i32) < 1 {
                        FUN_00427390(uVar23, uVar11, uVar22b);
                    }
                    let col = DAT_00a08100 as u32;
                    let mut iVar15 = FUN_00427590(DAT_009a8e30, DAT_009a8e34);
                    iVar15 = iVar15.wrapping_add(DAT_00a0813c as i32);
                    let iVar16 = FUN_00427570(DAT_009a8e2c, DAT_009a8e34);
                    let f: unsafe extern "C" fn(i32, i32, i32, i32, u32) =
                        core::mem::transmute(DAT_009cc454);
                    f(
                        iVar16.wrapping_add(DAT_009a6fe0 as i32),
                        iVar15,
                        iVar17,
                        iVar18,
                        col,
                    );
                }
                FUN_004a95c0(0x40);
                let local_8: i16 = iVar13 as i16;
                let local_18: i16 = iVar14 as i16;
                let r = FUN_00483030(param_1);
                uVar21 = (DAT_00a081ec as u32).wrapping_mul(r as u32) / uVar21;
                if uVar21 < DAT_009a7bac / 0x50 {
                    uVar21 = DAT_009a7bac / 0x50;
                }
                if (DAT_009a7bac >> 1) < uVar21 {
                    uVar21 = DAT_009a7bac >> 1;
                }
                let iVar13 = uVar21.wrapping_add(4) as i32;
                let iv = ent!(0x1f).cast::<i32>().read_unaligned();
                if iv == 0 && (ent!(2).read() as i8) == 8 {
                    DAT_009a1ed0 = 0;
                } else {
                    FUN_0043c290(param_1 as u16, 2);
                }
                if DAT_009a1ed0 != 0 {
                    let iVar14 = (local_18 as i32 as u32)
                        .wrapping_sub(DAT_009a20ae as u32)
                        .wrapping_sub((iVar13 / 2) as u32)
                        .wrapping_add(0xffffffff) as i32;
                    let uVar24 = FUN_004278c0(iVar14, iVar13 >> 31, &raw mut DAT_009a1ed0);
                    FUN_004a9500(
                        (local_8 as i32).wrapping_sub(((uVar24 as u16) >> 1) as i32) as i16,
                        iVar14 as i16,
                    );
                    FUN_004a9960(&raw mut DAT_009a1ed0);
                }
                let cVar2 = ent!(2).read() as i8;
                if cVar2 != 6 && cVar2 != 7 {
                    FUN_004a95c0(0x40);
                    let uVar4 = rec!(DAT_009e9a67).cast::<u16>().read_unaligned();
                    if uVar4 != 0xffff {
                        FUN_004034e0(uVar4 as u32, param_1);
                        let sVar12: i16 =
                            local_18.wrapping_add((iVar13 >> 1) as i16).wrapping_add(1);
                        let uVar24 = FUN_004278c0(
                            extraout_ECX as i32,
                            extraout_EDX as i32,
                            &raw mut DAT_00521540,
                        );
                        FUN_004a9500(local_8.wrapping_sub(uVar24 as i16), sVar12);
                        DAT_009a8c00 = DAT_009a8c00.wrapping_mul(0xa1);
                        let mut uVar23 = DAT_009a8c00 >> 0x10;
                        if 9999 < uVar23 {
                            uVar23 = 9999;
                        }
                        FUN_004277f0((uVar23 / 100) as u16, 2, 1);
                        let g: unsafe extern "C" fn(i32) = core::mem::transmute(DAT_009a6fe4);
                        g(0x2e);
                        let q = ((uVar23 as u64 as i64) / -100i64) as i16;
                        FUN_004277f0(q.wrapping_mul(100).wrapping_add(uVar23 as i16) as u16, 2, 2);
                    }
                }
            }
        }
    }
}
