// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00555d48: f32;
    static mut DAT_00555d4c: f32;
    static mut DAT_00555d50: f32;
    static mut DAT_00555d54: f32;
    static mut DAT_00555d68: f32;
    static mut DAT_00555d6c: f32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0042ad10(param_1: *mut core::ffi::c_void, param_2: *mut core::ffi::c_void);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042ae30(param_1: i32, param_2: i32) -> f32 {
    // SAFETY: the globals and the callee are the game's own; param_1 points
    // at the caller's structure, read at the offsets the original uses.
    unsafe {
        let mut i_var3: i32 = 0;
        DAT_00555d50 = 0.0;
        DAT_00555d54 = 0.0;
        DAT_00555d68 = 0.0;
        DAT_00555d6c = 0.0;
        DAT_00555d48 = 0.0;
        DAT_00555d4c = 0.0;

        if ((param_1.wrapping_add(6)) as *const i32).read_unaligned() > 0 {
            let mut i_var2: i32 = 0;
            loop {
                let base = ((param_1.wrapping_add(10)) as *const i32).read_unaligned();
                let puVar1 = (base.wrapping_add(i_var2)) as *const i32;
                i_var2 = i_var2.wrapping_add(4);
                i_var3 = i_var3.wrapping_add(1);
                FUN_0042ad10(
                    puVar1.read_unaligned() as *mut core::ffi::c_void,
                    param_1 as *mut core::ffi::c_void,
                );
                if !(i_var3 < ((param_1.wrapping_add(6)) as *const i32).read_unaligned()) {
                    break;
                }
            }
        }

        DAT_00555d50 = DAT_00555d50 - DAT_00555d54;
        DAT_00555d68 = DAT_00555d68 - DAT_00555d6c;
        DAT_00555d48 = DAT_00555d48 - DAT_00555d4c;

        if param_2 != 0 {
            let mut local_8: f32 = 0.0;
            if param_2 == 1 {
                local_8 = DAT_00555d50;
            }
            if param_2 == 2 {
                local_8 = DAT_00555d68;
            }
            if param_2 == 3 {
                local_8 = DAT_00555d48;
            }
            return local_8;
        }

        if DAT_00555d68 < DAT_00555d50 && DAT_00555d48 < DAT_00555d50 {
            return DAT_00555d50;
        }
        if DAT_00555d50 < DAT_00555d68 && DAT_00555d48 < DAT_00555d68 {
            return DAT_00555d68;
        }
        DAT_00555d48
    }
}
