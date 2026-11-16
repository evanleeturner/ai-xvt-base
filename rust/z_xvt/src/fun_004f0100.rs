// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_007579d0: [i32; 10];
    static mut DAT_00757a28: i32;
    static mut DAT_0075766c: i16;
    static mut DAT_007438d8: [u8; 1];
    static mut DAT_007579f8: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004f0100() {
    // SAFETY: the globals are the game's own memory; the pointers address
    // that memory exactly where the original reads and writes it.
    unsafe {
        let base = (&raw mut DAT_007579d0) as *mut i32;
        let mut p4 = base;
        let mut i2: i32 = 10;
        while i2 != 0 {
            p4.write_unaligned(0);
            p4 = p4.add(1);
            i2 = i2.wrapping_sub(1);
        }
        DAT_00757a28 = 0;
        if 0 < DAT_0075766c {
            let mut pb3 = (&raw mut DAT_007438d8) as *mut u8;
            let mut i2: i32 = DAT_0075766c as i32;
            loop {
                if pb3.byte_add(0xc).read() != 0 {
                    let slot = base.add(pb3.read() as usize);
                    slot.write_unaligned(slot.read_unaligned().wrapping_add(1));
                }
                pb3 = pb3.byte_add(0x562);
                i2 = i2.wrapping_sub(1);
                if i2 == 0 {
                    break;
                }
            }
        }
        let end = (&raw mut DAT_007579f8) as *mut i32;
        let mut p1 = base;
        loop {
            if p1.read_unaligned() != 0 {
                DAT_00757a28 = DAT_00757a28.wrapping_add(1);
            }
            p1 = p1.add(1);
            if !(p1 < end) {
                break;
            }
        }
    }
}
