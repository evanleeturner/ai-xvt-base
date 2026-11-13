// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052f1e8: *mut i32;
    static mut DAT_0052f1ec: *mut i32;
    static mut _DAT_0052f1e4: i32;
    static mut DAT_00742820: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b87e1(param_1: *mut i32) {
    // SAFETY: the globals and the list nodes are the game's own memory;
    // param_1 points at a node of the game's list, accessed where the original does.
    unsafe {
        if param_1 == DAT_0052f1e8 {
            DAT_0052f1e8 = param_1.byte_add(4).cast::<i32>().read_unaligned() as usize as *mut i32;
            if DAT_0052f1e8.is_null() {
                DAT_0052f1ec = core::ptr::null_mut();
            } else {
                DAT_0052f1e8.cast::<i32>().write_unaligned(0);
                if DAT_0052f1e8.byte_add(4).cast::<i32>().read_unaligned() == 0 {
                    DAT_0052f1ec = DAT_0052f1e8;
                }
            }
        } else if param_1 == DAT_0052f1ec {
            DAT_0052f1ec = param_1.cast::<i32>().read_unaligned() as usize as *mut i32;
            DAT_0052f1ec.byte_add(4).cast::<i32>().write_unaligned(0);
        } else {
            let a = param_1.cast::<i32>().read_unaligned();
            let b = param_1.byte_add(4).cast::<i32>().read_unaligned();
            (a as usize as *mut u8)
                .byte_add(4)
                .cast::<i32>()
                .write_unaligned(b);
            (b as usize as *mut i32).write_unaligned(a);
        }
        _DAT_0052f1e4 = _DAT_0052f1e4.wrapping_sub(1);
        DAT_00742820 =
            DAT_00742820.wrapping_add(param_1.byte_add(16).cast::<i32>().read_unaligned());
    }
}
