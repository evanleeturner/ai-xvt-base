// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0064ee60: *mut u32;
    static mut DAT_0064ef40: *mut u8;
    static mut DAT_0052f204: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004b5ce0() -> u32 {
    // SAFETY: the globals are the game's own memory; DAT_0064ee60 points at
    // the game's output buffer, written at the offsets the original uses.
    unsafe {
        DAT_0064ef40 = DAT_0064ee60 as *mut u8;
        (DAT_0064ee60 as *mut u8).write(9);
        (DAT_0064ee60 as *mut u8).byte_add(1).write(0x10);
        (DAT_0064ee60 as *mut u8)
            .byte_add(2)
            .cast::<u16>()
            .write_unaligned(1);
        DAT_0064ee60 = (DAT_0064ee60 as u32).wrapping_add(4) as *mut u32;
        DAT_0064ee60.write_unaligned(2);
        DAT_0064ee60.add(1).cast::<u16>().write_unaligned(0);
        (DAT_0064ee60 as *mut u8)
            .byte_add(6)
            .cast::<u16>()
            .write_unaligned(0);
        DAT_0064ee60.add(2).write_unaligned(DAT_0052f204);
        DAT_0064ee60.add(3).write_unaligned(0);
        DAT_0064ee60 = DAT_0064ee60.add(4);
    }
    1
}
