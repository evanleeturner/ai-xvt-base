// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __strcmpi(s1: *const i8, s2: *const i8) -> i32;
    fn FUN_0048e520(param_1: *mut c_void, param_2: *mut i8) -> *mut c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0048e480(param_1: i32, param_2: *mut i8) -> *mut c_void {
    // SAFETY: param_1 is the address of the game's own structure; every
    // pointer read below is read where the original reads it, and the
    // callees are the game's own.
    unsafe {
        let mut local_4: i32 = 0;
        let count: i32 = ((param_1 as u32).wrapping_add(6) as *const i32).read_unaligned();

        if count > 0 {
            let mut local_8: i32 = 0;
            loop {
                let base_int: i32 =
                    ((param_1 as u32).wrapping_add(10) as *const i32).read_unaligned();
                let base = base_int as u32 as *const u8;
                let p3: *mut *mut i8 = base
                    .byte_add(local_8 as u32 as usize)
                    .cast::<*mut *mut i8>()
                    .read_unaligned();

                if !p3.is_null() {
                    let name: *mut i8 = p3.read_unaligned();
                    if name.is_null() || __strcmpi(name, param_2) != 0 {
                        let sub_count: i32 =
                            p3.cast::<u8>().byte_add(8).cast::<i32>().read_unaligned();
                        if sub_count > 0 {
                            let mut i_var1: i32 = 0;
                            let mut i_var4: i32 = 0;
                            loop {
                                let sub_base_int: i32 =
                                    p3.cast::<u8>().byte_add(12).cast::<i32>().read_unaligned();
                                let sub_base = sub_base_int as u32 as *const u8;
                                let p2: *mut c_void = sub_base
                                    .byte_add(i_var4 as u32 as usize)
                                    .cast::<*mut c_void>()
                                    .read_unaligned();
                                let r = FUN_0048e520(p2, param_2);
                                if !r.is_null() {
                                    return r;
                                }
                                i_var4 = i_var4.wrapping_add(4);
                                i_var1 = i_var1.wrapping_add(1);
                                if !(i_var1 < sub_count) {
                                    break;
                                }
                            }
                        }
                    } else {
                        return p3.cast::<c_void>();
                    }
                }

                local_8 = local_8.wrapping_add(4);
                local_4 = local_4.wrapping_add(1);
                if !(local_4 < count) {
                    break;
                }
            }
        }
        core::ptr::null_mut()
    }
}
