// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4984: i16;
    static mut DAT_006d4a42: i16;
    static mut DAT_00709ffc: i32;
    static mut DAT_00732e44: i16;
    static mut DAT_0052562f: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00413f20(a: i32, b: i32) -> i32;
    fn FUN_0049c6b0(a: u32, b: i32, c: i32, d: u16);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0044b5d0(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: u16,
    param_5: i32,
) {
    // SAFETY: the callees and globals are the game's own memory; the table
    // index is read where the original reads it.
    unsafe {
        let s_var1: i16;
        if param_1 < 0 {
            if param_3 < param_1.wrapping_neg() {
                return;
            }
            s_var1 = (FUN_00413f20(param_1.wrapping_neg(), param_3) as i16).wrapping_neg();
        } else {
            if param_3 < param_1 {
                return;
            }
            s_var1 = FUN_00413f20(param_1, param_3) as i16;
        }

        let mut i_var3: i32;
        if param_2 < 0 {
            if param_3 < param_2.wrapping_neg() {
                return;
            }
            i_var3 = FUN_00413f20(param_2.wrapping_neg(), param_3).wrapping_neg();
        } else {
            if param_3 < param_2 {
                return;
            }
            i_var3 = FUN_00413f20(param_2, param_3);
        }

        i_var3 = i_var3
            .wrapping_add(DAT_006d4a42 as i32)
            .wrapping_add(DAT_00709ffc);

        let mut arg1: u32 = ((((i_var3 as u32) >> 16) as i16) as i32) as u32;
        arg1 = (arg1 << 16) | (param_4 as u32);
        arg1 >>= 8;
        arg1 &= 0xFFFFFF;
        let b = (&raw mut DAT_0052562f)
            .cast::<u8>()
            .offset(param_5 as isize)
            .read();
        arg1 = (arg1 << 8) | (b as u32);
        arg1 &= 0xFFFF00FF;

        FUN_0049c6b0(
            arg1,
            (s_var1 as i32).wrapping_add(DAT_00732e44 as i32),
            (DAT_006d4984 as i32).wrapping_sub((i_var3 as i16) as i32),
            param_4,
        );
    }
}
