// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0064f438: i32;
    static mut DAT_0064f038: i32;
    static s_Software_Microsoft_Windows_Curre_00532528: [i8; 0];
    static s_EnableAutodial_00532518: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RegOpenKeyExA(
        h_key: *mut core::ffi::c_void,
        lp_sub_key: *const i8,
        ul_options: u32,
        sam_desired: u32,
        phk_result: *mut *mut core::ffi::c_void,
    ) -> i32;
    fn RegQueryValueExA(
        h_key: *mut core::ffi::c_void,
        lp_value_name: *const i8,
        lp_reserved: *mut u32,
        lp_type: *mut u32,
        lp_data: *mut u8,
        lpcb_data: *mut u32,
    ) -> i32;
    fn RegSetValueExA(
        h_key: *mut core::ffi::c_void,
        lp_value_name: *const i8,
        reserved: u32,
        dw_type: u32,
        lp_data: *const u8,
        cb_data: u32,
    ) -> i32;
    fn RegCloseKey(h_key: *mut core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004cf740() -> i32 {
    let mut local_c: *mut core::ffi::c_void = core::ptr::null_mut();
    let mut local_8: u32;
    let mut local_4: [u8; 4] = [0; 4];
    // SAFETY: the globals, string constants and registry APIs are the game's
    // own; the locals passed by pointer outlive each call.
    unsafe {
        DAT_0064f438 = 0;
        let l_status = RegOpenKeyExA(
            0x8000_0001usize as *mut core::ffi::c_void,
            (&raw const s_Software_Microsoft_Windows_Curre_00532528).cast::<i8>(),
            0,
            0xf003f,
            &raw mut local_c,
        );
        if l_status != 0 {
            return 0;
        }
        local_8 = 5;
        let l_status = RegQueryValueExA(
            local_c,
            (&raw const s_EnableAutodial_00532518).cast::<i8>(),
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            (&raw mut DAT_0064f038).cast::<u8>(),
            &raw mut local_8,
        );
        if l_status != 0 {
            RegCloseKey(local_c);
            return 0;
        }
        if DAT_0064f038 == 0 {
            RegCloseKey(local_c);
            return 0;
        }
        local_4[0] = 0;
        local_4[1] = 0;
        local_4[2] = 0;
        local_4[3] = 0;
        let l_status = RegSetValueExA(
            local_c,
            (&raw const s_EnableAutodial_00532518).cast::<i8>(),
            0,
            3,
            local_4.as_ptr(),
            4,
        );
        if l_status != 0 {
            RegCloseKey(local_c);
            return 0;
        }
        DAT_0064f438 = 1;
        RegCloseKey(local_c);
        1
    }
}
