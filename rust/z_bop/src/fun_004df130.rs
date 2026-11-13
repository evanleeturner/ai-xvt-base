// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7fbf: i32;
    static mut DAT_00aa7fc3: i32;
    static mut DAT_00aa7fc7: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004df130(param_1: i32) {
    // SAFETY: the globals are the game's own; the table pointers they hold
    // address the game's record arrays, read and written where the original does.
    unsafe {
        if -1 < param_1 && param_1 < DAT_00aa7fc7 {
            if param_1 < DAT_00aa7fc7.wrapping_sub(1) {
                let mut i_var3: i32 = param_1.wrapping_mul(0x145);
                let mut i_var2: i32 = param_1;
                loop {
                    let mut p5 = DAT_00aa7fbf.wrapping_add(i_var3) as usize as *mut u8;
                    i_var2 = i_var2.wrapping_add(1);
                    i_var3 = i_var3.wrapping_add(0x145);
                    let mut p4 = (p5 as usize).wrapping_add(0x145) as *const u8;
                    let mut i_var1: i32 = 0x51;
                    while i_var1 != 0 {
                        let v = p4.cast::<i32>().read_unaligned();
                        p5.cast::<i32>().write_unaligned(v);
                        p4 = p4.byte_add(4);
                        p5 = p5.byte_add(4);
                        i_var1 -= 1;
                    }
                    *p5 = *p4;
                    if !(i_var2 < DAT_00aa7fc7.wrapping_sub(1)) {
                        break;
                    }
                }
            }
            let mut i_var3: i32 = 0;
            DAT_00aa7fc7 = DAT_00aa7fc7.wrapping_sub(1);
            loop {
                let addr = DAT_00aa7fc3.wrapping_add(i_var3) as usize as *mut i32;
                let i_var2 = addr.read_unaligned();
                if param_1 < i_var2 {
                    addr.write_unaligned(i_var2.wrapping_sub(1));
                }
                i_var3 = i_var3.wrapping_add(0xc);
                if !(i_var3 < 0x90) {
                    break;
                }
            }
        }
    }
}
