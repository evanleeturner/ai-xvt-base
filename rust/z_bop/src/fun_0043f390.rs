// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009ec464: i8;
    static mut DAT_00a09354: i16;
    static mut DAT_00a0910a: i16;
    static mut DAT_00a09108: i16;
    static mut DAT_00a0935c: i16;
    static mut DAT_00a09356: i16;
    static mut DAT_00a09110: i16;
    static mut DAT_00a0935e: i16;
    static mut DAT_00a0933c: i16;
    static mut DAT_00a0933e: i16;
    static mut DAT_009a20ae: u8;
    static mut DAT_00a08c74: u16;
    static mut DAT_00a0a29e: i16;
    static mut DAT_00a0a2a0: i16;
    static mut DAT_00a0a2a2: u16;
    static mut DAT_00a0a2b0: u16;
    static mut DAT_00a0a2b2: u16;
    static mut DAT_00a0a2b4: u16;
    static mut DAT_00a0a2b6: u16;
    static mut DAT_009d1300: *const core::ffi::c_void;
    static mut DAT_009a6fe4: *const core::ffi::c_void;
    static mut DAT_009ecc34: i32;
    static mut DAT_009a1fe0: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_009a2062: i8;
    static mut DAT_00a080f8: i32;
    static mut DAT_009d6959: i8;
    static mut DAT_009e968a: u8;
    static mut DAT_009e967c: u8;
    static mut DAT_009d8c88: [u8; 1];
    static mut DAT_009d8c94: [u8; 1];
    static mut DAT_009fd458: [i8; 1];
    static mut DAT_009a1ed0: u8;
    static mut DAT_009a8c00: u32;
    static mut DAT_00a09122: i16;
    static mut DAT_00a09120: i16;
    static mut DAT_00a09116: i16;
    static mut DAT_00a09114: i16;
    static mut DAT_00a0911c: i16;
    static mut DAT_00a09182: i16;
    static mut DAT_00a09180: i16;
    static mut DAT_00a09188: i16;
    static mut DAT_00a08102: i16;
    static mut DAT_00a0918c: i16;
    static mut DAT_00a0918e: i16;
    static mut DAT_00a09194: i16;
    static mut DAT_00a09198: i16;
    static mut DAT_00a0919a: i16;
    static mut DAT_00a091a0: i16;
    static mut DAT_00a091a4: i16;
    static mut DAT_00a091a6: i16;
    static mut DAT_00a091ac: i16;
    static mut DAT_00a0a0e0: *mut u8;
    static mut DAT_00a0a0f4: *mut u8;
    static mut DAT_00a607ac: *mut u8;
    static mut DAT_00a0a0fc: *mut u8;
    static mut DAT_00a0a100: *mut u8;
    static mut DAT_00a0a114: *mut i8;
    static mut DAT_00a0a118: *mut i8;
    static mut DAT_00a0a11c: *mut i8;
    static mut DAT_00a0a104: *mut i8;
    static mut DAT_00a0a108: *mut i8;
    static mut DAT_00a0a10c: *mut i8;
    static mut DAT_00a0a110: *mut i8;
    static mut DAT_005233ec: i32;
    static mut DAT_005272f8: [u8; 1];
    static mut DAT_009a1840: [*mut u8; 1];
    static mut DAT_009d1320: i8;
    static mut DAT_0051a2bc: i8;
    static mut DAT_00521690: i8;
    static mut DAT_005216b0: [i8; 1];
    static mut DAT_005216a4: [i8; 1];
    static mut DAT_0052169c: [i8; 1];
    static mut DAT_0051a368: [i8; 1];
    static mut DAT_00521648: [i8; 1];
    static mut DAT_00521638: [i8; 1];
    static mut DAT_00521628: [i8; 1];
    static mut DAT_0051bfc0: [i8; 1];
    static mut DAT_0051bfd4: [i8; 1];
    static mut DAT_0051bff8: [i8; 1];
    static mut DAT_00521614: [i8; 1];
    static mut DAT_0051bfe4: [i8; 1];
    static mut DAT_005215fc: [i8; 1];
    static mut DAT_005215e4: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004a95c0(a: i32);
    fn FUN_004a9640(a: i8);
    fn FUN_004a9520(a: i16, b: i16, c: i16, d: i16);
    fn FUN_004a9500(a: i16, b: i16);
    fn FUN_004a9630(a: i32);
    fn FUN_0043c290(a: u16, b: u8);
    fn FUN_004a9a50(a: *mut u8);
    fn FUN_004a9590(a: u32);
    fn FUN_004a9960(a: *mut u8);
    fn FUN_00485000(a: i32, b: u32);
    fn FUN_004278c0(a: *mut u8) -> u64;
    fn _sprintf(a: *mut i8, b: *const i8, ...) -> i32;
    fn FUN_0046b610(a: *mut i8) -> u32;
    fn FUN_004a97f0(a: *mut i8);
    fn FUN_004034e0(a: u32, b: u32);
    fn FUN_0046a8d0(a: i32, b: i32, c: i32);
    fn FUN_004277f0(a: u16, b: i32, c: i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(a: u32, b: u32, c: u32, d: *const u32);
}

