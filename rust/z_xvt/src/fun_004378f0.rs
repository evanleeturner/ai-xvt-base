// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052550c: i32;
    static mut DAT_00525510: i32;
    static mut DAT_0052552c: i32;
    static mut DAT_0052f0f4: u32;
    static mut DAT_006da538: i32;
    static mut DAT_006db594: i32;
    static mut DAT_006da4ec: i32;
    static mut DAT_0070acc8: i32;
    static mut DAT_00732f3c: i32;
    static mut DAT_006d4994: i32;
    static mut DAT_0070acc4: u8;
    static mut DAT_0071ed00: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a6a40() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004378f0() {
    // SAFETY: the globals and the callee are the game's own memory and code;
    // they are read and written where the original does.
    unsafe {
        let i_var1 = FUN_004a6a40();
        DAT_0052550c = (i_var1 as u32).wrapping_mul(0x1e0) as i32;
        DAT_00525510 = 1;
        let p = (&raw mut DAT_0052f0f4).cast::<u8>();
        if DAT_0052552c == 0x13 {
            DAT_0052550c = 0x10000;
            DAT_00525510 = 1;
            DAT_006da538 = 0x140;
            DAT_006db594 = 0xf0;
            DAT_006da4ec = 0xf0;
            DAT_0070acc8 = FUN_004a6a40();
            DAT_00732f3c = 0x100;
            DAT_006d4994 = 0x80;
            DAT_0070acc4 = 8;
            p.add(2).write(0x33);
            p.add(3).write(0x32);
            DAT_0071ed00 = 0;
            return;
        }
        if DAT_0052552c != 0x101 {
            if DAT_0052552c != 0x112 {
                DAT_0052550c = 0x10000;
                DAT_006da538 = 0x140;
                DAT_006db594 = 0xf0;
                DAT_006da4ec = 0xf0;
                DAT_0070acc8 = FUN_004a6a40();
                DAT_00732f3c = 0x100;
                DAT_006d4994 = 0x80;
                DAT_0070acc4 = 8;
                p.add(2).write(0x33);
                p.add(3).write(0x32);
                DAT_0071ed00 = 0;
                return;
            }
            DAT_006db594 = 0x168;
            DAT_006da538 = 0x1e0;
            DAT_006da4ec = 0x1e0;
            DAT_0070acc8 = FUN_004a6a40();
            DAT_00732f3c = 0x200;
            DAT_006d4994 = 0x100;
            DAT_0070acc4 = 9;
            p.add(2).write(0x34);
            p.add(3).write(0x38);
            DAT_0071ed00 = 0;
            return;
        }
        DAT_006da538 = 0x280;
        DAT_006db594 = 0x1e0;
        DAT_006da4ec = 0x1e0;
        DAT_0070acc8 = FUN_004a6a40();
        DAT_00732f3c = 0x200;
        DAT_006d4994 = 0x100;
        DAT_0070acc4 = 9;
        p.add(2).write(0x36);
        p.add(3).write(0x34);
        DAT_0071ed00 = 0;
    }
}
