// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006da850: [i16; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048f4f0(param_1: u32) -> u32;
    fn FUN_00485fa0(param_1: *mut i32);
    fn FUN_00449670(param_1: i32, param_2: u32) -> i32;
    fn FUN_0048f510() -> u32;
}

unsafe fn rd(a: i32) -> i32 {
    // SAFETY: the address is a game-owned structure address, read as the original does.
    unsafe { (a as u32 as usize as *const i32).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00449790(param_1: i32, mut param_2: i32) -> u32 {
    if param_2 < 0 {
        return 0;
    }
    // SAFETY: the callees, the global table and all pointers are the game's own
    // memory, accessed in the same order as the original.
    unsafe {
        let tbl = (&raw mut DAT_006da850) as *mut i16;
        let v = tbl.offset(param_1 as isize).read_unaligned();
        let arg = ((param_1 >> 16) as u32).wrapping_shl(16) | (v as u16 as u32);
        let r = FUN_0048f4f0(arg);
        let pivar3 = r as usize as *mut i32;
        let p3 = r as i32;
        if rd(p3) != p3 {
            FUN_00485fa0(pivar3);
        }
        if rd(rd(rd(p3.wrapping_add(10))).wrapping_add(4)) == 0x14 {
            param_2 = param_2.wrapping_add(1);
        }
        let cnt = rd(p3.wrapping_add(6));
        if cnt <= param_2 {
            param_2 = cnt.wrapping_sub(1);
        }
        let arr = rd(p3.wrapping_add(10));
        let ivar1 = rd(arr.wrapping_add(param_2.wrapping_mul(4)));
        let mut p4: i32 = 0;
        if ivar1 != 0 {
            if rd(ivar1.wrapping_add(4)) == 0x19 {
                p4 = rd(ivar1.wrapping_add(0x14));
            } else {
                let mut ivar6: i32 = 0;
                let mut ivar5: i32 = 0;
                if 0 < rd(ivar1.wrapping_add(8)) {
                    loop {
                        let ivar2 = rd(rd(ivar1.wrapping_add(0xc)).wrapping_add(ivar6));
                        if ivar2 != 0 {
                            let t = FUN_00449670(ivar2, r);
                            if t != 0 {
                                p4 = t;
                                break;
                            }
                        }
                        ivar6 = ivar6.wrapping_add(4);
                        ivar5 = ivar5.wrapping_add(1);
                        if !(ivar5 < rd(ivar1.wrapping_add(8))) {
                            break;
                        }
                    }
                }
            }
        }
        let u7: u32 = if p4 == 0 { 0 } else { rd(p4) as u32 };
        FUN_0048f510();
        u7
    }
}
