// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009ecc34: i32;
    static mut DAT_009e9698: i8;
    static mut DAT_00a0a1d0: i8;
    static mut DAT_009e968a: i8;
    static mut DAT_009e9696: i8;
    static mut DAT_00a08374: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00440760(param_1: i32, param_2: u32);
    fn FUN_0042ec80(param_1: u32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_004abe50();
    fn FUN_004aba50();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0043c900() {
    // SAFETY: the globals and callees are the game's own; the indexed
    // reads and writes stay at the offsets the original uses.
    unsafe {
        let i_var1 = DAT_009ecc34.wrapping_mul(0x5bd) as isize;
        let u_var2: u32;
        let p98 = (&raw mut DAT_009e9698).cast::<u8>().wrapping_offset(i_var1);
        let p96 = (&raw mut DAT_009e9696).cast::<u8>().wrapping_offset(i_var1);
        if p98.read() == 0 {
            u_var2 = if DAT_00a0a1d0 == 0 { 0 } else { 4 };
        } else {
            let s = (&raw mut DAT_009e968a)
                .cast::<u8>()
                .wrapping_offset(i_var1)
                .cast::<i16>()
                .read_unaligned();
            if s == -1 {
                u_var2 = 1;
            } else {
                u_var2 = (p96.read() as u32).wrapping_add(1) as u16 as u32;
            }
            DAT_00a0a1d0 = 1;
            if p96.read() != 2 {
                DAT_00a0a1d0 = 0;
            }
        }
        FUN_00440760(DAT_00a08374.wrapping_add(0x34), u_var2);
        FUN_004abe50();
        FUN_0042ec80(u_var2);
        FUN_004aba50();
    }
}
