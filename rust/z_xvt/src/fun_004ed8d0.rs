// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00757c78: *mut core::ffi::c_void;
    static mut DAT_00533790: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    #[link_name = "fscanf"]
    fn _fscanf(stream: *mut core::ffi::c_void, format: *const i8, ...) -> i32;
    #[link_name = "fgets"]
    fn _fgets(buffer: *mut i8, count: i32, stream: *mut core::ffi::c_void) -> *mut i8;
}

/// Index of the last byte before the terminator (wraps for an empty string).
unsafe fn last_index_ptr(p: *mut i8) -> *mut i8 {
    let mut len: isize = 0;
    // SAFETY: p points at a NUL-terminated string.
    unsafe {
        while *p.offset(len) != 0 {
            len += 1;
        }
        p.wrapping_offset(len - 1)
    }
}

unsafe fn strip_newline(p: *mut i8) {
    // SAFETY: p points at a NUL-terminated string, as in the original.
    unsafe {
        let q = last_index_ptr(p);
        if *q == 10 {
            let q2 = last_index_ptr(p);
            *q2 = 0;
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004ed8d0(param_1: *mut i8, param_2: *mut i8, param_3: *mut i8) -> u32 {
    let mut local_4: u32 = 0;
    // SAFETY: globals and CRT functions are the game's own; the params point
    // at caller buffers of at least 0x100 bytes.
    unsafe {
        if DAT_00757c78.is_null() {
            return 0;
        }
        let i_var2 = _fscanf(
            DAT_00757c78,
            (&raw mut DAT_00533790).cast::<i8>() as *const i8,
            &raw mut local_4,
        );
        if i_var2 == 0 || i_var2 == -1 {
            *param_1 = 0;
            return 0xffff;
        }
        let p = _fgets(param_1, 0x100, DAT_00757c78);
        if p.is_null() {
            *param_1 = 0;
        } else {
            strip_newline(param_1);
            if *param_1 == b'.' as i8 {
                *param_1 = 0;
            }
        }
        let p = _fgets(param_2, 0x100, DAT_00757c78);
        if p.is_null() {
            *param_2 = 0;
        } else {
            strip_newline(param_2);
            if *param_2 == b'.' as i8 {
                *param_2 = 0;
            }
        }
        let p = _fgets(param_3, 0x100, DAT_00757c78);
        if p.is_null() {
            *param_3 = 0;
            return local_4;
        }
        strip_newline(param_3);
        if *param_3 == b'.' as i8 {
            *param_3 = 0;
        }
        local_4
    }
}
