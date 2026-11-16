// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006db5ac: unsafe extern "C" fn(u16, u16) -> u32;
    static mut DAT_00709ffc: u32;
    static mut DAT_006fedd0: u16;
    static mut DAT_0071ecf4: i32;
    static mut DAT_0071bb1b: [u8; 0];
    static mut DAT_0071bb1f: [u8; 0];
    static mut DAT_0071bb23: [u8; 0];
    static mut DAT_0071bb27: [u8; 0];
    static mut DAT_0071bb44: [u8; 0];
    static mut DAT_0071b743: [u8; 0];
    static mut DAT_0071b749: [u8; 0];
    static mut DAT_0071b74a: [u8; 0];
    static mut DAT_0071b759: [u8; 0];
    static mut DAT_0071b730: [u8; 0];
    static mut DAT_00527e74: u16;
    static mut DAT_006da9f4: u32;
    static mut DAT_00655184: i16;
    static mut DAT_00646a58: [u8; 0];
    static mut DAT_00646878: [u8; 0];
    static mut DAT_00646c38: [u8; 0];
    static mut DAT_0052552c: i32;
    static mut DAT_0052ae18: i32;
    static mut DAT_00525548: u8;
    static mut DAT_00525508: u8;
    static mut DAT_006da9e2: i16;
    static mut DAT_00732a2c: i32;
    static mut DAT_00732a28: i32;
    static mut DAT_00732a24: i32;
    static mut DAT_0072f44c: i32;
    static mut DAT_0072f450: i32;
    static mut DAT_0072f448: i32;
    static mut DAT_006d4980: i32;
    static mut DAT_006d4988: i32;
    static mut DAT_0070ab18: i32;
    static mut DAT_0070ab14: i32;
    static mut DAT_0070ac20: i32;
    static mut DAT_006db7ac: i32;
    static mut DAT_006db7b0: i32;
    static mut DAT_006db7b4: i32;
    static mut DAT_00732e54: i32;
    static mut DAT_0071e536: i16;
    static mut DAT_00732a38: u32;
    static mut DAT_006db7a4: u32;
    static mut DAT_006d9d94: i8;
    static mut DAT_007092c1: i8;
    static mut DAT_00703ac0: i16;
    static mut DAT_00732910: i32;
    static mut DAT_006db718: u16;
    static mut DAT_0052686c: [u8; 0];
    static mut DAT_00525734: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00426840(a: u16, b: u16, c: u32, d: u32);
    fn FUN_0045f5c0();
    fn FUN_004b3b20(a: u32, b: i32, c: i32);
    fn FUN_00465970(a: i32);
    fn FUN_0044a2a0(a: u32, b: u32) -> i32;
    fn FUN_0044a440(a: u32, b: u32) -> i32;
    fn FUN_0044a370(a: u32, b: u32) -> i32;
    fn FUN_0042a230(a: i32, b: i32, c: i32, d: i16, e: i16, f: i16);
    fn FUN_0040ca00(a: i32, b: i32, c: i32) -> i32;
    fn FUN_0040ca30(a: i32, b: i32, c: i32) -> i32;
    fn FUN_0040ca60(a: i32, b: i32, c: i32) -> i32;
    fn FUN_004832a0(a: i16, b: i16, c: i16, d: i32, e: i32);
    fn FUN_0045b760(a: u32);
    fn FUN_0049c2b0(a: u32);
    fn FUN_00486310(a: i32);
    fn FUN_0049c590(a: u32);
    fn FUN_00412e60(a: *mut core::ffi::c_void, b: u16);
    fn FUN_0045b880(a: u32, b: u32) -> i32;
    fn FUN_0047f0f0(a: u32);
    fn FUN_00466020();
    fn FUN_0047f3d0(a: i32);
    fn FUN_0040ca90(a: i32, b: i32) -> i32;
    fn FUN_0040cad0(a: i32, b: i32) -> i32;
    fn FUN_00477c90(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32);
    fn FUN_00465c40();
    fn FUN_00426970();
}

macro_rules! a {
    ($n:ident, $i:expr) => {
        (&raw mut $n as *mut u8).add($i as usize)
    };
}

unsafe fn rd<T>(addr: usize) -> T {
    // SAFETY: the address is inside the game's own memory.
    unsafe { (addr as *const T).read_unaligned() }
}

unsafe fn pidx() -> u32 {
    // SAFETY: reads the game's global.
    unsafe { (DAT_0071ecf4 as u32).wrapping_mul(0x5bd) }
}

unsafe fn ent(n: u32) -> *mut u8 {
    // SAFETY: reads the game's global.
    unsafe { (DAT_006d4988 as u32).wrapping_add(n.wrapping_mul(0x23)) as usize as *mut u8 }
}

