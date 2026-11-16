// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: *mut u8;
    static mut DAT_00709310: [u32; 0];
    static mut DAT_00709374: [u16; 0];
    static mut DAT_0070ad48: [u8; 0];
    static mut DAT_0070b252: [u8; 0];
    static mut DAT_0071b738: [u8; 0];
    static mut DAT_0071b740: [u8; 0];
    static mut DAT_0071b79d: [u8; 0];
    static mut DAT_0071b7a1: [u8; 0];
    static mut DAT_0071b9c1: [u8; 0];
    static mut DAT_0071b9c3: [u8; 0];
    static mut DAT_0071b9c5: [u8; 0];
    static mut DAT_0071b9c7: [u8; 0];
    static mut DAT_0071e520: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a1980(param_1: u32) -> i32;
    fn FUN_004a01a0(param_1: u32, param_2: u32);
    fn FUN_00478420(param_1: u32, param_2: u32) -> u32;
}

unsafe fn r32(p: *const u8) -> u32 {
    // SAFETY: caller guarantees p addresses readable game memory.
    unsafe { p.cast::<u32>().read_unaligned() }
}

unsafe fn r16(p: *const u8) -> u16 {
    // SAFETY: caller guarantees p addresses readable game memory.
    unsafe { p.cast::<u16>().read_unaligned() }
}

unsafe fn w32(p: *mut u8, v: u32) {
    // SAFETY: caller guarantees p addresses writable game memory.
    unsafe { p.cast::<u32>().write_unaligned(v) }
}

