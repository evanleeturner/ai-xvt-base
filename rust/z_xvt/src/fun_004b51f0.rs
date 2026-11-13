// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005300f8: i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b51f0(param_1: i32, param_2: i32, param_3: i32) -> *mut i8 {
    let mut local_8: i32 = 0;
    loop {
        if param_3 <= local_8 {
            // SAFETY: the global is the game's own string; only its address is taken.
            return unsafe { &raw mut DAT_005300f8 };
        }
        // SAFETY: param_2 is the address of the caller's table of 8-byte entries.
        let key = unsafe {
            (param_2.wrapping_add(local_8.wrapping_mul(8)) as *const i32).read_unaligned()
        };
        if key == param_1 {
            break;
        }
        local_8 = local_8.wrapping_add(1);
    }
    // SAFETY: same table; reads the string pointer of the matching entry.
    unsafe {
        (param_2
            .wrapping_add(4)
            .wrapping_add(local_8.wrapping_mul(8)) as *const *mut i8)
            .read_unaligned()
    }
}
