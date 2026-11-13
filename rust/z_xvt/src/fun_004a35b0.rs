// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732fe2: u16;
    static mut DAT_0071b73a: u16;
    static mut DAT_0071ecf4: u32;
    static mut DAT_0070ad47: u8;
    static mut DAT_0070ad48: u8;
    static mut DAT_0071b73c: u16;
    static mut _DAT_00732950: u16;
    static mut _DAT_00732e74: i32;
    static mut _DAT_00732952: i16;
    static mut DAT_0052b070: u8;
    static mut DAT_006d4988: u32;
    static mut _DAT_00732954: i16;
    static mut DAT_00732956: i16;
    static mut _DAT_00732e70: *mut core::ffi::c_void;
    static mut DAT_0070acf0: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ac6f0(param_1: i32, param_2: *mut u32) -> u32;
    fn FUN_004a1d70(param_1: i32, param_2: u32);
    fn FUN_0040b730(
        param_1: u32,
        param_2: u32,
        param_3: u32,
        param_4: u32,
        param_5: u32,
        param_6: u32,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a35b0(
    param_1: u32,
    param_2: *mut u32,
    param_3: i16,
    param_4: u32,
    param_5: i16,
    param_6: u32,
) {
    let _ = param_6;
    // SAFETY: the globals and callees are the game's own; the addresses
    // computed are the game's tables, read and written where the original does.
    unsafe {
        let u_var3 = param_1 & 0xffff;
        let u_var2: u32 = ((u_var3
            .wrapping_mul(0x23)
            .wrapping_add(0x16)
            .wrapping_add(DAT_006d4988)) as usize as *const u8)
            .read() as u32;
        let mut i_var4: i32 = u_var2.wrapping_mul(0x566) as i32;
        DAT_00732fe2 = (&raw mut DAT_0070ad47).byte_add(i_var4 as usize).read() as u16;
        let idx = DAT_0071ecf4.wrapping_mul(0x5bd) as usize;
        if (&raw mut DAT_0071b73a)
            .byte_add(idx)
            .cast::<u16>()
            .read_unaligned()
            == DAT_00732fe2
            && ((&raw mut DAT_0070ad48).byte_add(i_var4 as usize).read() as u16)
                == (&raw mut DAT_0071b73c)
                    .byte_add(idx)
                    .cast::<u16>()
                    .read_unaligned()
        {
            if param_5 == 0 {
                _DAT_00732950 = 0x8000;
                _DAT_00732e74 = ((&raw mut DAT_0070acf0) as i32).wrapping_add(i_var4);
                _DAT_00732952 = -0x7fff;
                let b = param_2.add(1).cast::<u8>().read() as u32;
                _DAT_00732e70 = (&raw mut DAT_0052b070)
                    .byte_add(b.wrapping_mul(0xef) as usize)
                    .cast::<core::ffi::c_void>();
                let u_var1 = FUN_004ac6f0(u_var2 as i32, param_2);
                _DAT_00732954 = u_var1 as i16;
                if _DAT_00732954 == 0 {
                    _DAT_00732954 = param_3;
                    i_var4 = 0x93;
                } else {
                    DAT_00732956 = param_3;
                    i_var4 = 0x92;
                }
            } else {
                _DAT_00732e70 = (&raw mut DAT_0070acf0)
                    .byte_add(i_var4 as usize)
                    .cast::<core::ffi::c_void>();
                _DAT_00732952 = param_3;
                _DAT_00732950 = 0x8000;
                i_var4 = 0x109;
            }
            FUN_004a1d70(i_var4, DAT_0071ecf4);
            FUN_0040b730(DAT_0071ecf4, u_var3, 1, param_4 & 0xffff, u_var3, 0xffff);
        }
    }
}
