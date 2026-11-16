// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: *mut u8;
    static mut DAT_009a8e18: *mut u8;
    static mut DAT_009e9640: i32;
    static mut DAT_00a00854: i32;
    static mut DAT_00a07cd4: u32;
    static mut DAT_00a07cd8: u32;
    static mut DAT_00a07cdc: u32;
    static mut DAT_005241f8: [u8; 1];
    static mut DAT_009d8c85: [u8; 1];
    static mut DAT_009d8c87: [u8; 1];
    static mut DAT_009d8c88: [u8; 1];
    static mut DAT_0051a44f: [u8; 1];
    static mut DAT_0051a2fc: [u8; 1];
    static mut DAT_0051a194: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00459750(param_1: i32) -> u16;
    fn FUN_00405860(param_1: u32) -> i16;
    fn FUN_00458e20(param_1: u32, param_2: i32);
    fn FUN_00425cb0() -> u8;
    fn FUN_00407910(param_1: u16);
}

#[inline(always)]
unsafe fn rd16(p: *const u8) -> u16 {
    // SAFETY: caller passes a valid game-memory address.
    unsafe { p.cast::<u16>().read_unaligned() }
}
#[inline(always)]
unsafe fn rd32(p: *const u8) -> u32 {
    // SAFETY: caller passes a valid game-memory address.
    unsafe { p.cast::<u32>().read_unaligned() }
}
#[inline(always)]
unsafe fn wr16(p: *mut u8, v: u16) {
    // SAFETY: caller passes a valid game-memory address.
    unsafe { p.cast::<u16>().write_unaligned(v) }
}
#[inline(always)]
unsafe fn wr32(p: *mut u8, v: u32) {
    // SAFETY: caller passes a valid game-memory address.
    unsafe { p.cast::<u32>().write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00406d10(param_1: u16, param_2: u16) -> u16 {
    // SAFETY: all globals and callees are the game's own memory; the
    // pointers and offsets are used exactly as the original does.
    unsafe {
        let p1 = param_1 as u32;
        let i_var1: u32 = p1.wrapping_mul(8).wrapping_sub(p1).wrapping_mul(5);
        let u_var6: u32 = *DAT_009a1fe8.byte_add(0x16usize.wrapping_add(i_var1 as usize)) as u32;
        let i_var7: u32 = u_var6.wrapping_mul(0x566);
        let idx7 = i_var7 as usize;
        let b_var3: u8 = *(&raw const DAT_005241f8)
            .cast::<u8>()
            .add(*(&raw const DAT_009d8c85).cast::<u8>().add(idx7) as usize);
        if b_var3 == 0 {
            return 0xffff;
        }
        let mut u_var4: u16 = FUN_00459750(7);
        if u_var4 == 0xffff {
            u_var4 = (DAT_009e9640 as i16).wrapping_add(0x80) as u16;
            while (u_var4 as i32) < DAT_00a00854 {
                let i_var8 = (u_var4 as u32).wrapping_mul(0x23);
                let ent = DAT_009a1fe8.byte_add(i_var8 as usize);
                let t = *ent.byte_add(3);
                if *(&raw const DAT_0051a44f).cast::<u8>().add(t as usize) == 0 {
                    let sub = rd32(ent.byte_add(0x1f)) as usize as *mut u8;
                    if *sub.byte_add(0x89) as i8
                        == *(&raw const DAT_009d8c88).cast::<u8>().add(idx7) as i8
                    {
                        break;
                    }
                }
                u_var4 = u_var4.wrapping_add(1);
            }
        }
        let u_var6: u32 = u_var4 as u32;
        if DAT_00a00854 as u32 == u_var6 {
            return 0xffff;
        }
        let i_var8: usize = u_var6.wrapping_mul(0x23) as usize;
        let mut sub = rd32(DAT_009a1fe8.byte_add(0x1f + i_var8)) as usize as *mut u8;
        *sub = 1;
        *DAT_009a1fe8.byte_add(2 + i_var8) = 7;
        *DAT_009a1fe8.byte_add(3 + i_var8) = b_var3;
        wr16(sub.byte_add(0x83), 1);
        wr16(sub.byte_add(0x85), param_1);
        *sub.byte_add(0x87) = *DAT_009a1fe8.byte_add(3usize.wrapping_add(i_var1 as usize));
        *sub.byte_add(0x88) = *(&raw const DAT_009d8c87).cast::<u8>().add(idx7);
        wr16(DAT_009a1fe8.byte_add(0x12 + i_var8), 0);
        let u_var9 = b_var3 as u32;
        wr16(DAT_009a1fe8.byte_add(0x14 + i_var8), 0);
        wr16(DAT_009a1fe8.byte_add(0x10 + i_var8), 0);
        let t16 = rd16(
            (&raw const DAT_0051a2fc)
                .cast::<u8>()
                .add(u_var9 as usize * 2 + 10),
        );
        wr16(sub.byte_add(0x79), t16);
        let t16 = rd16(sub.byte_add(0x79));
        let off = u_var6.wrapping_sub(DAT_009e9640 as u32).wrapping_mul(10) as usize;
        wr16(DAT_009a8e18.byte_add(8usize.wrapping_add(off)), t16);
        let t32 = rd32(
            (&raw const DAT_0051a194)
                .cast::<u8>()
                .add(u_var9 as usize * 4),
        );
        wr32(sub.byte_add(0x7d), t32);
        let s_var5 = FUN_00405860(u_var9);
        sub = rd32(DAT_009a1fe8.byte_add(0x1f + i_var8)) as usize as *mut u8;
        wr16(sub.byte_add(0x81), s_var5 as u16);
        FUN_00458e20(p1, 0);
        let u_var2 = DAT_00a07cd8;
        let i_var7b: u32 = DAT_00a07cd4.wrapping_add(0x180);
        sub = rd32(DAT_009a1fe8.byte_add(0x1f + i_var8)) as usize as *mut u8;
        wr32(sub.byte_add(6), DAT_00a07cdc);
        let mut ent = DAT_009a1fe8.byte_add(i_var8);
        sub = rd32(ent.byte_add(0x1f)) as usize as *mut u8;
        let t32 = rd32(sub.byte_add(6));
        wr32(ent.byte_add(4), t32);
        sub = rd32(DAT_009a1fe8.byte_add(0x1f + i_var8)) as usize as *mut u8;
        wr32(sub.byte_add(10), u_var2);
        ent = DAT_009a1fe8.byte_add(i_var8);
        sub = rd32(ent.byte_add(0x1f)) as usize as *mut u8;
        let t32 = rd32(sub.byte_add(10));
        wr32(ent.byte_add(8), t32);
        sub = rd32(DAT_009a1fe8.byte_add(0x1f + i_var8)) as usize as *mut u8;
        wr32(sub.byte_add(0xe), i_var7b);
        ent = DAT_009a1fe8.byte_add(i_var8);
        sub = rd32(ent.byte_add(0x1f)) as usize as *mut u8;
        let t32 = rd32(sub.byte_add(0xe));
        wr32(ent.byte_add(0xc), t32);
        let u_var10: u16 = u_var4.wrapping_sub(DAT_009e9640 as i16 as u16);
        let off2 = (u_var10 as u32).wrapping_mul(10) as usize;
        let b = FUN_00425cb0();
        *DAT_009a8e18.byte_add(1 + off2) = (b & 3).wrapping_add(3);
        wr16(DAT_009a8e18.byte_add(4 + off2), param_2);
        if param_2 == 0xffff || 0x7fff < param_2 {
            wr16(DAT_009a8e18.byte_add(6 + off2), 0);
        } else {
            let t16 = rd16(DAT_009a1fe8.byte_add((param_2 as u32).wrapping_mul(0x23) as usize));
            wr16(DAT_009a8e18.byte_add(6 + off2), t16);
        }
        *DAT_009a8e18.byte_add(off2) = 0xff;
        sub = rd32(DAT_009a1fe8.byte_add(0x1f + i_var8)) as usize as *mut u8;
        let p = DAT_009a8e18.byte_add(off2);
        wr32(sub.byte_add(0xa5), p as usize as u32);
        let cmp = rd32(
            DAT_009a1fe8.byte_add((param_2 as u32).wrapping_mul(0x23).wrapping_add(0x1b) as usize),
        ) as i32;
        if cmp != -1 {
            FUN_00407910(u_var10);
        }
        u_var4
    }
}
