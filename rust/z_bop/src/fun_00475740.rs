// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00479de0(param_1: i32, param_2: *mut i8) -> *mut u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00475740(param_1: i32, param_2: *mut u32, param_3: i32, param_4: u32) {
    let mut param_2 = param_2;
    if param_2.is_null() {
        return;
    }
    // SAFETY: all pointers are the game's own structures, read and written
    // at the offsets the original uses.
    unsafe {
        while param_2.add(1).read_unaligned() == 7 {
            let p = param_2.add(5).read_unaligned();
            param_2 = FUN_00479de0(param_3, p as usize as *mut i8);
            if param_2.is_null() {
                return;
            }
        }
        if param_2.add(1).read_unaligned() == 3 {
            let mut pf1 = param_2.add(5).read_unaligned() as usize as *mut f32;
            let mut local_4: i32 = 0;
            if 0 < (param_2.add(4).read_unaligned() as i32) {
                loop {
                    let mut pf2 = ((param_1 as u32).wrapping_add(0x14) as usize as *const u32)
                        .read_unaligned() as usize as *mut f32;
                    let mut i3: i32 = 0;
                    let i4: i32 = ((param_1 as u32).wrapping_add(0x10) as usize as *const i32)
                        .read_unaligned();
                    let mut b5 = i4 == 0;
                    if 0 < i4 {
                        loop {
                            if pf1.read_unaligned() == pf2.read_unaligned()
                                && pf1.add(1).read_unaligned() == pf2.add(1).read_unaligned()
                                && pf1.add(2).read_unaligned() == pf2.add(2).read_unaligned()
                            {
                                break;
                            }
                            pf2 = pf2.wrapping_add(3);
                            i3 = i3.wrapping_add(1);
                            if !(i3 < i4) {
                                break;
                            }
                        }
                        b5 = i3 == i4;
                    }
                    if b5 {
                        pf2.write_unaligned(pf1.read_unaligned());
                        pf2.add(1).write_unaligned(pf1.add(1).read_unaligned());
                        pf2.add(2).write_unaligned(pf1.add(2).read_unaligned());
                        let cnt = (param_1 as u32).wrapping_add(0x10) as usize as *mut i32;
                        cnt.write_unaligned(cnt.read_unaligned().wrapping_add(1));
                    }
                    pf1 = pf1.wrapping_add(3);
                    local_4 = local_4.wrapping_add(1);
                    if !(local_4 < (param_2.add(4).read_unaligned() as i32)) {
                        break;
                    }
                }
            }
        }
        let mut i4: i32 = 0;
        if 0 < (param_2.add(2).read_unaligned() as i32) {
            let mut i3: u32 = 0;
            loop {
                i3 = i3.wrapping_add(4);
                i4 = i4.wrapping_add(1);
                let base = param_2.add(3).read_unaligned();
                let addr = base.wrapping_sub(4).wrapping_add(i3);
                let child = (addr as usize as *const u32).read_unaligned();
                FUN_00475740(param_1, child as usize as *mut u32, param_3, param_4);
                if !(i4 < (param_2.add(2).read_unaligned() as i32)) {
                    break;
                }
            }
        }
    }
}
