// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00758618: i32;
    static mut DAT_00757cc0: i32;
    static mut DAT_00757ccc: i32;
    static mut DAT_00757cc8: i32;
    static mut DAT_0081b740: [i8; 1];
    static mut DAT_00757cc4: *mut i8;
    static mut DAT_005330fc: u32;
    static mut DAT_00859c2c: u32;
    static mut DAT_00532aa4: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dcfd0(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
    ) -> u32;
    fn FUN_004dcfa0(param_1: u32) -> *mut i8;
    fn sprintf(dst: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_004daac0(
        param_1: i32,
        param_2: *mut i8,
        param_3: *mut i32,
        param_4: i32,
        param_5: i32,
        param_6: u32,
    ) -> u32;
    fn FUN_004daee0(
        param_1: i32,
        param_2: *mut i8,
        param_3: *mut i32,
        param_4: u32,
        param_5: i32,
        param_6: i32,
    ) -> i32;
    fn FUN_004d9010(
        param_1: *mut i32,
        param_2: u32,
        param_3: u32,
        param_4: i32,
        param_5: i32,
        param_6: u32,
        param_7: i32,
    ) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d75c0() -> i32 {
    let mut local_10: [i32; 4] = [0; 4];
    // SAFETY: all callees and globals are the game's own; the string copy
    // reads a NUL-terminated string and writes it to the game's buffer.
    unsafe {
        let l = local_10.as_mut_ptr();
        FUN_004dcfd0(l, 0x58, 0x135, 0x1ae, 0x144);
        let pcvar3: *mut i8;
        if DAT_00758618 == -1 {
            pcvar3 = FUN_004dcfa0(0xaf);
        } else {
            if DAT_00757cc0 == 0 || DAT_00757ccc <= DAT_00757cc8 {
                let src = FUN_004dcfa0(0xaf);
                let dst = &raw mut DAT_0081b740 as *mut i8;
                let mut i: usize = 0;
                loop {
                    let c = src.add(i).read();
                    dst.add(i).write(c);
                    i += 1;
                    if c == 0 {
                        break;
                    }
                }
            } else {
                let ivar2 = DAT_00757cc8
                    .wrapping_mul(0x144)
                    .wrapping_add(DAT_00757cc0)
                    .wrapping_add(0x40);
                let uvar7: u32 = 4;
                let p = FUN_004dcfa0(0xaf);
                sprintf(
                    &raw mut DAT_0081b740 as *mut i8,
                    &raw const DAT_00532aa4 as *const i8,
                    p,
                    uvar7,
                    ivar2,
                );
            }
            pcvar3 = &raw mut DAT_0081b740 as *mut i8;
        }
        FUN_004daac0(0xc, pcvar3, l, 0, 1, 0xffff);
        if DAT_00758618 != -1 && !DAT_00757cc4.is_null() {
            FUN_004dcfd0(l, 0x58, 0x148, 0x1a4, 0x1aa);
            let ivar2 = FUN_004daee0(0xc, DAT_00757cc4, l, 0xffff, 4, 0x400);
            let mut uvar7: u32 = 0x1ae;
            let n = (ivar2 as u32).wrapping_add(1);
            if 6 < n {
                FUN_004dcfd0(l, 0x1a5, 0x148, 0x1ae, 0x1aa);
                DAT_005330fc = FUN_004d9010(l, DAT_005330fc, n, 0, 5, DAT_00859c2c, 6);
                uvar7 = 0x1a4;
            }
            FUN_004dcfd0(l, 0x58, 0x148, uvar7 as i32, 0x1aa);
            FUN_004daee0(0xc, DAT_00757cc4, l, 0xffff, 4, DAT_005330fc as i32);
        }
    }
    1
}
