// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_007595d0: u32;
    static mut DAT_007595d4: u32;
    static mut DAT_007591d0: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004dbe40() -> u32 {
    // SAFETY: the globals are the game's own memory; the buffer is read at
    // the index the original uses.
    unsafe {
        if DAT_007595d0 == DAT_007595d4 {
            return DAT_007595d4 & 0xffffff00;
        }
        let i_var3 = DAT_007595d4.wrapping_add(1);
        let p_var1 = (&raw mut DAT_007591d0)
            .cast::<u8>()
            .add(DAT_007595d4 as usize);
        let u_var2 = DAT_007595d4 >> 8;
        DAT_007595d4 = i_var3;
        if i_var3 == 0x400 {
            DAT_007595d4 = 0;
        }
        (u_var2 << 8) | u32::from(p_var1.read())
    }
}
