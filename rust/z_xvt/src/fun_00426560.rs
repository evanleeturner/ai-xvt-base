// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052551c: u32;
    static mut DAT_0070acc8: u32;
    static mut DAT_00525518: u32;
    static mut DAT_007091ae: i16;
    static mut DAT_006ffbf2: i16;
    static mut DAT_00732e44: i16;
    static mut _DAT_0071f2e4: u32;
    static mut DAT_006d4984: i16;
    static mut DAT_00703be4: i16;
    static mut DAT_006d4a42: i16;
    static mut _DAT_006da4fc: u32;
    static mut _DAT_006db7bc: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(a: u32, b: u32, c: u32, d: *const u32);
}

#[inline(always)]
unsafe fn machine_div(a: u32, b: u32) -> (u32, u32) {
    let q: u32;
    let r: u32;
    // SAFETY: the machine's unsigned divide; faults on a zero divisor as the original does.
    unsafe {
        core::arch::asm!(
            "div {d}",
            d = in(reg) b,
            inout("eax") a => q,
            inout("edx") 0u32 => r,
            options(nostack)
        );
    }
    (q, r)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00426560(param_1: u32, param_2: u32, param_3: u32, param_4: u32) {
    let _ = param_3;
    let mut param_1 = param_1;
    let mut param_2 = param_2;
    let mut param_4 = param_4;
    // SAFETY: the globals and the imported function are the game's own.
    unsafe {
        if DAT_0052551c == 0xa0 {
            param_1 >>= 1;
            param_2 >>= 1;
            param_4 = param_4
                .wrapping_add(DAT_0070acc8.wrapping_mul(0x78))
                .wrapping_add(0xa0);
        }
        DAT_007091ae = param_1 as i16;
        DAT_006ffbf2 = (param_1 as u16).wrapping_sub(1) as i16;
        DAT_00732e44 = (param_1 >> 1) as i16;
        _DAT_0071f2e4 = param_4;
        DAT_006d4984 = param_2 as i16;
        DAT_00703be4 = (param_2 as u16).wrapping_sub(1) as i16;
        DAT_006d4a42 = (param_2 >> 1) as i16;

        if DAT_0070acc8 == 0 {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        }
        let (q, r) = machine_div(param_4, DAT_0070acc8);
        _DAT_006da4fc = q;

        if DAT_00525518 == 0 {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        }
        let (q2, _) = machine_div(r, DAT_00525518);
        _DAT_006db7bc = q2 as i32;
    }
}
