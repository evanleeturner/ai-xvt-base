// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00658654: i32;
    static mut DAT_0065865c: u8;
    static mut DAT_006a8814: i32;
    static mut DAT_006a8610: i32;
    static mut DAT_006a8614: i32;
    static mut DAT_006a8670: i32;
}

unsafe extern "C" {
    fn strncmp(s1: *const i8, s2: *const i8, n: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00491700(param_1: *mut i8, param_2: u32) -> u8 {
    // SAFETY: the globals and callee are the game's own memory; param_1 is the
    // caller's string, and the table entries and object pointers are read
    // exactly as the original does.
    unsafe {
        if DAT_00658654 == 0 {
            return 0;
        }
        if *param_1 == 0 {
            return 0;
        }
        let mut i_var2: i32 = 0;
        let mut pc_stack_4: *const u8 = &raw mut DAT_0065865c as *const u8;
        let mut i_var4: i32 = DAT_006a8814.wrapping_sub(1);
        let mut i_var5: i32;
        'found: {
            while -1 < i_var4 {
                i_var5 = i_var4 >> 1;
                let i_var1 = strncmp(
                    pc_stack_4.wrapping_offset((i_var5.wrapping_mul(0x145)) as isize) as *const i8,
                    param_1 as *const i8,
                    0x40,
                );
                if i_var1 == 0 {
                    i_var5 = i_var5.wrapping_add(i_var2);
                    break 'found;
                }
                if i_var4 < 1 {
                    break;
                }
                if i_var1 < 0 {
                    i_var2 = i_var5.wrapping_add(1).wrapping_add(i_var2);
                    i_var4 = i_var4.wrapping_sub(i_var5.wrapping_add(1));
                    pc_stack_4 = pc_stack_4
                        .wrapping_offset((i_var5.wrapping_mul(0x145).wrapping_add(0x145)) as isize);
                } else {
                    i_var4 = i_var5.wrapping_sub(1);
                }
            }
            i_var5 = -1;
        }
        if i_var5 == -1 {
            return 0;
        }
        i_var4 = -1;
        let mut i_var1: i32 = 0;
        let mut i_var2: i32 = -1;
        let mut p_var3: *const i32 = &raw mut DAT_006a8610 as *const i32;
        let end: *const i32 = &raw mut DAT_006a8670 as *const i32;
        loop {
            let prev = p_var3.byte_sub(4).read_unaligned();
            let cur = p_var3.read_unaligned();
            if prev == i_var5 && i_var4 < cur {
                i_var4 = cur;
                i_var2 = i_var1;
            }
            p_var3 = p_var3.wrapping_add(3);
            i_var1 = i_var1.wrapping_add(1);
            if !(p_var3 < end) {
                break;
            }
        }
        if i_var2 == -1 {
            return 0;
        }
        let elem = (&raw mut DAT_006a8614)
            .byte_offset((i_var2.wrapping_mul(12)) as isize)
            .read_unaligned();
        let obj = (elem as u32 as usize as *const i32).read_unaligned();
        let fptr = (obj.wrapping_add(0x44) as u32 as usize as *const u32).read_unaligned();
        let f: unsafe extern "stdcall" fn(i32, u32) -> i32 =
            core::mem::transmute(fptr as usize as *const core::ffi::c_void);
        let r = f(obj, param_2);
        (r == 0) as u8
    }
}
