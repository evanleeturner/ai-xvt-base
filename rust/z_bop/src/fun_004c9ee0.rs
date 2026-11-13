// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b6a250: u32;
    static mut DAT_0051a854: i8;
    static mut DAT_0052b5e8: [u8; 21];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004ccb90() -> i8;
    fn FID_conflict___wfopen(param_1: *const i8, param_2: *const i8) -> *mut core::ffi::c_void;
    fn _fclose(param_1: *mut core::ffi::c_void);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004c9ee0() -> u32 {
    let mut local_80: [u8; 128] = [0u8; 128];
    // SAFETY: the globals and callees are the game's own; the source string
    // is 21 bytes, and local_80 is a local 128-byte buffer filled as the
    // original does.
    unsafe {
        let src = (&raw const DAT_0052b5e8).cast::<u8>();
        let mut i = 0usize;
        while i < 21 {
            local_80[i] = src.add(i).read();
            i += 1;
        }
        let mut j = 21usize;
        while j < 128 {
            local_80[j] = 0;
            j += 1;
        }
        local_80[0] = FUN_004ccb90() as u8;
        if local_80[0] != 0 {
            let file = FID_conflict___wfopen(
                local_80.as_ptr().cast::<i8>(),
                &raw mut DAT_0051a854 as *const i8,
            );
            if !file.is_null() {
                _fclose(file);
            }
            DAT_00b6a250 = (!file.is_null()) as u32;
            return DAT_00b6a250;
        }
        0
    }
}
