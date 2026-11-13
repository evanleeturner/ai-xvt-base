// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_009a20ac: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _fread(
        dst: *mut core::ffi::c_void,
        size: u32,
        count: u32,
        stream: *mut core::ffi::c_void,
    ) -> u32;
    fn FUN_0049e060() -> i8;
    fn FUN_0049eb60(param: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0049ea10(
    param_1: *mut core::ffi::c_void,
    param_2: u32,
    param_3: u32,
    param_4: *mut core::ffi::c_void,
) -> u32 {
    let mut dst = param_1 as u32;
    let mut count = param_3;
    let mut i_var3: i32 = 0xf;
    // SAFETY: the callees and the global are the game's own; the buffer and
    // stream are the caller's, passed through to fread as the original does.
    unsafe {
        loop {
            loop {
                i_var3 = i_var3.wrapping_sub(1);
                let s_var2 = _fread(dst as *mut core::ffi::c_void, param_2, count, param_4);
                dst = dst.wrapping_add(s_var2.wrapping_mul(param_2));
                count = count.wrapping_sub(s_var2);
                if count == 0 {
                    _DAT_009a20ac = 0;
                    return param_3;
                }
                if i_var3 == 0 {
                    break;
                }
            }
            loop {
                let c_var1 = FUN_0049e060();
                if c_var1 == b'R' as i8 || c_var1 == b'r' as i8 {
                    break;
                }
                if c_var1 == b'F' as i8 || c_var1 == b'f' as i8 {
                    _DAT_009a20ac = 1;
                    FUN_0049eb60(1);
                    return 0;
                }
            }
            i_var3 = 5;
        }
    }
}
