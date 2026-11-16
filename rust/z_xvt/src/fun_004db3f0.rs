// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00523570: i8;
    static mut DAT_00759bd8: [i8; 0];
    static mut DAT_0075a1e0: [i8; 0];
    static mut DAT_0075a1dc: [i32; 0];
    static mut DAT_0075d846: [*mut core::ffi::c_void; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004be160(param_1: *mut i8, param_2: *mut i8) -> *mut core::ffi::c_void;
    fn FUN_004be320(
        param_1: *mut core::ffi::c_void,
        param_2: *mut core::ffi::c_void,
        param_3: u32,
    ) -> i32;
    fn malloc(param_1: u32) -> *mut core::ffi::c_void;
    fn FUN_004be1f0(param_1: *mut core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004db3f0(param_1: *mut i8, param_2: i32) -> i32 {
    let i_var2 = param_2.wrapping_mul(0x60b);
    // SAFETY: the globals, the file helpers and malloc are the game's own;
    // the record at the computed offset is read and written as the original does.
    unsafe {
        let p_var1 = (&raw mut DAT_00759bd8)
            .cast::<u8>()
            .byte_offset(i_var2 as isize)
            .cast::<u32>();
        let p_file = FUN_004be160(param_1, &raw mut DAT_00523570);
        if p_file.is_null() {
            return 0;
        }
        FUN_004be320(p_file, p_var1.cast::<core::ffi::c_void>(), 0x60b);
        let size = p_var1.read_unaligned();
        let pvar4 = malloc(size);
        p_var1.write_unaligned(pvar4 as usize as u32);
        if pvar4.is_null() {
            (&raw mut DAT_0075a1e0)
                .cast::<i8>()
                .offset(i_var2 as isize)
                .write(0);
            FUN_004be1f0(p_file);
            return 0;
        }
        FUN_004be320(p_file, pvar4, size);
        let idx = (&raw mut DAT_0075a1dc)
            .cast::<i32>()
            .offset(i_var2 as isize)
            .read_unaligned();
        (&raw mut DAT_0075d846)
            .cast::<*mut core::ffi::c_void>()
            .offset(idx as isize)
            .write_unaligned(p_var1.cast::<core::ffi::c_void>());
        FUN_004be1f0(p_file);
        1
    }
}
