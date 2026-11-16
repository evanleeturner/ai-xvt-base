// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00523570: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004be160(param_1: *mut i8, param_2: *mut i8) -> i32;
    fn FUN_004be320(param_1: i32, param_2: *mut core::ffi::c_void, param_3: i32);
    fn FUN_004df700(param_1: i32, param_2: *mut u32, param_3: i32) -> u8;
    fn FUN_004be1f0(param_1: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004df660(param_1: *mut i8, param_2: *mut u32) -> u32 {
    if param_2.is_null() {
        return 0;
    }
    let mut u_var2: u32 = 0;
    let mut local_38: [i16; 8] = [0; 8];
    // local_1c and local_1a overlap local_28 at offsets 0xc and 0xe.
    let mut local_28: [u8; 0x28] = [0; 0x28];
    // SAFETY: the callees and the global are the game's own; the buffers are
    // local and large enough for the sizes read into them; param_2 is passed
    // through as the caller gave it.
    unsafe {
        let p_var1 = FUN_004be160(param_1, &raw mut DAT_00523570 as *mut i8);
        if p_var1 == 0 {
            return 0;
        }
        FUN_004be320(p_var1, local_38.as_mut_ptr().cast(), 0xe);
        if local_38[0] == 0x4d42 {
            FUN_004be320(p_var1, local_28.as_mut_ptr().cast(), 0x28);
            let local_1c = local_28
                .as_ptr()
                .byte_add(0xc)
                .cast::<i16>()
                .read_unaligned();
            if local_1c == 1 {
                let local_1a = local_28
                    .as_ptr()
                    .byte_add(0xe)
                    .cast::<i16>()
                    .read_unaligned();
                let mut run = false;
                let mut i_var3: i32 = 0;
                if local_1a == 4 {
                    i_var3 = 0x10;
                    run = true;
                } else if local_1a == 8 {
                    i_var3 = 0x100;
                    run = true;
                }
                if run {
                    u_var2 = 1;
                    FUN_004df700(p_var1, param_2, i_var3);
                }
            }
        }
        FUN_004be1f0(p_var1);
    }
    u_var2
}