unsafe fn blk(p: *mut u8) {
    // SAFETY: p points at a game entry.
    unsafe {
        let s1: i16 = rd(p as usize + 0x14);
        let s2: i16 = rd(p as usize + 0x12);
        let s3: i16 = rd(p as usize + 0x10);
        FUN_004832a0(s1, s2, s3, 0, p as i32);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b3170(
    param_1: u16,
    param_2: u16,
    param_3: u16,
    param_4: u16,
    param_5: i16,
) {
    // SAFETY: all globals, callees and addresses belong to the game; reads and
    // writes follow the original's order.
    unsafe {
        let mut i_stack_14: i32 = 0;
        let mut i_stack_18: i32 = 0;
        let mut i_stack_1c: i32 = 0;

        let u_var9 = DAT_00709ffc;
        let u_var8 = DAT_006fedd0;
        let pi = pidx() as usize;
        let pu_var4: *mut u8 = a!(DAT_0071bb1b, pi).cast::<*mut u8>().read_unaligned();
        let u_var5: u32 = a!(DAT_0071bb1f, pi).cast::<u32>().read_unaligned();
        let u_var6: u32 = a!(DAT_0071bb23, pi).cast::<u32>().read_unaligned();
        DAT_00709ffc = 0;
        let u_var12 = (DAT_006db5ac)(param_1, param_2);
        FUN_00426840(param_3, param_4, param_5 as u16 as u32, u_var12);
        if param_5 != 0 {
            let mut pu_var16 = (DAT_00527e74 as u32).wrapping_add(DAT_006da9f4) as usize as *mut u8;
            let mut pu_var15: *mut u8;
            if DAT_00655184 == 0 {
                pu_var15 = a!(DAT_00646a58, 0);
            } else if DAT_00655184 == 0x78 {
                pu_var15 = a!(DAT_00646878, 0);
            } else {
                pu_var15 = pu_var4;
                if DAT_00655184 == 0xf0 {
                    pu_var15 = a!(DAT_00646c38, 0);
                }
            }
            let mut s_var11: i16 = if DAT_0052552c == 0x13 { 200 } else { 0x1e0 };
            loop {
                *pu_var16 = *pu_var15;
                pu_var15 = pu_var15.add(1);
                pu_var16 = pu_var16.add(1);
                s_var11 = s_var11.wrapping_sub(1);
                if s_var11 == 0 {
                    break;
                }
            }
        }
        if DAT_0052ae18 != 0 {
            FUN_0045f5c0();
        }
        let pi = pidx() as usize;
        FUN_004b3b20(
            (((DAT_0071ecf4 as u32).wrapping_mul(0x16f) >> 16) << 16)
                | (a!(DAT_0071b74a, pi).cast::<u16>().read_unaligned() as u32),
            1,
            DAT_0071ecf4,
        );
        let pi = pidx() as usize;
        if a!(DAT_0071b749, pi).read() == 0 {
            DAT_006fedd0 |= 0x400;
        } else {
            DAT_006fedd0 |= 0x200;
        }
        DAT_00525548 = 0x30;
        FUN_00465970(DAT_0071ecf4);
        DAT_006da9e2 = 0;
        let mut pi = pidx() as usize;
        DAT_0072f44c = (DAT_00732a2c as u32)
            .wrapping_sub(a!(DAT_0071bb1b, pi).cast::<u32>().read_unaligned())
            as i32;
        DAT_0072f450 = (DAT_00732a28 as u32)
            .wrapping_sub(a!(DAT_0071bb1f, pi).cast::<u32>().read_unaligned())
            as i32;
        DAT_0072f448 = (DAT_00732a24 as u32)
            .wrapping_sub(a!(DAT_0071bb23, pi).cast::<u32>().read_unaligned())
            as i32;
        if a!(DAT_0071b749, pi).read() != 0
            && (a!(DAT_0071b74a, pi).cast::<u16>().read_unaligned() as i32) < DAT_006d4980
        {
            let idx = a!(DAT_0071b74a, pi).cast::<u16>().read_unaligned() as u32;
            let w = a!(DAT_0071b759, pi).cast::<u16>().read_unaligned() as u32;
            DAT_0070ab18 = FUN_0044a2a0(ent(idx).add(3).read() as u32, w);
            let pi2 = pidx() as usize;
            let idx = a!(DAT_0071b74a, pi2).cast::<u16>().read_unaligned() as u32;
            let w = a!(DAT_0071b759, pi2).cast::<u16>().read_unaligned() as u32;
            DAT_0070ab14 = FUN_0044a440(ent(idx).add(3).read() as u32, w);
            let pi2 = pidx() as usize;
            let idx = a!(DAT_0071b74a, pi2).cast::<u16>().read_unaligned() as u32;
            let w = a!(DAT_0071b759, pi2).cast::<u16>().read_unaligned() as u32;
            let i_var13 = FUN_0044a370(ent(idx).add(3).read() as u32, w);
            DAT_0070ac20 = 0u32.wrapping_sub(i_var13 as u32) as i32;
            let pi2 = pidx() as usize;
            let idx = a!(DAT_0071b74a, pi2).cast::<u16>().read_unaligned() as u32;
            let i_var13p = ent(idx);
            let i_var13 = i_var13p as i32;
            if i_var13p.add(3).read() == 0x35 {
                DAT_0070ab18 >>= 1;
                DAT_0070ab14 >>= 1;
                DAT_0070ac20 >>= 1;
            }
            FUN_0042a230(
                DAT_0070ab14,
                i_var13,
                i_var13,
                DAT_0070ab18 as i16,
                DAT_0070ab14 as i16,
                DAT_0070ac20 as i16,
            );
            pi = pidx() as usize;
            let idx = a!(DAT_0071b74a, pi).cast::<u16>().read_unaligned() as u32;
            if ent(idx).add(3).read() == 0x35 {
                DAT_0070ab18 = (DAT_0070ab18 as u32).wrapping_mul(2) as i32;
                DAT_0070ab14 = (DAT_0070ab14 as u32).wrapping_mul(2) as i32;
                DAT_0070ac20 = (DAT_0070ac20 as u32).wrapping_mul(2) as i32;
            }
            i_stack_14 = (DAT_0070ab18 as u32)
                .wrapping_sub(a!(DAT_0071bb1b, pi).cast::<u32>().read_unaligned())
                .wrapping_add(DAT_00732a2c as u32) as i32;
            i_stack_18 = (DAT_0070ab14 as u32)
                .wrapping_sub(a!(DAT_0071bb1f, pi).cast::<u32>().read_unaligned())
                .wrapping_add(DAT_00732a28 as u32) as i32;
            i_stack_1c = (DAT_0070ac20 as u32)
                .wrapping_sub(a!(DAT_0071bb23, pi).cast::<u32>().read_unaligned())
                .wrapping_add(DAT_00732a24 as u32) as i32;
        }
        DAT_006db7ac = FUN_0040ca00(DAT_0072f44c, DAT_0072f450, DAT_0072f448);
        DAT_006db7b0 = FUN_0040ca30(DAT_0072f44c, DAT_0072f450, DAT_0072f448);
        DAT_006db7b4 = FUN_0040ca60(DAT_0072f44c, DAT_0072f450, DAT_0072f448);
        let pi = pidx() as usize;
        let u_var17 = a!(DAT_0071b74a, pi).cast::<u16>().read_unaligned();
        let i_var13p = ent(u_var17 as u32);
        let p1f: i32 = rd(i_var13p as usize + 0x1f);
        if p1f == 0 {
            let b = i_var13p.add(2).read();
            if 7 < b && b < 0xc {
                let s1: i16 = rd(i_var13p as usize + 0x14);
                let s2: i16 = rd(i_var13p as usize + 0x12);
                let s3: i16 = rd(i_var13p as usize + 0x10);
                FUN_004832a0(s1, s2, s3, 0, 0);
                DAT_0071e536 = 1;
                FUN_00412e60(core::ptr::null_mut(), u_var17);
            }
        } else {
            match i_var13p.add(2).read() {
                0..=5 => {
                    DAT_00732e54 = rd::<i32>((p1f as u32).wrapping_add(0xa7) as usize);
                    let s1: i16 = rd(i_var13p as usize + 0x14);
                    let s2: i16 = rd(i_var13p as usize + 0x12);
                    let s3: i16 = rd(i_var13p as usize + 0x10);
                    FUN_004832a0(s1, s2, s3, 0, i_var13p as i32);
                    let pi = pidx() as usize;
                    FUN_0045b760(a!(DAT_0071b74a, pi).cast::<u16>().read_unaligned() as u32);
                    let pi = pidx() as usize;
                    FUN_0049c2b0(
                        (((DAT_0071ecf4 as u32) >> 16) << 16)
                            | (a!(DAT_0071b74a, pi).cast::<u16>().read_unaligned() as u32),
                    );
                    let pi = pidx() as usize;
                    FUN_00486310(
                        ent(a!(DAT_0071b74a, pi).cast::<u16>().read_unaligned() as u32) as i32,
                    );
                }
                6 | 7 => {
                    let s1: i16 = rd(i_var13p as usize + 0x14);
                    let s2: i16 = rd(i_var13p as usize + 0x12);
                    let s3: i16 = rd(i_var13p as usize + 0x10);
                    FUN_004832a0(s1, s2, s3, 0, i_var13p as i32);
                    let pi = pidx() as usize;
                    FUN_0049c590(a!(DAT_0071b74a, pi).cast::<u16>().read_unaligned() as u32);
                }
                _ => {}
            }
        }
        let mut u_var12: u32 = 0;
        if 0 < DAT_00732a38 as i32 {
            loop {
                if u_var12 == DAT_006db7a4 && (DAT_006d9d94 == 0 || DAT_007092c1 != 0) && {
                    u_var12 = DAT_00703ac0 as u16 as u32;
                    (DAT_00703ac0 as u16 as u32) == DAT_00732a38
                } {
                    break;
                }
                let u_var17 = u_var12 as u16;
                let pi = pidx() as usize;
                if a!(DAT_0071bb27, pi).cast::<u16>().read_unaligned() != u_var17
                    || a!(DAT_0071bb44, pi).cast::<i16>().read_unaligned() != 0
                    || DAT_00732910 != 0
                {
                    let i_var7 = u_var12.wrapping_mul(0x23);
                    let b2 = ((DAT_006d4988 as u32).wrapping_add(3).wrapping_add(i_var7) as usize
                        as *const u8)
                        .read();
                    let i_var14p = (DAT_006d4988 as u32).wrapping_add(i_var7) as usize as *mut u8;
                    if b2 != 0 {
                        DAT_006db718 = a!(DAT_0052686c, (b2 as u32).wrapping_mul(0x16))
                            .cast::<u16>()
                            .read_unaligned();
                        let b2 = i_var14p.add(2).read();
                        if 5 < b2 {
                            if b2 < 8 {
                                let ptr83 = (rd::<i32>(i_var14p as usize + 0x1f) as u32)
                                    .wrapping_add(0x83)
                                    as usize;
                                if a!(DAT_0071b743, pi).read() == 0 {
                                    let u_var3: u16 = rd(ptr83);
                                    if (a!(DAT_0071b74a, pi).cast::<u16>().read_unaligned()
                                        == u_var3
                                        || a!(DAT_0071b730, pi).cast::<u32>().read_unaligned()
                                            == u_var3 as u32)
                                        && FUN_0045b880(u_var12, DAT_006db718 as u32) != 0
                                    {
                                        blk(i_var14p);
                                        FUN_0049c590(u_var17 as u32);
                                    }
                                } else if rd::<i16>(ptr83)
                                    == a!(DAT_0071b74a, pi).cast::<i16>().read_unaligned()
                                    && FUN_0045b880(u_var12, DAT_006db718 as u32) != 0
                                {
                                    blk(i_var14p);
                                    FUN_0049c590(u_var17 as u32);
                                }
                            } else if b2 == 0xd && FUN_0045b880(u_var12, DAT_006db718 as u32) != 0 {
                                blk(i_var14p);
                                FUN_0047f0f0(u_var12);
                            }
                        }
                    }
                }
                u_var12 = u_var17.wrapping_add(1) as u32;
                if !((u_var17.wrapping_add(1) as i32) < DAT_00732a38 as i32) {
                    break;
                }
            }
        }
        FUN_00466020();
        if DAT_006da9e2 != 0 {
            DAT_00525734 = 0;
            FUN_0047f3d0(0);
            DAT_00525734 = 0;
        }
        let pi = pidx() as usize;
        if a!(DAT_0071b749, pi).read() != 0 {
            let u_var12 = a!(DAT_0071b74a, pi).cast::<u16>().read_unaligned() as u32;
            if (u_var12 as i32) < DAT_006d4980 && ent(u_var12).add(2).read() != 0 {
                DAT_006db7ac = FUN_0040ca00(i_stack_14, i_stack_18, i_stack_1c);
                DAT_006db7b0 = FUN_0040ca30(i_stack_14, i_stack_18, i_stack_1c);
                DAT_006db7b4 = FUN_0040ca60(i_stack_14, i_stack_18, i_stack_1c);
                let i_var13 = FUN_0040ca90(DAT_006db7ac, DAT_006db7b4);
                let i_var14 = FUN_0040cad0(DAT_006db7b0, DAT_006db7b4);
                FUN_00477c90(
                    (i_var13 as u32).wrapping_sub(2) as i32,
                    (i_var14 as u32).wrapping_sub(2) as i32,
                    4,
                    4,
                    0xce,
                    1,
                );
            }
        }
        FUN_00465c40();
        DAT_00525548 = DAT_00525508;
        FUN_00426970();
        let i_var13 = DAT_0071ecf4;
        DAT_006fedd0 = u_var8;
        a!(DAT_0071bb1b, pidx())
            .cast::<*mut u8>()
            .write_unaligned(pu_var4);
        a!(DAT_0071bb1f, (i_var13 as u32).wrapping_mul(0x5bd))
            .cast::<u32>()
            .write_unaligned(u_var5);
        a!(DAT_0071bb23, (i_var13 as u32).wrapping_mul(0x5bd))
            .cast::<u32>()
            .write_unaligned(u_var6);
        DAT_00709ffc = u_var9;
    }
}
