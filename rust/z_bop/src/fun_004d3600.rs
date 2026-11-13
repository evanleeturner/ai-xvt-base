// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b69cac: *mut i8;
    static mut DAT_00b69cbc: u32;
    static mut DAT_00b69cb0: u32;
    static mut DAT_00b69cb8: u32;
    static mut DAT_00b69ca8: u32;
    static mut DAT_00b69cb4: u32;
    static mut DAT_00b6a2ac: *mut core::ffi::c_void;
    static mut DAT_00b69cd0: *mut core::ffi::c_void;
    static mut DAT_00b6a248: *mut core::ffi::c_void;
    static mut DAT_00b69e34: u32;
    static mut DAT_00b69cd8: *mut core::ffi::c_void;
    static mut DAT_00b69cc8: u32;
    static mut DAT_0052ba5c: u32;
    static mut DAT_0052ba80: i8;
    static mut DAT_00527fcc: i8;
    static mut DAT_00527fe4: i8;
    static mut DAT_0052ba74: i8;
    static mut DAT_0052ba6c: i8;
    static mut DAT_0052ba60: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    #[link_name = "strstr"]
    fn _strstr(a: *const i8, b: *const i8) -> *mut i8;
    fn FUN_004d5b90() -> i32;
    fn _exit(code: i32);
    fn FUN_004d4770(param_1: *mut core::ffi::c_void);
    #[link_name = "free"]
    fn _free(p: *mut core::ffi::c_void);
    fn FUN_004bf650(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d3600(
    param_1: *mut core::ffi::c_void,
    param_2: u32,
    param_3: *mut i8,
) -> u8 {
    let _ = param_2;
    // SAFETY: the globals and callees are the game's own; param_3 is the
    // caller's command-line string, and the needles are the game's string data.
    unsafe {
        DAT_00b69cac = param_3;
        let mut p = _strstr(param_3, &raw const DAT_0052ba80);
        DAT_00b69cbc = (!p.is_null()) as u32;
        p = _strstr(param_3, &raw const DAT_00527fcc);
        if p.is_null() {
            p = _strstr(param_3, &raw const DAT_00527fe4);
            if p.is_null() {
                DAT_00b69cb0 = 0;
            } else {
                DAT_00b69cb0 = 1;
            }
        } else {
            DAT_00b69cb0 = 1;
        }
        p = _strstr(param_3, &raw const DAT_0052ba74);
        DAT_00b69cb8 = (!p.is_null()) as u32;
        p = _strstr(param_3, &raw const DAT_0052ba6c);
        DAT_00b69ca8 = (!p.is_null()) as u32;
        p = _strstr(param_3, &raw const DAT_0052ba60);
        DAT_00b69cb4 = (!p.is_null()) as u32;
        let i_var2 = FUN_004d5b90();
        if i_var2 == 0 {
            FUN_004d4770(param_1);
            if !DAT_00b6a2ac.is_null() {
                _free(DAT_00b6a2ac);
                DAT_00b6a2ac = core::ptr::null_mut();
            }
            if !DAT_00b69cd0.is_null() {
                _free(DAT_00b69cd0);
                DAT_00b69cd0 = core::ptr::null_mut();
            }
            if !DAT_00b6a248.is_null() {
                _free(DAT_00b6a248);
                DAT_00b6a248 = core::ptr::null_mut();
                DAT_00b69e34 = 0;
            }
            if !DAT_00b69cd8.is_null() {
                _free(DAT_00b69cd8);
                DAT_00b69cd8 = core::ptr::null_mut();
                DAT_00b69cc8 = 0;
            }
            FUN_004bf650(0);
            return (DAT_0052ba5c == 0) as u8;
        }
        _exit(0);
        0
    }
}
