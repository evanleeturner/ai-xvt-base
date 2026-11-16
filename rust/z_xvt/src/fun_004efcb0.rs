// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0081b850: i32;
    static mut DAT_00757a20: i32;
    static mut DAT_00757a28: i32;
    static mut DAT_007579b0: [i32; 8];
    static mut DAT_007579d0: [i32; 0];
    static mut DAT_007579f8: i32;
    static mut DAT_00757a00: [i32; 0];
    static mut DAT_0075786c: [i32; 0];
    static mut DAT_00757870: [i32; 0];
    static mut DAT_0075788c: i32;
    static mut DAT_00757cee: [i8; 0];
    static mut DAT_0081b740: [i8; 0];
    static mut DAT_0081bc80: [i8; 0];
    static mut DAT_0081bcd0: u32;
    static mut _DAT_0081b6fc: u32;
    static mut _DAT_00758620: u32;
    static mut DAT_00758624: i32;
    static mut DAT_00758628: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dcfd0(p: *mut i32, a: i32, b: i32, c: i32, d: i32);
    fn FUN_004dcfa0(a: i32) -> *mut i8;
    fn FUN_004daac0(a: i32, b: *mut i8, c: *mut i32, d: i32, e: i32, f: u32);
    fn FUN_004cde00() -> i32;
    fn FUN_004d5180(p: *mut core::ffi::c_void);
    fn FUN_004d51b0(p: *mut core::ffi::c_void);
    fn _sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_004cdbc0() -> i32;
    fn FUN_004cc810() -> i32;
    fn FUN_004db630(a: i32) -> u8;
    fn FUN_004ddc70(a: *mut i32, b: *mut i32);
    fn FUN_004db530() -> u8;
    fn FUN_004db550() -> u8;
    fn FUN_004dd7d0(a: *mut i32, b: i32, c: i32) -> i32;
    fn FUN_004db510(a: i32);
    fn FUN_004cdbb0() -> i32;
    fn FUN_004cb830(a: i32, b: *mut u32, c: i32);
    fn FUN_004dd030(a: *mut i32, b: i32, c: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004efcb0(param_1: i32) -> u32 {
    // SAFETY: all globals and callees are the game's own; pointers walk the
    // game's arrays exactly as the original does.
    unsafe {
        let mut local_28: [i32; 4] = [0; 4];
        let mut local_10: [u32; 4] = [0; 4];
        let mut local_14: i32 = 0;
        let mut local_18: i32 = 0;
        let l28 = local_28.as_mut_ptr();
        let l10 = local_10.as_mut_ptr() as *mut core::ffi::c_void;

        let arr_b0 = (&raw mut DAT_007579b0).cast::<i32>();
        let end_d0 = (&raw mut DAT_007579d0).cast::<i32>();
        let arr_a00 = (&raw mut DAT_00757a00).cast::<i32>();
        let arr_86c = (&raw mut DAT_0075786c).cast::<i32>();
        let arr_870 = (&raw mut DAT_00757870).cast::<i32>();
        let buf_740 = (&raw mut DAT_0081b740).cast::<i8>();
        let buf_c80 = (&raw mut DAT_0081bc80).cast::<u8>();

        let mut iVar12: i32 = 1;
        let mut iVar11: i32 = 0;
        let pc = FUN_004dcfa0(0x1cc);
        let mut local_2c: i32 = 1;
        FUN_004daac0(0xf, pc, l28, iVar11, iVar12, 0xffff);
        if DAT_0081b850 != 2 {
            local_2c = FUN_004cde00();
        }
        let mut local_38: i32 = 0;
        FUN_004dcfd0(l28, 0x58, 0x86, 0x102, 0x94);
        if 0 < local_2c {
            let mut local_40: *mut i8 = (&raw mut DAT_00757cee).cast::<i8>();
            iVar11 = DAT_00757a20;
            loop {
                let mut iVar7: i32 = 0;
                let mut piVar5 = arr_b0;
                iVar12 = local_40.cast::<i32>().read_unaligned();
                loop {
                    if *piVar5 == iVar12 {
                        break;
                    }
                    piVar5 = piVar5.add(1);
                    iVar7 = iVar7.wrapping_add(1);
                    if !(piVar5 < end_d0) {
                        break;
                    }
                }
                if iVar7 == 8 && DAT_007579f8 != iVar12 {
                    iVar7 = 0;
                    if 0 < iVar11 {
                        let mut p = arr_a00;
                        loop {
                            if *p == iVar12 {
                                break;
                            }
                            p = p.add(1);
                            iVar7 = iVar7.wrapping_add(1);
                            if !(iVar7 < iVar11) {
                                break;
                            }
                        }
                    }
                    FUN_004d5180(l10);
                    FUN_004d51b0(l28 as *mut core::ffi::c_void);
                    let uVar14: u32;
                    if iVar7 == DAT_00757a20 {
                        iVar11 = (local_40 as i32).wrapping_sub(0xe);
                        let uVar13: i32 = 1;
                        let t1 = local_40.byte_add(4).cast::<i32>().read_unaligned();
                        let pcVar4 = FUN_004dcfa0((t1 as u32).wrapping_add(0x8e) as i32);
                        _sprintf(buf_740, c"%c%s %c%s".as_ptr(), 6i32, pcVar4, uVar13, iVar11);
                        iVar11 = FUN_004cdbc0();
                        let t0 = local_40.cast::<i32>().read_unaligned();
                        let mut u = 0u32;
                        let mut take = iVar11 == t0;
                        if !take {
                            u = DAT_0081bcd0;
                            take = DAT_0081b850 == 2;
                        }
                        if take {
                            let off = (((param_1 % 0x18) as u32) & 0xffff_fffe).wrapping_mul(2);
                            u = buf_c80
                                .byte_add(off as usize)
                                .cast::<u32>()
                                .read_unaligned();
                        }
                        uVar14 = u;
                    } else {
                        iVar11 = (local_40 as i32).wrapping_sub(0xe);
                        let t1 = local_40.byte_add(4).cast::<i32>().read_unaligned();
                        let pcVar4 = FUN_004dcfa0((t1 as u32).wrapping_add(0x8e) as i32);
                        _sprintf(buf_740, c"%s %s".as_ptr(), pcVar4, iVar11);
                        uVar14 = _DAT_0081b6fc;
                    }
                    FUN_004daac0(0xc, buf_740, l28, 0, 1, uVar14);
                    local_38 = local_38.wrapping_add(1);
                    FUN_004d51b0(l10);
                    iVar11 = FUN_004cc810();
                    if (iVar11 != 0 || DAT_0081b850 == 2) && FUN_004db630(2) == 0 {
                        FUN_004ddc70(&raw mut local_14, &raw mut local_18);
                        let mut ok = FUN_004db530() != 0;
                        if !ok {
                            ok = FUN_004db550() != 0;
                        }
                        if ok && FUN_004dd7d0(l28, local_14, local_18) != 0 {
                            if DAT_0081b850 == 2 {
                                DAT_007579f8 = local_40.cast::<i32>().read_unaligned();
                                FUN_004db510(2);
                            } else {
                                iVar11 = 0;
                                if 0 < DAT_00757a20 {
                                    let mut p = arr_a00;
                                    let t0 = local_40.cast::<i32>().read_unaligned();
                                    loop {
                                        if *p == t0 {
                                            break;
                                        }
                                        p = p.add(1);
                                        iVar11 = iVar11.wrapping_add(1);
                                        if !(iVar11 < DAT_00757a20) {
                                            break;
                                        }
                                    }
                                }
                                if iVar11 == DAT_00757a20 {
                                    DAT_00757a20 = DAT_00757a20.wrapping_add(1);
                                    _DAT_00758620 = 0x68;
                                    DAT_00758624 = local_40.cast::<i32>().read_unaligned();
                                    *arr_a00.offset(iVar11 as isize) = DAT_00758624;
                                    DAT_00758628 = FUN_004cdbb0();
                                    FUN_004cb830(0, &raw mut _DAT_00758620, 0xc);
                                    DAT_007579f8 = local_40.cast::<i32>().read_unaligned();
                                    FUN_004db510(2);
                                    _DAT_00758620 = 0x4f;
                                    DAT_00758624 = DAT_007579f8;
                                    DAT_00758628 = 10;
                                    FUN_004cb830(0, &raw mut _DAT_00758620, 0xc);
                                    let mut local_30 = DAT_00757a28;
                                    iVar11 = DAT_007579f8;
                                    let mut piVar5 = arr_b0;
                                    loop {
                                        if *piVar5 == iVar11 {
                                            *piVar5 = 0;
                                        }
                                        piVar5 = piVar5.add(1);
                                        if !(piVar5 < end_d0) {
                                            break;
                                        }
                                    }
                                    if 0 < local_30 {
                                        let mut puVar10 = &raw mut DAT_0075788c;
                                        iVar12 = 0;
                                        let mut local_3c = end_d0;
                                        loop {
                                            let mut iVar8: i32 = 0;
                                            let iVar7b = *local_3c;
                                            if 0 < iVar7b {
                                                loop {
                                                    if *arr_870
                                                        .offset(iVar12.wrapping_add(iVar8) as isize)
                                                        == iVar11
                                                    {
                                                        let iVar1 = iVar8.wrapping_add(1);
                                                        if iVar1 < 8 {
                                                            let mut iVar6 = 8 - iVar1;
                                                            let mut puVar9 = arr_86c
                                                                .offset(iVar12.wrapping_add(iVar1)
                                                                    as isize);
                                                            loop {
                                                                iVar6 = iVar6.wrapping_sub(1);
                                                                *puVar9 = *puVar9.add(1);
                                                                puVar9 = puVar9.add(1);
                                                                if iVar6 == 0 {
                                                                    break;
                                                                }
                                                            }
                                                        }
                                                        *puVar10 = 0;
                                                    }
                                                    iVar8 = iVar8.wrapping_add(1);
                                                    if !(iVar8 < iVar7b) {
                                                        break;
                                                    }
                                                }
                                            }
                                            puVar10 = puVar10.add(8);
                                            iVar12 = iVar12.wrapping_add(8);
                                            local_3c = local_3c.add(1);
                                            local_30 = local_30.wrapping_sub(1);
                                            if local_30 == 0 {
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if local_38 == 4 {
                        FUN_004dcfd0(l28, 0x103, 0x86, 0x1ae, 0x94);
                        iVar11 = DAT_00757a20;
                    } else {
                        FUN_004dd030(l28, 0, 0xf);
                        iVar11 = DAT_00757a20;
                    }
                }
                local_40 = local_40.byte_add(0x2a);
                local_2c = local_2c.wrapping_sub(1);
                if local_2c == 0 {
                    break;
                }
            }
        }
        1
    }
}
