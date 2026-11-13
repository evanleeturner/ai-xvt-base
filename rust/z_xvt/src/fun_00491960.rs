// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00658654: i32;
    static mut DAT_0065865c: [i8; 0];
    static mut DAT_006a8814: i32;
    static mut DAT_006a8614: [i32; 0];
    static mut DAT_006a8674: [i32; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _strncmp(s1: *const i8, s2: *const i8, n: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00491960(param_1: *mut i8) -> i32 {
    // SAFETY: globals and callee are the game's own memory; param_1 points at
    // the caller's string; the table entries hold COM-style object pointers.
    unsafe {
        if DAT_00658654 == 0 {
            return 0;
        }
        if param_1.read() == 0 {
            return 0;
        }
        let mut i_var2: i32 = 0;
        let mut pc_stack_8: *const i8 = (&raw mut DAT_0065865c).cast::<i8>();
        let mut i_var3: i32 = DAT_006a8814.wrapping_sub(1);
        let mut found = false;
        while i_var3 >= 0 {
            let i_var4 = i_var3 >> 1;
            let i_var1 = _strncmp(
                pc_stack_8.byte_offset((i_var4.wrapping_mul(0x145)) as isize),
                param_1,
                0x40,
            );
            if i_var1 == 0 {
                i_var2 = i_var2.wrapping_add(i_var4);
                found = true;
                break;
            }
            if i_var3 < 1 {
                break;
            }
            if i_var1 < 0 {
                i_var2 = i_var4.wrapping_add(1).wrapping_add(i_var2);
                i_var3 = i_var3.wrapping_sub(i_var4.wrapping_add(1));
                pc_stack_8 = pc_stack_8
                    .byte_offset((i_var4.wrapping_mul(0x145).wrapping_add(0x145)) as isize);
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
        let mut count: i32 = 0;
        let mut u_stack_4: u32 = 0;
        let mut p: *mut i32 = (&raw mut DAT_006a8614).cast::<i32>();
        let end: *mut i32 = (&raw mut DAT_006a8674).cast::<i32>();
        loop {
            if p.byte_sub(8).read_unaligned() == i_var2 {
                let obj = p.read_unaligned() as usize as *mut i32;
                let vt = obj.read_unaligned() as usize;
                let f = ((vt + 0x24) as *const i32).read_unaligned() as usize
                    as *const core::ffi::c_void;
                let f: unsafe extern "stdcall" fn(*mut i32, *mut u32) -> i32 =
                    core::mem::transmute(f);
                let r = f(obj, &raw mut u_stack_4);
                if r == 0 && ((u_stack_4 & 1) != 0 || (u_stack_4 & 4) != 0) {
                    count = count.wrapping_add(1);
                }
            }
            p = p.add(3);
            if !(p < end) {
                break;
            }
        }
        count
    }
}
