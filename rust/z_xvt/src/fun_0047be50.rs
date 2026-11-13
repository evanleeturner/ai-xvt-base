// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732948: *mut core::ffi::c_void;
    static mut DAT_00732fec: *mut i8;
    static mut DAT_0071f2f0: *mut *mut i8;
    static mut DAT_00525574: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0047ed40(param_1: *mut i8, param_2: *mut core::ffi::c_void, param_3: i32, param_4: i32);
    fn _fgetc(stream: *mut core::ffi::c_void) -> i32;
    fn _fclose(stream: *mut core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0047be50(param_1: *mut i8, param_2: u16, param_3: i16, param_4: u32) {
    let mut param_2 = param_2;
    let mut param_3 = param_3;
    // SAFETY: the callees and globals are the game's own; the file handle,
    // the line-pointer table and the output buffer are valid as the original assumes.
    unsafe {
        FUN_0047ed40(param_1, (&raw mut DAT_00525574).cast(), 1, 0);
        let file = DAT_00732948;
        let mut local_2: i16 = 0;
        if param_3 != 0 {
            let u_var4: u32 = param_4 & 0xffff;
            loop {
                let i_var3: i32 = local_2 as i32;
                DAT_0071f2f0
                    .add(param_2 as usize)
                    .write_unaligned(DAT_00732fec);
                if (u_var4 as i32) <= i_var3 {
                    param_2 = param_2.wrapping_add(1);
                }
                let mut i_var2: i32 = _fgetc(file);
                let mut u_var1: u32 = file.byte_add(0xC).cast::<u32>().read_unaligned();
                while (u_var1 & 0x10) == 0 && (i_var2 as i16 as i32) != 0xff {
                    if (u_var4 as i32) <= i_var3 {
                        *DAT_00732fec = i_var2 as i8;
                        DAT_00732fec = DAT_00732fec.wrapping_add(1);
                    }
                    i_var2 = _fgetc(file);
                    u_var1 = file.byte_add(0xC).cast::<u32>().read_unaligned();
                }
                if (u_var4 as i32) <= i_var3 {
                    param_3 = param_3.wrapping_sub(1);
                    *DAT_00732fec = 0xffu8 as i8;
                    DAT_00732fec = DAT_00732fec.wrapping_add(1);
                }
                local_2 = local_2.wrapping_add(1);
                if param_3 == 0 {
                    break;
                }
            }
        }
        if (DAT_00732948.byte_add(0xC).cast::<u32>().read_unaligned() & 0x20) == 0 {
            _fclose(DAT_00732948);
        }
    }
}
