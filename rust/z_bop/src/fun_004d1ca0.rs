// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b69a89: i32;
    static mut DAT_00b63f91: i32;
    static mut DAT_00acebf5: i32;
    static mut DAT_00b69851: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cfe80(param_1: *mut i32) -> *mut i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d1ca0() -> i32 {
    // SAFETY: all globals and the callee are the game's own memory; the
    // pointers walk the game's record table exactly as the original does.
    unsafe {
        let mut local_4: i32 = 0;
        let base: *mut i32 = &raw mut DAT_00b63f91;
        let mut i5: i32 = 0;
        let p_var1 = FUN_004cfe80(&mut local_4);
        if 0 < DAT_00b69a89 {
            let mut p6 = base;
            loop {
                let mut i3: i32 = 0;
                if 0 < local_4 {
                    let mut p2 = p_var1.wrapping_add(8);
                    loop {
                        if p2.read_unaligned() == p6.read_unaligned() {
                            break;
                        }
                        p2 = p2.wrapping_add(10);
                        i3 = i3.wrapping_add(1);
                        if !(i3 < local_4) {
                            break;
                        }
                    }
                }
                if local_4 <= i3 && p6.read_unaligned() != DAT_00acebf5 {
                    p6.write_unaligned(0);
                    p6.wrapping_add(1).write_unaligned(0x7f);
                    p6.wrapping_add(3).write_unaligned(0x7f);
                    p6.wrapping_add(5).write_unaligned(0x7f);
                    p6.wrapping_add(2).write_unaligned(0x7f);
                    p6.wrapping_add(4).write_unaligned(0x7f);
                    p6.wrapping_add(6).write_unaligned(0x7f);
                    p6.wrapping_add(7).write_unaligned(0);
                    p6.wrapping_add(8).cast::<u8>().write(0x39);
                    p6.wrapping_add(0x88).write_unaligned(1);
                    p6.wrapping_add(0x89).write_unaligned(0);
                    p6.wrapping_add(0x8d).write_unaligned(0);
                    p6.wrapping_add(0x8a).write_unaligned(0);
                    p6.wrapping_add(0x8b).write_unaligned(0);
                    p6.wrapping_add(0x8c).write_unaligned(0);
                }
                p6 = p6.wrapping_add(0x8e);
                i5 = i5.wrapping_add(1);
                if !(i5 < DAT_00b69a89) {
                    break;
                }
            }
        }
        i5 = 0;
        if DAT_00b69a89 != 1 && -1 < DAT_00b69a89.wrapping_sub(1) {
            let mut p6 = base;
            loop {
                if p6.read_unaligned() == 0 {
                    let mut i3 = i5.wrapping_add(1);
                    if i3 < DAT_00b69a89 {
                        let mut p2 = base.wrapping_offset((i3 as isize).wrapping_mul(0x8e));
                        loop {
                            if p2.read_unaligned() != 0 {
                                p2 = base.wrapping_offset((i3 as isize).wrapping_mul(0x8e));
                                let mut p7 = p6;
                                let mut i4: i32 = 0x8e;
                                while i4 != 0 {
                                    p7.write_unaligned(p2.read_unaligned());
                                    p2 = p2.wrapping_add(1);
                                    p7 = p7.wrapping_add(1);
                                    i4 = i4.wrapping_sub(1);
                                }
                                let r = base.wrapping_offset((i3 as isize).wrapping_mul(0x8e));
                                r.write_unaligned(0);
                                r.wrapping_add(1).write_unaligned(0x7f);
                                r.wrapping_add(3).write_unaligned(0x7f);
                                r.wrapping_add(5).write_unaligned(0x7f);
                                r.wrapping_add(2).write_unaligned(0x7f);
                                r.wrapping_add(4).write_unaligned(0x7f);
                                r.wrapping_add(6).write_unaligned(0x7f);
                                r.wrapping_add(7).write_unaligned(0);
                                r.wrapping_add(8).cast::<u8>().write(0x39);
                                r.wrapping_add(0x88).write_unaligned(1);
                                r.wrapping_add(0x89).write_unaligned(0);
                                r.wrapping_add(0x8d).write_unaligned(0);
                                r.wrapping_add(0x8a).write_unaligned(0);
                                r.wrapping_add(0x8b).write_unaligned(0);
                                r.wrapping_add(0x8c).write_unaligned(0);
                            }
                            p2 = p2.wrapping_add(0x8e);
                            i3 = i3.wrapping_add(1);
                            if DAT_00b69a89 <= i3 {
                                break;
                            }
                        }
                    }
                }
                p6 = p6.wrapping_add(0x8e);
                i5 = i5.wrapping_add(1);
                if !(i5 < DAT_00b69a89.wrapping_sub(1)) {
                    break;
                }
            }
        }
        DAT_00b69a89 = 0;
        let end: *mut i32 = &raw mut DAT_00b69851;
        let mut p6 = base;
        loop {
            if p6.read_unaligned() != 0 {
                DAT_00b69a89 = DAT_00b69a89.wrapping_add(1);
            }
            p6 = p6.wrapping_add(0x8e);
            if !(p6 < end) {
                break;
            }
        }
        1
    }
}
