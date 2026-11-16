// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006572c8: u32;
    static mut DAT_0052f1d8: u32;
    static mut DAT_0052f1d4: u32;
    static mut DAT_00521b60: u32;
    static mut DAT_0052f1f4: u32;
    static mut DAT_0052f1c0: u32;
    static mut DAT_0052f1cc: u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_0045bbe0();
    fn FUN_004b7958();
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004b51f0(param_1: u32, param_2: u32, param_3: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004b5283() -> u32 {
    // SAFETY: the globals and callees are the game's own; the COM-like
    // object pointers are read and called exactly as the original does.
    unsafe {
        if DAT_006572c8 == 0 {
            FUN_0045bbe0();
            return 0;
        }
        DAT_0052f1d8 = 0x3363;
        DAT_0052f1d4 = 1;
        FUN_0045bbe0();
        let f1 = (DAT_006572c8 as *const u32).read_unaligned();
        let f1: unsafe extern "stdcall" fn(
            *mut core::ffi::c_void,
            *mut core::ffi::c_void,
            *mut core::ffi::c_void,
        ) -> i32 = core::mem::transmute(f1 as usize as *const core::ffi::c_void);
        let i_var2 = f1(
            DAT_006572c8 as usize as *mut core::ffi::c_void,
            &raw mut DAT_00521b60 as *mut core::ffi::c_void,
            &raw mut DAT_0052f1f4 as *mut core::ffi::c_void,
        );
        if i_var2 != 0 {
            FUN_004b51f0(i_var2 as u32, 0x52f210, 0x79);
            FUN_0045bbe0();
            return 0;
        }
        FUN_0045bbe0();
        DAT_0052f1c0 = 0;
        let vt = (DAT_0052f1f4 as *const u32).read_unaligned();
        let f2 = ((vt as usize + 0x10) as *const u32).read_unaligned();
        let f2: unsafe extern "stdcall" fn(
            *mut core::ffi::c_void,
            *const core::ffi::c_void,
            i32,
        ) -> i32 = core::mem::transmute(f2 as usize as *const core::ffi::c_void);
        let i_var2 = f2(
            DAT_0052f1f4 as usize as *mut core::ffi::c_void,
            FUN_004b7958 as unsafe extern "stdcall" fn() as usize as *const core::ffi::c_void,
            0,
        );
        if i_var2 != 0 {
            FUN_004b51f0(i_var2 as u32, 0x52f210, 0x79);
            FUN_0045bbe0();
            return 0;
        }
        if DAT_0052f1c0 == 0 {
            return 0;
        }
        FUN_004b51f0(0, 0x52f210, 0x79);
        FUN_0045bbe0();
        DAT_0052f1cc = 1;
        FUN_0045bbe0();
        1
    }
}
