// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_007595d0: u32;
    static mut DAT_007595d4: u32;
    static mut DAT_007591d0: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004dbe80() -> u32 {
    // SAFETY: the globals are the game's own memory; the array is indexed
    // by the game's own counter exactly as the original does.
    unsafe {
        if DAT_007595d0 != DAT_007595d4 {
            return DAT_007595d4 & 0xffffff00;
        }
        let base = (&raw mut DAT_007591d0).cast::<u8>();
        (DAT_007595d4 & 0xffffff00) | u32::from(base.add(DAT_007595d4 as usize).read())
    }
}
