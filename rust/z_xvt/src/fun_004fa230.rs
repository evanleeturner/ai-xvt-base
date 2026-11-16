// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00534304: i8;
    static mut DAT_0081b740: i8;
    static mut DAT_005321b8: i8;
    static mut DAT_0075637a: i8;
    static mut DAT_007438e4: i8;
    static mut DAT_007438d8: i8;
    static mut DAT_00743880: i8;
    static mut DAT_0081bc80: i8;

    static mut DAT_00531334: i8;
    static mut DAT_005342f0: i8;
    static mut DAT_005342e4: i8;
    static mut DAT_005321bc: i8;
    static mut DAT_005342d4: i8;
    static mut DAT_005342cc: i8;
    static mut DAT_005342bc: i8;
    static mut DAT_005321a8: i8;

    static mut DAT_0081c012: i32;
    static mut DAT_0081bfba: i32;
    static mut DAT_0081bfb6: i32;
    static mut DAT_0081d32a: i32;
    static mut DAT_0081d326: i32;
    static mut DAT_0081d402: i32;
    static mut DAT_0081d40a: i32;
    static mut DAT_0081d40e: i32;
    static mut DAT_0081bcd0: u32;
    static mut _DAT_0081b6fc: u32;
    static mut DAT_0081b850: i32;
    static mut DAT_00654c18: u32;
    static mut DAT_00654c40: u32;
    static mut DAT_00654c60: i32;
    static mut DAT_00654bd0: i32;
    static mut DAT_00654c88: i32;
    static mut DAT_00654ca8: i32;
    static mut DAT_00654cb8: i32;
    static mut DAT_00654cd8: i32;
    static mut DAT_00654d44: i32;
    static mut DAT_0075766c: i16;
    static mut DAT_0081fa7e: i32;
    static mut DAT_0081fa8e: i32;
    static mut DAT_0081fa92: i32;
    static mut DAT_0081fa96: i32;
    static mut DAT_0081fa9e: i32;
    static mut DAT_0081e426: i32;
    static mut DAT_0081f7cc: u32;
    static mut DAT_0081f7da: i32;
    static mut DAT_0081f7de: i32;
    static mut DAT_0081f7e2: i32;
    static mut DAT_0081f7e6: i32;
    static mut DAT_0081f7ea: i32;
    static mut DAT_0081f7ee: i32;
    static mut DAT_0081f7fa: i32;
    static mut DAT_0081f812: i32;
    static mut DAT_0081e0a6: i32;
    static mut DAT_0081e0c6: i32;
    static mut DAT_0081e0e6: i32;
    static mut DAT_0081e1a6: i32;
    static mut DAT_0081e266: i32;
    static mut DAT_0081e286: i32;
    static mut DAT_0081e2a6: i32;
    static mut DAT_0081e366: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dbe00(param_1: i32) -> u8;
    fn FUN_004d0e60(param_1: *mut i8, param_2: i32, param_3: i32);
    fn FUN_004dcfd0(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_004dcfa0(param_1: u32) -> *mut i8;
    fn _sprintf(param_1: *mut i8, param_2: *const i8, ...) -> i32;
    fn FUN_004da920(param_1: i32, param_2: *mut i8, param_3: *mut i32, param_4: u32);
    fn FUN_004da7c0(param_1: i32, param_2: *mut i8, param_3: i32, param_4: i32, param_5: u32);
    fn FUN_004d5180(param_1: *mut core::ffi::c_void);
    fn FUN_004d51b0(param_1: *mut core::ffi::c_void);
    fn FUN_004cdbc0() -> i32;
}

unsafe fn ld32(p: *const u8) -> i32 {
    // SAFETY: the caller passes an address inside the game's own data.
    unsafe { p.cast::<i32>().read_unaligned() }
}

unsafe fn el32(base: *const u8, i: u32, s: u32) -> i32 {
    // SAFETY: the caller passes the address of a game table and its index.
    unsafe { ld32(base.wrapping_add(i.wrapping_mul(s.wrapping_mul(4)) as usize)) }
}

macro_rules! ld {
    ($b:ident) => {
        ld32((&raw const $b).cast::<u8>())
    };
}

macro_rules! el {
    ($b:ident, $i:expr, $s:expr) => {
        el32((&raw const $b).cast::<u8>(), ($i) as u32, $s)
    };
}

