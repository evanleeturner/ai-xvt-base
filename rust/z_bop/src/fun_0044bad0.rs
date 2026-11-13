// SPDX-License-Identifier: MIT

#[repr(C)]
struct Locals {
    local_10: i32,
    local_c: i32,
    local_8: i32,
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0044bad0(
    param_1: *mut i32,
    param_2: *mut core::ffi::c_void,
    param_3: i32,
) -> i32 {
    if !param_1.is_null() && !param_2.is_null() && param_3 != 0 {
        // SAFETY: param_1 is a game object whose first dword is a vtable
        // pointer; slots 0x2c and 0x4c are called as the original does; the
        // copies read and write raw memory exactly as the original does.
        unsafe {
            let mut l = Locals {
                local_10: 0,
                local_c: 0,
                local_8: 0,
            };
            let l_ptr: *mut Locals = &raw mut l;
            let p_c: *mut i32 = &raw mut (*l_ptr).local_c;
            let p_8: *mut i32 = &raw mut (*l_ptr).local_8;
            let p_10: *mut i32 = &raw mut (*l_ptr).local_10;

            let vtbl = param_1.read_unaligned() as u32 as usize as *const u8;
            let f1: unsafe extern "C" fn(*mut i32, i32, i32, *mut i32) -> i32 =
                core::mem::transmute(
                    vtbl.add(0x2c)
                        .cast::<*const core::ffi::c_void>()
                        .read_unaligned(),
                );
            let i_var1 = f1(param_1, 0, param_3, p_c);
            if i_var1 >= 0 {
                let mut src = param_2 as *const u8;
                let mut dst = p_8 as *mut u8;
                let mut u_var2 = (param_3 as u32) >> 2;
                while u_var2 != 0 {
                    dst.cast::<i32>()
                        .write_unaligned(src.cast::<i32>().read_unaligned());
                    src = src.add(4);
                    dst = dst.add(4);
                    u_var2 -= 1;
                }
                let mut rem = param_3 & 3;
                while rem != 0 {
                    dst.write(src.read());
                    src = src.add(1);
                    dst = dst.add(1);
                    rem -= 1;
                }

                let lc = p_c.read();
                if lc != 0 {
                    let mut src = (param_2 as *const u8).offset(param_3 as isize);
                    let mut dst = p_10 as *mut u8;
                    let mut u_var2 = (lc as u32) >> 2;
                    while u_var2 != 0 {
                        dst.cast::<i32>()
                            .write_unaligned(src.cast::<i32>().read_unaligned());
                        src = src.add(4);
                        dst = dst.add(4);
                        u_var2 -= 1;
                    }
                    let mut rem = lc & 3;
                    while rem != 0 {
                        dst.write(src.read());
                        src = src.add(1);
                        dst = dst.add(1);
                        rem -= 1;
                    }
                }

                let vtbl = param_1.read_unaligned() as u32 as usize as *const u8;
                let f2: unsafe extern "C" fn(*mut i32, i32, *mut i32, *mut i32, i32) =
                    core::mem::transmute(
                        vtbl.add(0x4c)
                            .cast::<*const core::ffi::c_void>()
                            .read_unaligned(),
                    );
                f2(param_1, 0, p_8, p_10, p_c.read());
                return 1;
            }
        }
    }
    0
}
