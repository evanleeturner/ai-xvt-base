// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00759af4: i32;
    static mut DAT_00759af8: i32;
    static mut DAT_00759afc: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004dedc0(param_1: i32) {
    // SAFETY: the globals are the game's own; the table pointers they hold
    // address the game's record array and counter table, accessed exactly
    // as the original does.
    unsafe {
        if -1 < param_1 && param_1 < DAT_00759afc {
            if param_1 < DAT_00759afc.wrapping_sub(1) {
                let mut i_var3: i32 = param_1.wrapping_mul(0x145);
                let mut i_var2: i32 = param_1;
                loop {
                    let mut puvar5 = DAT_00759af4.wrapping_add(i_var3) as *mut i32;
                    i_var2 = i_var2.wrapping_add(1);
                    i_var3 = i_var3.wrapping_add(0x145);
                    let mut puvar4 = (puvar5 as i32).wrapping_add(0x145) as *const i32;
                    let mut i_var1: i32 = 0x51;
                    while i_var1 != 0 {
                        puvar5.write_unaligned(puvar4.read_unaligned());
                        puvar4 = puvar4.add(1);
                        puvar5 = puvar5.add(1);
                        i_var1 = i_var1.wrapping_sub(1);
                    }
                    (puvar5 as *mut u8).write(*(puvar4 as *const u8));
                    if !(i_var2 < DAT_00759afc.wrapping_sub(1)) {
                        break;
                    }
                }
            }
            let mut i_var3: i32 = 0;
            DAT_00759afc = DAT_00759afc.wrapping_sub(1);
            loop {
                let p = DAT_00759af8.wrapping_add(i_var3) as *mut i32;
                let i_var2 = p.read_unaligned();
                if param_1 < i_var2 {
                    p.write_unaligned(i_var2.wrapping_sub(1));
                }
                i_var3 = i_var3.wrapping_add(0xc);
                if !(i_var3 < 0x90) {
                    break;
                }
            }
        }
    }
}
