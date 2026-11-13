// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: u32;
    static mut DAT_0070ad48: [u8; 0];
    static mut DAT_0072f508: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00444040(param_1: u32, param_2: u32) -> u8 {
    // SAFETY: the globals and tables are the game's own memory; the indices
    // are read exactly as the original computes them.
    unsafe {
        let idx1: u32 = (param_1 & 0xffff)
            .wrapping_mul(0x23)
            .wrapping_add(0x16)
            .wrapping_add(DAT_006d4988);
        let base1 = (&raw mut DAT_0070ad48).cast::<u8>();
        let val1: u32 = u32::from(base1.add(idx1 as usize).read());
        let val2: u32 = u32::from(base1.add(val1.wrapping_mul(0x566) as usize).read());
        if val2 == param_2 {
            return 0;
        }
        let base2 = (&raw mut DAT_0072f508).cast::<u8>();
        let idx2 = param_2.wrapping_mul(0x1e5).wrapping_add(val2);
        (base2.add(idx2 as usize).read() == 0) as u8
    }
}
