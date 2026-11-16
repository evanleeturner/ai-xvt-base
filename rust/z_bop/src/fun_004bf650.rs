// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b6a2e0: [i8; 1];
    static mut DAT_00b69d20: [i8; 1];
    static mut DAT_0051a854: [i8; 1];
    static mut DAT_00527004: [i8; 1];
    static mut DAT_0052b398: [i8; 1];
    static mut DAT_0052b35c: [i8; 1];
    static mut DAT_0052b388: [i8; 1];
    static mut DAT_0052b380: [i8; 1];
    static mut DAT_0052b378: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dfa10(param_1: *mut i8) -> *mut u32;
    fn FUN_004cc4a0(param_1: *mut i8, param_2: *mut i8) -> *mut core::ffi::c_void;
    fn FUN_004cc6c0(param_1: *mut core::ffi::c_void, param_2: *mut core::ffi::c_void, param_3: i32);
    fn FUN_004cc590(param_1: *mut core::ffi::c_void);
    fn __strcmpi(param_1: *const i8, param_2: *const i8) -> i32;
    fn FUN_004dfc30(param_1: *mut u32);
    fn FUN_004ccf50();
    fn FUN_004ccf70();
    fn _sprintf(param_1: *mut i8, param_2: *const i8, ...) -> i32;
    fn FUN_004cc790(param_1: *mut core::ffi::c_void, param_2: *mut core::ffi::c_void, param_3: i32);
    fn FUN_004cb310(param_1: *mut i8, param_2: *mut core::ffi::c_void);
}

unsafe fn copy_cstr(dst: *mut i8, src: *const i8) {
    let mut i = 0usize;
    // SAFETY: src is a NUL-terminated string from the game; dst is the caller's buffer.
    unsafe {
        loop {
            let c = *src.add(i);
            *dst.add(i) = c;
            i += 1;
            if c == 0 {
                break;
            }
        }
    }
}

unsafe fn str_len(s: *const i8) -> usize {
    let mut i = 0usize;
    // SAFETY: s is NUL-terminated.
    unsafe {
        while *s.add(i) != 0 {
            i += 1;
        }
    }
    i
}

unsafe fn scan_list(puvar2: *mut u32, local: *mut i8) {
    // SAFETY: the list and the globals are the game's own memory.
    unsafe {
        let mut node = puvar2.read() as *mut u32;
        let mut i: i32 = 0;
        if 0 < (puvar2.add(1).read() as i32) {
            loop {
                let f = FUN_004cc4a0(node.read() as *mut i8, &raw mut DAT_0051a854 as *mut i8);
                if !f.is_null() {
                    FUN_004cc6c0(f, &raw mut DAT_00b69d20 as *mut core::ffi::c_void, 0xe);
                    FUN_004cc590(f);
                    let r = __strcmpi(
                        &raw mut DAT_00b69d20 as *const i8,
                        &raw mut DAT_00b6a2e0 as *const i8,
                    );
                    if r == 0 {
                        copy_cstr(local, node.read() as *const i8);
                        return;
                    }
                }
                node = node.add(1).read() as *mut u32;
                i = i.wrapping_add(1);
                if !(i < (puvar2.add(1).read() as i32)) {
                    break;
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004bf650(param_1: i32) -> u32 {
    let mut local_20: [i8; 30] = [0; 30];
    let local = local_20.as_mut_ptr();
    // SAFETY: globals and callees are the game's own; local is a 30-byte buffer.
    unsafe {
        if *(&raw mut DAT_00b6a2e0 as *const i8) == 0 {
            return 0;
        }
        if param_1 == 0 {
            let p2 = FUN_004dfa10(&raw mut DAT_0052b398 as *mut i8);
            if !p2.is_null() {
                scan_list(p2, local);
                FUN_004dfc30(p2);
            }
        } else {
            let src = &raw mut DAT_0052b388 as *const i8;
            let mut k = 0usize;
            while k < 13 {
                *local.add(k) = *src.add(k);
                k += 1;
            }
        }
        if *local == 0 {
            FUN_004ccf50();
            let p2 = FUN_004dfa10(&raw mut DAT_0052b35c as *mut i8);
            FUN_004ccf70();
            if !p2.is_null() {
                scan_list(p2, local);
                FUN_004dfc30(p2);
            }
            if *local == 0 {
                _sprintf(
                    local,
                    &raw mut DAT_0052b380 as *const i8,
                    &raw mut DAT_00b6a2e0 as *const i8,
                );
            } else {
                let n = str_len(local);
                *local.add(n.wrapping_sub(1)) = 0x32;
            }
        }
        let f = FUN_004cc4a0(local, &raw mut DAT_00527004 as *mut i8);
        if f.is_null() {
            return 0;
        }
        FUN_004cc790(f, &raw mut DAT_00b6a2e0 as *mut core::ffi::c_void, 0x4852e);
        FUN_004cc590(f);
        if *local == 0 {
            _sprintf(
                local,
                &raw mut DAT_0052b378 as *const i8,
                &raw mut DAT_00b6a2e0 as *const i8,
            );
        } else {
            let n = str_len(local);
            *local.add(n.wrapping_sub(1)) = 0x74;
        }
        FUN_004cb310(local, f);
        1
    }
}
