// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005272b0: i32;
    static mut DAT_0060f204: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00479de0(param_1: i32, param_2: *mut i8) -> *mut u32;
}

unsafe fn add_unique(param_1: i32, pf1: *const f32) {
    // SAFETY: param_1 points at the game's vertex list structure; pf1 at the
    // caller's float triple; accessed exactly as the original does.
    unsafe {
        let cnt_p = param_1.wrapping_add(0x10) as u32 as usize as *mut i32;
        let mut pf2 =
            (param_1.wrapping_add(0x14) as u32 as usize as *const *mut f32).read_unaligned();
        let mut i4: i32 = 0;
        let i3: i32 = cnt_p.read_unaligned();
        let mut b5 = i3 == 0;
        if 0 < i3 {
            loop {
                if pf1.read_unaligned() == pf2.read_unaligned()
                    && pf2.add(1).read_unaligned() == pf1.add(1).read_unaligned()
                    && pf2.add(2).read_unaligned() == pf1.add(2).read_unaligned()
                {
                    break;
                }
                pf2 = pf2.add(3);
                i4 = i4.wrapping_add(1);
                if !(i4 < i3) {
                    break;
                }
            }
            b5 = i4 == i3;
        }
        if b5 {
            pf2.write_unaligned(pf1.read_unaligned());
            pf2.add(1).write_unaligned(pf1.add(1).read_unaligned());
            pf2.add(2).write_unaligned(pf1.add(2).read_unaligned());
            cnt_p.write_unaligned(cnt_p.read_unaligned().wrapping_add(1));
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00475940(param_1: i32, param_2: *mut u32, param_3: i32, param_4: i32) {
    let mut param_2 = param_2;
    if param_2.is_null() {
        return;
    }
    // SAFETY: all pointers and globals belong to the game; accessed as the original does.
    unsafe {
        while param_2.add(1).read_unaligned() == 7 {
            param_2 = FUN_00479de0(param_3, param_2.add(5).read_unaligned() as usize as *mut i8);
            if param_2.is_null() {
                return;
            }
        }
        let flag_p = param_4.wrapping_add(0x84) as u32 as usize as *mut i32;
        match param_2.add(1).read_unaligned() {
            1 | 0xf | 0x10 | 0x11 => {
                if flag_p.read_unaligned() == 0 {
                    let i3: u32 = if DAT_005272b0 == 0 {
                        param_2.add(4).read_unaligned().wrapping_mul(100)
                    } else {
                        param_2.add(4).read_unaligned().wrapping_mul(0x54)
                    };
                    let mut local_4: i32 = 0;
                    let mut pf1 = param_2
                        .add(5)
                        .read_unaligned()
                        .wrapping_add(i3)
                        .wrapping_add(4) as usize as *const f32;
                    if 0 < DAT_0060f204 {
                        loop {
                            add_unique(param_1, pf1);
                            pf1 = pf1.add(3);
                            local_4 = local_4.wrapping_add(1);
                            if !(local_4 < DAT_0060f204) {
                                break;
                            }
                        }
                    }
                }
            }
            3 => {
                DAT_0060f204 = param_2.add(4).read_unaligned() as i32;
            }
            0xb => {
                let mut local_4: i32 = 0;
                flag_p.write_unaligned(param_2.add(5).read_unaligned() as i32);
                let mut pf1 = param_2.add(5).read_unaligned() as usize as *const f32;
                if 0 < param_2.add(4).read_unaligned() as i32 {
                    loop {
                        add_unique(param_1, pf1);
                        pf1 = pf1.add(3);
                        local_4 = local_4.wrapping_add(1);
                        if !(local_4 < param_2.add(4).read_unaligned() as i32) {
                            break;
                        }
                    }
                }
            }
            _ => {}
        }
        if DAT_005272b0 != 0 {
            flag_p.write_unaligned(0);
        }
        let mut i4: u32 = 0;
        let mut i3: i32 = 0;
        if 0 < param_2.add(2).read_unaligned() as i32 {
            loop {
                i4 = i4.wrapping_add(4);
                i3 = i3.wrapping_add(1);
                let child = (param_2
                    .add(3)
                    .read_unaligned()
                    .wrapping_add(i4)
                    .wrapping_sub(4) as usize as *const *mut u32)
                    .read_unaligned();
                FUN_00475940(param_1, child, param_3, param_4);
                if !(i3 < param_2.add(2).read_unaligned() as i32) {
                    break;
                }
            }
        }
    }
}
