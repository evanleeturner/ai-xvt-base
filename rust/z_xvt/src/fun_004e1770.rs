// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0081bfb6: [u8; 4];
    static mut DAT_0081bfba: [u8; 4];
    static mut DAT_0081bfbe: [u8; 4];
    static mut DAT_0081c012: [u8; 4];
    static mut DAT_0081c016: [u8; 4];
    static mut DAT_0081fb96: [u8; 4];
    static mut DAT_0081fb9e: [u8; 4];
    static mut DAT_0081fba2: [u8; 4];
    static mut DAT_0081fba6: [u8; 4];
    static mut DAT_0081fbda: [u8; 4];
    static mut DAT_0081fbde: [u8; 4];
    static mut DAT_0083cbe6: [u8; 4];
    static mut DAT_0083cbea: [u8; 4];
    static mut DAT_0083cbee: [u8; 4];
    static mut DAT_0083cc22: [u8; 4];
    static mut DAT_0083cc26: [u8; 4];
    static mut DAT_0081d302: [u8; 4];
    static mut DAT_0081d402: [u8; 4];
    static mut DAT_0081f7be: [u8; 4];
    static mut DAT_0081fa7e: [u8; 4];
    static mut DAT_00757cee: [u8; 4];
    static mut DAT_00757cf2: [u8; 4];
    static mut DAT_00757e3e: [u8; 4];
    static mut DAT_0081e016: [u8; 4];
    static mut DAT_0081dc0e: [u8; 4];
    static mut DAT_0081d80e: [u8; 4];
    static mut DAT_0081d406: [u8; 4];
    static mut DAT_0081d32a: [u8; 4];
    static mut DAT_0081d40a: [u8; 4];

    static mut DAT_00743850: i32;
    static mut DAT_00757cd4: i32;
    static mut DAT_0081b850: i32;
    static mut DAT_0064ff44: i32;
    static mut DAT_0081b70c: i32;
    static mut DAT_00757cc4: *mut core::ffi::c_void;
    static mut DAT_005330fc: i32;
    static mut DAT_0064ff30: i32;
    static mut DAT_005330f8: i32;
    static mut DAT_0081bcb8: i32;
    static mut DAT_0064ff38: i32;
    static mut DAT_0064ff34: u32;
    static mut DAT_00757cc8: i32;
    static mut DAT_00757cc0: i32;
    static mut DAT_00757ccc: i32;
    static mut DAT_00757cd0: i32;
    static mut DAT_0053308c: i32;
    static mut DAT_00743860: [u32; 8];
    static mut DAT_0081bcf0: [i8; 1];
    static mut DAT_00757ce0: [i8; 1];
    static mut DAT_0085e7bd: i8;
    static mut DAT_0085e7bb: i8;
    static mut DAT_0085e7c6: i8;
    static mut DAT_0085e6f0: i8;
    static mut DAT_00758828: u32;
    static mut DAT_00758618: u32;
    static mut DAT_00758620: u32;
    static mut DAT_00758624: u32;
    static mut DAT_00758628: u32;
    static mut DAT_0075862c: u32;
    static mut DAT_00758630: u32;
    static mut DAT_0081bcd0: i32;
    static mut DAT_00533090: u32;
    static mut DAT_00533094: u32;
    static mut DAT_0081bfd2: [i8; 1];
    static mut DAT_0081b740: [i8; 1];
    static mut DAT_0081e082: [i8; 1];

    static mut DAT_00532a4c: [i8; 1];
    static mut DAT_00532aa4: [i8; 1];
    static mut DAT_00531e88: [i8; 1];
    static mut DAT_00531334: [i8; 1];
    static mut DAT_00531328: [i8; 1];
    static mut DAT_0053131c: [i8; 1];
    static mut DAT_00531310: [i8; 1];
    static mut DAT_00533128: [i8; 1];
    static mut DAT_00533134: [i8; 1];
    static mut DAT_005312fc: [i8; 1];
    static mut DAT_00531304: [i8; 1];
    static mut DAT_00532a58: [i8; 1];
    static mut DAT_00532a60: [i8; 1];
    static mut DAT_00532a6c: [i8; 1];
    static mut DAT_00533110: [i8; 1];
    static mut DAT_0053311c: [i8; 1];
    static mut DAT_00533100: [i8; 1];
    static mut DAT_00533108: [i8; 1];
}

