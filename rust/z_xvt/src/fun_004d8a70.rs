// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00757e40: [u8; 0];
    static mut DAT_0064f658: [i8; 0];
    static mut DAT_0085e6f0: u8;
    static mut DAT_0085e731: [i8; 0];
    static mut DAT_0085e6f1: [i8; 0];
    static mut DAT_0081b740: [i8; 0];
    static mut DAT_0081e016: u8;
    static mut DAT_00520278: i32;
    static mut DAT_0052027c: i32;
    static mut DAT_00520280: i32;
    static mut DAT_00520284: i32;
    static mut DAT_0081bcf0: [i8; 0];
    static mut DAT_00532a34: i32;
    static mut DAT_00758618: u32;
    static mut DAT_00758820: i32;
    static mut DAT_00758828: i32;
    static mut DAT_00757cc4: *mut u8;
    static mut DAT_00757cc0: i32;
    static mut DAT_00757cc8: i32;
    static mut DAT_00757ccc: i32;
    static mut DAT_005330fc: i32;
    static mut DAT_00758620: u32;
    static mut DAT_007585a0: i32;
    static mut DAT_007585a4: u8;
    static mut DAT_00757e34: u8;
    static mut DAT_0081bfba: [u8; 0];
    static mut DAT_0081bfbe: [u8; 0];
    static mut DAT_00757e70: [u8; 0];
    static mut DAT_00757e74: [u8; 0];
    static mut DAT_00757e78: [u8; 0];
    static mut DAT_00757e79: [u8; 0];
    static mut DAT_00757e7a: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ca200();
    fn FUN_004c96a0(
        a: i32,
        b: i32,
        c: i32,
        d: i32,
        e: *mut i8,
        f: *mut i8,
        g: i32,
        h: *mut i8,
        i: u32,
        j: i32,
    ) -> i32;
    fn FUN_004d8520();
    fn FUN_004cdbb0() -> i32;
    fn FUN_004cb830(a: i32, b: *mut u32, c: i32);
    fn FUN_004e0750() -> i32;
    fn FUN_004e43d0(a: i32);
    fn FUN_004e4820(a: *mut u8);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetTickCount() -> u32;
}

