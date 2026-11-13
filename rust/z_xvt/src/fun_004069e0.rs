// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006db6ca: u16;
    static mut DAT_006db6ce: u16;
    static mut DAT_006da4e8: i16;
    static mut DAT_006da4e0: i16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004069e0(param_1: i32, param_2: *mut u16) {
    // SAFETY: the globals are the game's own; param_1 is an address of the
    // caller's data and param_2 points at its table, read where the original does.
    unsafe {
        let rd =
            |off: i32| -> u32 { (param_1.wrapping_add(off) as *const u16).read_unaligned() as u32 };
        let u_var2 = DAT_006db6ce;
        let u_var1 = DAT_006db6ca;
        if (DAT_006db6ca as i16) < 0 {
            DAT_006db6ca = 0u32.wrapping_sub(DAT_006db6ca as u32) as u16;
        }
        if (DAT_006db6ce as i16) < 0 {
            DAT_006db6ce = 0u32.wrapping_sub(DAT_006db6ce as u32) as u16;
        }
        let p0 = param_2.read_unaligned() as u32;
        let u_var6: u32 = ((p0
            .wrapping_mul(DAT_006db6ca as i16 as i32 as u32)
            .wrapping_add(0x80) as i32)
            >> 8) as u32
            & 0xffff;
        let mut i_var5: i32 = rd(6).wrapping_mul(u_var6) as i32;
        if ((rd(8) as u16 ^ u_var1) & 0x8000) != 0 {
            i_var5 = 0u32.wrapping_sub(i_var5 as u32) as i32;
        }
        let p205 = param_2.add(0x205).read_unaligned() as u32;
        let inner: u32 = ((p205
            .wrapping_mul(DAT_006db6ce as i16 as i32 as u32)
            .wrapping_add(0x80) as i32)
            >> 8) as u32;
        let u_var3: u32 = ((inner.wrapping_mul(p0).wrapping_add(0x80) as i32) >> 8) as u32 & 0xffff;
        let mut i_var4: i32 = rd(2).wrapping_mul(u_var3) as i32;
        if ((rd(4) as u16 ^ u_var2) & 0x8000) != 0 {
            i_var4 = 0u32.wrapping_sub(i_var4 as u32) as i32;
        }
        DAT_006da4e8 = ((i_var5 as u32)
            .wrapping_add(i_var4 as u32)
            .wrapping_add(0x8000)
            >> 0x10) as i16;
        i_var5 = rd(2).wrapping_mul(u_var6) as i32;
        if ((rd(4) as u16 ^ u_var1) & 0x8000) != 0 {
            i_var5 = 0u32.wrapping_sub(i_var5 as u32) as i32;
        }
        i_var4 = rd(6).wrapping_mul(u_var3) as i32;
        if ((rd(8) as u16 ^ u_var2) & 0x8000) == 0 {
            i_var4 = 0u32.wrapping_sub(i_var4 as u32) as i32;
        }
        i_var4 = (i_var5 as u32)
            .wrapping_add(0x8000)
            .wrapping_add(i_var4 as u32) as i32;
        DAT_006da4e0 = ((i_var4 as u32) >> 0x10) as i16;
        if i_var4 < 0 {
            DAT_006da4e0 = 0u32.wrapping_sub(DAT_006da4e0 as i32 as u32) as i16;
        }
        let p204 = param_2.add(0x204).read_unaligned() as u32;
        DAT_006da4e0 = (p204
            .wrapping_mul(DAT_006da4e0 as i32 as u32)
            .wrapping_add(0x80)
            >> 8) as i16;
        if i_var4 < 0 {
            DAT_006da4e0 = 0u32.wrapping_sub(DAT_006da4e0 as i32 as u32) as i16;
        }
    }
}
