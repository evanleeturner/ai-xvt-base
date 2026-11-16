// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0055696d: i32;
    static mut DAT_0055695d: i32;
    static mut DAT_00556961: i32;
    static mut DAT_0055692c: i32;
    static mut DAT_00556928: *mut core::ffi::c_void;
    static mut DAT_00556959: *mut core::ffi::c_void;
    static mut DAT_00556930: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00444d90(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> *mut core::ffi::c_void;
    fn _malloc(size: u32) -> *mut core::ffi::c_void;
    fn FUN_004438b0();
    fn _exit(code: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00444cb0(param_1: i32) {
    let mut param_1 = param_1;
    // SAFETY: the globals and callees are the game's own; the pointers written
    // through are the ones the game's allocators returned, used as the original does.
    unsafe {
        DAT_0055696d = 0;
        DAT_0055695d = 0;
        DAT_00556961 = 0;
        if param_1 > 8 {
            param_1 = 8;
        }
        if param_1 < 2 {
            param_1 = 2;
        }
        DAT_0055692c = param_1;
        DAT_00556928 = FUN_00444d90(0, 0, 0, 0x80, 0x80, 0x80);
        DAT_00556959 = _malloc(0x7fc);
        if !DAT_00556928.is_null() && !DAT_00556959.is_null() {
            let tmp_ptr = DAT_00556928;
            DAT_00556928
                .byte_add(0x2e)
                .cast::<*mut core::ffi::c_void>()
                .write_unaligned(tmp_ptr);
            let p_var1 = DAT_00556928;
            DAT_00556928.byte_add(0xe).cast::<i32>().write_unaligned(0);
            p_var1.byte_add(0x12).cast::<i32>().write_unaligned(0);
            DAT_00556930 = 0;
            DAT_00556959 = DAT_00556959.byte_add(0x3fc);
            let mut i_var2: i32 = -0xff;
            loop {
                let addr =
                    (DAT_00556959 as usize as u32).wrapping_add((i_var2 as u32).wrapping_mul(4));
                (addr as usize as *mut i32).write_unaligned(i_var2.wrapping_mul(i_var2));
                i_var2 = i_var2.wrapping_add(1);
                if i_var2 >= 0x100 {
                    break;
                }
            }
            return;
        }
        FUN_004438b0();
        _exit(1);
    }
}
