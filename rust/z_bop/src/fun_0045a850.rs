// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_00a07cec: i32;
    static mut DAT_009a1fe0: i32;
    static mut DAT_009a1fe8: *mut u8;
    static mut DAT_00a08104: *mut u8;
    static mut DAT_00a07ce8: i32;
    static mut DAT_009d1308: i32;
    static mut DAT_009a73c2: u16;

    static mut DAT_009d9192: u8;
    static mut DAT_009e9670: u8;
    static mut DAT_009e9681: u8;
    static mut DAT_009e9684: u8;
    static mut DAT_009e9696: u8;
    static mut DAT_009e96bb: u8;
    static mut DAT_009e96bd: u8;
    static mut DAT_009e96bf: u8;
    static mut DAT_009e96c1: u8;
    static mut DAT_009e96c3: u8;
    static mut DAT_009e969d: u8;
    static mut DAT_009e968a: u8;
    static mut DAT_009a7a40: u8;
    static mut DAT_00524120: u8;
    static mut DAT_009d8cd2: u8;
    static mut DAT_009d8cd3: u8;
    static mut DAT_009d1320: u8;
    static mut DAT_0051c583: u8;
    static mut DAT_00527810: u16;
    static mut DAT_0051a1d0: i8;
    static mut s_stationaryldrpln_0051a1bc: i8;
    static mut s_stationaryflwpln_0051a1a8: i8;
    static mut s_disabledpln_0051a368: i8;
    static mut s_escortldr1pln_0051a1e4: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0041bba0(a: u32);
    fn FUN_0041bb00(a: u16);
    fn FUN_0045acd0(a: i32);
    fn FUN_00425b70(a: u32, b: u16) -> u32;
    fn FUN_00402cb0(a: u16);
    fn FUN_00402970(a: u32);
    fn FUN_00404450(a: u16) -> u32;
    fn FUN_0046b610(a: *mut i8) -> i32;
}

#[inline(always)]
unsafe fn rd<T>(p: *const u8, o: u32) -> T {
    // SAFETY: the pointer is game memory, read where the original reads.
    unsafe { p.byte_add(o as usize).cast::<T>().read_unaligned() }
}

#[inline(always)]
unsafe fn wr<T>(p: *mut u8, o: u32, v: T) {
    // SAFETY: the pointer is game memory, written where the original writes.
    unsafe { p.byte_add(o as usize).cast::<T>().write_unaligned(v) }
}

