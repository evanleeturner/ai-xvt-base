// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0085e7c0: u32;
    static mut DAT_00757ccc: u32;
    static mut DAT_00757cc0: u32;
    static mut DAT_00757cc8: u32;
    static mut DAT_0081bfba: i32;
    static mut DAT_0081bfbe: u8;
    static mut DAT_0081d402: u32;
    static mut DAT_0081d40a: u8;
    static mut DAT_0081d80e: u8;
    static mut DAT_0081dc0e: u8;
    static mut DAT_0081d326: u32;
    static mut DAT_0081c012: u32;
    static mut DAT_0081c016: u32;
    static mut DAT_0081d406: u32;
    static mut DAT_0085e7b7: i8;
    static mut DAT_0081b740: [i8; 256];
    static mut DAT_0052a330: i8;
    static mut PTR_s_train_005330e0: [*const i8; 0];
    static mut s__s__s_005331e4: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_004be160(name: *mut i8, mode: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn _fgets(buf: *mut i8, n: i32, f: *mut core::ffi::c_void) -> *mut i8;
    fn FUN_004be1f0(f: *mut core::ffi::c_void) -> i32;
    fn _atoi(s: *const i8) -> i32;
    fn _srand(seed: u32);
    fn _rand() -> i32;
    fn _tolower(c: i32) -> i32;
    fn FUN_004e43d0(a: i32);
    fn FUN_004f2990();
    fn FUN_004f0100();
}

