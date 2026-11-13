// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009fd394: i32;
    static mut DAT_009e9680: i8;
    static mut DAT_009ec468: i8;
    static mut DAT_009d6940: i8;
    static mut DAT_009ecc34: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00448d00() -> i8 {
    // SAFETY: the globals are the game's own memory; the pointer walks the
    // game's record table in 0x5bd-byte steps as the original does.
    unsafe {
        DAT_009fd394 = 0;
        let mut i_var2: i32 = 0;
        let mut pc_var3: *const i8 = &raw mut DAT_009e9680 as *const i8;
        let end: *const i8 = &raw mut DAT_009ec468 as *const i8;
        loop {
            let c_var1 = pc_var3.read();
            if c_var1 == 1 || c_var1 == 2 {
                DAT_009fd394 = DAT_009fd394.wrapping_add(1);
            }
            if c_var1 == 1 {
                i_var2 = i_var2.wrapping_add(1);
            }
            pc_var3 = pc_var3.wrapping_byte_add(0x5bd);
            if !(pc_var3 < end) {
                break;
            }
        }
        if i_var2 == 0 {
            DAT_009d6940 = 1;
        }
        let base = &raw mut DAT_009e9680 as *const i8;
        if base
            .wrapping_byte_offset((DAT_009ecc34.wrapping_mul(0x5bd)) as isize)
            .read()
            == 0
        {
            DAT_009d6940 = 1;
        }
        DAT_009d6940
    }
}
