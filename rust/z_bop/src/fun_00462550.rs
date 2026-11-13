// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a73b6: u32;
    static mut DAT_009a73c0: u16;
    static mut DAT_009a73c2: u16;
    static mut DAT_009a1fdc: u8;
    static mut DAT_009d8cde: [u8; 0];
    static mut DAT_009d8cdf: [u8; 0];
    static mut DAT_009d8ce2: [u8; 0];
    static mut DAT_009d8ce0: [u8; 0];
    static mut DAT_009d8ce1: [u8; 0];
    static mut DAT_009d8cd8: [u8; 0];
    static mut DAT_009d8cda: [u8; 0];
    static mut DAT_009d8cdc: [u8; 0];
    static mut DAT_009d8cd9: [u8; 0];
    static mut DAT_009d8cdb: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00462690(a: u16, b: u16, c: u16, d: u16, e: u16) -> u16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00462550() -> u32 {
    // SAFETY: the globals and callee are the game's own memory; DAT_009a73b6
    // holds the address of a live object and the tables are indexed as the
    // original does.
    unsafe {
        ((DAT_009a73b6 as usize + 0x2d) as *mut u8).write(0xff);
        let i1 = (DAT_009a73c2 as u32)
            .wrapping_mul(0x52)
            .wrapping_add((DAT_009a73c0 as u32).wrapping_mul(0x566)) as usize;
        let u3 = FUN_00462690(
            (&raw mut DAT_009d8cde as *mut u8).add(i1).read() as u16,
            (&raw mut DAT_009d8cdf as *mut u8).add(i1).read() as u16,
            (&raw mut DAT_009d8ce2 as *mut u8).add(i1).read() as u16,
            (&raw mut DAT_009d8ce0 as *mut u8).add(i1).read() as u16,
            (&raw mut DAT_009d8ce1 as *mut u8).add(i1).read() as u16,
        );
        let u2 = DAT_009a73b6;
        if u3 != 0xffff {
            ((DAT_009a73b6 as usize + 0x2d) as *mut u8).write(DAT_009a1fdc);
            return u2 & 0xffff0000;
        }
        let i1 = (DAT_009a73c2 as u32)
            .wrapping_mul(0x52)
            .wrapping_add((DAT_009a73c0 as u32).wrapping_mul(0x566)) as usize;
        let u3 = FUN_00462690(
            (&raw mut DAT_009d8cd8 as *mut u8).add(i1).read() as u16,
            (&raw mut DAT_009d8cda as *mut u8).add(i1).read() as u16,
            (&raw mut DAT_009d8cdc as *mut u8).add(i1).read() as u16,
            (&raw mut DAT_009d8cd9 as *mut u8).add(i1).read() as u16,
            (&raw mut DAT_009d8cdb as *mut u8).add(i1).read() as u16,
        );
        let u2 = DAT_009a73b6;
        if u3 != 0xffff {
            ((DAT_009a73b6 as usize + 0x2d) as *mut u8).write(DAT_009a1fdc);
            return u2 & 0xffff0000;
        }
        0
    }
}
