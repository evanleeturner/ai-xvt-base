// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00665050: i32;
    static mut DAT_00665054: i32;
    static mut DAT_00665058: i32;
    static mut DAT_0066505c: i32;
    static mut DAT_00665060: i32;
    static mut DAT_00665410: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d2b40(param_1: i32, param_2: u32) -> i32 {
    // SAFETY: the globals are the game's own table of 40 six-int entries;
    // all reads and writes stay at the offsets the original uses.
    unsafe {
        let base = &raw mut DAT_00665050;
        let end = &raw mut DAT_00665410 as usize;
        let mut p = base;
        let mut i: usize = 0;
        loop {
            if p.read() == param_1 {
                base.wrapping_add(i.wrapping_mul(6)).write(param_1);
                (&raw mut DAT_00665060)
                    .wrapping_add(i.wrapping_mul(6))
                    .write(param_2 as i32);
                break;
            }
            p = p.wrapping_add(6);
            i += 1;
            if !((p as usize) < end) {
                break;
            }
        }

        if i != 0x28 {
            i = 0;
            p = base;
            while p.read() != 0 {
                p = p.wrapping_add(6);
                i += 1;
                if (p as usize) > 0x66540f {
                    return 1;
                }
            }
            base.wrapping_add(i.wrapping_mul(6)).write(param_1);
            (&raw mut DAT_00665054)
                .wrapping_add(i.wrapping_mul(6))
                .write(1);
            (&raw mut DAT_00665058)
                .wrapping_add(i.wrapping_mul(6))
                .write(0);
            (&raw mut DAT_0066505c)
                .wrapping_add(i.wrapping_mul(6))
                .write(0);
            (&raw mut DAT_00665060)
                .wrapping_add(i.wrapping_mul(6))
                .write(param_2 as i32);
        }
        1
    }
}
