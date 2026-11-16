// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b69e30: i32;
    static mut DAT_00b6a5a6: [u8; 4];
    static mut DAT_00b6a5aa: [u8; 4];
    static mut DAT_00b6a606: [u8; 4];
    static mut DAT_00b6baca: [u8; 4];
    static mut DAT_00b6d962: [u8; 4];

    static mut DAT_00aa5ac0: [i32; 1];
    static mut DAT_00aa5cb0: [i32; 1];

    static mut DAT_00aa6104: i32;
    static mut DAT_00aa60f8: i32;
    static mut DAT_00aa6100: i32;
    static mut DAT_00aa60d8: i32;
    static mut DAT_00aa60e4: i32;
    static mut DAT_00aa610c: i32;
    static mut DAT_00aa60e8: u32;
    static mut DAT_00aa60ec: u32;
    static mut DAT_00aa6108: i32;
    static mut DAT_00aa60f0: i32;
    static mut DAT_00aa60d4: i32;
    static mut DAT_00aa60dc: i32;
    static mut DAT_00aa60e0: i32;

    static mut DAT_00a91d21: [u8; 1];
    static mut DAT_00a91d13: [u8; 1];
    static mut DAT_00a92202: [u8; 1];
    static mut DAT_00a92203: [u8; 1];
    static mut DAT_00a921f0: [u8; 1];
    static mut DAT_00a91d15: [u8; 1];
    static mut DAT_00a921f8: [u8; 1];
    static mut DAT_00a91d16: [u8; 1];
    static mut DAT_00a921fe: [u8; 1];
    static mut DAT_00a921e3: [u8; 1];
    static mut DAT_00a9220c: [u8; 1];
    static mut DAT_00a92216: [u8; 1];
    static mut DAT_00a91d12: [u8; 1];
    static mut DAT_0052c550: [u8; 1];
    static mut DAT_0052c564: [u32; 1];
    static mut DAT_0052c578: [u32; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d11e0() -> i32;
    fn FUN_004ee8e0(param_1: i32) -> u32;
}

unsafe fn rd<T>(p: *const T, i: i32) -> T {
    // SAFETY: the pointer is one of the game's own tables; the index is the original's.
    unsafe { p.offset(i as isize).read_unaligned() }
}

unsafe fn rd32(p: *const u8) -> i32 {
    // SAFETY: the pointer addresses four bytes of the game's own memory.
    unsafe { p.cast::<i32>().read_unaligned() }
}

