// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00645780: [u8; 0x180];
    static mut DAT_00645900: [u8; 0x180];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0049b2c0() {
    let mut i_var1: i32 = 0;
    loop {
        // SAFETY: the globals are the game's own memory; offsets stay below 0x180
        // and the writes go through raw addresses, unaligned-safe.
        unsafe {
            (&raw mut DAT_00645780)
                .cast::<u8>()
                .byte_add(i_var1 as usize)
                .cast::<u32>()
                .write_unaligned(0);
            (&raw mut DAT_00645900)
                .cast::<u8>()
                .byte_add(i_var1 as usize)
                .cast::<u32>()
                .write_unaligned(0);
        }
        i_var1 = i_var1.wrapping_add(0x30);
        if i_var1 >= 0x180 {
            break;
        }
    }
}