unsafe fn c_strlen(p: *const u8) -> usize {
    let mut n = 0usize;
    // SAFETY: p points at a NUL-terminated string.
    unsafe {
        while p.add(n).read() != 0 {
            n += 1;
        }
    }
    n
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004ee450() -> u32 {
    // SAFETY: all globals and callees belong to the game; pointers are
    // the game's own tables and buffers, accessed as the original does.
    unsafe {
        let mut local_80 = [0u8; 128];
        let l80 = local_80.as_mut_ptr();
        let bfba = &raw mut DAT_0081bfba as *mut u32;
        let bfbe = &raw mut DAT_0081bfbe as *mut u8;
        let d402 = &raw mut DAT_0081d402 as *mut u32;
        let d80e = &raw mut DAT_0081d80e as *mut u8;
        let dc0e = &raw mut DAT_0081dc0e as *mut u8;
        let buf = &raw mut DAT_0081b740 as *mut i8;
        let bufu = buf as *mut u8;

        let uvar10_seed = DAT_0085e7c0;
        let mut uvar9: u32 = 0;
        if DAT_00757ccc != 0 {
            let mut p = DAT_00757cc0.wrapping_add(0x140);
            loop {
                let uba = bfba.read_unaligned();
                let utmp = bfbe
                    .byte_add(uba.wrapping_mul(4) as usize)
                    .cast::<u32>()
                    .read_unaligned();
                if (p as *const u32).read_unaligned() == utmp {
                    break;
                }
                p = p.wrapping_add(0x144);
                uvar9 = uvar9.wrapping_add(1);
                if !(uvar9 < DAT_00757ccc) {
                    break;
                }
            }
        }
        let uba = bfba.read_unaligned();
        let name = (&raw const PTR_s_train_005330e0)
            .cast::<*const i8>()
            .byte_add(uba.wrapping_mul(4) as usize)
            .read_unaligned();
        _sprintf(
            l80 as *mut i8,
            (&raw const s__s__s_005331e4).cast::<i8>(),
            name,
            uvar9.wrapping_mul(0x144).wrapping_add(DAT_00757cc0) as *const i8,
        );
        let file = FUN_004be160(l80 as *mut i8, (&raw mut DAT_0052a330).cast());
        if file.is_null() {
            return 0;
        }
        if _fgets(buf, 0xff, file).is_null() {
            FUN_004be1f0(file);
            return 0;
        }
        let uvar5 = _atoi(buf) as u32;
        uvar9 = (&raw mut DAT_0081d326).read_unaligned();
        let uba = bfba.read_unaligned();
        if uba != 2 {
            let utmp = d402.read_unaligned();
            if (&raw mut DAT_0081d40a).byte_add(utmp as usize).read() == 2 {
                let t = d402.read_unaligned().wrapping_sub(1);
                d402.write_unaligned(t);
                let t = d402.read_unaligned();
                uvar9 = d80e
                    .byte_add(t.wrapping_mul(4) as usize)
                    .cast::<u32>()
                    .read_unaligned();
            } else {
                uvar9 = d402.read_unaligned();
                if DAT_0085e7b7 != 0 {
                    _srand(uvar10_seed);
                    loop {
                        let r = _rand() as u32;
                        let mut found = false;
                        let rem: u32;
                        core::arch::asm!(
                            "xor edx, edx",
                            "div {d:e}",
                            d = in(reg) uvar5,
                            inout("eax") r => _,
                            out("edx") rem,
                        );
                        uvar9 = rem;
                        let utmp = d402.read_unaligned();
                        if (utmp.wrapping_sub(1) as i32) >= 0 {
                            let mut p =
                                d80e.byte_add(utmp.wrapping_sub(1).wrapping_mul(4) as usize);
                            loop {
                                if p.cast::<u32>().read_unaligned() == uvar9 {
                                    found = true;
                                    break;
                                }
                                p = p.wrapping_sub(4);
                                if !(0x81d80du32 < p as usize as u32) {
                                    break;
                                }
                            }
                        }
                        if !found {
                            break;
                        }
                    }
                }
            }
            let utmp = d402.read_unaligned();
            d80e.byte_add(utmp.wrapping_mul(4) as usize)
                .cast::<u32>()
                .write_unaligned(uvar9);
        }
        if (uvar9 as i32) >= 0 {
            let mut iv13 = uvar9.wrapping_add(1);
            loop {
                _fgets(buf, 0xff, file);
                let len = c_strlen(bufu);
                let idx = len.wrapping_sub(1);
                if bufu.wrapping_add(idx).read() == b'\n' {
                    bufu.wrapping_add(idx).write(0);
                }
                iv13 = iv13.wrapping_sub(1);
                if iv13 == 0 {
                    break;
                }
            }
        }
        FUN_004be1f0(file);
        if bufu.read() == 0 {
            return 0;
        }
        (&raw mut DAT_0081c012).write_unaligned(1);
        let t = bfba.read_unaligned().wrapping_sub(1);
        bfba.write_unaligned(t);
        let t = bfba.read_unaligned();
        let v = bfbe
            .byte_add(t.wrapping_mul(4) as usize)
            .cast::<u32>()
            .read_unaligned();
        (&raw mut DAT_0081c016).write_unaligned(v);

        let n = c_strlen(bufu) + 1;
        let mut i = 0usize;
        while i < n {
            l80.add(i).write(bufu.add(i).read());
            i += 1;
        }

        let len = c_strlen(l80);
        if len > 0 {
            let mut i: usize = 0;
            loop {
                let r = _tolower(l80.add(i).read() as i32);
                l80.add(i).write(r as u8);
                let len = c_strlen(l80);
                i += 1;
                if !((i as i32) < len as i32) {
                    break;
                }
            }
        }

        let mut iv7: i32 = 0;
        FUN_004e43d0(bfba.read_unaligned() as i32);
        let mut iv13: u32 = 0;
        if 0 < DAT_00757ccc as i32 {
            loop {
                let other = DAT_00757cc0.wrapping_add(iv13) as *const u8;
                let mut k = 0usize;
                let equal = loop {
                    let a = l80.add(k).read();
                    let b = other.add(k).read();
                    if a != b {
                        break false;
                    }
                    if a == 0 {
                        break true;
                    }
                    k += 1;
                };
                if equal {
                    let p1 = (iv7 as u32)
                        .wrapping_mul(0x144)
                        .wrapping_add(0x140)
                        .wrapping_add(DAT_00757cc0) as *const u32;
                    let uba = bfba.read_unaligned();
                    let v = p1.read_unaligned();
                    bfbe.byte_add(uba.wrapping_mul(4) as usize)
                        .cast::<u32>()
                        .write_unaligned(v);
                    let uba = bfba.read_unaligned();
                    if uba == 3 {
                        let v = p1.read_unaligned();
                        (&raw mut DAT_0081d406).write_unaligned(v);
                    }
                    break;
                }
                iv13 = iv13.wrapping_add(0x144);
                iv7 = iv7.wrapping_add(1);
                if !(iv7 < DAT_00757ccc as i32) {
                    break;
                }
            }
        }
        if DAT_00757cc0 != 0 {
            DAT_00757cc8 = 0;
            if DAT_00757ccc != 0 {
                let mut p = DAT_00757cc0.wrapping_add(0x140);
                loop {
                    let uba = bfba.read_unaligned();
                    let utmp = bfbe
                        .byte_add(uba.wrapping_mul(4) as usize)
                        .cast::<u32>()
                        .read_unaligned();
                    if (p as *const u32).read_unaligned() == utmp {
                        let t2 = d402.read_unaligned();
                        dc0e.byte_add(t2.wrapping_mul(4) as usize)
                            .cast::<u32>()
                            .write_unaligned(DAT_00757cc8);
                        break;
                    }
                    p = p.wrapping_add(0x144);
                    DAT_00757cc8 = DAT_00757cc8.wrapping_add(1);
                    if !(DAT_00757cc8 < DAT_00757ccc) {
                        break;
                    }
                }
            }
        }
        FUN_004f2990();
        FUN_004f0100();
        1
    }
}
