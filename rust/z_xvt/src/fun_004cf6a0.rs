// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0064f070: i32;
    static mut DAT_0064f074: i32;
    static mut DAT_0064f078: i32;
    static mut DAT_0064f07c: i32;
    static mut DAT_0064f080: i32;
    static mut DAT_0064f430: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004cf6a0(param_1: i32, param_2: i32) -> i32 {
    // SAFETY: the globals are the game's own table of 40 six-int entries;
    // every access stays inside it, as in the original.
    unsafe {
        let base = &raw mut DAT_0064f070;
        let end = &raw mut DAT_0064f430;
        let mut p = base;
        let mut i: i32 = 0;
        loop {
            if p.read() == param_1 {
                base.offset((i as isize) * 6).write(param_1);
                (&raw mut DAT_0064f080)
                    .offset((i as isize) * 6)
                    .write(param_2);
                break;
            }
            p = p.offset(6);
            i = i.wrapping_add(1);
            if !(p < end) {
                break;
            }
        }

        if i == 0x28 {
            i = 0;
            p = base;
            let limit = 0x64f42f_usize as *mut i32;
            while p.read() != 0 {
                p = p.offset(6);
                i = i.wrapping_add(1);
                if limit < p {
                    return 1;
                }
            }
            base.offset((i as isize) * 6).write(param_1);
            (&raw mut DAT_0064f074).offset((i as isize) * 6).write(1);
            (&raw mut DAT_0064f078).offset((i as isize) * 6).write(0);
            (&raw mut DAT_0064f07c).offset((i as isize) * 6).write(0);
            (&raw mut DAT_0064f080)
                .offset((i as isize) * 6)
                .write(param_2);
        }
        1
    }
}
