// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_005f09a8: f32;
    static mut _DAT_005f09ac: f32;
    static mut _DAT_005f09b0: f32;
    static mut _DAT_005f09b4: f32;
    static mut _DAT_005f09c8: f32;
    static mut _DAT_005f09cc: f32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048e480(param_1: i32, param_2: *mut i8) -> *mut u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004536c0(param_1: *mut u32, param_2: i32) {
    let mut param_1 = param_1;
    if param_1.is_null() {
        return;
    }
    // SAFETY: the pointers are the game's own node structures and globals;
    // fields are read at the offsets the original uses.
    unsafe {
        while param_1.add(1).read_unaligned() == 7 {
            param_1 = FUN_0048e480(param_2, param_1.add(5).read_unaligned() as *mut i8);
            if param_1.is_null() {
                return;
            }
        }
        if param_1.add(1).read_unaligned() == 3 {
            let mut i_var3 = param_1.add(4).read_unaligned() as i32;
            let mut pf_var2 = param_1.add(5).read_unaligned() as *const f32;
            if 0 < i_var3 {
                loop {
                    let x = pf_var2.read_unaligned();
                    let y = pf_var2.add(1).read_unaligned();
                    let z = pf_var2.add(2).read_unaligned();
                    if _DAT_005f09b0 < x {
                        _DAT_005f09b0 = x;
                    }
                    if x < _DAT_005f09b4 {
                        _DAT_005f09b4 = x;
                    }
                    if _DAT_005f09c8 < y {
                        _DAT_005f09c8 = y;
                    }
                    if y < _DAT_005f09cc {
                        _DAT_005f09cc = y;
                    }
                    if _DAT_005f09a8 < z {
                        _DAT_005f09a8 = z;
                    }
                    if z < _DAT_005f09ac {
                        _DAT_005f09ac = z;
                    }
                    pf_var2 = pf_var2.add(3);
                    i_var3 = i_var3.wrapping_sub(1);
                    if i_var3 == 0 {
                        break;
                    }
                }
            }
        }
        let mut i_var4: i32 = 0;
        let mut i_var3: i32 = 0;
        if 0 < param_1.add(2).read_unaligned() as i32 {
            loop {
                let p_var1 =
                    (param_1.add(3).read_unaligned() as i32).wrapping_add(i_var4) as *const u32;
                i_var4 = i_var4.wrapping_add(4);
                i_var3 = i_var3.wrapping_add(1);
                FUN_004536c0(p_var1.read_unaligned() as *mut u32, param_2);
                if i_var3 >= param_1.add(2).read_unaligned() as i32 {
                    break;
                }
            }
        }
    }
}
