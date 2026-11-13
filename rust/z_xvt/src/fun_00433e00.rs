// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006da4f8: u32;
    static mut DAT_006db59c: u32;
    static mut DAT_0071e6b8: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00433e00(param_1: u32, param_2: u32) {
    // SAFETY: the globals are the game's own; the base pointer in
    // DAT_0071e6b8 addresses the game's table, written where the original does.
    unsafe {
        let n = DAT_006da4f8;
        if n < 0x128 {
            let base = DAT_0071e6b8;
            let off = n.wrapping_mul(0xc);
            (base.wrapping_add(off) as *mut u32).write_unaligned(param_2);
            (base.wrapping_add(4).wrapping_add(off) as *mut u32).write_unaligned(param_1);
            (base.wrapping_add(8).wrapping_add(off) as *mut u32).write_unaligned(DAT_006db59c);
            let idx = n
                .wrapping_add(0xfffffffe)
                .wrapping_add(n.wrapping_add(1).wrapping_mul(2));
            DAT_006db59c = DAT_0071e6b8.wrapping_add(idx.wrapping_mul(4));
            DAT_006da4f8 = DAT_006da4f8.wrapping_add(1);
        }
    }
}
