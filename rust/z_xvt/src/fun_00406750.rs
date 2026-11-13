// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0071b6b2: u16;
    static mut DAT_0070acec: u16;
    static mut DAT_007091ae: u16;
    static mut DAT_0070a170: u16;
    static mut DAT_006ffbf2: u16;
    static mut DAT_00732a20: u16;
    static mut DAT_006d4984: u16;
    static mut DAT_006db598: u32;
    static mut DAT_0070a144: u32;
    static mut DAT_0071f2e0: u16;
    static mut DAT_00703be4: u16;
    static mut DAT_006db6d0: u32;
    static mut DAT_00525518: u32;
    static mut DAT_00525738: u32;
    static mut DAT_0071ed00: u16;
    static mut DAT_00730884: *mut u16;
    static mut DAT_006ffc00: u16;
    static mut DAT_00732e42: u16;
    static mut DAT_00525734: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00406b50(param_1: u16, param_2: *mut u16);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00406750(param_1: u16) {
    // SAFETY: all globals and the callee are the game's own memory and code,
    // accessed in the same order as the original.
    unsafe {
        DAT_0071b6b2 = 0xffff;
        DAT_0070acec = DAT_007091ae;
        DAT_0070a170 = DAT_006ffbf2;
        DAT_00732a20 = DAT_006d4984;
        DAT_006db598 = DAT_0070a144;
        DAT_0071f2e0 = DAT_00703be4;
        DAT_006db6d0 = u32::from(DAT_007091ae).wrapping_mul(DAT_00525518);
        DAT_00525738 = (DAT_0071ed00 == 0) as u32;
        DAT_00730884 = &raw mut DAT_006ffc00;
        DAT_00732e42 = 0x2000;
        if DAT_00525738 != 1 {
            DAT_00732e42 = 0x2200;
        }
        if DAT_006ffc00 != param_1 || DAT_00525734 == 0 {
            FUN_00406b50(param_1, &raw mut DAT_006ffc00);
            DAT_00525734 = 1;
        }
    }
}
