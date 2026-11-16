// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0064fa20: u32;
    static mut DAT_0064fa24: u32;
    static mut DAT_0064faa8: u32;
    static mut DAT_0064faac: u32;
    static mut DAT_0064fab0: u32;
    static mut LAB_004dc640: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004da080() -> u32;
    fn FUN_004da050();
    fn FUN_004dcfd0(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_004ddc70(param_1: *mut core::ffi::c_void, param_2: *mut core::ffi::c_void);
    fn FUN_004db6f0(param_1: *mut u32, param_2: *mut i32);
    fn FUN_004db4c0();
    fn FUN_004da030();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004dc5a0(param_1: *mut u32) -> u32 {
    let mut local_10: [i32; 4] = [0; 4];
    // SAFETY: the callees and globals are the game's own; param_1 points at
    // the caller's buffer of at least 13 bytes, written where the original does.
    unsafe {
        let i_var1 = FUN_004da080();
        FUN_004da050();
        FUN_004dcfd0(local_10.as_mut_ptr(), 0, 0, 0x280, 0x1e0);
        FUN_004ddc70(
            (&raw mut DAT_0064fa20).cast::<core::ffi::c_void>(),
            (&raw mut DAT_0064fa24).cast::<core::ffi::c_void>(),
        );
        let mut puvar3 = &raw mut DAT_0064faa8;
        let mut i_var2: i32 = 0x40;
        while i_var2 != 0 {
            puvar3.write_unaligned(0);
            puvar3 = puvar3.add(1);
            i_var2 -= 1;
        }
        FUN_004db6f0(&raw mut LAB_004dc640, local_10.as_mut_ptr());
        FUN_004db4c0();
        if i_var1 != 0 {
            FUN_004da030();
        }
        param_1.write_unaligned(DAT_0064faa8);
        param_1.byte_add(4).write_unaligned(DAT_0064faac);
        param_1.byte_add(8).write_unaligned(DAT_0064fab0);
        param_1.byte_add(12).cast::<u8>().write(0);
    }
    1
}
