// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004765b0(param_1: i32, param_2: i32, param_3: i32) -> i32 {
    if param_1 == 0 {
        return 0;
    }
    if param_2 == 0 {
        return 0;
    }
    // SAFETY: the parameters are addresses of the game's own node structures,
    // read at the same offsets and in the same order as the original.
    unsafe {
        if ((param_2.wrapping_add(4)) as *const i32).read_unaligned()
            != ((param_1.wrapping_add(4)) as *const i32).read_unaligned()
        {
            return 0;
        }
        if ((param_1.wrapping_add(4)) as *const i32).read_unaligned() == 0x14 {
            let pi_var1 =
                ((param_1.wrapping_add(0x14)) as *const i32).read_unaligned() as *const i32;
            let mut i_var2: i32 = pi_var1
                .add(5)
                .read_unaligned()
                .wrapping_mul(pi_var1.add(4).read_unaligned());
            if pi_var1.add(2).read_unaligned() == i_var2 {
                i_var2 = pi_var1.add(3).read_unaligned();
            }
            i_var2 = (pi_var1 as i32).wrapping_add(i_var2).wrapping_add(0x18);
            if (pi_var1.read_unaligned() == i_var2 || pi_var1.add(1).read_unaligned() != 0)
                && param_3 == i_var2
            {
                return param_2;
            }
        }
        if ((param_2.wrapping_add(8)) as *const i32).read_unaligned()
            != ((param_1.wrapping_add(8)) as *const i32).read_unaligned()
        {
            return 0;
        }
        let mut i_var2: i32 = 0;
        let mut i_var4: i32 = 0;
        if 0 < ((param_1.wrapping_add(8)) as *const i32).read_unaligned() {
            loop {
                let a = (((param_1.wrapping_add(0xc)) as *const i32)
                    .read_unaligned()
                    .wrapping_add(i_var2)) as *const i32;
                let b = (((param_2.wrapping_add(0xc)) as *const i32)
                    .read_unaligned()
                    .wrapping_add(i_var2)) as *const i32;
                let i_var3 = FUN_004765b0(a.read_unaligned(), b.read_unaligned(), param_3);
                if i_var3 != 0 {
                    return i_var3;
                }
                i_var2 = i_var2.wrapping_add(4);
                i_var4 = i_var4.wrapping_add(1);
                if i_var4 >= ((param_1.wrapping_add(8)) as *const i32).read_unaligned() {
                    break;
                }
            }
        }
        0
    }
}
