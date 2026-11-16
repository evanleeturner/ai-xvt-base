// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d9d56: u32;
    static mut DAT_006d9d50: u16;
    static mut DAT_006d4988: u32;
    static mut DAT_00732a2c: u32;
    static mut DAT_00732a28: u32;
    static mut DAT_00732a24: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00421c70(param_1: u32, param_2: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0042a060() {
    // SAFETY: the globals and the callee are the game's own; the addresses
    // computed from DAT_006d9d56 and DAT_006d4988 point at game memory and
    // are read and written where the original does.
    unsafe {
        let a = (DAT_006d9d56.wrapping_add(0x18) as *const u16).read_unaligned();
        let idx = (DAT_006d9d50 as u32)
            .wrapping_mul(0x23)
            .wrapping_add(0x16)
            .wrapping_add(DAT_006d4988);
        let b = (idx as *const u8).read();
        FUN_00421c70(a as u32, b as u32);
        let tmp_1f = DAT_00732a2c;
        (DAT_006d9d56.wrapping_add(0x1f) as *mut u32).write_unaligned(tmp_1f);
        let tmp_23 = DAT_00732a28;
        (DAT_006d9d56.wrapping_add(0x23) as *mut u32).write_unaligned(tmp_23);
        let tmp_27 = DAT_00732a24;
        (DAT_006d9d56.wrapping_add(0x27) as *mut u32).write_unaligned(tmp_27);
    }
}
