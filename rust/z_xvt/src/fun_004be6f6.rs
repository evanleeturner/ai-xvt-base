// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut in_stack_00000160: i32;
    static mut DAT_0052a330: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetLogicalDriveStringsA(n_buffer_length: u32, lp_buffer: *mut i8) -> u32;
    fn GetDriveTypeA(lp_root_path_name: *const i8) -> u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _fopen(file: *const i8, mode: *const i8) -> *mut core::ffi::c_void;
    fn _fclose(file: *mut core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004be6f6() -> i8 {
    let mut i_var6: i32 = 0;
    let mut i_stack_00000004: i32 = 0;
    let mut stack_0x58: [i8; 0x200] = [0; 0x200];
    let mut stack_0x08: [i8; 0x200] = [0; 0x200];
    // SAFETY: the Win32 calls, the C runtime and the globals are the game's
    // own; the buffers are local and the string pointers are walked as the
    // original walks them.
    unsafe {
        let n_buffer_length = GetLogicalDriveStringsA(0, core::ptr::null_mut());
        GetLogicalDriveStringsA(n_buffer_length, stack_0x58.as_mut_ptr());
        let mut lp_root_path_name: *mut i8 = stack_0x58.as_mut_ptr();
        if 0 < (n_buffer_length as i32) {
            loop {
                let u_var2 = GetDriveTypeA(lp_root_path_name);
                if u_var2 == 5 {
                    // copy root path including NUL
                    let mut src = lp_root_path_name as *const i8;
                    let mut dst = stack_0x08.as_mut_ptr();
                    loop {
                        let c = *src;
                        *dst = c;
                        src = src.add(1);
                        dst = dst.add(1);
                        if c == 0 {
                            break;
                        }
                    }
                    // append string at in_stack_00000160 + 1
                    let mut src = in_stack_00000160.wrapping_add(1) as usize as *const i8;
                    let mut end = stack_0x08.as_mut_ptr();
                    while *end != 0 {
                        end = end.add(1);
                    }
                    loop {
                        let c = *src;
                        *end = c;
                        src = src.add(1);
                        end = end.add(1);
                        if c == 0 {
                            break;
                        }
                    }
                    let file = _fopen(stack_0x08.as_ptr(), (&raw mut DAT_0052a330) as *const i8);
                    if !file.is_null() {
                        _fclose(file);
                        i_stack_00000004 = 1;
                        break;
                    }
                }
                lp_root_path_name = lp_root_path_name.byte_add(4);
                i_var6 = i_var6.wrapping_add(4);
                if !(i_var6 < (n_buffer_length as i32)) {
                    break;
                }
            }
        }
        if i_stack_00000004 == 0 {
            return 0;
        }
        *lp_root_path_name
    }
}
