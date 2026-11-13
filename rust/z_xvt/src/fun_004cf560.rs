// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0064f070: [i32; 240];
    static mut DAT_0064f430: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004cf560(param_1: i32, param_2: i32) -> i32 {
    // SAFETY: the globals are the game's own table of 0x28 six-int entries;
    // every access stays inside it, as in the original.
    unsafe {
        let base = (&raw mut DAT_0064f070).cast::<i32>();
        let end = (&raw mut DAT_0064f430) as usize;
        let mut i_var2: i32 = 0;
        let mut p = base;
        loop {
            if p.read() == param_1 {
                base.offset((i_var2.wrapping_mul(6)) as isize)
                    .write(param_1);
                base.offset((i_var2.wrapping_mul(6).wrapping_add(2)) as isize)
                    .write(param_2);
                break;
            }
            p = p.add(6);
            i_var2 = i_var2.wrapping_add(1);
            if (p as usize) >= end {
                break;
            }
        }

        if i_var2 == 0x28 {
            i_var2 = 0;
            p = base;
            while p.read() != 0 {
                p = p.add(6);
                i_var2 = i_var2.wrapping_add(1);
                if 0x64f42f < (p as usize) {
                    return 1;
                }
            }
            let o = i_var2.wrapping_mul(6) as isize;
            base.offset(o).write(param_1);
            base.offset(o + 1).write(1);
            base.offset(o + 2).write(param_2);
            base.offset(o + 3).write(0);
            base.offset(o + 4).write(0);
        }
        1
    }
}