unsafe fn rd<T>(a: u32) -> T {
    // SAFETY: the address is the game's own memory, read where the original reads.
    unsafe { (a as *const T).read_unaligned() }
}

unsafe fn slot(base: *mut u8) -> u32 {
    // SAFETY: reads the game's global slot index.
    unsafe { (base as u32).wrapping_add((DAT_009ecc34 as u32).wrapping_mul(0x5bd)) }
}

unsafe fn call_d1300() {
    // SAFETY: the global holds a void(void) cdecl function pointer.
    unsafe {
        let f: unsafe extern "C" fn() = core::mem::transmute(DAT_009d1300);
        f();
    }
}

unsafe fn call_6fe4(a: i32) {
    // SAFETY: the global holds a void(int) cdecl function pointer.
    unsafe {
        let f: unsafe extern "C" fn(i32) = core::mem::transmute(DAT_009a6fe4);
        f(a);
    }
}

unsafe fn cmp(mut a: *const i8, mut b: *const i8, mut n: i32) -> bool {
    let mut eq = true;
    // SAFETY: compares game strings like repe cmpsb.
    unsafe {
        loop {
            if n == 0 {
                break;
            }
            n -= 1;
            eq = *a == *b;
            a = a.add(1);
            b = b.add(1);
            if !eq {
                break;
            }
        }
    }
    eq
}

