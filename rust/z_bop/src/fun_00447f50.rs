// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals, non_camel_case_types)]
unsafe extern "C" {
    static mut DAT_0055697c: u32;
    static mut DAT_00a0085c: i32;
    static mut DAT_00523414: i32;
    static mut DAT_00556980: i32;
    static mut DAT_00556974: i32;
    static mut DAT_00523650: [i32; 20];
    static mut DAT_009a8c2c: u32;
    static mut DAT_009e9658: u32;
    static mut DAT_009a8c24: i32;
    static mut DAT_009e9680: [i32; 0];
    static mut DAT_009ecc34: i32;
    static mut DAT_00a07cc8: u32;
    static mut DAT_009ecc3c: i32;
    static mut DAT_00a080f8: i32;
    static mut DAT_00556978: i32;
    static mut DAT_00523410: i32;
    static mut DAT_00523418: i32;
    static mut DAT_0052340c: i32;
    static mut _DAT_009ed23c: i32;
    static mut s_Fell_Behind__tickcounter___7d_se_005238cc: [i8; 0];
    static mut s_Too_far_Ahead__tickcounter___7d_s_00523890: [i8; 0];
    static mut DAT_00a60a08: *mut i8;
    static mut DAT_009d8a30: [i32; 0];
    static mut DAT_009d6940: u8;
    static mut DAT_00a60a10: *mut u8;
    static mut DAT_00a60a20: *mut u8;
    static mut DAT_00a60a24: *mut i8;
    static mut DAT_009ec45c: i32;
    static mut DAT_009a8bfc: i32;
    static mut DAT_009a7ba0: i32;
    static mut DAT_00a0829c: i32;
    static mut DAT_009a8d90: i32;
    static mut DAT_005235e0: i8;
    static mut s_R___2d_U___2d_N___2d_O___2d_T____00523840: [i8; 0];
    static mut DAT_00a00750: [i8; 0];
    static mut s_Raw_0__2d_1__2d_2__2d_3__2d_4__2_005237f4: [i8; 0];
    static mut s_Raw_10__2d_11__2d_12__2d_13__2d_1_005237a8: [i8; 0];
    static mut s_Pct_0__2d_1__2d_2__2d_3__2d_4__2_0052375c: [i8; 0];
    static mut s_Pct_10__2d_11__2d_12__2d_13__2d_1_00523710: [i8; 0];
    static mut s________Long_Update___d_______War_005236c4: [i8; 0];
    static mut DAT_009e9640: i32;
    static mut DAT_00a00854: i32;
    static mut DAT_009a1fe8: i32;

    fn _sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_00417d70(param_1: i32);
    fn FUN_00463c10(param_1: i32);
    fn FUN_00448f90(param_1: i32, param_2: *mut u8, param_3: i32);
    fn FUN_00463cc0(param_1: i32);
    fn FUN_00463c50(param_1: i32);
    fn FUN_00418500(param_1: i32);
    fn FUN_00470020(param_1: i32) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(a: u32, b: u32, c: u32, d: *const u32);
    fn FUN_004ac760() -> i32;
    fn FUN_00463d00();
    fn FUN_0046e040() -> i32;
    fn FUN_00448d00() -> u32;
    fn FUN_00464900();
    fn FUN_0042d0d0();
    fn FUN_00448da0();
    fn FUN_00462a10() -> *mut i8;
    fn FUN_004aa7f0() -> i32;
    fn FUN_004aa870() -> u32;
    fn FUN_00463bf0();
    fn FUN_00448ed0();
    fn FUN_00450550();
    fn FUN_0046f700() -> i32;
    fn FUN_00418b70();
    fn FUN_00418970();
}

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_004501c0(param_1: *mut core::ffi::c_void);
}

