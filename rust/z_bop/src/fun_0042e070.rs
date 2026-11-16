// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a8c10: i8;
    static mut DAT_00bb737d: i8;
    static mut DAT_00bb7375: i8;
    static mut DAT_00bb7376: i8;
    static mut DAT_00bb7374: i8;
    static mut DAT_00bb7377: i8;
    static mut DAT_005210b4: u32;
    static mut DAT_005210b8: u16;
    static mut DAT_009e967a: i16;
    static mut DAT_009ecc34: i32;
    static mut DAT_009a2000: i16;
    static mut DAT_009d8c94: i8;
    static mut DAT_009d9192: i8;
    static mut DAT_009d8c83: [u8; 1];
    static mut DAT_005210bc: [u32; 3];
    static mut DAT_005210c8: [u32; 3];
    static mut DAT_0052111c: [i8; 1];
    static mut DAT_005210ec: [i8; 1];
    static mut DAT_00521110: [i8; 1];
    static mut DAT_005210f8: [i8; 1];
    static mut DAT_00521104: [i8; 1];
    static mut DAT_005210e0: [i8; 1];
    static mut DAT_005210d4: [i8; 1];
    static mut DAT_00523448: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00425d10() -> u32;
    fn FUN_0042ded0(param_1: *mut i8, param_2: i32);
    fn FUN_004305d0(param_1: i32) -> u16;
}

unsafe fn c_strlen(p: *const i8) -> usize {
    let mut n = 0usize;
    // SAFETY: p points at a NUL-terminated string.
    unsafe {
        while *p.add(n) != 0 {
            n += 1;
        }
    }
    n
}

unsafe fn c_strcat(dst: *mut i8, src: *const i8) {
    // SAFETY: dst holds a NUL-terminated string with room; src is NUL-terminated.
    unsafe {
        let sl = c_strlen(src) + 1;
        let dl = c_strlen(dst);
        let mut i = 0usize;
        while i < sl {
            *dst.add(dl + i) = *src.add(i);
            i += 1;
        }
    }
}

