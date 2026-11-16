// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0099944a: [u8; 4];
    static mut DAT_0099945c: i32;
    static mut DAT_0099946a: [u8; 4];
    static mut DAT_00999470: i32;
    static mut DAT_00999478: i32;
    static mut DAT_00999482: [u8; 4];
    static mut DAT_009a1fe4: u16;
    static mut DAT_0060f1d0: *mut f32;
    static mut DAT_0060f1e0: *mut f32;
    static mut DAT_0060f1c4: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00471a10(param_1: i32, param_2: *mut u32, param_3: *mut f32, param_4: *mut f32) -> u32;
    fn FUN_00471ce0(param_1: *mut u32, param_2: *mut f32, param_3: *mut f32) -> u32;
    fn FUN_00471410(param_1: i32);
}

#[inline(always)]
fn a(x: i32, y: i32) -> i32 {
    x.wrapping_add(y)
}
#[inline(always)]
fn s(x: i32, y: i32) -> i32 {
    x.wrapping_sub(y)
}
#[inline(always)]
fn m(x: i32, y: i32) -> i32 {
    x.wrapping_mul(y)
}
#[inline(always)]
unsafe fn ld(addr: i32) -> i32 {
    // SAFETY: the address is the game's own memory, read where the original reads.
    unsafe { (addr as u32 as usize as *const i32).read_unaligned() }
}
#[inline(always)]
unsafe fn st(addr: i32, v: i32) {
    // SAFETY: the address is the game's own memory, written where the original writes.
    unsafe { (addr as u32 as usize as *mut i32).write_unaligned(v) }
}
#[inline(always)]
unsafe fn g82() -> i32 {
    // SAFETY: the global is the game's own array, read as 4 bytes.
    unsafe { (&raw mut DAT_00999482).cast::<i32>().read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00471020(param_1: i32) {
    // SAFETY: all globals, callees and addresses are the game's own memory,
    // accessed in the same order as the original.
    unsafe {
        let i_var7 = DAT_00999470;
        let i_var1 = a(DAT_0099945c, m(ld(a(param_1, 0xbc)), 0x18));
        let mut i_var8: i32 = 0;
        let g44a = (&raw mut DAT_0099944a).cast::<i32>().read_unaligned();
        let mut l10: i32 = a(m(ld(a(param_1, 0xb8)), 0x70), g44a);
        st(a(param_1, 0xc0), DAT_00999470);
        let mut i_var6: i32 = 0;
        st(a(param_1, 0xcc), 0);
        let g46a = (&raw mut DAT_0099946a).cast::<i32>().read_unaligned();
        let pu2: i32 = a(g46a, m(i_var7, 0x1c));
        if 0 < ld(a(param_1, 0x90)) {
            loop {
                i_var8 = a(i_var8, 4);
                st(a(s(g82(), 4), i_var8), -1);
                i_var6 = a(i_var6, 1);
                if !(i_var6 < ld(a(param_1, 0x90))) {
                    break;
                }
            }
        }
        let mut local_4: i32 = 0;
        let mut pu9: i32 = pu2;
        if 0 < ld(a(param_1, 0xc4)) {
            loop {
                let i_var7 = a(m(ld(l10), 0x40), ld(a(param_1, 0xa0)));
                let i_var12: i32;
                if ld(a(l10, 12)) == -1 {
                    st(a(l10, 12), DAT_009a1fe4 as i32);
                    DAT_0060f1d0 = core::ptr::null_mut();
                    DAT_0060f1e0 = core::ptr::null_mut();
                    let mut local_18: i32 = s(4, (ld(a(i_var7, 0x1c)) == -1) as i32);
                    i_var6 = 0;
                    i_var8 = a(a(i_var7, 0x10), m(local_18, 4));
                    let mut i_var10: i32 = 0;
                    let mut i_var11: i32;
                    loop {
                        let i_var3 = m(ld(s(i_var8, 4)), 4);
                        local_18 = s(local_18, 1);
                        DAT_0060f1c4 = 0;
                        let i_var4 = ld(a(i_var3, g82()));
                        i_var11 = i_var10;
                        if i_var4 == -1 {
                            let p3 = a(
                                i_var1,
                                m(ld(a(DAT_00999478, m(ld(s(i_var8, 0x14)), 4))), 0x18),
                            );
                            let p4 = a(
                                i_var1,
                                m(ld(a(DAT_00999478, m(ld(a(i_var7, m(i_var6, 4))), 4))), 0x18),
                            );
                            let u_var5 = FUN_00471a10(
                                param_1,
                                pu9 as u32 as usize as *mut u32,
                                p3 as u32 as usize as *mut f32,
                                p4 as u32 as usize as *mut f32,
                            );
                            if (u_var5 as i32) < 0 {
                                if DAT_0060f1c4 == 0 {
                                    st(a(i_var3, g82()), -2);
                                }
                            } else {
                                st(a(l10, m(a(i_var10, 0x14), 4)), pu9);
                                st(a(pu9, 24), DAT_0060f1c4);
                                st(a(i_var3, g82()), ld(a(param_1, 0xcc)));
                                st(a(param_1, 0xcc), a(ld(a(param_1, 0xcc)), 1));
                                pu9 = a(pu9, 0x1c);
                                i_var11 = a(i_var10, 1);
                            }
                        } else if i_var4 != -2 {
                            i_var11 = a(i_var10, 1);
                            st(a(l10, m(a(i_var10, 0x14), 4)), a(pu2, m(i_var4, 0x1c)));
                            let idx = a(m(ld(a(i_var3, g82())), 7), 6);
                            let val = ld(a(pu2, m(idx, 4)));
                            if val != 0 {
                                DAT_0060f1e0 = DAT_0060f1d0;
                                DAT_0060f1d0 = val as u32 as usize as *mut f32;
                            }
                        }
                        i_var6 = local_18;
                        i_var8 = s(i_var8, 4);
                        i_var10 = i_var11;
                        if !(0 < local_18) {
                            break;
                        }
                    }
                    let mut r = i_var11;
                    if !DAT_0060f1e0.is_null() {
                        let u_var5 = FUN_00471a10(
                            param_1,
                            pu9 as u32 as usize as *mut u32,
                            DAT_0060f1d0,
                            DAT_0060f1e0,
                        );
                        if -1 < (u_var5 as i32) {
                            r = a(i_var11, 1);
                            st(a(l10, m(a(i_var11, 0x14), 4)), pu9);
                            pu9 = a(pu9, 0x1c);
                            st(a(param_1, 0xcc), a(ld(a(param_1, 0xcc)), 1));
                        }
                    }
                    i_var12 = r;
                } else {
                    st(a(l10, 12), DAT_009a1fe4 as i32);
                    let mut local_18: i32 = s(4, (ld(a(i_var7, 0x1c)) == -1) as i32);
                    i_var6 = 0;
                    i_var8 = a(a(i_var7, 0x10), m(local_18, 4));
                    let mut i_var10: i32 = 0;
                    let mut r: i32;
                    loop {
                        let i_var3 = ld(s(i_var8, 4));
                        local_18 = s(local_18, 1);
                        let i_var4 = ld(a(g82(), m(i_var3, 4)));
                        r = i_var10;
                        if i_var4 == -1 {
                            let p2 = a(
                                i_var1,
                                m(ld(a(DAT_00999478, m(ld(s(i_var8, 0x14)), 4))), 0x18),
                            );
                            let p3 = a(
                                i_var1,
                                m(ld(a(DAT_00999478, m(ld(a(i_var7, m(i_var6, 4))), 4))), 0x18),
                            );
                            let u_var5 = FUN_00471ce0(
                                pu9 as u32 as usize as *mut u32,
                                p2 as u32 as usize as *mut f32,
                                p3 as u32 as usize as *mut f32,
                            );
                            if (u_var5 as i32) < 0 {
                                st(a(g82(), m(i_var3, 4)), -2);
                            } else {
                                st(a(l10, m(a(i_var10, 0x14), 4)), pu9);
                                pu9 = a(pu9, 0x1c);
                                st(a(g82(), m(i_var3, 4)), ld(a(param_1, 0xcc)));
                                st(a(param_1, 0xcc), a(ld(a(param_1, 0xcc)), 1));
                                r = a(i_var10, 1);
                            }
                        } else if i_var4 != -2 {
                            r = a(i_var10, 1);
                            st(a(l10, m(a(i_var10, 0x14), 4)), a(pu2, m(i_var4, 0x1c)));
                        }
                        i_var6 = local_18;
                        i_var8 = s(i_var8, 4);
                        i_var10 = r;
                        if !(0 < local_18) {
                            break;
                        }
                    }
                    i_var12 = r;
                }
                st(a(l10, m(0x19, 4)), i_var12);
                if i_var12 == 0 {
                    st(a(l10, m(0x11, 4)), 0);
                    st(a(l10, m(0x10, 4)), 0);
                } else {
                    FUN_00471410(l10);
                }
                local_4 = a(local_4, 1);
                l10 = a(l10, 0x70);
                if !(local_4 < ld(a(param_1, 0xc4))) {
                    break;
                }
            }
        }
        DAT_00999470 = a(DAT_00999470, ld(a(param_1, 0xcc)));
    }
}
