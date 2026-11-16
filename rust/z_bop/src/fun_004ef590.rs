// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00665da0: u8;
    static mut DAT_00665da3: u8;
    static mut DAT_00665dcb: u8;
    static mut DAT_0066617b: u8;
    static mut DAT_006661a0: u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetDC(hWnd: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn GetSystemPaletteEntries(
        hdc: *mut core::ffi::c_void,
        iStartIndex: u32,
        nEntries: u32,
        lppe: *mut core::ffi::c_void,
    ) -> u32;
    fn ReleaseDC(hWnd: *mut core::ffi::c_void, hdc: *mut core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004ef590(param_1: *mut core::ffi::c_void) {
    // SAFETY: the globals and the Win32 calls are the game's own; the loops
    // write the palette flag bytes at the same addresses, in the same order,
    // as the original.
    unsafe {
        let hdc = GetDC(param_1);
        GetSystemPaletteEntries(hdc, 0, 0x100, (&raw mut DAT_00665da0).cast());

        let mut p: *mut u8 = &raw mut DAT_00665da3;
        loop {
            p.write(0);
            p = p.wrapping_add(4);
            if !(p < (&raw mut DAT_00665dcb)) {
                break;
            }
        }

        p = &raw mut DAT_00665dcb;
        loop {
            p.write(4);
            p = p.wrapping_add(4);
            if !(p < (&raw mut DAT_0066617b)) {
                break;
            }
        }

        p = &raw mut DAT_0066617b;
        let end: *mut u8 = (&raw mut DAT_006661a0).wrapping_add(3);
        loop {
            p.write(0);
            p = p.wrapping_add(4);
            if !(p < end) {
                break;
            }
        }

        ReleaseDC(param_1, hdc);
    }
}
