// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006da850: [i16; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048f4f0(param_1: i32) -> i32;
    fn FUN_00485fa0(param_1: *mut i32);
    fn FUN_00449670(param_1: i32, param_2: *mut i32) -> i32;
    fn __ftol() -> i32;
    fn FUN_0048f510();
}

#[inline(always)]
unsafe fn rd(a: i32) -> i32 {
    // SAFETY: the caller passes an address of the game's own memory.
    unsafe { (a as u32 as usize as *const i32).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0044a510(param_1: i32, mut param_2: i32) -> i32 {
    if param_2 < 0 {
        return 0;
    }
    // SAFETY: all pointers and globals are the game's own memory, read and
    // written where the original does.
    unsafe {
        let idx = (*(&raw mut DAT_006da850 as *const i16).offset(param_1 as isize)) as u16;
        let p = FUN_0048f4f0((((param_1 as u32) & 0xFFFF0000) | idx as u32) as i32);
        let pivar1 = p as u32 as usize as *mut i32;
        if rd(p) != p {
            FUN_00485fa0(pivar1);
        }
        let arr = rd(p.wrapping_add(10));
        if rd(rd(arr).wrapping_add(4)) == 0x14 {
            param_2 = param_2.wrapping_add(1);
        }
        let cnt = rd(p.wrapping_add(6));
        if cnt <= param_2 {
            param_2 = cnt.wrapping_sub(1);
        }
        let i_var3 = rd(arr.wrapping_add(param_2.wrapping_mul(4)));
        if i_var3 != 0 && rd(i_var3.wrapping_add(4)) != 0x19 {
            let mut i_var4: i32 = 0;
            if 0 < rd(i_var3.wrapping_add(8)) {
                let mut i_var5: i32 = 0;
                loop {
                    let i_var2 = rd(rd(i_var3.wrapping_add(0xc)).wrapping_add(i_var5));
                    if i_var2 != 0 && FUN_00449670(i_var2, pivar1) != 0 {
                        break;
                    }
                    i_var5 = i_var5.wrapping_add(4);
                    i_var4 = i_var4.wrapping_add(1);
                    if !(i_var4 < rd(i_var3.wrapping_add(8))) {
                        break;
                    }
                }
            }
        }
        let a = __ftol();
        let b = __ftol();
        let c = __ftol();
        let mut r = b;
        if b < a || b < c {
            r = a;
            if a <= c && b <= c {
                r = c;
            }
        }
        FUN_0048f510();
        r
    }
}
