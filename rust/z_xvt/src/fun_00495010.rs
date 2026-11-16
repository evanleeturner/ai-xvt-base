// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d9d56: *mut u8;
    static mut DAT_006d4988: *mut u8;
    static mut DAT_00732e54: *mut u8;
    static mut DAT_0071ecf4: *mut u8;
    static mut DAT_006d9d50: u16;
    static mut DAT_006d9d60: u16;
    static mut DAT_006d9d62: u16;
    static mut DAT_006db580: u32;
    static mut DAT_00732a2c: i32;
    static mut DAT_00732a28: i32;
    static mut DAT_00732a24: i32;
    static mut DAT_0070ab18: i32;
    static mut DAT_0070ab14: i32;
    static mut DAT_0070ac20: i32;
    static mut DAT_00732fe2: u16;
    static mut DAT_00655a84: i16;
    static mut DAT_0070ad94: u8;
    static mut DAT_0070ad95: u8;
    static mut DAT_0070ad92: u8;
    static mut DAT_0070ad47: u8;
    static mut DAT_0070ad45: u8;
    static mut DAT_0070ad44: u8;
    static mut DAT_0070ad40: u8;
    static mut DAT_0052b141: u8;
    static mut DAT_0052b143: u8;
    static mut DAT_0052b145: u8;
    static mut DAT_0052b147: u8;
    static mut DAT_0052b149: u8;
    static mut DAT_0052b0b4: u8;
    static mut DAT_0052b0b6: u8;
    static mut DAT_0052b0ba: u8;
    static mut DAT_0052b13c: u8;
    static mut DAT_00524880: u8;
    static mut DAT_00524798: u8;
    static mut DAT_005247c0: u8;
    static mut DAT_006da3e0: u8;
    static mut DAT_006d9990: u8;
    static mut DAT_006d9cd8: u8;
    static mut DAT_00703ca0: u8;
    static mut DAT_006d4a9e: u8;
    static mut DAT_006d4abc: u8;
    static mut DAT_006d4a76: u8;
    static mut DAT_006d4a7a: u8;
    static mut DAT_006d4a7e: u8;
    static mut DAT_006d4a82: u8;
    static mut DAT_006d4aaa: u8;
    static mut DAT_006d4aac: u8;
    static mut DAT_006d4aae: u8;
    static mut DAT_0071b730: u8;
    static mut DAT_0071b74a: u8;
    static mut DAT_0071bb37: u8;
    static mut DAT_006d4870: i8;
    static mut DAT_0071e590: i8;
    static mut DAT_00523c18: [i8; 0x20];
    static mut DAT_00524f58: [i8; 0x20];
    static mut DAT_00524f30: [i8; 0x20];
    static mut DAT_00524f44: [i8; 0x20];
    static mut DAT_00524f1c: [i8; 0x20];
    static mut DAT_00524f90: [i8; 0x20];
    static mut DAT_00524f6c: [i8; 0x20];
    static mut DAT_00524f7c: [i8; 0x20];
    static mut DAT_00528360: [i8; 0x20];
    static mut DAT_00524934: [i8; 0x20];
    static mut DAT_00525b50: [i8; 0x20];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0042b150(a: i32) -> u32;
    fn FUN_00421c70(a: u32, b: i32);
    fn FUN_0042a230(a: i32, b: u32, c: i32, d: i32, e: i16, f: i16);
    fn FUN_004979e0(a: i32, b: i32);
    fn FUN_004a1d70(a: i32, b: i32);
    fn FUN_0040a1a0(a: i32, b: i32, c: i32) -> u32;
    fn FUN_004a3e40(a: u32, b: i32, c: *mut i8);
    fn FUN_004a3490(a: i32, b: *mut i8);
    fn FUN_0040c070(a: i32, b: i32, c: u32, d: u32) -> u32;
    fn FUN_00478320(a: u32, b: u16) -> u32;
    fn FUN_004a6a80();
    fn FUN_004b1c00(a: u32, b: i32);
    fn FUN_004a6f90();
    fn FUN_004a34c0(a: u32, b: *mut u32, c: i32);
    fn FUN_00496770(a: u32, b: i32, c: i32);
    fn FUN_00481380(a: *mut i8) -> i32;
    fn FUN_004296b0(a: u32);
    fn FUN_00429310(a: u32);
    fn FUN_004a01a0(a: u32, b: u32);
    fn FUN_00478470() -> u32;
}

macro_rules! ga {
    ($n:ident) => {
        (&raw mut $n).cast::<u8>()
    };
}

macro_rules! gs {
    ($n:ident) => {
        (&raw mut $n).cast::<i8>()
    };
}

unsafe fn ld<T>(p: *const u8) -> T {
    // SAFETY: the caller passes an address the game keeps readable.
    unsafe { p.cast::<T>().read_unaligned() }
}

unsafe fn st<T>(p: *mut u8, v: T) {
    // SAFETY: the caller passes an address the game keeps writable.
    unsafe { p.cast::<T>().write_unaligned(v) }
}

fn ip(v: i32) -> *mut u8 {
    v as u32 as usize as *mut u8
}

fn up(v: u32) -> *mut u8 {
    v as usize as *mut u8
}

fn pu(p: *mut u8) -> u32 {
    p as usize as u32
}

fn c22(a: u16, b: u16) -> u32 {
    ((a as u32) << 16) | (b as u32)
}

fn c31(a: u32, b: u8) -> u32 {
    (a << 8) | (b as u32)
}

unsafe fn ldp(p: *const u8) -> *mut u8 {
    // SAFETY: the caller passes a readable address.
    unsafe { up(ld::<u32>(p)) }
}

unsafe fn p56() -> *mut u8 {
    // SAFETY: DAT_006d9d56 is the game's own pointer, stored unaligned.
    unsafe { up(ld::<u32>((&raw const DAT_006d9d56).cast::<u8>())) }
}

unsafe fn d49() -> *mut u8 {
    // SAFETY: the game's own global.
    unsafe { DAT_006d4988 }
}

