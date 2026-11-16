// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_00732a3c: i32;
    static mut DAT_006d4980: i32;
    static mut DAT_006d4988: i32;
    static mut DAT_00732a38: i32;
    static mut DAT_00703c88: i32;
    static mut DAT_00732e54: i32;
    static mut DAT_006d9d62: i16;
    static mut DAT_0070b252: [u8; 0];
    static mut DAT_0071b730: [u8; 0];
    static mut DAT_0071b741: [u8; 0];
    static mut DAT_0071b744: [u8; 0];
    static mut DAT_0071b756: [u8; 0];
    static mut DAT_0071b77b: [u8; 0];
    static mut DAT_0071b77d: [u8; 0];
    static mut DAT_0071b77f: [u8; 0];
    static mut DAT_0071b781: [u8; 0];
    static mut DAT_0071b783: [u8; 0];
    static mut DAT_0071b75d: [u8; 0];
    static mut DAT_0071b74a: [u8; 0];
    static mut DAT_006da3e0: [u8; 0];
    static mut DAT_00524798: [u8; 0];
    static mut DAT_0070ad92: [u8; 0];
    static mut DAT_0070ad93: [u8; 0];
    static mut DAT_0052a650: [u8; 0];
    static mut DAT_0052b08d: [u8; 0];
    static mut DAT_00703ca0: [u8; 0];
    static mut DAT_00524948: [i8; 0];
    static mut DAT_00524934: [i8; 0];
    static mut DAT_00524920: [i8; 0];
    static mut DAT_00524914: [i8; 0];
    static mut DAT_00524904: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0040f0f0(a: u32);
    fn FUN_0040f050(a: u16);
    fn FUN_00423d00(a: i32);
    fn FUN_00478320(a: u32, b: u16) -> u32;
    fn FUN_004296b0(a: u16);
    fn FUN_00429310(a: u32);
    fn FUN_0042af80(a: i16) -> u32;
    fn FUN_0042b150(a: i32) -> i32;
    fn FUN_00481380(a: *mut i8) -> i32;
}

macro_rules! addr {
    ($x:ident) => {
        (&raw mut $x) as usize as u32
    };
}

