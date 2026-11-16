// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009eca24: u16;
    static mut DAT_009ecc30: u16;
    static mut DAT_009ec474: u16;
    static mut DAT_009ec5fe: u16;
    static mut DAT_009d8112: u16;
    static mut _DAT_00999414: u16;
    static mut DAT_00523410: i32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_009fe7f4: i32;
    static mut DAT_00a00490: i32;
    static mut DAT_00a004b0: i32;
    static mut DAT_00521f1c: [u8; 1];
    static mut DAT_009e9670: [u8; 1];
    static mut DAT_009e9683: [u8; 1];
    static mut DAT_009e9684: [u8; 1];
    static mut DAT_009e96bb: [u8; 1];
    static mut DAT_009e96bd: [u8; 1];
    static mut DAT_009e96bf: [u8; 1];
    static mut DAT_009e9a5b: [u8; 1];
    static mut DAT_009e9a5f: [u8; 1];
    static mut DAT_009e9a63: [u8; 1];
    static mut DAT_009e9a67: [u8; 1];
    static mut DAT_009e9a69: [u8; 1];
    static mut DAT_009e9a6b: [u8; 1];
    static mut DAT_009e9a6d: [u8; 1];
    static mut DAT_009e9a73: [u8; 1];
    static mut DAT_009e9a75: [u8; 1];
    static mut DAT_009e9a80: [u8; 1];
    static mut DAT_009e9a82: [u8; 1];
    static mut DAT_009e9a86: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004115f0();
    fn FUN_00425b70(a: u16, b: u16) -> u32;
    fn FUN_00425c20(a: u32, b: u32) -> u32;
    fn FUN_00481d70(a: i16) -> i16;
    fn FUN_00483a00(a: i16, b: i32, c: u32, d: i32);
    fn FUN_00438330(a: i16, b: i32, c: i32);
    fn FUN_0041f410(a: i32, b: i32, c: i32) -> i32;
    fn FUN_00427940(a: i32, b: i16, c: i16, d: i32, e: i16, f: i16, g: i32);
}

macro_rules! sl {
    ($d:ident, $o:expr) => {
        ((&raw mut $d) as *mut u8 as u32).wrapping_add($o as u32)
    };
}

unsafe fn lds(p: u32) -> i16 {
    // SAFETY: p is an address inside the game's own memory.
    unsafe { (p as *const i16).read_unaligned() }
}
unsafe fn ldu(p: u32) -> u16 {
    // SAFETY: p is an address inside the game's own memory.
    unsafe { (p as *const u16).read_unaligned() }
}
unsafe fn ldi(p: u32) -> i32 {
    // SAFETY: p is an address inside the game's own memory.
    unsafe { (p as *const i32).read_unaligned() }
}
unsafe fn ldui(p: u32) -> u32 {
    // SAFETY: p is an address inside the game's own memory.
    unsafe { (p as *const u32).read_unaligned() }
}
unsafe fn ldb(p: u32) -> u8 {
    // SAFETY: p is an address inside the game's own memory.
    unsafe { (p as *const u8).read() }
}
unsafe fn st16(p: u32, v: u16) {
    // SAFETY: p is an address inside the game's own memory.
    unsafe { (p as *mut u16).write_unaligned(v) }
}
unsafe fn st32(p: u32, v: u32) {
    // SAFETY: p is an address inside the game's own memory.
    unsafe { (p as *mut u32).write_unaligned(v) }
}
unsafe fn stb(p: u32, v: u8) {
    // SAFETY: p is an address inside the game's own memory.
    unsafe { (p as *mut u8).write(v) }
}

fn w(a: i32, k: u32) -> u32 {
    (a as u32).wrapping_add(k)
}

unsafe fn ent(i: u32) -> u32 {
    // SAFETY: reads the game's own entity table pointer.
    unsafe { i.wrapping_mul(0x23).wrapping_add(DAT_009a1fe8 as u32) }
}

unsafe fn tab(idx: u32) -> u32 {
    // SAFETY: the table and entity array are the game's own memory.
    unsafe {
        let b = ldb(ent(idx).wrapping_add(3)) as u32;
        let base = (&raw mut DAT_00521f1c) as *mut u8 as u32;
        (ldi(base.wrapping_add(b.wrapping_mul(0x1a))) as u32).wrapping_add(0x200)
    }
}

