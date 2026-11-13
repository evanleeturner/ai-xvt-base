// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d9d50: i16;
    static mut DAT_006d4988: i32;
    static mut DAT_006d9d56: i32;
    static mut DAT_00703ca0: i32;
    static mut DAT_00525028: i8;
    static mut DAT_00732e54: i32;
    static mut DAT_00525aae: i16;
    static mut DAT_006db580: i32;
    static mut DAT_0070a1d2: i16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0042a140(param_1: u32, param_2: u32);
    fn FUN_004187e0(param_1: u32, param_2: u32) -> u32;
    fn FUN_0042b250(param_1: i32) -> u32;
    fn FUN_00478320(param_1: u32, param_2: u32) -> u32;
}

unsafe fn rd32(a: i32) -> i32 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as usize as *const i32).read_unaligned() }
}

unsafe fn rd16(a: i32) -> i16 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as usize as *const i16).read_unaligned() }
}

unsafe fn rd8(a: i32) -> u8 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as usize as *const u8).read() }
}

unsafe fn wr32(a: i32, v: i32) {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as usize as *mut i32).write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00497f00(param_1: i32) {
    // SAFETY: all globals and pointers are the game's own memory, accessed
    // at the offsets the original uses.
    unsafe {
        let mut u_var9: u32 = DAT_006d9d50 as i32 as u32;
        let i_var6: i32 = param_1.wrapping_mul(0x23);
        if rd16(rd32(DAT_006d4988.wrapping_add(0x1f).wrapping_add(i_var6)).wrapping_add(0x79)) == 0
        {
            u_var9 = 0;
        } else {
            FUN_0042a140(u_var9, rd16(DAT_006d9d56.wrapping_add(0x18)) as u16 as u32);
            let mut b_var17: bool = false;
            let mut i_var10: i32 = 0xf;
            let mut pc15: *const i8 = (&raw mut DAT_00703ca0 as *const u8)
                .wrapping_add((rd8(DAT_006d9d56.wrapping_add(0xb)) as usize).wrapping_mul(0x55))
                as *const i8;
            let mut pc16: *const i8 = &raw mut DAT_00525028 as *const i8;
            loop {
                if i_var10 == 0 {
                    break;
                }
                i_var10 = i_var10.wrapping_sub(1);
                b_var17 = *pc15 == *pc16;
                pc15 = pc15.wrapping_add(1);
                pc16 = pc16.wrapping_add(1);
                if !b_var17 {
                    break;
                }
            }
            let u_var8: u32;
            if b_var17 && rd8(DAT_006d9d56.wrapping_add(0x1e)) == 1 {
                u_var8 = 0x8d;
            } else {
                let v = rd8(DAT_00732e54.wrapping_add(0x214));
                if v < 0x8a {
                    u_var8 = 0x89;
                } else {
                    u_var8 = v as u32;
                }
            }
            let mut s_var14: i16 = (&raw mut DAT_00525aae)
                .wrapping_add(u_var8.wrapping_mul(2) as usize)
                .read_unaligned();
            let i_var10b: i32 = (u_var9.wrapping_mul(0x23) as i32).wrapping_add(DAT_006d4988);
            let s_var1: i16 = rd16(rd32(i_var10b.wrapping_add(0x1f)).wrapping_add(0x79));
            let mut u_var13: u16 =
                rd16(rd32(DAT_006d4988.wrapping_add(i_var6).wrapping_add(0x1f)).wrapping_add(0x79))
                    as u16;
            let mut u_var12: u16 =
                (rd16(DAT_006d4988.wrapping_add(i_var6).wrapping_add(0x10)) as i32)
                    .wrapping_sub(rd16(i_var10b.wrapping_add(0x10)) as i32) as u16;
            if 0x7fff < u_var12 {
                u_var12 = u_var12.wrapping_neg();
            }
            let s_var7: i16;
            if u_var12 < 0x4000 {
                u_var9 = FUN_004187e0(u_var13 as u32, u_var12 as u32);
                s_var7 = (u_var9 as i16).wrapping_neg();
            } else {
                u_var9 = FUN_004187e0(u_var13 as u32, u_var12 as u32);
                s_var7 = u_var9 as i16;
            }
            u_var13 = (s_var14 as i32)
                .wrapping_add(s_var1 as i32)
                .wrapping_add(s_var7 as i32) as u16;
            u_var13 = (u_var13 as u32)
                .wrapping_mul(0x12)
                .wrapping_add(u_var13 as u32 / 5) as u16;
            if u_var13 == 0 {
                u_var13 = 0x13;
            }
            let i_var10c: i32 = DAT_006db580 / (u_var13 as i32);
            s_var14 = (i_var10c as i16 as i32).wrapping_mul(DAT_0070a1d2 as i32) as i16;
            u_var9 = FUN_0042b250(DAT_00732e54);
            u_var9 = FUN_00478320(
                (((((i_var10c as u32) >> 16) as i16) as i32 as u32) << 16)
                    | (s_var14 as i32 as u32),
                (u_var9 as u16) as u32,
            );
        }
        let i_var11: i32 = DAT_006d4988.wrapping_add(i_var6);
        let i_var10: i32 = rd32(i_var11.wrapping_add(0x1f));
        let i_var2: i32 = rd32(i_var11.wrapping_add(8));
        u_var9 &= 0xffff;
        let i_var3: i32 = rd32(i_var10.wrapping_add(0xe));
        let i_var4: i32 = rd32(i_var10.wrapping_add(10));
        let i_var5: i32 = rd32(i_var11.wrapping_add(0xc));
        let m = u_var9 as i32;
        wr32(
            DAT_006d9d56.wrapping_add(0x1f),
            rd32(i_var11.wrapping_add(4))
                .wrapping_sub(rd32(i_var10.wrapping_add(6)))
                .wrapping_mul(m)
                .wrapping_add(rd32(i_var11.wrapping_add(4))),
        );
        wr32(
            DAT_006d9d56.wrapping_add(0x23),
            rd32(DAT_006d4988.wrapping_add(8).wrapping_add(i_var6))
                .wrapping_add(m.wrapping_mul(i_var2.wrapping_sub(i_var4))),
        );
        wr32(
            DAT_006d9d56.wrapping_add(0x27),
            rd32(DAT_006d4988.wrapping_add(0xc).wrapping_add(i_var6))
                .wrapping_add(m.wrapping_mul(i_var5.wrapping_sub(i_var3))),
        );
    }
}
