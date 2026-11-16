// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00655110: u16;
    static mut DAT_006550dc: u32;
    static mut DAT_006550f4: *mut core::ffi::c_void;
    static mut DAT_0074244e: u16;
    static mut DAT_0074244c: u16;
    static mut DAT_006550e4: u16;
    static mut DAT_006550e0: u16;
    static mut DAT_00655108: u32;
    static mut _DAT_006550fc: u16;
    static mut DAT_006550f0: u32;
    static mut DAT_006550e8: u16;
    static mut DAT_0053670c: u16;
    static mut _DAT_00655100: u32;
    static mut DAT_00536704: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048f4f0(param_1: u32) -> i32;
    fn FUN_0048f510(param_1: i32);
    fn FUN_0048f4b0(param_1: u32);
    fn _fclose(stream: *mut core::ffi::c_void) -> i32;
    fn FUN_0051c130(param_1: i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_0051d360();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0051c4f0() -> u32 {
    // SAFETY: the globals and callees are the game's own memory and code;
    // the table pointer returned by FUN_0048f4f0 is read where the original reads it.
    unsafe {
        if DAT_00655110 == 0 {
            return 1;
        }
        let mut u_var2: u32 = 0;
        if DAT_006550dc != 0 {
            loop {
                let mut i_var1 = FUN_0048f4f0(DAT_00655110 as u32);
                let addr = (i_var1 as u32).wrapping_add(u_var2.wrapping_mul(2));
                let mut u_var3: u16 = (addr as *const u16).read_unaligned();
                i_var1 = ((u_var3 == 0) as i32).wrapping_sub(1);
                u_var3 = (i_var1 as u16) & u_var3;
                FUN_0048f510(i_var1);
                if u_var3 != 0 {
                    FUN_0048f4b0((((i_var1 as u32) >> 16) << 16) | (u_var3 as u32));
                }
                u_var2 = u_var2.wrapping_add(1);
                if !(u_var2 < DAT_006550dc) {
                    break;
                }
            }
        }
        FUN_0048f4b0(DAT_00655110 as u32);
        DAT_00655110 = 0;
        if !DAT_006550f4.is_null() {
            _fclose(DAT_006550f4);
        }
        FUN_0051d360();
        DAT_0074244e = 0;
        DAT_0074244c = 0;
        FUN_0051c130(5);
        DAT_006550e4 = 0;
        DAT_006550e0 = 0;
        DAT_00655108 = 0;
        DAT_006550f4 = core::ptr::null_mut();
        _DAT_006550fc = 0;
        DAT_006550f0 = 0;
        DAT_006550e8 = DAT_0053670c;
        _DAT_00655100 = 0;
        DAT_00536704 = 0;
        1
    }
}
