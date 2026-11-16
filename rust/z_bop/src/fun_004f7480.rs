// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0066925a: i16;
    static mut DAT_0066925c: i16;
    static mut DAT_00669264: [i16; 1];
    static mut DAT_00669262: [i16; 1];
    static mut DAT_006696e2: i16;
    static mut DAT_006696da: [i8; 2];
    static mut DAT_00669778: i16;
    static mut DAT_00669218: i32;
    static mut DAT_0066921c: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004f7380();
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004f7590(param_1: i32);
    fn FUN_004f7420(param_1: i16, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004f7480() {
    // SAFETY: all globals and callees are the game's own memory and code;
    // array reads are indexed by the game's short index as the original does.
    unsafe {
        let mut s_var1: i16 = DAT_0066925a;
        let mut b_var2: i32 = 0;
        let mut s_var6: i16 = 0;
        let mut s_var5: i16 = 0;
        let mut b_var3: i32 = 0;
        FUN_004f7380();
        loop {
            if s_var5 == 0x22 {
                break;
            }
            s_var5 = (&raw mut DAT_00669264)
                .cast::<i16>()
                .offset(DAT_0066925c as isize)
                .read_unaligned();
            if DAT_006696e2 != 0 {
                s_var6 = 0;
                b_var2 = 0;
            }
            let mut s_var4: i16 = 0;
            loop {
                if (&raw mut DAT_006696da)
                    .cast::<i8>()
                    .offset(s_var4 as isize)
                    .read()
                    != 0
                {
                    b_var2 = 1;
                }
                s_var4 = s_var4.wrapping_add(1);
                if !(s_var4 < 2) {
                    break;
                }
            }
            if b_var2 != 0 {
                s_var6 = s_var6.wrapping_add(1);
            }
            if (DAT_00669778 == 0 && s_var6 != 1) || DAT_0066925a < s_var1 {
                FUN_004f7590(1);
            } else {
                b_var3 = 1;
            }
            if b_var3 != 0 {
                break;
            }
        }
        if DAT_00669778 == 0 && s_var6 != 1 {
            s_var5 = (&raw mut DAT_00669264)
                .cast::<i16>()
                .offset(DAT_0066925c as isize)
                .read_unaligned();
            s_var1 = (&raw mut DAT_00669262)
                .cast::<i16>()
                .offset(DAT_0066925c as isize)
                .read_unaligned();
        } else {
            s_var5 = 0;
            s_var1 = DAT_0066925a;
        }
        if s_var5 == 0x22 {
            DAT_00669218 = 0;
            DAT_0066921c = 0;
            FUN_004f7380();
            return;
        }
        FUN_004f7420(s_var1, 0);
    }
}
