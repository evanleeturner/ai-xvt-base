// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006da850: [u16; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048f4f0(param_1: i32) -> *mut i32;
    fn FUN_00485fa0(param_1: *mut i32);
    fn FUN_00449190(param_1: i32) -> i32;
    fn FUN_0048f510();
}

unsafe fn rd(a: i32) -> i32 {
    // SAFETY: the address is a game-owned structure field, read as the original does.
    unsafe { (a as u32 as usize as *const i32).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0044a780(param_1: i32, mut param_2: i32) -> u32 {
    // SAFETY: all pointers and globals belong to the game; accesses mirror the original.
    unsafe {
        let idx = (*(&raw mut DAT_006da850)
            .cast::<u16>()
            .offset(param_1 as isize)) as i32;
        let pi2 = FUN_0048f4f0(idx);
        let pi2_addr = pi2 as usize as i32;
        if pi2.read() != pi2_addr {
            FUN_00485fa0(pi2);
        }
        let arr = rd(pi2_addr.wrapping_add(10));
        if rd(rd(arr).wrapping_add(4)) == 0x14 {
            param_2 = param_2.wrapping_add(1);
        }
        let field_6 = rd(pi2_addr.wrapping_add(6));
        if field_6 <= param_2 {
            param_2 = field_6.wrapping_sub(1);
        }
        let arr = rd(pi2_addr.wrapping_add(10));
        let i1 = rd(arr.wrapping_add(param_2.wrapping_mul(4)));
        let mut i3: i32 = 0;
        'l: {
            if i1 != 0 {
                i3 = i1;
                if rd(i1.wrapping_add(4)) == 0x17 {
                    break 'l;
                }
                let mut i5: i32 = 0;
                let mut i4: i32 = 0;
                if 0 < rd(i1.wrapping_add(8)) {
                    loop {
                        i3 = rd(rd(i1.wrapping_add(0xc)).wrapping_add(i5));
                        if i3 != 0 {
                            i3 = FUN_00449190(i3);
                            if i3 != 0 {
                                break 'l;
                            }
                        }
                        i5 = i5.wrapping_add(4);
                        i4 = i4.wrapping_add(1);
                        if !(i4 < rd(i1.wrapping_add(8))) {
                            break;
                        }
                    }
                }
            }
            i3 = 0;
        }
        let u6: u32 = if i3 == 0 {
            0
        } else {
            rd(i3.wrapping_add(0x14)) as u32
        };
        FUN_0048f510();
        u6
    }
}
