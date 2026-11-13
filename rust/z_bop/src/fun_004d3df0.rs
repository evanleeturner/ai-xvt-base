// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7b0d: i8;
    static mut DAT_00aa7b2e: *mut core::ffi::c_void;
    static mut DAT_00aa7b26: *mut core::ffi::c_void;
    static mut DAT_00aa6cfc: i32;
    static mut DAT_0052ba90: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d4970();
    fn FUN_004d48e0() -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn MessageBoxA(
        h_wnd: *mut core::ffi::c_void,
        lp_text: *const i8,
        lp_caption: *const i8,
        u_type: u32,
    ) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d3df0(param_1: *const i8) -> i32 {
    // SAFETY: the globals and callees are the game's own memory and code;
    // the object pointer is read and called exactly as the original does.
    unsafe {
        let c_var1 = DAT_00aa7b0d;
        FUN_004d4970();
        if !DAT_00aa7b2e.is_null() {
            let addr = DAT_00aa7b2e
                .cast::<i32>()
                .read_unaligned()
                .wrapping_add(0x28);
            let f: unsafe extern "stdcall" fn(*mut core::ffi::c_void) =
                core::mem::transmute(addr as usize as *const core::ffi::c_void);
            f(DAT_00aa7b2e);
        }
        MessageBoxA(DAT_00aa7b26, param_1, &raw const DAT_0052ba90, 0x30);
        if c_var1 != 0 {
            DAT_00aa6cfc = FUN_004d48e0();
        }
        1
    }
}
