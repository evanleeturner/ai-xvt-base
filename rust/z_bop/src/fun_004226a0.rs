// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009fe7d4: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_00422780();
    fn FUN_00422b10();
    fn FUN_00422e90();
    fn FUN_00423200();
    fn FUN_004235d0();
    fn FUN_00423900();
    fn FUN_00423cd0();
    fn FUN_00424060();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004226a0() {
    // SAFETY: the global and the callees are the game's own; the selector
    // is read at byte offset 0x16 of the game's array, as the original does.
    unsafe {
        let sel = (&raw mut DAT_009fe7d4)
            .cast::<u8>()
            .byte_add(0x16)
            .cast::<u16>()
            .read_unaligned();
        match sel {
            0 => FUN_00422780(),
            1 => FUN_00422b10(),
            2 => FUN_00422e90(),
            3 => FUN_00423200(),
            4 => FUN_004235d0(),
            5 => FUN_00423900(),
            6 => FUN_00423cd0(),
            7 => FUN_00424060(),
            _ => {}
        }
    }
}
