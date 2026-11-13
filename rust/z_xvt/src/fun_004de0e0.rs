// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00759645: i8;
    static mut DAT_00759af4: *mut i8;
    static mut DAT_00758830: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dee40(param_1: *mut i8) -> i32;
    fn FUN_004de5d0(param_1: *mut i8) -> i32;
    fn FUN_004dedc0(param_1: i32);
    fn FUN_004d4d00() -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004d4d90();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004de0e0(param_1: *mut i8) -> i32 {
    // SAFETY: the globals and callees are the game's own; param_1 points at
    // a caller-provided string, and the table at DAT_00759af4 holds entries
    // of 0x145 bytes whose last field is a COM-like object pointer.
    unsafe {
        if *param_1 == 0 {
            return 0;
        }
        let i_var4 = FUN_004dee40(param_1);
        let c_var2 = DAT_00759645;
        if i_var4 == -1 {
            return 0;
        }
        FUN_004d4d90();
        loop {
            let b_var3 = FUN_004de5d0(DAT_00759af4.byte_add(i_var4.wrapping_mul(0x145) as usize));
            if b_var3 != 1 {
                break;
            }
        }
        let p_var1 = DAT_00759af4
            .byte_add(0x140usize.wrapping_add(i_var4.wrapping_mul(0x145) as usize))
            .cast::<*mut i32>()
            .read_unaligned();
        let vtable = p_var1.read_unaligned() as usize as *const u8;
        let f = vtable
            .byte_add(8)
            .cast::<*const core::ffi::c_void>()
            .read_unaligned();
        let f: unsafe extern "stdcall" fn(*mut i32) = core::mem::transmute(f);
        f(p_var1);
        FUN_004dedc0(i_var4);
        if c_var2 != 0 {
            DAT_00758830 = FUN_004d4d00();
        }
        1
    }
}
