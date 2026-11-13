// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7b0d: i8;
    static mut DAT_00aa7fbf: *mut i8;
    static mut DAT_00aa6cfc: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004df1b0(param_1: *mut i8) -> i32;
    fn FUN_004de9a0(param_1: *mut i8) -> i32;
    fn FUN_004df130(param_1: i32);
    fn FUN_004d48e0() -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004d4970();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004de4f0(param_1: *mut i8) -> i32 {
    // SAFETY: the callees and globals are the game's own; param_1 points at
    // a NUL-terminated string, and the table at DAT_00aa7fbf holds entries
    // of 0x145 bytes whose object pointer at +0x140 has a vtable.
    unsafe {
        if *param_1 == 0 {
            return 0;
        }
        let i_var4 = FUN_004df1b0(param_1);
        let c_var2 = DAT_00aa7b0d;
        if i_var4 == -1 {
            return 0;
        }
        FUN_004d4970();
        loop {
            let b_var3 = FUN_004de9a0(DAT_00aa7fbf.byte_add((i_var4 as usize).wrapping_mul(0x145)));
            if b_var3 != 1 {
                break;
            }
        }
        let p_var1 = DAT_00aa7fbf
            .byte_add(0x140)
            .byte_add((i_var4 as usize).wrapping_mul(0x145))
            .cast::<*mut u32>()
            .read_unaligned();
        let vtable = p_var1.read_unaligned() as usize;
        let f = ((vtable + 8) as *const u32).read_unaligned() as usize;
        let func: unsafe extern "stdcall" fn(*mut u32) =
            core::mem::transmute(f as *const core::ffi::c_void);
        func(p_var1);
        FUN_004df130(i_var4);
        if c_var2 != 0 {
            DAT_00aa6cfc = FUN_004d48e0();
        }
        1
    }
}