#[allow(non_snake_case, non_upper_case_globals)]
unsafe extern "C" {
    fn FUN_004d66c0();
    fn LAB_004d6670();
    fn FUN_004bef20();
    fn LAB_004bee00();
    fn DAT_004ee9b0();
    fn LAB_004ee950();
    fn DAT_004e56b0();

    fn FUN_004ddc90(a: i32, b: i32);
    fn FUN_004ee3d0();
    fn _malloc(size: u32) -> *mut core::ffi::c_void;
    fn FUN_004f2990();
    fn FUN_004e4820(p: *mut core::ffi::c_void);
    fn FUN_004f0100();
    fn FUN_004e78b0();
    fn FUN_004db490(a: i32);
    fn FUN_004cc810() -> i32;
    fn FUN_004e5600(a: i32);
    fn FUN_004e5120();
    fn FUN_004e0750() -> i32;
    fn FUN_004d0430(s: *mut i8);
    fn FUN_004ca200();
    fn FUN_004dcfa0(id: i32) -> *mut i8;
    fn FUN_004dbf00() -> i32;
    fn FUN_004db670(a: unsafe extern "C" fn(), b: unsafe extern "C" fn());
    fn FUN_004cdbb0() -> i32;
    fn FUN_004cb830(a: i32, b: *mut u32, c: u32);
    fn FUN_004dcfd0(r: *mut i32, a: i32, b: i32, c: i32, d: i32);
    fn FUN_004da920(a: i32, s: *mut i8, r: *mut i32, c: i32);
    fn _sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_004dd070(r: *mut i32, a: i32, b: i32);
    fn FUN_004daac0(a: i32, s: *mut i8, r: *mut i32, c: i32, d: i32, e: i32);
    fn FUN_004e4a20();
    fn FUN_004d7900(a: u32);
    fn FUN_004d0e60(s: *mut i8, x: i32, y: i32);
    fn FUN_004e4b80(a: u32);
    fn FUN_004e6960();
    fn FUN_004da030();
    fn FUN_004da060(s: *mut i8);
    fn FUN_004d9a80(
        r: *mut i32,
        up: *mut i8,
        down: *mut i8,
        txt: *mut i8,
        a: i32,
        b: i32,
        c: i32,
        snd: *mut i8,
    ) -> i32;
    fn FUN_004e6630() -> i32;
    fn FUN_004da050();
    fn FUN_004cde00() -> i32;
    fn FUN_004e5320(a: i32);
    fn FUN_004db6c0(f: unsafe extern "C" fn(), r: *mut i32);
    fn FUN_004e2910() -> i32;
    fn FUN_004c0750(a: i32) -> i32;
    fn FUN_004cddb0(a: i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetTickCount() -> u32;
}

macro_rules! p {
    ($n:ident) => {
        (&raw mut $n).cast::<u8>()
    };
}

macro_rules! c {
    ($n:ident) => {
        (&raw mut $n).cast::<i8>()
    };
}

macro_rules! bfba {
    () => {
        ld(p!(DAT_0081bfba))
    };
}

macro_rules! c012 {
    () => {
        ld(p!(DAT_0081c012))
    };
}

macro_rules! fb96 {
    () => {
        ld(p!(DAT_0081fb96))
    };
}

macro_rules! fix_bd {
    () => {
        if DAT_0081b850 != 2 && (bfba!() == 3 || bfba!() == 4) && DAT_0085e7bd == 2 {
            DAT_0085e7bd = 1;
        }
    };
}

unsafe fn ld(p: *const u8) -> u32 {
    // SAFETY: the caller passes an address inside the game's own memory.
    unsafe { p.cast::<u32>().read_unaligned() }
}

unsafe fn st(p: *mut u8, v: u32) {
    // SAFETY: the caller passes an address inside the game's own memory.
    unsafe { p.cast::<u32>().write_unaligned(v) }
}

fn sel(base: *mut u8, idx: u32) -> *mut u8 {
    base.wrapping_add(idx.wrapping_mul(4) as usize)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004e1770(param_1: u32) -> u32 {
    let mut ai: [i32; 4] = [0; 4];
    let r = ai.as_mut_ptr();
    // SAFETY: all globals and callees belong to the game; pointers are
    // derived from its own memory and used as the original does.
    unsafe {
        if param_1 == 0 {
            FUN_004ddc90(0x25, 0x1bd);
            FUN_004ee3d0();
            DAT_00743850 = 0;
            DAT_00757cd4 = 0;
            if DAT_0081b850 == 2 {
                DAT_0064ff44 = 0;
                let u_key = fb96!().wrapping_mul(0x3a09);
                st(p!(DAT_0081bfb6), ld(sel(p!(DAT_0081fb9e), u_key)));
                st(p!(DAT_0081bfba), ld(sel(p!(DAT_0081fba2), u_key)));
                let mut s = sel(p!(DAT_0081fba6), u_key);
                let mut d = p!(DAT_0081bfbe);
                let mut i = 5;
                while i != 0 {
                    st(d, ld(s));
                    s = s.wrapping_add(4);
                    d = d.wrapping_add(4);
                    i -= 1;
                }
                st(p!(DAT_0081c012), ld(sel(p!(DAT_0081fbda), u_key)));
                st(sel(p!(DAT_0081fbda), fb96!().wrapping_mul(0x3a09)), 0);
                st(
                    p!(DAT_0081c016),
                    ld(sel(p!(DAT_0081fbde), fb96!().wrapping_mul(0x3a09))),
                );
            } else {
                st(p!(DAT_0081bfb6), ld(p!(DAT_0083cbe6)));
                st(p!(DAT_0081bfba), ld(p!(DAT_0083cbea)));
                DAT_0064ff44 = 1;
                let mut s = p!(DAT_0083cbee);
                let mut d = p!(DAT_0081bfbe);
                let mut i = 5;
                while i != 0 {
                    st(d, ld(s));
                    s = s.wrapping_add(4);
                    d = d.wrapping_add(4);
                    i -= 1;
                }
                st(p!(DAT_0081c012), ld(p!(DAT_0083cc22)));
                st(p!(DAT_0081c016), ld(p!(DAT_0083cc26)));
                st(p!(DAT_0083cc22), 0);
            }
            DAT_0081b70c = 0;
            DAT_00757cc4 = _malloc(0x400);
            DAT_005330fc = 0;
            DAT_0064ff30 = 0;
            DAT_005330f8 = 0;
            DAT_0081bcb8 = 0;
            DAT_0064ff38 = -1;
            DAT_0064ff34 = GetTickCount();
            DAT_00757cc8 = 0;
            if DAT_00757cc0 != 0 && DAT_00757ccc != 0 {
                let mut pi = (DAT_00757cc0.wrapping_add(0x140)) as *mut i32;
                loop {
                    if pi.read_unaligned() as u32 == ld(sel(p!(DAT_0081bfbe), bfba!())) {
                        break;
                    }
                    pi = pi.wrapping_add(0x51);
                    DAT_00757cc8 = DAT_00757cc8.wrapping_add(1);
                    if !(DAT_00757cc8 < DAT_00757ccc) {
                        break;
                    }
                }
            }
            if c012!() == 1 {
                st(sel(p!(DAT_0081bfbe), bfba!()), ld(p!(DAT_0081c016)));
            }
            let mut d = p!(DAT_0081d302);
            let mut i = 0x40;
            while i != 0 {
                st(d, 0);
                d = d.wrapping_add(4);
                i -= 1;
            }
            let mut d = p!(DAT_0081d402);
            let mut i = 0x305;
            while i != 0 {
                st(d, 0);
                d = d.wrapping_add(4);
                i -= 1;
            }
            let i_var13 = bfba!() as i32;
            if c012!() == 1 {
                st(p!(DAT_0081bfba), 2);
                if i_var13 != 1 {
                    st(p!(DAT_0081bfba), 4);
                }
            }
            st(p!(DAT_0081c012), 0);
            FUN_004f2990();
            FUN_004e4820(DAT_00757cc4);
            FUN_004f0100();
            fix_bd!();
            if DAT_0085e7bb == 2 && (DAT_0081b850 == 2 || (bfba!() != 1 && bfba!() != 2)) {
                DAT_0085e7bb = 1;
            }
            DAT_0053308c = 0;
            let mut p10 = p!(DAT_0081f7be);
            loop {
                let mut p11 = p10;
                let mut i = 0x16;
                while i != 0 {
                    st(p11, 0);
                    p11 = p11.wrapping_add(4);
                    i -= 1;
                }
                st(p10.wrapping_add(0x40), 0);
                p11 = p10.wrapping_add(0x58);
                st(p10.wrapping_add(0x44), 0xffffffff);
                st(p10.wrapping_add(0x48), 0xffffffff);
                st(p10.wrapping_add(0x4c), 0xffffffff);
                st(p10.wrapping_add(0x50), 0xffffffff);
                p10 = p11;
                if !(p11 < p!(DAT_0081fa7e)) {
                    break;
                }
            }
            let mut i = 0usize;
            while i < 8 {
                DAT_00743860[i] = 0;
                i += 1;
            }
            if DAT_0081b850 == 2 {
                let mut n: u32 = 0;
                let src0 = c!(DAT_0081bcf0);
                loop {
                    let ch = src0.wrapping_add(n as usize).read();
                    n = n.wrapping_add(1);
                    if ch == 0 {
                        break;
                    }
                }
                let mut s = src0;
                let mut d = c!(DAT_00757ce0);
                let mut w = n >> 2;
                while w != 0 {
                    st(d.cast::<u8>(), ld(s.cast::<u8>()));
                    s = s.wrapping_add(4);
                    d = d.wrapping_add(4);
                    w -= 1;
                }
                let mut b = n & 3;
                while b != 0 {
                    d.write(s.read());
                    s = s.wrapping_add(1);
                    d = d.wrapping_add(1);
                    b -= 1;
                }
                st(p!(DAT_00757cee), 1);
                st(p!(DAT_00757cf2), ld(p!(DAT_0081e016)));
            }
            FUN_004e78b0();
            FUN_004db490(0x14);
        }
        if DAT_0081b850 != 2 {
            let mut i_var7 = FUN_004cc810();
            if i_var7 != 0 {
                let t = GetTickCount();
                if 5000 < t.wrapping_sub(DAT_0064ff34) {
                    FUN_004e5600(0);
                    FUN_004e5120();
                    DAT_0064ff34 = t;
                }
            }
            i_var7 = FUN_004e0750();
            if i_var7 == 0x41 {
                st(p!(DAT_0081bfba), DAT_00758828);
                st(sel(p!(DAT_0081bfbe), DAT_00758828), DAT_00758618);
                FUN_004f2990();
                FUN_004d0430(c!(DAT_00532a4c));
                FUN_004e78b0();
                FUN_004e4820(DAT_00757cc4);
                FUN_004f0100();
                if DAT_00757cc0 != 0 {
                    DAT_00757cc8 = 0;
                    if DAT_00757ccc != 0 {
                        let mut pi = (DAT_00757cc0.wrapping_add(0x140)) as *mut i32;
                        loop {
                            if pi.read_unaligned() as u32 == ld(sel(p!(DAT_0081bfbe), bfba!())) {
                                break;
                            }
                            pi = pi.wrapping_add(0x51);
                            DAT_00757cc8 = DAT_00757cc8.wrapping_add(1);
                            if !(DAT_00757cc8 < DAT_00757ccc) {
                                break;
                            }
                        }
                    }
                }
                DAT_005330fc = 0;
                DAT_0064ff38 = -1;
                fix_bd!();
                if DAT_0085e7bb == 2 {
                    if DAT_0081b850 == 2 {
                        DAT_0085e7bb = 1;
                    } else if bfba!() != 1 && bfba!() != 2 {
                        DAT_0085e7bb = 1;
                    }
                }
            } else if i_var7 == 0x46 {
                FUN_004ca200();
                i_var7 = FUN_004cc810();
                if i_var7 == 0 {
                    FUN_004dcfa0(0x26b);
                    FUN_004dcfa0(0x26a);
                    FUN_004dcfa0(0x269);
                    FUN_004dbf00();
                }
                DAT_0081b850 = 3;
                if DAT_0085e6f0 == 0 {
                    FUN_004db670(FUN_004d66c0, LAB_004d6670);
                } else {
                    FUN_004db670(FUN_004bef20, LAB_004bee00);
                }
            } else if i_var7 == 0x69 {
                FUN_004ca200();
                FUN_004dcfa0(0x297);
                FUN_004dcfa0(0x296);
                FUN_004dcfa0(0x295);
                FUN_004dbf00();
                DAT_0081b850 = 3;
                if DAT_0085e6f0 == 0 {
                    FUN_004db670(FUN_004d66c0, LAB_004d6670);
                } else {
                    FUN_004db670(FUN_004bef20, LAB_004bee00);
                }
            } else {
                if i_var7 == 0x49 {
                    fix_bd!();
                    if DAT_0085e7bb == 2 && bfba!() != 1 && bfba!() != 2 {
                        DAT_0085e7bb = 1;
                    }
                    if bfba!() == 3 && c012!() == 1 {
                        st(p!(DAT_0081dc0e), DAT_00757cc8 as u32);
                        st(p!(DAT_0081d80e), DAT_00757cc8 as u32);
                        if DAT_00757cc0 != 0 {
                            let a = (DAT_00757cc8 as u32)
                                .wrapping_mul(0x144)
                                .wrapping_add(0x140)
                                .wrapping_add(DAT_00757cc0 as u32);
                            st(p!(DAT_0081d406), ld(a as *const u8));
                        }
                    }
                    DAT_0081bcb8 = 1;
                    FUN_004db670(DAT_004ee9b0, LAB_004ee950);
                    return 0;
                }
                if i_var7 == 0x5b {
                    DAT_0053308c = 1;
                    DAT_00758620 = 0x47;
                    let u = 4u32;
                    let pu = &raw mut DAT_00758620;
                    let a = FUN_004cdbb0();
                    FUN_004cb830(a, pu, u);
                    FUN_004ca200();
                    FUN_004dcfa0(0x226);
                    FUN_004dcfa0(0x225);
                    FUN_004dcfa0(0x224);
                    FUN_004dbf00();
                    if DAT_0085e6f0 == 0 {
                        FUN_004db670(FUN_004d66c0, LAB_004d6670);
                        return 0;
                    }
                    FUN_004db670(FUN_004bef20, LAB_004bee00);
                    return 0;
                }
                if i_var7 == 0x75 {
                    let mut idx: u32 = 0;
                    let mut pb = p!(DAT_00757cee);
                    loop {
                        if ld(pb) == DAT_00533090 {
                            st(
                                p!(DAT_00757cf2).wrapping_add(idx.wrapping_mul(0x2a) as usize),
                                DAT_00533094,
                            );
                            break;
                        }
                        pb = pb.wrapping_add(0x2a);
                        idx = idx.wrapping_add(1);
                        if !(pb < p!(DAT_00757e3e)) {
                            break;
                        }
                    }
                }
            }
            if DAT_0081b850 != 2 {
                FUN_004dcfd0(r, 0x9e, 0x34, 0x1eb, 0x44);
                FUN_004da920(0xc, c!(DAT_0081bfd2), r, 0xffff);
            }
        }
        let p5 = FUN_004dcfa0(bfba!().wrapping_add(0xb6) as i32);
        let u12 = 4u32;
        let p6 = FUN_004dcfa0(0xb5);
        _sprintf(c!(DAT_0081b740), c!(DAT_00532aa4), p6, u12, p5);
        FUN_004dcfd0(r, 0x54, 0x5a, 0x1b2, 0x6c);
        FUN_004da920(0xf, c!(DAT_0081b740), r, 0xffff);
        let mut i_var7 = FUN_004cc810();
        let u12: i32 = if i_var7 == 0 && DAT_0081b850 != 2 {
            0x1b2
        } else {
            0x1a0
        };
        FUN_004dcfd0(r, 0x54, 0x70, u12, 0x82);
        if DAT_00757cc8 < DAT_00757ccc {
            FUN_004dd070(r, 4, 0);
            let s = (DAT_00757cc8 as u32)
                .wrapping_mul(0x144)
                .wrapping_add(DAT_00757cc0 as u32)
                .wrapping_add(0x40) as *mut i8;
            FUN_004daac0(0xf, s, r, 0, 1, DAT_0081bcd0);
        }
        FUN_004e4a20();
        if DAT_0081b850 != 2 {
            FUN_004d7900(param_1);
        }
        FUN_004dcfd0(r, 0x1fb, 0x1c4, 0x232, 0x1d0);
        _sprintf(c!(DAT_0081b740), c!(DAT_00531e88), 1i32, 1i32);
        FUN_004da920(0xc, c!(DAT_0081b740), r, 0xffff);
        FUN_004dcfd0(r, 200, 0x1c4, 0x1b4, 0x1d0);
        if DAT_0081bcf0[0] != 0 {
            _sprintf(
                c!(DAT_0081b740),
                c!(DAT_00531334),
                6i32,
                c!(DAT_0081e082),
                1i32,
                c!(DAT_0081bcf0),
            );
            FUN_004da920(0xc, c!(DAT_0081b740), r, DAT_0081bcd0);
            let s = ((param_1 as i32) >> 0x1f) as u32;
            let q = ((((param_1 ^ s).wrapping_sub(s) & 0x1f) ^ s).wrapping_sub(s) as i32) >> 1;
            if DAT_0081b850 == 2 && bfba!() != 1 && bfba!() != 2 {
                if fb96!() == 0 {
                    _sprintf(c!(DAT_0081b740), c!(DAT_00531328), q);
                    FUN_004d0e60(c!(DAT_0081b740), 0xcc, 0x1c5);
                } else {
                    _sprintf(c!(DAT_0081b740), c!(DAT_0053131c), q);
                    FUN_004d0e60(c!(DAT_0081b740), 0xcc, 0x1c5);
                }
            } else {
                _sprintf(c!(DAT_0081b740), c!(DAT_00531328), q);
                FUN_004d0e60(c!(DAT_0081b740), 0xcc, 0x1c5);
                _sprintf(c!(DAT_0081b740), c!(DAT_0053131c), q);
            }
            FUN_004d0e60(c!(DAT_0081b740), 0x1a4, 0x1c5);
        }
        if DAT_0064ff30 == 1 || DAT_0081b850 == 2 {
            FUN_004e6960();
        } else {
            FUN_004e4b80(param_1);
        }
        FUN_004dcfd0(r, 0x55, 0x1bf, 0xb0, 0x1d7);
        if DAT_0085e7c6 != 0 {
            FUN_004da030();
        }
        'l: {
            if DAT_0081b850 == 2 {
                let p5 = FUN_004dcfa0(0x285);
                FUN_004da060(p5);
                let snd = c!(DAT_00531310);
                let p5 = FUN_004dcfa0(0x285);
                i_var7 = FUN_004d9a80(r, c!(DAT_00533128), c!(DAT_00533134), p5, 0xc, 0, 8, snd);
                if i_var7 != 0 {
                    if (bfba!() == 2 || bfba!() == 4) && FUN_004e6630() == 0 {
                        FUN_004da050();
                        return 0;
                    }
                    DAT_005330f8 = 1;
                    FUN_004db670(DAT_004ee9b0, LAB_004ee950);
                    FUN_004da050();
                    return 0;
                }
                break 'l;
            }
            i_var7 = FUN_004cc810();
            if i_var7 == 0 {
                let p5 = FUN_004dcfa0(0x22b);
                FUN_004da060(p5);
                let snd = c!(DAT_00531310);
                let p5 = FUN_004dcfa0(0xf6);
                i_var7 = FUN_004d9a80(r, c!(DAT_005312fc), c!(DAT_00531304), p5, 0xc, 0, 8, snd);
                if i_var7 == 0 {
                    break 'l;
                }
                FUN_004dcfa0(0x13);
                FUN_004dcfa0(0x1fd);
                FUN_004dcfa0(0x21f);
                FUN_004dcfa0(0x21e);
                FUN_004dcfa0(0x21d);
                i_var7 = FUN_004dbf00();
                if i_var7 == 0 {
                    break 'l;
                }
                DAT_0053308c = 1;
                DAT_00758620 = 0x47;
                let u = 4u32;
                let pu = &raw mut DAT_00758620;
                let a = FUN_004cdbb0();
                FUN_004cb830(a, pu, u);
                FUN_004ca200();
                FUN_004db670(FUN_004d66c0, LAB_004d6670);
            } else {
                let p5 = FUN_004dcfa0(0x22b);
                FUN_004da060(p5);
                let snd = c!(DAT_00531310);
                let p5 = FUN_004dcfa0(0xf5);
                i_var7 = FUN_004d9a80(r, c!(DAT_005312fc), c!(DAT_00531304), p5, 0xc, 0, 8, snd);
                if i_var7 == 0 {
                    break 'l;
                }
                if DAT_00757cd0 != 0 && DAT_0081b850 != 2 {
                    FUN_004dcfa0(0x13);
                    FUN_004dcfa0(0x1fd);
                    FUN_004dcfa0(0x222);
                    FUN_004dcfa0(0x221);
                    FUN_004dcfa0(0x220);
                    i_var7 = FUN_004dbf00();
                }
                if i_var7 == 0 {
                    break 'l;
                }
                DAT_0053308c = 1;
                DAT_00758620 = 0x46;
                FUN_004cb830(0, &raw mut DAT_00758620, 4);
                FUN_004ca200();
                DAT_0081b850 = 0;
                FUN_004db670(FUN_004bef20, LAB_004bee00);
            }
        }
        if DAT_00757cd4 != 0 {
            DAT_00757cd4 -= 1;
        }
        FUN_004dcfd0(r, 8, 0x195, 0x47, 0x1d6);
        if DAT_0081b850 == 2 {
            let p5 = FUN_004dcfa0(0x2b3);
            FUN_004da060(p5);
            let snd = c!(DAT_00532a6c);
            let p5 = FUN_004dcfa0(0x2b3);
            i_var7 = FUN_004d9a80(r, c!(DAT_00532a58), c!(DAT_00532a60), p5, 0xc, 0, 7, snd);
            if i_var7 != 0 {
                if (bfba!() == 2 || bfba!() == 4) && FUN_004e6630() == 0 {
                    FUN_004da050();
                    return 0;
                }
                FUN_004db670(DAT_004ee9b0, LAB_004ee950);
                FUN_004da050();
                return 0;
            }
        } else {
            i_var7 = FUN_004cc810();
            if i_var7 != 0 && DAT_00757cd4 == 0 {
                let p5 = FUN_004dcfa0(0x2b3);
                FUN_004da060(p5);
                let snd = c!(DAT_00532a6c);
                let p5 = FUN_004dcfa0(0x2b3);
                i_var7 = FUN_004d9a80(r, c!(DAT_00532a58), c!(DAT_00532a60), p5, 0xc, 0, 7, snd);
                if i_var7 != 0 {
                    let rec = (DAT_00757cc8 as u32)
                        .wrapping_mul(0x144)
                        .wrapping_add(DAT_00757cc0 as u32)
                        as *const u8;
                    let b2 = rec.read();
                    i_var7 = FUN_004cde00();
                    if ((b2 as i32).wrapping_sub(0x30)) < i_var7 {
                        let b2 = rec.read();
                        if b2 == 0x31 {
                            FUN_004dcfa0(0x271);
                            FUN_004dcfa0(0x270);
                            FUN_004dcfa0(0x26f);
                            FUN_004dbf00();
                        } else {
                            let v = (b2 as i32).wrapping_sub(0x30);
                            let p5 = FUN_004dcfa0(0x206);
                            _sprintf(c!(DAT_0081b740), p5, v);
                            FUN_004dcfa0(0x207);
                            FUN_004dbf00();
                        }
                    } else {
                        if (bfba!() == 2 || bfba!() == 4) && FUN_004e6630() == 0 {
                            FUN_004da050();
                            return 0;
                        }
                        FUN_004e5320(0);
                        DAT_00758628 = bfba!();
                        DAT_00758624 = ld(sel(p!(DAT_0081bfbe), bfba!()));
                        DAT_0075862c = c012!();
                        DAT_0081bcb8 = 1;
                        DAT_00758620 = 0x49;
                        let mut u12 = ld(p!(DAT_0081d32a));
                        if bfba!() == 1 {
                            DAT_00758630 = u12;
                        } else {
                            u12 = ld(p!(DAT_0081d40a));
                            if bfba!() == 3 {
                                DAT_00758630 = u12;
                            }
                        }
                        FUN_004cb830(0, &raw mut DAT_00758620, 0x14);
                    }
                }
            }
        }
        FUN_004da050();
        i_var7 = FUN_004cc810();
        if i_var7 != 0 || DAT_0081b850 == 2 {
            FUN_004dcfd0(r, 0x1a1, 0x70, 0x1b2, 0x82);
            let snd = c!(DAT_00531310);
            let p5 = FUN_004dcfa0(0x29e);
            i_var7 = FUN_004d9a80(
                r,
                c!(DAT_00533110),
                c!(DAT_0053311c),
                p5,
                0xf,
                0xffff,
                9,
                snd,
            );
            if i_var7 != 0 {
                FUN_004dcfd0(r, 0, 0, 0x27f, 0x1df);
                FUN_004db6c0(DAT_004e56b0, r);
            }
        }
        i_var7 = FUN_004e2910();
        if DAT_0081b850 != 2 {
            let i13 = FUN_004cc810();
            if i13 != 0 && i_var7 != 0 {
                FUN_004e5320(0);
            }
        }
        i_var7 = FUN_004c0750(1);
        if i_var7 != 1 {
            if DAT_0081b850 == 4 {
                let i13 = FUN_004cc810();
                if i13 != 0 && DAT_0064ff30 == 0 {
                    FUN_004dcfd0(r, 0x1a1, 0x135, 0x1b0, 0x150);
                    let snd = c!(DAT_00531310);
                    let p5 = FUN_004dcfa0(0x205);
                    i_var7 =
                        FUN_004d9a80(r, c!(DAT_00533100), c!(DAT_00533108), p5, 0xc, 0, 10, snd);
                    if i_var7 != 0 && DAT_0064ff38 != -1 {
                        DAT_00758620 = 0x5b;
                        let a = ld(p!(DAT_00757cee)
                            .wrapping_add((DAT_0064ff38 as u32).wrapping_mul(0x2a) as usize))
                            as i32;
                        FUN_004cb830(a, &raw mut DAT_00758620, 4);
                        let a = ld(p!(DAT_00757cee)
                            .wrapping_add((DAT_0064ff38 as u32).wrapping_mul(0x2a) as usize))
                            as i32;
                        FUN_004cddb0(a);
                        FUN_004e5320(0);
                    }
                }
            }
            return 0;
        }
        1
    }
}