unsafe fn repe_cmp(mut p: *const i8, mut q: *const i8, mut n: i32) -> bool {
    let mut eq = false;
    // SAFETY: both pointers address the game's NUL-padded name strings.
    unsafe {
        loop {
            if n == 0 {
                break;
            }
            n -= 1;
            eq = *p == *q;
            p = p.add(1);
            q = q.add(1);
            if !eq {
                break;
            }
        }
    }
    eq
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0045a850(param_1: i32, param_2: i32, param_3: i32) -> u32 {
    // SAFETY: all globals, callees and pointers belong to the game; accesses
    // follow the original's order and offsets.
    unsafe {
        if param_2 == 1 {
            let mut u_var12: u16 = 0;
            if _DAT_00a07cec < DAT_009a1fe0 {
                let mut p: *const u8 = DAT_009a1fe8
                    .byte_add((_DAT_00a07cec as u32).wrapping_mul(0x23).wrapping_add(3) as usize);
                let mut i_var7: i32 = DAT_009a1fe0.wrapping_sub(_DAT_00a07cec);
                loop {
                    if *p != 0 {
                        let idx = rd::<u8>(p, 0x13) as u32;
                        let tmpi: i32 = rd::<i32>(&raw mut DAT_009d9192, idx.wrapping_mul(0x566));
                        if tmpi == param_1 {
                            let tmp: *mut u8 = rd::<*mut u8>(p, 0x1c);
                            let tmp2: *mut u8 = rd::<*mut u8>(tmp, 0xa9);
                            let c = rd::<u8>(tmp2, 7) as i8;
                            if c != 3 && c != 5 && c != 4 {
                                u_var12 = u_var12.wrapping_add(1);
                            }
                        }
                    }
                    p = p.byte_add(0x23);
                    i_var7 = i_var7.wrapping_sub(1);
                    if i_var7 == 0 {
                        break;
                    }
                }
            }
            if u_var12 < 2 {
                return 0;
            }
        }
        let i_var7: i32 = (param_1 as u32).wrapping_mul(0x5bd) as i32;
        let o7: u32 = i_var7 as u32;
        let u_var10: u32 = rd::<u32>(&raw mut DAT_009e9670, o7);
        if u_var10 == 0xffff_ffff {
            return 0;
        }
        let i_var5: u32 = u_var10.wrapping_mul(0x23);
        wr::<u32>(DAT_009a1fe8, i_var5.wrapping_add(0x1b), 0xffff_ffff);
        let mut tmp: *mut u8 = rd::<*mut u8>(DAT_009a1fe8, i_var5.wrapping_add(0x1f));
        wr::<u8>(tmp, 0x92, 1);
        tmp = rd::<*mut u8>(DAT_009a1fe8, i_var5.wrapping_add(0x1f));
        wr::<u8>(tmp, 0x8b, 1);
        FUN_0041bba0(u_var10);
        let u_var12: u16 = u_var10 as u16;
        FUN_0041bb00(u_var12);
        FUN_0045acd0(param_1);
        let mut i: u32 = 0;
        tmp = rd::<*mut u8>(DAT_009a1fe8, i_var5.wrapping_add(0x1f));
        let i_var4: *mut u8 = rd::<*mut u8>(tmp, 0xa9);
        loop {
            wr::<u8>(i_var4, 0x220 + i, 0);
            wr::<u8>(i_var4, 0x222 + i, 0);
            wr::<u8>(i_var4, 0x224 + i, 0);
            wr::<u16>(i_var4, 0x226 + i * 2, 0);
            wr::<u32>(i_var4, 0x22a + i * 4, 0);
            i += 1;
            if i >= 2 {
                break;
            }
        }
        i = 0;
        loop {
            let old = i;
            let v = rd::<u8>(i_var4, 0x235 + old);
            i += 1;
            wr::<u8>(i_var4, 0x234 + i, (v & 0x81) | 1);
            wr::<u16>(i_var4, 0x235 + i * 2, 0);
            if i >= 2 {
                break;
            }
        }
        wr::<u16>(i_var4, 0x23b, 0);
        {
            let a: u32 = rd::<u32>(i_var4, 0x211);
            let b: u32 = rd::<u32>(i_var4, 0x215);
            wr::<u32>(i_var4, 0x211, a.wrapping_add(b));
        }
        wr::<u32>(i_var4, 0x215, 0);
        wr::<u8>(i_var4, 0x21a, 0);
        wr::<u32>(&raw mut DAT_009e9670, o7, 0xffff_ffff);
        wr::<u8>(&raw mut DAT_009e9681, o7, 0);
        wr::<u8>(&raw mut DAT_009e9684, o7, 0);
        wr::<u8>(&raw mut DAT_009e9696, o7, 0);
        let tmp0: *mut u8 = DAT_00a08104;
        wr::<u16>(&raw mut DAT_009e96bb, o7, 0);
        wr::<u16>(&raw mut DAT_009e96bd, o7, 0);
        wr::<u16>(&raw mut DAT_009e96bf, o7, 0);
        wr::<u16>(&raw mut DAT_009e96c1, o7, 0);
        wr::<u16>(&raw mut DAT_009e96c3, o7, 0);
        wr::<u16>(&raw mut DAT_009e969d, o7, 0xffff);
        wr::<u8>(tmp0, 0x2a, 0);
        if param_3 == 0 {
            let r = FUN_0046b610(&raw mut DAT_0051a1d0);
            wr::<u8>(DAT_00a08104, 0x34, r as u8);
            let v = rd::<u8>(DAT_00a08104, 0x34);
            wr::<u8>(DAT_00a08104, 0x35, v);
            DAT_00a08104 = i_var4;
            FUN_00402cb0(u_var12);
            FUN_00402970(u_var10);
            wr::<u8>(i_var4, 0xa4, 0);
            wr::<u8>(i_var4, 0x9c, 0);
            wr::<u8>(i_var4, 0xab, 0);
            wr::<u8>(i_var4, 0x96, 0);
            wr::<u8>(i_var4, 0x97, 0);
            return 1;
        }
        let k1: u32 =
            (rd::<u8>(DAT_009a1fe8, 0x16u32.wrapping_add(i_var5)) as u32).wrapping_mul(0x566);
        let k2: u8 = rd::<u8>(&raw mut DAT_009d8cd2, k1);
        let k3: u8 = rd::<u8>(&raw mut DAT_00524120, k2 as u32);
        let b_var3: u8 = rd::<u8>(&raw mut DAT_009a7a40, k3 as u32);
        wr::<u8>(DAT_00a08104, 0x35, b_var3);
        wr::<u8>(DAT_00a08104, 0x34, b_var3);
        let pc16: *const i8 = (&raw mut DAT_009d1320)
            .byte_add((b_var3 as u32).wrapping_mul(0x55) as usize)
            as *const i8;
        let u_var6: u16;
        'ab79: {
            if repe_cmp(pc16, &raw mut DAT_0051a1d0, 8) {
                u_var6 = 0;
                break 'ab79;
            }
            if repe_cmp(pc16, &raw mut s_stationaryldrpln_0051a1bc, 0x11) {
                u_var6 = 0;
                break 'ab79;
            }
            if repe_cmp(pc16, &raw mut s_stationaryflwpln_0051a1a8, 0x11) {
                u_var6 = 0;
                break 'ab79;
            }
            if repe_cmp(pc16, &raw mut s_disabledpln_0051a368, 0xc) {
                u_var6 = 0;
                break 'ab79;
            }
            if repe_cmp(pc16, &raw mut s_escortldr1pln_0051a1e4, 0xe) {
                u_var6 = 0x8000;
            } else {
                let kk: u32 = (rd::<u8>(DAT_009a1fe8, 0x16u32.wrapping_add(i_var5)) as u32)
                    .wrapping_mul(0x566);
                let idx = rd::<u8>(&raw mut DAT_009d8cd3, kk);
                u_var6 = (&raw mut DAT_00527810).add(idx as usize).read_unaligned();
            }
        }
        wr::<u16>(i_var4, 0xbd, u_var6);
        let b4 = rd::<u8>(i_var4, 4) as u32;
        let mut tmps: u16 = rd::<u16>(&raw mut DAT_0051c583, b4.wrapping_mul(0x101));
        let u_var9: u32 = FUN_00425b70(tmps as u32, u_var6);
        tmp = rd::<*mut u8>(DAT_009a1fe8, i_var5.wrapping_add(0x1f));
        wr::<u16>(tmp, 0x79, u_var9 as u16);
        tmp = rd::<*mut u8>(DAT_009a1fe8, i_var5.wrapping_add(0x1f));
        wr::<u16>(tmp, 0x7b, 0);
        DAT_00a08104 = i_var4;
        FUN_00402cb0(u_var12);
        FUN_00402970(u_var10);
        tmps = rd::<u16>(&raw mut DAT_009e968a, o7);
        if tmps != 0xffff {
            let v = tmps as u32;
            if (v as i32) < DAT_009a1fe0
                || (DAT_00a07ce8 <= v as i32
                    && (v as i32) < DAT_009d1308.wrapping_add(DAT_00a07ce8))
            {
                let mut flag = false;
                let mut c: u32 = 0;
                loop {
                    DAT_009a73c2 = c as u16;
                    tmps = rd::<u16>(&raw mut DAT_009e968a, o7);
                    let r = FUN_00404450(tmps);
                    if (r as i16) != 0 {
                        flag = true;
                    }
                    c = c.wrapping_add(1);
                    if c >= 3 {
                        break;
                    }
                }
                if flag {
                    tmps = rd::<u16>(&raw mut DAT_009e968a, o7);
                    wr::<u16>(i_var4, 0x55, tmps);
                }
            }
        }
        1
    }
}
