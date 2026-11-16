// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b69e30: i32;
    static mut DAT_00b6a24c: i32;
    static mut DAT_00aa5cb0: [i32; 0];
    static mut DAT_00aa5ac0: [i32; 0];
    static mut DAT_00aa5e10: [i32; 0];
    static mut DAT_00aa5e68: i32;
    static mut DAT_00aa5aac: i16;
    static mut DAT_00bb73a4: i8;
    static mut DAT_00a92203: [i8; 0];
    static mut DAT_00a91d24: i8;
    static mut DAT_00a91d17: [i8; 0];
    static mut DAT_00a91d12: [i8; 0];

    static mut DAT_00aa615e: [u8; 0];
    static mut DAT_00aa617a: [u8; 0];
    static mut DAT_00aa629e: [u8; 0];
    static mut DAT_00aa62ae: [u8; 0];
    static mut DAT_00b6bcbe: [u8; 0];
    static mut DAT_00b6bcc2: [u8; 0];
    static mut DAT_00b6bce2: [u8; 0];
    static mut DAT_00b6bd02: [u8; 0];
    static mut DAT_00b6bdc2: [u8; 0];
    static mut DAT_00b6be82: [u8; 0];
    static mut DAT_00b6bea2: [u8; 0];
    static mut DAT_00b6bec2: [u8; 0];
    static mut DAT_00b6bf82: [u8; 0];
    static mut DAT_00b6c102: [u8; 0];
    static mut DAT_00b6d84a: [u8; 0];
    static mut DAT_00b6d5a6: [u8; 0];
    static mut DAT_00b6d5ca: [u8; 0];
    static mut DAT_00b6d5ce: [u8; 0];
    static mut DAT_00b6a5a6: [u8; 0];
    static mut DAT_00b6a5aa: [u8; 0];
    static mut DAT_00b6a5ae: [u8; 0];
    static mut DAT_00b6a5b2: [u8; 0];
    static mut DAT_00b6a5b6: [u8; 0];
    static mut DAT_00b6a5ba: [u8; 0];
    static mut DAT_00b6a5be: [u8; 0];
    static mut DAT_00b6a606: [u8; 0];
    static mut DAT_00b6a60a: [u8; 0];
    static mut DAT_00b6baca: [u8; 0];
    static mut DAT_00b6bad2: [u8; 0];
    static mut DAT_00b6bb9a: [u8; 0];
    static mut DAT_00b6bb9e: [u8; 0];
    static mut DAT_00b6bc2a: [u8; 0];
    static mut DAT_00bb00f6: [u8; 0];
    static mut DAT_00b6d962: [u8; 0];
    static mut DAT_00b6d9aa: [u8; 0];
    static mut DAT_00b7e38a: [u8; 0];
    static mut DAT_00b8ed2a: [u8; 0];
    static mut DAT_00b8ed2e: [u8; 0];
    static mut DAT_00b8ed32: [u8; 0];
    static mut DAT_00b8ed6a: [u8; 0];
    static mut DAT_00b8ed6e: [u8; 0];
    static mut DAT_00b6d96a: [u8; 0];
    static mut DAT_00b6d96e: [u8; 0];
    static mut DAT_00b6d972: [u8; 0];
    static mut DAT_00b6d976: [u8; 0];
    static mut DAT_00b6d97a: [u8; 0];
    static mut DAT_00b6d97e: [u8; 0];
    static mut DAT_00b6d982: [u8; 0];
    static mut DAT_00b6d9ae: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d12f0(param_1: i32) -> *mut core::ffi::c_void;
    fn FUN_004d11e0() -> i32;
    fn FUN_004d1ca0();
}

macro_rules! a {
    ($x:ident) => {
        (&raw mut $x).cast::<u8>()
    };
}

unsafe fn rd(p: *const u8) -> i32 {
    // SAFETY: caller supplies a readable address in the game's memory.
    unsafe { p.cast::<i32>().read_unaligned() }
}

