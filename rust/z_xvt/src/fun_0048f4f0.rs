// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00625734: [u32; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0048f4f0(param_1: u32) -> u32 {
    // SAFETY: DAT_00625734 is the game's own table; the index is masked to 16 bits.
    unsafe {
        (&raw mut DAT_00625734)
            .cast::<u32>()
            .add((param_1 & 0xffff) as usize)
            .read()
    }
}
