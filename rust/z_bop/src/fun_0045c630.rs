// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a73c0: u16;
    static mut DAT_009d8cd8: [i8; 1];
    static mut DAT_009d8cd9: [i8; 1];
    static mut DAT_009d8cda: [i8; 1];
    static mut DAT_009d8cdb: [i8; 1];
    static mut DAT_009d8cdc: [i8; 1];
    static mut DAT_009d8cde: [i8; 1];
    static mut DAT_009d8cdf: [i8; 1];
    static mut DAT_009d8ce0: [i8; 1];
    static mut DAT_009d8ce1: [i8; 1];
    static mut DAT_009d8ce2: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0045c720(a: u16, b: u16, c: u16, d: u16, e: u16) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0045c630(param_1: u32) {
    // SAFETY: the globals and the callee are the game's own; the arrays are
    // indexed at the offsets the original computes.
    unsafe {
        let local_1: u32 = (param_1 & 0xffff).wrapping_mul(0x52);
        let mut local_2: u32 = local_1.wrapping_add((DAT_009a73c0 as u32).wrapping_mul(0x566));
        let i = local_2 as usize;
        let a = (&raw mut DAT_009d8cde).cast::<i8>().add(i).read() as u16;
        let b = (&raw mut DAT_009d8cdf).cast::<i8>().add(i).read() as u16;
        let c = (&raw mut DAT_009d8ce2).cast::<i8>().add(i).read() as u16;
        let d = (&raw mut DAT_009d8ce0).cast::<i8>().add(i).read() as u16;
        let e = (&raw mut DAT_009d8ce1).cast::<i8>().add(i).read() as u16;
        let u_var3 = FUN_0045c720(a, b, c, d, e);
        if (u_var3 as u16 as i16) == -1 {
            local_2 = local_1.wrapping_add((DAT_009a73c0 as u32).wrapping_mul(0x566));
            let i = local_2 as usize;
            let a = (&raw mut DAT_009d8cd8).cast::<i8>().add(i).read() as u16;
            let b = (&raw mut DAT_009d8cda).cast::<i8>().add(i).read() as u16;
            let c = (&raw mut DAT_009d8cdc).cast::<i8>().add(i).read() as u16;
            let d = (&raw mut DAT_009d8cd9).cast::<i8>().add(i).read() as u16;
            let e = (&raw mut DAT_009d8cdb).cast::<i8>().add(i).read() as u16;
            FUN_0045c720(a, b, c, d, e);
        }
    }
}
