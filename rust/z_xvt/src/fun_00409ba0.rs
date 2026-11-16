// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00740a80: i32;
    static mut DAT_0073b7e0: i32;
    static mut DAT_006d4980: i32;
    static mut DAT_0073b430: i32;
    static mut DAT_006d4a44: i32;
    static mut DAT_0073b650: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00409ba0() {
    // SAFETY: the globals are the game's own memory; the writes cover the
    // same ranges, in the same order, as the original.
    unsafe {
        let mut p3: *mut i32 = &raw mut DAT_00740a80;
        let mut i1: i32 = 0x1a1;
        while i1 != 0 {
            p3.write_unaligned(0);
            p3 = p3.wrapping_add(1);
            i1 = i1.wrapping_sub(1);
        }
        p3.cast::<u16>().write_unaligned(0);

        let mut u2: u32 = 0;
        loop {
            p3 = (&raw mut DAT_0073b7e0)
                .cast::<u8>()
                .wrapping_add(u2 as usize)
                .cast::<i32>();
            u2 = u2.wrapping_add(0x61);
            i1 = 0x18;
            while i1 != 0 {
                p3.write_unaligned(0);
                p3 = p3.wrapping_add(1);
                i1 = i1.wrapping_sub(1);
            }
            p3.cast::<u8>().write(0);
            if u2 >= 0x246 {
                break;
            }
        }

        if DAT_006d4980 != 0 {
            p3 = &raw mut DAT_0073b430;
            i1 = DAT_006d4980;
            while i1 != 0 {
                p3.write_unaligned(0);
                p3 = p3.wrapping_add(1);
                i1 = i1.wrapping_sub(1);
            }
        }
        DAT_006d4a44 = 0;
        DAT_0073b650 = 0;
    }
}
