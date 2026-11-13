// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005256b0: u16;
    static mut DAT_005256b4: u16;
    static mut DAT_005256b8: u16;
    static mut DAT_005256bc: u16;
    static mut DAT_005256c0: u16;
    static mut DAT_005256c4: u16;
    static mut DAT_00525670: [u8; 0];
    static mut DAT_00525630: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00478470() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00421d20() {
    // SAFETY: the globals and the callee are the game's own memory and code;
    // the arrays are written within their 0x16 entries as the original does.
    unsafe {
        DAT_005256b0 = 4;
        DAT_005256b4 = 4;
        DAT_005256c0 = 4;
        DAT_005256c4 = 4;
        DAT_005256b8 = 3;
        DAT_005256bc = 3;
        let mut u_var5: u16 = 0;
        while u_var5 < 0x16 {
            let mut b_var1: u8;
            loop {
                b_var1 = FUN_00478470() as u8;
                b_var1 = (b_var1 & 0xe).wrapping_add(4);
                if !(0xc < b_var1) {
                    break;
                }
            }
            let mut b_var2: u8;
            loop {
                b_var2 = FUN_00478470() as u8;
                b_var2 = (b_var2 & 0xe).wrapping_add(4);
                if !(0xc < b_var2) {
                    break;
                }
            }
            (&raw mut DAT_00525670)
                .cast::<u8>()
                .add(u_var5 as usize)
                .write(b_var2.wrapping_mul(0x10).wrapping_add(b_var1));
            u_var5 = u_var5.wrapping_add(1);
        }
        let mut u_var4: u32 = 0;
        loop {
            let mut u_var5: u16 = FUN_00478470() as u16;
            u_var5 &= 0x1f;
            let c_var3: i8;
            if u_var5 < 3 {
                (&raw mut DAT_00525630)
                    .cast::<i8>()
                    .add(u_var4 as usize)
                    .write(0x75);
            } else {
                if u_var5 < 0xc {
                    c_var3 = ((u_var5 / 3) as i8).wrapping_add(0x75);
                } else {
                    c_var3 = ((u_var5 as u8) & 1).wrapping_add(0x7d) as i8;
                }
                (&raw mut DAT_00525630)
                    .cast::<i8>()
                    .add(u_var4 as usize)
                    .write(c_var3);
            }
            u_var5 = (u_var4 as u16).wrapping_add(1);
            u_var4 = u_var5 as u32;
            if !(u_var5 < 0x16) {
                break;
            }
        }
    }
}
