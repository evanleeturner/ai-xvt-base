// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004b7a70(param_1: *mut i8) -> i32;
    fn _malloc(size: u32) -> *mut core::ffi::c_void;
    fn FUN_004b6fb0(param_1: *mut i8, param_2: *mut u32, param_3: i32, param_4: i32) -> i32;
    fn _free(ptr: *mut core::ffi::c_void);
    fn _strncpy(dst: *mut i8, src: *const i8, count: u32) -> *mut i8;
    fn FUN_004b7970(param_1: *mut i8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b4a60(param_1: *mut i8, param_2: *mut i8) -> i32 {
    let mut local_44: [i8; 64] = [0; 64];
    // SAFETY: the callees are the game's own; param_1 and param_2 point at
    // the caller's NUL-terminated strings, and the malloc'd block is 0x414
    // bytes, zeroed as 0x105 dwords.
    unsafe {
        if *param_1 == 0 {
            return 0;
        }
        if *param_2 == 0 {
            return 0;
        }
        let i_var1 = FUN_004b7a70(param_2);
        if i_var1 != -1 {
            return 0;
        }
        let local_4 = _malloc(0x414) as *mut u32;
        if local_4.is_null() {
            return 0;
        }
        let mut puVar2 = local_4;
        let mut count: i32 = 0x105;
        while count != 0 {
            puVar2.write_unaligned(0);
            puVar2 = puVar2.add(1);
            count -= 1;
        }
        let i_var1 = FUN_004b6fb0(param_1, local_4, 1, 1);
        if i_var1 == 0 {
            _free(local_4 as *mut core::ffi::c_void);
            return 0;
        }
        _strncpy(local_44.as_mut_ptr(), param_2, 0x40);
        FUN_004b7970(local_44.as_mut_ptr());
        1
    }
}
