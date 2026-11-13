// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00528100: i32;
    static mut DAT_000b0000: [i8; 4000];
    static mut DAT_00528114: i32;
    static mut s_mpDump_txt_0052811c: [i8; 1];
    static mut DAT_005240e0: [i8; 1];
    static mut DAT_00528118: [i8; 1];
    static mut DAT_00528104: i32;
    static mut DAT_00528108: i32;
    static mut DAT_00528110: i32;
    static mut DAT_0052810c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FID_conflict___wfopen(filename: *const i8, mode: *const i8) -> *mut core::ffi::c_void;
    fn _fprintf(stream: *mut core::ffi::c_void, format: *const i8, ...) -> i32;
    fn _fclose(stream: *mut core::ffi::c_void) -> i32;
    fn FID_conflict__memcpy(
        dest: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        count: u32,
    ) -> *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004acc30(param_1: *mut i8) {
    // SAFETY: all globals, the screen buffer at 0xb0000 and the callees are the
    // game's own memory; param_1 is a NUL-terminated string from the caller.
    unsafe {
        let base = (&raw mut DAT_000b0000).cast::<i8>();
        let mut param_1 = param_1;

        if DAT_00528100 == 0 {
            let mut p = base;
            let mut n: i32 = 2000;
            loop {
                p.write(0x20);
                p.add(1).write(7);
                p = p.add(2);
                n = n.wrapping_sub(1);
                if n == 0 {
                    break;
                }
            }
            DAT_00528100 = 1;
        }
        if DAT_00528114 != 0 {
            let f = FID_conflict___wfopen(
                (&raw mut s_mpDump_txt_0052811c).cast::<i8>(),
                (&raw mut DAT_005240e0).cast::<i8>(),
            );
            if !f.is_null() {
                _fprintf(f, (&raw mut DAT_00528118).cast::<i8>(), param_1);
                _fclose(f);
            }
        }
        if param_1.read() == 10 {
            param_1 = param_1.add(1);
            DAT_00528104 = 0;
            DAT_00528108 = DAT_00528108.wrapping_add(1);
        }
        loop {
            let mut len: i32 = 0;
            while param_1.offset(len as isize).read() != 0 {
                len = len.wrapping_add(1);
            }
            let mut ivar2: i32 = len;
            if DAT_00528110 < DAT_00528108 {
                FID_conflict__memcpy(
                    base.offset(DAT_0052810c.wrapping_mul(0xa0) as isize).cast(),
                    (DAT_0052810c.wrapping_mul(0xa0).wrapping_add(0xb00a0)) as usize
                        as *const core::ffi::c_void,
                    DAT_00528110.wrapping_sub(DAT_0052810c).wrapping_mul(0xa0) as u32,
                );
                let mut n: i32 = 0x50;
                let mut p = base.offset(DAT_00528110.wrapping_mul(0xa0) as isize);
                loop {
                    p.write(0x20);
                    p = p.add(2);
                    n = n.wrapping_sub(1);
                    if n == 0 {
                        break;
                    }
                }
                DAT_00528108 = DAT_00528110;
            }
            let mut ivar3: i32 = ivar2;
            if 0x50 < DAT_00528104.wrapping_add(ivar2) {
                ivar3 = 0x50i32.wrapping_sub(DAT_00528104);
            }
            let ivar4: i32 = DAT_00528108.wrapping_mul(0x50).wrapping_add(DAT_00528104);
            let mut ivar7: i32 = 0;
            if 0 < ivar3 {
                loop {
                    let c = param_1.offset(ivar7 as isize).read();
                    if c == 10 {
                        param_1 = param_1.offset(ivar7.wrapping_add(1) as isize);
                        DAT_00528108 = DAT_00528108.wrapping_add(1);
                        ivar2 = ivar2.wrapping_sub(ivar7.wrapping_add(1));
                        ivar3 = 0;
                        DAT_00528104 = 0;
                        break;
                    }
                    base.offset(ivar7.wrapping_mul(2).wrapping_add(ivar4.wrapping_mul(2)) as isize)
                        .write(c);
                    ivar7 = ivar7.wrapping_add(1);
                    if ivar7 >= ivar3 {
                        break;
                    }
                }
            }
            DAT_00528104 = DAT_00528104.wrapping_add(ivar3);
            if ivar3 == ivar2 {
                return;
            }
            param_1 = param_1.offset(ivar3 as isize);
            DAT_00528108 = DAT_00528108.wrapping_add(1);
            DAT_00528104 = 0;
        }
    }
}
