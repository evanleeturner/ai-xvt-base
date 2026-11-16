// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa6114: *mut core::ffi::c_void;
    static mut DAT_00aa6138: u32;
    static mut DAT_00b69e30: i32;
    static mut DAT_00b6d962: i32;
    static mut DAT_00b69d20: [i8; 256];
    static mut DAT_0051c4bc: [i8; 4];
    static mut DAT_0052c1e8: [*mut i8; 6];
    static mut DAT_0052c2e0: [i8; 16];
    static mut DAT_0052c2d0: [i8; 16];
    static mut DAT_0052c2f0: [i8; 16];
    static mut DAT_00b7ca4a: [u8; 4];
    static mut DAT_00b7d6ca: [u8; 4];
    static mut DAT_00b8e0aa: [u8; 4];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    #[link_name = "free"]
    fn c_free(p: *mut core::ffi::c_void);
    #[link_name = "malloc"]
    fn c_malloc(size: u32) -> *mut core::ffi::c_void;
    #[link_name = "sprintf"]
    fn c_sprintf(dst: *mut i8, fmt: *const i8, ...) -> i32;
    #[link_name = "fgets"]
    fn c_fgets(buf: *mut i8, n: i32, file: *mut core::ffi::c_void) -> *mut i8;
    #[link_name = "atoi"]
    fn c_atoi(s: *const i8) -> i32;
    #[link_name = "tolower"]
    fn c_tolower(c: i32) -> i32;
    fn FUN_004cc4a0(path: *mut i8, mode: *const i8) -> *mut core::ffi::c_void;
    fn FUN_004ddc10(id: i32) -> *mut i8;
    fn FUN_004dcb90(a: *mut i8, b: *mut i8, c: *mut i8, d: *mut i8, e: *mut i8) -> i32;
    fn FUN_004e9230(file: *mut core::ffi::c_void) -> i32;
    fn FUN_004cc5c0(file: *mut core::ffi::c_void, a: i32, b: i32);
    fn FUN_004cc590(file: *mut core::ffi::c_void);
}

unsafe fn slen(p: *const i8) -> u32 {
    let mut n: u32 = 0;
    // SAFETY: p points at a NUL-terminated string.
    unsafe {
        while *p.add(n as usize) != 0 {
            n = n.wrapping_add(1);
        }
    }
    n
}

unsafe fn cpy(dst: *mut i8, src: *const i8) {
    // SAFETY: the original copies strlen+1 bytes forward between distinct buffers.
    unsafe {
        let n = slen(src).wrapping_add(1) as usize;
        core::ptr::copy_nonoverlapping(src, dst, n);
    }
}

