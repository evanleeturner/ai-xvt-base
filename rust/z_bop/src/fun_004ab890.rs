// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00527ea8: i32;
    static mut DAT_0066ddd8: *mut *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ac7e0(param_1: *mut u8, param_2: i8, param_3: i16);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004ab890(param_1: *mut u8, param_2: i32, param_3: i32) {
    let mut ac_stack_400 = [0u8; 1024];
    // SAFETY: the globals and callees are the game's own; param_1 points at the
    // caller's pixel bytes, and the buffer offsets are computed as the original does.
    unsafe {
        if DAT_00527ea8 == 0 {
            FUN_004ac7e0(param_1, param_2 as i8, param_3 as i16);
            return;
        }
        let this = DAT_0066ddd8;
        if !this.is_null() {
            if param_2 < param_2.wrapping_add(param_3) {
                let mut i_var1 = param_3;
                let mut pc_var2: *mut u8 = ac_stack_400
                    .as_mut_ptr()
                    .wrapping_offset(param_2.wrapping_mul(4) as isize);
                let mut pc_var3: *mut u8 =
                    param_1.wrapping_offset(param_2.wrapping_mul(3) as isize);
                loop {
                    pc_var2.write(pc_var3.read() << 2);
                    pc_var2
                        .wrapping_add(1)
                        .write(pc_var3.wrapping_add(1).read() << 2);
                    i_var1 = i_var1.wrapping_sub(1);
                    pc_var2
                        .wrapping_add(2)
                        .write(pc_var3.wrapping_add(2).read() << 2);
                    pc_var2 = pc_var2.wrapping_add(4);
                    pc_var3 = pc_var3.wrapping_add(3);
                    if i_var1 == 0 {
                        break;
                    }
                }
            }
            let vtable = (*this) as *const u8;
            let fp = vtable
                .byte_add(0x18)
                .cast::<*const core::ffi::c_void>()
                .read_unaligned();
            let f: unsafe extern "stdcall" fn(*mut core::ffi::c_void, i32, i32, i32, *mut u8) =
                core::mem::transmute(fp);
            f(
                this as *mut core::ffi::c_void,
                0,
                param_2,
                param_3,
                ac_stack_400.as_mut_ptr(),
            );
            return;
        }
        FUN_004ac7e0(param_1, param_2 as i8, param_3 as i16);
    }
}
