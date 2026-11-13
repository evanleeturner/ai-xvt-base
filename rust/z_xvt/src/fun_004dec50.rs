// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00759aec: i32;
    static mut DAT_00759645: i8;
    static mut DAT_00759af8: *mut core::ffi::c_void;
    static mut DAT_00758830: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dee40(param_1: *mut i8) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004d4d90();
    fn FUN_004d4d00() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004dec50(param_1: *mut i8) -> i32 {
    // SAFETY: globals and callees are the game's own; param_1 and the table
    // at DAT_00759af8 are read where the original reads them.
    unsafe {
        if DAT_00759aec == 0 {
            return 0;
        }
        if *param_1 == 0 {
            return 0;
        }
        let i_var3 = FUN_004dee40(param_1);
        let c_var2 = DAT_00759645;
        if i_var3 == -1 {
            return 0;
        }
        let mut i_var6: i32 = 0;
        let mut i_var5: i32 = 0;
        let mut u_stack_4: u32 = 0;
        FUN_004d4d90();
        loop {
            let entry = DAT_00759af8.byte_add(i_var6 as usize);
            if entry.cast::<i32>().read_unaligned() == i_var3 {
                let pi_var1 = entry.byte_add(8).cast::<*mut i32>().read_unaligned();
                let vtbl = pi_var1.read_unaligned() as u32;
                let f = (vtbl.wrapping_add(0x24) as *const u32).read_unaligned();
                let func: unsafe extern "stdcall" fn(*mut i32, *mut u32) -> i32 =
                    core::mem::transmute(f as usize as *const core::ffi::c_void);
                let i_var4 = func(pi_var1, &raw mut u_stack_4);
                if i_var4 == 0 && (u_stack_4 & 5) != 0 {
                    i_var5 = i_var5.wrapping_add(1);
                }
            }
            i_var6 = i_var6.wrapping_add(0xc);
            if i_var6 >= 0x90 {
                break;
            }
        }
        if c_var2 != 0 {
            DAT_00758830 = FUN_004d4d00();
        }
        i_var5
    }
}
