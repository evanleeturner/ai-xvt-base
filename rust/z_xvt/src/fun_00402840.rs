// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00536820: [i8; 1];
    static mut DAT_00523570: [i8; 1];
    static mut DAT_0052356c: [i8; 1];
    static mut DAT_00732948: *mut core::ffi::c_void;
    static mut s__256s_00523564: [i8; 1];
    static mut s__256s_00523550: [i8; 1];
    static mut s__inventor_00523558: [i8; 1];
    static mut s_ascii_00523548: [i8; 1];
    static mut s_binary_00523540: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0047ed40(param_1: *mut i8, param_2: *mut i8, param_3: i32, param_4: i32);
    fn _fscanf(file: *mut core::ffi::c_void, fmt: *const i8, ...) -> i32;
    fn __strnicmp(a: *const i8, b: *const i8, n: u32) -> i32;
    fn FUN_00402a80(file: *mut core::ffi::c_void) -> i16;
    fn _fclose(file: *mut core::ffi::c_void) -> i32;
    fn FUN_0048be40(param_1: u32) -> u16;
    fn FUN_0048aa90(param_1: *mut i8, param_2: u32);
    fn FUN_0048a570(param_1: u32) -> i16;
    fn FUN_00488490(param_1: *mut i8) -> u16;
    fn FUN_00438dd0();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00402840(param_1: *mut i8) -> i16 {
    // SAFETY: the globals, callees and libc routines are the game's own;
    // param_1 is a caller NUL-terminated string and the game's buffer
    // DAT_00536820 is large enough, as in the original.
    unsafe {
        let buf = &raw mut DAT_00536820 as *mut i8;
        let ext = &raw mut DAT_00523570 as *mut i8;
        let suffix = &raw mut DAT_0052356c as *mut i8;

        // strcpy(buf, param_1)
        let mut n: usize = 0;
        while *param_1.add(n) != 0 {
            n += 1;
        }
        let mut i: usize = 0;
        while i <= n {
            *buf.add(i) = *param_1.add(i);
            i += 1;
        }

        FUN_0047ed40(buf, ext, 0, 0);

        if DAT_00732948 as usize != 0 {
            _fclose(DAT_00732948);
            let u_var3 = FUN_00488490(buf);
            FUN_00438dd0();
            let s_var4 = FUN_0048a570(u_var3 as u32);
            FUN_00438dd0();
            return s_var4;
        }

        let mut p = buf;
        while *p != b'.' as i8 {
            p = p.add(1);
        }
        *p = 0;

        // strcat(buf, suffix)
        let mut sl: usize = 0;
        while *suffix.add(sl) != 0 {
            sl += 1;
        }
        let mut dl: usize = 0;
        while *buf.add(dl) != 0 {
            dl += 1;
        }
        let mut i: usize = 0;
        while i <= sl {
            *buf.add(dl + i) = *suffix.add(i);
            i += 1;
        }

        FUN_0047ed40(buf, ext, 1, 0);
        let file = DAT_00732948;
        if file as usize == 0 {
            return 0;
        }
        let fmt1 = &raw const s__256s_00523564 as *const i8;
        let fmt2 = &raw const s__256s_00523550 as *const i8;
        if _fscanf(file, fmt1, buf) != 1 {
            return 0;
        }
        if __strnicmp(buf, &raw const s__inventor_00523558 as *const i8, 9) != 0 {
            return 0;
        }
        if _fscanf(file, fmt2, buf) != 1 {
            return 0;
        }
        if _fscanf(file, fmt2, buf) != 1 {
            return 0;
        }
        let u_var6: u32;
        if __strnicmp(buf, &raw const s_ascii_00523548 as *const i8, 5) == 0 {
            u_var6 = FUN_00402a80(file) as u16 as u32;
        } else {
            if __strnicmp(buf, &raw const s_binary_00523540 as *const i8, 6) != 0 {
                _fclose(file);
                return 0;
            }
            u_var6 = 0;
        }
        _fclose(file);
        let u_var3 = FUN_0048be40(u_var6);

        // strcpy(buf, param_1)
        let mut n: usize = 0;
        while *param_1.add(n) != 0 {
            n += 1;
        }
        let mut i: usize = 0;
        while i <= n {
            *buf.add(i) = *param_1.add(i);
            i += 1;
        }
        FUN_0048aa90(buf, u_var3 as u32);
        FUN_0048a570(u_var3 as u32)
    }
}
