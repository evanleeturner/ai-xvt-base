// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00658654: i32;
    static mut DAT_0065865c: i8;
    static mut DAT_006a8814: i32;
    static mut DAT_006a8610: i32;
    static mut DAT_006a8614: i32;
    static mut DAT_006a8670: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _strncmp(a: *mut i8, b: *mut i8, n: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00491370(param_1: *mut i8) -> i32 {
    // SAFETY: the globals and callees are the game's own memory; param_1 is a
    // caller-supplied C string, and the table and object pointers are read
    // exactly where the original reads them.
    unsafe {
        let unaff_esi: i32 = core::mem::MaybeUninit::<i32>::uninit().assume_init();
        let mut au_stack_8: [i8; 4] = core::mem::MaybeUninit::<[i8; 4]>::uninit().assume_init();

        if DAT_00658654 == 0 {
            return 0;
        }
        if *param_1 == 0 {
            return 0;
        }
        let mut i_var3: i32 = 0;
        let mut pc_stack_4: *mut i8 = &raw mut DAT_0065865c;
        let mut i_var4: i32 = DAT_006a8814.wrapping_sub(1);
        let i_var5: i32;
        'found: {
            while -1 < i_var4 {
                let mid = i_var4 >> 1;
                let str1 = pc_stack_4.wrapping_offset((mid as isize).wrapping_mul(0x145));
                let i_var1 = _strncmp(str1, param_1, 0x40);
                if i_var1 == 0 {
                    i_var5 = mid.wrapping_add(i_var3);
                    break 'found;
                }
                if i_var4 < 1 {
                    break;
                }
                if i_var1 < 0 {
                    i_var3 = mid.wrapping_add(1).wrapping_add(i_var3);
                    i_var4 = i_var4.wrapping_sub(mid.wrapping_add(1));
                    pc_stack_4 = str1.wrapping_offset(0x145);
                } else {
                    i_var4 = mid.wrapping_add(-1);
                }
            }
            i_var5 = -1;
        }
        if i_var5 == -1 {
            return 0;
        }
        let mut best: i32 = -1;
        let mut idx: i32 = 0;
        let mut found: i32 = -1;
        let mut p: *mut i32 = &raw mut DAT_006a8610;
        let end: *mut i32 = &raw mut DAT_006a8670;
        loop {
            let prev = p.wrapping_offset(-1).read_unaligned();
            let cur = p.read_unaligned();
            if prev == i_var5 && best < cur {
                best = cur;
                found = idx;
            }
            p = p.wrapping_add(3);
            idx = idx.wrapping_add(1);
            if p >= end {
                break;
            }
        }
        if found == -1 {
            return 0;
        }
        let base = (&raw mut DAT_006a8614) as *const i32;
        let obj = base
            .wrapping_offset((found as isize).wrapping_mul(3))
            .read_unaligned();
        let vt = (obj as u32 as usize as *const i32).read_unaligned();
        let addr = vt.wrapping_add(0x18) as u32 as usize;
        let f: unsafe extern "stdcall" fn(*mut i32, *mut i8) -> i32 =
            core::mem::transmute(addr as *const core::ffi::c_void);
        let r = f(obj as u32 as usize as *mut i32, au_stack_8.as_mut_ptr());
        if r != 0 {
            return 0;
        }
        (unaff_esi.wrapping_mul(0x7f) / 2000).wrapping_add(0x7f)
    }
}
