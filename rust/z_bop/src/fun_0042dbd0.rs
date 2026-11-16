// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a10514: i32;
    static mut DAT_00a604d4: i32;
    static mut DAT_00a60534: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0042dda0(param_1: *mut i8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042dbd0(param_1: *mut i8) -> i32 {
    // SAFETY: the globals and callee are the game's own; param_1 is the caller's
    // string, and the table entries hold object pointers whose slot at +0x24 is
    // a stdcall function, as in the original.
    unsafe {
        if DAT_00a10514 == 0 {
            return 0;
        }
        if param_1.read() == 0 {
            return 0;
        }
        let i_var1 = FUN_0042dda0(param_1);
        if i_var1 == -1 {
            return 0;
        }
        let mut i_var3: i32 = 0;
        let mut u_stack_4: u32 = 0;
        let mut p: *mut i32 = &raw mut DAT_00a604d4;
        let end: *mut i32 = &raw mut DAT_00a60534;
        loop {
            if p.byte_sub(8).read_unaligned() == i_var1 {
                let obj = p.read_unaligned();
                let slot = (obj as usize).wrapping_add(0x24) as *const *const core::ffi::c_void;
                let fp = slot.read_unaligned();
                let f: unsafe extern "stdcall" fn(*mut i32, *mut u32) -> i32 =
                    core::mem::transmute(fp);
                let i_var2 = f(obj as usize as *mut i32, &raw mut u_stack_4);
                if i_var2 == 0 && ((u_stack_4 & 1) != 0 || (u_stack_4 & 4) != 0) {
                    i_var3 = i_var3.wrapping_add(1);
                }
            }
            p = p.byte_add(12);
            if !(p < end) {
                break;
            }
        }
        i_var3
    }
}
