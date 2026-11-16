// SPDX-License-Identifier: MIT

use core::arch::asm;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00555cbc: u32;
    static mut DAT_009d682c: u32;
    static mut DAT_00555cb8: u32;
    static mut DAT_009a1fe4: u32;
    static mut DAT_00555cb4: u32;
    static mut DAT_009ed234: u32;
    static mut DAT_00555c8c: u32;
    static mut DAT_009a7b60: u32;
    static mut DAT_00555c90: u32;
    static mut DAT_00a004a8: u32;
    static mut DAT_00555c88: u32;
    static mut DAT_009a8e3c: u32;
    static mut DAT_00555c94: u32;
    static mut DAT_00a004a4: u32;
    static mut DAT_00555ca0: u32;
    static mut DAT_00a004b0: u32;
    static mut DAT_00555c98: u32;
    static mut DAT_00a004cc: u32;
    static mut DAT_00555c9c: u32;
    static mut DAT_00a004b8: u32;
    static mut DAT_00555cac: u32;
    static mut DAT_009fe7f4: u32;
    static mut DAT_00555ca4: u32;
    static mut DAT_00a00488: u32;
    static mut DAT_00555ca8: u32;
    static mut DAT_00a00480: u32;
    static mut DAT_00555cb0: u32;
    static mut DAT_00a00490: u32;
    static mut DAT_009cd272: u32;
    static mut DAT_00a080f4: u32;
    static mut DAT_009d1260: u32;
    static mut DAT_009a20a2: u32;
    static mut DAT_009d8c08: u32;
    static mut DAT_0052747c: u32;
    static mut DAT_005233d8: u32;
}

#[inline(always)]
unsafe fn div32(n: u32, d: u32) -> (u32, u32) {
    let q: u32;
    let r: u32;
    // SAFETY: the machine's own unsigned divide; faults on a zero divisor as the original does.
    unsafe {
        asm!("div {d}", d = in(reg) d, inout("eax") n => q, inout("edx") 0u32 => r, options(nomem, nostack));
    }
    (q, r)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00426f40(param_1: u16, param_2: u16, param_3: u32, param_4: u32) {
    let _ = param_3;
    // SAFETY: all globals are the game's own memory, read and written in the original's order.
    unsafe {
        DAT_00555cbc = DAT_009d682c;
        DAT_00555cb8 = DAT_009a1fe4;
        DAT_00555cb4 = DAT_009ed234;
        DAT_00555c8c = DAT_009a7b60;
        DAT_00555c90 = DAT_00a004a8;
        DAT_00555c88 = DAT_009a8e3c;
        DAT_00555c94 = DAT_00a004a4;
        DAT_00555ca0 = DAT_00a004b0;
        DAT_00555c98 = DAT_00a004cc;
        DAT_00555c9c = DAT_00a004b8;
        DAT_00555cac = DAT_009fe7f4;
        DAT_00555ca4 = DAT_00a00488;
        DAT_00555ca8 = DAT_00a00480;
        DAT_00555cb0 = DAT_00a00490;
        DAT_009d682c = param_1 as u32;
        DAT_009cd272 = (param_1 as u32).wrapping_sub(1);
        DAT_00a080f4 = (param_1 as u32) >> 1;
        DAT_009a1fe4 = param_2 as u32;
        DAT_009d1260 = (param_2 as u32).wrapping_sub(1);
        DAT_009a20a2 = (param_2 as u32) >> 1;
        DAT_009ed234 = param_4;
        let (q, r) = div32(param_4, DAT_009d8c08);
        DAT_009a7b60 = q;
        DAT_0052747c = 0xe000;
        let (q2, _) = div32(r, DAT_005233d8);
        DAT_009a8e3c = q2;
    }
}
