// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00665a90: [u8; 320];
    static mut DAT_00665abc: [i8; 260];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __getcwd(buf: *mut i8, size: i32) -> *mut i8;
    fn _malloc(size: u32) -> *mut c_void;
    fn __chdir(path: *const i8) -> i32;
    fn _free(p: *mut c_void);
    fn FUN_004dfc70(param_1: *mut u32, param_2: *mut u32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FindFirstFileA(lpFileName: *const i8, lpFindFileData: *mut c_void) -> *mut c_void;
    fn FindNextFileA(hFindFile: *mut c_void, lpFindFileData: *mut c_void) -> i32;
    fn FindClose(hFindFile: *mut c_void) -> i32;
}

/// Allocates a copy of the current file name and stores it in `*node`.
/// Returns false if the allocation failed.
unsafe fn dup_name(node: *mut u32) -> bool {
    // SAFETY: DAT_00665abc is the game's find-data file name buffer, NUL
    // terminated by the OS; node points at a live 8-byte allocation.
    unsafe {
        let src = (&raw mut DAT_00665abc).cast::<u8>();
        let mut n: u32 = 0;
        while *src.add(n as usize) != 0 {
            n = n.wrapping_add(1);
        }
        n = n.wrapping_add(1);
        let dst = _malloc(n) as *mut u8;
        node.write_unaligned(dst as usize as u32);
        if dst.is_null() {
            return false;
        }
        let mut i: u32 = 0;
        while i < n {
            *dst.add(i as usize) = *src.add(i as usize);
            i = i.wrapping_add(1);
        }
        true
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004dfa10(param_1: *const i8) -> *mut u32 {
    let mut local_100 = [0i8; 256];
    // SAFETY: the callees and globals are the game's own; all pointers are
    // either freshly allocated by the game's malloc or the local buffer.
    unsafe {
        __getcwd(local_100.as_mut_ptr(), 0x100);
        let memory = _malloc(8) as *mut u32;
        if memory.is_null() {
            __chdir(local_100.as_ptr());
            return core::ptr::null_mut();
        }
        let h = FindFirstFileA(param_1, (&raw mut DAT_00665a90).cast::<c_void>());
        if h as usize == 0xffff_ffff {
            __chdir(local_100.as_ptr());
            memory.write_unaligned(0);
            memory.add(1).write_unaligned(0);
            return memory;
        }
        let first = _malloc(8) as *mut u32;
        if first.is_null() {
            __chdir(local_100.as_ptr());
            _free(memory as *mut c_void);
            FindClose(h);
            return core::ptr::null_mut();
        }
        if !dup_name(first) {
            __chdir(local_100.as_ptr());
            _free(first as *mut c_void);
            _free(memory as *mut c_void);
            FindClose(h);
            return core::ptr::null_mut();
        }
        first.add(1).write_unaligned(0);
        memory.write_unaligned(first as usize as u32);
        memory.add(1).write_unaligned(1);
        let mut r = FindNextFileA(h, (&raw mut DAT_00665a90).cast::<c_void>());
        loop {
            if r == 0 {
                break;
            }
            let node = _malloc(8) as *mut u32;
            if node.is_null() {
                break;
            }
            if !dup_name(node) {
                _free(node as *mut c_void);
                break;
            }
            node.add(1).write_unaligned(0);
            FUN_004dfc70(memory, node);
            r = FindNextFileA(h, (&raw mut DAT_00665a90).cast::<c_void>());
        }
        __chdir(local_100.as_ptr());
        FindClose(h);
        memory
    }
}