unsafe fn e54() -> *mut u8 {
    // SAFETY: the game's own global.
    unsafe { DAT_00732e54 }
}

unsafe fn ent() -> usize {
    // SAFETY: the game's own global.
    unsafe { (DAT_006d9d50 as u32).wrapping_mul(0x23) as usize }
}

unsafe fn d16(i: usize) -> u32 {
    // SAFETY: the game's own table.
    unsafe { *d49().add(i + 0x16) as u32 }
}

unsafe fn g16(base: *mut u8, off: u32) -> i16 {
    // SAFETY: the game's own table.
    unsafe { ld::<i16>(base.add(off as usize)) }
}

unsafe fn inc16(p: *mut u8) {
    // SAFETY: the game's own counter.
    unsafe { st::<i16>(p, ld::<i16>(p).wrapping_add(1)) }
}

unsafe fn rc(a: *const i8, b: *const i8, n: i32) -> bool {
    let mut n = n;
    let mut a = a;
    let mut b = b;
    let mut f = false;
    loop {
        if n == 0 {
            break;
        }
        n -= 1;
        // SAFETY: the compared strings are the game's own.
        f = unsafe { *a == *b };
        // SAFETY: stepping within the game's strings.
        unsafe {
            a = a.add(1);
            b = b.add(1);
        }
        if !f {
            break;
        }
    }
    f
}

unsafe fn swap_call(puvar19: *mut u32, u5: u16) {
    let mut buf: [u32; 14] = [0; 14];
    // SAFETY: the game's own block of 55 bytes is saved and restored around the calls.
    unsafe {
        let save = DAT_00732e54;
        core::ptr::copy_nonoverlapping(
            (&raw const DAT_006d9d50).cast::<u8>(),
            buf.as_mut_ptr().cast::<u8>(),
            55,
        );
        DAT_00732e54 = puvar19.cast::<u8>();
        FUN_004296b0(u5 as u32);
        FUN_00429310(u5 as u32);
        DAT_00732e54 = save;
        core::ptr::copy_nonoverlapping(
            buf.as_ptr().cast::<u8>(),
            (&raw mut DAT_006d9d50).cast::<u8>(),
            55,
        );
    }
}