#[inline(always)]
unsafe fn rd8(a: u32) -> u8 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as usize as *const u8).read() }
}
#[inline(always)]
unsafe fn rd16(a: u32) -> u16 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as usize as *const u16).read_unaligned() }
}
#[inline(always)]
unsafe fn rd32(a: u32) -> u32 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as usize as *const u32).read_unaligned() }
}
#[inline(always)]
unsafe fn wr8(a: u32, v: u8) {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as usize as *mut u8).write(v) }
}
#[inline(always)]
unsafe fn wr16(a: u32, v: u16) {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as usize as *mut u16).write_unaligned(v) }
}
#[inline(always)]
unsafe fn wr32(a: u32, v: u32) {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as usize as *mut u32).write_unaligned(v) }
}
#[inline(always)]
unsafe fn eqn(a: u32, b: u32, n: u32) -> bool {
    let mut i = 0u32;
    while i < n {
        // SAFETY: reads the game's own string bytes.
        unsafe {
            if rd8(a.wrapping_add(i)) != rd8(b.wrapping_add(i)) {
                return false;
            }
        }
        i += 1;
    }
    true
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00423890(param_1: i32, param_2: i32, param_3: i32) -> u32 {
    // SAFETY: all globals, tables and callees are the game's own; every
    // address is read and written where the original does.
    unsafe {
        if param_2 == 1 {
            let mut u_var11: u16 = 0;
            if _DAT_00732a3c < DAT_006d4980 {
                let mut p: u32 = (_DAT_00732a3c as u32)
                    .wrapping_mul(0x23)
                    .wrapping_add(3)
                    .wrapping_add(DAT_006d4988 as u32);
                let mut cnt: u32 = (DAT_006d4980 as u32).wrapping_sub(_DAT_00732a3c as u32);
                loop {
                    if rd8(p) != 0 {
                        let t0 =
                            rd32(addr!(DAT_0070b252).wrapping_add(
                                (rd8(p.wrapping_add(0x13)) as u32).wrapping_mul(0x566),
                            )) as i32;
                        if t0 == param_1 {
                            let t1 = rd32(p.wrapping_add(0x1c));
                            let t2 = rd32(t1.wrapping_add(0xa7));
                            let c = rd8(t2.wrapping_add(7)) as i8;
                            if c != 3 && c != 5 && c != 4 {
                                u_var11 = u_var11.wrapping_add(1);
                            }
                        }
                    }
                    p = p.wrapping_add(0x23);
                    cnt = cnt.wrapping_sub(1);
                    if cnt == 0 {
                        break;
                    }
                }
            }
            if u_var11 < 2 {
                return 0;
            }
        }
        let i_var5: u32 = (param_1 as u32).wrapping_mul(0x5bd);
        let mut u_var8: u32 = rd32(addr!(DAT_0071b730).wrapping_add(i_var5));
        if u_var8 == 0xffff_ffff {
            return 0;
        }
        let i_var3: u32 = u_var8.wrapping_mul(0x23);
        let ent: u32 = (DAT_006d4988 as u32).wrapping_add(i_var3);
        wr32(ent.wrapping_add(0x1b), 0xffff_ffff);
        let mut t0 = rd32(ent.wrapping_add(0x1f));
        wr8(t0.wrapping_add(0x90), 1);
        t0 = rd32(ent.wrapping_add(0x1f));
        wr8(t0.wrapping_add(0x89), 1);
        FUN_0040f0f0(u_var8);
        let u_var11 = u_var8 as u16;
        FUN_0040f050(u_var11);
        FUN_00423d00(param_1);
        let mut i_var13: u32 = 0;
        t0 = rd32(ent.wrapping_add(0x1f));
        let i_var10: u32 = rd32(t0.wrapping_add(0xa7));
        loop {
            wr8(i_var10.wrapping_add(0x216).wrapping_add(i_var13), 0);
            wr8(i_var10.wrapping_add(0x218).wrapping_add(i_var13), 0);
            wr8(i_var10.wrapping_add(0x21a).wrapping_add(i_var13), 0);
            wr16(
                i_var10
                    .wrapping_add(0x21c)
                    .wrapping_add(i_var13.wrapping_mul(2)),
                0,
            );
            wr32(
                i_var10
                    .wrapping_add(0x220)
                    .wrapping_add(i_var13.wrapping_mul(4)),
                0,
            );
            i_var13 = i_var13.wrapping_add(1);
            if (i_var13 as i32) >= 2 {
                break;
            }
        }
        i_var13 = 0;
        loop {
            wr8(i_var10.wrapping_add(0x22b).wrapping_add(i_var13), 1);
            i_var13 = i_var13.wrapping_add(1);
            wr16(
                i_var10
                    .wrapping_add(0x22b)
                    .wrapping_add(i_var13.wrapping_mul(2)),
                0,
            );
            if (i_var13 as i32) >= 2 {
                break;
            }
        }
        wr16(i_var10.wrapping_add(0x231), 0);
        let a = rd16(i_var10.wrapping_add(0x20b));
        let b = rd16(i_var10.wrapping_add(0x20d));
        wr16(i_var10.wrapping_add(0x20b), a.wrapping_add(b));
        wr16(i_var10.wrapping_add(0x20d), 0);
        wr8(i_var10.wrapping_add(0x210), 0);
        wr32(addr!(DAT_0071b730).wrapping_add(i_var5), 0xffff_ffff);
        wr8(addr!(DAT_0071b741).wrapping_add(i_var5), 0);
        wr8(addr!(DAT_0071b744).wrapping_add(i_var5), 0);
        wr8(addr!(DAT_0071b756).wrapping_add(i_var5), 0);
        wr16(addr!(DAT_0071b77b).wrapping_add(i_var5), 0);
        wr16(addr!(DAT_0071b77d).wrapping_add(i_var5), 0);
        wr16(addr!(DAT_0071b77f).wrapping_add(i_var5), 0);
        wr16(addr!(DAT_0071b781).wrapping_add(i_var5), 0);
        wr16(addr!(DAT_0071b783).wrapping_add(i_var5), 0);
        wr16(addr!(DAT_0071b75d).wrapping_add(i_var5), 0xffff);
        t0 = rd32(i_var10.wrapping_add(0x432));
        let t1 = rd32(t0.wrapping_add(0x1f));
        let i_var13b: u32 = rd32(t1.wrapping_add(0xab));
        wr8(i_var13b.wrapping_add(4), 0);
        if param_3 == 0 {
            let r = FUN_00481380((&raw mut DAT_00524948) as *mut i8);
            wr8(i_var13b.wrapping_add(0xe), r as u8);
            wr8(i_var13b.wrapping_add(0xf), r as u8);
            DAT_00732e54 = i_var10 as i32;
            FUN_004296b0(u_var11);
            FUN_00429310(u_var8);
            wr8(i_var10.wrapping_add(0xa4), 0);
            wr8(i_var10.wrapping_add(0x9c), 0);
            wr8(i_var10.wrapping_add(0xab), 0);
            wr8(i_var10.wrapping_add(0x96), 0);
            wr8(i_var10.wrapping_add(0x97), 0);
            return 1;
        }
        let idx = (rd8(ent.wrapping_add(0x16)) as u32).wrapping_mul(0x566);
        let b2 = rd8(addr!(DAT_006da3e0).wrapping_add(rd8(
            addr!(DAT_00524798).wrapping_add(rd8(addr!(DAT_0070ad92).wrapping_add(idx)) as u32)
        ) as u32));
        wr8(i_var13b.wrapping_add(0xf), b2);
        wr8(i_var13b.wrapping_add(0xe), b2);
        let p15: u32 = addr!(DAT_00703ca0).wrapping_add((b2 as u32).wrapping_mul(0x55));
        let u_var4: u16;
        if eqn(p15, addr!(DAT_00524948), 8)
            || eqn(p15, addr!(DAT_00524934), 0x11)
            || eqn(p15, addr!(DAT_00524920), 0x11)
            || eqn(p15, addr!(DAT_00524914), 0xc)
        {
            u_var4 = 0;
        } else if eqn(p15, addr!(DAT_00524904), 0xe) {
            u_var4 = 0x8000;
        } else {
            u_var4 =
                rd16(addr!(DAT_0052a650).wrapping_add(
                    (rd8(addr!(DAT_0070ad93).wrapping_add(idx)) as u32).wrapping_mul(2),
                ));
        }
        wr16(i_var10.wrapping_add(0xbd), u_var4);
        let mut u16v = rd16(
            addr!(DAT_0052b08d)
                .wrapping_add((rd8(i_var10.wrapping_add(4)) as u32).wrapping_mul(0xef)),
        );
        let u_var7 = FUN_00478320(u16v as u32, u_var4);
        t0 = rd32(ent.wrapping_add(0x1f));
        wr16(t0.wrapping_add(0x79), u_var7 as u16);
        t0 = rd32(ent.wrapping_add(0x1f));
        wr16(t0.wrapping_add(0x7b), 0);
        DAT_00732e54 = i_var10 as i32;
        FUN_004296b0(u_var11);
        FUN_00429310(u_var8);
        u16v = rd16(addr!(DAT_0071b74a).wrapping_add(i_var5));
        if u16v != 0xffff {
            u_var8 = u16v as u32;
            if (u_var8 as i32) < DAT_006d4980
                || (DAT_00732a38 <= u_var8 as i32
                    && (u_var8 as i32)
                        < (DAT_00703c88 as u32).wrapping_add(DAT_00732a38 as u32) as i32)
            {
                let mut flag = false;
                u_var8 = 0;
                loop {
                    DAT_006d9d62 = u_var8 as i16;
                    u16v = rd16(addr!(DAT_0071b74a).wrapping_add(i_var5));
                    let u_var9 = FUN_0042af80(u16v as i16);
                    if (u_var9 as i16) != 0 {
                        flag = true;
                    }
                    u_var8 = u_var8.wrapping_add(1);
                    if u_var8 >= 3 {
                        break;
                    }
                }
                if flag {
                    let r = FUN_0042b150(i_var10 as i32) as u32;
                    u16v = rd16(addr!(DAT_0071b74a).wrapping_add(i_var5));
                    wr16(r.wrapping_add(0x2b), u16v);
                }
            }
        }
        1
    }
}
