// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052747c: u16;
    static mut DAT_009a8074: u32;
    static mut DAT_00527e90: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0040b520();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00442250(param_1: u16, param_2: u32) {
    // SAFETY: the globals and the callee are the game's own; the output
    // pointer is a game buffer address, written byte by byte where the
    // original writes.
    unsafe {
        let mut u_var1: u16 = 0;
        let mut p3: *mut u8 = (u32::from(DAT_0052747c).wrapping_add(DAT_009a8074)) as *mut u8;
        if param_2 != 0 {
            loop {
                p3.write(1);
                let mut p4 = p3.wrapping_add(1);
                let mut u_var2 = param_1;
                if 0xff < param_1 {
                    p4.write(0);
                    p4 = p3.wrapping_add(2);
                    u_var2 = param_1.wrapping_sub(0xff);
                    if 0xff < u_var2 {
                        u_var2 = param_1.wrapping_sub(0x1ff);
                        p4.write(0);
                        p4 = p3.wrapping_add(3);
                    }
                }
                u_var1 = u_var1.wrapping_add(1);
                p4.write(u_var2 as u8);
                p3 = p4.wrapping_add(1);
                if !(u32::from(u_var1) < param_2) {
                    break;
                }
            }
        }
        if DAT_00527e90 != 0 {
            FUN_0040b520();
        }
    }
}
