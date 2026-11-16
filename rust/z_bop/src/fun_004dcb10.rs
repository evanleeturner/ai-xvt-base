// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7a98: u32;
    static mut DAT_00aa7a9c: u32;
    static mut DAT_00aa7698: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004dcb10() -> u32 {
    // SAFETY: the globals are the game's own memory; the index is read
    // from the game's own cursor exactly as the original does.
    unsafe {
        let idx = DAT_00aa7a9c;
        if DAT_00aa7a98 == idx {
            return idx & 0xffffff00;
        }
        let p = (&raw mut DAT_00aa7698).cast::<u8>();
        (idx & 0xffffff00) | u32::from(p.add(idx as usize).read())
    }
}
