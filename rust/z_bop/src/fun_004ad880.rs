// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: u32;
    static mut DAT_009a7394: u16;
    static mut DAT_0051c585: u8;
    static mut DAT_0051c587: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00425b70(param_1: u32, param_2: u16) -> u32;
    fn FUN_004ad9e0(param_1: i32, param_2: u32);
    fn FUN_004ada90(param_1: i32, param_2: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004ad880(param_1: i32, param_2: i32, param_3: i32, param_4: u16) {
    // SAFETY: all addresses are the game's own memory, read exactly where
    // the original reads them; the callees and globals belong to the game.
    unsafe {
        let slot = DAT_009a1fe8
            .wrapping_add(0x1f)
            .wrapping_add(param_1.wrapping_mul(0x23) as u32);
        let obj = (slot as *const i32).read_unaligned() as u32;
        let tmp_us = (obj.wrapping_add(0x79) as *const u16).read_unaligned();
        let mut u_var3 = (param_2 as u32).wrapping_sub(tmp_us as u32);
        if u_var3 != 0 {
            let idx = DAT_009a7394 as u32;
            let hi = ((DAT_009a7394 >> 8) as u32) << 16;
            if u_var3 < 0x8000 {
                let p = (&raw mut DAT_0051c585)
                    .byte_add(idx.wrapping_mul(0x101) as usize)
                    .cast::<u16>();
                let t = p.read_unaligned();
                let mut u_var1 = FUN_00425b70(hi | t as u32, 0x4000);
                u_var1 &= 0xffff;
                if u_var1 == 0 {
                    u_var1 = 1;
                }
                let ts = p.read_unaligned() as i16;
                let a = ts.wrapping_sub(u_var1 as i16) as u16 as u32;
                let u_var2 = FUN_00425b70(a, param_4);
                u_var1 = u_var1.wrapping_add(u_var2 & 0xffff);
                let slot2 = DAT_009a1fe8
                    .wrapping_add(0x1f)
                    .wrapping_add(param_1.wrapping_mul(0x23) as u32);
                let o1 = (slot2 as *const i32).read_unaligned() as u32;
                let o2 = (o1.wrapping_add(0xa9) as *const i32).read_unaligned() as u32;
                let flag = (o2.wrapping_add(0xbf) as *const i16).read_unaligned();
                if flag == 0 {
                    u_var1 = u_var1.wrapping_mul(3);
                }
                if u_var1 <= u_var3 {
                    u_var3 = u_var1;
                }
                FUN_004ad9e0(param_1, u_var3);
                return;
            }
            if param_3 == 1 {
                let p = (&raw mut DAT_0051c587)
                    .byte_add(idx.wrapping_mul(0x101) as usize)
                    .cast::<u16>();
                let t = p.read_unaligned();
                let mut u_var1 = FUN_00425b70(t as u32, 0x4000);
                u_var1 &= 0xffff;
                if u_var1 == 0 {
                    u_var1 = 1;
                }
                let ts = p.read_unaligned() as i16;
                let a = hi | (ts.wrapping_sub(u_var1 as i16) as u16 as u32);
                let b = (!param_4) as u16;
                let u_var2 = FUN_00425b70(a, b);
                u_var1 = u_var1.wrapping_add(u_var2 & 0xffff);
                let mut u_var2 = 0u32.wrapping_sub(u_var3);
                if u_var1 <= u_var2 {
                    u_var2 = u_var1;
                }
                FUN_004ada90(param_1, u_var2);
            }
        }
    }
}
