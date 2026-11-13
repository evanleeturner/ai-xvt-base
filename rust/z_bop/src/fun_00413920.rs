// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0051be2c: *const i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00413b80(param_1: i32) -> i32;
    fn _fscanf(stream: i32, format: *const i8, ...) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00413920(param_1: i32) {
    let mut local_1: i8 = 0;
    // SAFETY: the callees and the global are the game's own; local_1 is a
    // valid stack byte that fscanf writes through the passed address.
    unsafe {
        let i_var1 = FUN_00413b80(param_1);
        if i_var1 == 0 {
            loop {
                let r = _fscanf(param_1, DAT_0051be2c, &raw mut local_1);
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
