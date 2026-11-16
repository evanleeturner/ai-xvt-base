// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006026bc: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __strcmpi(_Str1: *const i8, _Str2: *const i8) -> i32;
    fn FUN_0048e520(param_1: *mut u32, param_2: *mut i8) -> *mut u32;
}

#[inline(always)]
unsafe fn rd(addr: i32) -> i32 {
    // SAFETY: the address is the game's own memory, read as the original does.
    unsafe { (addr as u32 as usize as *const i32).read_unaligned() }
}

#[inline(always)]
unsafe fn wr(addr: i32, v: i32) {
    // SAFETY: the address is the game's own memory, written as the original does.
    unsafe { (addr as u32 as usize as *mut i32).write_unaligned(v) }
}

unsafe fn add_unique(param_1: i32, pf3: *const f32) {
    // SAFETY: param_1 points at the game's vertex list; pf3 at the source floats.
    unsafe {
        let mut pf4 = rd(param_1.wrapping_add(0x14)) as u32 as usize as *mut f32;
        let mut i5: i32 = 0;
        let i1 = rd(param_1.wrapping_add(0x10));
        let mut b6 = i1 == 0;
        if 0 < i1 {
            loop {
                if pf3.read_unaligned() == pf4.read_unaligned()
                    && pf4.add(1).read_unaligned() == pf3.add(1).read_unaligned()
                    && pf4.add(2).read_unaligned() == pf3.add(2).read_unaligned()
                {
                    break;
                }
                pf4 = pf4.add(3);
                i5 = i5.wrapping_add(1);
                if !(i5 < i1) {
                    break;
                }
            }
            b6 = i5 == i1;
        }
        if b6 {
            pf4.write_unaligned(pf3.read_unaligned());
            pf4.add(1).write_unaligned(pf3.add(1).read_unaligned());
            pf4.add(2).write_unaligned(pf3.add(2).read_unaligned());
            wr(
                param_1.wrapping_add(0x10),
                rd(param_1.wrapping_add(0x10)).wrapping_add(1),
            );
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00489b40(param_1: i32, param_2: *mut u32, param_3: i32, param_4: i32) {
    let mut param_2 = param_2;
    if param_2.is_null() {
        return;
    }
    // SAFETY: all pointers are the game's own structures, accessed as the original does.
    unsafe {
        while param_2.add(1).read_unaligned() == 7 {
            let str2 = param_2.add(5).read_unaligned() as usize as *mut i8;
            let mut local_4: i32 = 0;
            let mut local_8: i32;
            let mut tmp_count = rd(param_3.wrapping_add(6));
            'search: {
                if 0 < tmp_count {
                    local_8 = 0;
                    loop {
                        let tmp_base = rd(param_3.wrapping_add(10));
                        let p2 = rd(tmp_base.wrapping_add(local_8)) as u32 as usize as *mut u32;
                        if p2.is_null() {
                            param_2 = core::ptr::null_mut();
                        } else {
                            let s = p2.read_unaligned() as usize as *const i8;
                            if s.is_null() || __strcmpi(s, str2) != 0 {
                                let mut i1: i32 = 0;
                                let mut i5: i32 = 0;
                                if 0 < (p2.add(2).read_unaligned() as i32) {
                                    loop {
                                        let base = p2.add(3).read_unaligned() as i32;
                                        let child =
                                            rd(base.wrapping_add(i1)) as u32 as usize as *mut u32;
                                        param_2 = FUN_0048e520(child, str2);
                                        if !param_2.is_null() {
                                            break 'search;
                                        }
                                        i1 = i1.wrapping_add(4);
                                        i5 = i5.wrapping_add(1);
                                        if !(i5 < (p2.add(2).read_unaligned() as i32)) {
                                            break;
                                        }
                                    }
                                }
                                param_2 = core::ptr::null_mut();
                            } else {
                                param_2 = p2;
                                break 'search;
                            }
                        }
                        local_8 = local_8.wrapping_add(4);
                        local_4 = local_4.wrapping_add(1);
                        tmp_count = rd(param_3.wrapping_add(6));
                        if !(local_4 < tmp_count) {
                            break;
                        }
                    }
                }
                param_2 = core::ptr::null_mut();
            }
            if param_2.is_null() {
                return;
            }
        }

        match param_2.add(1).read_unaligned() {
            1 | 0xf | 0x10 | 0x11 => {
                if rd(param_4.wrapping_add(0x84)) == 0 {
                    let mut local_8: i32 = 0;
                    let mut pf3 = (param_2
                        .add(5)
                        .read_unaligned()
                        .wrapping_add(4)
                        .wrapping_add(param_2.add(4).read_unaligned().wrapping_mul(0x54)))
                        as usize as *const f32;
                    if 0 < DAT_006026bc {
                        loop {
                            add_unique(param_1, pf3);
                            pf3 = pf3.add(3);
                            local_8 = local_8.wrapping_add(1);
                            if !(local_8 < DAT_006026bc) {
                                break;
                            }
                        }
                    }
                }
            }
            3 => {
                DAT_006026bc = param_2.add(4).read_unaligned() as i32;
            }
            0xb => {
                let mut local_8: i32 = 0;
                wr(
                    param_4.wrapping_add(0x84),
                    param_2.add(5).read_unaligned() as i32,
                );
                let mut pf3 = param_2.add(5).read_unaligned() as usize as *const f32;
                if 0 < (param_2.add(4).read_unaligned() as i32) {
                    loop {
                        add_unique(param_1, pf3);
                        pf3 = pf3.add(3);
                        local_8 = local_8.wrapping_add(1);
                        if !(local_8 < (param_2.add(4).read_unaligned() as i32)) {
                            break;
                        }
                    }
                }
            }
            _ => {}
        }

        let mut i5: i32 = 0;
        let mut i1: i32 = 0;
        wr(param_4.wrapping_add(0x84), 0);
        if 0 < (param_2.add(2).read_unaligned() as i32) {
            loop {
                i5 = i5.wrapping_add(4);
                i1 = i1.wrapping_add(1);
                let base = param_2.add(3).read_unaligned() as i32;
                let child = rd(base.wrapping_sub(4).wrapping_add(i5)) as u32 as usize as *mut u32;
                FUN_00489b40(param_1, child, param_3, param_4);
                if !(i1 < (param_2.add(2).read_unaligned() as i32)) {
                    break;
                }
            }
        }
    }
}
