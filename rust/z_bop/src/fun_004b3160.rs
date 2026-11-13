// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00528c8c: i32;
    static mut DAT_00528c90: i32;
    static mut DAT_00528c88: i32;
    static mut DAT_00528c5c: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b3160(param_1: i32) {
    // SAFETY: the globals are the game's own; param_1 and the linked
    // nodes it references are game memory addresses accessed at the
    // same offsets as the original.
    unsafe {
        if param_1 == DAT_00528c8c {
            DAT_00528c8c = ((param_1.wrapping_add(0x94)) as *const i32).read_unaligned();
            if DAT_00528c8c == 0 {
                DAT_00528c90 = 0;
            } else {
                ((DAT_00528c8c.wrapping_add(0x90)) as *mut i32).write_unaligned(0);
                if ((DAT_00528c8c.wrapping_add(0x94)) as *const i32).read_unaligned() == 0 {
                    DAT_00528c90 = DAT_00528c8c;
                }
            }
        } else if param_1 == DAT_00528c90 {
            DAT_00528c90 = ((param_1.wrapping_add(0x90)) as *const i32).read_unaligned();
            ((DAT_00528c90.wrapping_add(0x94)) as *mut i32).write_unaligned(0);
        } else {
            let prev = ((param_1.wrapping_add(0x90)) as *const i32).read_unaligned();
            let next = ((param_1.wrapping_add(0x94)) as *const i32).read_unaligned();
            ((prev.wrapping_add(0x94)) as *mut i32).write_unaligned(next);
            let next2 = ((param_1.wrapping_add(0x94)) as *const i32).read_unaligned();
            let prev2 = ((param_1.wrapping_add(0x90)) as *const i32).read_unaligned();
            ((next2.wrapping_add(0x90)) as *mut i32).write_unaligned(prev2);
        }
        DAT_00528c88 = DAT_00528c88.wrapping_sub(1);
        let p = (DAT_00528c5c.wrapping_add(0x14c)) as *mut i32;
        let add = ((param_1.wrapping_add(0x88)) as *const i32).read_unaligned();
        p.write_unaligned(p.read_unaligned().wrapping_add(add));
    }
}
