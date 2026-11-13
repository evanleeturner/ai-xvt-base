// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a08104: *mut u8;
    static mut DAT_009a73b6: *mut u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004a45e0() {
    // SAFETY: the globals are the game's own pointers; the writes and the
    // read land at the offsets the original uses.
    unsafe {
        DAT_00a08104.byte_add(0x97).write(0);
        DAT_00a08104.byte_add(0x96).write(0);
        DAT_009a73b6
            .byte_add(0x2e)
            .cast::<u16>()
            .write_unaligned(0x4000);
        DAT_00a08104
            .byte_add(0x9e)
            .cast::<u16>()
            .write_unaligned(0xffff);
        DAT_00a08104.byte_add(0x9d).write(0);
        let val_0xd = DAT_00a08104.byte_add(0xd).cast::<u16>().read_unaligned();
        if val_0xd < 0x4000 {
            DAT_00a08104.byte_add(0x9c).write(2);
        } else if val_0xd < 0x4001 {
            DAT_00a08104.byte_add(0x9c).write(3);
        } else {
            DAT_00a08104.byte_add(0x9c).write(1);
        }
        DAT_00a08104.byte_add(0xa4).write(1);
        DAT_00a08104
            .byte_add(0xa5)
            .cast::<u16>()
            .write_unaligned(0xffff);
        DAT_009a73b6.byte_add(0x30).cast::<u16>().write_unaligned(0);
        DAT_00a08104.byte_add(0xab).write(0);
    }
}
