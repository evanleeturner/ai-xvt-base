// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00523574: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _ftell(stream: i32) -> i32;
    fn _fscanf(stream: i32, format: *const i8, ...) -> i32;
    fn _fseek(stream: i32, offset: i32, origin: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00405160(param_1: i32) {
    let mut local_2: i8 = 0;
    let mut local_1: i8 = 0;
    // SAFETY: the callees and the format global are the game's own; the
    // local buffers are valid for the single char each scan writes.
    unsafe {
        let offset = _ftell(param_1);
        let r = _fscanf(
            param_1,
            (&raw mut DAT_00523574).cast::<i8>(),
            &raw mut local_2,
        );
        let b_var1: i32;
        if r == 1 && local_2 == b']' as i8 {
            _fseek(param_1, offset, 0);
            b_var1 = 1;
        } else {
            _fseek(param_1, offset, 0);
            b_var1 = 0;
        }
        if b_var1 == 0 {
            loop {
                let r = _fscanf(
                    param_1,
                    (&raw mut DAT_00523574).cast::<i8>(),
                    &raw mut local_1,
                );
                if r != 1 {
                    return;
                }
                if local_1 == b',' as i8 {
                    break;
                }
            }
        }
    }
}
