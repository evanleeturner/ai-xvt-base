// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00418240(param_1: *mut i32, param_2: *mut u32, param_3: i32) -> u32 {
    if !param_1.is_null() && !param_2.is_null() && param_3 != 0 {
        // Stack layout: [local_10, (unnamed), local_8, local_4]
        let mut buf: [u32; 4] = [0; 4];
        let p = buf.as_mut_ptr();
        // SAFETY: param_1 points at the caller's object; the callee addresses are
        // computed from it as the original does; the locals live in buf, and the
        // copies read and write where the original does.
        unsafe {
            let f1: unsafe extern "C" fn(*mut i32, i32, i32, *mut u32) -> i32 =
                core::mem::transmute(
                    (param_1.read().wrapping_add(0x2c) as usize) as *const core::ffi::c_void,
                );
            let size1 = f1(param_1, 0, param_3, p.add(3));
            if size1 >= 0 {
                let mut pu3: *mut u32 = param_2;
                let mut pu4: *mut u32 = p.add(2);
                let mut u_var2: u32 = (size1 as u32) >> 2;
                while u_var2 != 0 {
                    pu4.write_unaligned(pu3.read_unaligned());
                    pu3 = pu3.add(1);
                    pu4 = pu4.add(1);
                    u_var2 = u_var2.wrapping_sub(1);
                }
                let mut i: i32 = size1 & 3;
                while i != 0 {
                    (pu4 as *mut u8).write(*(pu3 as *const u8));
                    pu3 = (pu3 as *mut u8).add(1) as *mut u32;
                    pu4 = (pu4 as *mut u8).add(1) as *mut u32;
                    i = i.wrapping_sub(1);
                }
                if p.add(3).read() != 0 {
                    pu4 = p;
                    u_var2 = p.add(3).read() >> 2;
                    while u_var2 != 0 {
                        pu4.write_unaligned(pu3.read_unaligned());
                        pu3 = pu3.add(1);
                        pu4 = pu4.add(1);
                        u_var2 = u_var2.wrapping_sub(1);
                    }
                    i = (p.add(3).read() as i32) & 3;
                    while i != 0 {
                        (pu4 as *mut u8).write(*(pu3 as *const u8));
                        pu3 = (pu3 as *mut u8).add(1) as *mut u32;
                        pu4 = (pu4 as *mut u8).add(1) as *mut u32;
                        i = i.wrapping_sub(1);
                    }
                }
                let f2: unsafe extern "C" fn(*mut i32, i32, *mut u32, *mut u32, u32) =
                    core::mem::transmute(
                        (param_1.read().wrapping_add(0x4c) as usize) as *const core::ffi::c_void,
                    );
                f2(param_1, 0, p.add(2), p, p.add(3).read());
                return 1;
            }
        }
    }
    0
}