fn mulsh(a: i32, b: i32) -> u32 {
    let m = (a as i64).wrapping_mul(b as i64);
    ((m as u32) >> 0xf) | (((m as u64) >> 0x20) as u32).wrapping_shl(0x11)
}

unsafe fn clamp(addr: u32) {
    // SAFETY: addr is inside the game's own memory.
    unsafe {
        if ldi(addr) < -0x1000000 {
            st32(addr, 0xff000000);
        }
        if 0x1000000 < ldi(addr) {
            st32(addr, 0x1000000);
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00480570(param_1: i32) {
    // SAFETY: all globals, callees and addresses are the game's own memory
    // and code, accessed where the original accesses them.
    unsafe {
        FUN_004115f0();
        DAT_009eca24 = DAT_009eca24.wrapping_shl(1);
        let v2: i32 = (param_1 as u32).wrapping_mul(0x5bd) as i32;
        macro_rules! p {
            ($d:ident) => {
                sl!($d, v2)
            };
        }
        if lds(p!(DAT_009e9a80)) == 0 && ldb(p!(DAT_009e9683)) == 0 {
            if ldb(p!(DAT_009e9684)) != 0 {
                return;
            }
            let i_var12 = ldui(p!(DAT_009e9670)).wrapping_mul(0x23);
            let mut i_var9: i32 = ldi(w(
                ldi(i_var12.wrapping_add(0x1f).wrapping_add(DAT_009a1fe8 as u32)),
                0xa9,
            ));
            let mut u_var10: u16 = ldu(w(i_var9, 0xbd));
            if ldb(w(i_var9, 0x1e5)) & 0x40 == 0 {
                u_var10 = 0;
            }
            if u_var10 < 0x5555 {
                u_var10 = (u_var10 as u32 * 2 + 0x5555) as u16;
            } else {
                let d = ((0x5555u32.wrapping_sub(u_var10 as u32)) as i32) / 2;
                u_var10 = (d as i16).wrapping_sub(1) as u16;
            }
            let mut s_var11: i16 = ldb(w(i_var9, 0x21c)) as i16;
            if ldb(w(i_var9, 0x1e3)) & 1 == 0 {
                s_var11 = s_var11.wrapping_mul(2);
            } else {
                s_var11 = s_var11.wrapping_add(ldb(w(i_var9, 0x219)) as u16 as i16);
            }
            s_var11 = 4i16.wrapping_sub(s_var11);
            let u_var14: u16 = if s_var11 < 0 {
                (s_var11 as i32).wrapping_mul(-0xc00) as u16
            } else {
                (s_var11 as i32).wrapping_mul(0xc00) as u16
            };
            let mut u_var13: u32 = FUN_00425b70(ldu(w(i_var9, 0xa0)), u_var10);
            let mut s_var17: i16;
            if s_var11 < 1 {
                let u5 = FUN_00425b70(u_var13 as u16, u_var14);
                s_var17 = (u5 as i16).wrapping_neg();
            } else {
                let u5 = FUN_00425b70(u_var13 as u16, u_var14);
                s_var17 = u5 as i16;
            }
            let u_var16: u16 = (u_var13 as i16).wrapping_add(s_var17) as u16;
            let mut u_var6 = FUN_00425c20((u_var16 % 0x3800) as u32, 0x3800);
            let mut u_var8: u16 = DAT_009ecc30;
            if 0x7fff < DAT_009ecc30 {
                u_var8 = DAT_009ecc30.wrapping_neg();
            }
            u_var13 = FUN_00425b70(u_var8, u_var6 as u16);
            s_var17 = ((u_var16 / 0x3800) as u32)
                .wrapping_mul(u_var8 as u32)
                .wrapping_add((u_var13 as i16) as i32 as u32) as i16;
            if 0x7fff < DAT_009ecc30 {
                s_var17 = s_var17.wrapping_neg();
            }
            _DAT_00999414 = DAT_009ecc30;
            if 0x7fff < DAT_009ecc30 {
                _DAT_00999414 = DAT_009ecc30.wrapping_neg();
            }
            let u_var5 = FUN_00425b70(ldu(w(i_var9, 0x98)), u_var10);
            u_var13 = u_var5;
            if s_var11 < 1 {
                u_var13 = FUN_00425b70(u_var13 as u16, u_var14);
                s_var11 = (u_var13 as i16).wrapping_neg();
            } else {
                u_var13 = FUN_00425b70(u_var13 as u16, u_var14);
                s_var11 = u_var13 as i16;
            }
            let local_6: u16 = (u_var5 as i16).wrapping_add(s_var11) as u16;
            u_var6 = FUN_00425c20((local_6 % 0x1400) as u32, 0x1400);
            u_var10 = DAT_009eca24;
            if 0x7fff < DAT_009eca24 {
                u_var10 = DAT_009eca24.wrapping_neg();
            }
            u_var13 = FUN_00425b70(u_var10, u_var6 as u16);
            s_var11 = ((local_6 / 0x1400) as u32)
                .wrapping_mul(u_var10 as u32)
                .wrapping_add((u_var13 as i16) as i32 as u32) as i16;
            if 0x7fff < DAT_009eca24 {
                s_var11 = s_var11.wrapping_neg();
            }
            if (ldb(w(i_var9, 0x1e5)) & 0x20) == 0
                || (ldi(w(i_var9, 0x19)) != 0 && lds(w(i_var9, 0x248)) == 0)
            {
                s_var17 = 0;
                s_var11 = 0;
            }
            let u_stack_a: u16 = (((DAT_009ec474 as u8) & 0xe) == 2) as u16;
            if lds(p!(DAT_009e96bb)) as i32 == u_stack_a as i32 {
                let s_var15 = lds(p!(DAT_009e96bd));
                let s_var18 = s_var17.wrapping_sub(s_var15);
                if s_var18 != 0 {
                    let mut s_var4 = s_var18;
                    if s_var17 < s_var15 {
                        s_var4 = s_var18.wrapping_neg();
                    }
                    if s_var4 < 8 {
                        st16(p!(DAT_009e96bd), s_var15.wrapping_add(s_var18) as u16);
                    } else {
                        if 4 < DAT_009d8112 {
                            let local_3c = DAT_009d8112 as u32;
                            s_var4 = ((s_var4 as i32) / (local_3c as i32)) as i16;
                            if s_var4 == 0 {
                                s_var4 = 1;
                            }
                            s_var4 = s_var4.wrapping_shl(2);
                        }
                        if s_var18 < 0 {
                            st16(p!(DAT_009e96bd), s_var15.wrapping_sub(s_var4) as u16);
                        } else {
                            st16(p!(DAT_009e96bd), s_var4.wrapping_add(s_var15) as u16);
                        }
                    }
                }
                let s_var17b = lds(p!(DAT_009e96bf));
                let s_var15b = s_var11.wrapping_sub(s_var17b);
                if s_var15b != 0 {
                    let mut s_var18b = s_var15b;
                    if s_var11 < s_var17b {
                        s_var18b = s_var15b.wrapping_neg();
                    }
                    if s_var18b < 8 {
                        st16(p!(DAT_009e96bf), s_var17b.wrapping_add(s_var15b) as u16);
                    } else {
                        if 4 < DAT_009d8112 {
                            s_var18b = ((s_var18b as i32) / (DAT_009d8112 as u32 as i32)) as i16;
                            if s_var18b == 0 {
                                s_var18b = 1;
                            }
                            s_var18b = s_var18b.wrapping_shl(2);
                        }
                        if s_var15b < 0 {
                            st16(p!(DAT_009e96bf), s_var17b.wrapping_sub(s_var18b) as u16);
                        } else {
                            st16(p!(DAT_009e96bf), s_var18b.wrapping_add(s_var17b) as u16);
                        }
                    }
                }
            } else {
                st16(p!(DAT_009e96bd), 0);
                st16(p!(DAT_009e96bf), 0);
            }
            st16(p!(DAT_009e96bb), u_stack_a);
            s_var11 = FUN_00481d70(lds(p!(DAT_009e96bd)));
            s_var17 = FUN_00481d70(lds(p!(DAT_009e96bf)));
            if (ldb(w(i_var9, 0x1e5)) & 0x20) == 0
                || (ldi(w(i_var9, 0x19)) != 0 && lds(w(i_var9, 0x248)) == 0)
            {
                s_var17 = 0;
                s_var11 = 0;
            }
            if u_stack_a != 0 {
                if s_var17 != 0 {
                    FUN_00483a00(s_var17, 0, ldui(p!(DAT_009e9670)), i_var9);
                    stb(
                        w(ldi(ent(ldui(p!(DAT_009e9670))).wrapping_add(0x1f)), 0x92),
                        1,
                    );
                    i_var9 = ldi(ent(ldui(p!(DAT_009e9670))).wrapping_add(0x1f));
                    stb(w(i_var9, 0x8b), ldb(w(i_var9, 0x92)));
                }
                if s_var11 == 0 {
                    return;
                }
                let ps = ent(ldui(p!(DAT_009e9670))).wrapping_add(0x14);
                st16(ps, lds(ps).wrapping_add(s_var11.wrapping_mul(-2)) as u16);
                stb(
                    w(ldi(ent(ldui(p!(DAT_009e9670))).wrapping_add(0x1f)), 0x92),
                    1,
                );
                let t = ldi(ent(ldui(p!(DAT_009e9670))).wrapping_add(0x1f));
                stb(w(t, 0x8b), ldb(w(t, 0x92)));
                return;
            }
            if s_var17 == 0 && s_var11 == 0 {
                return;
            }
            FUN_00483a00(s_var17, -(s_var11 as i32), ldui(p!(DAT_009e9670)), i_var9);
            stb(
                w(ldi(ent(ldui(p!(DAT_009e9670))).wrapping_add(0x1f)), 0x92),
                1,
            );
            i_var9 = ldi(ent(ldui(p!(DAT_009e9670))).wrapping_add(0x1f));
            stb(w(i_var9, 0x8b), ldb(w(i_var9, 0x92)));
            if s_var11 == 0 {
                return;
            }
            let ps = ent(ldui(p!(DAT_009e9670))).wrapping_add(0x14);
            st16(ps, lds(ps).wrapping_sub(s_var11) as u16);
            return;
        }
        let mut b_var7: u8 = ldb(p!(DAT_009e9683));
        if b_var7 != 0 {
            if b_var7 & 0x80 == 0 {
                let mut u_var10: u16 = (b_var7 & 0x7f) as u16;
                if (u_var10 != 1 && (b_var7 & 0x7f) != 0) && DAT_00523410 == 0 {
                    if u_var10 <= DAT_009ec5fe {
                        u_var10 = DAT_009ec5fe.wrapping_add(1);
                    }
                    b_var7 = (u_var10 as u8).wrapping_sub(DAT_009ec5fe as u8);
                    stb(p!(DAT_009e9683), b_var7);
                }
            } else {
                let mut u_var10: u16 = (b_var7 & 0x7f) as u16;
                if u_var10 < 0x7f {
                    if DAT_00523410 == 0 {
                        if 0x7fu32
                            < (DAT_009ec5fe as u32).wrapping_add((u_var10 as i16) as i32 as u32)
                        {
                            u_var10 = (0x7fi32.wrapping_sub(DAT_009ec5fe as i32)) as u16;
                        }
                        b_var7 = ((u_var10 as u8).wrapping_add(DAT_009ec5fe as u8)) | 0x80;
                        stb(p!(DAT_009e9683), b_var7);
                    }
                } else {
                    if ldu(p!(DAT_009e9a67)) != 0xffff {
                        let e = ent(ldu(p!(DAT_009e9a67)) as u32);
                        st32(p!(DAT_009e9a5b), ldi(e.wrapping_add(4)) as u32);
                        let e = ent(ldu(p!(DAT_009e9a67)) as u32);
                        st32(p!(DAT_009e9a5f), ldi(e.wrapping_add(8)) as u32);
                        let e = ent(ldu(p!(DAT_009e9a67)) as u32);
                        let i_var9 = ldi(e.wrapping_add(0xc));
                        st32(p!(DAT_009e9a63), i_var9 as u32);
                        st32(
                            p!(DAT_009e9a63),
                            (ldi(p!(DAT_009e9a86)) as u32).wrapping_add(i_var9 as u32),
                        );
                    }
                    st16(p!(DAT_009e9a67), 0xffff);
                }
            }
            if ldb(p!(DAT_009e9683)) != 0 {
                let mut u_var10: u16 = DAT_009ecc30;
                if (DAT_009ecc30 as i16) < 0 {
                    u_var10 = DAT_009ecc30.wrapping_neg();
                }
                let mut u_var14: u16 = DAT_009eca24;
                if (DAT_009eca24 as i16) < 0 {
                    u_var14 = DAT_009eca24.wrapping_neg();
                }
                if (u_var10 as i16) < 0x80 {
                    DAT_009ecc30 = 0;
                }
                if (u_var14 as i16) < 0x30 {
                    DAT_009eca24 = 0;
                }
            }
        }
        let mut s_var11: i16 = FUN_00481d70(DAT_009ecc30 as i16);
        let mut s_var17: i16 = FUN_00481d70(DAT_009eca24 as i16);
        if ldb(p!(DAT_009e9683)) & 0x80 == 0 {
            if lds(p!(DAT_009e9a67)) == -1 {
                if s_var17 != 0 || s_var11 != 0 {
                    FUN_00438330(s_var17, -(s_var11 as i32), param_1);
                }
            } else {
                st16(
                    p!(DAT_009e9a75),
                    lds(p!(DAT_009e9a75)).wrapping_add(s_var11) as u16,
                );
                st16(
                    p!(DAT_009e9a73),
                    lds(p!(DAT_009e9a73)).wrapping_add(s_var17) as u16,
                );
            }
        } else {
            let k = ((ldi(p!(DAT_009e9a63)) >> 0xe) as u32).wrapping_add(1);
            st32(
                p!(DAT_009e9a5b),
                (ldi(p!(DAT_009e9a5b)) as u32).wrapping_add(k.wrapping_mul(s_var11 as i32 as u32)),
            );
            let k = ((ldi(p!(DAT_009e9a63)) >> 0xe) as u32).wrapping_add(1);
            st32(
                p!(DAT_009e9a5f),
                (ldi(p!(DAT_009e9a5f)) as u32).wrapping_add(k.wrapping_mul(s_var17 as i32 as u32)),
            );
        }
        let u_var10: u16 = DAT_009ec474 & 0xf;
        if u_var10 != 1 && u_var10 != 2 {
            if ldb(p!(DAT_009e9683)) == 0 {
                st16(p!(DAT_009e9a82), 0x20);
                return;
            }
            let u = ldu(p!(DAT_009e9a82));
            if u < 0x101 {
                st16(p!(DAT_009e9a82), 0x20);
                return;
            }
            st16(
                p!(DAT_009e9a82),
                (u as u32).wrapping_sub((u >> 3) as u32).wrapping_sub(0x20) as u16,
            );
            return;
        }
        'lab: {
            if 1 < ldb(p!(DAT_009e9683)) {
                let puvar1 = p!(DAT_009e9a82);
                if ((ldui(p!(DAT_009e9a63)) & 0xfffffffe) as i32) < 0xe001 {
                    st16(puvar1, ((ldui(p!(DAT_009e9a63)) as i32) >> 1) as u16);
                } else {
                    st16(puvar1, 0x7000);
                }
                if ldu(puvar1) < 0x800 {
                    st16(puvar1, 0x800);
                }
                break 'lab;
            }
            if ldb(p!(DAT_009e9683)) == 1 {
                if lds(p!(DAT_009e9a67)) != -1 {
                    let puvar1 = p!(DAT_009e9a82);
                    if ((ldui(p!(DAT_009e9a86)) & 0xfffffffe) as i32) < 0xe001 {
                        st16(puvar1, ((ldui(p!(DAT_009e9a86)) as i32) >> 1) as u16);
                    } else {
                        st16(puvar1, 0x7000);
                    }
                    if ldu(puvar1) < 0x100 {
                        st16(puvar1, 0x100);
                    }
                    break 'lab;
                }
                if ldu(p!(DAT_009e9a69)) == 0xffff {
                    let puvar1 = p!(DAT_009e9a82);
                    let mut u_var14: u16 = (lds(p!(DAT_009e9a82)) as i32).wrapping_add(0x20) as u16;
                    st16(puvar1, u_var14);
                    u_var14 = ((u_var14 as u32).wrapping_add((u_var14 >> 3) as u32)) as u16;
                    st16(puvar1, u_var14);
                    if 0x4000 < u_var14 {
                        st16(puvar1, 0x4000);
                    }
                    break 'lab;
                }
                let i_var9 = ent(ldu(p!(DAT_009e9a69)) as u32) as i32;
                let puvar1 = p!(DAT_009e9a82);
                let r = FUN_0041f410(
                    (ldi(w(i_var9, 4)) as u32).wrapping_sub(ldi(p!(DAT_009e9a5b)) as u32) as i32,
                    (ldi(w(i_var9, 8)) as u32).wrapping_sub(ldi(p!(DAT_009e9a5f)) as u32) as i32,
                    (ldi(w(i_var9, 0xc)) as u32).wrapping_sub(ldi(p!(DAT_009e9a63)) as u32) as i32,
                );
                if r < 0x7001 {
                    st16(puvar1, r as u16);
                } else {
                    st16(puvar1, 0x7000);
                }
                if 0x3ff < ldu(puvar1) {
                    break 'lab;
                }
            } else {
                let s = lds(p!(DAT_009e9a82));
                st16(
                    p!(DAT_009e9a82),
                    (s as i32 as u32).wrapping_add(0x20) as u16,
                );
                if ((s as i32 as u32).wrapping_add(0x20) as u16) < 0x401 {
                    break 'lab;
                }
            }
            st16(p!(DAT_009e9a82), 0x400);
        }
        let psvar19 = p!(DAT_009e9a82);
        if u_var10 == 1 {
            if ldb(p!(DAT_009e9683)) == 0 {
                s_var11 = FUN_00481d70(lds(psvar19));
                let i_var9 =
                    (ldi(p!(DAT_009e9a86)) as u32).wrapping_sub(s_var11 as i32 as u32) as i32;
                st32(p!(DAT_009e9a86), i_var9 as u32);
                if i_var9 < 0x30 {
                    st32(p!(DAT_009e9a86), 0x30);
                    return;
                }
            } else if lds(p!(DAT_009e9a67)) == -1 {
                let s_var15: i16;
                let s_var18: i16;
                if lds(p!(DAT_009e9a69)) == -1 {
                    s_var15 = lds(p!(DAT_009e9a75));
                    s_var18 = lds(p!(DAT_009e9a73));
                    s_var11 = lds(p!(DAT_009e9a6d));
                    s_var17 = lds(p!(DAT_009e9a6b));
                } else {
                    s_var11 = lds(p!(DAT_009e9a6d));
                    s_var15 = 0;
                    s_var17 = lds(p!(DAT_009e9a6b));
                    s_var18 = 0;
                }
                FUN_00427940(0, s_var17, s_var11, 0, s_var18, s_var15, 0);
                let mut i_var12 = DAT_00a004b0;
                s_var11 = FUN_00481d70(lds(psvar19));
                let mut i_var9 = DAT_009fe7f4;
                st32(
                    p!(DAT_009e9a5b),
                    (ldi(p!(DAT_009e9a5b)) as u32).wrapping_add(mulsh(s_var11 as i32, i_var12)),
                );
                s_var11 = FUN_00481d70(lds(psvar19));
                i_var12 = DAT_00a00490;
                st32(
                    p!(DAT_009e9a5f),
                    (ldi(p!(DAT_009e9a5f)) as u32).wrapping_add(mulsh(s_var11 as i32, i_var9)),
                );
                s_var11 = FUN_00481d70(lds(psvar19));
                i_var9 = (ldi(p!(DAT_009e9a63)) as u32).wrapping_add(mulsh(s_var11 as i32, i_var12))
                    as i32;
                st32(p!(DAT_009e9a63), i_var9 as u32);
                if ldu(p!(DAT_009e9a69)) != 0xffff {
                    i_var12 = ent(ldu(p!(DAT_009e9a69)) as u32) as i32;
                    i_var12 = FUN_0041f410(
                        (ldi(w(i_var12, 4)) as u32).wrapping_sub(ldi(p!(DAT_009e9a5b)) as u32)
                            as i32,
                        (ldi(w(i_var12, 8)) as u32).wrapping_sub(ldi(p!(DAT_009e9a5f)) as u32)
                            as i32,
                        (ldi(w(i_var12, 0xc)) as u32).wrapping_sub(i_var9 as u32) as i32,
                    );
                    i_var9 = DAT_009fe7f4;
                    if i_var12 < tab(ldu(p!(DAT_009e9a69)) as u32) as i32 {
                        i_var12 =
                            (i_var12 as u32).wrapping_sub(tab(ldu(p!(DAT_009e9a67)) as u32)) as i32;
                        st32(
                            p!(DAT_009e9a5b),
                            (ldi(p!(DAT_009e9a5b)) as u32)
                                .wrapping_add(mulsh(i_var12, DAT_00a004b0)),
                        );
                        let i_var3 = DAT_00a00490;
                        st32(
                            p!(DAT_009e9a5f),
                            (ldi(p!(DAT_009e9a5f)) as u32).wrapping_add(mulsh(i_var12, i_var9)),
                        );
                        st32(
                            p!(DAT_009e9a63),
                            (ldi(p!(DAT_009e9a63)) as u32).wrapping_add(mulsh(i_var12, i_var3)),
                        );
                    }
                }
                if 1 < ldb(p!(DAT_009e9683)) && ldi(p!(DAT_009e9a63)) < 0x30 {
                    st32(p!(DAT_009e9a63), 0x30);
                }
                clamp(p!(DAT_009e9a5b));
                clamp(p!(DAT_009e9a5f));
                clamp(p!(DAT_009e9a63));
            } else {
                s_var11 = FUN_00481d70(lds(psvar19));
                st32(
                    p!(DAT_009e9a86),
                    (ldi(p!(DAT_009e9a86)) as u32).wrapping_sub(s_var11 as i32 as u32),
                );
                if ldi(p!(DAT_009e9a86)) < tab(ldu(p!(DAT_009e9a67)) as u32) as i32 {
                    st32(p!(DAT_009e9a86), tab(ldu(p!(DAT_009e9a67)) as u32));
                    return;
                }
            }
        } else if ldb(p!(DAT_009e9683)) == 0 {
            s_var11 = FUN_00481d70(lds(psvar19));
            let i_var9 = (ldi(p!(DAT_009e9a86)) as u32).wrapping_add(s_var11 as i32 as u32) as i32;
            st32(p!(DAT_009e9a86), i_var9 as u32);
            if 0x1400 < i_var9 {
                st32(p!(DAT_009e9a86), 0x1400);
                return;
            }
        } else {
            if lds(p!(DAT_009e9a67)) != -1 {
                s_var11 = FUN_00481d70(lds(psvar19));
                st32(
                    p!(DAT_009e9a86),
                    (ldi(p!(DAT_009e9a86)) as u32).wrapping_add(s_var11 as i32 as u32),
                );
                return;
            }
            let s_var15: i16;
            let s_var18: i16;
            if lds(p!(DAT_009e9a69)) == -1 {
                s_var15 = lds(p!(DAT_009e9a75));
                s_var18 = lds(p!(DAT_009e9a73));
                s_var11 = lds(p!(DAT_009e9a6d));
                s_var17 = lds(p!(DAT_009e9a6b));
            } else {
                s_var11 = lds(p!(DAT_009e9a6d));
                s_var15 = 0;
                s_var17 = lds(p!(DAT_009e9a6b));
                s_var18 = 0;
            }
            FUN_00427940(0, s_var17, s_var11, 0, s_var18, s_var15, 0);
            let mut i_var12 = DAT_00a004b0;
            s_var11 = FUN_00481d70(lds(psvar19));
            let i_var9 = DAT_009fe7f4;
            st32(
                p!(DAT_009e9a5b),
                (ldi(p!(DAT_009e9a5b)) as u32).wrapping_sub(mulsh(s_var11 as i32, i_var12)),
            );
            s_var11 = FUN_00481d70(lds(psvar19));
            i_var12 = DAT_00a00490;
            st32(
                p!(DAT_009e9a5f),
                (ldi(p!(DAT_009e9a5f)) as u32).wrapping_sub(mulsh(s_var11 as i32, i_var9)),
            );
            s_var11 = FUN_00481d70(lds(psvar19));
            st32(
                p!(DAT_009e9a63),
                (ldi(p!(DAT_009e9a63)) as u32).wrapping_sub(mulsh(s_var11 as i32, i_var12)),
            );
            clamp(p!(DAT_009e9a5b));
            clamp(p!(DAT_009e9a5f));
            clamp(p!(DAT_009e9a63));
        }
    }
}
