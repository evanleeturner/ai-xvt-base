// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00665418: u32;
    static mut DAT_00665018: u32;
    static s_EnableAutodial_0052ba10: [i8; 0];
    static s_Software_Microsoft_Windows_Curre_0052ba20: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RegOpenKeyExA(
        hKey: *mut core::ffi::c_void,
        lpSubKey: *const i8,
        ulOptions: u32,
        samDesired: u32,
        phkResult: *mut *mut core::ffi::c_void,
    ) -> i32;
    fn RegSetValueExA(
        hKey: *mut core::ffi::c_void,
        lpValueName: *const i8,
        Reserved: u32,
        dwType: u32,
        lpData: *const u8,
        cbData: u32,
    ) -> i32;
    fn RegCloseKey(hKey: *mut core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d2cb0() -> u32 {
    let mut h_key: *mut core::ffi::c_void = core::ptr::null_mut();
    // SAFETY: the globals, strings and registry functions are the game's
    // own; h_key is a local out-parameter used only after a successful open.
    unsafe {
        if DAT_00665418 == 0 {
            return 0;
        }
        let l_status = RegOpenKeyExA(
            0x80000001usize as *mut core::ffi::c_void,
            (&raw const s_Software_Microsoft_Windows_Curre_0052ba20).cast::<i8>(),
            0,
            0xf003f,
            &raw mut h_key,
        );
        if l_status != 0 {
            return 0;
        }
        let l_status = RegSetValueExA(
            h_key,
            (&raw const s_EnableAutodial_0052ba10).cast::<i8>(),
            0,
            3,
            (&raw mut DAT_00665018).cast::<u8>().cast_const(),
            4,
        );
        if l_status != 0 {
            RegCloseKey(h_key);
            return 0;
        }
        DAT_00665418 = 0;
        RegCloseKey(h_key);
        1
    }
}
