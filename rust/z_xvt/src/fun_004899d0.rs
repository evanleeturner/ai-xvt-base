// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    fn __strcmpi(s1: *const i8, s2: *const i8) -> i32;
    fn FUN_0048e520(param_1: *mut u32, param_2: *mut i8) -> *mut u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004899d0(param_1: i32, param_2: *mut u32, param_3: i32, param_4: u32) {
    let mut param_2 = param_2;
    if param_2.is_null() {
        return;
    }
    // SAFETY: all pointers are the game's own data structures, read and
    // written exactly where the original does.
    unsafe {
        let p3_6: i32 = ((param_3 as usize + 6) as *const i32).read_unaligned();
        let p3_10: i32 = ((param_3 as usize + 10) as *const i32).read_unaligned();

        while param_2.add(1).read_unaligned() == 7 {
            let str2 = param_2.add(5).read_unaligned() as usize as *mut i8;
            let mut res: *mut u32 = core::ptr::null_mut();
            'search: {
                let mut local_4: i32 = 0;
                if 0 < p3_6 {
                    let mut local_8: i32 = 0;
                    loop {
                        let mut puvar2 = ((p3_10.wrapping_add(local_8)) as usize as *const u32)
                            .read_unaligned() as usize
                            as *mut u32;
                        if puvar2.is_null() {
                            puvar2 = core::ptr::null_mut();
                        } else {
                            let first = puvar2.read_unaligned();
                            if first == 0 || __strcmpi(first as usize as *const i8, str2) != 0 {
                                let mut ivar1: i32 = 0;
                                let mut ivar5: i32 = 0;
                                if 0 < puvar2.add(2).read_unaligned() as i32 {
                                    loop {
                                        let arr = puvar2.add(3).read_unaligned() as i32;
                                        let arg = (arr.wrapping_add(ivar1) as usize as *const u32)
                                            .read_unaligned()
                                            as usize
                                            as *mut u32;
                                        let r = FUN_0048e520(arg, str2);
                                        if !r.is_null() {
                                            res = r;
                                            break 'search;
                                        }
                                        ivar1 = ivar1.wrapping_add(4);
                                        ivar5 = ivar5.wrapping_add(1);
                                        if !(ivar5 < puvar2.add(2).read_unaligned() as i32) {
                                            break;
                                        }
                                    }
                                }
                                puvar2 = core::ptr::null_mut();
                            }
                        }
                        if !puvar2.is_null() {
                            res = puvar2;
                            break 'search;
                        }
                        local_8 = local_8.wrapping_add(4);
                        local_4 = local_4.wrapping_add(1);
                        if !(local_4 < p3_6) {
                            break;
                        }
                    }
                }
            }
            param_2 = res;
            if param_2.is_null() {
                return;
            }
        }

        if param_2.add(1).read_unaligned() == 0xd {
            let mut pf3 = param_2.add(5).read_unaligned() as usize as *mut f32;
            let mut local_8: i32 = 0;
            if 0 < param_2.add(4).read_unaligned() as i32 {
                loop {
                    let mut pf4 = ((param_1 as usize).wrapping_add(0x14) as *const u32)
                        .read_unaligned() as usize as *mut f32;
                    let mut ivar5: i32 = 0;
                    let ivar1: i32 =
                        ((param_1 as usize).wrapping_add(0x10) as *const i32).read_unaligned();
                    let mut b6 = ivar1 == 0;
                    if 0 < ivar1 {
                        loop {
                            if pf4.read_unaligned() == pf3.read_unaligned()
                                && pf4.add(1).read_unaligned() == pf3.add(1).read_unaligned()
                            {
                                break;
                            }
                            pf4 = pf4.add(2);
                            ivar5 = ivar5.wrapping_add(1);
                            if !(ivar5 < ivar1) {
                                break;
                            }
                        }
                        b6 = ivar1 == ivar5;
                    }
                    if b6 {
                        pf4.write_unaligned(pf3.read_unaligned());
                        pf4.add(1).write_unaligned(pf3.add(1).read_unaligned());
                        let cnt =
                            ((param_1 as usize).wrapping_add(0x10) as *const i32).read_unaligned();
                        ((param_1 as usize).wrapping_add(0x10) as *mut i32)
                            .write_unaligned(cnt.wrapping_add(1));
                    }
                    pf3 = pf3.add(2);
                    local_8 = local_8.wrapping_add(1);
                    if !(local_8 < param_2.add(4).read_unaligned() as i32) {
                        break;
                    }
                }
            }
        }

        let mut ivar1: i32 = 0;
        let mut ivar5: i32 = 0;
        if 0 < param_2.add(2).read_unaligned() as i32 {
            loop {
                ivar1 = ivar1.wrapping_add(4);
                ivar5 = ivar5.wrapping_add(1);
                let arr = param_2.add(3).read_unaligned() as i32;
                let child = (arr.wrapping_add(ivar1).wrapping_sub(4) as usize as *const u32)
                    .read_unaligned() as usize as *mut u32;
                FUN_004899d0(param_1, child, param_3, param_4);
                if !(ivar5 < param_2.add(2).read_unaligned() as i32) {
                    break;
                }
            }
        }
    }
}
