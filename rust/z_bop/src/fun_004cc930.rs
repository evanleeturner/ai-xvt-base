// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b69aa6: i8;
    static mut DAT_0052b6f4: i8;
    static mut DAT_0051a854: i8;
    static mut DAT_0052b6dc: i8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004d3350();
    fn FUN_004d3400();
    fn FID_conflict___wfopen(param_1: *mut i8, param_2: *mut i8) -> *mut core::ffi::c_void;
    fn _fclose(param_1: *mut core::ffi::c_void);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004cc930() -> i32 {
    let mut local_50: [i8; 80] = [0; 80];
    // SAFETY: the globals and callees are the game's own; the copies read the
    // game's string data and write within the 80-byte local buffer.
    unsafe {
        if DAT_00b69aa6 == 0 {
            return 0;
        }
        FUN_004d3350();
        let mut pc3: *const i8 = &raw mut DAT_0052b6f4;
        let mut pc4: *mut i8 = local_50.as_mut_ptr();
        let mut i = 6;
        while i != 0 {
            pc4.cast::<i32>()
                .write_unaligned(pc3.cast::<i32>().read_unaligned());
            pc3 = pc3.byte_add(4);
            pc4 = pc4.byte_add(4);
            i -= 1;
        }
        *pc4 = *pc3;
        local_50[0] = DAT_00b69aa6;
        let p1 = FID_conflict___wfopen(local_50.as_mut_ptr(), &raw mut DAT_0051a854);
        if p1.is_null() {
            FUN_004d3400();
            return 0;
        }
        _fclose(p1);
        let mut pc3: *const i8 = &raw mut DAT_0052b6dc;
        let mut pc4: *mut i8 = local_50.as_mut_ptr();
        let mut i = 5;
        while i != 0 {
            pc4.cast::<i32>()
                .write_unaligned(pc3.cast::<i32>().read_unaligned());
            pc3 = pc3.byte_add(4);
            pc4 = pc4.byte_add(4);
            i -= 1;
        }
        pc4.cast::<i16>()
            .write_unaligned(pc3.cast::<i16>().read_unaligned());
        local_50[0] = DAT_00b69aa6;
        let p1 = FID_conflict___wfopen(local_50.as_mut_ptr(), &raw mut DAT_0051a854);
        if p1.is_null() {
            FUN_004d3400();
            return 0;
        }
        FUN_004d3400();
        _fclose(p1);
        1
    }
}
