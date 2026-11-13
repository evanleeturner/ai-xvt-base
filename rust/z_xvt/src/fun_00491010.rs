// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00658654: i32;
    static mut DAT_0065865c: i32;
    static mut DAT_006a8814: i32;
    static mut DAT_006a8818: i32;
    static mut DAT_006a881c: i32;
    static mut DAT_006a8610: i32;
    static mut DAT_006a8614: i32;
    static mut DAT_006a860c: i32;
    static mut DAT_006a8670: i32;
}

unsafe extern "C" {
    fn strncmp(s1: *const i8, s2: *const i8, n: u32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00491010(param_1: *mut i8) -> u8 {
    // SAFETY: the globals, the strncmp import and the object vtables are the
    // game's own memory; param_1 points at the caller's string, read where
    // the original reads it.
    unsafe {
        if DAT_00658654 == 0 {
            return 0;
        }
        if param_1.read() == 0 {
            return 0;
        }
        let mut i_var2: i32 = 0;
        let mut pc_stack_4: *const i8 = (&raw mut DAT_0065865c).cast::<i8>() as *const i8;
        let mut i_var3: i32 = DAT_006a8814.wrapping_sub(1);
        let found: bool = 'search: {
            while -1 < i_var3 {
                let i_var4 = i_var3 >> 1;
                let i_var1 = strncmp(
                    pc_stack_4.byte_offset(i_var4.wrapping_mul(0x145) as isize),
                    param_1 as *const i8,
                    0x40,
                );
                if i_var1 == 0 {
                    i_var2 = i_var2.wrapping_add(i_var4);
                    break 'search true;
                }
                if i_var3 < 1 {
                    break;
                }
                if i_var1 < 0 {
                    i_var2 = i_var4.wrapping_add(1).wrapping_add(i_var2);
                    i_var3 = i_var3.wrapping_sub(i_var4.wrapping_add(1));
                    pc_stack_4 = pc_stack_4
                        .byte_offset(i_var4.wrapping_mul(0x145).wrapping_add(0x145) as isize);
                } else {
                    i_var3 = i_var4.wrapping_sub(1);
                }
            }
            false
        };
        if !found {
            i_var2 = -1;
        }
        if i_var2 == -1 {
            return 0;
        }
        let mut i_var1: i32 = -1;
        let mut i_var3: i32 = DAT_006a881c.wrapping_add(1);
        let mut i_var4: i32 = 0;
        let mut pi_var5: *mut i32 = &raw mut DAT_006a8610;
        let end: *mut i32 = &raw mut DAT_006a8670;
        loop {
            if pi_var5.offset(-1).read() == i_var2 && pi_var5.read() < i_var3 {
                i_var3 = pi_var5.read();
                i_var1 = i_var4;
            }
            pi_var5 = pi_var5.add(3);
            i_var4 = i_var4.wrapping_add(1);
            if !(pi_var5 < end) {
                break;
            }
        }
        if i_var1 == -1 {
            return 0;
        }
        let tab14: *mut i32 = (&raw mut DAT_006a8614).cast::<i32>();
        let tab0c: *mut i32 = (&raw mut DAT_006a860c).cast::<i32>();
        let idx = i_var1.wrapping_mul(3) as isize;
        let pi_var5 = tab14.offset(idx).read() as usize as *mut i32;
        if pi_var5.is_null() {
            return 0;
        }
        let vtable = pi_var5.read() as usize as *const *const u8;
        let f1 = vtable.read().byte_add(0x48);
        let f1: unsafe extern "C" fn(*mut i32) -> i32 = core::mem::transmute(f1);
        let i_var3 = f1(pi_var5);
        let vtable = pi_var5.read() as usize as *const *const u8;
        let f2 = vtable.read().byte_add(8);
        let f2: unsafe extern "C" fn(*mut i32) = core::mem::transmute(f2);
        f2(pi_var5);
        tab14.offset(idx).write(0);
        tab0c.offset(idx).write(-1);
        DAT_006a8818 = DAT_006a8818.wrapping_sub(1);
        (-1 < i_var3) as u8
    }
}
