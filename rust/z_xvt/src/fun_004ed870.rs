// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00533778: [i8; 1];
    static mut DAT_0052a330: [i8; 1];
    static mut DAT_00757c68: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _fopen(filename: *const i8, mode: *const i8) -> i32;
    fn _fread(ptr: *mut core::ffi::c_void, size: u32, count: u32, stream: i32) -> u32;
    fn _fclose(stream: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004ed870() -> i8 {
    // SAFETY: the callees and globals are the game's own; the buffer is the
    // game's 8-byte area at DAT_00757c68, filled by _fread as the original does.
    unsafe {
        let file = _fopen(
            (&raw const DAT_00533778).cast::<i8>(),
            (&raw const DAT_0052a330).cast::<i8>(),
        );
        if file != 0 {
            let n = _fread(
                (&raw mut DAT_00757c68).cast::<core::ffi::c_void>(),
                1,
                8,
                file,
            );
            _fclose(file);
            if n == 8 {
                let v = DAT_00757c68;
                return ((((v >> 8) & 0x80) == 0x80) as i8).wrapping_add(1);
            }
        }
        0
    }
}