unsafe fn clear() {
    // SAFETY: the globals are the game's own memory.
    unsafe {
        DAT_00aa60d8 = 0;
        DAT_00aa60e4 = 0;
        DAT_00aa60f8 = 0;
        DAT_00aa6100 = 0;
        DAT_00aa610c = 0;
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004ee1a0() {
    // SAFETY: all globals and callees are the game's own; tables are read
    // at the indices the original computes.
    unsafe {
        let ac0 = (&raw mut DAT_00aa5ac0).cast::<i32>() as *const i32;
        let cb0 = (&raw mut DAT_00aa5cb0).cast::<i32>() as *const i32;
        let sel = (&raw mut DAT_00b6a5a6).cast::<u8>() as *const u8;

        if DAT_00b69e30 == 2 {
            let i8_ = rd32(sel);
            DAT_00aa6104 = rd(ac0, i8_.wrapping_mul(8));
        } else {
            let mut i6: i32 = 0;
            loop {
                let i8_ = rd32(sel);
                let i4 = rd(cb0, i6.wrapping_add(i8_.wrapping_mul(8)));
                let i3 = FUN_004d11e0();
                if i4 == i3 {
                    let i8_ = rd32(sel);
                    DAT_00aa6104 = rd(ac0, i6.wrapping_add(i8_.wrapping_mul(8)));
                }
                i6 = i6.wrapping_add(1);
                if i6 >= 8 {
                    break;
                }
            }
        }
        DAT_00aa60f8 = 0;
        DAT_00aa6100 = 0;
        DAT_00aa60d8 = 0;
        DAT_00aa60e4 = 0;
        DAT_00aa610c = 0;
        let i6 = (DAT_00aa6104 as u32).wrapping_mul(0x562) as i32;
        let p21 = (&raw mut DAT_00a91d21).cast::<u8>() as *const u8;
        let p13 = (&raw mut DAT_00a91d13).cast::<u8>() as *const u8;
        let p2202 = (&raw mut DAT_00a92202).cast::<u8>() as *const u8;
        let p2203 = (&raw mut DAT_00a92203).cast::<u8>() as *const u8;
        DAT_00aa60e8 = rd(p21, i6) as u32;
        DAT_00aa60ec = rd(p13, i6) as u32;
        match rd(p2202, i6) {
            0 => {
                DAT_00aa6108 = 0;
                DAT_00aa60f0 = 1;
            }
            1 => {
                DAT_00aa6108 = 0xb;
                DAT_00aa60f0 = 0;
            }
            2 | 3 => {
                DAT_00aa6108 = 6;
                DAT_00aa60f0 = 0;
            }
            4 => {
                DAT_00aa60f0 = 1;
                let mut i4: i32 = 0;
                DAT_00aa6108 = 0;
                loop {
                    if rd(p2203, i4.wrapping_add(i6)) != 0 {
                        DAT_00aa60f0 = DAT_00aa60f0.wrapping_add(1);
                    }
                    i4 = i4.wrapping_add(1);
                    if i4 >= 10 {
                        break;
                    }
                }
            }
            _ => {}
        }

        let p1f0 = (&raw mut DAT_00a921f0).cast::<u8>() as *const u8;
        let p1d15 = (&raw mut DAT_00a91d15).cast::<u8>() as *const u8;
        DAT_00aa60d4 = 0;
        let mut i4: i32 = 0;
        loop {
            if rd(p1f0, i4.wrapping_add(i6)) != 0 {
                DAT_00aa60d4 = DAT_00aa60d4.wrapping_add(1);
            }
            i4 = i4.wrapping_add(1);
            if i4 >= 8 {
                break;
            }
        }
        if rd(p1d15, i6) != 0 || DAT_00aa60d4 != 0 {
            DAT_00aa60d4 = DAT_00aa60d4.wrapping_add(1);
        }

        let p1f8 = (&raw mut DAT_00a921f8).cast::<u8>() as *const u8;
        let p1d16 = (&raw mut DAT_00a91d16).cast::<u8>() as *const u8;
        DAT_00aa60dc = 0;
        let mut i4: i32 = 0;
        loop {
            if rd(p1f8, i4.wrapping_add(i6)) != 0 {
                DAT_00aa60dc = DAT_00aa60dc.wrapping_add(1);
            }
            i4 = i4.wrapping_add(1);
            if i4 >= 6 {
                break;
            }
        }
        if rd(p1d16, i6) != 0 || DAT_00aa60dc != 0 {
            DAT_00aa60dc = DAT_00aa60dc.wrapping_add(1);
        }

        let p1fe = (&raw mut DAT_00a921fe).cast::<u8>() as *const u8;
        let p1e3 = (&raw mut DAT_00a921e3).cast::<u8>() as *const u8;
        DAT_00aa60e0 = 0;
        let mut i4: i32 = 0;
        loop {
            if rd(p1fe, i4.wrapping_add(i6)) != 0 {
                DAT_00aa60e0 = DAT_00aa60e0.wrapping_add(1);
            }
            i4 = i4.wrapping_add(1);
            if i4 >= 4 {
                break;
            }
        }
        if rd(p1e3, i6) != 0 || DAT_00aa60e0 != 0 {
            DAT_00aa60e0 = DAT_00aa60e0.wrapping_add(1);
        }

        if rd32((&raw mut DAT_00b6a606).cast::<u8>() as *const u8) != 1 {
            clear();
            return;
        }
        let i8_ = rd32((&raw mut DAT_00b6a5aa).cast::<u8>() as *const u8);
        if i8_ != 1 && i8_ != 2 {
            clear();
            return;
        }
        if rd32((&raw mut DAT_00b6baca).cast::<u8>() as *const u8) == 0 {
            clear();
            return;
        }

        let mut u7: u32 = 0;
        let u5 = FUN_004ee8e0(-1);
        let i9 = rd32((&raw mut DAT_00b6d962).cast::<u8>() as *const u8);
        let cond = (u5 as i32) < 1 || (4 < (u5 as i32) && u5 != 0xe);
        let skip = if cond { i9 != 0 } else { i9 != 1 };
        if !skip {
            u7 = rd((&raw mut DAT_0052c550).cast::<u8>() as *const u8, u5 as i32) as u32;
        }

        if u7 != 0 {
            let p220c = (&raw mut DAT_00a9220c).cast::<u8>() as *const u8;
            let p2216 = (&raw mut DAT_00a92216).cast::<u8>() as *const u8;
            let p1d12 = (&raw mut DAT_00a91d12).cast::<u8>() as *const u8;
            let t564 = (&raw mut DAT_0052c564).cast::<u32>() as *const u32;
            let t578 = (&raw mut DAT_0052c578).cast::<u32>() as *const u32;
            loop {
                let u5 = FUN_004ee8e0(-1);
                if u7 == u5 {
                    break;
                }
                if DAT_00aa6108 == 0 {
                    DAT_00aa60f8 = DAT_00aa60f8.wrapping_add(1);
                    if DAT_00aa60f0 <= DAT_00aa60f8 {
                        DAT_00aa60f8 = 0;
                    }
                    let b1: u8;
                    let b2: u8;
                    if DAT_00aa60f8 == 0 {
                        let idx = (DAT_00aa6104 as u32).wrapping_mul(0x562) as i32;
                        b1 = rd(p13, idx);
                        b2 = rd(p21, idx);
                    } else {
                        let idx = ((DAT_00aa6104 as u32).wrapping_mul(0x562))
                            .wrapping_add(DAT_00aa60f8 as u32)
                            as i32;
                        b1 = rd(p220c, idx);
                        b2 = rd(p2216, idx);
                    }
                    DAT_00aa60ec = b1 as u32;
                    DAT_00aa60e8 = b2 as u32;
                } else {
                    DAT_00aa6100 = DAT_00aa6100.wrapping_add(1);
                    if DAT_00aa6108 <= DAT_00aa6100 {
                        DAT_00aa6100 = 0;
                    }
                    if DAT_00aa6100 != 0 {
                        let i6 = (DAT_00aa6104 as u32).wrapping_mul(0x562) as i32;
                        let b1 = rd(p2202, i6);
                        if b1 != 0 {
                            if b1 < 3 {
                                if rd(p1d12, i6) as u32 == rd(t564, DAT_00aa6100) {
                                    DAT_00aa6100 = DAT_00aa6100.wrapping_add(1);
                                }
                            } else if b1 == 3 && rd(p1d12, i6) as u32 == rd(t578, DAT_00aa6100) {
                                DAT_00aa6100 = DAT_00aa6100.wrapping_add(1);
                            }
                        }
                    }
                    if DAT_00aa6108 <= DAT_00aa6100 {
                        DAT_00aa6100 = 0;
                    }
                }
            }
        }
    }
}
