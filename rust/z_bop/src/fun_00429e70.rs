// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals, non_snake_case)]
unsafe extern "C" {
    static mut DAT_005233ac: i32;
    static mut DAT_005233d8: i32;
    static mut DAT_00520ec4: *mut i32;
    static mut DAT_009a7ed0: i16;
    static mut DAT_00a07bd8: *mut core::ffi::c_void;
    static mut DAT_00520ee0: [i8; 1];
    static mut DAT_0051a854: [i8; 1];
    static mut DAT_0051bd9c: [i8; 1];
    static mut DAT_00555cc8: [i8; 1];
    static mut _DAT_00520ed8: f64;
    static mut _DAT_005561e0: f64;
    static mut _DAT_005180f8: f64;
    static mut DAT_009ec476: u8;
    static mut DAT_00520ec8: i32;
    static mut DAT_00556218: [u32; 1];
    static mut DAT_00555d78: [u32; 1];
    static mut DAT_005561eb: u8;
    static mut DAT_005561ec: i32;
    static mut DAT_005561fa: i16;
    static mut DAT_005561f0: i32;
    static mut DAT_005561f8: i16;
    static mut DAT_005561f4: i32;
    static mut DAT_005561fc: i16;
    static mut DAT_00520ec0: i16;
    static mut DAT_00556207: [u8; 4];
    static mut _DAT_005562c1: [u8; 4];

    fn FUN_0042a350();
    fn FUN_0042af90();
    fn FUN_0049e720(param_1: *mut i8, param_2: *mut i8, param_3: i32, param_4: i32) -> u32;
    fn _fscanf(param_1: *mut core::ffi::c_void, param_2: *const i8, ...) -> i32;
    fn _fclose(param_1: *mut core::ffi::c_void) -> i32;
    fn __strnicmp(param_1: *const i8, param_2: *const i8, param_3: u32) -> i32;
    fn FUN_004ac6e0(param_1: u32);
    fn FUN_00412040(param_1: *mut core::ffi::c_void) -> i16;
    fn FUN_00412030() -> i16;
    fn FUN_00477950(param_1: i16) -> i16;
    fn FUN_004766c0(param_1: *mut i8, param_2: i16);
    fn FUN_004742a0(param_1: *mut i8) -> u32;
    fn FUN_004762f0(param_1: i16) -> i16;
    fn FUN_004ac720(param_1: i16) -> *mut i32;
    fn FUN_00471ff0(param_1: *mut i32);
    fn FUN_0042ae30(param_1: i32, param_2: i32) -> f64;
    fn FUN_0042ac50(param_1: i32, param_2: f64);
    fn FUN_0042b010(param_1: i32, param_2: i32, param_3: i32);
}

unsafe fn z_strlen(mut p: *const i8) -> usize {
    let mut n = 0usize;
    // SAFETY: p points at a NUL-terminated string.
    unsafe {
        while *p != 0 {
            p = p.add(1);
            n += 1;
        }
    }
    n
}

unsafe fn z_strcpy(dst: *mut i8, src: *const i8) {
    // SAFETY: src is NUL-terminated and dst is large enough, as in the original.
    unsafe {
        let n = z_strlen(src) + 1;
        let mut i = 0usize;
        while i < n {
            *dst.add(i) = *src.add(i);
            i += 1;
        }
    }
}