unsafe fn copy_str(mut src: *const i8, mut dst: *mut i8) {
    // SAFETY: caller passes game buffers; copies through the NUL.
    unsafe {
        loop {
            let c = *src;
            *dst = c;
            src = src.add(1);
            dst = dst.add(1);
            if c == 0 {
                break;
            }
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d8a70(param_1: i32) -> u32 {
    // SAFETY: all globals and callees are the game's own memory and code.
    unsafe {
        FUN_004ca200();
        let i_var6 = (param_1 as u32).wrapping_mul(0x3b) as i32;
        let off = i_var6 as usize;
        copy_str(
            (&raw mut DAT_00757e40).cast::<u8>().add(off).cast::<i8>(),
            (&raw mut DAT_0064f658).cast::<i8>(),
        );
        let mode = DAT_0085e6f0;
        if mode == 0 {
            (&raw mut DAT_0081b740).cast::<i8>().write(0);
        } else if mode == 1 {
            copy_str(
                (&raw mut DAT_0085e731).cast::<i8>(),
                (&raw mut DAT_0081b740).cast::<i8>(),
            );
        } else if mode == 2 {
            copy_str(
                (&raw mut DAT_0085e6f1).cast::<i8>(),
                (&raw mut DAT_0081b740).cast::<i8>(),
            );
        }
        let mut local_a: [i8; 2] = [(DAT_0081e016 as i8).wrapping_add(1), 0];
        let mut i_var4 = FUN_004c96a0(
            DAT_00520278,
            DAT_0052027c,
            DAT_00520280,
            DAT_00520284,
            local_a.as_mut_ptr(),
            (&raw mut DAT_0081bcf0).cast::<i8>(),
            0,
            (&raw mut DAT_0064f658).cast::<i8>(),
            DAT_0085e6f0 as u32,
            0,
        );
        let p70 = (&raw mut DAT_00757e70).cast::<u8>().add(off);
        let p74 = (&raw mut DAT_00757e74).cast::<u8>().add(off);
        let p78 = (&raw mut DAT_00757e78).cast::<u8>().add(off);
        let p79 = (&raw mut DAT_00757e79).cast::<u8>().add(off);
        let p7a = (&raw mut DAT_00757e7a).cast::<u8>().add(off);
        if i_var4 == 0 {
            if DAT_00532a34 != -1 {
                DAT_00532a34 = -1;
                DAT_00758618 = 0xffff_ffff;
                DAT_00758820 = 0;
                let mut p = (&raw mut DAT_0064f658).cast::<u32>();
                for _ in 0..8 {
                    p.write_unaligned(0);
                    p = p.add(1);
                }
                let mut b = DAT_00757cc4;
                for _ in 0..0x100 {
                    b.write(0);
                    b.add(1).write(0);
                    b.add(2).write(0);
                    b.add(3).write(0);
                    b = b.add(4);
                }
                FUN_004d8520();
            }
            p70.cast::<i32>().write_unaligned(0);
            let t = GetTickCount();
            p74.cast::<u32>().write_unaligned(t);
            p78.write(0);
            p79.write(0);
            p7a.write(0);
            return 0;
        }
        if DAT_00532a34 != -1 && param_1 == DAT_00532a34 {
            DAT_00758828 = 0;
            DAT_00758618 = 0xffff_ffff;
        }
        i_var4 = FUN_004cdbb0();
        DAT_00758620 = 0x40;
        FUN_004cb830(i_var4, &raw mut DAT_00758620, 4);
        if DAT_00532a34 != -1 && param_1 == DAT_00532a34 {
            DAT_00758620 = 0x6c;
            FUN_004cb830(i_var4, &raw mut DAT_00758620, 4);
        }
        DAT_007585a0 = 0;
        DAT_00758820 = 0;
        let local_4 = GetTickCount();
        let mut i_var6b;
        loop {
            i_var6b = FUN_004e0750();
            let d = GetTickCount();
            if i_var6b == 0x3a || i_var6b == 0x72 {
                break;
            }
            if i_var6b == 0x41 {
                if DAT_00532a34 == -1 || param_1 != DAT_00532a34 {
                    DAT_00758618 = 0xffff_ffff;
                } else {
                    let mut i_tmp = DAT_00758828;
                    (&raw mut DAT_0081bfba).cast::<i32>().write_unaligned(i_tmp);
                    let u_tmp = DAT_00758618;
                    (&raw mut DAT_0081bfbe)
                        .cast::<u8>()
                        .byte_add((DAT_00758828 as u32).wrapping_mul(4) as usize)
                        .cast::<u32>()
                        .write_unaligned(u_tmp);
                    FUN_004e43d0(i_tmp);
                    FUN_004e4820(DAT_00757cc4);
                    if DAT_00757cc0 != 0 {
                        DAT_00757cc8 = 0;
                        if DAT_00757ccc != 0 {
                            let mut p9 =
                                (DAT_00757cc0 as u32).wrapping_add(0x140) as usize as *const i32;
                            loop {
                                i_tmp = (&raw mut DAT_0081bfba).cast::<i32>().read_unaligned();
                                let i_cmp = (&raw mut DAT_0081bfbe)
                                    .cast::<u8>()
                                    .byte_add((i_tmp as u32).wrapping_mul(4) as usize)
                                    .cast::<i32>()
                                    .read_unaligned();
                                if p9.read_unaligned() == i_cmp {
                                    break;
                                }
                                p9 = p9.wrapping_byte_add(0x51 * 4);
                                DAT_00757cc8 = DAT_00757cc8.wrapping_add(1);
                                if !(DAT_00757cc8 < DAT_00757ccc) {
                                    break;
                                }
                            }
                        }
                    }
                    DAT_005330fc = 0;
                }
            }
            if !(d.wrapping_sub(local_4) < 2000) {
                break;
            }
        }
        if i_var6b == 0x3a {
            let t = GetTickCount();
            let u3 = DAT_007585a4;
            let u2 = DAT_00757e34;
            p74.cast::<u32>().write_unaligned(t);
            p70.cast::<i32>().write_unaligned(DAT_007585a0);
            p78.write(u2);
            p79.write(u3);
            p7a.write(1);
        } else if i_var6b == 0x72 {
            let t = GetTickCount();
            let u3 = DAT_007585a4;
            let u2 = DAT_00757e34;
            p74.cast::<u32>().write_unaligned(t);
            p70.cast::<i32>().write_unaligned(DAT_007585a0);
            p78.write(u2);
            p79.write(u3);
            p7a.write(0);
        } else {
            let t = GetTickCount();
            p74.cast::<u32>().write_unaligned(t);
            p70.cast::<i32>().write_unaligned(0);
            p78.write(0);
            p79.write(0);
            p7a.write(0);
        }
        1
    }
}