unsafe fn rec_ptr() -> *const u8 {
    // SAFETY: the game's own record table.
    unsafe {
        (&raw mut DAT_009e967a)
            .cast::<u8>()
            .wrapping_add((DAT_009ecc34 as u32).wrapping_mul(0x5bd) as usize)
            .cast_const()
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042e070() {
    let mut bufw: [u32; 16] = [0; 16];
    let buf = bufw.as_mut_ptr().cast::<i8>();
    let mut path: [i8; 0x40] = [0; 0x40];
    let pathp = path.as_mut_ptr();
    let mut tmp: [i8; 12] = [0; 12];

    // SAFETY: all globals and callees are the game's own; buffers are local.
    unsafe {
        if DAT_009a8c10 != 0 && DAT_00bb737d != 0 {
            if DAT_00bb7375 != 0 {
                bufw[0] = DAT_005210b4;
                buf.add(4).cast::<u16>().write_unaligned(DAT_005210b8);
                let s2 = rec_ptr().cast::<i16>().read_unaligned();
                let src: *const i8;
                if s2 == 0 {
                    let u = FUN_00425d10();
                    if u & 1 == 0 {
                        src = (&raw const DAT_00521110).cast::<i8>();
                    } else {
                        src = (&raw const DAT_0052111c).cast::<i8>();
                    }
                } else {
                    let u = FUN_00425d10();
                    if u & 1 == 0 {
                        src = (&raw const DAT_005210f8).cast::<i8>();
                    } else {
                        src = (&raw const DAT_00521104).cast::<i8>();
                    }
                }
                c_strcat(buf, src);
                FUN_0042ded0(buf, 0x2b8);
            }
            if DAT_00bb7376 != 0 {
                bufw[0] = DAT_005210b4;
                buf.add(4).cast::<u16>().write_unaligned(DAT_005210b8);
                let s2 = rec_ptr().cast::<i16>().read_unaligned();
                let src: *const i8 = if s2 == 0 {
                    (&raw const DAT_005210ec).cast::<i8>()
                } else if s2 == 1 {
                    (&raw const DAT_005210e0).cast::<i8>()
                } else {
                    (&raw const DAT_005210d4).cast::<i8>()
                };
                c_strcat(buf, src);
                FUN_0042ded0(buf, 0x324);
            }
            if DAT_00bb7374 != 0 {
                let mut i8v: i32 = 0;
                if 0 < DAT_009a2000 {
                    let mut i5 = DAT_009a2000 as i32;
                    let mut p = (&raw const DAT_009d8c94).cast::<u8>();
                    loop {
                        if *p != 0 {
                            i8v = i8v.wrapping_add(1);
                        }
                        p = p.wrapping_add(0x566);
                        i5 = i5.wrapping_sub(1);
                        if i5 == 0 {
                            break;
                        }
                    }
                }
                let mut i5: i32 = 0;
                if 0 < DAT_009a2000 {
                    let mut pid = (&raw const DAT_009d9192).cast::<u8>();
                    loop {
                        let idv = pid.cast::<i32>().read_unaligned();
                        if idv == DAT_009ecc34 {
                            break;
                        }
                        pid = pid.wrapping_add(0x566);
                        i5 = i5.wrapping_add(1);
                        if !(i5 < DAT_009a2000 as i32) {
                            break;
                        }
                    }
                }
                let cnt_ptr = (&raw const DAT_009d8c83).cast::<u8>();
                if i5 < DAT_009a2000 as i32
                    && (1 < *cnt_ptr.wrapping_add((i5 as usize).wrapping_mul(0x566)) || 1 < i8v)
                {
                    let mut cnt: i32 =
                        *cnt_ptr.wrapping_add((i5 as usize).wrapping_mul(0x566)) as i32;
                    let mut u3 = FUN_004305d0(6);
                    let mut b = cnt != 0;
                    cnt = cnt.wrapping_sub(1);
                    if b {
                        let mut u12: u16 = 0x72;
                        loop {
                            bufw[0] = DAT_005210b4;
                            buf.add(4).cast::<u16>().write_unaligned(DAT_005210b8);
                            let s2 = rec_ptr().cast::<i16>().read_unaligned();
                            let s: *const u8 = if s2 == 1 {
                                (&raw const DAT_005210c8).cast::<u8>()
                            } else {
                                (&raw const DAT_005210bc).cast::<u8>()
                            };
                            tmp[0] = *s as i8;
                            tmp[1] = *s.add(1) as i8;
                            tmp[2] = *s.add(2) as i8;
                            tmp[3] = (u3 as u8).wrapping_add(0x31) as i8;
                            tmp[4] = *s.add(4) as i8;
                            tmp[5] = *s.add(5) as i8;
                            tmp[6] = *s.add(6) as i8;
                            tmp[7] = *s.add(7) as i8;
                            tmp[8] = *s.add(8) as i8;
                            u3 = u3.wrapping_add(1);
                            c_strcat(buf, tmp.as_ptr());
                            FUN_0042ded0(buf, u12 as i32);
                            if 5 < u3 {
                                u3 = 0;
                            }
                            b = cnt != 0;
                            cnt = cnt.wrapping_sub(1);
                            u12 = u12.wrapping_add(0x61);
                            if !b {
                                break;
                            }
                        }
                    }
                }
            }
            if DAT_00bb7377 != 0 {
                bufw[0] = DAT_005210b4;
                buf.add(4).cast::<u16>().write_unaligned(DAT_005210b8);
                let src = (&raw const DAT_00523448).cast::<i8>();
                let len = c_strlen(src) + 1;
                let mut i = 0usize;
                while i < len {
                    *pathp.add(i) = *src.add(i);
                    i += 1;
                }
                let u4 = len as u32;
                let i8v = u4.wrapping_sub(1) as i32;
                *pathp.add(u4.wrapping_sub(4) as usize) = b'l' as i8;
                *pathp.add(u4.wrapping_sub(3) as usize) = b's' as i8;
                *pathp.add(u4.wrapping_sub(2) as usize) = b't' as i8;
                let mut i5: i32 = 0;
                let mut zero = false;
                loop {
                    if *pathp == b'\\' as i8 {
                        break;
                    }
                    if i8v <= i5 {
                        zero = true;
                        break;
                    }
                    let i7 = i5.wrapping_add(1);
                    i5 = i5.wrapping_add(1);
                    *pathp = *pathp.offset(i7 as isize);
                }
                if zero {
                    i5 = 0;
                } else if i5 < i8v {
                    i5 = i5.wrapping_add(1);
                } else {
                    i5 = 0;
                }
                let blen = c_strlen(buf) as i32;
                let mut i7 = blen;
                if i5 < i8v {
                    let u11 = i8v.wrapping_sub(i5) as u32;
                    let mut sp = pathp.offset(i5 as isize);
                    let mut dp = buf.offset(blen as isize);
                    let mut k = 0u32;
                    while k < u11 {
                        *dp = *sp;
                        sp = sp.add(1);
                        dp = dp.add(1);
                        k += 1;
                    }
                    i7 = i7.wrapping_add(u11 as i32);
                }
                *buf.offset(i7 as isize) = 0;
                FUN_0042ded0(buf, 0x5f);
            }
        }
    }
}
