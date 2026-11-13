// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00665050: [i32; 0];
    static mut DAT_00665410: [i32; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d2ab0(param_1: i32, param_2: u32) -> u32 {
    // SAFETY: the tables are the game's own memory; every access stays
    // within the 0x28-entry, six-int-per-entry table the original walks.
    unsafe {
        let base = (&raw mut DAT_00665050).cast::<i32>();
        let end = (&raw mut DAT_00665410).cast::<i32>();
        let mut p = base;
        let mut i: i32 = 0;
        loop {
            if p.read() == param_1 {
                let e = base.offset((i.wrapping_mul(6)) as isize);
                e.write(param_1);
                e.add(3).write(param_2 as i32);
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
                if p > (0x0066540f_usize as *mut i32) {
                    return 1;
                }
            }
            let e = base.offset((i.wrapping_mul(6)) as isize);
            e.write(param_1);
            e.add(1).write(1);
            e.add(2).write(0);
            e.add(3).write(param_2 as i32);
            e.add(4).write(0);
        }
        1
    }
}
