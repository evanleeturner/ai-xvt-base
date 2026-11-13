// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0049c2e0(param_1: u32, param_2: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0049c2b0(param_1: u32) {
    // SAFETY: the global and callee are the game's own; the address computed
    // from the table base points at the game's byte array entry.
    unsafe {
        let idx = param_1 & 0xffff;
        let addr = idx
            .wrapping_mul(0x23)
            .wrapping_add(3)
            .wrapping_add(DAT_006d4988);
        let b = (addr as *const u8).read();
        FUN_0049c2e0(idx, b as u32);
    }
}
