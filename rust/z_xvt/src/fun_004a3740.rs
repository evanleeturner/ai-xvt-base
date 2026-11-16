// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: u32;
    static mut _DAT_00732e74: u32;
    static mut DAT_00732fe2: u16;
    static mut _DAT_00732950: u16;
    static mut _DAT_00732952: u16;
    static mut _DAT_00732954: u16;
    static mut DAT_00732956: u16;
    static mut _DAT_00732e70: u32;
    static mut DAT_0071ecf4: u32;
    static mut DAT_0070ad47: i8;
    static mut DAT_0070acf0: i8;
    static mut DAT_0052b070: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ac6f0(param_1: u32, param_2: *mut u32) -> u32;
    fn FUN_004a1d70(param_1: u32, param_2: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a3740(param_1: u32, param_2: *mut u32, param_3: u16) {
    // SAFETY: all globals and callees are the game's own memory; param_2 and
    // the table addresses point at the game's data, read where the original does.
    unsafe {
        let addr = (param_1 & 0xffff)
            .wrapping_mul(0x23)
            .wrapping_add(0x16)
            .wrapping_add(DAT_006d4988);
        let u_var2: u32 = u32::from((addr as usize as *const u8).read());
        _DAT_00732e74 = u_var2.wrapping_mul(0x566);
        DAT_00732fe2 = u16::from(
            (&raw mut DAT_0070ad47)
                .byte_add(_DAT_00732e74 as usize)
                .read() as u8,
        );
        _DAT_00732950 = 0x8000;
        _DAT_00732e74 = ((&raw mut DAT_0070acf0) as usize as u32).wrapping_add(_DAT_00732e74);
        _DAT_00732952 = 0x8001;
        let idx = u32::from(param_2.add(1).cast::<u8>().read());
        _DAT_00732e70 =
            ((&raw mut DAT_0052b070) as usize as u32).wrapping_add(idx.wrapping_mul(0xef));
        let u_var1 = FUN_004ac6f0(u_var2, param_2);
        _DAT_00732954 = u_var1 as u16;
        if _DAT_00732954 != 0 {
            DAT_00732956 = param_3;
            FUN_004a1d70(0x9c, DAT_0071ecf4);
            return;
        }
        _DAT_00732954 = param_3;
        FUN_004a1d70(0x9d, DAT_0071ecf4);
    }
}
