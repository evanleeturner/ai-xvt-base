// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_0052b6b8: i16;
    static mut DAT_00b69aa6: i8;
    static s__c__BalanceOfPower__s_0052b6c4: [i8; 0];
    static s__c__s_0052b6bc: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FID_conflict___wfopen(param_1: *mut i8, param_2: *mut i8) -> *mut core::ffi::c_void;
    fn FUN_004ccf50();
    fn FUN_004ccf70();
    fn FUN_004d3350();
    fn _sprintf(dst: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_004d3400();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004cc4a0(
    param_1: *mut i8,
    param_2: *mut i8,
) -> *mut core::ffi::c_void {
    let mut local_100 = [0i8; 256];
    // SAFETY: the callees and globals are the game's own; param_1 and param_2
    // are the caller's C strings, passed through as the original does.
    unsafe {
        let mut p_fvar1 = FID_conflict___wfopen(param_1, param_2);
        'done: {
            if !p_fvar1.is_null() {
                _DAT_0052b6b8 = _DAT_0052b6b8.wrapping_add(1);
                break 'done;
            }
            FUN_004ccf50();
            p_fvar1 = FID_conflict___wfopen(param_1, param_2);
            FUN_004ccf70();
            if !p_fvar1.is_null() {
                _DAT_0052b6b8 = _DAT_0052b6b8.wrapping_add(1);
                break 'done;
            }
            if *param_2 == b'w' as i8 {
                break 'done;
            }
            if *param_2 == b'a' as i8 {
                break 'done;
            }
            if DAT_00b69aa6 == 0 {
                break 'done;
            }
            FUN_004d3350();
            _sprintf(
                local_100.as_mut_ptr(),
                (&raw const s__c__BalanceOfPower__s_0052b6c4).cast::<i8>(),
                DAT_00b69aa6 as u8 as u32,
                param_1,
            );
            p_fvar1 = FID_conflict___wfopen(local_100.as_mut_ptr(), param_2);
            if p_fvar1.is_null() {
                _sprintf(
                    local_100.as_mut_ptr(),
                    (&raw const s__c__s_0052b6bc).cast::<i8>(),
                    DAT_00b69aa6 as u8 as u32,
                    param_1,
                );
                p_fvar1 = FID_conflict___wfopen(local_100.as_mut_ptr(), param_2);
                if p_fvar1.is_null() {
                    break 'done;
                }
            }
            _DAT_0052b6b8 = _DAT_0052b6b8.wrapping_add(1);
        }
        FUN_004d3400();
        p_fvar1
    }
}
