// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009d1320: i8;
    static mut DAT_009d6820: i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0046acb0() -> i32 {
    let mut i_var1: i32 = 0;
    // SAFETY: the globals are the game's own memory; only addresses are
    // taken and the bytes read lie in the game's table.
    unsafe {
        let mut pc_var2: *mut i8 = &raw mut DAT_009d1320;
        let end: *mut i8 = &raw mut DAT_009d6820;
        loop {
            if pc_var2.read() == 0 {
                return i_var1;
            }
            pc_var2 = pc_var2.byte_add(0x55);
            i_var1 = i_var1.wrapping_add(1);
            if pc_var2 >= end {
                break;
            }
        }
        i_var1
    }
}
