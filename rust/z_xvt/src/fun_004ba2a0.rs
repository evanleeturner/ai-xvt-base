// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0085e6d0: [u8; 0xf7];
    static mut DAT_0081b740: [i8; 0x100];
    static mut DAT_0052a330: [i8; 1];
    static mut DAT_00531160: [*mut i8; 1];
    static mut DAT_005313c4: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004df3a0(a: i32) -> i32;
    fn FUN_004be160(name: *const i8, mode: *const i8) -> *mut core::ffi::c_void;
    fn FUN_004be1f0(f: *mut core::ffi::c_void);
    fn fgets(buf: *mut i8, n: i32, f: *mut core::ffi::c_void) -> *mut i8;
    fn atoi(s: *const i8) -> i32;
}

unsafe fn c_strlen(p: *const i8) -> i32 {
    let mut n: i32 = 0;
    // SAFETY: p points at a NUL-terminated buffer.
    unsafe {
        while *p.offset(n as isize) != 0 {
            n += 1;
        }
    }
    n
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004ba2a0() {
    // SAFETY: all globals and callees are the game's own memory and code;
    // offsets stay within the buffers as in the original.
    unsafe {
        let d = (&raw mut DAT_0085e6d0).cast::<u8>();
        let buf = (&raw mut DAT_0081b740).cast::<i8>();

        let mut k: usize = 0;
        while k < 0xf7 {
            *d.add(k) = 0;
            k += 1;
        }

        let mut i7: i32 = 0;
        loop {
            let o = i7 as usize;
            *d.add(0x00 + o) = 1;
            let i11 = i7 + 1;
            *d.add(0x02 + o) = 2;
            *d.add(0x04 + o) = 1;
            *d.add(0x06 + o) = 1;
            *d.add(0x08 + o) = 1;
            *d.add(0x0a + o) = 1;
            *d.add(0x0c + o) = 1;
            *d.add(0x0e + o) = 2;
            *d.add(0x10 + o) = 10;
            *d.add(0x12 + o) = 10;
            *d.add(0x14 + o) = 2;
            *d.add(0x16 + o) = 2;
            *d.add(0x18 + o) = 0;
            *d.add(0x1a + o) = 2;
            *d.add(0x1c + o) = 1;
            *d.add(0x1e + o) = 1;
            i7 = i11;
            if !(i11 < 2) {
                break;
            }
        }
        i7 = 0;
        *d.add(0x20) = 0;
        *d.add(0xc1) = 1;
        *d.add(0xc2) = 1;
        *d.add(0xc3) = 1;
        *d.add(0xf5) = 1;
        *d.add(0xc4) = 2;
        *d.add(0xc5) = 2;
        *d.add(0xc6) = 1;
        *d.add(0xc7) = 1;
        *d.add(0xc8) = 1;
        *d.add(0xcf) = 1;
        *d.add(0xc9) = 9;
        *d.add(0xca) = 9;
        *d.add(0xcb) = 9;
        *d.add(0xcc) = 9;
        *d.add(0xcd) = 9;
        *d.add(0xe4) = 1;
        *d.add(0xe5) = 1;
        *d.add(0xe6) = 1;
        *d.add(0xe7) = 0;
        *d.add(0xe8) = 1;
        *d.add(0xe9) = 0;
        *d.add(0xea) = 0;
        *d.add(0xeb) = 1;
        *d.add(0xec) = 1;
        *d.add(0xed) = 1;
        *d.add(0xef) = 1;
        *d.add(0xf0) = 0;
        *d.add(0xf1) = 0;
        *d.add(0xf2) = 0;
        *d.add(0xf3) = 0;
        *d.add(0xbf) = 0;
        *d.add(0xf4) = 0;
        *d.add(0xf6) = 1;
        *d.add(0xce) = 5;
        *d.add(0xee) = 0xff;
        *d.add(0xc0) = 8;

        let n = FUN_004df3a0(0);
        if n != 0 && -1 < (n >> 8) {
            loop {
                if 0xf < i7 {
                    break;
                }
                let v: Option<u8> = match i7 {
                    0 => Some(0x98),
                    1 => Some(0x99),
                    2 => Some(0x72),
                    3 => Some(0x2e),
                    4 => Some(0x65),
                    5 => Some(0x69),
                    6 => Some(0x5b),
                    7 => Some(8),
                    8 => Some(0xd),
                    9 => Some(0x5d),
                    _ => None,
                };
                if let Some(v) = v {
                    *d.add(0xd0 + i7 as usize) = v;
                }
                i7 += 1;
                if !(i7 < n) {
                    break;
                }
            }
        }

        let file = FUN_004be160(
            (&raw const DAT_005313c4).cast::<i8>(),
            (&raw const DAT_0052a330).cast::<i8>(),
        );
        if file.is_null() {
            return;
        }
        while !fgets(buf, 0x100, file).is_null() {
            let len = c_strlen(buf);
            if *buf.offset((len - 1) as isize) == 10 {
                let len2 = c_strlen(buf);
                *buf.offset((len2 - 1) as isize) = 0;
            }
            let mut p: *mut i8 = core::ptr::null_mut();
            let mut i: i32 = 0;
            if 0 < c_strlen(buf) {
                loop {
                    if *buf.offset(i as isize) == 0x20 {
                        *buf.offset(i as isize) = 0;
                        p = buf.offset((i + 1) as isize);
                        break;
                    }
                    i += 1;
                    if !(i < c_strlen(buf)) {
                        break;
                    }
                }
            }
            if p.is_null() {
                continue;
            }
            let table = (&raw mut DAT_00531160).cast::<*mut i8>();
            let mut idx: i32 = -1;
            let mut cnt: i32 = 0;
            if *(*table) != 0 {
                let mut pp = table;
                loop {
                    let mut a = (*pp).cast::<u8>();
                    let mut b = buf.cast::<u8>();
                    let eq;
                    loop {
                        if *a != *b {
                            eq = false;
                            break;
                        }
                        if *a == 0 {
                            eq = true;
                            break;
                        }
                        a = a.add(1);
                        b = b.add(1);
                    }
                    idx = cnt;
                    if eq {
                        break;
                    }
                    pp = pp.add(1);
                    cnt += 1;
                    idx = -1;
                    if !(*(*pp) != 0) {
                        break;
                    }
                }
            }

            let set = |off: usize| {
                *d.add(off) = atoi(p) as u8;
            };
            match idx {
                0 => {
                    let mut j: usize = 0;
                    while j < 13 {
                        *d.add(0xa1 + j) = *p.add(j) as u8;
                        j += 1;
                    }
                }
                1 => set(0x00),
                2 => set(0x02),
                3 => set(0x04),
                4 => set(0x06),
                5 => set(0x08),
                6 => set(0x0a),
                7 => set(0x0c),
                8 => set(0x0e),
                9 => set(0x10),
                10 => set(0x12),
                0xb => set(0x14),
                0xc => set(0x16),
                0xd => set(0x18),
                0xe => set(0x1a),
                0xf => set(0x01),
                0x10 => set(0x03),
                0x11 => set(0x05),
                0x12 => set(0x07),
                0x13 => set(0x09),
                0x14 => set(0x0b),
                0x15 => set(0x0d),
                0x16 => set(0x0f),
                0x17 => set(0x11),
                0x18 => set(0x13),
                0x19 => set(0x15),
                0x1a => set(0x17),
                0x1b => set(0x19),
                0x1c => set(0x1b),
                0x1d => set(0x20),
                0x1e => {
                    let mut j: usize = 0;
                    while j < 64 {
                        *d.add(0x21 + j) = *p.add(j) as u8;
                        j += 1;
                    }
                }
                0x1f => {
                    let mut j: usize = 0;
                    while j < 64 {
                        *d.add(0x61 + j) = *p.add(j) as u8;
                        j += 1;
                    }
                }
                0x20 => set(0xc1),
                0x21 => set(0xc2),
                0x22 => set(0xc3),
                0x23 => set(0xf5),
                0x24 => set(0xc4),
                0x25 => set(0xc5),
                0x26 => set(0xc6),
                0x27 => set(0xc7),
                0x28 => set(0xc8),
                0x29 => set(0xc9),
                0x2a => set(0xca),
                0x2b => set(0xcb),
                0x2c => set(0xcc),
                0x2d => set(0xcd),
                0x2e => set(0xce),
                0x2f..=0x42 => set(0xa1 + idx as usize),
                0x4f..=0x5a => set(0xe4 + (idx - 0x4f) as usize),
                0x5b => {
                    let v = atoi(p);
                    *d.add(0xf0) = v as u8;
                    *d.add(0xf1) = ((v as u32) >> 8) as u8;
                    *d.add(0xf2) = ((v as u32) >> 16) as u8;
                    *d.add(0xf3) = ((v as u32) >> 24) as u8;
                }
                0x5c => {
                    let mut j: usize = 0;
                    while j < 16 {
                        *d.add(0xaf + j) = *p.add(j) as u8;
                        j += 1;
                    }
                }
                0x5d => set(0xbf),
                0x5e => set(0xf4),
                0x5f => set(0xf6),
                0x60 => set(0xcf),
                0x61 => set(0xc0),
                0x62 => set(0x1c),
                99 => set(0x1e),
                100 => set(0x1d),
                0x65 => set(0x1f),
                _ => {}
            }
        }
        FUN_004be1f0(file);
    }
}
