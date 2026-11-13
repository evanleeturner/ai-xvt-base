// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00523574: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    #[link_name = "fscanf"]
    fn _fscanf(param_1: i32, param_2: *const i8, ...) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00405230(param_1: i32) {
    let mut local_1: i8 = 0;
    loop {
        // SAFETY: the format string and fscanf are the game's own; local_1
        // is a valid one-byte buffer for the scanned character.
        let i_var1 = unsafe {
            _fscanf(
                param_1,
                (&raw mut DAT_00523574).cast::<i8>() as *const i8,
                &raw mut local_1,
            )
        };
        if i_var1 != 1 {
            return;
        }
        if local_1 == b'}' as i8 {
            return;
        }
    }
}
