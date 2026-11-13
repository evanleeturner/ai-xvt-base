// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0081b5da: i8;
    static mut DAT_0081b5db: i8;
    static mut DAT_0081b5dc: i8;
    static mut DAT_00531aa8: i8;
    static mut DAT_00531a64: i8;
    static mut DAT_00531a54: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004be6f0() -> u32;
    fn _sprintf(dst: *mut i8, fmt: *const i8, ...) -> i32;
    fn __getdrive() -> i32;
    fn __getcwd(buf: *mut i8, len: i32) -> i32;
    fn _tolower(c: u32) -> i32;
    fn __chdir(path: *const i8) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RegCreateKeyA(
        hKey: *mut core::ffi::c_void,
        lpSubKey: *const i8,
        phkResult: *mut *mut core::ffi::c_void,
    ) -> i32;
    fn RegCloseKey(hKey: *mut core::ffi::c_void) -> i32;
    fn RegOpenKeyExA(
        hKey: *mut core::ffi::c_void,
        lpSubKey: *const i8,
        ulOptions: u32,
        samDesired: u32,
        phkResult: *mut *mut core::ffi::c_void,
    ) -> i32;
    fn RegQueryValueExA(
        hKey: *mut core::ffi::c_void,
        lpValueName: *const i8,
        lpReserved: *mut u32,
        lpType: *mut u32,
        lpData: *mut u8,
        lpcbData: *mut u32,
    ) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004be7f0(param_1: i32) {
    let mut local_108: *mut core::ffi::c_void = core::ptr::null_mut();
    let mut local_104: u32;
    let mut local_100: [i8; 256] = [0; 256];

    // SAFETY: the globals, imports and callees are the game's own; the
    // buffers passed are valid locals or the game's path buffer.
    unsafe {
        if param_1 != 0 {
            let u_var3 = FUN_004be6f0();
            DAT_0081b5db = u_var3 as u8 as i8;
        }
        _sprintf(
            local_100.as_mut_ptr(),
            &raw const DAT_00531aa8 as *const i8,
            1i32,
            1i32,
        );
        RegCreateKeyA(
            0x80000002usize as *mut core::ffi::c_void,
            local_100.as_ptr(),
            &raw mut local_108,
        );
        RegCloseKey(local_108);
        let l_var4 = RegOpenKeyExA(
            0x80000002usize as *mut core::ffi::c_void,
            &raw const DAT_00531a64 as *const i8,
            0,
            0x20019,
            &raw mut local_108,
        );
        if l_var4 != 0 {
            let i_var5 = __getdrive();
            DAT_0081b5da = (i_var5 as i8).wrapping_add(0x60);
            __getcwd(&raw mut DAT_0081b5dc, 0x100);
            return;
        }
        local_104 = 0x100;
        let l_var4 = RegQueryValueExA(
            local_108,
            &raw const DAT_00531a54 as *const i8,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            &raw mut DAT_0081b5dc as *mut u8,
            &raw mut local_104,
        );
        let b_var2 = DAT_0081b5dc as u8;
        if l_var4 != 0 {
            let i_var5 = __getdrive();
            DAT_0081b5da = (i_var5 as i8).wrapping_add(0x60);
            __getcwd(&raw mut DAT_0081b5dc, 0x100);
            RegCloseKey(local_108);
            return;
        }
        let base = &raw mut DAT_0081b5dc;
        let mut len: usize = 0;
        while *base.add(len) != 0 {
            len += 1;
        }
        // index len+1 from DAT_0081b5da == last character of the string
        let p = (&raw mut DAT_0081b5da).wrapping_add(len.wrapping_add(1));
        if *p == b'\\' as i8 {
            *p = 0;
        }
        let i_var5 = _tolower(b_var2 as u32);
        DAT_0081b5da = i_var5 as i8;
        RegCloseKey(local_108);
        __chdir(&raw const DAT_0081b5dc);
    }
}
