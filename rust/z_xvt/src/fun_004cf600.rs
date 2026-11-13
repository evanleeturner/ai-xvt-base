// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0064f070: [i32; 0];
    static mut DAT_0064f430: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004cf600(param_1: i32, param_2: i32) -> i32 {
    // SAFETY: the globals are the game's own table of 40 six-int entries;
    // all accesses stay within it as in the original.
    unsafe {
        let base = (&raw mut DAT_0064f070).cast::<i32>();
        let end = (&raw mut DAT_0064f430).cast::<i32>();
        let mut p = base;
        let mut i: i32 = 0;
        loop {
            if p.read() == param_1 {
                base.offset((i as isize) * 6).write(param_1);
                base.offset((i as isize) * 6 + 3).write(param_2);
                break;
            }
            p = p.add(6);
            i = i.wrapping_add(1);
            if !(p < end) {
                break;
            }
        }

        if i == 0x28 {
            i = 0;
            p = base;
            while p.read() != 0 {
                p = p.add(6);
                i = i.wrapping_add(1);
                if (p as usize) > 0x64f42f {
                    return 1;
                }
            }
            let e = base.offset((i as isize) * 6);
            e.write(param_1);
            e.add(1).write(1);
            e.add(2).write(0);
            e.add(3).write(param_2);
            e.add(4).write(0);
        }
        1
    }
}
