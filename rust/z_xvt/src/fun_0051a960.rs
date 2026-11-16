// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005366fc: u16;
    static mut DAT_00742660: i32;
    static mut DAT_0074278c: i32;
    static mut DAT_005366d4: i32;
    static mut DAT_00742668: i32;
    static mut DAT_00742730: i32;
    static mut DAT_00757c30: i32;
    static mut DAT_007427c0: i32;
    static mut DAT_007426d0: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0051aa30(param_1: u32, param_2: u32, param_3: u16) -> i16;
    fn FUN_0051c6f0(param_1: *mut i8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0051a960(param_1: u32, param_2: u32, param_3: u16) -> i16 {
    // SAFETY: the globals, arrays and callees are the game's own memory;
    // indices and the vtable-like pointer are used as the original does.
    unsafe {
        DAT_005366fc = param_3;
        let mut s_var1: i16 = 1;
        if DAT_00742660 < DAT_0074278c {
            if DAT_005366d4 != 0 {
                s_var1 = FUN_0051aa30(param_1, param_2, param_3);
                return s_var1;
            }
            FUN_0051aa30(param_1, param_2, param_3);
            if DAT_00742660 < DAT_0074278c.wrapping_sub(1) {
                DAT_00742660 = DAT_00742660.wrapping_add(1);
                let v = (&raw mut DAT_007426d0).offset(DAT_00742660 as isize).read();
                FUN_0051c6f0(v as u32 as *mut i8);
                DAT_00742668 = (&raw mut DAT_00742730).offset(DAT_00742660 as isize).read();
                let fp = ((DAT_00757c30 as u32).wrapping_add(0x30) as *const u32).read_unaligned();
                let arg = (&raw mut DAT_007427c0).offset(DAT_00742660 as isize).read();
                let f: unsafe extern "C" fn(u32) =
                    core::mem::transmute(fp as usize as *const core::ffi::c_void);
                f(arg as u32);
                return 1;
            }
        } else {
            s_var1 = 0;
        }
        s_var1
    }
}
