// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732ff2: u8;
    static mut DAT_00732ff3: u8;
    static mut DAT_00732ff4: u8;
    static mut DAT_00732ff5: u8;
    static mut DAT_00732ff0: i32;
    static mut DAT_006db592: u8;
    static mut DAT_0073254c: u16;
    static mut DAT_00732554: u16;
    static mut DAT_00732f94: u16;
    static mut DAT_005373d0: [u8; 0];
    static mut DAT_005371b8: [u8; 0];
    static mut DAT_006db5a8: u32;
    static mut DAT_005397f8: u32;
    static mut DAT_005373b8: u32;
    static mut DAT_005373c4: u32;
    static mut DAT_005375f0: u32;
    static mut DAT_00536f68: u32;
    static mut DAT_00732e48: i32;
    static mut DAT_005250c8: i32;
    static mut DAT_005250cc: *mut c_void;
    static mut DAT_00525568: i32;
    static mut DAT_0071ecf4: i32;
    static mut DAT_0072f444: i32;
    static mut s_inputlog_txt_00525154: [i8; 0];
    static mut DAT_00525164: [i8; 0];
    static mut s__8x__2x__2x__2x__2x_0052513c: [i8; 0];
    static mut DAT_0071b740: i8;
    static mut DAT_00732920: i32;
    static mut DAT_00732940: i32;
    static mut DAT_0071bcb8: i32;
    static mut DAT_0071eaa0: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00405cf0(a: i32);
    fn FUN_0048f530() -> i32;
    fn _fopen(name: *const i8, mode: *const i8) -> *mut c_void;
    fn _fprintf(fp: *mut c_void, fmt: *const i8, ...) -> i32;
    fn _fflush(fp: *mut c_void) -> i32;
    fn FUN_00431110(a: i32, buf: *mut u32, len: u32);
    fn FUN_00432ca0() -> i32;
    fn FUN_00431d30() -> i32;
    fn FUN_004a7530() -> i32;
    fn FUN_004a7560(buf: *mut u8, max: i32) -> i32;
    fn FUN_0049b0d0(a: i32, b: u32, c: *mut i32) -> *mut i32;
    fn FUN_004a5320();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0042d530() {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers are formed into the game's buffers exactly as the original does.
    unsafe {
        let b373d0: *mut u8 = (&raw mut DAT_005373d0).cast::<u8>();
        let b371b8: *mut u8 = (&raw mut DAT_005371b8).cast::<u8>();

        FUN_00405cf0(-2);
        DAT_00732ff2 = DAT_006db592;
        DAT_00732ff3 = (DAT_0073254c as u8) & 0xfe;
        b373d0.cast::<u32>().write_unaligned(1);
        DAT_00732ff4 = (DAT_00732554 as u8) & 0xfe;
        DAT_00732ff5 = (DAT_00732f94 as u8) & 3;
        let r = FUN_0048f530();
        DAT_006db5a8 = DAT_006db5a8.wrapping_add(r as u32);
        let d = DAT_006db5a8.wrapping_sub(DAT_005397f8) as i32;
        let mut u_var2: u32 = if d < 0x7f && -1 < d && DAT_005397f8 != 0 {
            DAT_006db5a8 & 0x7f
        } else {
            0x7f
        };
        if 0xec < (DAT_005397f8.wrapping_sub(DAT_005373b8) as i32) {
            u_var2 = 0x7f;
        }
        let mut i_var1: i32;
        if u_var2 == 0x7f {
            i_var1 = 9;
            *b373d0.add(5) = DAT_006db5a8 as u8;
            *b373d0.add(6) = (DAT_006db5a8 >> 8) as u8;
            *b373d0.add(7) = (DAT_006db5a8 >> 16) as u8;
            *b373d0.add(4) = 0x7f;
            *b373d0.add(8) = (DAT_006db5a8 >> 0x18) as u8;
            DAT_005373b8 = DAT_006db5a8;
        } else {
            i_var1 = 5;
            *b373d0.add(4) = u_var2 as u8;
        }
        let mut i_var3: i32 = i_var1;
        DAT_005397f8 = DAT_006db5a8;
        if DAT_00732ff2 != 0 {
            i_var3 = i_var1.wrapping_add(1);
            *b373d0.add(4) |= 0x80;
            *b373d0.offset(i_var1 as isize) = DAT_00732ff2;
        }
        *b373d0.offset(i_var3 as isize) = DAT_00732ff3;
        *b373d0.offset(i_var3 as isize + 1) = DAT_00732ff4;
        if (DAT_00732ff5 & 1) != 0 {
            *b373d0.offset(i_var3 as isize) |= 1;
        }
        if (DAT_00732ff5 & 2) != 0 {
            *b373d0.offset(i_var3 as isize + 1) |= 1;
        }
        if 1 < DAT_00732e48 {
            if DAT_005250c8 == 1 {
                let mut ok = !DAT_005250cc.is_null();
                if !ok {
                    DAT_005250cc = _fopen(
                        (&raw const s_inputlog_txt_00525154).cast::<i8>(),
                        (&raw const DAT_00525164).cast::<i8>(),
                    );
                    ok = !DAT_005250cc.is_null();
                }
                if ok {
                    let tmp: u32 = b373d0.add(4).cast::<u32>().read_unaligned();
                    _fprintf(
                        DAT_005250cc,
                        (&raw const s__8x__2x__2x__2x__2x_0052513c).cast::<i8>(),
                        tmp,
                        DAT_00732ff2 as u32,
                        DAT_00732ff3 as u32,
                        DAT_00732ff4 as u32,
                        DAT_00732ff5 as u32,
                    );
                    _fflush(DAT_005250cc);
                }
            }
            if DAT_00525568 == 0 {
                i_var1 = 0;
                let mut pc: *mut i8 = &raw mut DAT_0071b740;
                let mut pi: *mut i32 = &raw mut DAT_00732920;
                loop {
                    if *pc != 0 && (*pi != 0 || i_var1 == DAT_0071ecf4) {
                        let tmpi: i32 = pc.byte_add(0x578).cast::<i32>().read_unaligned();
                        FUN_00431110(tmpi, b373d0.cast::<u32>(), i_var3.wrapping_add(2) as u32);
                    }
                    pc = pc.byte_add(0x5bd);
                    pi = pi.add(1);
                    i_var1 = i_var1.wrapping_add(1);
                    if !(pi < &raw mut DAT_00732940) {
                        break;
                    }
                }
            } else {
                b371b8.cast::<u32>().write_unaligned(0x16);
                *b371b8.add(4) = (*b371b8.add(4)).wrapping_add(1);
                let mut src: *const u8 = b373d0.add(4);
                let mut dst: *mut u8 = b371b8.add(DAT_005373c4 as usize);
                let n = (i_var3 as u32).wrapping_sub(2);
                let mut c = n >> 2;
                while c != 0 {
                    dst.cast::<u32>()
                        .write_unaligned(src.cast::<u32>().read_unaligned());
                    src = src.add(4);
                    dst = dst.add(4);
                    c -= 1;
                }
                c = n & 3;
                while c != 0 {
                    *dst = *src;
                    src = src.add(1);
                    dst = dst.add(1);
                    c -= 1;
                }
                DAT_005373c4 = DAT_005373c4
                    .wrapping_add(i_var3 as u32)
                    .wrapping_add(0xfffffffe);
                if DAT_005375f0 < DAT_006db5a8.wrapping_sub(DAT_00536f68) {
                    DAT_00536f68 = DAT_006db5a8;
                    u_var2 = DAT_005373c4;
                    i_var1 = FUN_00432ca0();
                    FUN_00431110(i_var1, b371b8.cast::<u32>(), u_var2);
                    i_var1 = FUN_00431d30();
                    if i_var1 == 0 {
                        if DAT_0072f444 < 4 {
                            i_var1 = 0;
                            let mut pi: *mut i32 = &raw mut DAT_00732920;
                            let mut p8: *mut i32 = &raw mut DAT_0071bcb8;
                            loop {
                                if *p8.byte_sub(0x578).cast::<i8>() != 0 && i_var1 != DAT_0071ecf4 {
                                    i_var3 = FUN_00432ca0();
                                    let mut tmpi: i32 = p8.read_unaligned();
                                    if i_var3 != tmpi && *pi != 0 {
                                        tmpi = p8.read_unaligned();
                                        FUN_00431110(tmpi, b371b8.cast::<u32>(), DAT_005373c4);
                                    }
                                }
                                pi = pi.add(1);
                                p8 = p8.byte_add(0x5bd);
                                i_var1 = i_var1.wrapping_add(1);
                                if !(p8 < &raw mut DAT_0071eaa0) {
                                    break;
                                }
                            }
                        }
                    } else if DAT_0072f444 == 2 {
                        i_var1 = 0;
                        let mut pc: *mut i8 = &raw mut DAT_0071b740;
                        let mut pi: *mut i32 = &raw mut DAT_00732920;
                        loop {
                            if *pc != 0 && i_var1 != DAT_0071ecf4 && *pi != 0 {
                                let tmpi: i32 = pc.byte_add(0x578).cast::<i32>().read_unaligned();
                                FUN_00431110(tmpi, b371b8.cast::<u32>(), DAT_005373c4);
                            }
                            pc = pc.byte_add(0x5bd);
                            pi = pi.add(1);
                            i_var1 = i_var1.wrapping_add(1);
                            if !(pi < &raw mut DAT_00732940) {
                                break;
                            }
                        }
                    }
                    DAT_005373c4 = 5;
                    b371b8.cast::<u32>().write_unaligned(0x16);
                    *b371b8.add(4) = 0;
                }
            }
            i_var1 = FUN_004a7530();
            if i_var1 != 0 {
                b373d0.cast::<u32>().write_unaligned(0x19);
                let tmpi = FUN_004a7560(b373d0.add(8), 500);
                b373d0.add(4).cast::<i32>().write_unaligned(tmpi);
                let tmp: u32 = b373d0.add(4).cast::<u32>().read_unaligned();
                u_var2 = tmp.wrapping_add(8);
                i_var1 = FUN_00432ca0();
                FUN_00431110(i_var1, b373d0.cast::<u32>(), u_var2);
            }
        }
        let p = FUN_0049b0d0(DAT_0071ecf4, DAT_006db5a8, &raw mut DAT_00732ff0);
        if !p.is_null() {
            *p = 0;
            *p.add(1) = 1;
        }
        FUN_004a5320();
    }
}
