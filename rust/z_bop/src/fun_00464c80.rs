// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005242e0: i32;
    static mut DAT_00557358: i32;
    static mut DAT_005242e4: i32;
    static mut _DAT_009ed23c: i32;
    static mut DAT_0052340c: i32;
    static mut DAT_00523418: i32;
    static mut DAT_009e9680: i8;
    static mut DAT_009ec468: i8;
    static mut DAT_005242e8: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00418890(param_1: i32) -> *mut i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00464c80(param_1: i32) -> i32 {
    // SAFETY: the globals and the callee are the game's own memory; the
    // pointer returned by the callee is read where the original reads it.
    unsafe {
        if DAT_005242e0 != 0 {
            return 0;
        }
        if DAT_00557358 != 0 {
            return 0;
        }
        if DAT_005242e4 == 0 {
            DAT_005242e4 = (_DAT_009ed23c >> 3)
                .wrapping_add(param_1)
                .wrapping_add(DAT_0052340c);
        }
        let i_var2 = param_1
            .wrapping_add(DAT_0052340c)
            .wrapping_sub(DAT_005242e4);
        if i_var2 < DAT_00523418 {
            return 0;
        }
        if ((DAT_00523418 as u32).wrapping_mul(5) as i32) < i_var2 {
            DAT_005242e4 = DAT_005242e4.wrapping_add(DAT_00523418);
            return 1;
        }
        let mut i_var2: i32 = 0x7fffffff;
        let mut i_var4: i32 = 0;
        let mut pc_var3: *mut i8 = &raw mut DAT_009e9680;
        let end: *mut i8 = &raw mut DAT_009ec468;
        loop {
            if *pc_var3 != 0 {
                let pi_var1 = FUN_00418890(i_var4);
                if pi_var1.is_null() {
                    i_var2 = 0;
                    break;
                }
                let v = pi_var1.byte_add(8).read_unaligned();
                if v < i_var2 {
                    i_var2 = v;
                }
            }
            pc_var3 = pc_var3.wrapping_byte_add(0x5bd);
            i_var4 = i_var4.wrapping_add(1);
            if pc_var3 >= end {
                break;
            }
        }
        if i_var2 <= DAT_005242e8.wrapping_add(DAT_00523418) {
            return 0;
        }
        DAT_005242e4 = DAT_005242e4.wrapping_add(DAT_00523418);
        1
    }
}
