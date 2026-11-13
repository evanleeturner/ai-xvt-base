// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00528128: i32;
    static mut DAT_005270b0: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00479de0(param_1: i32, param_2: *mut i8) -> *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004af090(
    param_1: *mut core::ffi::c_void,
    param_2: i32,
    param_3: i32,
) -> *mut core::ffi::c_void {
    let mut param_1 = param_1;
    if param_1.is_null() {
        return core::ptr::null_mut();
    }
    // SAFETY: param_1 points at the game's node structure; the globals and
    // callee are the game's own, accessed in the original's order.
    unsafe {
        loop {
            let p = param_1 as *mut u32;
            let kind = p.add(1).read_unaligned();
            if kind != 7 {
                if kind == 0x16 {
                    if param_3 == DAT_00528128 {
                        return param_1;
                    }
                    DAT_00528128 = DAT_00528128.wrapping_add(1);
                }
                let mut i_var3: u32 = 0;
                let mut i_var2: i32 = 0;
                if 0 < (p.add(2).read_unaligned() as i32) {
                    loop {
                        let base = p.add(3).read_unaligned();
                        let child = (base.wrapping_add(i_var3) as *const *mut core::ffi::c_void)
                            .read_unaligned();
                        let r = FUN_004af090(child, param_2, param_3);
                        if !r.is_null() {
                            return r;
                        }
                        i_var3 = i_var3.wrapping_add(4);
                        i_var2 = i_var2.wrapping_add(1);
                        if !(i_var2 < (p.add(2).read_unaligned() as i32)) {
                            break;
                        }
                    }
                }
                return core::ptr::null_mut();
            }
            let s = p.add(5).read_unaligned() as *mut i8;
            if DAT_005270b0 == 0 {
                param_1 = FUN_00479de0(param_2, s);
            } else if s.read() == 0 {
                param_1 = (param_1 as *const *mut core::ffi::c_void).read_unaligned();
            } else {
                let r = FUN_00479de0(param_2, s);
                (param_1 as *mut *mut core::ffi::c_void).write_unaligned(r);
                let s2 = p.add(5).read_unaligned() as *mut i8;
                s2.write(0);
                param_1 = (param_1 as *const *mut core::ffi::c_void).read_unaligned();
            }
            if param_1.is_null() {
                break;
            }
        }
    }
    core::ptr::null_mut()
}
