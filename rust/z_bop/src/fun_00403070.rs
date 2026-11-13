// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a73d0: u16;
    static mut DAT_00524100: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00425b70(param_1: u32, param_2: u16) -> u32;
    fn FUN_00403360(param_1: u32, param_2: u32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00403070(param_1: u32) -> u8 {
    // SAFETY: the globals and callees are the game's own; the table read is
    // at the same offset (in u16 units) as the original.
    unsafe {
        let idx = (DAT_009a73d0 as u32).wrapping_mul(2) as usize;
        let v = (&raw mut DAT_00524100)
            .cast::<u16>()
            .add(idx)
            .read_unaligned();
        let u_var2 = FUN_00425b70(0x500, v);
        let r = FUN_00403360(
            param_1 & 0xffff,
            (u_var2 & 0xffff).wrapping_add(0xa00).wrapping_mul(0x100),
        );
        (r == 1) as u8
    }
}
