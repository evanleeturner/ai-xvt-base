// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a73c0: u16;
    static mut DAT_009a73c2: u16;
    static mut DAT_009d8cde: [u8; 0];
    static mut DAT_009d8cdf: [u8; 0];
    static mut DAT_009d8ce0: [u8; 0];
    static mut DAT_009d8ce1: [u8; 0];
    static mut DAT_009d8ce2: [u8; 0];
    static mut DAT_009d8cd8: [u8; 0];
    static mut DAT_009d8cda: [u8; 0];
    static mut DAT_009d8cd9: [u8; 0];
    static mut DAT_009d8cdb: [u8; 0];
    static mut DAT_009d8cdc: [u8; 0];
    static mut unaff_EDI: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00433390(param_1: u32, param_2: u16, param_3: u16) -> u32;
}

#[inline(always)]
unsafe fn at(p: *mut u8, i: u32) -> u8 {
    // SAFETY: the caller passes the game's own table and an index the original computes.
    unsafe { p.add(i as usize).read() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00404450(param_1: u16) -> u32 {
    // SAFETY: the globals, tables and callee are the game's own memory and code;
    // reads and calls happen in the original's order.
    unsafe {
        let u_var6: u32 = (unaff_EDI & 0xFFFF0000) | (param_1 as u32);
        let idx: u32 = (DAT_009a73c2 as u32)
            .wrapping_mul(0x52)
            .wrapping_add((DAT_009a73c0 as u32).wrapping_mul(0x566));

        let a = at((&raw mut DAT_009d8cde).cast::<u8>(), idx) as u16;
        let b = at((&raw mut DAT_009d8cdf).cast::<u8>(), idx) as u16;
        let ret = FUN_00433390(u_var6, a, b);
        let mut u_var5: u16 = ret as u16;

        let a = at((&raw mut DAT_009d8ce0).cast::<u8>(), idx) as u16;
        let b = at((&raw mut DAT_009d8ce1).cast::<u8>(), idx) as u16;
        let ret = FUN_00433390(u_var6, a, b);
        let u_var4: u16 = ret as u16;

        if at((&raw mut DAT_009d8ce2).cast::<u8>(), idx) == 1 {
            u_var5 |= u_var4;
        } else {
            u_var5 &= u_var4;
        }

        let a = at((&raw mut DAT_009d8cd8).cast::<u8>(), idx) as u16;
        let b = (at((&raw mut DAT_009d8cda).cast::<u8>(), idx) & 0xFF) as u16;
        let ret = FUN_00433390(u_var6, a, b);
        let mut u_var4: u16 = ret as u16;

        let a = at((&raw mut DAT_009d8cd9).cast::<u8>(), idx) as u16;
        let b = at((&raw mut DAT_009d8cdb).cast::<u8>(), idx) as u16;
        let ret = FUN_00433390(u_var6, a, b);
        let u_var3: u16 = (ret >> 16) as u16;

        if at((&raw mut DAT_009d8cdc).cast::<u8>(), idx) == 1 {
            u_var4 |= u_var3;
        } else {
            u_var4 &= u_var3;
        }

        if u_var5 == 0 && u_var4 == 0 {
            return (u_var3 as u32) << 16;
        }
        ((u_var3 as u32) << 16) | 1
    }
}
