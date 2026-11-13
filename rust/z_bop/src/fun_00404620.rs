// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00404620(param_1: i32) -> u32 {
    // SAFETY: param_1 is the address of the game's own object; every read and
    // write is at the offsets the original uses, and the pointers read from it
    // are the game's own.
    unsafe {
        let p1 = param_1 as u32;
        let psvar1 = ((p1.wrapping_add(0x45e)) as *const u32).read_unaligned();
        if psvar1 == 0 {
            return ((p1.wrapping_add(9)) as *const u16).read_unaligned() as u32;
        }
        let a = ((p1.wrapping_add(0x3b0)) as *const i16).read_unaligned();
        let b = (psvar1 as *const i16).read_unaligned();
        if a != b {
            ((p1.wrapping_add(0x45e)) as *mut u32).write_unaligned(0);
            return (psvar1 & 0xFFFF0000)
                | (((p1.wrapping_add(9)) as *const u16).read_unaligned() as u32);
        }
        let inner = ((psvar1.wrapping_add(0x1f)) as *const i32).read_unaligned();
        if inner == 0 {
            return ((p1.wrapping_add(9)) as *const u16).read_unaligned() as u32;
        }
        let puvar2 = (((inner as u32).wrapping_add(0xad)) as *const u32).read_unaligned();
        if puvar2 != 0 {
            return (puvar2 & 0xFFFF0000) | ((puvar2 as *const u16).read_unaligned() as u32);
        }
        ((p1.wrapping_add(9)) as *const u16).read_unaligned() as u32
    }
}
