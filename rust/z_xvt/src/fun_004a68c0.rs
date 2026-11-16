// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052ae2c: i32;
    static mut DAT_006572cc: *mut i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048f5c0(param_1: *mut i8, param_2: i8, param_3: i16);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a68c0(param_1: *mut i8, param_2: i32, param_3: i32) {
    let mut acStack_400 = core::mem::MaybeUninit::<[i8; 1024]>::uninit();
    // SAFETY: the globals, the callee and the vtable are the game's own; the
    // pointer arithmetic mirrors the original, including its lack of bounds checks.
    unsafe {
        if DAT_0052ae2c == 0 {
            FUN_0048f5c0(param_1, param_2 as i8, param_3 as i16);
            return;
        }
        let obj = DAT_006572cc;
        if !obj.is_null() {
            if param_2 < param_2.wrapping_add(param_3) {
                let mut i_var1 = param_3;
                let mut pc2: *mut i8 = (acStack_400.as_mut_ptr() as *mut i8)
                    .wrapping_offset(param_2.wrapping_mul(4) as isize);
                let mut pc3: *mut i8 = param_1.wrapping_offset(param_2.wrapping_mul(3) as isize);
                loop {
                    pc2.write(pc3.read().wrapping_shl(2));
                    pc2.wrapping_add(1)
                        .write(pc3.wrapping_add(1).read().wrapping_shl(2));
                    i_var1 = i_var1.wrapping_sub(1);
                    pc2.wrapping_add(2)
                        .write(pc3.wrapping_add(2).read().wrapping_shl(2));
                    pc2 = pc2.wrapping_add(4);
                    pc3 = pc3.wrapping_add(3);
                    if i_var1 == 0 {
                        break;
                    }
                }
            }
            let vtable = obj.read_unaligned() as u32;
            let fp =
                ((vtable.wrapping_add(0x18)) as *const *const core::ffi::c_void).read_unaligned();
            let f: unsafe extern "stdcall" fn(*mut i32, i32, i32, i32, *mut i8) =
                core::mem::transmute(fp);
            f(
                obj,
                0,
                param_2,
                param_3,
                acStack_400.as_mut_ptr() as *mut i8,
            );
            return;
        }
        FUN_0048f5c0(param_1, param_2 as i8, param_3 as i16);
    }
}