unsafe fn strip_nl(buf: *mut i8) {
    // SAFETY: mirrors the original's last-char check (index len-1, even if len is 0).
    unsafe {
        let len = slen(buf);
        let p = buf.offset((len as i32).wrapping_sub(1) as isize);
        if *p == 10 {
            *p = 0;
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004e5000(param_1: i32) {
    // SAFETY: all globals and callees are the game's own; buffers are used
    // exactly as the original does.
    unsafe {
        let buf: *mut i8 = (&raw mut DAT_00b69d20).cast::<i8>();
        let mut local_80: [i8; 128] = [0; 128];
        let l80: *mut i8 = local_80.as_mut_ptr();

        if !DAT_00aa6114.is_null() {
            c_free(DAT_00aa6114);
            DAT_00aa6114 = core::ptr::null_mut();
        }
        DAT_00aa6138 = 0;

        let table = (&raw const DAT_0052c1e8).cast::<*mut i8>();
        let mut fmt: *const i8 = core::ptr::null();
        let mut arg: *mut i8 = core::ptr::null_mut();
        let mut do_sprintf = true;
        if DAT_00b69e30 == 2 || DAT_00b69e30 == 0 {
            match param_1 {
                0 | 3 | 5 => {
                    arg = table.offset(param_1 as isize).read();
                    if DAT_00b6d962 == 0 {
                        fmt = (&raw const DAT_0052c2e0).cast::<i8>();
                    } else {
                        fmt = (&raw const DAT_0052c2d0).cast::<i8>();
                    }
                }
                1 | 2 | 4 => {
                    arg = table.offset(param_1 as isize).read();
                    fmt = (&raw const DAT_0052c2f0).cast::<i8>();
                }
                _ => {
                    do_sprintf = false;
                }
            }
        } else {
            arg = table.offset(param_1 as isize).read();
            fmt = (&raw const DAT_0052c2f0).cast::<i8>();
        }
        if do_sprintf {
            c_sprintf(buf, fmt, arg);
        }

        let file: *mut core::ffi::c_void;
        loop {
            let f = FUN_004cc4a0(buf, (&raw const DAT_0051c4bc).cast::<i8>());
            if !f.is_null() {
                file = f;
                break;
            }
            let p12 = FUN_004ddc10(0x13);
            let p2 = FUN_004ddc10(0x20b);
            let p3 = FUN_004ddc10(0x2c4);
            let p4 = FUN_004ddc10(0x2c3);
            let p5 = FUN_004ddc10(0x2c2);
            if FUN_004dcb90(p5, p4, p3, p2, p12) == 0 {
                return;
            }
        }

        DAT_00aa6138 = FUN_004e9230(file) as u32;
        FUN_004cc5c0(file, 0, 0);
        if DAT_00aa6138 == 0 {
            FUN_004cc590(file);
            return;
        }
        DAT_00aa6114 = c_malloc(DAT_00aa6138.wrapping_mul(0x148));
        if DAT_00aa6114.is_null() {
            FUN_004cc590(file);
            return;
        }

        let mut iv10: u32 = 0;
        let mut local_84: u32 = 0;
        loop {
            if DAT_00aa6138 <= local_84 {
                FUN_004cc590(file);
                return;
            }
            'sec: loop {
                loop {
                    if c_fgets(buf, 0x100, file).is_null() {
                        DAT_00aa6138 = local_84;
                        FUN_004cc590(file);
                        return;
                    }
                    if !(*buf == 0x2f && *buf.add(1) == 0x2f) {
                        break;
                    }
                }
                strip_nl(buf);
                if *buf == 0x5b {
                    let len = slen(buf);
                    *buf.offset((len as i32).wrapping_sub(1) as isize) = 0;
                    cpy(l80, buf.add(1));
                    continue 'sec;
                }
                break 'sec;
            }

            let base = DAT_00aa6114.cast::<u8>();
            cpy(base.add(iv10 as usize).add(0xc0).cast::<i8>(), l80);
            base.add(iv10 as usize)
                .add(0x144)
                .cast::<i32>()
                .write_unaligned(0);
            let v = c_atoi(buf);
            base.add(iv10 as usize)
                .add(0x140)
                .cast::<i32>()
                .write_unaligned(v);

            if c_fgets(buf, 0x100, file).is_null() {
                DAT_00aa6138 = local_84;
                FUN_004cc590(file);
                return;
            }
            strip_nl(buf);

            if slen(buf) != 0 {
                let mut i: u32 = 0;
                loop {
                    let u11 = i.wrapping_add(1);
                    let r = c_tolower(*buf.add(i as usize) as u8 as i32);
                    *buf.add(i as usize) = r as i8;
                    i = u11;
                    if !(u11 < slen(buf)) {
                        break;
                    }
                }
            }

            let src: *const i8;
            if *buf == 0x2a {
                let mut iv6: i32 = 2;
                let mut c = *buf.add(2);
                while c != 0x20 {
                    c = *buf.offset((iv6 as isize) + 1);
                    iv6 = iv6.wrapping_add(1);
                }
                *buf.offset(iv6 as isize) = 0;
                c_atoi(buf.add(2));
                let p12 = buf.offset(iv6 as isize + 1);
                c = *p12;
                while c != 0x20 {
                    c = *buf.offset(iv6 as isize + 2);
                    iv6 = iv6.wrapping_add(1);
                }
                *buf.offset(iv6 as isize + 1) = 0;
                c_atoi(p12);
                let id = base
                    .add(iv10 as usize)
                    .add(0x140)
                    .cast::<i32>()
                    .read_unaligned();
                if DAT_00b69e30 == 2 {
                    let idx = (DAT_00b6d962 as u32)
                        .wrapping_mul(0x84f)
                        .wrapping_add(id as u32)
                        .wrapping_mul(0x20);
                    let t = (&raw mut DAT_00b7ca4a)
                        .cast::<u8>()
                        .add(idx as usize)
                        .cast::<i32>()
                        .read_unaligned();
                    if t == 0 {
                        base.add(iv10 as usize)
                            .add(0x144)
                            .cast::<i32>()
                            .write_unaligned(1);
                    }
                } else {
                    let idx = (id as u32).wrapping_mul(0x20);
                    let a = (&raw mut DAT_00b7d6ca)
                        .cast::<u8>()
                        .add(idx as usize)
                        .cast::<i32>()
                        .read_unaligned();
                    let b = (&raw mut DAT_00b8e0aa)
                        .cast::<u8>()
                        .add(idx as usize)
                        .cast::<i32>()
                        .read_unaligned();
                    if a == 0 && b == 0 {
                        base.add(iv10 as usize)
                            .add(0x144)
                            .cast::<i32>()
                            .write_unaligned(1);
                    }
                }
                src = buf.offset(iv6 as isize + 2);
            } else if *buf != 0x26 {
                src = buf;
            } else {
                base.add(iv10 as usize)
                    .add(0x144)
                    .cast::<i32>()
                    .write_unaligned(1);
                src = buf.add(2);
            }
            cpy(base.add(iv10 as usize).cast::<i8>(), src);

            if c_fgets(buf, 0x100, file).is_null() {
                DAT_00aa6138 = local_84;
                FUN_004cc590(file);
                return;
            }
            strip_nl(buf);
            cpy(base.add(iv10 as usize).add(0x40).cast::<i8>(), buf);
            iv10 = iv10.wrapping_add(0x148);
            local_84 = local_84.wrapping_add(1);
        }
    }
}
