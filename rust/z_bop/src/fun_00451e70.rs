// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: u32;
    static mut DAT_00a08292: u16;
    static mut DAT_009d8c87: u32;
    static mut DAT_0051c560: u32;
    static mut DAT_009d8c30: u32;
    static mut DAT_00a07be4: i16;
    static mut DAT_00a07be6: i16;
    static mut DAT_009ecc34: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00451bf0(param_1: i16, param_2: u32);
    fn FUN_0043c740(param_1: u32, param_2: *mut u32) -> u32;
    fn FUN_00450650(param_1: i16, param_2: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00451e70(param_1: u32, param_2: *mut u32, param_3: u16) {
    // SAFETY: the globals and callees are the game's own; param_2 and the
    // table at DAT_009a1fe8 are read where the original reads them.
    unsafe {
        let u_var2: u32 = u32::from(
            ((param_1 & 0xffff)
                .wrapping_mul(0x23)
                .wrapping_add(0x16)
                .wrapping_add(DAT_009a1fe8) as *const u8)
                .read(),
        );
        DAT_00a08292 = u16::from(
            (&raw mut DAT_009d8c87)
                .cast::<u8>()
                .byte_add(u_var2.wrapping_mul(0x566) as usize)
                .read(),
        );
        FUN_00451bf0(
            0,
            (&raw mut DAT_0051c560 as u32)
                .wrapping_add(u32::from(param_2.add(1).cast::<u8>().read()).wrapping_mul(0x101)),
        );
        FUN_00451bf0(
            1,
            (&raw mut DAT_009d8c30 as u32).wrapping_add(u_var2.wrapping_mul(0x566)),
        );
        let u_var1 = FUN_0043c740(u_var2, param_2);
        DAT_00a07be4 = u_var1 as i16;
        if DAT_00a07be4 != 0 {
            DAT_00a07be6 = param_3 as i16;
            FUN_00450650(0x9d, DAT_009ecc34);
            return;
        }
        DAT_00a07be4 = param_3 as i16;
        FUN_00450650(0x9e, DAT_009ecc34);
    }
}
