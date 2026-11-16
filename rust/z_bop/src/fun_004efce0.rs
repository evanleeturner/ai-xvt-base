// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052c850: u8;
    static mut DAT_0052c848: u8;
    static mut DAT_0051a854: u8;
    static mut DAT_0052c83c: u32;
    static mut DAT_0052c840: u32;
    static mut DAT_0052c844: u16;
    static mut DAT_0052c846: u8;
    static mut DAT_0052c858: u32;
    static mut DAT_0052c85c: u32;
    static mut DAT_00b69e30: i32;
    static mut DAT_00b69cb0: i32;
    static mut DAT_00b69cbc: i32;
    static mut DAT_00aa7b36: [u8; 4];
    static mut DAT_00aa7b3a: [u8; 4];
    static mut DAT_00aa7b32: [u8; 4];
    static mut DAT_00aa7fb7: [u8; 4];
    static mut DAT_00aa7b26: [u8; 4];
    static mut DAT_00aa7b9a: [u8; 4];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FID_conflict___wfopen(_Filename: *mut i8, _Mode: *mut i8) -> *mut core::ffi::c_void;
    fn FUN_004ccb90() -> i8;
    fn FUN_004d4f60();
    fn FUN_004d4f70();
    fn FUN_004d4c10();
    fn FUN_004ddc10(param_1: i32) -> *mut i8;
    fn FUN_004dcb90(a: *mut i8, b: *mut i8, c: *mut i8, d: *mut i8, e: *mut i8) -> i32;
    fn _fclose(_File: *mut core::ffi::c_void);
    fn FUN_004f07d0();
    fn FUN_004f0070();
    fn FUN_004f0140();
    fn FUN_004ef100(param_1: *mut u32) -> u8;
}

unsafe fn str_cat(dst: *mut u8, src: *const u8) {
    // SAFETY: both point at nul-terminated buffers owned by the caller; the
    // destination has room for the appended string.
    unsafe {
        let mut d = dst;
        while *d != 0 {
            d = d.add(1);
        }
        let mut s = src;
        loop {
            let c = *s;
            *d = c;
            if c == 0 {
                break;
            }
            s = s.add(1);
            d = d.add(1);
        }
    }
}

unsafe fn build_fallback(path: *mut u8, name: *const u8) {
    // SAFETY: path is the caller's 0x80-byte buffer; the globals are the
    // game's own memory.
    unsafe {
        path.cast::<u32>().write_unaligned(DAT_0052c83c);
        path.add(4).cast::<u32>().write_unaligned(DAT_0052c840);
        path.add(8).cast::<u16>().write_unaligned(DAT_0052c844);
        path.add(10).write(DAT_0052c846);
        let c = FUN_004ccb90();
        path.write(c as u8);
        str_cat(path, name);
        str_cat(path, &raw mut DAT_0052c848 as *const u8);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004efce0(param_1: *mut i8, param_2: i32) -> u8 {
    let mut local_1b4: [u32; 12] = [0; 12];
    let mut local_180: [u8; 0x80] = [0; 0x80];
    let mut local_100: [u32; 64] = [0; 64];
    // SAFETY: the callees and globals are the game's own; param_1 is the
    // caller's nul-terminated string, copied into the local buffer.
    unsafe {
        let name = local_100.as_mut_ptr().cast::<u8>();
        let path = local_180.as_mut_ptr();
        let mut s = param_1 as *const u8;
        let mut d = name;
        loop {
            let c = *s;
            *d = c;
            if c == 0 {
                break;
            }
            s = s.add(1);
            d = d.add(1);
        }
        path.cast::<u32>()
            .write_unaligned((&raw mut DAT_0052c850).cast::<u32>().read_unaligned());
        path.add(4).cast::<u32>().write_unaligned(
            (&raw mut DAT_0052c850)
                .cast::<u8>()
                .add(4)
                .cast::<u32>()
                .read_unaligned(),
        );
        str_cat(path, name);
        str_cat(path, &raw mut DAT_0052c848 as *const u8);
        let mut file =
            FID_conflict___wfopen(path.cast::<i8>(), (&raw mut DAT_0051a854).cast::<i8>());
        let mut skip_close = false;
        if param_2 == 0 || DAT_00b69e30 == 2 {
            if file.is_null() {
                build_fallback(path, name);
                loop {
                    file = FID_conflict___wfopen(
                        path.cast::<i8>(),
                        (&raw mut DAT_0051a854).cast::<i8>(),
                    );
                    if !file.is_null() {
                        break;
                    }
                    FUN_004d4f60();
                    let p3 = FUN_004ddc10(0x13);
                    let p4 = FUN_004ddc10(0x20b);
                    let p5 = FUN_004ddc10(0x33d);
                    let p6 = FUN_004ddc10(0x33c);
                    let p7 = FUN_004ddc10(0x33b);
                    let r = FUN_004dcb90(p7, p6, p5, p4, p3);
                    if r == 0 {
                        FUN_004d4f70();
                        FUN_004d4c10();
                        return 2;
                    }
                    FUN_004d4f70();
                    FUN_004d4c10();
                }
            }
        } else if file.is_null() {
            build_fallback(path, name);
            file = FID_conflict___wfopen(path.cast::<i8>(), (&raw mut DAT_0051a854).cast::<i8>());
            if file.is_null() {
                local_100[0] = DAT_0052c858;
                local_100[1] = DAT_0052c85c;
                skip_close = true;
            }
        }
        if !skip_close {
            _fclose(file);
        }
        local_1b4[0] = local_100.as_mut_ptr() as u32;
        local_1b4[3] = (&raw mut DAT_00aa7b36).cast::<u32>().read_unaligned();
        local_1b4[1] = 0x280;
        local_1b4[2] = 0x1e0;
        local_1b4[4] = (&raw mut DAT_00aa7b3a).cast::<u32>().read_unaligned();
        if DAT_00b69cb0 == 0 && DAT_00b69cbc == 0 {
            local_1b4[4] = (&raw mut DAT_00aa7b32).cast::<u32>().read_unaligned();
        }
        local_1b4[6] = (&raw mut DAT_00aa7fb7).cast::<u32>().read_unaligned();
        local_1b4[8] = (&raw mut DAT_00aa7b26).cast::<u32>().read_unaligned();
        local_1b4[5] = (&raw mut DAT_00aa7b9a).cast::<u32>().read_unaligned();
        if param_2 == 0 || DAT_00b69e30 == 2 {
            local_1b4[9] = FUN_004f0070 as usize as u32;
            local_1b4[11] = 0;
        } else {
            local_1b4[9] = FUN_004f0140 as usize as u32;
            local_1b4[11] = FUN_004f07d0 as usize as u32;
        }
        FUN_004ef100(local_1b4.as_mut_ptr())
    }
}