unsafe fn w16(p: *mut u8, v: u16) {
    // SAFETY: caller guarantees p addresses writable game memory.
    unsafe { p.cast::<u16>().write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004a10d0(param_1: u32, param_2: i32) -> u32 {
    // SAFETY: all globals and pointers are the game's own memory, accessed
    // at the offsets the original uses.
    unsafe {
        let mut local_4: u32 = 0xffff_ffff;
        let mut local_8: i32;
        let i_var5_first = FUN_004a1980(param_1);
        let mut u_var6: u32 = (i_var5_first / 2) as u32;
        let p: *mut u8 = DAT_006d4988.wrapping_add(param_1.wrapping_mul(0x23) as usize);

        let b0 = (&raw mut DAT_0070ad48).cast::<u8>();
        let mut idx =
            *b0.wrapping_add((p.add(0x16).read() as u32).wrapping_mul(0x566) as usize) as usize;
        let a310 = (&raw mut DAT_00709310).cast::<u32>();
        *a310.add(idx) = (*a310.add(idx)).wrapping_sub(u_var6);
        idx = *b0.wrapping_add((p.add(0x16).read() as u32).wrapping_mul(0x566) as usize) as usize;
        let a374 = (&raw mut DAT_00709374).cast::<u16>();
        *a374.add(idx) = (*a374.add(idx)).wrapping_add(1);

        let mut i_var5 = r32(p.add(0x1b)) as i32;
        if i_var5 == -1 {
            let q = (&raw mut DAT_0070b252)
                .cast::<u8>()
                .wrapping_add((p.add(0x16).read() as u32).wrapping_mul(0x566) as usize);
            i_var5 = r32(q) as i32;
            if i_var5 == -1 {
                return u_var6;
            }
            let t = (&raw mut DAT_0071b740)
                .cast::<u8>()
                .wrapping_add((i_var5 as u32).wrapping_mul(0x5bd) as usize);
            if t.read() == 0 {
                return u_var6;
            }
        }
        let p_inner = r32(p.add(0x1f)) as usize as *mut u8;
        let i_var12 = r32(p_inner.add(0xa7)) as usize as *mut u8;
        let i_var4: u32 = (i_var5 as u32).wrapping_mul(0x5bd);
        let o = i_var4 as usize;
        let q79d = (&raw mut DAT_0071b79d).cast::<u8>().wrapping_add(o);
        w32(q79d, r32(q79d).wrapping_sub(u_var6));

        if param_2 != 0 {
            let mut tmps = r16(i_var12.add(0xc3));
            let tmps2 = r16(i_var12.add(199));
            if tmps < tmps2 {
                u_var6 = (tmps2 as u32).wrapping_sub(tmps as u32);
            } else {
                u_var6 = 0;
            }
            tmps = r16(i_var12.add(0x20b));
            if 0 < (tmps as i16) {
                u_var6 = u_var6.wrapping_add(tmps as u32);
            }
            tmps = r16(i_var12.add(0x20d));
            if 0 < (tmps as i16) {
                u_var6 = u_var6.wrapping_add(tmps as u32);
            }
            let mut u_var10: u32 = 0;
            let mut pv = i_var12.add(0x1a1);
            for _ in 0..8 {
                u_var10 = u_var10.wrapping_add(r32(pv));
                pv = pv.add(4);
            }
            pv = i_var12.add(0x1c1);
            for _ in 0..6 {
                u_var10 = u_var10.wrapping_add(r32(pv));
                pv = pv.add(4);
            }
            if (u_var6 >> 1) < u_var10 {
                FUN_004a01a0(param_1, param_1);
            }
        }
        {
            let a = r32(i_var12.add(0xd1));
            let b = r32(i_var12.add(0xcd));
            u_var6 = FUN_00478420(a, b);
        }
        if 0x7fff < (u_var6 as u16) {
            let mut puvar9 = i_var12.add(0x1a1);
            let c1 = (&raw mut DAT_0071b9c1).cast::<u8>().wrapping_add(o);
            w16(c1, r16(c1).wrapping_add(1));
            u_var6 = 0;
            local_8 = 0;
            let mut puvar15 = puvar9;
            for _ in 0..8 {
                u_var6 = u_var6.wrapping_add(r32(puvar15));
                puvar15 = puvar15.add(4);
            }
            let mut u_var10: u32 = 0;
            puvar15 = i_var12.add(0x1c1);
            let mut puvar7 = puvar15;
            for _ in 0..6 {
                u_var10 = u_var10.wrapping_add(r32(puvar7));
                puvar7 = puvar7.add(4);
            }
            let u_var13 = r32(i_var12.add(0xd5));
            let u_var2 = r32(i_var12.add(0xd9));
            let u_var3 = r32(i_var12.add(0xdd));
            let mut u_var14 = u_var2 | u_var3 | u_var13 | u_var10 | u_var6;
            let mut u_var13 = u_var13;
            if u_var14 == 0 {
                local_8 = 0xfa;
            } else if u_var13 < u_var3 || u_var13 < u_var2 || u_var13 < u_var6 || u_var13 < u_var10
            {
                if u_var2 < u_var3 || u_var2 < u_var13 || u_var2 < u_var6 || u_var2 < u_var10 {
                    if u_var3 < u_var2 || u_var3 < u_var13 || u_var3 < u_var6 || u_var3 < u_var10 {
                        if u_var6 < u_var2
                            || u_var6 < u_var13
                            || u_var6 < u_var3
                            || u_var6 < u_var10
                        {
                            u_var6 = 0;
                            for _ in 0..6 {
                                let t = r32(puvar15);
                                if u_var6 < t {
                                    u_var6 = r32(puvar15);
                                }
                                puvar15 = puvar15.add(4);
                            }
                        } else {
                            u_var6 = 0;
                            u_var13 = 0xffff_ffff;
                            u_var10 = 0;
                            let mut puvar11 = (&raw mut DAT_0071b738).cast::<u8>();
                            let end = (&raw mut DAT_0071e520).cast::<u8>() as usize as u32;
                            loop {
                                let t = r32(puvar9);
                                if u_var6 < t {
                                    u_var13 = r16(puvar11) as u32;
                                    u_var6 = t;
                                    u_var14 = u_var10;
                                }
                                puvar11 = puvar11.wrapping_add(0x5bd);
                                puvar9 = puvar9.add(4);
                                u_var10 = u_var10.wrapping_add(1);
                                if !((puvar11 as usize as u32) < end) {
                                    break;
                                }
                            }
                            if u_var13 != 0xffff_ffff {
                                local_4 = u_var14;
                            }
                        }
                    } else {
                        local_8 = 0x19;
                        let c7 = (&raw mut DAT_0071b9c7).cast::<u8>().wrapping_add(o);
                        w16(c7, r16(c7).wrapping_add(1));
                    }
                } else {
                    local_8 = 0x32;
                    let c5 = (&raw mut DAT_0071b9c5).cast::<u8>().wrapping_add(o);
                    w16(c5, r16(c5).wrapping_add(1));
                }
            } else {
                let c3 = (&raw mut DAT_0071b9c3).cast::<u8>().wrapping_add(o);
                w16(c3, r16(c3).wrapping_add(1));
                let t = r16((&raw mut DAT_0071b738).cast::<u8>().wrapping_add(o));
                local_8 = (t as u32).wrapping_mul(5).wrapping_add(5) as i32;
            }
            let q7a1 = (&raw mut DAT_0071b7a1).cast::<u8>().wrapping_add(o);
            w32(q7a1, r32(q7a1).wrapping_sub(local_8 as u32));
        }
        local_4
    }
}
