// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut LAB_00462460: i8;
    static mut LAB_004624f0: i8;
    static mut LAB_00462530: i8;
    static mut LAB_00462aa0: i8;
    static mut LAB_00462ac0: i8;
    static mut LAB_00462af0: i8;
    static mut DAT_00463720: i8;
    static mut LAB_00463dd0: i8;
    static mut LAB_004643c0: i8;
    static mut LAB_004646d0: i8;
    static mut LAB_004647b0: i8;
    static mut LAB_00463020: i8;
    static mut LAB_00464880: i8;
    static mut LAB_00464920: i8;
    static mut LAB_00415840: i8;
    static mut LAB_00415860: i8;
    static mut LAB_00415890: i8;
    static mut DAT_00416420: i8;
    static mut LAB_00416970: i8;
    static mut LAB_00416d80: i8;
    static mut LAB_00416f10: i8;
    static mut LAB_00416fc0: i8;
    static mut LAB_00415e50: i8;
    static mut LAB_004172b0: i8;

    static mut _DAT_0070a140: *const core::ffi::c_void;
    static mut DAT_006db6c4: *const core::ffi::c_void;
    static mut DAT_006db5a0: *const core::ffi::c_void;
    static mut DAT_0070a174: *mut core::ffi::c_void;
    static mut _DAT_00732e5c: *mut core::ffi::c_void;
    static mut _DAT_00730890: *mut core::ffi::c_void;
    static mut DAT_006db5ac: *mut core::ffi::c_void;
    static mut DAT_0070a184: *mut core::ffi::c_void;
    static mut DAT_00703c8c: *mut core::ffi::c_void;
    static mut DAT_006d9984: *mut core::ffi::c_void;
    static mut DAT_00703c80: *mut core::ffi::c_void;
    static mut DAT_0071eae0: *mut core::ffi::c_void;
    static mut DAT_0070ac2c: *const core::ffi::c_void;
    static mut DAT_00732a30: *const core::ffi::c_void;
    static mut DAT_006d4a48: *const core::ffi::c_void;
    static mut DAT_00703c64: *const core::ffi::c_void;
    static mut _DAT_0071b6b4: *mut core::ffi::c_void;
    static mut DAT_0071e52c: *const core::ffi::c_void;
    static mut DAT_006d4990: *mut core::ffi::c_void;
    static mut DAT_006fedd4: *const core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00426540();
    fn FUN_00461db0();
    fn FUN_00462400();
    fn FUN_00462410();
    fn FUN_004645b0();
    fn FUN_00464640();
    fn FUN_00465320();
    fn FUN_00417070();
    fn FUN_00417130();
    fn FUN_004171d0();
    fn FUN_00417540();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00406040(param_1: i32) {
    // SAFETY: all globals, labels and callees are the game's own memory and
    // code; only their addresses are stored, and the callee is called as in
    // the original.
    unsafe {
        if param_1 != 2 && param_1 != 0 && param_1 != 1 {
            FUN_00426540();
            return;
        }
        _DAT_0070a140 = FUN_00461db0 as *const core::ffi::c_void;
        DAT_006db6c4 = FUN_00462400 as *const core::ffi::c_void;
        DAT_006db5a0 = FUN_00462410 as *const core::ffi::c_void;
        DAT_0070a174 = (&raw mut LAB_00462460).cast();
        _DAT_00732e5c = (&raw mut LAB_004624f0).cast();
        _DAT_00730890 = (&raw mut LAB_00462530).cast();
        if param_1 == 2 {
            DAT_006db5ac = (&raw mut LAB_00415840).cast();
            DAT_0070a184 = (&raw mut LAB_00415860).cast();
            DAT_00703c8c = (&raw mut LAB_00415890).cast();
            DAT_006d9984 = (&raw mut DAT_00416420).cast();
            DAT_00703c80 = (&raw mut LAB_00416970).cast();
            DAT_0071eae0 = (&raw mut LAB_00416d80).cast();
            DAT_0070ac2c = (&raw mut LAB_00416f10)
                .cast::<core::ffi::c_void>()
                .cast_const();
            DAT_00732a30 = (&raw mut LAB_00416fc0)
                .cast::<core::ffi::c_void>()
                .cast_const();
            DAT_006d4a48 = FUN_00417070 as *const core::ffi::c_void;
            DAT_00703c64 = FUN_00417130 as *const core::ffi::c_void;
            _DAT_0071b6b4 = (&raw mut LAB_00415e50).cast();
            DAT_0071e52c = FUN_004171d0 as *const core::ffi::c_void;
            DAT_006d4990 = (&raw mut LAB_004172b0).cast();
            DAT_006fedd4 = FUN_00417540 as *const core::ffi::c_void;
        } else {
            DAT_006db5ac = (&raw mut LAB_00462aa0).cast();
            DAT_0070a184 = (&raw mut LAB_00462ac0).cast();
            DAT_00703c8c = (&raw mut LAB_00462af0).cast();
            DAT_006d9984 = (&raw mut DAT_00463720).cast();
            DAT_00703c80 = (&raw mut LAB_00463dd0).cast();
            DAT_0071eae0 = (&raw mut LAB_004643c0).cast();
            DAT_0070ac2c = FUN_004645b0 as *const core::ffi::c_void;
            DAT_00732a30 = FUN_00464640 as *const core::ffi::c_void;
            DAT_006d4a48 = (&raw mut LAB_004646d0)
                .cast::<core::ffi::c_void>()
                .cast_const();
            DAT_00703c64 = (&raw mut LAB_004647b0)
                .cast::<core::ffi::c_void>()
                .cast_const();
            _DAT_0071b6b4 = (&raw mut LAB_00463020).cast();
            DAT_0071e52c = (&raw mut LAB_00464880)
                .cast::<core::ffi::c_void>()
                .cast_const();
            DAT_006d4990 = (&raw mut LAB_00464920).cast();
            DAT_006fedd4 = FUN_00465320 as *const core::ffi::c_void;
        }
        FUN_00426540();
    }
}
