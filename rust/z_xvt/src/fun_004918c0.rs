// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0065865c: i32;
    static mut DAT_006a8814: i32;
    static mut DAT_006587a0: i32;
}

unsafe extern "C" {
    fn strncmp(a: *const i8, b: *const i8, n: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004918c0(param_1: *mut i8) -> u8 {
    // SAFETY: the globals and the table they describe are the game's own
    // memory; param_1 points at the caller's string, compared and indexed
    // exactly as the original does.
    unsafe {
        let mut local_4: *const i8 = (&raw mut DAT_0065865c) as *const i8;
        let mut i_var3: i32 = DAT_006a8814.wrapping_sub(1);
        let mut i_var2: i32 = 0;
        let mut i_var4: i32;
        loop {
            if !(-1 < i_var3) {
                i_var4 = -1;
                break;
            }
            i_var4 = i_var3 >> 1;
            let i_var1 = strncmp(
                local_4.wrapping_byte_offset(i_var4.wrapping_mul(0x145) as isize),
                param_1 as *const i8,
                0x40,
            );
            if i_var1 == 0 {
                i_var4 = i_var4.wrapping_add(i_var2);
                break;
            }
            if i_var3 < 1 {
                i_var4 = -1;
                break;
            }
            if i_var1 < 0 {
                i_var2 = i_var4.wrapping_add(1).wrapping_add(i_var2);
                i_var3 = i_var3.wrapping_sub(i_var4.wrapping_add(1));
                local_4 = local_4
                    .wrapping_byte_offset(i_var4.wrapping_mul(0x145).wrapping_add(0x145) as isize);
            } else {
                i_var3 = i_var4.wrapping_sub(1);
            }
        }
        if i_var4 == -1 {
            return 0;
        }
        let p = (&raw mut DAT_006587a0).wrapping_offset(i_var4.wrapping_mul(0x145) as isize);
        p.read_unaligned() as u8
    }
}
