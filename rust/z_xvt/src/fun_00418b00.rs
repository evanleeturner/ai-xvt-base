// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00523c60: [u16; 512];
    static mut DAT_007091a4: u32;
    static mut DAT_007329b8: u16;
    static mut DAT_006db794: u32;
    static mut DAT_0071b70c: u32;
    static mut DAT_007325d0: u32;
    static mut DAT_00703c30: u16;
    static mut DAT_006d4a50: u32;
    static mut DAT_006ffbec: u32;
    static mut DAT_0072f4e8: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00418b00(param_1: u32, param_2: i16, param_3: u16) {
    // SAFETY: all globals are the game's own memory; the table index is at
    // most 0x1ff, within the 512-entry table.
    unsafe {
        DAT_007091a4 = param_1 & 0xffff;
        DAT_007329b8 = 0x4000u32.wrapping_sub(param_2 as i32 as u32) as u16;
        let idx = (((param_3 as u32) >> 5 & 0x3fe) >> 1) as usize;
        let mut u_var3: u32 = DAT_007091a4
            .wrapping_mul(DAT_00523c60[idx] as u32)
            .wrapping_add(0x8000)
            >> 0x10;
        if (param_3 & 0x8000) != 0 {
            u_var3 = 0u32.wrapping_sub(u_var3);
        }
        let mut b_var2: u8 = 0;
        let mut u_var4: u32 = u_var3;
        if (u_var3 as i32) < 0 {
            u_var4 = 0u32.wrapping_sub(u_var3);
            b_var2 = 0x80;
        }
        let u_var1: u16 = 0x8000u32.wrapping_sub(param_2 as i32 as u32) as u16;
        let local_4: u32 = DAT_00523c60[(((u_var1 as u32) >> 5 & 0x3fe) >> 1) as usize] as u32;
        DAT_006db794 = (u_var4 >> 0x10)
            .wrapping_mul(local_4)
            .wrapping_add((u_var4 & 0xffff).wrapping_mul(local_4).wrapping_add(0x8000) >> 0x10);
        if ((b_var2 ^ (u_var1 >> 8) as u8) & 0x80) != 0 {
            DAT_006db794 = 0u32.wrapping_sub(DAT_006db794);
        }
        b_var2 = 0;
        if (u_var3 as i32) < 0 {
            u_var3 = 0u32.wrapping_sub(u_var3);
            b_var2 = 0x80;
        }
        let u_var5: u32 = DAT_00523c60[(((DAT_007329b8 as u32) >> 5 & 0x3fe) >> 1) as usize] as u32;
        DAT_0071b70c = (u_var3 >> 0x10)
            .wrapping_mul(u_var5)
            .wrapping_add((u_var3 & 0xffff).wrapping_mul(u_var5).wrapping_add(0x8000) >> 0x10);
        if ((b_var2 ^ (DAT_007329b8 >> 8) as u8) & 0x80) != 0 {
            DAT_0071b70c = 0u32.wrapping_sub(DAT_0071b70c);
        }
        DAT_00703c30 = param_3;
        let p: u16 = param_3.wrapping_add(0x4000);
        DAT_007325d0 = DAT_007091a4
            .wrapping_mul(DAT_00523c60[(((p as u32) >> 5 & 0x3fe) >> 1) as usize] as u32)
            .wrapping_add(0x8000)
            >> 0x10;
        if (p & 0x8000) != 0 {
            DAT_007325d0 = 0u32.wrapping_sub(DAT_007325d0);
        }
        DAT_006d4a50 = DAT_006db794;
        DAT_006ffbec = DAT_0071b70c;
        DAT_0072f4e8 = DAT_007325d0;
    }
}
