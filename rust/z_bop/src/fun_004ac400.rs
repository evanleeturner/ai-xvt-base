// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004ac460() -> i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004aafe0();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004ac400(param_1: *mut i32) {
    let mut local_64: [i32; 20] = [0; 20];
    local_64[0] = 100;
    loop {
        // SAFETY: param_1 is the caller's object. The C adds 0x14 to an
        // `int *` (0x50 bytes) and calls that address without loading from
        // it; that is kept as is. The callees are the game's own.
        let i_var2: i32 = unsafe {
            let vt = param_1.cast::<*const i32>().read_unaligned();
            let target = vt.wrapping_add(0x14) as *const core::ffi::c_void;
            let f: unsafe extern "stdcall" fn(*mut i32, i32, i32, i32, i32, *mut i32) -> i32 =
                core::mem::transmute(target);
            f(param_1, 0, 0, 0, 0x400, local_64.as_mut_ptr())
        };
        if i_var2 == 0 {
            return;
        }
        if i_var2 == 0x887601C2u32 as i32 {
            // SAFETY: the callees are the game's own.
            let b_var1 = unsafe { FUN_004ac460() };
            if b_var1 == 0 {
                return;
            }
            // SAFETY: the callee is the game's own.
            unsafe { FUN_004aafe0() };
        }
        if i_var2 != 0x8876021Cu32 as i32 {
            break;
        }
    }
}
