// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b69a89: i32;
    static mut DAT_00b63f99: [i32; 1];
    static mut DAT_00b63fa1: [i32; 1];
    static mut DAT_00b63fa9: [i32; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d23b0(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d1fa0(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
) -> i32 {
    // SAFETY: the globals and the callee are the game's own; the index is
    // bounded by 0x27 before use, so the element pointers stay in the game's tables.
    unsafe {
        let i_var2 = DAT_00b69a89;
        let u_var1 = FUN_004d23b0(param_1) as u32;
        if DAT_00b69a89 != i_var2 || 0x27 < u_var1 {
            return 0;
        }
        let idx = (u_var1 as usize).wrapping_mul(0x8e);
        let p_a = (&raw mut DAT_00b63f99).cast::<i32>().add(idx);
        let p_b = (&raw mut DAT_00b63fa1).cast::<i32>().add(idx);
        let p_c = (&raw mut DAT_00b63fa9).cast::<i32>().add(idx);
        let cur = if param_3 == 0 {
            if param_4 == 0 {
                p_a.read_unaligned()
            } else {
                p_c.read_unaligned()
            }
        } else {
            p_b.read_unaligned()
        };
        let d = param_2.wrapping_sub(cur);
        if -0x41 < d && (d < 1 || 0x3f < d) {
            return 1;
        }
        if param_3 != 0 {
            p_b.write_unaligned(param_2);
            return 0;
        }
        if param_4 != 0 {
            p_c.write_unaligned(param_2);
            return 0;
        }
        p_a.write_unaligned(param_2);
        0
    }
}
