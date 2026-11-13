// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a73c0: u16;
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
    fn FUN_0045d6e0(a: u16, b: u16, c: u16, d: u16, e: u16) -> u16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0045d5f0(param_1: u32) {
    // SAFETY: the globals and the callee are the game's own; the byte tables
    // are indexed exactly as the original does.
    unsafe {
        let i_var2: u32 = (param_1 & 0xffff).wrapping_mul(0x52);
        let i_var1: u32 = i_var2.wrapping_add((DAT_009a73c0 as u32).wrapping_mul(0x566));
        let i = i_var1 as usize;
        let a = (&raw mut DAT_009d8cde).cast::<u8>().add(i).read() as u16;
        let b = (&raw mut DAT_009d8cdf).cast::<u8>().add(i).read() as u16;
        let c = (&raw mut DAT_009d8ce2).cast::<u8>().add(i).read() as u16;
        let d = (&raw mut DAT_009d8ce0).cast::<u8>().add(i).read() as u16;
        let e = (&raw mut DAT_009d8ce1).cast::<u8>().add(i).read() as u16;
        let u_var3 = FUN_0045d6e0(a, b, c, d, e);
        if u_var3 == 0xffff {
            let i_var1: u32 = i_var2.wrapping_add((DAT_009a73c0 as u32).wrapping_mul(0x566));
            let i = i_var1 as usize;
            let a = (&raw mut DAT_009d8cd8).cast::<u8>().add(i).read() as u16;
            let b = (&raw mut DAT_009d8cda).cast::<u8>().add(i).read() as u16;
            let c = (&raw mut DAT_009d8cdc).cast::<u8>().add(i).read() as u16;
            let d = (&raw mut DAT_009d8cd9).cast::<u8>().add(i).read() as u16;
            let e = (&raw mut DAT_009d8cdb).cast::<u8>().add(i).read() as u16;
            FUN_0045d6e0(a, b, c, d, e);
        }
    }
}
