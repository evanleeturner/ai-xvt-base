// SPDX-License-Identifier: MIT

use core::mem::MaybeUninit;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006697c8: u16;
    static mut DAT_0066d800: [u16; 1];
    static mut DAT_00669800: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d6e30(a: u8, b: u8, c: u8) -> i32;
    fn _atoi(s: *const i8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004fbcd0(param_1: *mut i8, param_2: i32, param_3: i32) -> i32 {
    let mut p = param_1;
    let mut local: MaybeUninit<[i8; 256]> = MaybeUninit::uninit();
    // SAFETY: raw pointer walks over the caller's NUL-terminated string and
    // the local buffer, as the original does; globals are the game's own.
    unsafe {
        let buf = local.as_mut_ptr().cast::<i8>();
        let mut len: i32 = 0;
        while *param_1.offset(len as isize) != 0 {
            len = len.wrapping_add(1);
        }
        if *p == b'~' as i8 {
            p = p.add(1);

            // token 1
            let mut i: i32 = 0;
            let mut cnt: i32 = 0;
            if 0 < len {
                loop {
                    let c = *p;
                    if c == b' ' as i8 || c == 0 {
                        p = p.add(1);
                        len = len.wrapping_sub(1);
                        *buf.offset(i as isize) = 0;
                        break;
                    }
                    *buf.offset(i as isize) = c;
                    i = i.wrapping_add(1);
                    p = p.add(1);
                    len = len.wrapping_sub(1);
                    cnt = cnt.wrapping_add(1);
                    if !(cnt < len) {
                        break;
                    }
                }
            }
            let r = _atoi(buf);

            // token 2
            let mut i: i32 = 0;
            let mut cnt: i32 = 0;
            if 0 < len {
                loop {
                    let c = *p;
                    if c == b' ' as i8 || c == 0 {
                        p = p.add(1);
                        len = len.wrapping_sub(1);
                        *buf.offset(i as isize) = 0;
                        break;
                    }
                    *buf.offset(i as isize) = c;
                    i = i.wrapping_add(1);
                    p = p.add(1);
                    len = len.wrapping_sub(1);
                    cnt = cnt.wrapping_add(1);
                    if !(cnt < len) {
                        break;
                    }
                }
            }
            let g = _atoi(buf);

            // token 3
            let mut i: i32 = 0;
            let mut cnt: i32 = 0;
            if 0 < len {
                loop {
                    let c = *p;
                    if c == b' ' as i8 || c == 0 {
                        *buf.offset(i as isize) = 0;
                        p = p.add(1);
                        break;
                    }
                    *buf.offset(i as isize) = c;
                    i = i.wrapping_add(1);
                    p = p.add(1);
                    len = len.wrapping_sub(1);
                    cnt = cnt.wrapping_add(1);
                    if !(cnt < len) {
                        break;
                    }
                }
            }
            let b = _atoi(buf);
            let v = FUN_004d6e30(r as u8, g as u8, b as u8);
            DAT_006697c8 = v as u16;
        }

        let idx = param_3.wrapping_add(param_2.wrapping_mul(0x20));
        (&raw mut DAT_0066d800)
            .cast::<u16>()
            .offset(idx as isize)
            .write(DAT_006697c8);

        let mut n: u32 = 0;
        while *p.add(n as usize) != 0 {
            n = n.wrapping_add(1);
        }
        n = n.wrapping_add(1);

        let mut src = p as *const i8;
        let mut dst = (&raw mut DAT_00669800)
            .cast::<i8>()
            .offset(idx.wrapping_mul(0x100) as isize);
        let mut q = n >> 2;
        while q != 0 {
            dst.cast::<i32>()
                .write_unaligned(src.cast::<i32>().read_unaligned());
            src = src.add(4);
            dst = dst.add(4);
            q -= 1;
        }
        let mut r = n & 3;
        while r != 0 {
            *dst = *src;
            src = src.add(1);
            dst = dst.add(1);
            r -= 1;
        }
    }
    1
}