#[unsafe(no_mangle)]
#[allow(
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_variables,
    unused_mut
)]
pub unsafe extern "stdcall" fn FUN_00447f50() {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers are read and written exactly where the original does.
    unsafe {
        let mut iVar4: i32;
        let mut iVar5: i32;
        let mut iVar6: i32;
        let mut iVar8: i32;
        let mut iVar10: i32;
        let mut iVar12: i32;
        let mut iVar16: i32;
        let mut uVar3: u32;
        let mut uVar9: u32;
        let mut uVar17: u32;
        let mut local_37c: i32;
        let mut uStack_378: u32 = 0;
        let mut abStack_36c = [0u8; 80];
        let mut local_31c = [0u8; 256];
        let mut local_21c = [0i8; 180];
        let mut local_168 = [0i8; 180];
        let mut local_b4 = [0i8; 180];

        DAT_0055697c = 0;
        DAT_00a0085c = 0;
        DAT_00523414 = 0;
        DAT_00556980 = 0;
        DAT_00556974 = 0;
        let mut k = 0usize;
        while k < 20 {
            DAT_00523650[k] = 0;
            k += 1;
        }
        loop {
            'inner: loop {
                local_37c = 0;
                iVar8 = FUN_004ac760();
                DAT_009a8c2c = DAT_009a8c2c.wrapping_add(iVar8 as u32);
                iVar16 = DAT_009a8c2c as i32;
                iVar8 = FUN_004ac760();
                iVar8 = DAT_009a8c2c.wrapping_add(iVar8 as u32) as i32;
                iVar4 = (iVar8 as u32).wrapping_sub(DAT_009e9658) as i32;
                loop {
                    DAT_009a8c2c = iVar8 as u32;
                    if !(iVar4 < 8) {
                        break;
                    }
                    iVar8 = FUN_004ac760();
                    iVar8 = DAT_009a8c2c.wrapping_add(iVar8 as u32) as i32;
                    iVar4 = (iVar8 as u32).wrapping_sub(DAT_009e9658) as i32;
                }
                FUN_00463d00();
                iVar4 = FUN_0046e040();
                if iVar4 == 0 && 0x1ba8 < DAT_009a8c24 {
                    (&raw mut DAT_009e9680)
                        .cast::<i32>()
                        .offset(DAT_009ecc34.wrapping_mul(0x5bd) as isize)
                        .write(0);
                    FUN_00463cc0(DAT_009ecc34);
                    return;
                }
                iVar4 = FUN_004ac760();
                DAT_009a8c2c = DAT_009a8c2c.wrapping_add(iVar4 as u32);
                iVar4 = DAT_009a8c2c as i32;
                uVar3 = FUN_00448d00();
                iVar12 = DAT_009a8c2c as i32;
                if uVar3 == 0 {
                    break 'inner;
                }
                if DAT_00a07cc8 == DAT_009e9658 {
                    if DAT_009ecc3c == 0 {
                        return;
                    }
                    FUN_00450550();
                    DAT_009ecc3c = 0;
                    return;
                }
                if 0x49c < (DAT_009a8c2c.wrapping_sub(DAT_009e9658) as i32) {
                    return;
                }
                iVar8 = FUN_004ac760();
                DAT_009a8c2c = DAT_009a8c2c.wrapping_add(iVar8 as u32);
                iVar8 = DAT_009a8c2c.wrapping_sub(DAT_009e9658) as i32;
                while iVar8 < 8 {
                    iVar8 = FUN_004ac760();
                    DAT_009a8c2c = DAT_009a8c2c.wrapping_add(iVar8 as u32);
                    iVar8 = DAT_009a8c2c.wrapping_sub(DAT_009e9658) as i32;
                }
            }
            if DAT_00a080f8 == 1 {
                iVar5 = FUN_004ac760();
                uVar9 = DAT_009a8c2c.wrapping_add(iVar5 as u32);
                uVar17 = uVar9;
                if DAT_0055697c != 0 {
                    uVar17 = (DAT_00556978 as u32).wrapping_add(DAT_0055697c);
                    iVar5 = DAT_00556978 >> 3;
                    if uVar17 < uVar9 {
                        iVar6 = uVar9.wrapping_sub(uVar17) as i32;
                        if iVar5 < uVar9.wrapping_sub(uVar17) as i32 {
                            iVar6 = iVar5;
                        }
                        if iVar6 == 0 {
                            iVar6 = 1;
                        }
                        if 4 < iVar6 {
                            iVar6 = uVar9.wrapping_sub(uVar17) as i32;
                        }
                        uVar17 = uVar17.wrapping_add(iVar6 as u32);
                    } else if uVar9 < uVar17 {
                        iVar6 = uVar17.wrapping_sub(uVar9) as i32;
                        if iVar5 < uVar17.wrapping_sub(uVar9) as i32 {
                            iVar6 = iVar5;
                        }
                        if iVar6 == 0 {
                            iVar6 = 1;
                        }
                        if 4 < iVar6 {
                            iVar6 = uVar17.wrapping_sub(uVar9) as i32;
                        }
                        uVar17 = uVar17.wrapping_sub(iVar6 as u32);
                    }
                }
                if (uVar17.wrapping_sub(DAT_009e9658) as i32) < 4 {
                    uVar17 = DAT_009e9658.wrapping_add(4);
                }
                DAT_00556978 = uVar17.wrapping_sub(DAT_0055697c) as i32;
                DAT_009a8c2c = uVar17;
                FUN_00464900();
                DAT_00523410 = 0;
                DAT_00523418 = DAT_009a8c2c.wrapping_sub(DAT_009e9658) as i32;
                FUN_00417d70(DAT_009a8c2c as i32);
                DAT_009e9658 = DAT_009a8c2c;
                DAT_00a07cc8 = DAT_009a8c2c;
                DAT_0055697c = DAT_009a8c2c;
                DAT_009a8c2c = DAT_009a8c2c.wrapping_sub(uVar17).wrapping_add(uVar9);
                FUN_0042d0d0();
            } else {
                iVar5 = FUN_004ac760();
                DAT_009a8c2c = DAT_009a8c2c.wrapping_add(iVar5 as u32);
                iVar5 = DAT_00556978 >> 3;
                if (_DAT_009ed23c as u32).wrapping_add(DAT_00a07cc8) < DAT_009a8c2c {
                    iVar10 = (DAT_009a8c2c
                        .wrapping_sub(_DAT_009ed23c as u32)
                        .wrapping_sub(DAT_00a07cc8) as i32)
                        >> 4;
                    if iVar10 == 0 {
                        iVar10 = 1;
                    }
                    if iVar5 < iVar10 {
                        iVar10 = iVar5;
                    }
                    iVar6 = 0i32.wrapping_sub(iVar10);
                    DAT_009a8c2c = DAT_009a8c2c.wrapping_add(iVar6 as u32);
                    DAT_0052340c = DAT_0052340c.wrapping_add(iVar10);
                } else if DAT_009a8c2c < (_DAT_009ed23c as u32).wrapping_add(DAT_00a07cc8) {
                    iVar6 = ((_DAT_009ed23c as u32)
                        .wrapping_sub(DAT_009a8c2c)
                        .wrapping_add(DAT_00a07cc8) as i32)
                        >> 4;
                    if iVar6 == 0 {
                        iVar6 = 1;
                    }
                    if iVar5 < iVar6 {
                        iVar6 = iVar5;
                    }
                    iVar10 = 0i32.wrapping_sub(iVar6);
                    DAT_009a8c2c = DAT_009a8c2c.wrapping_add(iVar6 as u32);
                    DAT_0052340c = DAT_0052340c.wrapping_add(iVar10);
                }
                if (DAT_009a8c2c as i32) < (DAT_00a07cc8 as i32) {
                    _sprintf(
                        local_b4.as_mut_ptr(),
                        (&raw const s_Fell_Behind__tickcounter___7d_se_005238cc).cast::<i8>(),
                        DAT_009a8c2c,
                        DAT_00a07cc8,
                        (_DAT_009ed23c as u32)
                            .wrapping_sub(DAT_009a8c2c)
                            .wrapping_add(DAT_00a07cc8),
                    );
                    iVar5 = (_DAT_009ed23c as u32)
                        .wrapping_sub(DAT_009a8c2c)
                        .wrapping_add(DAT_00a07cc8) as i32;
                    DAT_009a8c2c = DAT_009a8c2c.wrapping_add(iVar5 as u32);
                    DAT_0052340c = DAT_0052340c.wrapping_sub(iVar5);
                }
                if ((_DAT_009ed23c.wrapping_add(0x674))
                    < (DAT_009a8c2c.wrapping_sub(DAT_00a07cc8) as i32))
                    && {
                        iVar5 = FUN_0046e040();
                        iVar5 == 0
                    }
                {
                    FUN_00463c10(DAT_009ecc34);
                    _sprintf(
                        local_168.as_mut_ptr(),
                        (&raw const s_Too_far_Ahead__tickcounter___7d_s_00523890).cast::<i8>(),
                        DAT_009a8c2c,
                        DAT_00a07cc8,
                        DAT_009a8c2c.wrapping_sub(DAT_00a07cc8),
                    );
                    iVar6 = 0;
                    iVar5 = FUN_004ac760();
                    DAT_009a8c24 = 0;
                    DAT_009a8c2c = DAT_009a8c2c.wrapping_add(iVar5 as u32);
                    FUN_00448da0();
                    // strlen + 1 of DAT_00a60a08, copied into local_31c
                    let src = DAT_00a60a08;
                    let mut n = 0usize;
                    while *src.add(n) != 0 {
                        n += 1;
                    }
                    n += 1;
                    let mut k = 0usize;
                    while k < n {
                        local_31c[k] = *src.add(k) as u8;
                        k += 1;
                    }
                    let pcVar18 = FUN_00462a10();
                    if !pcVar18.is_null() {
                        let mut n = 0usize;
                        while *pcVar18.add(n) != 0 {
                            n += 1;
                        }
                        n += 1;
                        let mut e = 0usize;
                        while local_31c[e] != 0 {
                            e += 1;
                        }
                        let mut k = 0usize;
                        while k < n {
                            local_31c[e + k] = *pcVar18.add(k) as u8;
                            k += 1;
                        }
                    }
                    FUN_00448f90(1, local_31c.as_mut_ptr(), 0x34);
                    loop {
                        if (DAT_009a8c2c.wrapping_sub(DAT_00a07cc8) as i32) <= _DAT_009ed23c {
                            break;
                        }
                        iVar5 = FUN_004aa7f0();
                        if iVar5 != 0 {
                            uVar17 = FUN_004aa870();
                            if uVar17 == 0x1b {
                                DAT_009d6940 = 1;
                                FUN_00463c50(DAT_009ecc34);
                                iVar8 = DAT_009ecc34;
                                (&raw mut DAT_009d8a30)
                                    .cast::<i32>()
                                    .offset(DAT_009ecc34 as isize)
                                    .write(1);
                                (&raw mut DAT_009e9680)
                                    .cast::<i32>()
                                    .offset(iVar8.wrapping_mul(0x5bd) as isize)
                                    .write(0);
                                FUN_00463cc0(DAT_009ecc34);
                                return;
                            }
                        }
                        iVar5 = DAT_009a8c2c as i32;
                        FUN_00463d00();
                        iVar10 = FUN_004ac760();
                        DAT_009a8c24 = DAT_009a8c24.wrapping_add(
                            DAT_009a8c2c
                                .wrapping_add(iVar10 as u32)
                                .wrapping_sub(iVar5 as u32) as i32,
                        );
                        iVar6 = (iVar6 as u32)
                            .wrapping_sub(iVar5 as u32)
                            .wrapping_add(DAT_009a8c2c)
                            .wrapping_add(iVar10 as u32) as i32;
                        DAT_009a8c2c = iVar5 as u32;
                        if 0x1ba8 < DAT_009a8c24 {
                            DAT_009d6940 = 1;
                            FUN_00463c50(DAT_009ecc34);
                            iVar8 = DAT_009ecc34;
                            (&raw mut DAT_009d8a30)
                                .cast::<i32>()
                                .offset(DAT_009ecc34 as isize)
                                .write(1);
                            (&raw mut DAT_009e9680)
                                .cast::<i32>()
                                .offset(iVar8.wrapping_mul(0x5bd) as isize)
                                .write(0);
                            FUN_00463cc0(DAT_009ecc34);
                            return;
                        }
                        if 0xec < iVar6 {
                            iVar6 = 0;
                            FUN_00463bf0();
                        }
                        uVar17 = 0x1ba8u32.wrapping_sub(DAT_009a8c24 as u32) / 0x76u32;
                        if uStack_378 != uVar17 {
                            let pbVar11: *mut u8;
                            if (uVar17 as i32) < 0x32 {
                                _sprintf(
                                    abStack_36c.as_mut_ptr().cast::<i8>(),
                                    DAT_00a60a24 as *const i8,
                                    (uVar17 as i32) / 2,
                                    (uVar17 & 1).wrapping_mul(5),
                                );
                                pbVar11 = abStack_36c.as_mut_ptr();
                            } else {
                                let mut p = DAT_00a60a10;
                                if (uVar17 & 1) != 0 {
                                    p = DAT_00a60a20;
                                }
                                pbVar11 = p;
                            }
                            FUN_00448f90(3, pbVar11, 0x34);
                            uStack_378 = uVar17;
                        }
                        uVar3 = FUN_00448d00();
                        if !(uVar3 == 0 || DAT_00a07cc8 != DAT_009e9658) {
                            break;
                        }
                    }
                    FUN_00448ed0();
                    FUN_004ac760();
                    uVar3 = FUN_00448d00();
                    if uVar3 != 0 && DAT_00a07cc8 == DAT_009e9658 {
                        return;
                    }
                    DAT_009a8c2c = (_DAT_009ed23c as u32).wrapping_add(DAT_00a07cc8);
                }
                uVar17 = DAT_009a8c2c;
                if DAT_009e9658 < DAT_009a8c2c {
                    uVar9 = DAT_009a8c2c;
                    if DAT_0055697c != 0 {
                        uVar9 = (DAT_00556978 as u32).wrapping_add(DAT_0055697c);
                        iVar5 = DAT_00556978 >> 3;
                        if uVar9 < DAT_009a8c2c {
                            iVar6 = DAT_009a8c2c.wrapping_sub(uVar9) as i32;
                            if iVar5 < DAT_009a8c2c.wrapping_sub(uVar9) as i32 {
                                iVar6 = iVar5;
                            }
                            if iVar6 == 0 {
                                iVar6 = 1;
                            }
                            if 4 < iVar6 {
                                iVar6 = DAT_009a8c2c.wrapping_sub(uVar9) as i32;
                            }
                            uVar9 = uVar9.wrapping_add(iVar6 as u32);
                        } else if DAT_009a8c2c < uVar9 {
                            iVar6 = uVar9.wrapping_sub(DAT_009a8c2c) as i32;
                            if iVar5 < uVar9.wrapping_sub(DAT_009a8c2c) as i32 {
                                iVar6 = iVar5;
                            }
                            if iVar6 == 0 {
                                iVar6 = 1;
                            }
                            if 4 < iVar6 {
                                iVar6 = uVar9.wrapping_sub(DAT_009a8c2c) as i32;
                            }
                            uVar9 = uVar9.wrapping_sub(iVar6 as u32);
                        }
                    }
                    if (uVar9.wrapping_sub(DAT_009e9658) as i32) < 4 {
                        uVar9 = DAT_009e9658.wrapping_add(4);
                    }
                    DAT_00556978 = uVar9.wrapping_sub(DAT_0055697c) as i32;
                    DAT_009a8c2c = uVar9;
                    FUN_00464900();
                    FUN_00418500(DAT_00556978);
                    DAT_00523410 = 1;
                    local_37c = DAT_009a8c2c.wrapping_sub(DAT_009e9658) as i32;
                    FUN_00417d70(DAT_009a8c2c as i32);
                    DAT_009e9658 = DAT_009a8c2c;
                    DAT_0055697c = DAT_009a8c2c;
                    DAT_009a8c2c = DAT_009a8c2c.wrapping_sub(uVar9).wrapping_add(uVar17);
                }
            }
            iVar5 = FUN_004ac760();
            DAT_009a8c2c = DAT_009a8c2c.wrapping_add(iVar5 as u32);
            iVar12 = DAT_009a8c2c.wrapping_sub(iVar12 as u32) as i32;
            iVar5 = FUN_004ac760();
            iVar5 = DAT_009a8c2c.wrapping_add(iVar5 as u32) as i32;
            iVar6 = (iVar5 as u32)
                .wrapping_sub(_DAT_009ed23c as u32)
                .wrapping_sub(DAT_00a07cc8) as i32;
            if iVar6 < 0x1d8 {
                DAT_009ec45c = 0;
            } else if iVar6 < 0x3b0 {
                DAT_009ec45c = 1;
            } else {
                DAT_009ec45c = 2;
                if 0x587 < iVar6 {
                    DAT_009ec45c = 3;
                }
            }
            DAT_009a8c2c = iVar5 as u32;
            if DAT_00556980 == 0 {
                DAT_009a8bfc = 0;
            } else {
                iVar6 = FUN_0046f700();
                iVar6 = FUN_00470020(iVar6);
                DAT_00556974 = DAT_00556974.wrapping_add(
                    (iVar6 as u32)
                        .wrapping_sub(DAT_00556980 as u32)
                        .wrapping_mul(10) as i32,
                );
                if DAT_00556974 == 0 {
                    DAT_009a8bfc = 0;
                } else if DAT_00556974 < 10 {
                    DAT_009a8bfc = 1;
                } else {
                    DAT_009a8bfc = 2;
                    if 0x13 < DAT_00556974 {
                        DAT_009a8bfc = 3;
                    }
                }
                DAT_00556980 = iVar6;
                if DAT_00556974 != 0 {
                    DAT_00556974 = DAT_00556974.wrapping_sub(1);
                }
            }
            FUN_00418b70();
            if 1 < DAT_00a080f8 {
                DAT_00523414 = 1;
            }
            FUN_004501c0(core::ptr::null_mut());
            DAT_00523414 = 0;
            FUN_0042d0d0();
            FUN_00418970();
            iVar6 = FUN_004ac760();
            DAT_009a8c2c = DAT_009a8c2c.wrapping_add(iVar6 as u32);
            iVar16 = DAT_009a8c2c.wrapping_sub(iVar16 as u32) as i32;
            if iVar16 == 0 {
                iVar16 = 1;
            }
            if DAT_005235e0 == 0 {
                DAT_00a0829c = 0;
                DAT_009a7ba0 = 0;
            } else {
                if 0x3b0 < DAT_009a7ba0 {
                    DAT_00a0829c = 0;
                    DAT_009a7ba0 = 0;
                }
                DAT_009a7ba0 = DAT_009a7ba0.wrapping_add(iVar16);
                DAT_00a0829c = DAT_00a0829c.wrapping_add(1);
                DAT_009a8d90 = iVar16;
                let mut iVarQ: i32 = 0;
                if iVar16 == 0 {
                    RaiseException(0xC0000094, 0, 0, core::ptr::null());
                } else {
                    let lVarQ: i64 = 0xec_i64 / (iVar16 as i64);
                    if lVarQ > 2147483647i64 || lVarQ < -2147483647i64 - 1 {
                        RaiseException(0xC0000094, 0, 0, core::ptr::null());
                    } else {
                        iVarQ = lVarQ as i32;
                    }
                }
                _sprintf(
                    local_21c.as_mut_ptr(),
                    (&raw const s_R___2d_U___2d_N___2d_O___2d_T____00523840).cast::<i8>(),
                    DAT_009a8c2c.wrapping_sub(iVar5 as u32),
                    iVar12,
                    (iVar4 as u32).wrapping_sub(iVar8 as u32) as i32,
                    (iVar16 as u32)
                        .wrapping_sub((iVar4 as u32).wrapping_sub(iVar8 as u32))
                        .wrapping_sub(iVar12 as u32)
                        .wrapping_sub(DAT_009a8c2c.wrapping_sub(iVar5 as u32))
                        as i32,
                    iVar16,
                    iVarQ,
                    DAT_009a8c2c,
                    DAT_009e9658,
                    DAT_00a07cc8,
                    DAT_009a8c2c.wrapping_sub(DAT_00a07cc8),
                    _DAT_009ed23c,
                    DAT_0052340c,
                );
                if iVar12 < 0 {
                    iVar12 = 0;
                }
                if iVar12 < 0x14 {
                    DAT_00523650[iVar12 as usize] = DAT_00523650[iVar12 as usize].wrapping_add(1);
                } else {
                    DAT_00523650[19] = DAT_00523650[19].wrapping_add(1);
                }
                _sprintf(
                    (&raw mut DAT_00a00750).cast::<i8>(),
                    (&raw const s_Raw_0__2d_1__2d_2__2d_3__2d_4__2_005237f4).cast::<i8>(),
                    DAT_00523650[0],
                    DAT_00523650[1],
                    DAT_00523650[2],
                    DAT_00523650[3],
                    DAT_00523650[4],
                    DAT_00523650[5],
                    DAT_00523650[6],
                    DAT_00523650[7],
                    DAT_00523650[8],
                    DAT_00523650[9],
                );
                uVar17 = 0;
                _sprintf(
                    (&raw mut DAT_00a00750).cast::<i8>(),
                    (&raw const s_Raw_10__2d_11__2d_12__2d_13__2d_1_005237a8).cast::<i8>(),
                    DAT_00523650[10],
                    DAT_00523650[11],
                    DAT_00523650[12],
                    DAT_00523650[13],
                    DAT_00523650[14],
                    DAT_00523650[15],
                    DAT_00523650[16],
                    DAT_00523650[17],
                    DAT_00523650[18],
                    DAT_00523650[19],
                );
                let mut k = 0usize;
                while k < 20 {
                    uVar17 = uVar17.wrapping_add(DAT_00523650[k] as u32);
                    k += 1;
                }
                if uVar17 != 0 {
                    _sprintf(
                        (&raw mut DAT_00a00750).cast::<i8>(),
                        (&raw const s_Pct_0__2d_1__2d_2__2d_3__2d_4__2_0052375c).cast::<i8>(),
                        (DAT_00523650[0] as u32).wrapping_mul(100) / uVar17,
                        (DAT_00523650[1] as u32).wrapping_mul(100) / uVar17,
                        (DAT_00523650[2] as u32).wrapping_mul(100) / uVar17,
                        (DAT_00523650[3] as u32).wrapping_mul(100) / uVar17,
                        (DAT_00523650[4] as u32).wrapping_mul(100) / uVar17,
                        (DAT_00523650[5] as u32).wrapping_mul(100) / uVar17,
                        (DAT_00523650[6] as u32).wrapping_mul(100) / uVar17,
                        (DAT_00523650[7] as u32).wrapping_mul(100) / uVar17,
                        (DAT_00523650[8] as u32).wrapping_mul(100) / uVar17,
                        (DAT_00523650[9] as u32).wrapping_mul(100) / uVar17,
                    );
                    _sprintf(
                        (&raw mut DAT_00a00750).cast::<i8>(),
                        (&raw const s_Pct_10__2d_11__2d_12__2d_13__2d_1_00523710).cast::<i8>(),
                        (DAT_00523650[10] as u32).wrapping_mul(100) / uVar17,
                        (DAT_00523650[11] as u32).wrapping_mul(100) / uVar17,
                        (DAT_00523650[12] as u32).wrapping_mul(100) / uVar17,
                        (DAT_00523650[13] as u32).wrapping_mul(100) / uVar17,
                        (DAT_00523650[14] as u32).wrapping_mul(100) / uVar17,
                        (DAT_00523650[15] as u32).wrapping_mul(100) / uVar17,
                        (DAT_00523650[16] as u32).wrapping_mul(100) / uVar17,
                        (DAT_00523650[17] as u32).wrapping_mul(100) / uVar17,
                        (DAT_00523650[18] as u32).wrapping_mul(100) / uVar17,
                        (DAT_00523650[19] as u32).wrapping_mul(100) / uVar17,
                    );
                }
                iVar16 = 0;
                iVar8 = 0;
                if DAT_009e9640 < DAT_00a00854 {
                    iVar4 = (DAT_00a00854 as u32).wrapping_sub(DAT_009e9640 as u32) as i32;
                    let mut pcVar18 = (DAT_009e9640 as u32)
                        .wrapping_mul(0x23)
                        .wrapping_add(3)
                        .wrapping_add(DAT_009a1fe8 as u32)
                        as *const i8;
                    loop {
                        if *pcVar18 != 0 {
                            if *pcVar18.offset(-1) == 6 {
                                iVar8 = iVar8.wrapping_add(1);
                            } else {
                                iVar16 = iVar16.wrapping_add(1);
                            }
                        }
                        pcVar18 = pcVar18.wrapping_add(0x23);
                        iVar4 = iVar4.wrapping_sub(1);
                        if iVar4 == 0 {
                            break;
                        }
                    }
                }
                if 7 < iVar12 {
                    _sprintf(
                        (&raw mut DAT_00a00750).cast::<i8>(),
                        (&raw const s________Long_Update___d_______War_005236c4).cast::<i8>(),
                        iVar12,
                        local_37c,
                        iVar8,
                        iVar16,
                    );
                }
            }
        }
    }
}
