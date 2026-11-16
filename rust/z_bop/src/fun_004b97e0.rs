// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00bb72b0: u8;
    static mut DAT_00bb72b2: u8;
    static mut DAT_00bb72b4: u8;
    static mut DAT_00bb72b6: u8;
    static mut DAT_00bb72b8: u8;
    static mut DAT_00bb72ba: u8;
    static mut DAT_00bb72bc: u8;
    static mut DAT_00bb72be: u8;
    static mut DAT_00bb72c0: u8;
    static mut DAT_00bb72c2: u8;
    static mut DAT_00bb72c4: u8;
    static mut DAT_00bb72c6: u8;
    static mut DAT_00bb72c8: u8;
    static mut DAT_00bb72ca: u8;
    static mut DAT_00bb72cc: u8;
    static mut DAT_00bb72ce: u8;
    static mut DAT_00bb72b1: u8;
    static mut DAT_00bb72b3: u8;
    static mut DAT_00bb72b5: u8;
    static mut DAT_00bb72b7: u8;
    static mut DAT_00bb72b9: u8;
    static mut DAT_00bb72bb: u8;
    static mut DAT_00bb72bd: u8;
    static mut DAT_00bb72bf: u8;
    static mut DAT_00bb72c1: u8;
    static mut DAT_00bb72c3: u8;
    static mut DAT_00bb72c5: u8;
    static mut DAT_00bb72c7: u8;
    static mut DAT_00bb72c9: u8;
    static mut DAT_00bb72cb: u8;
    static mut DAT_00bb72cd: u8;
    static mut DAT_00bb72cf: u8;
    static mut DAT_00bb72d0: u8;
    static mut DAT_00bb72d1: u8;
    static mut DAT_00bb7311: u8;
    static mut DAT_00bb7351: u8;
    static mut DAT_00bb7355: u8;
    static mut DAT_00bb7359: u8;
    static mut DAT_00bb735d: u8;
    static mut DAT_00bb735f: u8;
    static mut DAT_00bb7363: u8;
    static mut DAT_00bb7367: u8;
    static mut DAT_00bb736b: u8;
    static mut DAT_00bb736f: u8;
    static mut DAT_00bb7370: u8;
    static mut DAT_00bb7371: u8;
    static mut DAT_00bb7372: u8;
    static mut DAT_00bb7373: u8;
    static mut DAT_00bb7374: u8;
    static mut DAT_00bb7375: u8;
    static mut DAT_00bb7376: u8;
    static mut DAT_00bb7377: u8;
    static mut DAT_00bb7378: u8;
    static mut DAT_00bb7379: u8;
    static mut DAT_00bb737a: u8;
    static mut DAT_00bb737b: u8;
    static mut DAT_00bb737c: u8;
    static mut DAT_00bb737d: u8;
    static mut DAT_00bb737e: u8;
    static mut DAT_00bb737f: u8;
    static mut DAT_00bb7380: u8;
    static mut DAT_00bb7394: u8;
    static mut DAT_00bb7395: u8;
    static mut DAT_00bb7396: u8;
    static mut DAT_00bb7397: u8;
    static mut DAT_00bb7398: u8;
    static mut DAT_00bb7399: u8;
    static mut DAT_00bb739a: u8;
    static mut DAT_00bb739b: u8;
    static mut DAT_00bb739c: u8;
    static mut DAT_00bb739d: u8;
    static mut DAT_00bb739e: u8;
    static mut DAT_00bb739f: u8;
    static mut DAT_00bb73a0: i32;
    static mut DAT_00bb73a4: u8;
    static mut DAT_00bb73a5: u8;
    static mut DAT_00bb73a6: u8;
    static mut DAT_00bb73a7: u8;
    static mut DAT_00bb73a8: u8;
    static mut DAT_00bb73a9: u8;
    static mut DAT_00bb73ef: u8;
    static mut DAT_00bb7435: u8;
    static mut DAT_00bb747b: u8;
    static mut DAT_00bb5897: u8;
    static mut DAT_00b69d20: i8;
    static mut DAT_0051c4bc: i8;
    static mut s_config2_cfg_0052a74c: i8;
    static mut s_config_cfg_0052a740: i8;
    static mut PTR_s_lastpilot_0052a4c8: *mut i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d5e60() -> u8;
    fn FUN_004d6300(param_1: i32) -> i32;
    fn FUN_004ddc10(param_1: i32) -> *mut i8;
    fn FUN_004cc4a0(param_1: *mut i8, param_2: *mut i8) -> *mut c_void;
    fn FUN_004ccf50();
    fn FUN_004ccf70();
    fn FUN_004cc590(param_1: *mut c_void);
    fn _fgets(param_1: *mut i8, param_2: i32, param_3: *mut c_void) -> *mut i8;
    fn _atoi(param_1: *const i8) -> i32;
}

