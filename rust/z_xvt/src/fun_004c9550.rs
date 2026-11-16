// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00532310: [i8; 1];
    static mut DAT_0052a330: [i8; 1];
}

unsafe extern "C" {
    fn _fopen(filename: *const i8, mode: *const i8) -> i32;
    fn _fgets(buffer: *mut i8, count: i32, stream: i32) -> *mut i8;
    fn _fclose(stream: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004c9550(param_1: i32, param_2: *mut i8) -> i32 {
    let mut local_100: [i8; 0x200] = [0; 0x200];
    let mut fgets_ret: *mut i8 = core::ptr::null_mut();
    let mut param_2 = param_2;

    // SAFETY: the callees and globals are the game's own; param_2 points at
    // the caller's output buffer of at least 0xff bytes, and local_100 is
    // indexed at most to 0xff.
    unsafe {
        let file = _fopen(
            (&raw mut DAT_00532310).cast::<i8>(),
            (&raw mut DAT_0052a330).cast::<i8>(),
        );
        if file == 0 {
            return 0;
        }
        if -1 < param_1 {
            let mut i_var1 = param_1.wrapping_add(1);
            loop {
                fgets_ret = _fgets(local_100.as_mut_ptr(), 0x200, file);
                i_var1 = i_var1.wrapping_sub(1);
                if i_var1 == 0 {
                    break;
                }
            }
        }
        _fclose(file);
        if !fgets_ret.is_null() {
            let mut i: usize = 0;
            loop {
                if local_100[i] == b'\\' as i8 && local_100[i + 1] == b'n' as i8 {
                    *param_2 = b'\n' as i8;
                    i += 1;
                } else {
                    *param_2 = local_100[i];
                }
                param_2 = param_2.add(1);
                i += 1;
                if i >= 0xff {
                    break;
                }
            }
            return 1;
        }
        0
    }
}
