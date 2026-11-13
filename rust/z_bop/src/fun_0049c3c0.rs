// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a07bd8: *mut core::ffi::c_void;
    static mut DAT_00523434: core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0049e720(param_1: *mut i8, param_2: *mut core::ffi::c_void, param_3: i32, param_4: i32);
    fn FUN_0049eb60(param_1: i32);
    fn _fread(
        buf: *mut core::ffi::c_void,
        size: u32,
        count: u32,
        file: *mut core::ffi::c_void,
    ) -> u32;
    fn FUN_0049e9c0(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0049c3c0(param_1: *mut i8, param_2: *mut u8) -> i16 {
    let mut local_200 = [0u8; 512];
    let mut param_2 = param_2;
    // SAFETY: the callees and globals are the game's own; param_2 points at
    // the caller's output buffer, written byte by byte as the original does.
    unsafe {
        FUN_0049e720(param_1, &raw mut DAT_00523434, 1, 0);
        let file = DAT_00a07bd8;
        if file.is_null() {
            FUN_0049eb60(1);
        }
        let mut s_var6: i16 = 0;
        let mut u_var2: u32 = 0x200;
        loop {
            u_var2 = _fread(local_200.as_mut_ptr().cast(), 1, u_var2 & 0xffff, file);
            let u_var1 = u_var2 as u16;
            let mut p_var5 = param_2;
            let mut u_var3: u16 = 0;
            if u_var1 != 0 {
                loop {
                    param_2 = p_var5.add(1);
                    let u_var4 = u_var3.wrapping_add(1);
                    *p_var5 = local_200[u_var3 as usize];
                    p_var5 = param_2;
                    u_var3 = u_var4;
                    if !(u_var4 < u_var1) {
                        break;
                    }
                }
            }
            s_var6 = s_var6.wrapping_add(u_var1 as i16);
            if u_var1 != 0x200 {
                break;
            }
        }
        FUN_0049e9c0(0);
        s_var6
    }
}
