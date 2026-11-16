// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0081bc68: u32;
    static mut DAT_00523570: [i8; 1];
    static mut DAT_0053231c: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004be6c0() -> u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _fopen(filename: *const i8, mode: *const i8) -> *mut core::ffi::c_void;
    fn _fclose(stream: *mut core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004c9600() -> u32 {
    let mut local_80: [i8; 128] = [0; 128];
    // SAFETY: the globals and callees are the game's own; the source string
    // is at least 21 bytes and local_80 holds 128 bytes.
    unsafe {
        let src = (&raw mut DAT_0053231c).cast::<i8>();
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
        local_80[0] = FUN_004be6c0() as i8;
        if local_80[0] != 0 {
            let file = _fopen(local_80.as_ptr(), (&raw mut DAT_00523570).cast::<i8>());
            if !file.is_null() {
                _fclose(file);
            }
            DAT_0081bc68 = (!file.is_null()) as u32;
            return DAT_0081bc68;
        }
        0
    }
}
