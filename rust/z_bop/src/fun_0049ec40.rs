// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005235dc: i32;
    static mut DAT_009d1148: i32;
    static mut DAT_009a7ec0: i32;
    static mut DAT_009a7b44: i32;
    static mut DAT_009a806c: i32;
    static mut DAT_009a7bac: i32;
    static mut DAT_009a8c14: i32;
    static mut DAT_005233d8: i32;
    static mut DAT_00a08374: u16;
    static mut DAT_00a0a2ca: [i16; 1];
    static mut DAT_00a08ba4: i16;
    static mut DAT_005233c8: u8;
    static mut DAT_009d1300: *const core::ffi::c_void;
    static mut DAT_00521548: i16;
    static mut DAT_0052154c: i16;
    static mut DAT_009ec464: i32;
    static mut DAT_009a20ae: u8;
    static mut DAT_005235d0: i16;
    static mut DAT_005235d4: i16;
    static mut DAT_005235d8: i16;
    static mut DAT_009ec5fc: u16;
    static mut DAT_009993fc: i32;
    static mut DAT_009d6930: u16;
    static mut DAT_00622bbc: u16;
    static mut DAT_00622bb0: i16;
    static mut DAT_00622bb8: u32;
    static mut DAT_009a807a: i8;
    static mut DAT_009a6fe4: *const core::ffi::c_void;
    static mut DAT_009e9683: [u8; 1];
    static mut DAT_009ecc34: i32;
    static mut DAT_009eca20: *const core::ffi::c_void;
    static mut s_BJFNREBRJRFNJN_005240a0: [i8; 1];
    static mut s_RJFNJN_005240b0: [i8; 1];
    static mut DAT_00524078: [u8; 1];
    static mut s_00_00_00527748: [i8; 1];
    static mut s_00_00_00_00527750: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0040e0c0(a: i32, b: i32, c: i32, d: i32);
    fn FUN_004a95c0(a: u32);
    fn FUN_004a9520(a: i16, b: i16, c: i16, d: i16);
    fn FUN_004a9620(a: i32);
    fn FUN_004a9630(a: i32);
    fn FUN_004a9640(a: i8);
    fn FUN_004a9590(a: u32) -> i32;
    fn FUN_004ac720(a: i32) -> i32;
    fn FUN_004a9500(a: i16, b: i16);
    fn FUN_0049f330(a: i32) -> i32;
    fn FUN_004278c0(c: *mut u8) -> u64;
    fn FUN_004277f0(a: u16, b: u32, c: u32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_00480530() -> u32;
    fn FUN_004ac740();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0049ec40() -> i16 {
    // SAFETY: all globals, callees and the record table pointed to by
    // DAT_009993fc belong to the game; accessed exactly as the original does.
    unsafe {
        let mut s_stack_10: i16 = 0;
        let s_var5 = DAT_005235dc as i16;
        let s_var7 = DAT_009d1148 as i16;
        let mut s_var6 = s_var7.wrapping_add(2);
        let s_var9 = DAT_009a7ec0 as i16;
        let s_var8 = s_var9.wrapping_sub(2);
        let s_var11 = DAT_009a7b44 as i16;
        let mut s_var10 = s_var11.wrapping_sub(2);

        FUN_0040e0c0(
            DAT_009a806c,
            DAT_009a7bac,
            DAT_009a8c14,
            (DAT_005233d8 as u32).wrapping_mul(DAT_009a7bac as u32) as i32,
        );

        let tbl = &raw const DAT_00a0a2ca as *const i16;
        if tbl.add(DAT_00a08374 as usize).read() != DAT_00a08ba4 {
            FUN_004a95c0(DAT_005233c8 as u32);
            FUN_004a9520(
                DAT_005235dc as i16,
                DAT_009d1148 as i16,
                DAT_009a7ec0 as i16,
                DAT_009a7b44 as i16,
            );
            core::mem::transmute::<*const core::ffi::c_void, unsafe extern "C" fn()>(DAT_009d1300)(
            );
            if DAT_00a08ba4 == -2 {
                if DAT_00521548 == 2 {
                    DAT_00521548 = DAT_0052154c;
                    let u = FUN_00480530();
                    DAT_0052154c = u as i16;
                }
                FUN_0040e0c0(0, 0x140, 200, 0);
                return s_stack_10;
            }
        }

        FUN_004a9520(s_var5.wrapping_add(2), s_var6, s_var8, s_var10);
        FUN_004a9620(1);
        FUN_004a9630(1);
        FUN_004a9640(0);
        FUN_004a95c0(DAT_005233c8 as u32);
        DAT_009ec464 = 1;
        let s_var12 = (DAT_009a20ae as i16).wrapping_add(2);
        FUN_004a9590(0x43);
        let i_var22_0: i32 = DAT_005235d0 as i32;
        DAT_009993fc = FUN_004ac720(DAT_009ec5fc as i32);
        FUN_004ac740();

        if tbl.add(DAT_00a08374 as usize).read() == DAT_00a08ba4 {
            if DAT_00521548 == 2 {
                let mut set = false;
                match DAT_009d6930 {
                    0xa6 => {
                        if DAT_00622bbc != 0 {
                            DAT_00622bbc = DAT_00622bbc.wrapping_sub(1);
                            set = true;
                        }
                    }
                    0xa7 => {
                        let mut i_var19: i32 = 300;
                        if DAT_005235d8 == 0 {
                            i_var19 = i_var22_0;
                        }
                        if (DAT_00622bbc as i32)
                            <= ((s_var6 as i32 - s_var10 as i32) / s_var12 as i32)
                                .wrapping_add(i_var19 as i16 as i32)
                        {
                            DAT_00622bbc = DAT_00622bbc.wrapping_add(1);
                            set = true;
                        }
                    }
                    0xac => {
                        if 4 < DAT_00622bbc {
                            DAT_00622bbc = DAT_00622bbc.wrapping_sub(4);
                            set = true;
                        }
                    }
                    0xad => {
                        let mut i_var19: i32 = 300;
                        if DAT_005235d8 == 0 {
                            i_var19 = i_var22_0;
                        }
                        if (DAT_00622bbc as i32)
                            <= (i_var19 as i16 as i32)
                                .wrapping_add((s_var6 as i32 - s_var10 as i32) / s_var12 as i32)
                                .wrapping_sub(4)
                        {
                            DAT_00622bbc = DAT_00622bbc.wrapping_add(4);
                            set = true;
                        }
                    }
                    _ => {}
                }
                if set {
                    DAT_00622bb0 = 1;
                }
            }
            if (DAT_005235d4 as i32 as u32) != DAT_00622bb8 {
                DAT_00622bb0 = 1;
            }
        } else {
            DAT_00622bbc = 0;
            DAT_00622bb8 = 0;
            DAT_00622bb0 = 1;
            core::mem::transmute::<*const core::ffi::c_void, unsafe extern "C" fn()>(DAT_009d1300)(
            );
        }

        s_var10 = (((s_var10 as i32 - s_var6 as i32) / s_var12 as i32) as i16)
            .wrapping_add(DAT_00622bbc as i16);
        s_stack_10 = s_var6;

        if DAT_00622bb0 != 0 {
            core::mem::transmute::<*const core::ffi::c_void, unsafe extern "C" fn()>(DAT_009d1300)(
            );
            let mut s_stack_a: i16 = 0;
            if -1 < s_var10 {
                let mut b_var17: u8 = s_var6 as u8;
                loop {
                    s_stack_10 = s_var6;
                    let i_var19 = s_stack_a as i32;
                    if (DAT_00622bbc as i32) <= i_var19 && s_stack_a < 300 {
                        FUN_004a9500(s_var5.wrapping_add(2), s_stack_10);
                        let i_var14 = FUN_0049f330(i_var19);
                        if -1 < i_var14 {
                            let i_var22 = (i_var14 as u32).wrapping_mul(0x54);
                            let base = DAT_009993fc as u32;
                            let pb1 = base.wrapping_add(0xe).wrapping_add(i_var22) as *mut u8;
                            let mut b2 = pb1.read();
                            let mut pb20: *mut u8;
                            if b2 < 9 {
                                pb20 = pb1.add(1);
                                let t1 = (&raw const s_BJFNREBRJRFNJN_005240a0 as *const u8)
                                    .add(b2 as usize)
                                    .read();
                                FUN_004a9590(t1 as u32);
                                if b2 == 1 {
                                    b2 = pb20.read();
                                    if 0x2f < b2 && b2 < 0x34 {
                                        pb20 = pb1.add(2);
                                        let t2 = (&raw const DAT_00524078 as *const u8)
                                            .add(b2 as usize)
                                            .read();
                                        FUN_004a9590(t2 as u32);
                                    }
                                } else if b2 == 2 {
                                    let idx = (base.wrapping_add(10).wrapping_add(i_var22)
                                        as *const u16)
                                        .read_unaligned();
                                    let t3 = (&raw const s_RJFNJN_005240b0 as *const u8)
                                        .add(idx as usize)
                                        .read();
                                    FUN_004a9590(t3 as u32);
                                }
                            } else {
                                pb20 = pb1;
                                FUN_004a9590(0x42);
                            }
                            b2 = pb20.read();
                            while b2 != 0 {
                                b2 = pb20.read();
                                if b2 == 0x5b {
                                    if DAT_009a807a == -0x2c {
                                        DAT_009a807a = -0x2d;
                                    } else {
                                        DAT_009a807a = DAT_009a807a.wrapping_add(1);
                                    }
                                } else if b2 == 0x5d {
                                    if DAT_009a807a == -0x2d {
                                        DAT_009a807a = -0x2c;
                                    } else {
                                        DAT_009a807a = DAT_009a807a.wrapping_sub(1);
                                    }
                                } else {
                                    core::mem::transmute::<
                                        *const core::ffi::c_void,
                                        unsafe extern "C" fn(i32),
                                    >(DAT_009a6fe4)(b2 as i32);
                                    b_var17 = pb20.read();
                                }
                                pb20 = pb20.add(1);
                                b2 = pb20.read();
                            }
                            let putc = |c: i32| {
                                core::mem::transmute::<
                                    *const core::ffi::c_void,
                                    unsafe extern "C" fn(i32),
                                >(DAT_009a6fe4)(c)
                            };
                            if b_var17 != 0x3f
                                && b_var17 != 0x21
                                && b_var17 != 0x3a
                                && b_var17 != 0x20
                            {
                                putc(0x2e);
                            }
                            FUN_004a95c0(DAT_005233c8 as u32);
                            putc(10);
                            FUN_004a9590(0x42);
                            FUN_004a95c0(DAT_005233c8 as u32);
                            let rec8 = base.wrapping_add(8).wrapping_add(i_var22) as *const u8;
                            let u_var13: u32;
                            if rec8.read() == 0 {
                                let u24 = FUN_004278c0(&raw mut s_00_00_00527748 as *mut u8);
                                let v = (s_var8 as i32 as u32).wrapping_sub((u24 as u32) & 0xffff)
                                    as i32;
                                FUN_004a9500(v as i16, s_stack_10);
                                u_var13 = 1;
                            } else {
                                let u24 = FUN_004278c0(&raw mut s_00_00_00_00527750 as *mut u8);
                                let v = (s_var8 as i32 as u32).wrapping_sub((u24 as u32) & 0xffff)
                                    as i32;
                                FUN_004a9500(v as i16, s_stack_10);
                                FUN_004277f0(rec8.read() as u16, 2, 1);
                                putc(0x3a);
                                u_var13 = 2;
                            }
                            let rec7 = base.wrapping_add(7).wrapping_add(i_var22) as *const u8;
                            FUN_004277f0(rec7.read() as u16, 2, u_var13);
                            putc(0x3a);
                            let rec6 = base.wrapping_add(6).wrapping_add(i_var22) as *const u8;
                            FUN_004277f0(rec6.read() as u16, 2, 2);
                            putc(0x20);
                            s_stack_10 = s_stack_10.wrapping_add(s_var12);
                        }
                    }
                    s_stack_a = s_stack_a.wrapping_add(1);
                    s_var6 = s_stack_10;
                    if !(s_stack_a <= s_var10) {
                        break;
                    }
                }
            }
        }

        FUN_004a9620(0);
        DAT_00622bb0 = 0;
        let arr = &raw const DAT_009e9683 as *const u8;
        if arr
            .add((DAT_009ecc34 as u32).wrapping_mul(0x5bd) as usize)
            .read()
            == 0
            && DAT_00521548 == -1
        {
            DAT_00521548 = 2;
        }

        let mut do_call = true;
        if DAT_00521548 == 2 {
            FUN_004a95c0(0x46);
            FUN_004a9520(s_var5, s_var7, s_var9, s_var11);
        } else if DAT_0052154c != 2 {
            do_call = false;
        } else {
            FUN_004a95c0(DAT_005233c8 as u32);
            FUN_004a9520(s_var5, s_var7, s_var9, s_var11);
        }
        if do_call {
            core::mem::transmute::<
                *const core::ffi::c_void,
                unsafe extern "C" fn(i32, i32, i32, i32, i32),
            >(DAT_009eca20)(
                s_var5 as i32,
                s_var7 as i32,
                s_var9 as i32,
                s_var11 as i32,
                1,
            );
        }

        FUN_0040e0c0(0, 0x140, 200, 0);
        DAT_00622bb8 = DAT_005235d4 as i32 as u32;
        s_stack_10
    }
}
