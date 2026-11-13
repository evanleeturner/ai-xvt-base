// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00664c80: *mut u32;
    static mut DAT_00664c84: *mut u32;
    static mut DAT_00528ca0: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004b1d70() -> u32 {
    // SAFETY: the globals are the game's own; the buffer pointer in
    // DAT_00664c80 points at writable memory the original writes to.
    unsafe {
        DAT_00664c84 = DAT_00664c80;
        DAT_00664c80.cast::<u8>().write(9);
        DAT_00664c80.cast::<u8>().byte_add(1).write(0x10);
        DAT_00664c80
            .cast::<u8>()
            .byte_add(2)
            .cast::<u16>()
            .write_unaligned(1);
        DAT_00664c80 = DAT_00664c80.cast::<u8>().byte_add(4).cast::<u32>();
        DAT_00664c80.write_unaligned(2);
        DAT_00664c80.add(1).cast::<u16>().write_unaligned(0);
        DAT_00664c80
            .cast::<u8>()
            .byte_add(6)
            .cast::<u16>()
            .write_unaligned(0);
        DAT_00664c80.add(2).write_unaligned(DAT_00528ca0);
        DAT_00664c80.add(3).write_unaligned(0);
        DAT_00664c80 = DAT_00664c80.add(4);
    }
    1
}