unsafe fn str_len(p: *const i8) -> u32 {
    let mut n: u32 = 0;
    // SAFETY: p points at a NUL-terminated string.
    unsafe {
        while *p.add(n as usize) != 0 {
            n += 1;
        }
    }
    n
}

unsafe fn cp4(dst: *mut u8, src: *const i8) {
    // SAFETY: caller supplies valid source and destination of 4 bytes.
    unsafe {
        dst.cast::<u32>()
            .write_unaligned(src.cast::<u32>().read_unaligned());
    }
}

unsafe fn copy_str(dst: *mut u8, src: *const i8) {
    // SAFETY: copies the NUL-terminated string including its terminator.
    unsafe {
        let total = str_len(src).wrapping_add(1);
        let mut d = dst;
        let mut s = src;
        let mut n = total >> 2;
        while n != 0 {
            cp4(d, s);
            s = s.add(4);
            d = d.add(4);
            n -= 1;
        }
        let mut r = total & 3;
        while r != 0 {
            *d = *s as u8;
            s = s.add(1);
            d = d.add(1);
            r -= 1;
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004b97e0() {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers are used where the original uses them.
    unsafe {
        let mut local_50: [i32; 20] = [0; 20];
        let buf: *mut i8 = &raw mut DAT_00b69d20;

        let mut pu = (&raw mut DAT_00bb72b0).cast::<u32>();
        let mut k: i32 = 0x84;
        while k != 0 {
            pu.write_unaligned(0);
            pu = pu.add(1);
            k -= 1;
        }
        pu.cast::<u8>().write(0);

        let mut i: i32 = 0;
        loop {
            let u = i as usize;
            let n = i + 1;
            (&raw mut DAT_00bb72b0).add(u).write(1);
            (&raw mut DAT_00bb72b2).add(u).write(2);
            (&raw mut DAT_00bb72b4).add(u).write(1);
            (&raw mut DAT_00bb72b6).add(u).write(1);
            (&raw mut DAT_00bb72b8).add(u).write(1);
            (&raw mut DAT_00bb72ba).add(u).write(1);
            (&raw mut DAT_00bb72bc).add(u).write(1);
            (&raw mut DAT_00bb72be).add(u).write(2);
            (&raw mut DAT_00bb72c0).add(u).write(10);
            (&raw mut DAT_00bb72c2).add(u).write(10);
            (&raw mut DAT_00bb72c4).add(u).write(2);
            (&raw mut DAT_00bb72c6).add(u).write(2);
            (&raw mut DAT_00bb72c8).add(u).write(0);
            (&raw mut DAT_00bb72ca).add(u).write(2);
            let v = FUN_004d5e60();
            (&raw mut DAT_00bb72cc).add(u).write(v);
            (&raw mut DAT_00bb72ce).add(u).write(1);
            i = n;
            if !(n < 2) {
                break;
            }
        }
        let mut i7: i32 = 0;
        DAT_00bb72d0 = 0;
        DAT_00bb7371 = 1;
        DAT_00bb7372 = 1;
        DAT_00bb7373 = 1;
        DAT_00bb73a7 = 1;
        DAT_00bb7374 = 2;
        DAT_00bb7375 = 2;
        DAT_00bb7376 = 1;
        DAT_00bb7377 = 1;
        DAT_00bb7378 = 1;
        DAT_00bb737f = 1;
        DAT_00bb7379 = 9;
        DAT_00bb737a = 9;
        DAT_00bb737b = 9;
        DAT_00bb737c = 9;
        DAT_00bb737d = 9;
        DAT_00bb7394 = 1;
        DAT_00bb7395 = 1;
        DAT_00bb7396 = 1;
        DAT_00bb7397 = 0;
        DAT_00bb7398 = 1;
        DAT_00bb7399 = 0;
        DAT_00bb739a = 0;
        DAT_00bb739b = 1;
        DAT_00bb739c = 1;
        DAT_00bb739d = 1;
        DAT_00bb739f = 1;
        DAT_00bb73a0 = 0;
        DAT_00bb736f = 0;
        DAT_00bb73a4 = 0;
        DAT_00bb73a8 = 1;
        DAT_00bb73a5 = 0;
        DAT_00bb73a6 = 1;
        DAT_00bb737e = 5;
        DAT_00bb739e = 0xff;
        DAT_00bb7370 = 8;

        let i6 = FUN_004d6300(0);
        let extra = i6 >> 8;
        if i6 != 0 && -1 < extra {
            loop {
                if 0xf < i7 {
                    break;
                }
                let p = (&raw mut DAT_00bb7380).add(i7 as usize);
                match i7 {
                    0 => p.write(0x9c),
                    1 => p.write(0x9d),
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
                i7 += 1;
                if !(i7 < i6) {
                    break;
                }
            }
        }

        copy_str(&raw mut DAT_00bb73a9, FUN_004ddc10(0x316));
        copy_str(&raw mut DAT_00bb73ef, FUN_004ddc10(0x317));
        copy_str(&raw mut DAT_00bb7435, FUN_004ddc10(0x318));
        copy_str(&raw mut DAT_00bb747b, FUN_004ddc10(0x319));

        let mut file = FUN_004cc4a0(&raw mut s_config2_cfg_0052a74c, &raw mut DAT_0051c4bc);
        let b18 = file.is_null();
        if b18 {
            FUN_004ccf50();
            file = FUN_004cc4a0(&raw mut s_config_cfg_0052a740, &raw mut DAT_0051c4bc);
            FUN_004ccf70();
        }
        if !file.is_null() {
            loop {
                let r = _fgets(buf, 0x100, file);
                if r.is_null() {
                    break;
                }
                let len = str_len(buf);
                let lp = buf.offset(len as isize - 1);
                if *lp == 10 {
                    let len2 = str_len(buf);
                    *buf.offset(len2 as isize - 1) = 0;
                }
                let mut p5: *mut i8 = core::ptr::null_mut();
                let mut i7: i32 = 0;
                let len = str_len(buf);
                if len != 0 && -1 < (len as i32) {
                    loop {
                        if *buf.add(i7 as usize) == 0x20 {
                            *buf.add(i7 as usize) = 0;
                            p5 = buf.add(i7 as usize + 1);
                            break;
                        }
                        i7 += 1;
                        let l = str_len(buf);
                        if !(i7 < l as i32) {
                            break;
                        }
                    }
                }
                if p5.is_null() {
                    continue;
                }
                let mut i14: i32 = 0;
                let mut i7: i32 = -1;
                let table: *mut *mut i8 = &raw mut PTR_s_lastpilot_0052a4c8;
                if **table != 0 {
                    let mut pp = table;
                    loop {
                        let mut a = *pp as *const u8;
                        let mut b = buf as *const u8;
                        let eq;
                        loop {
                            let x = *a;
                            let y = *b;
                            if x != y {
                                eq = false;
                                break;
                            }
                            if x == 0 {
                                eq = true;
                                break;
                            }
                            a = a.add(1);
                            b = b.add(1);
                        }
                        i7 = i14;
                        if eq {
                            break;
                        }
                        let nx = pp.add(1);
                        pp = pp.add(1);
                        i14 += 1;
                        i7 = -1;
                        if **nx == 0 {
                            break;
                        }
                    }
                }
                match i7 {
                    0 => {
                        cp4(&raw mut DAT_00bb7351, p5);
                        cp4(&raw mut DAT_00bb7355, p5.add(4));
                        cp4(&raw mut DAT_00bb7359, p5.add(8));
                        DAT_00bb735d = *p5.add(0xc) as u8;
                    }
                    1 => DAT_00bb72b0 = _atoi(p5) as u8,
                    2 => DAT_00bb72b2 = _atoi(p5) as u8,
                    3 => DAT_00bb72b4 = _atoi(p5) as u8,
                    4 => DAT_00bb72b6 = _atoi(p5) as u8,
                    5 => DAT_00bb72b8 = _atoi(p5) as u8,
                    6 => DAT_00bb72ba = _atoi(p5) as u8,
                    7 => DAT_00bb72bc = _atoi(p5) as u8,
                    8 => DAT_00bb72be = _atoi(p5) as u8,
                    9 => DAT_00bb72c0 = _atoi(p5) as u8,
                    10 => DAT_00bb72c2 = _atoi(p5) as u8,
                    0xb => DAT_00bb72c4 = _atoi(p5) as u8,
                    0xc => DAT_00bb72c6 = _atoi(p5) as u8,
                    0xd => DAT_00bb72c8 = _atoi(p5) as u8,
                    0xe => DAT_00bb72ca = _atoi(p5) as u8,
                    0xf => DAT_00bb72b1 = _atoi(p5) as u8,
                    0x10 => DAT_00bb72b3 = _atoi(p5) as u8,
                    0x11 => DAT_00bb72b5 = _atoi(p5) as u8,
                    0x12 => DAT_00bb72b7 = _atoi(p5) as u8,
                    0x13 => DAT_00bb72b9 = _atoi(p5) as u8,
                    0x14 => DAT_00bb72bb = _atoi(p5) as u8,
                    0x15 => DAT_00bb72bd = _atoi(p5) as u8,
                    0x16 => DAT_00bb72bf = _atoi(p5) as u8,
                    0x17 => DAT_00bb72c1 = _atoi(p5) as u8,
                    0x18 => DAT_00bb72c3 = _atoi(p5) as u8,
                    0x19 => DAT_00bb72c5 = _atoi(p5) as u8,
                    0x1a => DAT_00bb72c7 = _atoi(p5) as u8,
                    0x1b => DAT_00bb72c9 = _atoi(p5) as u8,
                    0x1c => DAT_00bb72cb = _atoi(p5) as u8,
                    0x1d => DAT_00bb72d0 = _atoi(p5) as u8,
                    0x1e => {
                        let mut d = &raw mut DAT_00bb72d1;
                        let mut s = p5 as *const i8;
                        let mut c = 0x10;
                        while c != 0 {
                            cp4(d, s);
                            s = s.add(4);
                            d = d.add(4);
                            c -= 1;
                        }
                    }
                    0x1f => {
                        let mut d = &raw mut DAT_00bb7311;
                        let mut s = p5 as *const i8;
                        let mut c = 0x10;
                        while c != 0 {
                            cp4(d, s);
                            s = s.add(4);
                            d = d.add(4);
                            c -= 1;
                        }
                    }
                    0x20 => DAT_00bb7371 = _atoi(p5) as u8,
                    0x21 => DAT_00bb7372 = _atoi(p5) as u8,
                    0x22 => DAT_00bb7373 = _atoi(p5) as u8,
                    0x23 => DAT_00bb73a7 = _atoi(p5) as u8,
                    0x24 => DAT_00bb7374 = _atoi(p5) as u8,
                    0x25 => DAT_00bb7375 = _atoi(p5) as u8,
                    0x26 => DAT_00bb7376 = _atoi(p5) as u8,
                    0x27 => DAT_00bb7377 = _atoi(p5) as u8,
                    0x28 => DAT_00bb7378 = _atoi(p5) as u8,
                    0x29 => DAT_00bb7379 = _atoi(p5) as u8,
                    0x2a => DAT_00bb737a = _atoi(p5) as u8,
                    0x2b => DAT_00bb737b = _atoi(p5) as u8,
                    0x2c => DAT_00bb737c = _atoi(p5) as u8,
                    0x2d => DAT_00bb737d = _atoi(p5) as u8,
                    0x2e => DAT_00bb737e = _atoi(p5) as u8,
                    0x2f..=0x42 => {
                        let v = _atoi(p5);
                        (&raw mut DAT_00bb7351).add(i7 as usize).write(v as u8);
                        if b18 {
                            local_50[(i7 - 0x2f) as usize] = 1;
                        }
                    }
                    0x4f => DAT_00bb7394 = _atoi(p5) as u8,
                    0x50 => DAT_00bb7395 = _atoi(p5) as u8,
                    0x51 => DAT_00bb7396 = _atoi(p5) as u8,
                    0x52 => DAT_00bb7397 = _atoi(p5) as u8,
                    0x53 => DAT_00bb7398 = _atoi(p5) as u8,
                    0x54 => DAT_00bb7399 = _atoi(p5) as u8,
                    0x55 => DAT_00bb739a = _atoi(p5) as u8,
                    0x56 => DAT_00bb739b = _atoi(p5) as u8,
                    0x57 => DAT_00bb739c = _atoi(p5) as u8,
                    0x58 => DAT_00bb739d = _atoi(p5) as u8,
                    0x59 => DAT_00bb739e = _atoi(p5) as u8,
                    0x5a => DAT_00bb739f = _atoi(p5) as u8,
                    0x5b => DAT_00bb73a0 = _atoi(p5),
                    0x5c => {
                        cp4(&raw mut DAT_00bb735f, p5);
                        cp4(&raw mut DAT_00bb7363, p5.add(4));
                        cp4(&raw mut DAT_00bb7367, p5.add(8));
                        cp4(&raw mut DAT_00bb736b, p5.add(0xc));
                    }
                    0x5d => DAT_00bb736f = _atoi(p5) as u8,
                    0x5e => DAT_00bb73a4 = _atoi(p5) as u8,
                    0x5f => DAT_00bb73a8 = _atoi(p5) as u8,
                    0x60 => DAT_00bb737f = _atoi(p5) as u8,
                    0x61 => DAT_00bb7370 = _atoi(p5) as u8,
                    0x62 => DAT_00bb73a5 = _atoi(p5) as u8,
                    99..=0x66 => {
                        let mut d = (&raw mut DAT_00bb5897).offset((i7 as isize) * 0x46);
                        let mut s = p5 as *const i8;
                        let mut c = 0x11;
                        while c != 0 {
                            cp4(d, s);
                            s = s.add(4);
                            d = d.add(4);
                            c -= 1;
                        }
                        d.cast::<u16>()
                            .write_unaligned(s.cast::<u16>().read_unaligned());
                    }
                    0x67 => DAT_00bb72cc = _atoi(p5) as u8,
                    0x68 => DAT_00bb72ce = _atoi(p5) as u8,
                    0x69 => DAT_00bb72cd = _atoi(p5) as u8,
                    0x6a => DAT_00bb72cf = _atoi(p5) as u8,
                    _ => {}
                }
            }
            FUN_004cc590(file);
            if b18 {
                let mut i7: usize = 0;
                loop {
                    if local_50[i7] != 0 {
                        let p = (&raw mut DAT_00bb7380).add(i7);
                        let b = p.read();
                        if 0x7b < b && b < 0xe6 {
                            p.write(b.wrapping_add(4));
                        }
                    }
                    i7 += 1;
                    if !(i7 < 0x14) {
                        break;
                    }
                }
            }
        }
    }
}
