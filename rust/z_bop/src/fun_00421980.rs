// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0051c018: [u16; 0];
    static mut DAT_0051c248: [u16; 0];
    static mut DAT_0051c130: [u16; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00421980(param_1: u16, param_2: i16) -> u16 {
    let u_var1 = (param_1 >> 6) as usize;
    // SAFETY: the tables are the game's own arrays; the index is at most 0x3ff
    // and is read where the original reads it.
    unsafe {
        if param_2 == 0x5b {
            return (&raw mut DAT_0051c018).cast::<u16>().add(u_var1).read();
        }
        if param_2 == 0x6e {
            return (&raw mut DAT_0051c248).cast::<u16>().add(u_var1).read();
        }
        (&raw mut DAT_0051c130).cast::<u16>().add(u_var1).read()
    }
}
