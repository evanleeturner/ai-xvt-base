// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00658654: *mut i32;
    static mut DAT_0065865c: [i8; 1];
    static mut DAT_006a8814: i32;
    static mut DAT_006a860c: [i32; 1];
    static mut DAT_006a866c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn strncmp(a: *const i8, b: *const i8, n: u32) -> i32;
    fn strncpy(a: *mut i8, b: *const i8, n: u32) -> *mut i8;
    fn FUN_00417a40(param_1: *mut i32, param_2: *mut i8, param_3: i32) -> *mut i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00490550(param_1: *mut i8, param_2: *mut i8, param_3: i32) -> u8 {
    // SAFETY: globals and callees are the game's own memory; pointers are the
    // caller's buffers, accessed where the original does.
    unsafe {
        if *param_1 == 0 {
            return 0;
        }
        if *param_2 == 0 {
            return 0;
        }
        if 999 < DAT_006a8814 {
            return 0;
        }
        if DAT_00658654.is_null() {
            return 0;
        }
        let base = &raw mut DAT_0065865c as *mut i8;
        let mut ivar4: i32 = 0;
        let mut local_14c: *const i8 = base;
        let mut ivar5: i32 = DAT_006a8814.wrapping_sub(1);
        let mut ivar7: i32;
        'search: {
            while -1 < ivar5 {
                ivar7 = ivar5 >> 1;
                let ivar1 = strncmp(
                    local_14c.wrapping_offset((ivar7 as isize).wrapping_mul(0x145)),
                    param_2,
                    0x40,
                );
                if ivar1 == 0 {
                    ivar7 = ivar7.wrapping_add(ivar4);
                    break 'search;
                }
                if ivar5 < 1 {
                    break;
                }
                if ivar1 < 0 {
                    ivar4 = ivar7.wrapping_add(1).wrapping_add(ivar4);
                    ivar5 = ivar5.wrapping_sub(ivar7.wrapping_add(1));
                    local_14c = local_14c
                        .wrapping_offset((ivar7 as isize).wrapping_mul(0x145).wrapping_add(0x145));
                } else {
                    ivar5 = ivar7.wrapping_sub(1);
                }
            }
            ivar7 = -1;
        }
        if ivar7 != -1 {
            return 0;
        }
        ivar5 = 0;
        let local_8 = FUN_00417a40(DAT_00658654, param_1, param_3);
        if local_8.is_null() {
            return 0;
        }
        let vt = local_8.read() as u32;
        let f = ((vt.wrapping_add(0x34)) as *const u32).read_unaligned();
        let f: unsafe extern "stdcall" fn(*mut i32, i32) =
            core::mem::transmute(f as usize as *const core::ffi::c_void);
        f(local_8, 0);

        let mut buf = core::mem::MaybeUninit::<[u32; 0x52]>::uninit();
        let l150 = buf.as_mut_ptr() as *mut i8;
        strncpy(l150, param_2, 0x40);
        strncpy(l150.add(0x40), param_1, 0x100);
        *l150.add(0xFF) = 0;
        *l150.add(0x144) = 0;

        if 0 < DAT_006a8814 {
            let mut str2: *const i8 = base;
            loop {
                let r = strncmp(l150, str2, 0x40);
                if r < 0 {
                    break;
                }
                str2 = str2.wrapping_add(0x145);
                ivar5 = ivar5.wrapping_add(1);
                if !(ivar5 < DAT_006a8814) {
                    break;
                }
            }
            if ivar5 < DAT_006a8814 {
                let mut cnt = DAT_006a8814.wrapping_sub(ivar5);
                let mut p = base.wrapping_offset((DAT_006a8814 as isize).wrapping_mul(0x145));
                loop {
                    core::ptr::copy_nonoverlapping(
                        p.wrapping_sub(0x145) as *const u8,
                        p as *mut u8,
                        0x145,
                    );
                    cnt = cnt.wrapping_sub(1);
                    p = p.wrapping_sub(0x145);
                    if cnt == 0 {
                        break;
                    }
                }
            }
        }
        core::ptr::copy_nonoverlapping(
            l150 as *const u8,
            base.wrapping_offset((ivar5 as isize).wrapping_mul(0x145)) as *mut u8,
            0x145,
        );
        let mut pivar2 = &raw mut DAT_006a860c as *mut i32;
        DAT_006a8814 = DAT_006a8814.wrapping_add(1);
        let end = &raw mut DAT_006a866c as *mut i32;
        loop {
            if ivar5 <= pivar2.read() {
                pivar2.write(pivar2.read().wrapping_add(1));
            }
            pivar2 = pivar2.wrapping_add(3);
            if !(pivar2 < end) {
                break;
            }
        }
        let istack_10 = (l150.add(0x140) as *const i32).read_unaligned();
        (1u32.wrapping_sub((istack_10 == 0) as u32)) as u8
    }
}
