// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00759663: i32;
    static mut DAT_00759667: *mut i32;
    static mut DAT_0075964b: i32;
    static mut DAT_00759653: i32;
    static mut DAT_00758830: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d52b0() -> i32 {
    let mut local_6c: [i32; 27] = [0; 27];
    let u_stack_48: i32 = 0;

    // SAFETY: the globals are the game's own memory; DAT_00759667 is checked
    // non-null before it is read through, and the called addresses are
    // computed exactly as the original does.
    unsafe {
        if DAT_00759663 == 0 {
            return 0;
        }
        if DAT_00759667.is_null() {
            return 0;
        }
        local_6c[0] = 0x6c;
        loop {
            let target = (DAT_00759667.read_unaligned().wrapping_add(0x64)) as u32 as usize
                as *const core::ffi::c_void;
            let f: unsafe extern "stdcall" fn(*mut i32, i32, *mut i32, i32, i32) -> i32 =
                core::mem::transmute(target);
            let i_var1 = f(DAT_00759667, 0, local_6c.as_mut_ptr(), 0, 0);
            if i_var1 == 0 {
                DAT_0075964b = DAT_00759653;
                DAT_00758830 = u_stack_48;
                return 1;
            }
            if i_var1 == -0x7789fe3e {
                let target2 = (DAT_00759667.read_unaligned().wrapping_add(0x6c)) as u32 as usize
                    as *const core::ffi::c_void;
                let g: unsafe extern "stdcall" fn(*mut i32) = core::mem::transmute(target2);
                g(DAT_00759667);
                continue;
            }
            if i_var1 == -0x7789fde4 {
                continue;
            }
            return 0;
        }
    }
}
