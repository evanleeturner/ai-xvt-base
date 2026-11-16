// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00758830: u32;
    static mut DAT_0075964f: i8;
    static mut DAT_00759657: i8;
    static mut DAT_0075965b: i8;
    static mut DAT_00523570: i8;
    static mut s_frontscreen_d_bmp_005329fc: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _sprintf(dst: *mut i8, fmt: *const i8, ...) -> i32;
    fn _fopen(filename: *const i8, mode: *const i8) -> *mut core::ffi::c_void;
    fn _fclose(stream: *mut core::ffi::c_void) -> i32;
    fn FUN_004d3610(
        param_1: *mut i8,
        param_2: u32,
        param_3: i32,
        param_4: i32,
        param_5: u32,
        param_6: u32,
        param_7: u32,
        param_8: i32,
    ) -> u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004d4d00() -> u32;
    fn FUN_004d4d90();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d5830() -> u32 {
    let mut local_40: [i8; 64] = [0; 64];
    // SAFETY: all callees and globals belong to the game; local_40 is a
    // 64-byte buffer passed where the original passes its own stack buffer,
    // and the global byte ranges are read where the original copies them.
    unsafe {
        let mut i_var2: i32 = 0;
        loop {
            _sprintf(
                local_40.as_mut_ptr(),
                (&raw const s_frontscreen_d_bmp_005329fc).cast::<i8>(),
                i_var2,
            );
            let file = _fopen(local_40.as_ptr(), (&raw const DAT_00523570).cast::<i8>());
            if file.is_null() {
                break;
            }
            i_var2 = i_var2.wrapping_add(1);
            _fclose(file);
        }
        DAT_00758830 = FUN_004d4d00();
        let local_75964f: u32 = (&raw const DAT_0075964f).cast::<u32>().read_unaligned();
        let local_759657: u32 = (&raw const DAT_00759657).cast::<u32>().read_unaligned();
        let local_75965b: i8 = (&raw const DAT_0075965b).read();
        let u_var1 = FUN_004d3610(
            local_40.as_mut_ptr(),
            DAT_00758830,
            0x280,
            0x1e0,
            local_75964f,
            local_759657,
            (local_75965b as i32) as u32,
            0x7596d3,
        );
        FUN_004d4d90();
        u_var1
    }
}
