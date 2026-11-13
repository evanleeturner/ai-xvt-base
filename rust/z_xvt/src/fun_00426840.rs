// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00536ec4: u16;
    static mut DAT_00536ec0: u16;
    static mut DAT_00536ebc: u16;
    static mut DAT_00536e94: u16;
    static mut DAT_00536e98: u32;
    static mut DAT_00536e90: u16;
    static mut DAT_00536e9c: u32;
    static mut DAT_00536ea8: u32;
    static mut DAT_00536ea0: u32;
    static mut DAT_00536ea4: u32;
    static mut DAT_00536eb4: u32;
    static mut DAT_00536eac: u32;
    static mut DAT_00536eb0: u32;
    static mut DAT_00536eb8: u32;

    static mut DAT_007091ae: u16;
    static mut DAT_006d4984: u16;
    static mut DAT_0071f2e4: u32;
    static mut DAT_006da4fc: u32;
    static mut DAT_0073255c: u32;
    static mut DAT_006db7bc: u32;
    static mut DAT_00732558: u32;
    static mut DAT_00732568: u32;
    static mut DAT_00732584: u32;
    static mut DAT_00732578: u32;
    static mut DAT_00732530: u32;
    static mut DAT_0073253c: u32;
    static mut DAT_00732538: u32;
    static mut DAT_00732548: u32;

    static mut DAT_006ffbf2: u16;
    static mut DAT_00732e44: u16;
    static mut DAT_00703be4: u16;
    static mut DAT_006d4a42: u16;
    static mut DAT_00527e74: u16;
    static mut DAT_0070acc8: u32;
    static mut DAT_00525518: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(code: u32, flags: u32, nargs: u32, args: *const u32);
}

#[inline(always)]
unsafe fn machine_div(num: u32, hi: u32, den: u32) -> (u32, u32) {
    let q: u32;
    let r: u32;
    // SAFETY: the machine's own unsigned divide; a zero divisor faults as in the original.
    unsafe {
        core::arch::asm!(
            "div {d}",
            d = in(reg) den,
            inout("eax") num => q,
            inout("edx") hi => r,
        );
    }
    (q, r)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00426840(param_1: u16, param_2: u16, param_3: u32, param_4: u32) {
    let _ = param_3;
    // SAFETY: all globals and the imported function are the game's own memory
    // and code, accessed in the original's order.
    unsafe {
        DAT_00536ec4 = DAT_007091ae;
        DAT_00536ec0 = DAT_006d4984;
        DAT_00536ebc = DAT_0071f2e4 as u16;
        DAT_00536e94 = DAT_006da4fc as u16;
        DAT_00536e98 = DAT_0073255c;
        DAT_00536e90 = DAT_006db7bc as u16;
        DAT_00536e9c = DAT_00732558;
        DAT_00536ea8 = DAT_00732568;
        DAT_00536ea0 = DAT_00732584;
        DAT_00536ea4 = DAT_00732578;
        DAT_00536eb4 = DAT_00732530;
        DAT_00536eac = DAT_0073253c;
        DAT_00536eb0 = DAT_00732538;
        DAT_00536eb8 = DAT_00732548;

        DAT_007091ae = param_1;
        DAT_006ffbf2 = param_1.wrapping_sub(1);
        DAT_00732e44 = param_1 >> 1;
        DAT_0071f2e4 = param_4;
        DAT_006d4984 = param_2;
        DAT_00703be4 = param_2.wrapping_sub(1);
        DAT_006d4a42 = param_2 >> 1;

        if DAT_0070acc8 == 0 {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        }
        let (q, _) = machine_div(param_4, 0, DAT_0070acc8);
        DAT_006da4fc = q;

        DAT_00527e74 = 0xe000;

        if DAT_0070acc8 == 0 || DAT_00525518 == 0 {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        }
        let (_, rem) = machine_div(param_4, 0, DAT_0070acc8);
        let (q2, _) = machine_div(rem, 0, DAT_00525518);
        DAT_006db7bc = q2;
    }
}
