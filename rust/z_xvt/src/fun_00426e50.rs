// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00536ecc: *mut i32;
    static mut DAT_00536ed0: i32;
    static mut DAT_00524a18: [i8; 256];
    static mut DAT_00524e1c: u32;
    static mut DAT_00524e18: u32;
    static mut DAT_00524e20: u32;
    static mut unaff_EDI: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00426e50() -> i32 {
    let mut local_24: i32 = 0;
    let mut local_20: [i8; 32] = [0; 32];
    let mut p_stack_38: *mut i32;
    let mut u_stack_34: u32 = 0;
    // SAFETY: the globals and the object they point at are the game's own;
    // the callee addresses are computed exactly as the original does, and
    // the locals outlive every call that receives their addresses.
    unsafe {
        loop {
            loop {
                p_stack_38 = &raw mut local_24;
                u_stack_34 = 1;
                local_24 = 1;
                let vt = DAT_00536ecc.cast::<*mut i32>().read();
                let f: unsafe extern "stdcall" fn(*mut i32, i32, *mut i8) -> i32 =
                    core::mem::transmute(vt.wrapping_add(0xA) as *const core::ffi::c_void);
                let i_var2 = f(DAT_00536ecc, 0x10, local_20.as_mut_ptr());
                if i_var2 != -0x7ff8ffe2 {
                    if i_var2 < 0 {
                        return 0;
                    }
                    break;
                }
                if DAT_00536ecc.is_null() {
                    return 0;
                }
                let vt = DAT_00536ecc.cast::<*mut i32>().read();
                let g: unsafe extern "stdcall" fn(*mut i32) =
                    core::mem::transmute(vt.wrapping_add(0x7) as *const core::ffi::c_void);
                g(DAT_00536ecc);
                DAT_00536ed0 = 1;
            }
            if p_stack_38.is_null() {
                break;
            }
            let u_var3: u32 = unaff_EDI & 0x80;
            let c_var1: i8 = (&raw const DAT_00524a18)
                .cast::<i8>()
                .add((u_stack_34 & 0xff) as usize)
                .read();
            if c_var1 == -3 {
                p_stack_38 = 1usize as *mut i32;
                DAT_00524e1c = u_var3;
            } else if c_var1 == -2 {
                p_stack_38 = 1usize as *mut i32;
                DAT_00524e18 = u_var3;
            } else if c_var1 == -1 {
                p_stack_38 = 1usize as *mut i32;
                DAT_00524e20 = u_var3;
            } else {
                if u_var3 != 0 {
                    return 1;
                }
                p_stack_38 = 1usize as *mut i32;
            }
            let vt = DAT_00536ecc.cast::<*mut i32>().read();
            let h: unsafe extern "stdcall" fn(*mut i32, i32, *mut u32, *mut *mut i32, i32) =
                core::mem::transmute(vt.wrapping_add(0xA) as *const core::ffi::c_void);
            h(
                DAT_00536ecc,
                0x10,
                &raw mut u_stack_34,
                &raw mut p_stack_38,
                0,
            );
        }
        0
    }
}
