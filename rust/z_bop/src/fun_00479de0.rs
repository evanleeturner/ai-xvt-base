// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00479e20(param_1: *mut core::ffi::c_void, param_2: *mut i8) -> *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00479de0(
    param_1: *mut core::ffi::c_void,
    param_2: *mut i8,
) -> *mut core::ffi::c_void {
    // SAFETY: param_1 points at the game's own structure holding a count and
    // an array pointer at the given offsets; the callee is the game's own.
    unsafe {
        let count: i32 = param_1.byte_add(6).cast::<i32>().read_unaligned();
        if count > 0 {
            let base_addr: i32 = param_1.byte_add(10).cast::<i32>().read_unaligned();
            let array_base = base_addr as usize as *const *mut core::ffi::c_void;
            let mut i: i32 = 0;
            loop {
                let elem = array_base.offset(i as isize).read_unaligned();
                let res = FUN_00479e20(elem, param_2);
                if !res.is_null() {
                    return res;
                }
                i = i.wrapping_add(1);
                if i >= count {
                    break;
                }
            }
        }
        core::ptr::null_mut()
    }
}
