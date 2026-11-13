// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FID_conflict___expand(ptr: *mut core::ffi::c_void, size: i32) -> *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00427740(param_1: i32) -> i32 {
    // SAFETY: param_1 points at the game's own structure; the fields and the
    // reallocated buffer are read and written where the original does.
    unsafe {
        let mut i_var1: i32 = (param_1.wrapping_add(0x1034) as *const i32)
            .read_unaligned()
            .wrapping_mul((param_1.wrapping_add(0x1030) as *const i32).read_unaligned());
        let field_108e: i32 = (param_1.wrapping_add(0x108e) as *const i32).read_unaligned();
        if field_108e == i_var1 {
            return 1;
        }
        let field_1086: *mut core::ffi::c_void =
            (param_1.wrapping_add(0x1086) as *const *mut core::ffi::c_void).read_unaligned();
        let pv_var2 = FID_conflict___expand(field_1086, i_var1.wrapping_mul(6));
        if !pv_var2.is_null() {
            (param_1.wrapping_add(0x1086) as *mut *mut core::ffi::c_void).write_unaligned(pv_var2);
            let base = pv_var2 as i32;
            let mut p6: i32 = base
                .wrapping_add(field_108e.wrapping_mul(6))
                .wrapping_sub(6);
            let prod = (param_1.wrapping_add(0x1034) as *const i32)
                .read_unaligned()
                .wrapping_mul((param_1.wrapping_add(0x1030) as *const i32).read_unaligned());
            let mut p5: i32 = base.wrapping_add(prod.wrapping_mul(6)).wrapping_sub(6);
            let mut u_var3: u32 = 0;
            if field_108e != 0 {
                loop {
                    i_var1 = i32::from((p6.wrapping_add(3) as *const u8).read_unaligned())
                        .wrapping_add(1);
                    let mut p4 = p5;
                    loop {
                        p5 = p4.wrapping_sub(6);
                        i_var1 = i_var1.wrapping_sub(1);
                        (p4 as *mut i32).write_unaligned((p6 as *const i32).read_unaligned());
                        (p4.wrapping_add(4) as *mut u16)
                            .write_unaligned((p6.wrapping_add(4) as *const u16).read_unaligned());
                        (p4.wrapping_add(3) as *mut u8).write_unaligned(0);
                        p4 = p5;
                        if i_var1 == 0 {
                            break;
                        }
                    }
                    p6 = p6.wrapping_sub(6);
                    u_var3 = u_var3.wrapping_add(1);
                    if u_var3 >= field_108e as u32 {
                        break;
                    }
                }
            }
            i_var1 = (param_1.wrapping_add(0x1034) as *const i32)
                .read_unaligned()
                .wrapping_mul((param_1.wrapping_add(0x1030) as *const i32).read_unaligned());
            (param_1.wrapping_add(0x108e) as *mut i32).write_unaligned(i_var1);
            return 1;
        }
        0
    }
}
