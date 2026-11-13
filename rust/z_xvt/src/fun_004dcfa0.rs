// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0081b5d2: u32;
    static mut DAT_00532bc8: *mut i8;
    static mut DAT_0081b5ca: u32;
    static mut DAT_0081b5ce: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004dcfa0(param_1: u32) -> *mut i8 {
    // SAFETY: the globals are the game's own memory; the table address held
    // in DAT_0081b5ca is read at the index the original uses.
    unsafe {
        if DAT_0081b5d2 <= param_1 {
            return DAT_00532bc8;
        }
        let entry = DAT_0081b5ca.wrapping_add(param_1.wrapping_mul(4)) as *const i32;
        let off = entry.read_unaligned();
        off.wrapping_add(DAT_0081b5ce as i32) as u32 as *mut i8
    }
}
