// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00759663: i32;
    static mut DAT_0075966f: *mut i32;
    static mut DAT_0075964b: i32;
    static mut DAT_0075964f: i32;
    static mut DAT_00759645: i32;
    static mut DAT_007595fd: i32;
    static mut _DAT_007595d9: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d4d00() -> i32 {
    // SAFETY: the globals are the game's own memory; the object pointer and
    // its table are read and called exactly as the original does.
    unsafe {
        if DAT_00759663 == 0 {
            return 0;
        }
        if DAT_0075966f.is_null() {
            return 0;
        }
        DAT_0075964b = DAT_0075964f;
        if DAT_00759645 != 0 {
            return DAT_007595fd;
        }
        _DAT_007595d9 = 0x6c;
        loop {
            let mut i_var1: i32;
            loop {
                let base = DAT_0075966f.cast::<u32>().read_unaligned();
                let f25 = base.wrapping_add(25) as usize as *const core::ffi::c_void;
                let f: unsafe extern "stdcall" fn(
                    *mut core::ffi::c_void,
                    i32,
                    *mut i32,
                    i32,
                    i32,
                ) -> i32 = core::mem::transmute(f25);
                i_var1 = f(
                    DAT_0075966f.cast::<core::ffi::c_void>(),
                    0,
                    &raw mut _DAT_007595d9,
                    0,
                    0,
                );
                if i_var1 != -0x7789fe3e {
                    break;
                }
                let base2 = DAT_0075966f.cast::<u32>().read_unaligned();
                let f27 = base2.wrapping_add(27) as usize as *const core::ffi::c_void;
                let g: unsafe extern "stdcall" fn(*mut core::ffi::c_void) =
                    core::mem::transmute(f27);
                g(DAT_0075966f.cast::<core::ffi::c_void>());
            }
            if !(i_var1 == -0x7789fde4 || i_var1 == -0x7789fe52 || i_var1 == -0x7789fe48) {
                break;
            }
        }
        DAT_00759645 = 1;
        DAT_007595fd
    }
}
