// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_005f09a8: f32;
    static mut _DAT_005f09ac: f32;
    static mut _DAT_005f09b0: f32;
    static mut _DAT_005f09b4: f32;
    static mut _DAT_005f09c8: f32;
    static mut _DAT_005f09cc: f32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004536c0(param_1: i32, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004537e0(param_1: i32, param_2: i32) -> f32 {
    // SAFETY: the globals and the callee are the game's own; param_1 is an
    // address of the caller's structure, read where the original reads it.
    unsafe {
        let mut i_var2: i32 = 0;
        _DAT_005f09b0 = 0.0;
        _DAT_005f09b4 = 0.0;
        _DAT_005f09c8 = 0.0;
        _DAT_005f09cc = 0.0;
        _DAT_005f09a8 = 0.0;
        _DAT_005f09ac = 0.0;
        let count: i32 = (param_1.wrapping_add(6) as *const i32).read_unaligned();
        let array_base: i32 = (param_1.wrapping_add(10) as *const i32).read_unaligned();
        if count > 0 {
            let mut i_var3: i32 = 0;
            loop {
                let elem = (array_base.wrapping_add(i_var3) as *const i32).read_unaligned();
                i_var3 = i_var3.wrapping_add(4);
                i_var2 = i_var2.wrapping_add(1);
                FUN_004536c0(elem, param_1);
                if !(i_var2 < count) {
                    break;
                }
            }
        }
        _DAT_005f09b0 = _DAT_005f09b0 - _DAT_005f09b4;
        _DAT_005f09c8 = _DAT_005f09c8 - _DAT_005f09cc;
        _DAT_005f09a8 = _DAT_005f09a8 - _DAT_005f09ac;
        if param_2 != 0 {
            let mut local_8: f32 = 0.0;
            if param_2 == 1 {
                local_8 = _DAT_005f09b0;
            }
            if param_2 == 2 {
                local_8 = _DAT_005f09c8;
            }
            if param_2 == 3 {
                local_8 = _DAT_005f09a8;
            }
            return local_8;
        }
        if (_DAT_005f09c8 < _DAT_005f09b0) && (_DAT_005f09a8 < _DAT_005f09b0) {
            return _DAT_005f09b0;
        }
        if (_DAT_005f09b0 < _DAT_005f09c8) && (_DAT_005f09a8 < _DAT_005f09c8) {
            return _DAT_005f09c8;
        }
        _DAT_005f09a8
    }
}
