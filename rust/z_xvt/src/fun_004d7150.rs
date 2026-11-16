// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00532a8c: i8;
    static mut DAT_00532a4c: i8;
    static mut DAT_00531358: i8;
    static mut DAT_00531eac: i8;
    static mut DAT_00531ebc: i8;
    static mut DAT_00532a84: i8;
    static mut DAT_00531e88: i8;
    static mut DAT_00531334: i8;
    static mut DAT_00531328: i8;
    static mut DAT_0053131c: i8;
    static mut DAT_00531310: i8;
    static mut DAT_005312fc: i8;
    static mut DAT_00531304: i8;

    static mut DAT_0081bc68: i32;
    static mut DAT_0085e6f0: i8;
    static mut DAT_0085e7c6: i8;
    static mut DAT_0053308c: i32;
    static mut DAT_00757ce0: u32;
    static mut DAT_0081b740: i8;
    static mut DAT_0081bcf0: i8;
    static mut DAT_0081bcd0: u32;
    static mut DAT_0081e082: i8;

    static mut LAB_004d6670: u8;
    static mut LAB_004bee00: u8;
    static mut LAB_004e1600: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d0290(a: *mut i8, b: *mut i8);
    fn FUN_004d52b0();
    fn FUN_004d1750(a: *mut i8, b: i32, c: i32);
    fn FUN_004d0e60(a: *mut i8, b: i32, c: i32);
    fn FUN_004d0580(a: *mut i8, b: i32, c: i32);
    fn FUN_004d5350(a: i32);
    fn FUN_004db490(a: i32);
    fn FUN_004dcfd0(a: *mut i32, b: i32, c: i32, d: i32, e: i32);
    fn FUN_004dcfa0(a: i32) -> *mut i8;
    fn FUN_004da920(a: i32, b: *mut i8, c: *mut i32, d: u32);
    fn FUN_004e0750() -> i32;
    fn FUN_004ca210();
    fn FUN_004dbf00();
    fn FUN_004db670(a: *const c_void, b: *mut u8);
    fn sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_004c0750(a: i32) -> i32;
    fn FUN_004da030();
    fn FUN_004da060(a: *mut i8);
    fn FUN_004d9a80(
        a: *mut i32,
        b: *mut i8,
        c: *mut i8,
        d: *mut i8,
        e: i32,
        f: u32,
        g: i32,
        h: *mut i8,
    ) -> i32;
    fn FUN_004da050();
    fn FUN_004d0430(a: *mut i8);
    fn FUN_004d66c0();
    fn FUN_004bef20();
    fn FUN_004e1770();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d7150(param_1: u32) -> u32 {
    let mut local_10: [i32; 4] = [0; 4];
    // SAFETY: all globals and callees are the game's own; pointers passed are
    // addresses of the game's buffers or of local storage.
    unsafe {
        if param_1 == 0 {
            FUN_004d0290(&raw mut DAT_00532a8c, &raw mut DAT_00532a4c);
            FUN_004d52b0();
            FUN_004d1750(&raw mut DAT_00532a4c, 0, 0);
            FUN_004d0e60(&raw mut DAT_00531358, 0, 0);
            let pc = if DAT_0081bc68 == 0 {
                &raw mut DAT_00531eac
            } else {
                &raw mut DAT_00531ebc
            };
            FUN_004d0e60(pc, 0, 0);
            FUN_004d0580(&raw mut DAT_00532a84, 0, 0);
            FUN_004d5350(1);
            FUN_004db490(0x14);
        }
        FUN_004dcfd0(local_10.as_mut_ptr(), 0x54, 0x6b, 0x1b2, 0x1b1);
        let pc = FUN_004dcfa0(0x14);
        FUN_004da920(0xf, pc, local_10.as_mut_ptr(), 0xffff);
        let mut i_var1 = FUN_004e0750();
        'a: {
            let pcv6: *const c_void;
            let pu9: *mut u8;
            if i_var1 == 0x44
                || i_var1 == 0x5e
                || i_var1 == 0x5f
                || i_var1 == 0x60
                || i_var1 == 0x46
                || i_var1 == 0x3a
            {
                FUN_004ca210();
                'sw: {
                    let u_var8: i32 = match i_var1 {
                        0x3a | 0x60 => {
                            FUN_004dcfa0(0x21c);
                            FUN_004dcfa0(0x21b);
                            0x21a
                        }
                        0x44 => {
                            FUN_004dcfa0(0x213);
                            FUN_004dcfa0(0x212);
                            0x211
                        }
                        0x46 => {
                            FUN_004dcfa0(0x26b);
                            FUN_004dcfa0(0x26a);
                            0x269
                        }
                        0x5e => {
                            FUN_004dcfa0(0x216);
                            FUN_004dcfa0(0x215);
                            0x214
                        }
                        0x5f => {
                            FUN_004dcfa0(0x219);
                            FUN_004dcfa0(0x218);
                            0x217
                        }
                        _ => break 'sw,
                    };
                    FUN_004dcfa0(u_var8);
                    FUN_004dbf00();
                }
                if DAT_0085e6f0 == 0 {
                    DAT_0053308c = 1;
                    pu9 = &raw mut LAB_004d6670;
                    pcv6 = FUN_004d66c0 as *const c_void;
                } else {
                    pu9 = &raw mut LAB_004bee00;
                    pcv6 = FUN_004bef20 as *const c_void;
                }
            } else {
                if i_var1 != 0x45 {
                    break 'a;
                }
                pu9 = &raw mut LAB_004e1600;
                let mut p = &raw mut DAT_00757ce0;
                let mut n = 0x54;
                while n != 0 {
                    p.write_unaligned(0);
                    p = p.add(1);
                    n -= 1;
                }
                pcv6 = FUN_004e1770 as *const c_void;
            }
            FUN_004db670(pcv6, pu9);
        }
        FUN_004dcfd0(local_10.as_mut_ptr(), 0x1fb, 0x1c4, 0x232, 0x1d0);
        sprintf(&raw mut DAT_0081b740, &raw const DAT_00531e88, 1i32, 1i32);
        FUN_004da920(0xc, &raw mut DAT_0081b740, local_10.as_mut_ptr(), 0xffff);
        FUN_004dcfd0(local_10.as_mut_ptr(), 200, 0x1c4, 0x1b4, 0x1d0);
        if DAT_0081bcf0 != 0 {
            sprintf(
                &raw mut DAT_0081b740,
                &raw const DAT_00531334,
                6i32,
                &raw mut DAT_0081e082,
                1i32,
                &raw mut DAT_0081bcf0,
            );
            FUN_004da920(
                0xc,
                &raw mut DAT_0081b740,
                local_10.as_mut_ptr(),
                DAT_0081bcd0,
            );
            let u_var8 = ((param_1 as i32) >> 0x1f) as u32;
            let t = (param_1 ^ u_var8).wrapping_sub(u_var8) & 0x1f;
            i_var1 = (((t ^ u_var8).wrapping_sub(u_var8)) as i32) >> 1;
            sprintf(&raw mut DAT_0081b740, &raw const DAT_00531328, i_var1);
            FUN_004d0e60(&raw mut DAT_0081b740, 0xcc, 0x1c5);
            sprintf(&raw mut DAT_0081b740, &raw const DAT_0053131c, i_var1);
            FUN_004d0e60(&raw mut DAT_0081b740, 0x1a4, 0x1c5);
        }
        i_var1 = FUN_004c0750(0);
        if i_var1 == 1 {
            return 1;
        }
        if DAT_0085e7c6 != 0 {
            FUN_004da030();
        }
        let pc = FUN_004dcfa0(0x13);
        FUN_004da060(pc);
        FUN_004dcfd0(local_10.as_mut_ptr(), 0x55, 0x1bf, 0xb0, 0x1d7);
        let pc10 = &raw mut DAT_00531310;
        let pc = FUN_004dcfa0(0x13);
        i_var1 = FUN_004d9a80(
            local_10.as_mut_ptr(),
            &raw mut DAT_005312fc,
            &raw mut DAT_00531304,
            pc,
            0xc,
            0,
            8,
            pc10,
        );
        FUN_004da050();
        if i_var1 != 0 || param_1 == 0x1e0 {
            FUN_004ca210();
            DAT_0053308c = 1;
            FUN_004db670(FUN_004d66c0 as *const c_void, &raw mut LAB_004d6670);
            FUN_004d0430(&raw mut DAT_00532a4c);
        }
        0
    }
}
