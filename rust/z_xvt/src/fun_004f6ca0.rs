// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006509e0: u16;
    static mut DAT_00654a18: u16;
    static mut DAT_00650a18: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn atoi(s: *const i8) -> i32;
    fn FUN_004df760(a: u8, b: u8, c: u8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004f6ca0(param_1: *mut i8, param_2: i32, param_3: i32) -> i32 {
    let mut p = param_1 as *const i8;
    let mut buf = [0i8; 256];
    let b = buf.as_mut_ptr();
    // SAFETY: param_1 is a NUL-terminated string from the caller; the globals
    // are the game's own tables, indexed as the original does.
    unsafe {
        let mut len: i32 = 0;
        while *p.offset(len as isize) != 0 {
            len += 1;
        }
        if *p == b'~' as i8 {
            p = p.add(1);
            let mut i5: i32 = 0;
            let mut i2: i32 = 0;
            if 0 < len {
                loop {
                    let c = *p;
                    if c == b' ' as i8 || c == 0 {
                        p = p.add(1);
                        len = len.wrapping_sub(1);
                        *b.offset(i5 as isize) = 0;
                        break;
                    }
                    *b.offset(i5 as isize) = c;
                    i5 = i5.wrapping_add(1);
                    p = p.add(1);
                    len = len.wrapping_sub(1);
                    i2 = i2.wrapping_add(1);
                    if !(i2 < len) {
                        break;
                    }
                }
            }
            let v1 = atoi(b);
            let mut i6: i32 = 0;
            let mut i5: i32 = 0;
            if 0 < len {
                loop {
                    let c = *p;
                    if c == b' ' as i8 || c == 0 {
                        p = p.add(1);
                        len = len.wrapping_sub(1);
                        *b.offset(i6 as isize) = 0;
                        break;
                    }
                    *b.offset(i6 as isize) = c;
                    i6 = i6.wrapping_add(1);
                    p = p.add(1);
                    len = len.wrapping_sub(1);
                    i5 = i5.wrapping_add(1);
                    if !(i5 < len) {
                        break;
                    }
                }
            }
            let v2 = atoi(b);
            let mut i7: i32 = 0;
            let mut i6: i32 = 0;
            if 0 < len {
                loop {
                    let c = *p;
                    if c == b' ' as i8 || c == 0 {
                        *b.offset(i7 as isize) = 0;
                        p = p.add(1);
                        break;
                    }
                    *b.offset(i7 as isize) = c;
                    i7 = i7.wrapping_add(1);
                    p = p.add(1);
                    len = len.wrapping_sub(1);
                    i6 = i6.wrapping_add(1);
                    if !(i6 < len) {
                        break;
                    }
                }
            }
            let v3 = atoi(b);
            let r = FUN_004df760(v1 as u8, v2 as u8, v3 as u8);
            DAT_006509e0 = r as u16;
        }
        let idx = param_3.wrapping_add(param_2.wrapping_mul(0x20));
        (&raw mut DAT_00654a18)
            .offset(idx as isize)
            .write(DAT_006509e0);
        let mut n: usize = 0;
        while *p.add(n) != 0 {
            n += 1;
        }
        n += 1;
        let dst = (&raw mut DAT_00650a18).offset(idx.wrapping_mul(0x100) as isize);
        let mut k = 0usize;
        while k < n {
            *dst.add(k) = *p.add(k);
            k += 1;
        }
    }
    1
}
