// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00528c8c: i32;
    static mut DAT_00528c90: i32;
    static mut _DAT_00528c88: i32;
    static mut DAT_00528c5c: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b30f0(param_1: i32) {
    // SAFETY: the globals are the game's own; param_1 and the pointers
    // derived from the globals address the game's list nodes and structs,
    // read and written where the original does.
    unsafe {
        if DAT_00528c8c == 0 {
            DAT_00528c90 = param_1;
            DAT_00528c8c = param_1;
            ((param_1 as u32).wrapping_add(0x90) as *mut i32).write_unaligned(0);
            ((param_1 as u32).wrapping_add(0x94) as *mut i32).write_unaligned(0);
        } else {
            ((DAT_00528c90 as u32).wrapping_add(0x94) as *mut i32).write_unaligned(param_1);
            ((param_1 as u32).wrapping_add(0x94) as *mut i32).write_unaligned(0);
            ((param_1 as u32).wrapping_add(0x90) as *mut i32).write_unaligned(DAT_00528c90);
            DAT_00528c90 = param_1;
        }
        _DAT_00528c88 = _DAT_00528c88.wrapping_add(1);
        let p = (DAT_00528c5c as u32).wrapping_add(0x14c) as *mut i32;
        let sub = ((param_1 as u32).wrapping_add(0x88) as *const i32).read_unaligned();
        p.write_unaligned(p.read_unaligned().wrapping_sub(sub));
    }
}
