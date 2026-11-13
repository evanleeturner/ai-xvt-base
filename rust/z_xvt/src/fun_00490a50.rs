// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00658654: i32;
    static mut DAT_0065865c: i8;
    static mut DAT_006a8814: i32;
    static mut DAT_006a8810: i32;
    static mut DAT_006a86bc: i32;
    static mut DAT_006a86c0: i8;
    static mut DAT_006a866c: i8;
    static mut DAT_006a86b4: i32;
    static mut DAT_006a86b8: i32;
    static mut DAT_006a86ac: i32;
    static mut DAT_006a86b0: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _strncmp(s1: *const i8, s2: *const i8, n: u32) -> i32;
    fn memcpy(
        dest: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        n: u32,
    ) -> *mut core::ffi::c_void;
    fn _strncpy(dest: *mut i8, src: *const i8, n: u32) -> *mut i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00490a50(
    param_1: *mut i8,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
) -> i32 {
    // SAFETY: the globals and callees are the game's own memory and
    // functions; param_1 points at the caller's string, and all table
    // offsets follow the original's layout.
    unsafe {
        if DAT_00658654 == 0 {
            return 0;
        }
        if *param_1 == 0 {
            return 0;
        }
        let mut i_var3: i32 = 0;
        let mut pc_stack_4: *mut i8 = &raw mut DAT_0065865c;
        let mut i_var4: i32 = DAT_006a8814.wrapping_sub(1);
        let mut i_var5: i32 = -1;
        while i_var4 >= 0 {
            let mid = i_var4 >> 1;
            let i_var1 = _strncmp(
                pc_stack_4.wrapping_offset(mid.wrapping_mul(0x145) as isize),
                param_1,
                0x40,
            );
            if i_var1 == 0 {
                i_var5 = mid.wrapping_add(i_var3);
                break;
            }
            if i_var4 < 1 {
                break;
            }
            if i_var1 < 0 {
                i_var3 = mid.wrapping_add(1).wrapping_add(i_var3);
                i_var4 = i_var4.wrapping_sub(mid.wrapping_add(1));
                pc_stack_4 = pc_stack_4
                    .wrapping_offset(mid.wrapping_mul(0x145).wrapping_add(0x145) as isize);
            } else {
                i_var4 = mid.wrapping_sub(1);
            }
        }
        if i_var5 == -1 {
            return 0;
        }
        i_var4 = 0;
        if DAT_006a8810 > 0 {
            let mut pi_var2: *mut i32 = &raw mut DAT_006a86bc;
            loop {
                if pi_var2.read() < param_4 {
                    break;
                }
                pi_var2 = pi_var2.wrapping_add(0x15);
                i_var4 = i_var4.wrapping_add(1);
                if !(i_var4 < DAT_006a8810) {
                    break;
                }
            }
        }
        if i_var4 == DAT_006a8810 && i_var4 == 4 {
            return 0;
        }
        let off = i_var4.wrapping_mul(0x54) as isize;
        memcpy(
            (&raw mut DAT_006a86c0).wrapping_offset(off).cast(),
            (&raw mut DAT_006a866c)
                .wrapping_offset(off)
                .cast_const()
                .cast(),
            DAT_006a8810.wrapping_sub(i_var4).wrapping_mul(0x54) as u32,
        );
        _strncpy((&raw mut DAT_006a866c).wrapping_offset(off), param_1, 0x40);
        let idx = i_var4.wrapping_mul(0x15) as isize;
        (&raw mut DAT_006a86b4).wrapping_offset(idx).write(param_3);
        (&raw mut DAT_006a86b8).wrapping_offset(idx).write(param_2);
        (&raw mut DAT_006a86bc).wrapping_offset(idx).write(param_4);
        (&raw mut DAT_006a86ac).wrapping_offset(idx).write(param_5);
        (&raw mut DAT_006a86b0).wrapping_offset(idx).write(param_6);
        DAT_006a8810 = DAT_006a8810.wrapping_add(1);
        if 4 < DAT_006a8810 {
            DAT_006a8810 = 4;
        }
        1
    }
}
