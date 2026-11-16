// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00759afc: i32;
    static mut DAT_00759aec: *mut i32;
    static mut DAT_00759645: i8;
    static mut DAT_00758830: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dee40(param_1: *mut i8) -> i32;
    fn FUN_00417a40(param_1: *mut i32, param_2: *mut i8, param_3: i32) -> *mut i32;
    fn strncpy(dest: *mut i8, src: *const i8, n: u32) -> *mut i8;
    fn FUN_004ded00(param_1: *mut i8) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004d4d90();
    fn FUN_004d4d00() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004ddf70(param_1: *mut i8, param_2: *mut i8, param_3: i32) -> u8 {
    let mut ac_stack_110: [i8; 191] = [0; 191];
    let mut buf_40: [i8; 0x40] = [0; 0x40];
    // SAFETY: the globals and callees are the game's own; param_1 and
    // param_2 point at the caller's NUL-terminated strings; the vtable call
    // and the buffer copies mirror the original, including its overflow.
    unsafe {
        if *param_1 == 0 {
            return 0;
        }
        if *param_2 == 0 {
            return 0;
        }
        if 0x7f < DAT_00759afc {
            return 0;
        }
        if DAT_00759aec.is_null() {
            return 0;
        }
        let i_var2 = FUN_004dee40(param_2);
        let c_var1 = DAT_00759645;
        if i_var2 != -1 {
            return 1;
        }
        FUN_004d4d90();
        let local_8 = FUN_00417a40(DAT_00759aec, param_1, param_3);
        if !local_8.is_null() {
            let vtable = local_8.read_unaligned() as u32 as usize;
            let f = ((vtable + 0x34) as *const *const core::ffi::c_void).read_unaligned();
            let f: unsafe extern "stdcall" fn(*mut i32, i32) = core::mem::transmute(f);
            f(local_8, 0);
            strncpy(buf_40.as_mut_ptr(), param_2, 0x40);
            strncpy(ac_stack_110.as_mut_ptr(), param_1, 0x100);
            let i_stack_10 = FUN_004ded00(buf_40.as_mut_ptr());
            if c_var1 != 0 {
                DAT_00758830 = FUN_004d4d00();
            }
            return 1u8.wrapping_sub((i_stack_10 == 0) as u8);
        }
        if c_var1 != 0 {
            DAT_00758830 = FUN_004d4d00();
        }
        0
    }
}
