// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7b36: i32;
    static mut DAT_00aa7b9a: i32;
    static mut DAT_00aa7b32: i32;
    static mut DAT_00b69cb0: i32;
    static mut DAT_00b69cbc: i32;
    static mut DAT_00aa7b3a: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004d4970();
    fn FUN_004de4d0();
    fn FUN_004de2e0();
    fn FUN_004d31a0();
}

unsafe fn call_at_plus_8(obj: i32) {
    // SAFETY: mirrors the original: the first dword at obj is read, 8 is added
    // to that value (void* arithmetic), and the result is called as a stdcall
    // function taking obj.
    unsafe {
        let base = (obj as usize as *const u32).read_unaligned();
        let target = (base as usize as *const core::ffi::c_void).wrapping_byte_add(8);
        let f: unsafe extern "stdcall" fn(i32) = core::mem::transmute::<
            *const core::ffi::c_void,
            unsafe extern "stdcall" fn(i32),
        >(target);
        f(obj);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d56b0() -> i32 {
    // SAFETY: the callees and globals are the game's own memory; the objects
    // pointed to by the globals are called exactly as the original does.
    unsafe {
        FUN_004d4970();
        FUN_004de4d0();
        FUN_004de2e0();
        FUN_004d31a0();
        if DAT_00aa7b36 != 0 {
            call_at_plus_8(DAT_00aa7b36);
            DAT_00aa7b36 = 0;
        }
        if DAT_00aa7b9a != 0 {
            call_at_plus_8(DAT_00aa7b9a);
            DAT_00aa7b9a = 0;
        }
        if DAT_00aa7b32 != 0 {
            call_at_plus_8(DAT_00aa7b32);
            DAT_00aa7b32 = 0;
        }
        if (DAT_00b69cb0 != 0 || DAT_00b69cbc != 0) && DAT_00aa7b3a != 0 {
            call_at_plus_8(DAT_00aa7b3a);
            DAT_00aa7b3a = 0;
        }
        1
    }
}
