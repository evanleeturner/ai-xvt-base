// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0065865c: [i8; 0];
    static mut DAT_006a8814: i32;
    static mut DAT_006587a0: [i8; 0];
}

unsafe extern "C" {
    fn strncmp(s1: *const i8, s2: *const i8, n: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00491810(param_1: *mut i8, param_2: i32) -> i32 {
    // SAFETY: the globals and strncmp are the game's own; the table is
    // indexed exactly as the original does.
    unsafe {
        let mut param_2 = param_2;
        let mut local_4: *const i8 = (&raw mut DAT_0065865c).cast::<i8>();
        let mut i_var3: i32 = DAT_006a8814.wrapping_sub(1);
        let mut i_var2: i32 = 0;
        let mut found = false;
        while -1 < i_var3 {
            let i_var4 = i_var3 >> 1;
            let off = (i_var4.wrapping_mul(0x145)) as isize;
            let i_var1 = strncmp(local_4.byte_offset(off), param_1, 0x40);
            if i_var1 == 0 {
                i_var2 = i_var2.wrapping_add(i_var4);
                found = true;
                break;
            }
            if i_var3 < 1 {
                break;
            }
            if i_var1 >= 0 {
                i_var2 = i_var4.wrapping_add(1).wrapping_add(i_var2);
                i_var3 = i_var3.wrapping_sub(i_var4.wrapping_add(1));
                local_4 = local_4.byte_offset(off.wrapping_add(0x145));
            } else {
                i_var3 = i_var4.wrapping_sub(1);
            }
        }
        if !found {
            i_var2 = -1;
        }
        if i_var2 == -1 {
            return 0;
        }
        if 0xff < param_2 {
            param_2 = 0xff;
        }
        if param_2 < 0 {
            param_2 = 0;
        }
        (&raw mut DAT_006587a0)
            .cast::<i8>()
            .byte_offset(i_var2.wrapping_mul(0x145) as isize)
            .write(param_2 as i8);
        1
    }
}
