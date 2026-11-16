// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005366d4: u16;
    static mut DAT_006550d0: u16;
    static mut DAT_007427a0: [u8; 24];
    static mut DAT_00742668: *mut i32;
    static mut DAT_00742724: *const core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn EnterCriticalSection(lpCriticalSection: *mut core::ffi::c_void);
    fn LeaveCriticalSection(lpCriticalSection: *mut core::ffi::c_void);
}

/// Returns `~uVar2` after the C's scan loop (bytes scanned, terminator included).
unsafe fn scan(start: *const i32) -> u32 {
    let mut u_var2: u32 = 0xffff_ffff;
    let mut p = start as *const u8;
    loop {
        if u_var2 == 0 {
            break;
        }
        u_var2 = u_var2.wrapping_sub(1);
        // SAFETY: the game's own NUL-terminated string is read byte by byte.
        let c = unsafe { p.read() };
        p = p.wrapping_add(1);
        if c == 0 {
            break;
        }
    }
    !u_var2
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0051a870() {
    // SAFETY: the globals, the critical section and the callback are the
    // game's own; the pointer walks the game's own buffer as the original does.
    unsafe {
        let u_var2: u32 = DAT_005366d4 as u32;
        let u_var3: u32 = DAT_006550d0 as u32;
        EnterCriticalSection((&raw mut DAT_007427a0).cast::<core::ffi::c_void>());
        if !DAT_00742668.is_null() && (DAT_00742668.read() as u32) == u_var3.wrapping_sub(u_var2) {
            DAT_00742668 = DAT_00742668.wrapping_add(1);
            let n = scan(DAT_00742668);
            if !DAT_00742724.is_null() {
                let f: unsafe extern "C" fn(*mut i32, i32, i32, i32, u32, u32) =
                    core::mem::transmute(DAT_00742724);
                let base = DAT_00742668;
                let a = base.byte_add(n as usize).cast::<u32>().read_unaligned();
                let b = base
                    .byte_add(n as usize)
                    .byte_add(4)
                    .cast::<u32>()
                    .read_unaligned();
                f(base, 0, 0, 0x7f, a, b);
                let n2 = scan(DAT_00742668);
                DAT_00742668 = DAT_00742668.byte_add(n2.wrapping_add(8) as usize);
            }
        }
        LeaveCriticalSection((&raw mut DAT_007427a0).cast::<core::ffi::c_void>());
    }
}