#[unsafe(no_mangle)]
#[allow(
    non_snake_case,
    unused_mut,
    unused_assignments,
    unused_variables,
    unused_unsafe
)]
pub unsafe extern "stdcall" fn FUN_00495010() -> u32 {
    // SAFETY: every global, callee and pointer here belongs to the game and is
    // read and written where the original does.
    unsafe {
        let extraout_ecx: *mut u8 = core::ptr::null_mut();
        let extraout_edx: i32 = 0;
        let mut local_38: *mut u32 = core::ptr::null_mut();
        let mut local_48: u16;
        let mut s_stack_46: i16 = 0;
        let mut local_4c: u16 = 0;
        let mut local_3c: i32 = 0;
        let puvar19: *mut u32;
        let b_var1: u8;
        let u_var3: u16;

        let mut u_var5: u16 = ld::<u16>(p56().add(0x18));
        let mut i_var15: i32 = (DAT_006d9d62 as u32).wrapping_mul(0x52) as i32;
        let mut b_var12: u8 = *ga!(DAT_0070ad94).add(
            (DAT_006d9d60 as u32)
                .wrapping_mul(0x566)
                .wrapping_add(i_var15 as u32) as usize,
        );
        let mut u_var6: u16 = b_var12 as u16;
        let i_var14: usize = (u_var5 as u32)
            .wrapping_mul(8)
            .wrapping_sub(u_var5 as u32)
            .wrapping_mul(5) as usize;
        let mut pb13: *mut u8 = d49().add(i_var14);
        if ld::<i32>(pb13.add(0x1f)) == 0 {
            b_var1 = *pb13.add(0x16);
            u_var3 = ld::<u16>(pb13);
            local_48 = 0xff;
            puvar19 = local_38;
        } else {
            puvar19 = ldp(ip(ld::<i32>(pb13.add(0x1f))).add(0xa7)).cast::<u32>();
            local_38 = FUN_0042b150(pu(puvar19.cast::<u8>()) as i32) as usize as *mut u32;
            local_48 = *puvar19.cast::<u8>().add(4) as u16;
            b_var1 = *d49().add(i_var14).add(0x16);
            u_var3 = ld::<u16>(d49().add(i_var14));
            i_var15 = extraout_edx;
        }
        let pb19: *mut u8 = puvar19.cast::<u8>();
        pb13 = d49();
        let local_44: u16 = b_var1 as u16;
        let mut pb9: *mut u8 = p56();
        let mut pb4: *mut u8;

        match *p56().add(0x35) {
            0 => {
                pb13 = d49().add(i_var14);
                if ld::<i32>(pb13.add(0x1f)) == 0 {
                    FUN_00421c70(u_var5 as u32, 0);
                    st::<i32>(p56().add(0x1f), DAT_00732a2c);
                    st::<i32>(p56().add(0x23), DAT_00732a28);
                    i_var15 = (DAT_00732a24 as u32).wrapping_add(0x800) as i32;
                } else {
                    i_var15 = (local_48 as u32).wrapping_mul(0xef) as i32;
                    let o = i_var15 as u32;
                    let e = (*e54().add(4) as u32).wrapping_mul(0xef);
                    let mut s_var8: i16;
                    if *pb13.add(2) == 0 || *pb13.add(2) == 1 {
                        s_var8 = (g16(ga!(DAT_0052b143), o) as u32).wrapping_sub(g16(
                            ga!(DAT_0052b147),
                            e,
                        )
                            as u32) as i16;
                    } else if *d49().add(ent() + 2) == 0 || *d49().add(ent() + 2) == 1 {
                        s_var8 = (g16(ga!(DAT_0052b143), o) as u32)
                            .wrapping_sub(g16(ga!(DAT_0052b149), e) as u32)
                            .wrapping_add(g16(ga!(DAT_0052b145), o) as u32)
                            as i16;
                    } else {
                        s_var8 = (g16(ga!(DAT_0052b145), o) as u32)
                            .wrapping_mul(2)
                            .wrapping_sub(g16(ga!(DAT_0052b149), e) as u32)
                            as i16;
                    }
                    s_var8 = (s_var8 as u32).wrapping_add(
                        (g16(ga!(DAT_0052b145), o) as u32)
                            .wrapping_sub(g16(ga!(DAT_0052b149), e) as u32)
                            .wrapping_mul(2),
                    ) as i16;
                    if s_var8 < 0 {
                        s_var8 = 0x7000;
                    }
                    FUN_0042a230(
                        e as i32,
                        (g16(ga!(DAT_0052b145), o) as i32)
                            .wrapping_sub(g16(ga!(DAT_0052b149), e) as i32)
                            as u16 as u32,
                        pu(pb13) as i32,
                        0,
                        s_var8,
                        g16(ga!(DAT_0052b141), o),
                    );
                    st::<i32>(
                        p56().add(0x1f),
                        (ld::<i32>(d49().add(i_var14 + 4)) as u32).wrapping_add(DAT_0070ab18 as u32)
                            as i32,
                    );
                    st::<i32>(
                        p56().add(0x23),
                        (ld::<i32>(d49().add(i_var14 + 8)) as u32).wrapping_add(DAT_0070ab14 as u32)
                            as i32,
                    );
                    i_var15 = (ld::<i32>(d49().add(i_var14 + 0xc)) as u32)
                        .wrapping_add(DAT_0070ac20 as u32) as i32;
                }
                st::<i32>(p56().add(0x27), i_var15);
                FUN_004979e0(0, 1);
                if 0x4000 < DAT_006db580 {
                    pb13 = e54();
                    *pb13.add(0xbd) = 0xff;
                    *pb13.add(0xbe) = 0xff;
                }
                pb13 = e54();
                if 0x2000 < DAT_006db580 {
                    *pb13.add(0xbd) = 0;
                    *pb13.add(0xbe) = 0x80;
                    return pu(pb13) & 0xffff0000;
                }
                if 0x800 < DAT_006db580 {
                    *pb13.add(0xbd) = 0;
                    *pb13.add(0xbe) = 0x40;
                    return pu(pb13) & 0xffff0000;
                }
                if pu(ldp(d49().add(i_var14 + 0x1b))) == 0xffffffff
                    || ld::<i32>(d49().add(i_var14 + 0x1f)) == 0
                    || ld::<i16>(ip(ld::<i32>(d49().add(i_var14 + 0x1f))).add(0x79)) == 0
                {
                    *pb13.add(0xbd) = 0;
                    *pb13.add(0xbe) = 0;
                    *p56().add(0x35) = 1;
                    return pu(pb13) & 0xffff0000;
                }
                pb9 = DAT_0071ecf4;
                if DAT_0071ecf4 == ldp(d49().add(i_var14 + 0x1b)) {
                    let mut u_var11: u32 = 0;
                    pb9 = ga!(DAT_006d9990);
                    loop {
                        if ld::<i16>(pb9) == 0x103 {
                            break;
                        }
                        pb9 = pb9.add(0x54);
                        u_var11 = u_var11.wrapping_add(1);
                        if !(pb9 < ga!(DAT_006d9cd8)) {
                            break;
                        }
                    }
                    if 9 < u_var11 {
                        DAT_00732fe2 = *ga!(DAT_0070ad47)
                            .add((DAT_006d9d60 as u32).wrapping_mul(0x566) as usize)
                            as u16;
                        FUN_004a1d70(0x103, pu(DAT_0071ecf4) as i32);
                        u_var11 = FUN_0040a1a0(0x24, -1, pu(DAT_0071ecf4) as i32);
                        return u_var11 & 0xffff0000;
                    }
                }
            }
            1 => 'c1: {
                pb13 = d49().add(i_var14);
                let mut i_var18: i32;
                if ld::<i32>(pb13.add(0x1f)) == 0 {
                    FUN_00421c70(u_var5 as u32, 0);
                    st::<i32>(
                        e54().add(0xb1),
                        (DAT_00732a2c as u32).wrapping_sub(ld::<i32>(d49().add(ent() + 4)) as u32)
                            as i32,
                    );
                    local_3c = ld::<i32>(e54().add(0xb1));
                    st::<i32>(
                        e54().add(0xb5),
                        (DAT_00732a28 as u32).wrapping_sub(ld::<i32>(d49().add(ent() + 8)) as u32)
                            as i32,
                    );
                    i_var15 = ld::<i32>(e54().add(0xb5));
                    local_48 = 0;
                    s_stack_46 = 0;
                    st::<i32>(
                        e54().add(0xb9),
                        (DAT_00732a24 as u32)
                            .wrapping_sub(ld::<i32>(d49().add(ent() + 0xc)) as u32)
                            .wrapping_add(0x80) as i32,
                    );
                    local_4c = 0x4000;
                    i_var18 = ld::<i32>(e54().add(0xb9));
                } else {
                    let o = (local_48 as u32).wrapping_mul(0xef);
                    i_var15 = o as i32;
                    let i18: u32;
                    let u_var11: u32;
                    if *pb13.add(2) == 0 || *pb13.add(2) == 1 {
                        i18 = (*e54().add(4) as u32).wrapping_mul(0xef);
                        u_var11 = (g16(ga!(DAT_0052b143), o) as i32).wrapping_sub(g16(
                            ga!(DAT_0052b147),
                            i18,
                        )
                            as i32) as u16 as u32;
                    } else if *d49().add(ent() + 2) == 0 || *d49().add(ent() + 2) == 1 {
                        i18 = (*e54().add(4) as u32).wrapping_mul(0xef);
                        u_var11 = (g16(ga!(DAT_0052b143), o) as i32).wrapping_sub(g16(
                            ga!(DAT_0052b149),
                            i18,
                        )
                            as i32) as u16 as u32;
                    } else {
                        i18 = (*e54().add(4) as u32).wrapping_mul(0xef);
                        u_var11 = (g16(ga!(DAT_0052b145), o) as i32).wrapping_sub(g16(
                            ga!(DAT_0052b149),
                            i18,
                        )
                            as i32) as u16 as u32;
                    }
                    FUN_0042a230(
                        i18 as i32,
                        u_var11,
                        pu(pb13) as i32,
                        0,
                        u_var11 as i16,
                        g16(ga!(DAT_0052b141), o),
                    );
                    st::<i32>(
                        e54().add(0xb1),
                        (ld::<i32>(d49().add(i_var14 + 4)) as u32)
                            .wrapping_sub(ld::<i32>(d49().add(ent() + 4)) as u32)
                            .wrapping_add(DAT_0070ab18 as u32) as i32,
                    );
                    local_3c = ld::<i32>(e54().add(0xb1));
                    st::<i32>(
                        e54().add(0xb5),
                        (ld::<i32>(d49().add(i_var14 + 8)) as u32)
                            .wrapping_sub(ld::<i32>(d49().add(ent() + 8)) as u32)
                            .wrapping_add(DAT_0070ab14 as u32) as i32,
                    );
                    i_var15 = ld::<i32>(e54().add(0xb5));
                    st::<i32>(
                        e54().add(0xb9),
                        (ld::<i32>(d49().add(i_var14 + 0xc)) as u32)
                            .wrapping_sub(ld::<i32>(d49().add(ent() + 0xc)) as u32)
                            .wrapping_add(DAT_0070ac20 as u32) as i32,
                    );
                    i_var18 = ld::<i32>(e54().add(0xb9));
                    local_48 = ld::<u16>(d49().add(i_var14 + 0x14));
                    s_stack_46 = ld::<i16>(d49().add(i_var14 + 0x10));
                    local_4c = ld::<u16>(d49().add(i_var14 + 0x12));
                }
                if (ld::<i16>(d49().add(ent() + 0x14)) as i32) != (local_48 as i32) {
                    *e54().add(0xa4) = 1;
                    pb13 = e54();
                    *pb13.add(0xa5) = 0;
                    *pb13.add(0xa6) = 0x80;
                    st::<u16>(p56().add(0x30), local_48);
                }
                local_38 = (DAT_006d9d50 as u32) as usize as *mut u32;
                if ld::<i16>(
                    d49().add((local_38 as usize as u32).wrapping_mul(0x23) as usize + 0x10),
                ) != s_stack_46
                {
                    *e54().add(0xab) = 2;
                    pb13 = e54();
                    *pb13.add(0xac) = 0;
                    *pb13.add(0xad) = 0x80;
                    st::<i16>(p56().add(0x32), s_stack_46);
                }
                if ld::<i16>(d49().add(ent() + 0x12)) != ld::<i16>(d49().add(i_var14 + 0x12)) {
                    st::<u16>(p56().add(0x2e), local_4c);
                    pb13 = e54();
                    *pb13.add(0x9e) = 0;
                    *pb13.add(0x9f) = 0x80;
                    *e54().add(0x9d) = 0;
                    if ld::<u16>(e54().add(0xd)) < ld::<u16>(p56().add(0x2e)) {
                        *e54().add(0x9c) = 2;
                    } else {
                        *e54().add(0x9c) = 1;
                    }
                }
                if local_3c < 0 {
                    local_3c = 0u32.wrapping_sub(local_3c as u32) as i32;
                }
                if i_var15 < 0 {
                    i_var15 = 0u32.wrapping_sub(i_var15 as u32) as i32;
                }
                if i_var18 < 0 {
                    i_var18 = 0u32.wrapping_sub(i_var18 as u32) as i32;
                }
                pb9 = up((i_var15 as u32)
                    .wrapping_add(i_var18 as u32)
                    .wrapping_add(local_3c as u32));
                if 0xf < (pu(pb9) as i32) {
                    break 'c1;
                }
                pb13 = e54();
                for k in 0xb1usize..0xb5 {
                    *pb13.add(k) = 0;
                }
                pb13 = e54();
                for k in 0xb5usize..0xb9 {
                    *pb13.add(k) = 0;
                }
                pb13 = e54();
                for k in 0xb9usize..0xbd {
                    *pb13.add(k) = 0;
                }
                *p56().add(0x35) = 2;
                st::<u32>(p56().add(0x36), (u_var6 as u32).wrapping_mul(0x49c));
                pb13 = p56();
                *pb13.add(0x3a) = 0xec;
                *pb13.add(0x3b) = 0;
                if ld::<i32>(d49().add(ent() + 0x1f)) != 0 && *e54().add(0x7c) == 0 {
                    *e54().add(0x7c) = 1;
                    let u11 = DAT_006d9d60 as u32;
                    let i15 = u11.wrapping_mul(0x126) as usize;
                    inc16(ga!(DAT_006d4a9e).add(i15));
                    if *ga!(DAT_0070ad40).add(u11.wrapping_mul(0x566) as usize) == *e54().add(0xb0)
                    {
                        *ga!(DAT_006d4abc).add(i15) = 1;
                    }
                }
                pb13 = d49();
                if ld::<i32>(d49().add(i_var14 + 0x1f)) != 0 {
                    pb13 = up((pu(d49()) & 0xffffff00) | (*pb19.add(0x7a) as u32));
                    if *pb19.add(0x7a) == 0 {
                        *pb19.add(0x7a) = 1;
                        pb13 = up((local_44 as u32).wrapping_mul(0x126));
                        inc16(up(pu(pb13).wrapping_add(0x6d4a9c)));
                        if (*ga!(DAT_0070ad40).add((local_44 as u32).wrapping_mul(0x566) as usize)
                            as i32)
                            == (*pb19.add(0xb0) as i8 as i32)
                        {
                            *up(pu(pb13).wrapping_add(0x6d4abb)) = 1;
                        }
                    }
                }
                FUN_004a3e40(
                    c22((pu(pb13) >> 16) as u16, DAT_006d9d50),
                    1,
                    &raw mut DAT_006d4870,
                );
                FUN_004a3490(0, &raw mut DAT_006d4870);
                FUN_004a3e40(c22(u_var5, u_var5), 1, &raw mut DAT_0071e590);
                FUN_004a3490(1, &raw mut DAT_0071e590);
                DAT_00732fe2 = *ip(ld::<i32>(d49().add(ent() + 0x1f))).add(0x86) as u16;
                FUN_004a1d70(0xe9, pu(DAT_0071ecf4) as i32);
                if u_var6 >= 2 {
                    let p25 = ga!(DAT_00703ca0)
                        .add((*p56().add(0xb) as usize) * 0x55)
                        .cast::<i8>();
                    let code: i32;
                    let call: bool;
                    if rc(p25, gs!(DAT_00523c18), 0xf)
                        || rc(p25, gs!(DAT_00524f58), 0x13)
                        || rc(p25, gs!(DAT_00524f30), 0x12)
                        || rc(p25, gs!(DAT_00524f44), 0x11)
                        || rc(p25, gs!(DAT_00524f1c), 0x11)
                    {
                        code = 0x25;
                        call = true;
                    } else if rc(p25, gs!(DAT_00524f90), 0x12)
                        || rc(p25, gs!(DAT_00524f6c), 0xf)
                        || rc(p25, gs!(DAT_00524f7c), 0x12)
                    {
                        code = 0x27;
                        call = true;
                    } else {
                        code = 0;
                        call = false;
                    }
                    if call {
                        FUN_0040c070(1, code, DAT_006d9d50 as u32, 0xffff);
                    }
                }
                let u_var11 = FUN_0040c070(1, 0x1e, u_var5 as u32, 0xffff);
                return u_var11 & 0xffff0000;
            }
            2 => 'c2: {
                let mut b24: bool = ld::<i32>(p56().add(0x36)) == 0;
                if !b24 {
                    let p25 = ga!(DAT_00703ca0)
                        .add((*p56().add(0xb) as usize) * 0x55)
                        .cast::<i8>();
                    b24 = rc(p25, gs!(DAT_00523c18), 0xf);
                    pb9 = core::ptr::null_mut();
                    if b24 {
                        pb9 = d49();
                        if ld::<i32>(d49().add(i_var14 + 0x1b)) != -1 {
                            pb9 = p56();
                            if ld::<i16>(p56().add(0x3a)) == 0 {
                                let mut u_var11: u32 = 0;
                                b24 = false;
                                if *pb19.add(0x228) != 0 {
                                    loop {
                                        if *pb19.add(u_var11 as usize + 0x229) != 0 {
                                            let base = (local_48 as u32).wrapping_mul(0xef);
                                            i_var15 = base.wrapping_add(u_var11) as i32;
                                            let bound = *ga!(DAT_0052b0b6)
                                                .add(u_var11.wrapping_add(base) as usize);
                                            u_var5 = *ga!(DAT_0052b0b4).add(i_var15 as u32 as usize)
                                                as u16;
                                            while u_var5 <= bound as u16 {
                                                let mut u10: u32 = *ga!(DAT_0070ad45)
                                                    .add(d16(i_var14) as usize * 0x566)
                                                    as u32;
                                                if (u_var11 as i16) == 1 {
                                                    u10 = 5;
                                                }
                                                u10 = FUN_00478320(
                                                    c31(
                                                        d16(i_var14).wrapping_mul(0x2b3) >> 8,
                                                        *ga!(DAT_0052b0ba)
                                                            .add(i_var15 as u32 as usize),
                                                    ) & 0xffff00ff,
                                                    ld::<u16>(
                                                        ga!(DAT_00524880)
                                                            .add(u10.wrapping_mul(2) as usize),
                                                    ),
                                                );
                                                if (u10 as i16) == 0 {
                                                    u10 = 1;
                                                }
                                                if *ga!(DAT_0070ad44)
                                                    .add(d16(i_var14) as usize * 0x566)
                                                    == 1
                                                {
                                                    u10 = (u10 as u16).wrapping_mul(2) as u32;
                                                } else if *ga!(DAT_0070ad44)
                                                    .add(d16(i_var14) as usize * 0x566)
                                                    == 2
                                                {
                                                    u10 = ((u10 as u16) >> 1) as u32;
                                                }
                                                if (u10 as i16) == 0 {
                                                    u10 = 1;
                                                }
                                                if 99 < (u10 as u16) {
                                                    u10 = 99;
                                                }
                                                let u16v = u_var5 as u32;
                                                let b2 = *pb19.add(u16v as usize * 4 + 0x346);
                                                local_38 = b2 as usize as *mut u32;
                                                if (b2 as u32) < (u10 & 0xffff) {
                                                    b24 = true;
                                                    *pb19.add(u16v as usize * 4 + 0x346) =
                                                        b2.wrapping_add(1);
                                                }
                                                *pb19.add(u16v as usize * 4 + 0x345) = 0x7f;
                                                u_var5 = u_var5.wrapping_add(1);
                                            }
                                        }
                                        u_var11 = (u_var11 as u16).wrapping_add(1) as u32;
                                        if !(u_var11 < (*pb19.add(0x228) as u32)) {
                                            break;
                                        }
                                    }
                                }
                                if *pb19.add(0x23c) != 0 {
                                    *pb19.add(0x23d) = *ga!(DAT_0052b13c)
                                        .add((local_48 as u32).wrapping_mul(0xef) as usize);
                                }
                                u_var5 = 1;
                                u_var6 = 0;
                                loop {
                                    if (ld::<u16>(pb19.add(0x1dd)) & u_var5) != 0
                                        && (ld::<u16>(pb19.add(0x1df)) & u_var5) == 0
                                    {
                                        b24 = true;
                                        st::<u16>(
                                            pb19.add(0x1df),
                                            ld::<u16>(pb19.add(0x1df)) | u_var5,
                                        );
                                        break;
                                    }
                                    u_var5 = u_var5.wrapping_mul(2);
                                    u_var6 = u_var6.wrapping_add(1);
                                    if !(u_var6 < 10) {
                                        break;
                                    }
                                }
                                pb9 = p56();
                                pb13 = p56();
                                *pb13.add(0x3a) = 0xd8;
                                pb4 = p56();
                                *pb13.add(0x3b) = 1;
                                if b24 {
                                    pb13 = p56();
                                    *pb13.add(0x36) = 0x88;
                                    *pb13.add(0x37) = 5;
                                    *pb13.add(0x38) = 0;
                                    *pb13.add(0x39) = 0;
                                    return pu(pb4) & 0xffff0000;
                                }
                            }
                        }
                    }
                    break 'c2;
                }
                let p25 = ga!(DAT_00703ca0)
                    .add((*p56().add(0xb) as usize) * 0x55)
                    .cast::<i8>();
                'l63b9: {
                    if rc(p25, gs!(DAT_00523c18), 0xf) {
                        if ld::<i32>(d49().add(i_var14 + 0x1f)) != 0 {
                            u_var6 = 0;
                            loop {
                                let u7 = u_var6.wrapping_add(1);
                                *pb19.add(u_var6 as usize + 0x1fb) =
                                    *e54().add(u_var6 as usize + 0x1fb);
                                u_var6 = u7;
                                if !(u7 < 0x10) {
                                    break;
                                }
                            }
                            *pb19.add(0x1fa) = 2;
                        }
                        local_38 = *ga!(DAT_0070ad95).add(
                            (DAT_006d9d60 as u32).wrapping_mul(0x566) as usize
                                + (*p56() as usize) * 0x52,
                        ) as usize as *mut u32;
                        if (local_38 as usize) <= (*p56().add(*p56() as usize + 5) as usize + 1) {
                            *e54().add(0x1fb) = 0;
                        }
                        *e54().add(0x1fa) = 1;
                        pb13 = d49();
                        if pu(ldp(d49().add(i_var14 + 0x1b))) != 0xffffffff {
                            pb13 = DAT_0071ecf4;
                            if DAT_0071ecf4 == ldp(d49().add(i_var14 + 0x1b)) {
                                i_var15 = ld::<i32>(
                                    ga!(DAT_0071b730)
                                        .add(pu(DAT_0071ecf4).wrapping_mul(0x5bd) as usize),
                                );
                                let ok: bool;
                                if i_var15 == -1 {
                                    ok = false;
                                } else {
                                    pb13 = up((i_var15 as u32).wrapping_mul(7));
                                    b_var12 = *d49()
                                        .add((i_var15 as u32).wrapping_mul(0x23) as usize + 3);
                                    ok = !(b_var12 != 1
                                        && b_var12 != 2
                                        && b_var12 != 3
                                        && b_var12 != 0xe
                                        && b_var12 != 4);
                                }
                                if ok {
                                    u_var6 = 1;
                                    let mut s_var8: i16 = 0xd;
                                    loop {
                                        if (ld::<u16>(pb19.add(0x1d9)) & u_var6) != 0
                                            && (ld::<u16>(pb19.add(0x1db)) & u_var6) == 0
                                        {
                                            st::<u16>(
                                                pb19.add(0x1db),
                                                u_var6 | ld::<u16>(pb19.add(0x1db)),
                                            );
                                        }
                                        u_var6 = u_var6.wrapping_mul(2);
                                        s_var8 = s_var8.wrapping_sub(1);
                                        if s_var8 == 0 {
                                            break;
                                        }
                                    }
                                    FUN_004a6a80();
                                    FUN_004b1c00(
                                        *ga!(DAT_0071bb37)
                                            .add(pu(DAT_0071ecf4).wrapping_mul(0x5bd) as usize)
                                            as u32,
                                        pu(DAT_0071ecf4) as i32,
                                    );
                                    FUN_004a6f90();
                                    pb13 = extraout_ecx;
                                }
                            }
                            pb13 = up(pu(pb13) & 0xffff0000);
                            loop {
                                let u11 = pu(pb13) & 0xffff;
                                *pb19.add(u11 as usize + 0x293) = pu(pb13) as u8;
                                let u6 = (pu(pb13) as u16).wrapping_add(1);
                                pb13 = up(c22((pu(pb13) >> 16) as u16, u6));
                                st::<u16>(pb19.add(u11 as usize * 2 + 0x29e), 100);
                                st::<u16>(pb19.add(u11 as usize * 2 + 0x2b4), 0);
                                u_var6 = u6;
                                if !(u6 < 10) {
                                    break;
                                }
                            }
                        }
                        FUN_004a34c0(
                            c22((pu(pb13) >> 16) as u16, DAT_006d9d50),
                            DAT_00732e54.cast::<u32>(),
                            0x9b,
                        );
                        FUN_0040c070(1, 0x26, DAT_006d9d50 as u32, 0xffff);
                    } else if rc(p25, gs!(DAT_00524f6c), 0xf) {
                        let mut u17: u16 = 0;
                        if ld::<i32>(d49().add(i_var14 + 0x1f)) != 0 {
                            i_var15 = ld::<i32>(d49().add(i_var14 + 0x1f));
                            u_var6 = 0;
                            loop {
                                let u7 = u_var6.wrapping_add(1);
                                b_var12 = *pb19.add(u_var6 as usize + 0x1fb);
                                u17 = ((i_var15 as u32) >> 16) as u16;
                                *e54().add(u_var6 as usize + 0x1fb) = b_var12;
                                i_var15 = c31((i_var15 as u32) >> 8, b_var12) as i32;
                                u_var6 = u7;
                                if !(u7 < 0x10) {
                                    break;
                                }
                            }
                            *pb19.add(0x1fb) = 0;
                            *pb19.add(0x1fa) = 1;
                        }
                        *e54().add(0x1fa) = 2;
                        FUN_004a34c0(c22(u17, DAT_006d9d50), DAT_00732e54.cast::<u32>(), 0x9b);
                        FUN_0040c070(1, 0x28, DAT_006d9d50 as u32, 0xffff);
                    } else if rc(p25, gs!(DAT_00524f58), 0x13) {
                        let mut u17: u16 = 0;
                        if ld::<i32>(d49().add(i_var14 + 0x1f)) != 0 {
                            i_var15 = ld::<i32>(d49().add(i_var14 + 0x1f));
                            u_var6 = 0;
                            loop {
                                let u7 = u_var6.wrapping_add(1);
                                let u11 = u_var6 as usize;
                                b_var12 = *pb19.add(u11 + 0x1fb);
                                u17 = ((i_var15 as u32) >> 16) as u16;
                                let b1 = *e54().add(u11 + 0x1fb);
                                *e54().add(u11 + 0x1fb) = b_var12;
                                *pb19.add(u11 + 0x1fb) = b1;
                                i_var15 = c31((i_var15 as u32) >> 8, b_var12) as i32;
                                u_var6 = u7;
                                if !(u7 < 0x10) {
                                    break;
                                }
                            }
                            *pb19.add(0x1fa) = 2;
                        }
                        *e54().add(0x1fa) = 2;
                        FUN_004a34c0(c22(u17, DAT_006d9d50), DAT_00732e54.cast::<u32>(), 0x9b);
                    } else if !rc(p25, gs!(DAT_00524f90), 0x12) {
                        if rc(p25, gs!(DAT_00524f7c), 0x12) {
                            if ld::<i32>(d49().add(i_var14 + 0x1f)) != 0 {
                                FUN_004a01a0(
                                    c22((pu(d49()) >> 16) as u16, DAT_006d9d50),
                                    c22(u_var5, u_var5),
                                );
                                i_var15 = ld::<i32>(d49().add(i_var14 + 0x1f));
                                if ld::<i16>(ip(i_var15).add(0x7f)) != 0 {
                                    u_var6 = FUN_00478470() as u16;
                                    st::<i16>(
                                        ip(i_var15).add(0x7f),
                                        (((u_var6 as i32 & 0xf) + 0xf).wrapping_mul(0xec)) as i16,
                                    );
                                }
                                i_var15 = FUN_00481380(gs!(DAT_00528360));
                                *local_38.cast::<u8>().add(10) = i_var15 as u8;
                                swap_call(puvar19, u_var5);
                            }
                            FUN_0040c070(1, 0x28, DAT_006d9d50 as u32, 0xffff);
                        } else if rc(p25, gs!(DAT_00524f44), 0x11) {
                            if ld::<i32>(d49().add(i_var14 + 0x1f)) == 0 {
                                inc16(ga!(DAT_006d4a76).add(d16(i_var14) as usize * 0x126));
                                *pb13.add(i_var14 + 3) = 0;
                            } else {
                                FUN_00496770(u_var5 as u32, pu(pb19) as i32, 0x80);
                                st::<u16>(e54().add(0x66), u_var5);
                                st::<u16>(pb19.add(0x68), DAT_006d9d50);
                            }
                            FUN_0040c070(1, 0x26, DAT_006d9d50 as u32, 0xffff);
                        } else if rc(p25, gs!(DAT_00524f30), 0x12) {
                            i_var15 = ld::<i32>(d49().add(i_var14 + 0x1f));
                            if i_var15 != 0 {
                                *pb19.add(0x1fa) = 2;
                            }
                            FUN_004a34c0(
                                c22(((i_var15 as u32) >> 16) as u16, DAT_006d9d50),
                                DAT_00732e54.cast::<u32>(),
                                0x9b,
                            );
                            FUN_0040c070(1, 0x28, DAT_006d9d50 as u32, 0xffff);
                        } else if rc(p25, gs!(DAT_00524f1c), 0x11) {
                            if ld::<i32>(d49().add(i_var14 + 0x1f)) != 0 {
                                st::<u16>(pb19.add(0x1df), ld::<u16>(pb19.add(0x1dd)));
                                st::<u16>(pb19.add(0xc9), 0);
                                *pb19.add(7) = 0;
                            }
                            FUN_004a34c0(DAT_006d9d50 as u32, DAT_00732e54.cast::<u32>(), 0x9b);
                            FUN_0040c070(1, 0x28, DAT_006d9d50 as u32, 0xffff);
                            FUN_0040c070(1, 0x1d, u_var5 as u32, 0xffff);
                        } else {
                            break 'l63b9;
                        }
                    } else {
                        if ld::<i32>(d49().add(i_var14 + 0x1f)) != 0 {
                            FUN_00496770(u_var5 as u32, pu(pb19) as i32, 0x80);
                            if *pb19.add(0x1e5) == 0 {
                                if *pb19.add(5) as i8 == -1 {
                                    b_var12 = *ga!(DAT_00524798)
                                        .add(*ga!(DAT_0070ad92).add((local_44 as usize) * 0x566)
                                            as usize);
                                } else {
                                    b_var12 = *ga!(DAT_005247c0)
                                        .add(*ga!(DAT_0070ad92).add((local_44 as usize) * 0x566)
                                            as usize);
                                }
                                *local_38.cast::<u8>().add(10) =
                                    *ga!(DAT_006da3e0).add(b_var12 as usize);
                            } else {
                                let p: *mut i8;
                                if ld::<i16>(pb19.add(0x92)) == 0 {
                                    p = gs!(DAT_00524934);
                                } else {
                                    p = gs!(DAT_00525b50);
                                }
                                i_var15 = FUN_00481380(p);
                                *local_38.cast::<u8>().add(10) = i_var15 as u8;
                                FUN_004a34c0(c22(u_var5, u_var5), puvar19, 0x8a);
                            }
                            FUN_0040c070(1, 0x28, DAT_006d9d50 as u32, 0xffff);
                            FUN_0040c070(1, 0x1f, u_var5 as u32, 0xffff);
                            swap_call(puvar19, u_var5);
                        }
                    }
                }
                u_var6 = 0;
                if ld::<i32>(d49().add(i_var14 + 0x1f)) != 0 {
                    if *ip(ld::<i32>(d49().add(i_var14 + 0x1f))).add(0x86)
                        == *ip(ld::<i32>(d49().add(ent() + 0x1f))).add(0x86)
                    {
                        let u11 = *ip(ld::<i32>(d49().add(ent() + 0x1f))).add(0x87) as u32;
                        if *pb19.add(u11 as usize + 0x1f0) == 0 {
                            b_var12 = 0;
                            loop {
                                let b1 = *pb19.add(u_var6 as usize + 0x1f0);
                                if b_var12 < b1 {
                                    b_var12 = b1;
                                }
                                u_var6 = u_var6.wrapping_add(1);
                                if !(u_var6 < 10) {
                                    break;
                                }
                            }
                            *pb19.add(u11 as usize + 0x1f0) = b_var12.wrapping_add(1);
                            pb13 = d49().add(i_var14 + 0x16);
                            inc16(ga!(DAT_006d4a7a).add(d16(i_var14) as usize * 0x126));
                            if (*ga!(DAT_0070ad40).add(*pb13 as usize * 0x566) as i32)
                                == (*pb19.add(0xb0) as i8 as i32)
                            {
                                *ga!(DAT_006d4aaa).add(*pb13 as usize * 0x126) = 1;
                            }
                        }
                    } else {
                        FUN_0040a1a0(0x24, -1, pu(DAT_0071ecf4) as i32);
                    }
                }
                let idx = *p56() as usize + 5;
                *p56().add(idx) = (*p56().add(idx)).wrapping_add(1);
                st::<u16>(e54().add(*e54().add(0x7d) as usize * 2 + 0x7e), u_var3);
                *e54().add(0x7d) = (*e54().add(0x7d)).wrapping_add(1);
                if 9 < *e54().add(0x7d) {
                    *e54().add(0x7d) = (*e54().add(0x7d)).wrapping_sub(1);
                }
                if *e54().add(0x7d) == 1 {
                    let u11 = DAT_006d9d60 as u32;
                    let i15 = u11.wrapping_mul(0x126) as usize;
                    inc16(ga!(DAT_006d4a82).add(i15));
                    if *ga!(DAT_0070ad40).add(u11.wrapping_mul(0x566) as usize) == *e54().add(0xb0)
                    {
                        *ga!(DAT_006d4aae).add(i15) = 1;
                    }
                }
                if ld::<i32>(d49().add(i_var14 + 0x1f)) != 0 && *pb19.add(0x7b) == 0 {
                    let i15 = (local_44 as usize) * 0x126;
                    inc16(ga!(DAT_006d4a7e).add(i15));
                    if (*ga!(DAT_0070ad40).add((local_44 as usize) * 0x566) as i32)
                        == (*pb19.add(0xb0) as i8 as i32)
                    {
                        *ga!(DAT_006d4aac).add(i15) = 1;
                    }
                }
                *p56().add(0x35) = 3;
                pb13 = p56();
                *pb13.add(0x36) = 0x38;
                pb4 = p56();
                *pb13.add(0x37) = 9;
                *pb13.add(0x38) = 0;
                *pb13.add(0x39) = 0;
                if ld::<u16>(ga!(DAT_0071b74a).add(pu(DAT_0071ecf4).wrapping_mul(0x5bd) as usize))
                    == u_var5
                {
                    DAT_00655a84 = -3;
                }
                pb9 = d49();
                if ld::<i32>(d49().add(i_var14 + 0x1b)) != -1 {
                    pb13 = p56();
                    *pb13.add(0x2b) = 0xff;
                    *pb13.add(0x2c) = 0xff;
                    return pu(pb4) & 0xffff0000;
                }
            }
            3 => {
                if ld::<i32>(p56().add(0x36)) == 0 {
                    pb13 = p56();
                    *pb13.add(0x18) = 0xff;
                    *pb13.add(0x19) = 0xff;
                    pb13 = p56();
                    *pb13.add(0x1a) = 0;
                    pb9 = p56();
                    *pb13.add(0x1b) = 0;
                    *p56().add(0x1e) = 0;
                    return c22((pu(pb9) >> 16) as u16, 1);
                }
                if ld::<i32>(d49().add(i_var14 + 0x1f)) != 0 {
                    FUN_0042a230(
                        pu(d49()) as i32,
                        i_var15 as u32,
                        pu(d49().add(ent())) as i32,
                        0,
                        0x4000,
                        0,
                    );
                    st::<i32>(
                        e54().add(0xb1),
                        (ld::<i32>(d49().add(i_var14 + 4)) as u32)
                            .wrapping_sub(ld::<i32>(d49().add(ent() + 4)) as u32)
                            .wrapping_add(DAT_0070ab18 as u32) as i32,
                    );
                    st::<i32>(
                        e54().add(0xb5),
                        (ld::<i32>(d49().add(i_var14 + 8)) as u32)
                            .wrapping_sub(ld::<i32>(d49().add(ent() + 8)) as u32)
                            .wrapping_add(DAT_0070ab14 as u32) as i32,
                    );
                    let u_var11 = (DAT_0070ac20 as u32) & 0xffff0000;
                    st::<i32>(
                        e54().add(0xb9),
                        (ld::<i32>(d49().add(i_var14 + 0xc)) as u32)
                            .wrapping_sub(ld::<i32>(d49().add(ent() + 0xc)) as u32)
                            .wrapping_add(DAT_0070ac20 as u32) as i32,
                    );
                    return u_var11;
                }
                pb13 = e54();
                for k in 0xb1usize..0xb5 {
                    *pb13.add(k) = 0;
                }
                pb13 = e54();
                *pb13.add(0xb5) = 0;
                pb9 = e54();
                *pb13.add(0xb6) = 0;
                *pb13.add(0xb7) = 0;
                *pb13.add(0xb8) = 0;
                pb13 = e54();
                *pb13.add(0xb9) = 0xf4;
                *pb13.add(0xba) = 1;
                *pb13.add(0xbb) = 0;
                *pb13.add(0xbc) = 0;
            }
            _ => {}
        }
        pu(pb9) & 0xffff0000
    }
}
