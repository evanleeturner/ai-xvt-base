// SPDX-License-Identifier: MIT

unsafe extern "C" {
    fn FUN_0048e520(param_1: *mut u32, param_2: *const i8) -> *mut u32;
    fn __strcmpi(s1: *const i8, s2: *const i8) -> i32;
}

#[inline(always)]
unsafe fn rd(addr: usize) -> u32 {
    // SAFETY: the caller passes an address inside the game's own structures.
    unsafe { (addr as *const u32).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00489840(param_1: i32, param_2: *mut u32, param_3: i32, param_4: u32) {
    let mut param_2 = param_2;
    if param_2.is_null() {
        return;
    }
    // SAFETY: all pointers are the game's own structures, accessed at the
    // offsets the original uses.
    unsafe {
        while param_2.add(1).read_unaligned() == 7 {
            let str2 = param_2.add(5).read_unaligned() as usize as *const i8;
            let mut found: *mut u32 = core::ptr::null_mut();
            let mut local_4: i32 = 0;
            let mut local_8: usize = 0;
            'search: {
                if 0 < rd((param_3 as usize).wrapping_add(6)) as i32 {
                    loop {
                        let arr = rd((param_3 as usize).wrapping_add(10)) as usize;
                        let mut p = rd(arr.wrapping_add(local_8)) as usize as *mut u32;
                        if p.is_null() {
                            p = core::ptr::null_mut();
                        } else {
                            let name = p.read_unaligned() as usize as *const i8;
                            if name.is_null() || __strcmpi(name, str2) != 0 {
                                let mut iv5: usize = 0;
                                let mut iv1: i32 = 0;
                                if 0 < p.add(2).read_unaligned() as i32 {
                                    loop {
                                        let child_arr = p.add(3).read_unaligned() as usize;
                                        let child =
                                            rd(child_arr.wrapping_add(iv5)) as usize as *mut u32;
                                        let r = FUN_0048e520(child, str2);
                                        if !r.is_null() {
                                            found = r;
                                            break 'search;
                                        }
                                        iv5 = iv5.wrapping_add(4);
                                        iv1 = iv1.wrapping_add(1);
                                        if !(iv1 < p.add(2).read_unaligned() as i32) {
                                            break;
                                        }
                                    }
                                }
                                p = core::ptr::null_mut();
                            }
                        }
                        if !p.is_null() {
                            found = p;
                            break 'search;
                        }
                        local_8 = local_8.wrapping_add(4);
                        local_4 = local_4.wrapping_add(1);
                        if !(local_4 < rd((param_3 as usize).wrapping_add(6)) as i32) {
                            break;
                        }
                    }
                }
                found = core::ptr::null_mut();
            }
            param_2 = found;
            if param_2.is_null() {
                return;
            }
        }
        if param_2.add(1).read_unaligned() == 3 {
            let mut pf3 = param_2.add(5).read_unaligned() as usize as *mut f32;
            let mut local_8: i32 = 0;
            if 0 < param_2.add(4).read_unaligned() as i32 {
                loop {
                    let mut pf4 = rd((param_1 as usize).wrapping_add(0x14)) as usize as *mut f32;
                    let mut iv5: i32 = 0;
                    let iv1 = rd((param_1 as usize).wrapping_add(0x10)) as i32;
                    let mut b6 = iv1 == 0;
                    if 0 < iv1 {
                        loop {
                            if pf3.read_unaligned() == pf4.read_unaligned()
                                && pf3.add(1).read_unaligned() == pf4.add(1).read_unaligned()
                                && pf3.add(2).read_unaligned() == pf4.add(2).read_unaligned()
                            {
                                break;
                            }
                            pf4 = pf4.add(3);
                            iv5 = iv5.wrapping_add(1);
                            if !(iv5 < iv1) {
                                break;
                            }
                        }
                        b6 = iv5 == iv1;
                    }
                    if b6 {
                        pf4.write_unaligned(pf3.read_unaligned());
                        pf4.add(1).write_unaligned(pf3.add(1).read_unaligned());
                        pf4.add(2).write_unaligned(pf3.add(2).read_unaligned());
                        let cp = ((param_1 as usize).wrapping_add(0x10)) as *mut i32;
                        cp.write_unaligned(cp.read_unaligned().wrapping_add(1));
                    }
                    pf3 = pf3.add(3);
                    local_8 = local_8.wrapping_add(1);
                    if !(local_8 < param_2.add(4).read_unaligned() as i32) {
                        break;
                    }
                }
            }
        }
        let mut iv1: usize = 0;
        let mut iv5: i32 = 0;
        if 0 < param_2.add(2).read_unaligned() as i32 {
            loop {
                iv1 = iv1.wrapping_add(4);
                iv5 = iv5.wrapping_add(1);
                let arr = param_2.add(3).read_unaligned() as usize;
                let child = rd(arr.wrapping_add(iv1).wrapping_sub(4)) as usize as *mut u32;
                FUN_00489840(param_1, child, param_3, param_4);
                if !(iv5 < param_2.add(2).read_unaligned() as i32) {
                    break;
                }
            }
        }
    }
}