unsafe fn udiv(lo: u32, d: u32) -> u32 {
    let q: u32;
    // SAFETY: the machine's own div, faulting on a zero divisor.
    unsafe {
        core::arch::asm!("div {d:e}", d = in(reg) d, inout("eax") lo => q, inout("edx") 0u32 => _);
    }
    q
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0043f390() {
    // SAFETY: everything touched is the game's own memory and functions.
    unsafe {
        let mut s_var1: i16;
        let mut s_var11: i16;
        let mut s_var3: i16;
        let mut i_var4: i32 = 0;
        let mut u_var5: u32 = 0;
        let mut u_var6: u32;
        let mut i_var7: i32;
        let mut i_var8: i32 = 0;
        let mut u_var9: u16;
        let mut u_var10: u16;
        let mut pbvar12: *mut u8;
        let mut pc17: *mut i8;
        let mut u_stack_10: u16 = 0;
        let mut u_stack_e: u16 = 0;
        let mut local_8: u16;

        DAT_009ec464 = 0;
        FUN_004a95c0(0x2c);
        FUN_004a9640(2);
        s_var3 = DAT_00a09354;
        s_var11 = DAT_00a0910a;
        s_var1 = DAT_00a09108;
        u_var10 = DAT_00a08c74;
        local_8 = u_var10;
        let s_var2: i16 = DAT_00a0935c.wrapping_add(DAT_00a09354);
        u_var9 = rd(slot(&raw mut DAT_009e968a));
        if u_var9 != DAT_00a08c74 {
            DAT_00a0a29e = -1;
            DAT_00a0a2a0 = -1;
            DAT_00a0a2a2 = 0xffff;
            DAT_00a0a2b0 = 0xffff;
            DAT_00a0a2b2 = 0xfffe;
            DAT_00a0a2b4 = 0xfffe;
            DAT_00a0a2b6 = 0xffff;
            DAT_00a08c74 = u_var9;
            FUN_004a9520(
                DAT_00a09108,
                DAT_00a0910a,
                DAT_00a09110.wrapping_add(DAT_00a09108),
                (DAT_009a20ae as i16)
                    .wrapping_add(DAT_00a0910a)
                    .wrapping_add(1),
            );
            call_d1300();
            FUN_004a9500(s_var1, s_var11);
            FUN_004a9630(1);
            u_var9 = rd(slot(&raw mut DAT_009e968a));
            if u_var9 != 0xffff {
                i_var4 = ((u_var9 as u32)
                    .wrapping_mul(0x23)
                    .wrapping_add(DAT_009a1fe8 as u32)) as i32;
                i_var7 = rd((i_var4 as u32).wrapping_add(0x1f));
                if i_var7 == 0 {
                    i_var8 = (((u_stack_e as u32) << 16) | u_stack_10 as u32) as i32;
                } else {
                    i_var8 = rd((i_var7 as u32).wrapping_add(0xa9));
                }
                let b_var18: u8;
                if DAT_009a2062 == 2 && 1 < DAT_00a080f8 {
                    let mut b = 3u8;
                    if i_var7 != 0 && i_var8 != 0 && DAT_009d6959 == 0 {
                        let t16: u16 = rd(slot(&raw mut DAT_009e967c));
                        let u6 = t16 as u32;
                        if *(u6.wrapping_add(0x1f6).wrapping_add(i_var8 as u32) as *const i8) == 0 {
                            let i7 = (*((i_var4 as u32).wrapping_add(0x16) as *const u8) as u32)
                                .wrapping_mul(0x566);
                            let d88 = (&raw mut DAT_009d8c88).cast::<u8>();
                            let v = d88.add(i7 as usize).read();
                            let b15: bool = if v as u32 == u6 {
                                false
                            } else {
                                (&raw mut DAT_009fd458)
                                    .cast::<i8>()
                                    .add((v as u32).wrapping_add(u6.wrapping_mul(0x1e5)) as usize)
                                    .read()
                                    == 0
                            };
                            if b15
                                && (&raw mut DAT_009d8c94).cast::<u8>().add(i7 as usize).read() != 0
                            {
                                b = 1;
                            }
                        }
                    }
                    b_var18 = b;
                } else {
                    b_var18 = 3;
                }
                FUN_0043c290(u_var9, b_var18);
                FUN_004a9a50(&raw mut DAT_009a1ed0);
            }
            local_8 = u_var10;
            if DAT_009a1fe0 <= DAT_00a08c74 as i32 {
                FUN_004a9520(
                    s_var3,
                    DAT_00a09356,
                    s_var2,
                    DAT_00a09356.wrapping_add(DAT_00a0935e),
                );
                call_d1300();
            }
        }
        s_var11 = DAT_00a09122;
        s_var1 = DAT_00a09120;
        let t16: u16 = rd(slot(&raw mut DAT_009e968a));
        if (t16 as i16) != -1 {
            if local_8 == 0xffff {
                FUN_004a9500(DAT_00a0933c, DAT_00a0933e);
                FUN_004a9590(0x49);
                FUN_004a9960(DAT_00a0a0e0);
            }
            FUN_00485000(DAT_009ecc34, DAT_00a08c74 as u32);
            s_var3 = (DAT_009a20ae as i16).wrapping_add(s_var11).wrapping_add(1);
            let u16v = FUN_004278c0((&raw mut DAT_005216b0).cast::<u8>());
            FUN_004a9520(s_var1, s_var11, (u16v as i16).wrapping_add(s_var1), s_var3);
            FUN_004a9590(0x4a);
            DAT_009a8c00 = DAT_009a8c00.wrapping_mul(0xa1);
            u_var6 = DAT_009a8c00 >> 0x10;
            if 9999 < (u_var6 as u16) {
                u_var6 = 9999;
            }
            u_var5 = (u_var6 & 0xffff) / 100;
            s_var3 = u_var5 as i16;
            u_var6 = ((s_var3.wrapping_mul(-100)) as u16 as u32).wrapping_add(u_var6);
            u_var9 = u_var6 as u16;
            if s_var3 != DAT_00a0a2a0 || u_var9 != DAT_00a0a2a2 {
                DAT_00a0a2a0 = s_var3;
                DAT_00a0a2a2 = u_var9;
                FUN_004a9500(s_var1, s_var11);
                call_d1300();
                let fmt: *const i8 = if u_var9 < 10 {
                    (&raw mut DAT_005216a4).cast::<i8>()
                } else {
                    (&raw mut DAT_0052169c).cast::<i8>()
                };
                _sprintf(
                    (&raw mut DAT_009a1ed0).cast::<i8>(),
                    fmt,
                    u_var5,
                    u_var6 & 0xffff,
                );
                FUN_004a9960(&raw mut DAT_009a1ed0);
            }
            s_var11 = DAT_00a09116;
            s_var1 = DAT_00a09114;
            s_var3 = 2;
            let t16: u16 = rd(slot(&raw mut DAT_009e968a));
            u_var6 = t16 as u32;
            pbvar12 = DAT_00a0a0f4;
            if (u_var6 as i32) < DAT_009a1fe0 {
                let t32: u32 = rd(u_var6
                    .wrapping_mul(0x23)
                    .wrapping_add(0x1f)
                    .wrapping_add(DAT_009a1fe8 as u32));
                pc17 = t32 as *mut i8;
                if *pc17 == 0 {
                    i_var7 = rd((t32).wrapping_add(0xa9));
                    let t16b: u16 = rd(slot(&raw mut DAT_009e967c));
                    if *((t16b as u32)
                        .wrapping_add(0x1f6)
                        .wrapping_add(i_var7 as u32) as *const i8)
                        == 0
                    {
                        s_var3 = 0;
                        pbvar12 = DAT_00a607ac;
                    } else {
                        s_var3 = 1;
                        pbvar12 = (i_var7 as u32).wrapping_add(0x201) as *mut u8;
                        if *pbvar12 == 0 {
                            s_var3 = 2;
                            pbvar12 = DAT_00a0a0f4;
                        }
                    }
                }
            }
            if s_var3 != DAT_00a0a29e {
                DAT_00a0a29e = s_var3;
                FUN_004a9520(
                    DAT_00a09114,
                    DAT_00a09116,
                    DAT_00a0911c.wrapping_add(DAT_00a09114),
                    (DAT_009a20ae as i16)
                        .wrapping_add(DAT_00a09116)
                        .wrapping_add(1),
                );
                call_d1300();
                FUN_004a9500(s_var1, s_var11);
                FUN_004a9590(0x46);
                FUN_004a9960(pbvar12);
            }
        }
        u_var9 = rd(slot(&raw mut DAT_009e968a));
        u_var6 = u_var9 as u32;
        if DAT_009a1fe0 <= u_var6 as i32 {
            return;
        }
        if u_var9 == 0xffff {
            return;
        }
        i_var8 = u_var6.wrapping_mul(0x23) as i32;
        i_var7 = rd((DAT_009a1fe8 as u32)
            .wrapping_add(i_var8 as u32)
            .wrapping_add(0x1f));
        i_var4 = rd((i_var7 as u32).wrapping_add(0xa9));
        if DAT_009a2062 == 2 && 1 < DAT_00a080f8 && i_var7 != 0 && i_var4 != 0 && DAT_009d6959 == 0
        {
            let t16: u16 = rd(slot(&raw mut DAT_009e967c));
            let u5 = t16 as u32;
            if *(u5.wrapping_add(0x1f6).wrapping_add(i_var4 as u32) as *const i8) == 0 {
                let mut b15 = false;
                let idx = (*((DAT_009a1fe8 as u32)
                    .wrapping_add(i_var8 as u32)
                    .wrapping_add(0x16) as *const u8) as u32)
                    .wrapping_mul(0x566);
                let v = (&raw mut DAT_009d8c88)
                    .cast::<u8>()
                    .add(idx as usize)
                    .read();
                if u5 != v as u32 {
                    b15 = (&raw mut DAT_009fd458)
                        .cast::<i8>()
                        .add((v as u32).wrapping_add(u5.wrapping_mul(0x1e5)) as usize)
                        .read()
                        == 0;
                }
                if b15 {
                    return;
                }
            }
        }
        u_var5 = *((i_var4 as u32).wrapping_add(0x34) as *const u8) as u32;
        let t16: u16 = rd((i_var4 as u32).wrapping_add(0x1e5));
        'lab: {
            let mut p: *mut i8;
            if t16 == 0 {
                p = (&raw mut DAT_0051a368).cast::<i8>();
            } else {
                let t16b: u16 = rd((i_var7 as u32).wrapping_add(0x79));
                if t16b != 0 {
                    break 'lab;
                }
                p = (&raw mut DAT_009d1320)
                    .cast::<i8>()
                    .add(u_var5.wrapping_mul(0x55) as usize);
                let matched = cmp(p, (&raw mut DAT_00521648).cast::<i8>(), 0xb)
                    || cmp(p, (&raw mut DAT_00521638).cast::<i8>(), 0xe)
                    || cmp(p, (&raw mut DAT_00521628).cast::<i8>(), 0x10)
                    || cmp(p, (&raw mut DAT_0051bfc0).cast::<i8>(), 0x13)
                    || cmp(p, (&raw mut DAT_0051bfd4).cast::<i8>(), 0xf)
                    || cmp(p, (&raw mut DAT_0051bff8).cast::<i8>(), 0xe)
                    || cmp(p, (&raw mut DAT_00521614).cast::<i8>(), 0x12)
                    || cmp(p, (&raw mut DAT_0051bfe4).cast::<i8>(), 0x13)
                    || cmp(p, (&raw mut DAT_005215fc).cast::<i8>(), 0x15)
                    || cmp(p, (&raw mut DAT_005215e4).cast::<i8>(), 0x16);
                if !matched {
                    break 'lab;
                }
                p = &raw mut DAT_0051a2bc;
            }
            u_var5 = FUN_0046b610(p);
        }
        s_var11 = DAT_00a09182;
        s_var1 = DAT_00a09180;
        if DAT_00a0a2b0 as u32 != u_var5 {
            DAT_00a0a2b0 = u_var5 as u16;
            FUN_004a9520(
                DAT_00a09180,
                DAT_00a09182,
                s_var2,
                (DAT_009a20ae as i16)
                    .wrapping_add(DAT_00a09182)
                    .wrapping_add(1),
            );
            call_d1300();
            FUN_004a9500(s_var1, s_var11);
            let u19: u32 = if DAT_005233ec == 0x13 { 0x45 } else { 0x46 };
            FUN_004a9590(u19);
            FUN_004a9960(DAT_00a0a0fc);
            if DAT_00a09188 == 0 {
                DAT_00a09188 = FUN_004278c0(DAT_00a0a0fc) as i16;
            }
            FUN_004a9500(DAT_00a09188.wrapping_add(s_var1), DAT_00a08102);
            FUN_004a9590(0x4e);
            let ti = (&raw mut DAT_005272f8)
                .cast::<u8>()
                .add(u_var5 as usize)
                .read();
            FUN_004a9960(
                (&raw mut DAT_009a1840)
                    .cast::<*mut u8>()
                    .add(ti as usize)
                    .read(),
            );
            s_var11 = DAT_00a091a6;
            s_var1 = DAT_00a091a4;
            FUN_004a9520(
                DAT_00a091a4,
                DAT_00a091a6,
                s_var2,
                (DAT_009a20ae as i16)
                    .wrapping_add(DAT_00a091a6)
                    .wrapping_add(1),
            );
            call_d1300();
            FUN_004a9500(s_var1, s_var11);
            let u19: u32 = if DAT_005233ec == 0x13 { 0x45 } else { 0x46 };
            FUN_004a9590(u19);
            pc17 = DAT_00a0a114;
            let t16: u16 = rd(slot(&raw mut DAT_009e968a));
            let t32: u32 = rd((t16 as u32)
                .wrapping_mul(0x23)
                .wrapping_add(0x1f)
                .wrapping_add(DAT_009a1fe8 as u32));
            let t16b: u16 = rd(t32.wrapping_add(0x79));
            if t16b != 0 {
                pc17 = DAT_00a0a11c;
                let t16: u16 = rd((i_var4 as u32).wrapping_add(0x42));
                if t16 < 0x8000 {
                    pc17 = DAT_00a0a118;
                }
            }
            FUN_004a97f0(pc17);
            DAT_00a091ac = FUN_004278c0(&raw mut DAT_009a1ed0) as i16;
            FUN_004a9960(&raw mut DAT_009a1ed0);
        }
        i_var7 = rd((DAT_009a1fe8 as u32)
            .wrapping_add(0x1b)
            .wrapping_add(i_var8 as u32));
        if i_var7 == -1 {
            u_var9 = rd((i_var4 as u32).wrapping_add(0x42));
        } else {
            u_var9 =
                rd((&raw mut DAT_009e968a as u32)
                    .wrapping_add((i_var7 as u32).wrapping_mul(0x5bd)));
        }
        let t16: u16 = rd((i_var4 as u32).wrapping_add(0x1e5));
        if t16 == 0 {
            u_var9 = 0xffff;
        }
        let base = (&raw mut DAT_009d1320)
            .cast::<i8>()
            .add(u_var5.wrapping_mul(0x55) as usize);
        if cmp(base, &raw mut DAT_0051a2bc, 8) {
            u_var9 = 0xffff;
        }
        if u_var9 != DAT_00a0a2b2 {
            let u5: u32 = if DAT_005233ec == 0x13 { 0x45 } else { 0x46 };
            DAT_00a0a2b2 = u_var9;
            FUN_004a9590(u5);
            FUN_004a9520(
                DAT_00a0918c,
                DAT_00a0918e,
                s_var2,
                (DAT_009a20ae as i16).wrapping_add(DAT_00a0918e),
            );
            call_d1300();
            FUN_004a9500(DAT_00a0918c, DAT_00a0918e);
            pc17 = DAT_00a0a108;
            if u_var9 < 0x8000 {
                pc17 = DAT_00a0a104;
            }
            FUN_004a97f0(pc17);
            FUN_004a9960(&raw mut DAT_009a1ed0);
            DAT_00a09194 = FUN_004278c0(&raw mut DAT_009a1ed0) as i16;
            FUN_004a9520(
                DAT_00a09198,
                DAT_00a0919a,
                s_var2,
                (DAT_009a20ae as i16).wrapping_add(DAT_00a0919a),
            );
            call_d1300();
            FUN_004a9500(DAT_00a09198, DAT_00a0919a);
            pc17 = DAT_00a0a110;
            if u_var9 < 0x8000 {
                pc17 = DAT_00a0a10c;
            }
            FUN_004a97f0(pc17);
            FUN_004a9960(&raw mut DAT_009a1ed0);
            let r = FUN_004278c0(&raw mut DAT_009a1ed0);
            s_var1 = DAT_00a0918e;
            s_var11 = DAT_00a09194.wrapping_add(DAT_00a0918c);
            DAT_00a091a0 = r as i16;
            FUN_004a9520(
                s_var11,
                DAT_00a0918e,
                s_var2,
                (DAT_009a20ae as i16)
                    .wrapping_add(DAT_00a0918e)
                    .wrapping_add(1),
            );
            call_d1300();
            FUN_004a9500(s_var11, s_var1);
            pbvar12 = DAT_00a0a100;
            if u_var9 != 0xff && u_var9 != 0xffff {
                FUN_0043c290(u_var9, 3);
                pbvar12 = &raw mut DAT_009a1ed0;
            }
            FUN_004a9960(pbvar12);
        }
        u_stack_10 = 0;
        u_stack_e = 0;
        if u_var9 != 0xffff {
            let i7 = (i_var8 as u32).wrapping_add(DAT_009a1fe8 as u32);
            let t32: i32 = rd(i7.wrapping_add(0x1b));
            if t32 == -1 {
                let a0: i32 = rd((i_var4 as u32).wrapping_add(0x49));
                let b0: i32 = rd(i7.wrapping_add(4));
                let a1: i32 = rd((i_var4 as u32).wrapping_add(0x4d));
                let b1: i32 = rd(i7.wrapping_add(8));
                let a2: i32 = rd((i_var4 as u32).wrapping_add(0x51));
                let b2: i32 = rd(i7.wrapping_add(0xc));
                FUN_0046a8d0(
                    a0.wrapping_sub(b0),
                    a1.wrapping_sub(b1),
                    a2.wrapping_sub(b2),
                );
            } else {
                FUN_004034e0(u_var6, u_var9 as u32);
            }
            s_var1 = DAT_00a0919a;
            s_var11 = DAT_00a09198.wrapping_add(DAT_00a091a0);
            u_stack_10 = DAT_009a8c00 as u16;
            u_stack_e = (DAT_009a8c00 >> 0x10) as u16;
            FUN_004a9520(
                s_var11,
                DAT_00a0919a,
                s_var2,
                (DAT_009a20ae as i16)
                    .wrapping_add(DAT_00a0919a)
                    .wrapping_add(1),
            );
            FUN_004a9590(0x4a);
            DAT_009a8c00 = DAT_009a8c00.wrapping_mul(0xa1);
            u_var6 = DAT_009a8c00 >> 0x10;
            if 9999 < (u_var6 as u16) {
                u_var6 = 9999;
            }
            let q = ((u_var6 & 0xffff) / 100) as u16;
            u_var10 = (q as i32)
                .wrapping_mul(-100)
                .wrapping_add(u_var6 as i16 as i32) as u16;
            if u_var10 != DAT_00a0a2b4 {
                DAT_00a0a2b4 = u_var10;
                FUN_004a9500(s_var11, s_var1);
                FUN_004277f0(q, 2, 1);
                call_6fe4(0x2e);
                FUN_004277f0(u_var10, 2, 2);
            }
        }
        s_var1 = DAT_00a091a6;
        s_var11 = DAT_00a091ac.wrapping_add(DAT_00a091a4);
        FUN_004a9520(
            s_var11,
            DAT_00a091a6,
            s_var2,
            (DAT_009a20ae as i16)
                .wrapping_add(DAT_00a091a6)
                .wrapping_add(1),
        );
        FUN_004a9590(0x52);
        FUN_004a9500(s_var11, s_var1);
        let t32: u32 = rd((DAT_009a1fe8 as u32)
            .wrapping_add(0x1f)
            .wrapping_add(i_var8 as u32));
        let sv: i16 = rd(t32.wrapping_add(0x79));
        if sv == 0 {
            let nb = (&raw mut DAT_009d1320).cast::<i8>().add(
                (*((i_var4 as u32).wrapping_add(0x34) as *const u8) as u32).wrapping_mul(0x55)
                    as usize,
            );
            let mut do_time = cmp(nb, &raw mut DAT_00521690, 10);
            if !do_time {
                do_time = cmp(nb, &raw mut DAT_0051a2bc, 8);
            }
            if !do_time {
                if (((u_stack_e as u32) << 16) | u_stack_10 as u32) as i32 == 0 {
                    DAT_00a0a2b6 = 0;
                    call_d1300();
                    FUN_004277f0(0, 2, 1);
                    call_6fe4(0x3a);
                    FUN_004277f0(0, 2, 2);
                    return;
                }
                FUN_004a9960(DAT_00a607ac);
                return;
            }
            let t32b: i32 = rd((i_var4 as u32).wrapping_add(0x60));
            let u6 = (t32b / 0xec) as u32;
            let u9 = ((u6 & 0xffff) / 0x3c) as u16;
            let u10 = (u6 as i16 as i32).wrapping_add((u9 as i32).wrapping_mul(-0x3c)) as u16;
            if u10 != DAT_00a0a2b6 {
                DAT_00a0a2b6 = u10;
                call_d1300();
                FUN_004277f0(u9, 2, 1);
                call_6fe4(0x3a);
                FUN_004277f0(u10, 2, 2);
                return;
            }
        } else {
            let t32c: i32 = (sv.wrapping_mul(0x12)) as u16 as i32;
            if t32c == 0 {
                RaiseException(0xC0000094, 0, 0, core::ptr::null());
            }
            let lo = ((u_stack_e as u32) << 16) | u_stack_10 as u32;
            let u9 = udiv(lo, t32c as u32) as u16;
            let u10 = u9 % 0x3c;
            if u10 != DAT_00a0a2b6 {
                DAT_00a0a2b6 = u10;
                call_d1300();
                FUN_004277f0(u9 / 0x3c, 2, 1);
                call_6fe4(0x3a);
                FUN_004277f0(u10, 2, 2);
            }
        }
    }
}
