// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052f1e8: *mut i32;
    static mut DAT_0052f1ec: *mut i32;
    static mut DAT_0052f1e4: i32;
    static mut DAT_00742820: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b8761(param_1: *mut i32) {
    // SAFETY: the globals are the game's own; param_1 points at a node of at
    // least five ints, and the list tail is a valid node when the head is set.
    unsafe {
        if DAT_0052f1e8.is_null() {
            DAT_0052f1ec = param_1;
            DAT_0052f1e8 = param_1;
            param_1.write_unaligned(0);
            param_1.add(1).write_unaligned(0);
        } else {
            DAT_0052f1ec
                .byte_add(4)
                .cast::<*mut i32>()
                .write_unaligned(param_1);
            param_1.write_unaligned(DAT_0052f1ec as usize as i32);
            param_1.add(1).write_unaligned(0);
            DAT_0052f1ec = param_1;
        }
        DAT_0052f1e4 = DAT_0052f1e4.wrapping_add(1);
        DAT_00742820 = DAT_00742820.wrapping_sub(param_1.add(4).read_unaligned());
    }
}
