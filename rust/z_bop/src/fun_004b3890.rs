// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a90bf0: i32;
    static mut DAT_00a90bf4: i32;
    static mut DAT_00a90bf8: i32;
    static mut DAT_00a90bfc: i32;
    static mut DAT_00a90b60: f32;
    static mut DAT_00a90b64: f32;
    static mut _DAT_00a90b80: f32;
    static mut _DAT_00a90b84: f32;
    static mut _DAT_00a90ba0: f32;
    static mut _DAT_00a90b34: i32;
    static mut _DAT_00a90b30: i32;
    static mut _DAT_00a90b38: i32;
    static mut _DAT_00a90ba4: f32;
    static mut _DAT_00a90bc0: f32;
    static mut _DAT_00a90bc4: f32;
    static mut _DAT_00a90b40: i32;
    static mut _DAT_00a90b3c: i32;
    static mut _DAT_00a90b44: i32;
    static mut _DAT_00a90b48: i32;
    static mut _DAT_00a90b54: i32;
    static mut _DAT_00a90b4c: i32;
    static mut _DAT_00a90b50: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b3890(param_1: *mut i32) {
    // SAFETY: the globals are the game's own memory; param_1 points at four
    // caller-provided ints; the zeroing loop covers 0x20 floats from
    // DAT_00a90b60 exactly as the original does.
    unsafe {
        DAT_00a90bf0 = param_1.read_unaligned();
        DAT_00a90bf4 = param_1.add(1).read_unaligned();
        DAT_00a90bf8 = param_1.add(2).read_unaligned();
        DAT_00a90bfc = param_1.add(3).read_unaligned();
        let mut p = &raw mut DAT_00a90b60;
        let mut i_var1: i32 = 0x20;
        while i_var1 != 0 {
            p.write_unaligned(0.0);
            p = p.add(1);
            i_var1 = i_var1.wrapping_sub(1);
        }
        DAT_00a90b60 = DAT_00a90bf0 as f32;
        DAT_00a90b64 = DAT_00a90bf4 as f32;
        _DAT_00a90b80 = DAT_00a90bf8.wrapping_add(DAT_00a90bf0) as f32;
        _DAT_00a90b84 = DAT_00a90bf4 as f32;
        _DAT_00a90ba0 = DAT_00a90bf8.wrapping_add(DAT_00a90bf0) as f32;
        _DAT_00a90b34 = 1;
        _DAT_00a90b30 = 0;
        _DAT_00a90b38 = 2;
        _DAT_00a90ba4 = DAT_00a90bf4.wrapping_add(DAT_00a90bfc) as f32;
        _DAT_00a90bc0 = DAT_00a90bf0 as f32;
        _DAT_00a90bc4 = DAT_00a90bf4.wrapping_add(DAT_00a90bfc) as f32;
        _DAT_00a90b40 = 0;
        _DAT_00a90b3c = 0x8200;
        _DAT_00a90b44 = 0;
        _DAT_00a90b48 = 2;
        _DAT_00a90b54 = 0;
        _DAT_00a90b4c = 3;
        _DAT_00a90b50 = 0x8200;
    }
}
