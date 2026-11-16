// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a082aa: u8;
    static mut DAT_00a082ab: u8;
    static mut DAT_00a082ac: u16;
    static mut DAT_009a8c12: u8;
    static mut DAT_00a00498: u16;
    static mut DAT_00a004a0: u16;
    static mut DAT_00a08244: u16;
    static mut DAT_00557360: [u8; 1];
    static mut DAT_00557364: u32;
    static mut DAT_00557368: u8;
    static mut DAT_009a8c2c: u32;
    static mut DAT_00559788: u32;
    static mut DAT_00557348: u32;
    static mut DAT_00a080f8: i32;
    static mut DAT_005242d0: i32;
    static mut DAT_005242d4: *mut core::ffi::c_void;
    static mut DAT_0052435c: [i8; 1];
    static mut DAT_0052436c: [i8; 1];
    static mut DAT_00524344: [i8; 1];
    static mut DAT_00523428: i32;
    static mut DAT_009e9680: [i8; 1];
    static mut DAT_00a07bb0: [i32; 1];
    static mut DAT_00a07bd0: i32;
    static mut DAT_009ecc34: i32;
    static mut DAT_00557148: [u8; 1];
    static mut DAT_00557354: u32;
    static mut DAT_00557580: u32;
    static mut DAT_00556ef8: u32;
    static mut DAT_009fd394: i32;
    static mut DAT_005242c4: i32;
    static mut DAT_009e9bf8: i32;
    static mut DAT_009ec9e0: i32;
    static mut DAT_00a082a8: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00411810(param_1: i32) -> u32;
    fn FUN_004ac760() -> i32;
    fn FID_conflict___wfopen(a: *const i8, b: *const i8) -> *mut core::ffi::c_void;
    fn _fprintf(f: *mut core::ffi::c_void, fmt: *const i8, ...) -> i32;
    fn _fflush(f: *mut core::ffi::c_void) -> i32;
    fn FUN_0046d350(param_1: i32, param_2: *mut u32, param_3: i32);
    fn FUN_0046f700() -> i32;
    fn FUN_00418760(param_1: i32, param_2: u32, param_3: *mut i32) -> *mut i32;
    fn FUN_004aa720();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00464900() {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers are formed from their addresses and accessed as the original does.
    unsafe {
        let buf360: *mut u8 = (&raw mut DAT_00557360).cast::<u8>();
        let buf148: *mut u8 = (&raw mut DAT_00557148).cast::<u8>();
        let p364: *mut u8 = (&raw mut DAT_00557364).cast::<u8>();

        FUN_00411810(-2);
        DAT_00a082aa = DAT_009a8c12;
        DAT_00a082ab = (DAT_00a00498 as u8) & 0xfe;
        buf360.cast::<u32>().write_unaligned(1);
        DAT_00a082ac = (DAT_00a082ac & 0xff00) | (((DAT_00a004a0 as u8) & 0xfe) as u16);
        DAT_00a082ac = (DAT_00a082ac & 0x00ff) | ((((DAT_00a08244 as u8) & 3) as u16) << 8);
        let mut i_var1: i32 = FUN_004ac760();
        DAT_009a8c2c = DAT_009a8c2c.wrapping_add(i_var1 as u32);
        let d = DAT_009a8c2c.wrapping_sub(DAT_00559788) as i32;
        let mut u_var2: u32 = if d < 0x7f && -1 < d && DAT_00559788 != 0 {
            DAT_009a8c2c & 0x7f
        } else {
            0x7f
        };
        if 0xec < (DAT_00559788.wrapping_sub(DAT_00557348) as i32) {
            u_var2 = 0x7f;
        }
        if u_var2 == 0x7f {
            i_var1 = 9;
            DAT_00557364 = (DAT_009a8c2c << 8) | 0x7f;
            DAT_00557368 = (DAT_009a8c2c >> 0x18) as u8;
            DAT_00557348 = DAT_009a8c2c;
        } else {
            i_var1 = 5;
            DAT_00557364 = (DAT_00557364 & 0xffffff00) | (u_var2 as u8 as u32);
        }
        let mut i_var3: i32 = i_var1;
        DAT_00559788 = DAT_009a8c2c;
        if DAT_00a082aa != 0 {
            i_var3 = i_var1.wrapping_add(1);
            DAT_00557364 |= 0x80;
            *buf360.offset(i_var1 as isize) = DAT_00a082aa;
        }
        *buf360.offset(i_var3 as isize) = DAT_00a082ab;
        *buf360.offset(i_var3.wrapping_add(1) as isize) = DAT_00a082ac as u8;
        if ((DAT_00a082ac >> 8) & 1) != 0 {
            *buf360.offset(i_var3 as isize) |= 1;
        }
        if ((DAT_00a082ac >> 8) & 2) != 0 {
            *buf360.offset(i_var3.wrapping_add(1) as isize) |= 1;
        }
        if 1 < DAT_00a080f8 {
            if DAT_005242d0 == 1 {
                let mut ok = !DAT_005242d4.is_null();
                if !ok {
                    DAT_005242d4 = FID_conflict___wfopen(
                        (&raw const DAT_0052435c).cast::<i8>(),
                        (&raw const DAT_0052436c).cast::<i8>(),
                    );
                    ok = !DAT_005242d4.is_null();
                }
                if ok {
                    _fprintf(
                        DAT_005242d4,
                        (&raw const DAT_00524344).cast::<i8>(),
                        DAT_00557364,
                        DAT_00a082aa as u32,
                        DAT_00a082ab as u32,
                        (DAT_00a082ac as u8) as u32,
                        ((DAT_00a082ac >> 8) as u8) as u32,
                    );
                    _fflush(DAT_005242d4);
                }
            }
            if DAT_00523428 == 0 {
                let mut i = 0i32;
                let mut pc: *mut i8 = (&raw mut DAT_009e9680).cast::<i8>();
                let mut p5: *mut i32 = (&raw mut DAT_00a07bb0).cast::<i32>();
                let end = (&raw mut DAT_00a07bd0) as *mut i32;
                loop {
                    if *pc != 0 && (*p5 != 0 || i == DAT_009ecc34) {
                        let i_tmp = pc.byte_add(0x578).cast::<i32>().read_unaligned();
                        FUN_0046d350(i_tmp, buf360.cast::<u32>(), i_var3.wrapping_add(2));
                    }
                    pc = pc.byte_add(0x5bd);
                    p5 = p5.add(1);
                    i = i.wrapping_add(1);
                    if !(p5 < end) {
                        break;
                    }
                }
            } else {
                buf148.cast::<u32>().write_unaligned(0x16);
                *buf148.add(4) = (*buf148.add(4)).wrapping_add(1);
                let mut s: *mut u8 = p364;
                let mut dst: *mut u8 = buf148.add(DAT_00557354 as usize);
                let n = i_var3.wrapping_sub(2) as u32;
                let mut c = n >> 2;
                while c != 0 {
                    dst.cast::<u32>()
                        .write_unaligned(s.cast::<u32>().read_unaligned());
                    s = s.add(4);
                    dst = dst.add(4);
                    c -= 1;
                }
                c = n & 3;
                while c != 0 {
                    *dst = *s;
                    s = s.add(1);
                    dst = dst.add(1);
                    c -= 1;
                }
                DAT_00557354 = DAT_00557354
                    .wrapping_add(i_var3 as u32)
                    .wrapping_add((-2i32) as u32);
                if DAT_00557580 < DAT_009a8c2c.wrapping_sub(DAT_00556ef8) {
                    let pu9 = buf148.cast::<u32>();
                    DAT_00556ef8 = DAT_009a8c2c;
                    let u2 = DAT_00557354;
                    let r = FUN_0046f700();
                    FUN_0046d350(r, pu9, u2 as i32);
                    if DAT_009fd394 < DAT_005242c4 {
                        let mut i = 0i32;
                        let mut p10: *mut i32 = (&raw mut DAT_00a07bb0).cast::<i32>();
                        let mut p8: *mut u8 = (&raw mut DAT_009e9bf8).cast::<u8>();
                        let end = (&raw mut DAT_009ec9e0).cast::<u8>();
                        loop {
                            if *p8.byte_sub(0x578) != 0 && i != DAT_009ecc34 {
                                let r3 = FUN_0046f700();
                                let i_tmp = p8.cast::<i32>().read_unaligned();
                                if r3 != i_tmp && *p10 != 0 {
                                    FUN_0046d350(i_tmp, buf148.cast::<u32>(), DAT_00557354 as i32);
                                }
                            }
                            p10 = p10.add(1);
                            p8 = p8.add(0x5bd);
                            i = i.wrapping_add(1);
                            if !(p8 < end) {
                                break;
                            }
                        }
                    }
                    DAT_00557354 = 5;
                    buf148.cast::<u32>().write_unaligned(0x16);
                    *buf148.add(4) = 0;
                }
            }
        }
        let p5 = FUN_00418760(DAT_009ecc34, DAT_009a8c2c, &raw mut DAT_00a082a8);
        if !p5.is_null() {
            *p5 = 0;
            *p5.add(1) = 1;
        }
        FUN_004aa720();
    }
}
