// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00665050: [i32; 240];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d2a20(param_1: i32, param_2: i32) -> i32 {
    // SAFETY: DAT_00665050 is the game's own table of 40 entries of 6 ints;
    // every access stays inside it, at the same slots the original uses.
    unsafe {
        let base = (&raw mut DAT_00665050).cast::<i32>();
        let mut i_var2: i32 = 0;
        let mut pi_var1 = base;
        loop {
            if pi_var1.read() == param_1 {
                base.offset((i_var2 as isize) * 6).write(param_1);
                base.offset((i_var2 as isize) * 6 + 2).write(param_2);
                break;
            }
            pi_var1 = pi_var1.add(6);
            i_var2 = i_var2.wrapping_add(1);
            if !((pi_var1 as usize) < 0x665410) {
                break;
            }
        }

        if i_var2 == 0x28 {
            i_var2 = 0;
            pi_var1 = base;
            while pi_var1.read() != 0 {
                pi_var1 = pi_var1.add(6);
                i_var2 = i_var2.wrapping_add(1);
                if (pi_var1 as usize) >= 0x665410 {
                    return 1;
                }
            }
            let slot = base.offset((i_var2 as isize) * 6);
            slot.write(param_1);
            slot.add(1).write(1);
            slot.add(2).write(param_2);
            slot.add(3).write(0);
            slot.add(4).write(0);
        }
        1
    }
}
