// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005235d0: i32;
    static mut DAT_009993fc: i32;
    static mut DAT_0051c4bc: i8;
    static mut DAT_005240e0: i8;
    static mut DAT_005240e4: i8;
    static mut DAT_005240d0: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _sprintf(buffer: *mut i8, format: *const i8, ...) -> i32;
    #[link_name = "FID_conflict___wfopen"]
    fn FID_conflict___wfopen(
        filename: *const i8,
        mode: *const core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    fn _fclose(stream: *mut core::ffi::c_void) -> i32;
    fn _fprintf(stream: *mut core::ffi::c_void, format: *const i8, ...) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00450550() {
    let mut local_10: [i8; 16] = [0; 16];
    let mut i_var3: i32 = 0;
    let mut file: *mut core::ffi::c_void;
    // SAFETY: all globals, the format strings and the C library functions are
    // the game's own; the record pointers are read where the original reads them.
    unsafe {
        loop {
            _sprintf(local_10.as_mut_ptr(), &raw const DAT_005240e4, i_var3);
            file = FID_conflict___wfopen(
                local_10.as_ptr(),
                (&raw mut DAT_0051c4bc).cast::<core::ffi::c_void>(),
            );
            if file.is_null() {
                file = FID_conflict___wfopen(
                    local_10.as_ptr(),
                    (&raw mut DAT_005240e0).cast::<core::ffi::c_void>(),
                );
                break;
            }
            if i_var3 == 99 {
                _fclose(file);
                file = FID_conflict___wfopen(
                    local_10.as_ptr(),
                    (&raw mut DAT_005240e0).cast::<core::ffi::c_void>(),
                );
                break;
            }
            i_var3 = i_var3.wrapping_add(1);
            if !(i_var3 < 100) {
                break;
            }
        }

        i_var3 = 0;
        if !file.is_null() {
            if DAT_005235d0 != 0 {
                let mut i_var4: i32 = 0;
                loop {
                    let i_var2 = DAT_009993fc.wrapping_add(i_var4);
                    let mut pc = i_var2.wrapping_add(0xe) as *const i8;
                    if *pc < 9 {
                        pc = i_var2.wrapping_add(0xf) as *const i8;
                        if *(i_var2.wrapping_add(0xe) as *const i8) == 1 && *pc > 0x2f && *pc < 0x34
                        {
                            pc = i_var2.wrapping_add(0x10) as *const i8;
                        }
                    }
                    i_var4 = i_var4.wrapping_add(0x54);
                    i_var3 = i_var3.wrapping_add(1);
                    _fprintf(
                        file,
                        &raw const DAT_005240d0,
                        pc,
                        u32::from(*(i_var2.wrapping_add(8) as *const u8)),
                        u32::from(*(i_var2.wrapping_add(7) as *const u8)),
                        u32::from(*(i_var2.wrapping_add(6) as *const u8)),
                    );
                    if !(i_var3 < DAT_005235d0) {
                        break;
                    }
                }
            }
            _fclose(file);
        }
    }
}
