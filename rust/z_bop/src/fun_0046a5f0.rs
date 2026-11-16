// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00524520: [u16; 512];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0046a5f0(param_1: u16, param_2: u16) -> u32 {
    let mut p1 = param_1;
    let u_var1: u16 = p1 & 0x8000;
    if u_var1 != 0 {
        p1 = p1.wrapping_neg();
    }
    let idx = ((param_2 & 0x7fc0) >> 6) as usize;
    // SAFETY: the table is the game's own; idx is at most 0x1ff.
    let entry = unsafe { (&raw const DAT_00524520).cast::<u16>().add(idx).read() };
    let mut u_var2: u32 = u32::from(entry)
        .wrapping_mul(u32::from(p1))
        .wrapping_add(0x8000);
    if (param_2 & 0x8000) != u_var1 {
        u_var2 = u_var2.wrapping_neg();
    }
    u_var2 >> 0x10
}
