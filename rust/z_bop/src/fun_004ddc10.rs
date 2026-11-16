// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b69a9d: u32;
    static mut DAT_0052c0f0: *mut i8;
    static mut DAT_00b69a95: u32;
    static mut DAT_00b69a99: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004ddc10(param_1: u32) -> *mut i8 {
    // SAFETY: the globals are the game's own memory; the table address held
    // in DAT_00b69a95 is read at the index the original uses.
    unsafe {
        if DAT_00b69a9d <= param_1 {
            return DAT_0052c0f0;
        }
        let addr = DAT_00b69a95.wrapping_add(param_1.wrapping_mul(4));
        let v = (addr as *const i32).read_unaligned();
        v.wrapping_add(DAT_00b69a99) as u32 as usize as *mut i8
    }
}
