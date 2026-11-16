// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static DAT_0051be2c: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _ftell(stream: *mut i32) -> i32;
    fn _fscanf(stream: *mut i32, format: *const i8, ...) -> i32;
    fn _fseek(stream: *mut i32, offset: i32, origin: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00413b10(param_1: *mut i32) -> i32 {
    let mut local_1: i8 = 0;
    // SAFETY: the callees and the format array are the game's own; param_1 is
    // the caller's file handle, and local_1 is a valid destination for the scan.
    unsafe {
        let offset = _ftell(param_1);
        let i_var1 = _fscanf(
            param_1,
            (&raw const DAT_0051be2c).cast::<i8>(),
            &raw mut local_1,
        );
        if i_var1 != 1 {
            _fseek(param_1, offset, 0);
            return 0;
        }
        if local_1 == b'{' as i8 {
            _fseek(param_1, offset, 0);
            return 0;
        }
        _fseek(param_1, offset, 0);
        1
    }
}
