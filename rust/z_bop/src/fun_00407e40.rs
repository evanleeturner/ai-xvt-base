// SPDX-License-Identifier: MIT

unsafe extern "C" {
    fn FUN_004079f0(param_1: u32) -> *mut core::ffi::c_void;
    fn FUN_004b3280(param_1: i32);
    fn FUN_004b0b60(param_1: *mut u32, param_2: u32);
    fn FUN_004b2680(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32) -> i32;
}

unsafe extern "stdcall" {
    fn FUN_004079e0();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00407e40(
    param_1: u32,
    param_2: u32,
    param_3: *mut u32,
    param_4: u32,
) -> *mut core::ffi::c_void {
    let mut local_d8: [i32; 0x36] = [0; 0x36];
    // SAFETY: the callees are the game's own; puVar1 points at the object
    // returned by FUN_004079f0, read at offset 0x78 as the original does.
    unsafe {
        let p_var1 = FUN_004079f0(param_4);
        if p_var1.byte_add(0x78).cast::<i32>().read_unaligned() != 0 {
            FUN_004b3280(p_var1 as i32);
            return p_var1;
        }
        local_d8 = [0; 0x36];
        local_d8[23] = param_4 as i32;
        local_d8[0] = 0;
        local_d8[3] = param_1 as i32;
        local_d8[4] = param_2 as i32;
        local_d8[6] = param_1 as i32;
        local_d8[9] = 8;
        local_d8[8] = 0;
        FUN_004b0b60(param_3, 0x100);
        let i_var2 = FUN_004b2680(local_d8.as_mut_ptr(), p_var1 as i32, 0, 0);
        if i_var2 == 0 {
            FUN_004079e0();
            return core::ptr::null_mut();
        }
        p_var1
    }
}