unsafe fn wr(p: *mut u8, v: i32) {
    // SAFETY: caller supplies a writable address in the game's memory.
    unsafe { p.cast::<i32>().write_unaligned(v) }
}

unsafe fn fill(mut p: *mut u8, n: i32) -> *mut u8 {
    let mut i = n;
    while i != 0 {
        // SAFETY: the original clears this many dwords here.
        unsafe { wr(p, 0) };
        p = p.wrapping_add(4);
        i -= 1;
    }
    p
}

fn off(base: *mut u8, o: u32) -> *mut u8 {
    base.wrapping_offset((o as i32) as isize)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004fad00() {
    // SAFETY: all globals and callees are the game's own; every address
    // accessed is the one the original accesses.
    unsafe {
        let mut local_c: i32;
        let local_18: i32 = -1;
        let local_1c: i32 = 1;

        if DAT_00b69e30 == 2 {
            let mut p = a!(DAT_00aa617a);
            loop {
                let p10 = p.wrapping_add(0x2a);
                p = fill(p, 10);
                p.cast::<i16>().write_unaligned(0);
                p = p10;
                if !((p as usize) < (a!(DAT_00aa629e).wrapping_add(2) as usize)) {
                    break;
                }
            }
        } else {
            let mut p13 = a!(DAT_00aa615e);
            loop {
                let local_30 = rd(p13);
                if local_30 != 0 && FUN_004d12f0(local_30).is_null() {
                    let p = fill(p13.wrapping_sub(0xe), 10);
                    p.cast::<i16>().write_unaligned(0);
                }
                p13 = p13.wrapping_add(0x2a);
                if !((p13 as usize) < (a!(DAT_00aa62ae) as usize)) {
                    break;
                }
            }
        }
        wr(a!(DAT_00b6bcbe), 0);
        DAT_00b6a24c = 0;
        fill(a!(DAT_00b6bcc2), 8);
        fill(a!(DAT_00b6bce2), 8);
        fill(a!(DAT_00b6bd02), 0x30);
        fill(a!(DAT_00b6bdc2), 0x30);
        fill(a!(DAT_00b6be82), 8);
        fill(a!(DAT_00b6bea2), 8);
        fill(a!(DAT_00b6bec2), 0x30);
        fill(a!(DAT_00b6bf82), 0x30);
        fill(a!(DAT_00b6c102), 0x522);
        fill(a!(DAT_00b6d84a), 0x46);
        local_c = 0;
        let mut local_24 = rd(a!(DAT_00b6a5aa));
        let mut local_28 = rd(a!(DAT_00b6a606));
        let mut local_2c = rd(a!(DAT_00b6baca));
        if local_24 == 1 && local_28 == 1 && local_2c == 0 {
            let mut p = a!(DAT_00b6bad2);
            loop {
                wr(p, local_18);
                p = p.wrapping_add(20);
                if !((p as usize) < (a!(DAT_00b6bb9a) as usize)) {
                    break;
                }
            }
        }
        let mut local_4: i32 = 0;
        let mut local_10 = a!(DAT_00aa615e);
        let mut p13 = a!(DAT_00b6d5ce);
        loop {
            let local_30 = rd(local_10);
            if local_30 != 0 {
                let u_var2 = rd(local_10.wrapping_sub(10));
                local_c = local_c.wrapping_add(1);
                let u_var3 = rd(local_10.wrapping_sub(6));
                wr(p13.wrapping_sub(0x36), rd(local_10.wrapping_sub(0xe)));
                let u_var1 = *local_10.wrapping_sub(2);
                wr(p13.wrapping_sub(0x32), u_var2);
                wr(p13.wrapping_sub(0x2e), u_var3);
                let i_var7 = rd(local_10.wrapping_add(8));
                *p13.wrapping_sub(0x2a) = u_var1;
                let i_var12 = rd(local_10.wrapping_add(12));
                *p13.wrapping_sub(0x2a) = 0;
                wr(p13.wrapping_sub(4), i_var7);
                wr(p13, i_var12);
                let mut local_34 = rd(local_10.wrapping_add(16)).wrapping_sub(1);
                wr(p13.wrapping_add(4), local_34);
                let i_var7 = rd(local_10);
                let i_var12 = rd(local_10.wrapping_add(4));
                local_34 = rd(local_10.wrapping_add(20)).wrapping_sub(1);
                wr(p13.wrapping_add(8), local_34);
                local_34 = rd(local_10.wrapping_add(24)).wrapping_sub(1);
                wr(p13.wrapping_add(12), local_34);
                wr(p13.wrapping_sub(36), i_var7);
                wr(p13.wrapping_sub(32), i_var12);
                wr(p13.wrapping_sub(28), 0);
                wr(p13.wrapping_sub(24), 0);
                wr(p13.wrapping_sub(20), 0);
                wr(p13.wrapping_sub(16), 0);
                wr(p13.wrapping_sub(12), 0);
                wr(p13.wrapping_sub(8), 0);
                wr(p13.wrapping_add(16), 0);
                let mut local_8 = 10;
                let mut i_var12: i32 = 0;
                loop {
                    let mut i_var11 = 8;
                    loop {
                        if rd(a!(DAT_00aa5cb0).wrapping_offset(i_var12 as isize)) == i_var7 {
                            let i_var4 = rd(a!(DAT_00aa5ac0).wrapping_offset(i_var12 as isize));
                            wr(p13.wrapping_sub(40), i_var4);
                            let local_38 = rd(p13.wrapping_sub(4));
                            let local_3c = rd(p13);
                            if local_38 == 0 && local_3c != -1 {
                                let o = (local_3c as u32)
                                    .wrapping_add((i_var4 as u32).wrapping_mul(0x562));
                                if *off(a!(DAT_00a92203), o) == 0 {
                                    wr(p13, local_18);
                                }
                            }
                        }
                        i_var12 = i_var12.wrapping_add(4);
                        i_var11 -= 1;
                        if i_var11 == 0 {
                            break;
                        }
                    }
                    local_8 -= 1;
                    if local_8 == 0 {
                        break;
                    }
                }
                let r = FUN_004d11e0();
                if r == i_var7 {
                    DAT_00b6a24c = local_4;
                }
            }
            p13 = p13.wrapping_add(0x58);
            local_4 = local_4.wrapping_add(1);
            local_10 = local_10.wrapping_add(0x2a);
            if !((local_10 as usize) < (a!(DAT_00aa62ae) as usize)) {
                break;
            }
        }

        local_24 = rd(a!(DAT_00b6a5aa));
        local_28 = rd(a!(DAT_00b6a606));
        if local_24 == 3 && local_28 == 1 {
            wr(a!(DAT_00b6bc2a), local_c);
        }
        if local_24 == 0 && local_28 == 1 {
            wr(a!(DAT_00bb00f6), local_c);
        }
        'l188: {
            let mut go_lab = true;
            if local_24 == 1 {
                local_2c = rd(a!(DAT_00b6baca));
                if local_28 == 1 && local_2c == 0 {
                    if 0 < DAT_00aa5e68 {
                        let mut p = a!(DAT_00b6bad2);
                        let mut q13 = (&raw mut DAT_00aa5cb0).cast::<i32>();
                        let mut q14 = (&raw mut DAT_00aa5e10).cast::<i32>();
                        let mut i_var7 = DAT_00aa5e68;
                        loop {
                            let mut i_var11 = 0;
                            let i_var12 = q14.read_unaligned();
                            let mut q6 = q13;
                            if 0 < i_var12 {
                                loop {
                                    if q6.read_unaligned() != 0 {
                                        break;
                                    }
                                    i_var11 += 1;
                                    q6 = q6.wrapping_add(1);
                                    if !(i_var11 < i_var12) {
                                        break;
                                    }
                                }
                                if i_var11 < i_var12 {
                                    wr(p, 0);
                                }
                            }
                            p = p.wrapping_add(20);
                            q13 = q13.wrapping_add(8);
                            q14 = q14.wrapping_add(1);
                            i_var7 -= 1;
                            if i_var7 == 0 {
                                break;
                            }
                        }
                    }
                    if (DAT_00bb73a4 == 1 || DAT_00b69e30 == 2) && 0 < DAT_00aa5aac {
                        let mut pc8 = (&raw mut DAT_00a91d24).cast::<u8>();
                        let mut i_var7 = DAT_00aa5aac as i32;
                        loop {
                            if *pc8 != 0 {
                                let b = *pc8.wrapping_sub(0xc) as u32;
                                wr(off(a!(DAT_00b6bad2), b.wrapping_mul(20)), 0);
                            }
                            pc8 = pc8.wrapping_add(0x562);
                            i_var7 -= 1;
                            if i_var7 == 0 {
                                break;
                            }
                        }
                    }
                    wr(a!(DAT_00b6bb9a), local_c);
                    if DAT_00b69e30 == 2 || DAT_00bb73a4 != 0 {
                        wr(a!(DAT_00b6bb9e), DAT_00aa5e68);
                    } else {
                        wr(a!(DAT_00b6bb9e), 0);
                        if 0 < DAT_00aa5e68 {
                            let mut p = a!(DAT_00b6bad2);
                            let mut q13 = (&raw mut DAT_00aa5cb0).cast::<i32>();
                            let mut q14 = (&raw mut DAT_00aa5e10).cast::<i32>();
                            let mut local_8 = DAT_00aa5e68;
                            loop {
                                let mut i_var12 = 0;
                                let i_var7 = q14.read_unaligned();
                                let mut q6 = q13;
                                if 0 < i_var7 {
                                    loop {
                                        if q6.read_unaligned() != 0 {
                                            break;
                                        }
                                        i_var12 += 1;
                                        q6 = q6.wrapping_add(1);
                                        if !(i_var12 < i_var7) {
                                            break;
                                        }
                                    }
                                    if i_var12 < i_var7 {
                                        let mut local_40 = rd(a!(DAT_00b6bb9e));
                                        local_40 = local_40.wrapping_add(1);
                                        wr(a!(DAT_00b6bb9e), local_40);
                                        wr(p, 0);
                                    }
                                }
                                p = p.wrapping_add(20);
                                q13 = q13.wrapping_add(8);
                                q14 = q14.wrapping_add(1);
                                local_8 -= 1;
                                if local_8 == 0 {
                                    break;
                                }
                            }
                        }
                    }
                }
                local_24 = rd(a!(DAT_00b6a5aa));
                if local_24 == 1 {
                    go_lab = false;
                }
            }
            if go_lab {
                local_24 = rd(a!(DAT_00b6a5aa));
                if local_24 != 2 {
                    let idx = (DAT_00b6a24c as u32).wrapping_mul(88);
                    let local_44 = rd(off(a!(DAT_00b6d5a6), idx));
                    let o = (local_44 as u32).wrapping_mul(0x562);
                    let u_var9 = *off(a!(DAT_00a91d17), o) as u32;
                    wr(a!(DAT_00b6d962), u_var9 as i32);
                    break 'l188;
                }
            }
            let idx = (DAT_00b6a24c as u32).wrapping_mul(88);
            let mut u_var9 = rd(off(a!(DAT_00b6d5ca), idx)) as u32;
            if u_var9 == 0 {
                let i_var7 = rd(off(a!(DAT_00b6d5ce), idx));
                if i_var7 == -1 || 9 < i_var7 {
                    let local_44 = rd(off(a!(DAT_00b6d5a6), idx));
                    let o = (local_44 as u32).wrapping_mul(0x562);
                    u_var9 = *off(a!(DAT_00a91d12), o) as u32;
                } else {
                    let local_44 = rd(off(a!(DAT_00b6d5a6), idx));
                    let o = (i_var7 as u32).wrapping_add((local_44 as u32).wrapping_mul(0x562));
                    u_var9 = *off(a!(DAT_00a92203), o) as u32;
                }
            }
            if (u_var9 as i32) < 1 || (4 < (u_var9 as i32) && u_var9 != 0xe) {
                wr(a!(DAT_00b6d962), local_1c);
                wr(a!(DAT_00b6d9aa), 0);
            } else {
                wr(a!(DAT_00b6d962), 0);
                wr(a!(DAT_00b7e38a), 0);
            }
        }
        if DAT_00b69e30 == 2 {
            let mut l4c = rd(a!(DAT_00b6d962));
            wr(
                off(a!(DAT_00b6d96a), (l4c as u32).wrapping_mul(0x109e0)),
                rd(a!(DAT_00b6a5a6)),
            );
            l4c = rd(a!(DAT_00b6d962));
            wr(
                off(a!(DAT_00b6d96e), (l4c as u32).wrapping_mul(0x109e0)),
                rd(a!(DAT_00b6a5aa)),
            );
            local_24 = rd(a!(DAT_00b6a5aa));
            if local_24 == 0 {
                l4c = rd(a!(DAT_00b6d962));
                wr(
                    off(a!(DAT_00b6d972), (l4c as u32).wrapping_mul(0x109e0)),
                    rd(a!(DAT_00b6a5ae)),
                );
            } else if local_24 == 1 || local_24 == 2 {
                l4c = rd(a!(DAT_00b6d962));
                wr(
                    off(a!(DAT_00b6d976), (l4c as u32).wrapping_mul(0x109e0)),
                    rd(a!(DAT_00b6a5b2)),
                );
                l4c = rd(a!(DAT_00b6d962));
                wr(
                    off(a!(DAT_00b6d97a), (l4c as u32).wrapping_mul(0x109e0)),
                    rd(a!(DAT_00b6a5b6)),
                );
            } else {
                l4c = rd(a!(DAT_00b6d962));
                wr(
                    off(a!(DAT_00b6d97e), (l4c as u32).wrapping_mul(0x109e0)),
                    rd(a!(DAT_00b6a5ba)),
                );
                l4c = rd(a!(DAT_00b6d962));
                wr(
                    off(a!(DAT_00b6d982), (l4c as u32).wrapping_mul(0x109e0)),
                    rd(a!(DAT_00b6a5be)),
                );
            }
            l4c = rd(a!(DAT_00b6d962));
            wr(
                off(a!(DAT_00b6d9aa), (l4c as u32).wrapping_mul(0x109e0)),
                rd(a!(DAT_00b6a606)),
            );
            wr(a!(DAT_00b8ed6a), 0);
            l4c = rd(a!(DAT_00b6d962));
            wr(
                off(a!(DAT_00b6d9ae), (l4c as u32).wrapping_mul(0x109e0)),
                rd(a!(DAT_00b6a60a)),
            );
        } else {
            wr(a!(DAT_00b8ed2a), rd(a!(DAT_00b6a5a6)));
            wr(a!(DAT_00b8ed2e), rd(a!(DAT_00b6a5aa)));
            let mut src = a!(DAT_00b6a5ae);
            let mut dst = a!(DAT_00b8ed32);
            let mut i = 6;
            while i != 0 {
                wr(dst, rd(src));
                src = src.wrapping_add(4);
                dst = dst.wrapping_add(4);
                i -= 1;
            }
            wr(a!(DAT_00b8ed6a), rd(a!(DAT_00b6a606)));
            wr(a!(DAT_00b8ed6e), rd(a!(DAT_00b6a60a)));
            wr(a!(DAT_00b6d9aa), 0);
            wr(a!(DAT_00b7e38a), 0);
        }
        FUN_004d1ca0();
    }
}