unsafe fn z_strcat(dst: *mut i8, src: *const i8) {
    // SAFETY: both are NUL-terminated and dst has room, as in the original.
    unsafe {
        let n = z_strlen(dst);
        z_strcpy(dst.add(n), src);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00429e70(param_1: *mut i8) -> u32 {
    let mut local_200 = [0i8; 256];
    let mut local_100 = [0i8; 256];
    // SAFETY: the callees and globals are the game's own; the buffers are
    // local and sized as in the original.
    unsafe {
        let l200 = local_200.as_mut_ptr();
        let l100 = local_100.as_mut_ptr();
        let ext = (&raw mut DAT_00520ee0).cast::<i8>();
        let a854 = (&raw mut DAT_0051a854).cast::<i8>();
        let b9c = (&raw mut DAT_0051bd9c).cast::<i8>();
        let c8 = (&raw mut DAT_00555cc8).cast::<i8>();

        FUN_0042a350();
        FUN_0042af90();
        DAT_005233ac = 1;
        if DAT_00520ec4.is_null() {
            DAT_009a7ed0 = 0;
        }
        z_strcpy(l100, param_1);
        let mut p = l100;
        while *p != b'.' as i8 {
            p = p.add(1);
        }
        *p = 0;
        z_strcpy(l200, l100);
        z_strcat(l200, ext);
        FUN_0049e720(l200, a854, 0, 0);
        if DAT_00a07bd8.is_null() {
            z_strcpy(l200, l100);
            z_strcat(l200, b9c);
            FUN_0049e720(l200, a854, 0, 0);
            let file = DAT_00a07bd8;
            if file.is_null() {
                return 0;
            }
            if _fscanf(file, c"%256s".as_ptr(), l200) != 1 {
                return 0;
            }
            if __strnicmp(l200, c"#inventor".as_ptr(), 9) != 0 {
                return 0;
            }
            if _fscanf(file, c" %256s".as_ptr(), l200) != 1 {
                return 0;
            }
            if _fscanf(file, c" %256s".as_ptr(), l200) != 1 {
                return 0;
            }
            let u2: i16;
            if __strnicmp(l200, c"ascii".as_ptr(), 5) == 0 {
                if DAT_009a7ed0 != 0 {
                    FUN_004ac6e0(DAT_009a7ed0 as i32 as u32);
                }
                u2 = FUN_00412040(file);
            } else {
                if __strnicmp(l200, c"binary".as_ptr(), 6) != 0 {
                    _fclose(file);
                    return 0;
                }
                if DAT_009a7ed0 != 0 {
                    FUN_004ac6e0(DAT_009a7ed0 as i32 as u32);
                }
                u2 = FUN_00412030();
            }
            _fclose(file);
            let s3 = FUN_00477950(u2);
            z_strcpy(l200, l100);
            z_strcat(l200, ext);
            FUN_004766c0(l200, s3);
            DAT_009a7ed0 = s3;
        } else {
            _fclose(DAT_00a07bd8);
            if DAT_009a7ed0 != 0 {
                FUN_004ac6e0(DAT_009a7ed0 as i32 as u32);
            }
            let r = FUN_004742a0(l200);
            DAT_009a7ed0 = r as u16 as i16;
        }
        z_strcpy(c8, l100);
        z_strcat(c8, ext);
        if DAT_005233ac != 0 {
            DAT_005233d8 = 2;
            DAT_009a7ed0 = FUN_004762f0(DAT_009a7ed0);
            DAT_005233d8 = 2;
        }
        DAT_00520ec4 = FUN_004ac720(DAT_009a7ed0);
        if DAT_00520ec4.read() != DAT_00520ec4 as i32 {
            FUN_00471ff0(DAT_00520ec4);
        }
        let f11 = FUN_0042ae30(DAT_00520ec4 as i32, 0);
        _DAT_00520ed8 = f11;
        _DAT_005561e0 = _DAT_005180f8 / f11;
        FUN_0042ac50(DAT_00520ec4 as i32, _DAT_005561e0);
        DAT_009ec476 = 1;
        if DAT_00520ec8 == 0 {
            let a = (&raw mut DAT_00556218).cast::<u32>();
            let mut i = 0usize;
            while i < 0x2c {
                a.add(i).write(0);
                i += 1;
            }
            a.add(0x2c).cast::<u8>().write(0);
            let b = (&raw mut DAT_00555d78).cast::<u32>();
            let mut j = 0usize;
            while j < 0x118 {
                b.add(j).write(0);
                j += 1;
            }
            b.add(0x118).cast::<u16>().write(0);
            DAT_005561eb = 0;
            DAT_005561ec = 0;
            DAT_005561fa = 0;
            DAT_005561f0 = 0;
            DAT_005561f8 = 0;
            DAT_005561f4 = 0;
            (&raw mut DAT_00556207)
                .cast::<u32>()
                .write_unaligned(a as usize as u32);
            (&raw mut _DAT_005562c1)
                .cast::<u32>()
                .write_unaligned(b as usize as u32);
            DAT_005561fc = 0;
            DAT_00520ec0 = 0;
            FUN_0042af90();
            FUN_0042b010(0, 0, 0);
        }
        1
    }
}