macro_rules! idx {
    ($b:ident, $i:expr) => {
        (&raw mut $b).wrapping_add(($i) as usize).read_unaligned()
    };
}

macro_rules! buf {
    () => {
        (&raw mut DAT_0081b740)
    };
}

#[unsafe(no_mangle)]
#[allow(
    non_snake_case,
    unused_assignments,
    unused_mut,
    unused_variables,
    unused_labels
)]
pub unsafe extern "C" fn FUN_004fa230(param_1: i32) -> u32 {
    let mut s_var2: i16;
    let mut pcVar3: *mut i8 = core::ptr::null_mut();
    let mut iVar4: i32;
    let mut pcVar5: *mut i8;
    let mut pcVar6: *mut i8;
    let mut piVar7: *mut i32 = core::ptr::null_mut();
    let mut iVar8: i32;
    let mut puVar9: *mut u32;
    let mut iVar10: i32;
    let mut piVar11: i32;
    let mut iVar12: i32;
    let mut iVar13: i32;
    let mut uVar14: u32 = 0;
    let mut uVar15: u32 = 0;
    let mut uVar16: u32 = 0;
    let mut puVar17: *mut u32;
    let mut local_38: i32;
    let mut local_34: i32;
    let mut local_30: [u32; 3] = [0; 3];
    let mut local_24: i32;
    let mut local_20: [i32; 4] = [0; 4];
    let mut local_10: [u32; 4] = [0; 4];

    // SAFETY: every global, callee and pointer here belongs to the game and is
    // read and written where the original does.
    unsafe {
        if FUN_004dbe00(0x12) != 0 && FUN_004dbe00(0x10) != 0 && FUN_004dbe00(0x11) != 0 {
            FUN_004d0e60(&raw mut DAT_00534304, 0xb8, 0x16a);
        }
        FUN_004dcfd0(local_20.as_mut_ptr(), 0x54, 0x5a, 0x1b2, 0x6a);
        'l430: {
            if ld!(DAT_0081c012) == 0 {
                piVar7 = local_20.as_mut_ptr();
                uVar16 = 0xffff;
                pcVar3 = FUN_004dcfa0(0x12a);
                break 'l430;
            }
            'tail: {
                'l404: {
                    if ld!(DAT_0081bfba) == 1 {
                        if ld!(DAT_0081d32a).wrapping_sub(ld!(DAT_0081d326)) == 1 {
                            pcVar3 = FUN_004dcfa0(0xc1);
                            uVar16 = 0x2a1;
                            break 'l404;
                        } else {
                            pcVar3 = FUN_004dcfa0(0x147);
                            iVar10 = ld!(DAT_0081d32a);
                            pcVar5 = FUN_004dcfa0(0x142);
                            iVar12 = ld!(DAT_0081d326).wrapping_add(1);
                            uVar14 = 4;
                            pcVar6 = FUN_004dcfa0(0x2a1);
                            _sprintf(
                                buf!(),
                                &raw const DAT_005342f0,
                                1i32,
                                pcVar6,
                                uVar14,
                                iVar12,
                                pcVar5,
                                iVar10,
                                pcVar3,
                            );
                        }
                    } else {
                        local_30 = [0; 3];
                        if -1 < ld!(DAT_0081d402) {
                            piVar7 = &raw mut DAT_0081d40e;
                            iVar10 = ld!(DAT_0081d402).wrapping_add(1);
                            loop {
                                iVar12 = piVar7.read_unaligned();
                                piVar7 = piVar7.wrapping_add(1);
                                iVar10 = iVar10.wrapping_sub(1);
                                let p = local_30.as_mut_ptr().wrapping_offset(iVar12 as isize);
                                p.write(p.read().wrapping_add(1));
                                if iVar10 == 0 {
                                    break;
                                }
                            }
                        }
                        if ld!(DAT_0081bfb6) == 0 {
                            iVar10 = 0;
                        } else if ld!(DAT_0081bfb6) == 1 {
                            iVar10 = 1;
                        } else {
                            iVar10 = 0;
                        }
                        if local_30[0] == ld!(DAT_0081d40a) as u32
                            || local_30[1] == ld!(DAT_0081d40a) as u32
                        {
                            pcVar3 = FUN_004dcfa0(0xc1);
                            uVar16 = 0x2a3;
                            break 'l404;
                        }
                        pcVar3 = FUN_004dcfa0(0x2a4);
                        piVar11 = ld!(DAT_0081d40a);
                        pcVar5 = FUN_004dcfa0(0x142);
                        let cnt = local_30[iVar10 as usize];
                        uVar14 = 4;
                        pcVar6 = FUN_004dcfa0(0x2a3);
                        _sprintf(
                            buf!(),
                            &raw const DAT_005342f0,
                            1i32,
                            pcVar6,
                            uVar14,
                            cnt,
                            pcVar5,
                            piVar11,
                            pcVar3,
                        );
                    }
                    break 'tail;
                }
                uVar14 = 4;
                pcVar5 = FUN_004dcfa0(uVar16);
                _sprintf(
                    buf!(),
                    &raw const DAT_00531334,
                    1i32,
                    pcVar5,
                    uVar14,
                    pcVar3,
                );
            }
            piVar7 = local_20.as_mut_ptr();
            uVar16 = 0xffff;
            pcVar3 = buf!();
        }
        FUN_004da920(0xf, pcVar3, piVar7, uVar16);
        iVar12 = 0x72;
        iVar10 = 0xe4;
        uVar16 = DAT_0081bcd0;
        pcVar3 = FUN_004dcfa0(0x13d);
        FUN_004da7c0(0xc, pcVar3, iVar10, iVar12, uVar16);
        iVar12 = 0x72;
        iVar10 = 0x12a;
        uVar16 = DAT_0081bcd0;
        pcVar3 = FUN_004dcfa0(0x143);
        FUN_004da7c0(0xc, pcVar3, iVar10, iVar12, uVar16);
        iVar13 = 0x72;
        iVar12 = 0x16b;
        iVar10 = 0x86;
        uVar16 = DAT_0081bcd0;
        pcVar3 = FUN_004dcfa0(0x20d);
        FUN_004da7c0(0xc, pcVar3, iVar12, iVar13, uVar16);

        puVar9 = &raw mut DAT_00654c18;
        local_34 = 0;
        local_24 = 0;
        local_38 = el!(DAT_0081fa7e, DAT_00654c18, 7);
        loop {
            iVar12 = 0x58;
            uVar16 = *puVar9;
            if uVar16 == 0xffffffff {
                break;
            }
            if idx!(DAT_00654c60, uVar16) != 0 {
                if local_38 != el!(DAT_0081fa7e, uVar16, 7) {
                    local_34 = local_24;
                    local_38 = el!(DAT_0081fa7e, uVar16, 7);
                }
                s_var2 = ((local_34 < 3) as i16).wrapping_add(1);
                if DAT_00654d44 == 0 {
                    if ld!(DAT_0081bfba) == 1 || ld!(DAT_0081bfba) == 2 {
                        _sprintf(
                            buf!(),
                            &raw const DAT_005342e4,
                            s_var2 as i32,
                            local_34.wrapping_add(1),
                            1i32,
                            (&raw mut DAT_0075637a)
                                .wrapping_add(uVar16.wrapping_mul(0x1e5) as usize),
                        );
                        FUN_004da7c0(0xc, buf!(), 0x58, iVar10, 0xffff);
                        _sprintf(
                            buf!(),
                            &raw const DAT_005321b8,
                            el!(DAT_0081fa7e, *puVar9, 7),
                        );
                        FUN_004da7c0(0xc, buf!(), 0xe4, iVar10, 0xffff);
                        _sprintf(
                            buf!(),
                            &raw const DAT_005321bc,
                            el!(DAT_0081fa8e, *puVar9, 7),
                            el!(DAT_0081fa92, *puVar9, 7),
                        );
                        FUN_004da7c0(0xc, buf!(), 0x12a, iVar10, 0xffff);
                        _sprintf(
                            buf!(),
                            &raw const DAT_005321b8,
                            el!(DAT_0081fa96, *puVar9, 7),
                        );
                        iVar12 = 0x67;
                        FUN_004da7c0(0xc, buf!(), 0x16b, iVar10, 0xffff);
                        iVar10 = iVar10.wrapping_add(0xf);
                    } else if idx!(DAT_00654bd0, uVar16) == 0 {
                        _sprintf(
                            buf!(),
                            &raw const DAT_005342e4,
                            s_var2 as i32,
                            local_34.wrapping_add(1),
                            1i32,
                            (&raw mut DAT_0075637a)
                                .wrapping_add(uVar16.wrapping_mul(0x1e5) as usize),
                        );
                        iVar12 = 0x67;
                        FUN_004da7c0(0xc, buf!(), 0x58, iVar10, 0xffff);
                        iVar10 = iVar10.wrapping_add(0xf);
                    } else {
                        _sprintf(
                            buf!(),
                            &raw const DAT_005342e4,
                            s_var2 as i32,
                            local_34.wrapping_add(1),
                            1i32,
                            (&raw mut DAT_0075637a)
                                .wrapping_add(uVar16.wrapping_mul(0x1e5) as usize),
                        );
                        FUN_004da7c0(0xc, buf!(), 0x58, iVar10, 0xffff);
                        _sprintf(
                            buf!(),
                            &raw const DAT_005321b8,
                            el!(DAT_0081fa7e, *puVar9, 7),
                        );
                        FUN_004da7c0(0xc, buf!(), 0xe4, iVar10, 0xffff);
                        _sprintf(
                            buf!(),
                            &raw const DAT_005321bc,
                            el!(DAT_0081fa8e, *puVar9, 7),
                            el!(DAT_0081fa92, *puVar9, 7),
                        );
                        FUN_004da7c0(0xc, buf!(), 0x12a, iVar10, 0xffff);
                        _sprintf(
                            buf!(),
                            &raw const DAT_005321b8,
                            el!(DAT_0081fa96, *puVar9, 7),
                        );
                        iVar12 = 0x67;
                        FUN_004da7c0(0xc, buf!(), 0x16b, iVar10, 0xffff);
                        iVar10 = iVar10.wrapping_add(0xf);
                    }
                } else if idx!(DAT_00654bd0, uVar16) != 0 && {
                    iVar13 = 0;
                    0 < DAT_0075766c
                } {
                    local_30[0] = (&raw mut DAT_0081e426) as usize as u32;
                    pcVar3 = &raw mut DAT_007438e4;
                    iVar8 = iVar10;
                    loop {
                        iVar10 = iVar8;
                        if *pcVar3 != 0
                            && (pcVar3.wrapping_offset(-0xc).read() as u8 as u32) == *puVar9
                        {
                            pcVar5 = pcVar3.wrapping_offset(-100);
                            uVar14 = 1;
                            pcVar6 = FUN_004dcfa0(
                                ld32(local_30[0] as usize as *const u8).wrapping_add(0x8e) as u32,
                            );
                            _sprintf(
                                buf!(),
                                &raw const DAT_005342d4,
                                s_var2 as i32,
                                local_34.wrapping_add(1),
                                6i32,
                                pcVar6,
                                uVar14,
                                pcVar5,
                            );
                            FUN_004da7c0(0xc, buf!(), iVar12, iVar8, 0xffff);
                            _sprintf(
                                buf!(),
                                &raw const DAT_005321b8,
                                el!(DAT_0081fa7e, *puVar9, 7),
                            );
                            FUN_004da7c0(0xc, buf!(), 0xe4, iVar8, 0xffff);
                            _sprintf(
                                buf!(),
                                &raw const DAT_005321bc,
                                el!(DAT_0081fa8e, *puVar9, 7),
                                el!(DAT_0081fa92, *puVar9, 7),
                            );
                            iVar12 = 0x16b;
                            FUN_004da7c0(0xc, buf!(), 0x12a, iVar8, 0xffff);
                            _sprintf(
                                buf!(),
                                &raw const DAT_005321b8,
                                el!(DAT_0081fa96, *puVar9, 7),
                            );
                            iVar10 = iVar8.wrapping_add(0xf);
                            FUN_004da7c0(0xc, buf!(), 0x16b, iVar8, 0xffff);
                        }
                        pcVar3 = pcVar3.wrapping_add(0x562);
                        local_30[0] = local_30[0].wrapping_add(4);
                        iVar13 = iVar13.wrapping_add(1);
                        iVar8 = iVar10;
                        if !(iVar13 < DAT_0075766c as i32) {
                            break;
                        }
                    }
                }
                if idx!(DAT_00654bd0, *puVar9) == 0 {
                    piVar7 = &raw mut DAT_00654c88;
                    iVar13 = iVar10;
                    loop {
                        iVar8 = piVar7.read_unaligned();
                        iVar10 = iVar13;
                        if iVar8 == -1 {
                            break;
                        }
                        if ((&raw mut DAT_007438d8)
                            .wrapping_offset(
                                (el!(DAT_0081f7da, iVar8, 0x16) as u32).wrapping_mul(0x562) as i32
                                    as isize,
                            )
                            .read() as u8 as u32)
                            == *puVar9
                        {
                            FUN_004dcfd0(
                                local_20.as_mut_ptr(),
                                iVar12,
                                iVar13,
                                0xe1,
                                iVar13.wrapping_add(0xe),
                            );
                            FUN_004d5180(local_10.as_mut_ptr().cast());
                            FUN_004d51b0(local_20.as_mut_ptr().cast());
                            if DAT_00654d44 == 0 {
                                puVar17 = (&raw mut DAT_0081f7cc)
                                    .wrapping_add((iVar8 as u32).wrapping_mul(0x16) as usize);
                                if el!(DAT_0081f812, iVar8, 0x16) == 0 {
                                    uVar14 = 1;
                                    pcVar3 = FUN_004dcfa0(
                                        el!(DAT_0081f7e2, iVar8, 0x16).wrapping_add(0x8e) as u32,
                                    );
                                    _sprintf(
                                        buf!(),
                                        &raw const DAT_00531334,
                                        6i32,
                                        pcVar3,
                                        uVar14,
                                        puVar17,
                                    );
                                } else {
                                    pcVar3 = FUN_004dcfa0(
                                        el!(DAT_0081f7e2, iVar8, 0x16).wrapping_add(0x8e) as u32,
                                    );
                                    _sprintf(buf!(), &raw const DAT_005342cc, pcVar3);
                                }
                            } else {
                                puVar17 = (&raw mut DAT_0081f7cc)
                                    .wrapping_add((iVar8 as u32).wrapping_mul(0x16) as usize);
                                if el!(DAT_0081f812, iVar8, 0x16) == 0 {
                                    uVar14 = 1;
                                    pcVar3 = FUN_004dcfa0(
                                        el!(DAT_0081f7e2, iVar8, 0x16).wrapping_add(0x8e) as u32,
                                    );
                                    _sprintf(
                                        buf!(),
                                        &raw const DAT_005342d4,
                                        s_var2 as i32,
                                        local_34.wrapping_add(1),
                                        6i32,
                                        pcVar3,
                                        uVar14,
                                        puVar17,
                                    );
                                } else {
                                    pcVar3 = FUN_004dcfa0(
                                        el!(DAT_0081f7e2, iVar8, 0x16).wrapping_add(0x8e) as u32,
                                    );
                                    _sprintf(
                                        buf!(),
                                        &raw const DAT_005342bc,
                                        s_var2 as i32,
                                        local_34.wrapping_add(1),
                                        1i32,
                                        pcVar3,
                                    );
                                }
                            }
                            uVar16 = _DAT_0081b6fc;
                            if el!(DAT_0081f812, iVar8, 0x16) == 0 {
                                iVar10 = FUN_004cdbc0();
                                let cond;
                                if iVar10 == el!(DAT_0081f7de, iVar8, 0x16) {
                                    cond = true;
                                } else {
                                    uVar16 = DAT_0081bcd0;
                                    cond = DAT_0081b850 == 2;
                                }
                                if cond {
                                    let off =
                                        (((param_1 % 0x18) as u32) & 0xfffffffe).wrapping_mul(2);
                                    uVar16 = (&raw const DAT_0081bc80)
                                        .wrapping_add(off as usize)
                                        .cast::<u32>()
                                        .read_unaligned();
                                }
                            }
                            FUN_004da7c0(0xc, buf!(), iVar12, iVar13, uVar16);
                            FUN_004d51b0(local_10.as_mut_ptr().cast());
                            if (ld!(DAT_0081bfba) == 1 || ld!(DAT_0081bfba) == 2)
                                && DAT_00654d44 != 0
                            {
                                uVar14 = el!(DAT_0081fa7e, *puVar9, 7) as u32;
                            } else {
                                uVar14 = el!(DAT_0081f7e6, iVar8, 0x16) as u32;
                            }
                            _sprintf(buf!(), &raw const DAT_005321b8, uVar14);
                            FUN_004da7c0(0xc, buf!(), 0xe4, iVar13, 0xffff);
                            if DAT_00654d44 == 0 {
                                _sprintf(
                                    buf!(),
                                    &raw const DAT_005321bc,
                                    el!(DAT_0081f7ea, iVar8, 0x16),
                                    el!(DAT_0081f7ee, iVar8, 0x16),
                                );
                                FUN_004da7c0(0xc, buf!(), 0x12a, iVar13, 0xffff);
                                uVar14 = el!(DAT_0081f7fa, iVar8, 0x16) as u32;
                            } else {
                                _sprintf(
                                    buf!(),
                                    &raw const DAT_005321bc,
                                    el!(DAT_0081fa8e, *puVar9, 7),
                                    el!(DAT_0081fa92, *puVar9, 7),
                                );
                                FUN_004da7c0(0xc, buf!(), 0x12a, iVar13, 0xffff);
                                uVar14 = el!(DAT_0081fa96, *puVar9, 7) as u32;
                            }
                            _sprintf(buf!(), &raw const DAT_005321b8, uVar14);
                            iVar10 = iVar13.wrapping_add(0xf);
                            FUN_004da7c0(0xc, buf!(), 0x16b, iVar13, 0xffff);
                            iVar12 = ((0u32.wrapping_sub((DAT_00654d44 == 0) as u32)) & 0xf)
                                .wrapping_add(0x58) as i32;
                        }
                        piVar7 = piVar7.wrapping_add(1);
                        iVar13 = iVar10;
                        if !((piVar7 as usize) < ((&raw mut DAT_00654ca8) as usize)) {
                            break;
                        }
                    }
                }
                if (ld!(DAT_0081bfba) == 1 || ld!(DAT_0081bfba) == 2)
                    && (DAT_00654d44 == 0 && {
                        iVar13 = 0;
                        0 < DAT_0075766c
                    })
                {
                    local_30[0] = (&raw mut DAT_0081e426) as usize as u32;
                    pcVar3 = &raw mut DAT_007438e4;
                    iVar8 = iVar10;
                    loop {
                        iVar10 = iVar8;
                        if *pcVar3 != 0
                            && (pcVar3.wrapping_offset(-0xc).read() as u8 as u32) == *puVar9
                        {
                            iVar4 = 0;
                            piVar7 = &raw mut DAT_0081f7de;
                            loop {
                                if piVar7.read_unaligned() != 0
                                    && piVar7.wrapping_sub(1).read_unaligned() == iVar13
                                {
                                    break;
                                }
                                piVar7 = piVar7.wrapping_add(0x16);
                                iVar4 = iVar4.wrapping_add(1);
                                if !((piVar7 as usize) < ((&raw mut DAT_0081fa9e) as usize)) {
                                    break;
                                }
                            }
                            if iVar4 == 8 {
                                pcVar5 = pcVar3.wrapping_offset(-100);
                                uVar14 = 1;
                                pcVar6 = FUN_004dcfa0(
                                    ld32(local_30[0] as usize as *const u8).wrapping_add(0x8e)
                                        as u32,
                                );
                                _sprintf(
                                    buf!(),
                                    &raw const DAT_00531334,
                                    6i32,
                                    pcVar6,
                                    uVar14,
                                    pcVar5,
                                );
                                iVar10 = iVar8.wrapping_add(0xf);
                                FUN_004da7c0(0xc, buf!(), iVar12, iVar8, 0xffff);
                            }
                        }
                        local_30[0] = local_30[0].wrapping_add(4);
                        pcVar3 = pcVar3.wrapping_add(0x562);
                        iVar13 = iVar13.wrapping_add(1);
                        iVar8 = iVar10;
                        if !(iVar13 < DAT_0075766c as i32) {
                            break;
                        }
                    }
                }
            }
            puVar9 = puVar9.wrapping_add(1);
            local_24 = local_24.wrapping_add(1);
            if !((puVar9 as usize) < ((&raw mut DAT_00654c40) as usize)) {
                break;
            }
        }
        iVar8 = iVar10.wrapping_add(0xf);
        iVar10 = iVar10.wrapping_add(0x1e);
        iVar13 = iVar8;
        uVar16 = DAT_0081bcd0;
        pcVar3 = FUN_004dcfa0(0x210);
        FUN_004da7c0(0xc, pcVar3, iVar12, iVar13, uVar16);
        local_30[0] = (&raw mut DAT_00654cb8) as usize as u32;
        iVar12 = iVar10;
        loop {
            iVar13 = ld32(local_30[0] as usize as *const u8);
            if iVar13 == -1 {
                break;
            }
            let mut go_f = false;
            if iVar13 < 8 {
                iVar4 = FUN_004cdbc0();
                if iVar4 != el!(DAT_0081f7de, iVar13, 0x16)
                    && (el!(DAT_0081e0a6, iVar13, 1) != 0 || el!(DAT_0081e0c6, iVar13, 1) != 0)
                {
                    FUN_004dcfd0(
                        local_20.as_mut_ptr(),
                        0x58,
                        iVar12,
                        0xce,
                        iVar12.wrapping_add(0xe),
                    );
                    FUN_004d5180(local_10.as_mut_ptr().cast());
                    FUN_004d51b0(local_20.as_mut_ptr().cast());
                    if el!(DAT_0081f812, iVar13, 0x16) == 0 {
                        puVar17 = (&raw mut DAT_0081f7cc)
                            .wrapping_add((iVar13 as u32).wrapping_mul(0x16) as usize);
                        uVar14 = 4;
                        pcVar3 =
                            FUN_004dcfa0(el!(DAT_0081f7e2, iVar13, 0x16).wrapping_add(0x8e) as u32);
                        _sprintf(
                            buf!(),
                            &raw const DAT_00531334,
                            6i32,
                            pcVar3,
                            uVar14,
                            puVar17,
                        );
                        uVar16 = 0xffff;
                    } else {
                        puVar17 = (&raw mut DAT_0081f7cc)
                            .wrapping_add((iVar13 as u32).wrapping_mul(0x16) as usize);
                        pcVar3 =
                            FUN_004dcfa0(el!(DAT_0081f7e2, iVar13, 0x16).wrapping_add(0x8e) as u32);
                        _sprintf(buf!(), &raw const DAT_005342cc, pcVar3, puVar17);
                        uVar16 = _DAT_0081b6fc;
                    }
                    FUN_004da7c0(0xc, buf!(), 0x58, iVar12, uVar16);
                    FUN_004d51b0(local_10.as_mut_ptr().cast());
                    uVar14 = el!(DAT_0081e0c6, iVar13, 1) as u32;
                    uVar15 = el!(DAT_0081e0a6, iVar13, 1) as u32;
                    go_f = true;
                }
            } else {
                iVar4 = iVar13.wrapping_sub(8);
                iVar13 = ((iVar4 as u32).wrapping_mul(0x52) as i32).wrapping_add(
                    (((iVar13 as u32).wrapping_mul(9)).wrapping_sub(0x48)).wrapping_mul(0x90)
                        as i32,
                );
                if idx!(
                    DAT_00654bd0,
                    (&raw mut DAT_007438d8)
                        .wrapping_offset(iVar13 as isize)
                        .read() as u8
                ) != 0
                    && (el!(DAT_0081e0e6, iVar4, 1) != 0 || el!(DAT_0081e1a6, iVar4, 1) != 0)
                {
                    iVar13 = ((&raw mut DAT_00743880) as usize as i32).wrapping_add(iVar13);
                    uVar14 = 4;
                    pcVar3 = FUN_004dcfa0(el!(DAT_0081e426, iVar4, 1).wrapping_add(0x8e) as u32);
                    _sprintf(
                        buf!(),
                        &raw const DAT_00531334,
                        6i32,
                        pcVar3,
                        uVar14,
                        iVar13,
                    );
                    FUN_004da7c0(0xc, buf!(), 0x58, iVar12, 0xffff);
                    uVar14 = el!(DAT_0081e1a6, iVar4, 1) as u32;
                    uVar15 = el!(DAT_0081e0e6, iVar4, 1) as u32;
                    go_f = true;
                }
            }
            if go_f {
                _sprintf(buf!(), &raw const DAT_005321a8, uVar15, uVar14);
                FUN_004da7c0(0xc, buf!(), 0xd0, iVar12, 0xffff);
                iVar12 = iVar12.wrapping_add(0xf);
            }
            local_30[0] = local_30[0].wrapping_add(4);
            if !(local_30[0] < ((&raw mut DAT_00654cd8) as usize as u32)) {
                break;
            }
        }
        iVar12 = 0x103;
        piVar7 = &raw mut DAT_00654cb8;
        uVar16 = DAT_0081bcd0;
        pcVar3 = FUN_004dcfa0(0x20f);
        FUN_004da7c0(0xc, pcVar3, iVar12, iVar8, uVar16);
        loop {
            iVar12 = piVar7.read_unaligned();
            if iVar12 == -1 {
                return 1;
            }
            let mut go_f = false;
            if iVar12 < 8 {
                iVar13 = FUN_004cdbc0();
                if iVar13 != el!(DAT_0081f7de, iVar12, 0x16)
                    && (el!(DAT_0081e266, iVar12, 1) != 0 || el!(DAT_0081e286, iVar12, 1) != 0)
                {
                    FUN_004dcfd0(
                        local_20.as_mut_ptr(),
                        0x103,
                        iVar10,
                        0x179,
                        iVar10.wrapping_add(0xe),
                    );
                    FUN_004d5180(local_10.as_mut_ptr().cast());
                    FUN_004d51b0(local_20.as_mut_ptr().cast());
                    puVar17 = (&raw mut DAT_0081f7cc)
                        .wrapping_add((iVar12 as u32).wrapping_mul(0x16) as usize);
                    if el!(DAT_0081f812, iVar12, 0x16) == 0 {
                        uVar14 = 4;
                        pcVar3 =
                            FUN_004dcfa0(el!(DAT_0081f7e2, iVar12, 0x16).wrapping_add(0x8e) as u32);
                        _sprintf(
                            buf!(),
                            &raw const DAT_00531334,
                            6i32,
                            pcVar3,
                            uVar14,
                            puVar17,
                        );
                        uVar16 = 0xffff;
                    } else {
                        pcVar3 =
                            FUN_004dcfa0(el!(DAT_0081f7e2, iVar12, 0x16).wrapping_add(0x8e) as u32);
                        _sprintf(buf!(), &raw const DAT_005342cc, pcVar3);
                        uVar16 = _DAT_0081b6fc;
                    }
                    FUN_004da7c0(0xc, buf!(), 0x103, iVar10, uVar16);
                    FUN_004d51b0(local_10.as_mut_ptr().cast());
                    uVar14 = el!(DAT_0081e286, iVar12, 1) as u32;
                    uVar15 = el!(DAT_0081e266, iVar12, 1) as u32;
                    go_f = true;
                }
            } else {
                iVar13 = iVar12.wrapping_sub(8);
                iVar12 = ((iVar13 as u32).wrapping_mul(0x52) as i32).wrapping_add(
                    (((iVar12 as u32).wrapping_mul(9)).wrapping_sub(0x48)).wrapping_mul(0x90)
                        as i32,
                );
                if idx!(
                    DAT_00654bd0,
                    (&raw mut DAT_007438d8)
                        .wrapping_offset(iVar12 as isize)
                        .read() as u8
                ) != 0
                    && (el!(DAT_0081e2a6, iVar13, 1) != 0 || el!(DAT_0081e366, iVar13, 1) != 0)
                {
                    iVar12 = ((&raw mut DAT_00743880) as usize as i32).wrapping_add(iVar12);
                    uVar14 = 4;
                    pcVar3 = FUN_004dcfa0(el!(DAT_0081e426, iVar13, 1).wrapping_add(0x8e) as u32);
                    _sprintf(
                        buf!(),
                        &raw const DAT_00531334,
                        6i32,
                        pcVar3,
                        uVar14,
                        iVar12,
                    );
                    FUN_004da7c0(0xc, buf!(), 0x103, iVar10, 0xffff);
                    uVar14 = el!(DAT_0081e366, iVar13, 1) as u32;
                    uVar15 = el!(DAT_0081e2a6, iVar13, 1) as u32;
                    go_f = true;
                }
            }
            if go_f {
                _sprintf(buf!(), &raw const DAT_005321a8, uVar15, uVar14);
                FUN_004da7c0(0xc, buf!(), 0x17b, iVar10, 0xffff);
                iVar10 = iVar10.wrapping_add(0xf);
            }
            piVar7 = piVar7.wrapping_add(1);
            if ((&raw mut DAT_00654cd8) as usize) <= (piVar7 as usize) {
                return 1;
            }
        }
    }
}
