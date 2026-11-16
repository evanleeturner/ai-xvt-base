// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0099f6e0: i32;
    static mut DAT_00999bf0: [i32; 1];
    static mut DAT_00999c00: [i32; 1];
    static mut DAT_00999bf8: [i32; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0046fee0(param_1: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0046fc00(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
) -> i32 {
    // SAFETY: the globals and the callee are the game's own; the index is
    // bounded by 0x27 before any table access, as in the original.
    unsafe {
        let saved = DAT_0099f6e0;
        let u_var1 = FUN_0046fee0(param_1);
        if DAT_0099f6e0 != saved || 0x27 < u_var1 {
            return 0;
        }
        let idx = (u_var1.wrapping_mul(0x8e)) as usize;
        let p_bf0 = (&raw mut DAT_00999bf0).cast::<i32>().add(idx);
        let p_c00 = (&raw mut DAT_00999c00).cast::<i32>().add(idx);
        let p_bf8 = (&raw mut DAT_00999bf8).cast::<i32>().add(idx);
        let cur = if param_3 == 0 {
            if param_4 == 0 {
                p_bf0.read()
            } else {
                p_c00.read()
            }
        } else {
            p_bf8.read()
        };
        let d = param_2.wrapping_sub(cur);
        if -0x41 < d && (d < 1 || 0x3f < d) {
            return 1;
        }
        if param_3 != 0 {
            p_bf8.write(param_2);
            return 0;
        }
        if param_4 != 0 {
            p_c00.write(param_2);
            return 0;
        }
        p_bf0.write(param_2);
        0
    }
}
