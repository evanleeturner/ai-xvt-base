// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00525cc8: i32;
    static mut DAT_0052a348: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048e480(param_1: i32, param_2: *mut i8) -> *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0044a830(param_1: *mut u32, param_2: i32, param_3: i32) -> *mut u32 {
    let mut param_1 = param_1;
    if param_1.is_null() {
        return core::ptr::null_mut();
    }
    // SAFETY: the globals and callees are the game's own; param_1 points at
    // the game's node structure, read and written where the original does.
    unsafe {
        loop {
            let f1 = param_1.byte_add(4).read_unaligned();
            if f1 != 7 {
                if f1 == 0x16 {
                    if param_3 == DAT_00525cc8 {
                        return param_1;
                    }
                    DAT_00525cc8 = DAT_00525cc8.wrapping_add(1);
                }
                let mut i_var3: i32 = 0;
                let mut i_var2: i32 = 0;
                if 0 < (param_1.byte_add(8).read_unaligned() as i32) {
                    loop {
                        let base = param_1.byte_add(12).read_unaligned();
                        let child = ((base as i32).wrapping_add(i_var3) as usize as *const u32)
                            .read_unaligned() as usize
                            as *mut u32;
                        let p_var1 = FUN_0044a830(child, param_2, param_3);
                        if !p_var1.is_null() {
                            return p_var1;
                        }
                        i_var3 = i_var3.wrapping_add(4);
                        i_var2 = i_var2.wrapping_add(1);
                        if !(i_var2 < (param_1.byte_add(8).read_unaligned() as i32)) {
                            break;
                        }
                    }
                }
                return core::ptr::null_mut();
            }
            let s = param_1.byte_add(20).read_unaligned() as usize as *mut i8;
            if DAT_0052a348 == 0 {
                param_1 = FUN_0048e480(param_2, s) as *mut u32;
            } else if s.read() == 0 {
                param_1 = param_1.read_unaligned() as usize as *mut u32;
            } else {
                let p_var1 = FUN_0048e480(param_2, s) as *mut u32;
                param_1.write_unaligned(p_var1 as usize as u32);
                let s2 = param_1.byte_add(20).read_unaligned() as usize as *mut i8;
                s2.write(0);
                param_1 = param_1.read_unaligned() as usize as *mut u32;
            }
            if param_1.is_null() {
                break;
            }
        }
        core::ptr::null_mut()
    }
}
