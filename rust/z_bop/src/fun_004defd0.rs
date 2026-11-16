// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7fb7: i32;
    static mut DAT_00aa7b0d: i8;
    static mut DAT_00aa7fc3: i32;
    static mut DAT_00aa6cfc: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004df1b0(param_1: *mut i8) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004d4970();
    fn FUN_004d48e0() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004defd0(param_1: *mut i8) -> i32 {
    // SAFETY: globals and callees are the game's own; param_1 points at a
    // NUL-terminated string, and the table and COM-like objects are read
    // where the original reads them.
    unsafe {
        if DAT_00aa7fb7 == 0 {
            return 0;
        }
        if param_1.read() == 0 {
            return 0;
        }
        let i_var3 = FUN_004df1b0(param_1);
        let c_var2 = DAT_00aa7b0d;
        if i_var3 == -1 {
            return 0;
        }
        let mut i_var6: i32 = 0;
        let mut i_var5: i32 = 0;
        FUN_004d4970();
        loop {
            let entry = DAT_00aa7fc3.wrapping_add(i_var6) as usize as *const u8;
            if entry.cast::<i32>().read_unaligned() == i_var3 {
                let pi_var1 =
                    entry.byte_add(8).cast::<i32>().read_unaligned() as usize as *const i32;
                let vtable = pi_var1.read_unaligned() as usize as *const u8;
                let f = vtable
                    .byte_add(0x24)
                    .cast::<*const core::ffi::c_void>()
                    .read_unaligned();
                let func: unsafe extern "stdcall" fn(*const i32, *mut u8) -> i32 =
                    core::mem::transmute(f);
                let mut local_4: [u8; 4] = [0; 4];
                let i_var4 = func(pi_var1, local_4.as_mut_ptr());
                if i_var4 == 0 && (local_4[0] & 5) != 0 {
                    i_var5 = i_var5.wrapping_add(1);
                }
            }
            i_var6 = i_var6.wrapping_add(0xc);
            if i_var6 >= 0x90 {
                break;
            }
        }
        if c_var2 != 0 {
            DAT_00aa6cfc = FUN_004d48e0();
        }
        i_var5
    }
}
