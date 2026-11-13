// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006da850: [i16; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048f4f0(param_1: i32) -> i32;
    fn FUN_00485fa0(param_1: *mut i32);
    fn FUN_00449670(param_1: i32, param_2: i32) -> i32;
    fn FUN_0048f510();
}

#[inline(always)]
unsafe fn rd(addr: i32) -> i32 {
    // SAFETY: the caller passes an address of game memory, read where the original does.
    unsafe { (addr as u32 as usize as *const i32).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0044a1f0(param_1: i32, mut param_2: i32) -> i32 {
    if param_2 < 0 {
        return 0;
    }
    // SAFETY: all pointers and globals are the game's own memory, accessed as the original does.
    unsafe {
        let lo = (&raw mut DAT_006da850)
            .cast::<i16>()
            .offset(param_1 as isize)
            .read_unaligned();
        let hi = ((param_1 as u32) >> 16) as u16 as i16;
        let arg = (((hi as i32) as u32) << 16) | ((lo as i32) as u32);
        let p3 = FUN_0048f4f0(arg as i32);
        let piv = p3 as u32 as usize as *mut i32;

        if rd(p3) != p3 {
            FUN_00485fa0(piv);
        }

        let arr = rd(p3.wrapping_add(10));
        if rd(arr.wrapping_add(16)) == 0x14 {
            param_2 = param_2.wrapping_add(1);
        }

        let field_6 = rd(p3.wrapping_add(6));
        if field_6 <= param_2 {
            param_2 = field_6.wrapping_sub(1);
        }

        let i_var1 = rd(arr.wrapping_add(param_2.wrapping_mul(4)));

        let mut i_var4: i32 = 0;
        'done: {
            if i_var1 != 0 {
                if rd(i_var1.wrapping_add(4)) == 0x19 {
                    i_var4 = rd(i_var1.wrapping_add(0x14));
                    break 'done;
                }
                let mut i_var6: i32 = 0;
                let mut i_var5: i32 = 0;
                if 0 < rd(i_var1.wrapping_add(8)) {
                    loop {
                        let t = rd(rd(i_var1.wrapping_add(0xc)).wrapping_add(i_var6));
                        if t != 0 {
                            let r = FUN_00449670(t, p3);
                            if r != 0 {
                                i_var4 = r;
                                break 'done;
                            }
                        }
                        i_var6 = i_var6.wrapping_add(4);
                        i_var5 = i_var5.wrapping_add(1);
                        if !(i_var5 < rd(i_var1.wrapping_add(8))) {
                            break;
                        }
                    }
                }
            }
            i_var4 = 0;
        }

        let u_var2 = rd(i_var4.wrapping_add(0x38));
        FUN_0048f510();
        u_var2
    }
}
