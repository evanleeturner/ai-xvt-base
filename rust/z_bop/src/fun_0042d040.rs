// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a1051c: [i8; 0];
    static mut DAT_00a1065c: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0042dda0(param_1: *mut i8) -> i32;
    fn FUN_0042d600(param_1: *mut core::ffi::c_void) -> i32;
    fn FUN_0042dd20(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0042d040(param_1: *mut i8) -> i32 {
    // SAFETY: the globals and callees are the game's own; param_1 points at
    // the caller's string, and the table entries and the object's vtable are
    // read where the original reads them.
    unsafe {
        if param_1.read() == 0 {
            return 0;
        }
        let i_var3 = FUN_0042dda0(param_1);
        if i_var3 == -1 {
            return 0;
        }
        let off = i_var3.wrapping_mul(0x145) as isize;
        loop {
            let p = (&raw mut DAT_00a1051c)
                .cast::<i8>()
                .byte_offset(off)
                .cast::<core::ffi::c_void>();
            if FUN_0042d600(p) != 1 {
                break;
            }
        }
        let pi_var1 = (&raw mut DAT_00a1065c)
            .cast::<i8>()
            .byte_offset(off)
            .cast::<*mut i32>()
            .read_unaligned();
        let vtbl = pi_var1.read_unaligned();
        let slot = (vtbl as usize).wrapping_add(8) as *const *const core::ffi::c_void;
        let f: unsafe extern "stdcall" fn(*mut i32) = core::mem::transmute(slot.read_unaligned());
        f(pi_var1);
        FUN_0042dd20(i_var3);
        1
    }
}
