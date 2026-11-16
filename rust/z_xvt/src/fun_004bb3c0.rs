// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0064ef64: i32;
    static mut DAT_0081b850: i32;
    static mut DAT_0064ef60: i32;
    static mut DAT_0064ef58: i32;
    static mut DAT_0064ef5c: i32;
    static mut DAT_0085e6c0: i32;

    static mut DAT_0085e6d0: u8;
    static mut DAT_0085e6d1: u8;
    static mut DAT_0085e6d2: u8;
    static mut DAT_0085e6d3: u8;
    static mut DAT_0085e6d4: u8;
    static mut DAT_0085e6d5: u8;
    static mut DAT_0085e6d6: u8;
    static mut DAT_0085e6d7: u8;
    static mut DAT_0085e6d8: u8;
    static mut DAT_0085e6d9: u8;
    static mut DAT_0085e6da: u8;
    static mut DAT_0085e6db: u8;
    static mut DAT_0085e6dc: u8;
    static mut DAT_0085e6dd: u8;
    static mut DAT_0085e6de: u8;
    static mut DAT_0085e6df: u8;
    static mut DAT_0085e6e0: u8;
    static mut DAT_0085e6e1: u8;
    static mut DAT_0085e6e2: u8;
    static mut DAT_0085e6e3: u8;
    static mut DAT_0085e6e4: u8;
    static mut DAT_0085e6e5: u8;
    static mut DAT_0085e6e6: u8;
    static mut DAT_0085e6e7: u8;
    static mut DAT_0085e6e8: u8;
    static mut DAT_0085e6e9: u8;
    static mut DAT_0085e6ea: u8;
    static mut DAT_0085e6eb: u8;
    static mut DAT_0085e6ec: u8;
    static mut DAT_0085e6ed: u8;
    static mut DAT_0085e6ee: u8;
    static mut DAT_0085e6ef: u8;
    static mut DAT_0085e6f0: u8;
    static mut DAT_0085e78f: u8;
    static mut DAT_0085e790: u8;
    static mut DAT_0085e791: u8;
    static mut DAT_0085e792: u8;
    static mut DAT_0085e793: u8;
    static mut DAT_0085e794: u8;
    static mut DAT_0085e795: u8;
    static mut DAT_0085e796: u8;
    static mut DAT_0085e797: u8;
    static mut DAT_0085e798: u8;
    static mut DAT_0085e799: u8;
    static mut DAT_0085e79a: u8;
    static mut DAT_0085e79b: u8;
    static mut DAT_0085e79c: u8;
    static mut DAT_0085e79d: u8;
    static mut DAT_0085e79e: u8;
    static mut DAT_0085e79f: i8;
    static mut DAT_0085e7c5: u8;
    static mut DAT_0085e7a0: i8;
    static mut _DAT_0085e7a0: i32;
    static mut DAT_0085e7a4: i32;
    static mut DAT_0085e7a8: i32;
    static mut DAT_0085e7ac: i32;
    static mut DAT_0085e7b0: i32;
    static mut DAT_00859c40: i8;

    static mut DAT_00531990: i8;
    static mut DAT_00531978: i8;
    static mut DAT_00531984: i8;
    static mut DAT_0053196c: i8;
    static mut DAT_00531960: i8;
    static mut DAT_00531954: i8;
    static mut DAT_00531948: i8;
    static mut DAT_0053193c: i8;
    static mut DAT_00531930: i8;
    static mut DAT_00531924: i8;
    static mut DAT_00531918: i8;
    static mut DAT_0053190c: i8;
    static mut DAT_00531900: i8;
    static mut DAT_00531378: i8;
    static mut DAT_00531358: i8;
    static mut DAT_00531350: i8;
    static mut DAT_00531340: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dcfd0(rect: *mut i32, a: i32, b: i32, c: i32, d: i32);
    fn FUN_004ddc70(a: *mut i32, b: *mut i32);
    fn FUN_004dd7d0(rect: *mut i32, a: i32, b: i32) -> i32;
    fn FUN_004db530() -> i32;
    fn FUN_004db550() -> i32;
    fn FUN_004db570() -> i32;
    fn FUN_004db590() -> i32;
    fn FUN_004d9ea0(a: *mut i32);
    fn FUN_004dcfa0(id: i32) -> *mut i8;
    fn FUN_004d9a80(
        rect: *mut i32,
        s1: *mut i8,
        s2: *mut i8,
        s3: *mut i8,
        a: i32,
        b: u32,
        c: i32,
        s4: *mut i8,
    ) -> i32;
    fn FUN_004dbf00() -> i32;
    fn FUN_004d0070(a: i32);
    fn FUN_004cfbc0(a: i32, b: i32, c: i32);
    fn FUN_004df3a0(a: i32) -> i32;
    fn FUN_004d9d20(rect: *mut i32, s1: *mut i8, s2: *mut i8, a: i32);
    fn FUN_004d52b0();
    fn FUN_004d1750(s: *mut i8, a: i32, b: i32);
    fn FUN_004d0e60(s: *mut i8, a: i32, b: i32);
    fn FUN_004d0580(s: *mut i8, a: i32, b: i32);
    fn FUN_004df760(a: i32, b: i32, c: i32) -> u32;
    fn FUN_004dd130(rect: *mut i32, a: i32, b: i32, c: u32);
    fn FUN_004d5350(a: i32);
    fn FUN_004dd030(rect: *mut i32, a: i32, b: i32);
    fn FUN_004dbea0();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004bb3c0() -> u32 {
    let mut local_34: [i32; 4] = [0; 4];
    let mut local_24: [i32; 8] = [0; 8];
    let mut local_4: i32 = 0;
    // SAFETY: all globals and callees are the game's own; locals are passed
    // by pointer exactly as the original does.
    unsafe {
        let r = local_34.as_mut_ptr();
        let l24 = local_24.as_mut_ptr();
        local_24[0] = 1;
        local_24[1] = 1;
        local_24[2] = 1;
        local_24[3] = 1;
        local_24[4] = 1;
        if DAT_0064ef64 == 0 {
            if DAT_0081b850 == 4 || DAT_0081b850 == 3 {
                local_24[5] = 0;
            } else {
                local_24[5] = 1;
            }
        } else {
            local_24[5] = 1;
        }
        local_24[6] = 0;
        l24.offset(DAT_0064ef64 as isize).write(2);
        FUN_004dcfd0(r, 0x16, 0x132, 0x2a, 0x14a);
        FUN_004ddc70(&raw mut local_4, l24.add(7));
        if local_24[5] != 0 && FUN_004dd7d0(r, local_4, local_24[7]) != 0 {
            if FUN_004db530() != 0
                || (FUN_004db550() != 0 || FUN_004db570() != 0)
                || FUN_004db590() != 0
            {
                local_24[5] = 2;
            }
        }
        FUN_004d9ea0(l24);
        if local_24[5] != 0 {
            FUN_004dcfd0(r, 0x16, 0x132, 0x2a, 0x14a);
            let pcvar4 = FUN_004dcfa0(0x197);
            let mut ivar3 = FUN_004d9a80(
                r,
                &raw mut DAT_00531978,
                &raw mut DAT_00531984,
                pcvar4,
                0xc,
                0,
                0x10,
                &raw mut DAT_00531990,
            );
            if ivar3 != 0 {
                FUN_004dcfa0(0x13);
                FUN_004dcfa0(0x1fd);
                FUN_004dcfa0(0x294);
                FUN_004dcfa0(0x293);
                FUN_004dcfa0(0x292);
                ivar3 = FUN_004dbf00();
                let cvar1 = DAT_0085e79f;
                if ivar3 != 0 {
                    match DAT_0064ef64 {
                        0 => {
                            DAT_0085e790 = 8;
                            DAT_0085e6f0 = 0;
                            DAT_0085e78f = 0;
                        }
                        1 => {
                            DAT_0085e6d0 = 1;
                            DAT_0085e6d2 = 2;
                            DAT_0085e6d4 = 1;
                            DAT_0085e6d6 = 1;
                            DAT_0085e6d8 = 1;
                            DAT_0085e6da = 1;
                            DAT_0085e6dc = 1;
                            DAT_0085e6de = 2;
                            DAT_0085e6e0 = 10;
                            DAT_0085e6e2 = 10;
                            DAT_0085e6e4 = 2;
                            DAT_0085e6e6 = 2;
                            DAT_0085e6ea = 2;
                            DAT_0085e6ec = 1;
                            DAT_0085e6ee = 1;
                            DAT_0085e6e8 = 0;
                        }
                        2 => {
                            DAT_0085e6d1 = 1;
                            DAT_0085e6d3 = 2;
                            DAT_0085e6d5 = 1;
                            DAT_0085e6d7 = 1;
                            DAT_0085e6d9 = 1;
                            DAT_0085e6db = 1;
                            DAT_0085e6dd = 1;
                            DAT_0085e6df = 2;
                            DAT_0085e6e1 = 10;
                            DAT_0085e6e3 = 10;
                            DAT_0085e6e5 = 2;
                            DAT_0085e6e7 = 2;
                            DAT_0085e6eb = 2;
                            DAT_0085e6ed = 1;
                            DAT_0085e6ef = 1;
                            DAT_0085e6e9 = 0;
                        }
                        3 => {
                            DAT_0085e791 = 1;
                            DAT_0085e792 = 1;
                            DAT_0085e793 = 1;
                            DAT_0085e7c5 = 1;
                            DAT_0085e794 = 2;
                            DAT_0085e795 = 2;
                            DAT_0085e796 = 1;
                            DAT_0085e797 = 1;
                            DAT_0085e798 = 1;
                            DAT_0085e79f = 1;
                            DAT_0085e799 = 9;
                            DAT_0085e79a = 9;
                            DAT_0085e79b = 9;
                            DAT_0085e79c = 9;
                            DAT_0085e79d = 9;
                            DAT_0085e79e = 5;
                            if cvar1 != 1 {
                                FUN_004d0070(0x8e38);
                                FUN_004cfbc0(7, 0, 0);
                            }
                        }
                        4 => {
                            ivar3 = 0;
                            _DAT_0085e7a0 = 0;
                            DAT_0085e7a4 = 0;
                            DAT_0085e7a8 = 0;
                            DAT_0085e7ac = 0;
                            DAT_0085e7b0 = 0;
                            let uvar2 = FUN_004df3a0(0);
                            if uvar2 != 0 && -1 < uvar2 {
                                loop {
                                    if 0xf < ivar3 {
                                        break;
                                    }
                                    let p = (&raw mut DAT_0085e7a0).offset(ivar3 as isize);
                                    match ivar3 {
                                        0 => p.write(0x98u8 as i8),
                                        1 => p.write(0x99u8 as i8),
                                        2 => p.write(0x72),
                                        3 => p.write(0x2e),
                                        4 => p.write(0x65),
                                        5 => p.write(0x69),
                                        6 => p.write(0x5b),
                                        7 => p.write(8),
                                        8 => p.write(0xd),
                                        9 => p.write(0x5d),
                                        _ => {}
                                    }
                                    ivar3 = ivar3.wrapping_add(1);
                                    if !(ivar3 < uvar2) {
                                        break;
                                    }
                                }
                            }
                            ivar3 = 0;
                            if 0 < DAT_0085e6c0 {
                                let mut pc = &raw mut DAT_00859c40;
                                loop {
                                    if pc.read()
                                        == (&raw mut DAT_0085e7a0)
                                            .offset(DAT_0064ef58 as isize)
                                            .read()
                                    {
                                        DAT_0064ef5c = ivar3;
                                    }
                                    pc = pc.byte_add(0x95);
                                    ivar3 = ivar3.wrapping_add(1);
                                    if !(ivar3 < DAT_0085e6c0) {
                                        break;
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        FUN_004dcfd0(r, 0x16, 0xe2, 0x2a, 0xfa);
        if DAT_0064ef64 == 4 {
            let pcvar4 = FUN_004dcfa0(0x192);
            FUN_004d9d20(r, &raw mut DAT_0053196c, pcvar4, 0xc);
        } else {
            let pcvar4 = FUN_004dcfa0(0x192);
            let ivar3 = FUN_004d9a80(
                r,
                &raw mut DAT_00531960,
                &raw mut DAT_00531960,
                pcvar4,
                0xc,
                0,
                0xf,
                &raw mut DAT_00531990,
            );
            if ivar3 != 0 {
                DAT_0064ef60 = 1;
                DAT_0064ef64 = 4;
                FUN_004d52b0();
                FUN_004d1750(&raw mut DAT_00531378, 0, 0);
                FUN_004d0e60(&raw mut DAT_00531358, 0, 0);
                FUN_004d0e60(&raw mut DAT_00531350, 0, 0);
                FUN_004dcfd0(r, 0x54, 0x6b, 0x25c, 0x1b1);
                FUN_004d0580(&raw mut DAT_00531340, 0, 0);
                FUN_004dcfd0(r, 0x14a, 0x6f, 0x24e, 0x19f);
                let u5 = FUN_004df760(0, 0, 0x40);
                FUN_004dd130(r, 0, 0, u5);
                FUN_004d5350(1);
            }
        }
        FUN_004dd030(r, 0, -0x1c);
        if DAT_0064ef64 == 3 {
            let pcvar4 = FUN_004dcfa0(0x129);
            FUN_004d9d20(r, &raw mut DAT_00531954, pcvar4, 0xc);
        } else {
            let pcvar4 = FUN_004dcfa0(0x129);
            let ivar3 = FUN_004d9a80(
                r,
                &raw mut DAT_00531948,
                &raw mut DAT_00531948,
                pcvar4,
                0xc,
                0,
                0xe,
                &raw mut DAT_00531990,
            );
            if ivar3 != 0 {
                DAT_0064ef60 = 1;
                DAT_0064ef64 = 3;
                FUN_004d52b0();
                FUN_004d1750(&raw mut DAT_00531378, 0, 0);
                FUN_004d0e60(&raw mut DAT_00531358, 0, 0);
                FUN_004d0e60(&raw mut DAT_00531350, 0, 0);
                FUN_004dcfd0(r, 0x54, 0x6b, 0x25c, 0x1b1);
                FUN_004d0580(&raw mut DAT_00531340, 0, 0);
                FUN_004d5350(1);
            }
        }
        FUN_004dd030(r, 0, -0x1c);
        if DAT_0064ef64 == 2 {
            let pcvar4 = FUN_004dcfa0(0x273);
            FUN_004d9d20(r, &raw mut DAT_0053193c, pcvar4, 0xc);
        } else {
            let pcvar4 = FUN_004dcfa0(0x273);
            let ivar3 = FUN_004d9a80(
                r,
                &raw mut DAT_00531930,
                &raw mut DAT_00531930,
                pcvar4,
                0xc,
                0,
                0xd,
                &raw mut DAT_00531990,
            );
            if ivar3 != 0 {
                DAT_0064ef60 = 1;
                DAT_0064ef64 = 2;
                FUN_004d52b0();
                FUN_004d1750(&raw mut DAT_00531378, 0, 0);
                FUN_004d0e60(&raw mut DAT_00531358, 0, 0);
                FUN_004d0e60(&raw mut DAT_00531350, 0, 0);
                FUN_004dcfd0(r, 0x54, 0x6b, 0x25c, 0x1b1);
                FUN_004d0580(&raw mut DAT_00531340, 0, 0);
                FUN_004d5350(1);
            }
        }
        FUN_004dd030(r, 0, -0x1c);
        if DAT_0064ef64 == 1 {
            let pcvar4 = FUN_004dcfa0(0xce);
            FUN_004d9d20(r, &raw mut DAT_00531924, pcvar4, 0xc);
        } else {
            let pcvar4 = FUN_004dcfa0(0xce);
            let ivar3 = FUN_004d9a80(
                r,
                &raw mut DAT_00531918,
                &raw mut DAT_00531918,
                pcvar4,
                0xc,
                0,
                0xc,
                &raw mut DAT_00531990,
            );
            if ivar3 != 0 {
                DAT_0064ef60 = 1;
                DAT_0064ef64 = 1;
                FUN_004d52b0();
                FUN_004d1750(&raw mut DAT_00531378, 0, 0);
                FUN_004d0e60(&raw mut DAT_00531358, 0, 0);
                FUN_004d0e60(&raw mut DAT_00531350, 0, 0);
                FUN_004dcfd0(r, 0x54, 0x6b, 0x25c, 0x1b1);
                FUN_004d0580(&raw mut DAT_00531340, 0, 0);
                FUN_004d5350(1);
            }
        }
        FUN_004dd030(r, 0, -0x1c);
        if DAT_0064ef64 != 0 {
            let pcvar4 = FUN_004dcfa0(0x123);
            let ivar3 = FUN_004d9a80(
                r,
                &raw mut DAT_00531900,
                &raw mut DAT_00531900,
                pcvar4,
                0xc,
                0,
                0xb,
                &raw mut DAT_00531990,
            );
            if ivar3 != 0 {
                FUN_004dbea0();
                DAT_0064ef60 = 1;
                DAT_0064ef64 = 0;
                FUN_004d52b0();
                FUN_004d1750(&raw mut DAT_00531378, 0, 0);
                FUN_004d0e60(&raw mut DAT_00531358, 0, 0);
                FUN_004d0e60(&raw mut DAT_00531350, 0, 0);
                FUN_004dcfd0(r, 0x54, 0x6b, 0x25c, 0x1b1);
                FUN_004d0580(&raw mut DAT_00531340, 0, 0);
                FUN_004d5350(1);
            }
            return 1;
        }
        let pcvar4 = FUN_004dcfa0(0x123);
        FUN_004d9d20(r, &raw mut DAT_0053190c, pcvar4, 0xc);
        1
    }
}
