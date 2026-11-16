// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b6a5a6: [u8; 0];
    static mut DAT_00b6a5aa: [u8; 0];
    static mut DAT_00b6a5ae: [u8; 0];
    static mut DAT_00b6a606: [u8; 0];
    static mut DAT_00b6a60a: [u8; 0];
    static mut DAT_00b6d962: [u8; 0];
    static mut DAT_00b6d96a: [u8; 0];
    static mut DAT_00b6d96e: [u8; 0];
    static mut DAT_00b6d972: [u8; 0];
    static mut DAT_00b6d9aa: [u8; 0];
    static mut DAT_00b6d9ae: [u8; 0];
    static mut DAT_00b7e34a: [u8; 0];
    static mut DAT_00b7e34e: [u8; 0];
    static mut DAT_00b7e352: [u8; 0];

    static mut DAT_00b69e30: i32;
    static mut DAT_00665d10: i32;
    static mut DAT_00aa6114: i32;
    static mut DAT_00aa6118: i32;
    static mut DAT_00aa6134: u32;
    static mut DAT_00aa6138: u32;
    static mut DAT_00aa5aac: i16;
    static mut DAT_0052c208: i32;

    static mut DAT_00bb7394: i8;
    static mut DAT_00bb7397: i8;
    static mut DAT_00bb7398: i8;
    static mut DAT_00bb739d: i8;
    static mut DAT_00bb739e: i8;
    static mut DAT_00bb73a6: i8;
    static mut DAT_00bb73a7: i8;

    static mut DAT_00bb0106: [i8; 0];
    static mut DAT_00bb010a: [i8; 0];
    static mut DAT_00bb010e: [i8; 0];
    static mut DAT_00bb10a6: [i8; 0];
    static mut DAT_00bb10aa: [i8; 0];
    static mut DAT_00bb10ae: [i8; 0];
    static mut DAT_00bb2046: [i8; 0];
    static mut DAT_00bb204a: [i8; 0];
    static mut DAT_00bb242e: [i8; 0];
    static mut DAT_00bb2432: [i8; 0];
    static mut DAT_00bb260e: [i8; 0];
    static mut DAT_00bb2612: [i8; 0];
    static mut DAT_00a91d24: i8;
    static mut DAT_00a91d17: [i8; 0];

    static mut DAT_0052ad74: [i8; 0];
    static mut DAT_0052b590: [i8; 0];
    static mut DAT_0052b598: [i8; 0];
    static mut DAT_0052b5a0: [i8; 0];
    static mut DAT_0052b5a8: [i8; 0];
    static mut DAT_0052bf74: [i8; 0];
    static mut DAT_0052c250: [i8; 0];
    static mut DAT_0052c258: [i8; 0];
    static mut DAT_0052c260: [i8; 0];
    static mut DAT_0052c268: [i8; 0];
    static mut DAT_0052c270: [i8; 0];
    static mut DAT_0052c278: [i8; 0];
    static mut DAT_0052c280: [i8; 0];
    static mut DAT_0052c288: [i8; 0];
    static mut DAT_0052c290: [i8; 0];
    static mut DAT_0052c298: [i8; 0];
    static mut DAT_0052c2a0: [i8; 0];
    static mut DAT_0052c2a8: [i8; 0];
    static mut DAT_0052c2b0: [i8; 0];
    static mut DAT_0052c2b8: [i8; 0];
    static mut DAT_0052c2c0: [i8; 0];
    static mut DAT_0052c2c8: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004de090(a: *mut i32, b: *mut i32);
    fn FUN_004daba0(a: *mut i32);
    fn FUN_004cfee0() -> i32;
    fn FUN_004d6460(a: *mut i32, b: i32, c: i32, d: i32, e: i32);
    fn FUN_004ddc10(a: i32) -> *mut i8;
    fn FUN_004da780(
        a: *mut i32,
        b: *mut i8,
        c: *mut i8,
        d: *mut i8,
        e: i32,
        f: i32,
        g: i32,
        h: *mut i8,
    ) -> i32;
    fn FUN_004daa20(a: *mut i32, b: *mut i8, c: *mut i8, d: i32);
    fn FUN_004d64a0(a: *mut i32, b: i32, c: i32);
    fn FUN_004e5000(a: u32);
    fn FUN_004f70f0();
    fn FUN_004f36d0();
    fn FUN_004e5590(a: i32);
    fn FUN_004d6b90(a: *mut i32, b: i32, c: i32) -> i32;
    fn FUN_004dc240() -> u8;
    fn FUN_004dc220() -> u8;
    fn FUN_004de5a0(a: *mut i8, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32;
    fn FUN_004e6310(a: i32);
    fn FUN_004b4bf0(a: *mut i8);
    fn FUN_004e9330();
}

macro_rules! pb {
    ($s:ident) => {
        (&raw mut $s).cast::<u8>()
    };
}
macro_rules! pc {
    ($s:ident) => {
        (&raw mut $s).cast::<i8>()
    };
}

unsafe fn rd(p: *const u8) -> u32 {
    // SAFETY: the address is game memory read where the original reads it.
    unsafe { p.cast::<u32>().read_unaligned() }
}

unsafe fn wr(p: *mut u8, v: u32) {
    // SAFETY: the address is game memory written where the original writes it.
    unsafe { p.cast::<u32>().write_unaligned(v) }
}

unsafe fn rd8(p: *const u8) -> u8 {
    // SAFETY: the address is game memory read where the original reads it.
    unsafe { p.read() }
}

fn at(p: *mut u8, o: u32) -> *mut u8 {
    p.wrapping_add(o as usize)
}

unsafe fn item_ptr() -> *mut u8 {
    // SAFETY: game globals addressed as the original does.
    unsafe {
        let t = rd(pb!(DAT_00b6a5aa));
        at(pb!(DAT_00b6a5ae), t.wrapping_mul(4))
    }
}

unsafe fn cur_item() -> u32 {
    // SAFETY: reads the game's current item slot.
    unsafe { rd(item_ptr()) }
}

unsafe fn set_item(v: u32) {
    // SAFETY: writes the game's current item slot.
    unsafe { wr(item_ptr(), v) }
}

unsafe fn rec(i: u32) -> *mut u8 {
    // SAFETY: reads a game global; the record pointer is the game's own.
    let b = unsafe { DAT_00aa6114 } as u32;
    b.wrapping_add(i.wrapping_mul(0x148)) as usize as *mut u8
}

unsafe fn rec_item(i: u32) -> u32 {
    // SAFETY: reads a field of the game's record array.
    unsafe { rd(rec(i).wrapping_add(0x140)) }
}

unsafe fn rec_blk(i: u32) -> u32 {
    // SAFETY: reads a field of the game's record array.
    unsafe { rd(rec(i).wrapping_add(0x144)) }
}

unsafe fn calls3() {
    // SAFETY: the callees and global are the game's own.
    unsafe {
        FUN_004f70f0();
        FUN_004f36d0();
        FUN_004e5590(DAT_00aa6118);
    }
}

unsafe fn snd() {
    // SAFETY: game globals and callee.
    unsafe {
        if DAT_00bb73a7 != 0 {
            FUN_004de5a0(pc!(DAT_0052ad74), 1, 0, 0xff, 0, 0);
        }
    }
}

unsafe fn scan() {
    // SAFETY: walks the game's record array like the original.
    unsafe {
        let mut p = (DAT_00aa6114 as u32).wrapping_add(0x140);
        loop {
            let item = cur_item();
            if rd(p as usize as *const u8) == item {
                break;
            }
            p = p.wrapping_add(0x148);
            DAT_00aa6134 = DAT_00aa6134.wrapping_add(1);
            if DAT_00aa6134 >= DAT_00aa6138 {
                break;
            }
        }
    }
}

unsafe fn up_scan() -> bool {
    // SAFETY: walks the game's record array like the original.
    unsafe {
        let mut p = (DAT_00aa6114 as u32).wrapping_add(0x140);
        loop {
            let item = cur_item();
            if rd(p as usize as *const u8) == item {
                return rec_blk(DAT_00aa6134) != 0;
            }
            p = p.wrapping_add(0x148);
            DAT_00aa6134 = DAT_00aa6134.wrapping_add(1);
            if DAT_00aa6134 >= DAT_00aa6138 {
                return false;
            }
        }
    }
}

unsafe fn fix() {
    // SAFETY: walks the game's record array like the original.
    unsafe {
        let mut u: u32 = 0;
        if (DAT_00aa6138 as i32) >= 1 {
            let mut p = (DAT_00aa6114 as u32).wrapping_add(0x144);
            loop {
                if rd(p as usize as *const u8) == 0 {
                    let v = rec_item(u);
                    set_item(v);
                    DAT_00aa6134 = u;
                    break;
                }
                p = p.wrapping_add(0x148);
                u = u.wrapping_add(1);
                if (DAT_00aa6138 as i32) <= (u as i32) {
                    break;
                }
            }
        }
        calls3();
    }
}

unsafe fn next_core() {
    // SAFETY: game globals and callees.
    unsafe {
        snd();
        loop {
            if DAT_00aa6138.wrapping_sub(DAT_00aa6134) == 1 {
                DAT_00aa6134 = 0;
            } else {
                DAT_00aa6134 = DAT_00aa6134.wrapping_add(1);
            }
            if rec_blk(DAT_00aa6134) == 0 {
                break;
            }
        }
        DAT_00bb73a6 = 1;
        let v = rec_item(DAT_00aa6134);
        set_item(v);
        calls3();
        DAT_0052c208 = 0;
        if DAT_00aa6114 != 0 {
            DAT_00aa6134 = 0;
            if DAT_00aa6138 != 0 {
                scan();
            }
        }
    }
}

unsafe fn prev_core() {
    // SAFETY: game globals and callees.
    unsafe {
        snd();
        loop {
            if DAT_00aa6134 == 0 {
                DAT_00aa6134 = DAT_00aa6138;
            }
            DAT_00aa6134 = DAT_00aa6134.wrapping_sub(1);
            if rec_blk(DAT_00aa6134) == 0 {
                break;
            }
        }
        DAT_00bb73a6 = 1;
        let v = rec_item(DAT_00aa6134);
        set_item(v);
        calls3();
    }
}

unsafe fn prev_tail(a: bool) {
    // SAFETY: game globals.
    unsafe {
        if a {
            if DAT_00aa6114 != 0 {
                DAT_0052c208 = 0;
                DAT_00aa6134 = 0;
                if DAT_00aa6138 != 0 {
                    scan();
                }
            }
        } else {
            DAT_0052c208 = 0;
            if DAT_00aa6114 != 0 {
                DAT_00aa6134 = 0;
                if DAT_00aa6138 != 0 {
                    scan();
                }
            }
        }
    }
}

unsafe fn post8a() {
    // SAFETY: game tables indexed as the original does.
    unsafe {
        let o = rec_item(DAT_00aa6134).wrapping_mul(0x28);
        if rd(at(pb!(DAT_00bb2046), o)) != 0 {
            DAT_00bb7397 = rd8(at(pb!(DAT_00bb204a), o)) as i8;
        }
    }
}

unsafe fn post8() {
    // SAFETY: game tables indexed as the original does.
    unsafe {
        if DAT_00b69e30 == 2 {
            post8a();
        } else {
            let c = FUN_004cfee0();
            let o = rec_item(DAT_00aa6134).wrapping_mul(0x28);
            if c == 0 {
                if rd(at(pb!(DAT_00bb260e), o)) != 0 {
                    DAT_00bb7397 = rd8(at(pb!(DAT_00bb2612), o)) as i8;
                }
            } else if rd(at(pb!(DAT_00bb242e), o)) != 0 {
                DAT_00bb7397 = rd8(at(pb!(DAT_00bb2432), o)) as i8;
            }
        }
    }
}

unsafe fn post5x(use_a: bool) {
    // SAFETY: game tables indexed as the original does.
    unsafe {
        let o = rec_item(DAT_00aa6134).wrapping_mul(0xa0);
        if use_a {
            if rd(at(pb!(DAT_00bb0106), o)) != 0 {
                DAT_00bb7398 = rd8(at(pb!(DAT_00bb010a), o)) as i8;
                DAT_00bb7397 = rd8(at(pb!(DAT_00bb010e), o)) as i8;
            }
        } else if rd(at(pb!(DAT_00bb10a6), o)) != 0 {
            DAT_00bb7398 = rd8(at(pb!(DAT_00bb10aa), o)) as i8;
            DAT_00bb7397 = rd8(at(pb!(DAT_00bb10ae), o)) as i8;
        }
    }
}

unsafe fn post5() {
    // SAFETY: reads a game global.
    unsafe { post5x(DAT_00b69e30 == 2) }
}

unsafe fn click_tail() {
    // SAFETY: game global and callees.
    unsafe {
        if DAT_00b69e30 != 2 {
            if FUN_004cfee0() != 0 {
                FUN_004e6310(0);
            }
        }
    }
}

unsafe fn draw(l40: *mut i32, name: *mut i8, msg: i32) {
    // SAFETY: the callees are the game's own.
    unsafe {
        let s = FUN_004ddc10(msg);
        FUN_004daa20(l40, name, s, 0xc);
    }
}

unsafe fn btn(l40: *mut i32, name: *mut i8, msg: i32, sid: i32) -> i32 {
    // SAFETY: the callees are the game's own.
    unsafe {
        let s = FUN_004ddc10(msg);
        FUN_004da780(l40, name, name, s, 0xc, 0, sid, pc!(DAT_0052ad74))
    }
}

unsafe fn down_click(
    kind: u32,
    l40: *mut i32,
    l28: i32,
    l2c: i32,
    en: i32,
    name: *mut i8,
    msg: i32,
) {
    // SAFETY: game callees and globals.
    unsafe {
        draw(l40, name, msg);
        if en != 0 && FUN_004d6b90(l40, l28, l2c) != 0 {
            if FUN_004dc240() == 0 {
                if FUN_004dc220() != 0 {
                    if DAT_00aa6114 != 0 {
                        next_core();
                        if kind == 8 {
                            post8();
                        } else if kind == 5 {
                            post5();
                        }
                    }
                    click_tail();
                }
            } else {
                if DAT_00aa6114 != 0 {
                    prev_core();
                    prev_tail(kind >= 5);
                    if kind == 8 {
                        post8();
                    } else if kind == 5 {
                        post5();
                    }
                }
                click_tail();
            }
        }
    }
}

unsafe fn find_a() -> i32 {
    // SAFETY: walks the game's table like the original.
    unsafe {
        let mut i: i32 = 0;
        if 0 < (DAT_00aa5aac as i32) {
            let mut p = (&raw mut DAT_00a91d24).cast::<u8>();
            loop {
                if rd8(p) != 0 {
                    break;
                }
                p = p.wrapping_add(0x562);
                i = i.wrapping_add(1);
                if !(i < (DAT_00aa5aac as i32)) {
                    break;
                }
            }
        }
        i
    }
}

unsafe fn sec1_move(next: bool) -> bool {
    // SAFETY: game globals and callees.
    unsafe {
        let mut changed = false;
        if DAT_00aa6114 != 0 {
            snd();
            if next {
                loop {
                    if DAT_00aa6138.wrapping_sub(DAT_00aa6134) == 1 {
                        DAT_00aa6134 = 0;
                    } else {
                        DAT_00aa6134 = DAT_00aa6134.wrapping_add(1);
                    }
                    if rec_blk(DAT_00aa6134) == 0 {
                        break;
                    }
                }
            } else {
                loop {
                    if DAT_00aa6134 == 0 {
                        DAT_00aa6134 = DAT_00aa6138;
                    }
                    DAT_00aa6134 = DAT_00aa6134.wrapping_sub(1);
                    if rec_blk(DAT_00aa6134) == 0 {
                        break;
                    }
                }
            }
            let v = rec_item(DAT_00aa6134);
            set_item(v);
            let i = find_a();
            DAT_00bb73a6 = 1;
            let l30 = rd8(at(pb!(DAT_00a91d17), (i as u32).wrapping_mul(0x562)));
            FUN_004f70f0();
            let j = find_a();
            if rd8(at(pb!(DAT_00a91d17), (j as u32).wrapping_mul(0x562))) != l30 {
                changed = true;
            }
            FUN_004f36d0();
            FUN_004e5590(DAT_00aa6118);
            DAT_0052c208 = 0;
            if DAT_00aa6114 != 0 {
                DAT_00aa6134 = 0;
                if DAT_00aa6138 != 0 {
                    scan();
                }
            }
        }
        changed
    }
}

unsafe fn copy6(dst: *mut u8, src: *mut u8) {
    // SAFETY: copies six dwords of game memory as the original does.
    unsafe {
        let mut i: u32 = 0;
        while i < 6 {
            wr(at(dst, i * 4), rd(at(src, i * 4)));
            i += 1;
        }
    }
}

unsafe fn store_cur() -> (u32, i32) {
    // SAFETY: game tables addressed as the original does.
    unsafe {
        let slot = rd(pb!(DAT_00b6d962));
        let off = slot.wrapping_mul(0x109E0);
        wr(at(pb!(DAT_00b6d96a), off), rd(pb!(DAT_00b6a5a6)));
        wr(at(pb!(DAT_00b6d96e), off), rd(pb!(DAT_00b6a5aa)));
        copy6(at(pb!(DAT_00b6d972), off), pb!(DAT_00b6a5ae));
        wr(at(pb!(DAT_00b6d9aa), off), rd(pb!(DAT_00b6a606)));
        wr(at(pb!(DAT_00b6d9ae), off), rd(pb!(DAT_00b6a60a)));
        let l30 = rd(pb!(DAT_00b6a5aa));
        let t = rd(pb!(DAT_00b6a5aa));
        let l24 = rd(at(pb!(DAT_00b6a5ae), t.wrapping_mul(4))) as i32;
        (l30, l24)
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004e2970() -> i32 {
    let mut i_var6: i32 = 0;
    let mut local_40 = [0i32; 4];
    let mut local_30: u32 = 0;
    let mut local_2c: i32 = 0;
    let mut local_28: i32 = 0;
    let mut local_24: i32 = 0;
    let mut local_20 = [0i32; 8];
    let l40 = local_40.as_mut_ptr();
    let l20 = local_20.as_mut_ptr();

    // SAFETY: every callee and global is the game's own; pointers are to
    // locals or game memory, accessed where the original accesses them.
    unsafe {
        FUN_004de090(&raw mut local_28, &raw mut local_2c);
        let mut k = 0usize;
        while k < 5 {
            l20.add(k).write(1);
            k += 1;
        }
        if DAT_00b69e30 == 0 {
            l20.add(5).write(0);
            l20.add(6).write(0);
        } else {
            l20.add(5).write(1);
            l20.add(6).write(1);
            let mut s = rd(pb!(DAT_00b6d962)) as i32;
            if DAT_00b69e30 != 2 {
                s = DAT_00665d10;
            }
            l20.offset(s.wrapping_add(5) as isize).write(2);
        }
        l20.add(7).write(1);
        let t0 = rd(pb!(DAT_00b6a5aa));
        if t0 == 5 {
            l20.add(7).write(2);
        } else {
            l20.add(t0 as usize).write(2);
        }
        FUN_004daba0(l20);
        let mut i_var2 = FUN_004cfee0();
        if DAT_00b69e30 == 2 {
            i_var2 = 1;
        }
        FUN_004d6460(l40, 0x16, 0x14e, 0x2a, 0x166);
        if DAT_00b69e30 == 2 {
            if rd(pb!(DAT_00b6d962)) == 1 {
                draw(l40, pc!(DAT_0052b5a0), 0x283);
            } else if btn(l40, pc!(DAT_0052b5a8), 0x283, 0x11) != 0 {
                i_var6 = 1;
                let (a, b) = store_cur();
                local_30 = a;
                local_24 = b;
                wr(pb!(DAT_00b6d962), 1);
                wr(pb!(DAT_00b6a5a6), rd(pb!(DAT_00b7e34a)));
                wr(pb!(DAT_00b6a5aa), rd(pb!(DAT_00b7e34e)));
                copy6(pb!(DAT_00b6a5ae), pb!(DAT_00b7e352));
                wr(pb!(DAT_00b6a606), rd(at(pb!(DAT_00b6d9aa), 0x109E0)));
                wr(pb!(DAT_00b6a60a), rd(at(pb!(DAT_00b6d9ae), 0x109E0)));
            }
            FUN_004d64a0(l40, 0, -0x1c);
            if rd(pb!(DAT_00b6d962)) == 0 {
                draw(l40, pc!(DAT_0052b590), 0x282);
            } else if btn(l40, pc!(DAT_0052b598), 0x282, 0x10) != 0 {
                i_var6 = 1;
                let (a, b) = store_cur();
                local_30 = a;
                local_24 = b;
                wr(pb!(DAT_00b6d962), 0);
                wr(pb!(DAT_00b6a5a6), rd(pb!(DAT_00b6d96a)));
                wr(pb!(DAT_00b6a5aa), rd(pb!(DAT_00b6d96e)));
                copy6(pb!(DAT_00b6a5ae), pb!(DAT_00b6d972));
                wr(pb!(DAT_00b6a606), rd(pb!(DAT_00b6d9aa)));
                wr(pb!(DAT_00b6a60a), rd(pb!(DAT_00b6d9ae)));
            }
            if i_var6 != 0 {
                if rd(pb!(DAT_00b6a5aa)) != 5 && DAT_00bb7394 == 3 {
                    DAT_00bb7394 = 0;
                }
                if rd(pb!(DAT_00b6a5aa)) != 4 && DAT_00bb7397 == 2 {
                    DAT_00bb7397 = 0;
                }
                wr(pb!(DAT_00b6a5aa), local_30);
                FUN_004e5000(local_30);
                if DAT_00aa6114 != 0 {
                    DAT_00aa6134 = 0;
                    if DAT_00aa6138 != 0 {
                        let mut p = (DAT_00aa6114 as u32).wrapping_add(0x140);
                        loop {
                            if (rd(p as usize as *const u8) as i32) == local_24 {
                                set_item(local_24 as u32);
                                break;
                            }
                            p = p.wrapping_add(0x148);
                            DAT_00aa6134 = DAT_00aa6134.wrapping_add(1);
                            if DAT_00aa6134 >= DAT_00aa6138 {
                                break;
                            }
                        }
                    }
                    if DAT_00aa6138 <= DAT_00aa6134 {
                        DAT_00aa6134 = 0;
                        if DAT_00aa6138 != 0 {
                            scan();
                        }
                        if DAT_00aa6138 <= DAT_00aa6134 {
                            let v = rec_item(0);
                            set_item(v);
                        }
                    }
                }
                calls3();
                DAT_0052c208 = 0;
                let t = rd(pb!(DAT_00b6a5aa));
                if t == 4 {
                    post5x(true);
                } else if t == 5 {
                    post8a();
                }
            }
        } else {
            if l20.add(6).read() != 0 {
                if DAT_00665d10 == 1 {
                    draw(l40, pc!(DAT_0052c2c0), 0xcb);
                } else if btn(l40, pc!(DAT_0052c2c8), 0xcb, 0x11) != 0 {
                    i_var6 = 1;
                    DAT_00665d10 = 1;
                }
            }
            FUN_004d64a0(l40, 0, -0x1c);
            if l20.add(5).read() != 0 {
                if DAT_00665d10 == 0 {
                    draw(l40, pc!(DAT_0052c2b0), 0xca);
                } else if btn(l40, pc!(DAT_0052c2b8), 0xca, 0x10) != 0 {
                    DAT_00665d10 = 0;
                    i_var6 = 1;
                }
            }
        }

        FUN_004d6460(l40, 0x16, 0xfe, 0x2a, 0x116);
        if rd(pb!(DAT_00b6a5aa)) == 5 {
            down_click(8, l40, local_28, local_2c, i_var2, pc!(DAT_0052c2a0), 199);
        } else if i_var2 == 0 {
            draw(l40, pc!(DAT_0052c2a8), 199);
        } else if btn(l40, pc!(DAT_0052c2a8), 199, 0x12) != 0 {
            let t = rd(pb!(DAT_00b6a5aa));
            if DAT_00b69e30 != 2 && t != 0 && t != 5 {
                DAT_00bb739e = -1;
            }
            wr(pb!(DAT_00b6a5aa), 5);
            DAT_00bb7397 = 0;
            i_var6 = 1;
            DAT_00bb73a6 = 1;
            calls3();
            DAT_0052c208 = 0;
            if DAT_00aa6114 != 0 {
                DAT_00aa6134 = 0;
                if DAT_00aa6138 != 0 {
                    scan();
                }
                if DAT_00aa6134 == DAT_00aa6138 || rec_blk(DAT_00aa6134) != 0 {
                    fix();
                }
            }
            DAT_0052c208 = 0;
            if rd(pb!(DAT_00b6a5aa)) != 4 && DAT_00bb7397 == 2 {
                DAT_00bb7397 = 0;
            }
            post8();
        }

        FUN_004d6460(l40, 0x16, 0xe2, 0x2a, 0xfa);
        if rd(pb!(DAT_00b6a5aa)) == 4 {
            down_click(5, l40, local_28, local_2c, i_var2, pc!(DAT_0052c290), 0xc6);
        } else if i_var2 == 0 {
            draw(l40, pc!(DAT_0052c298), 0xc6);
        } else if btn(l40, pc!(DAT_0052c298), 0xc6, 0xf) != 0 {
            if DAT_00b69e30 != 2 {
                let t = rd(pb!(DAT_00b6a5aa));
                if t != 3 && t != 4 {
                    DAT_00bb739e = -1;
                }
                if DAT_00bb739d == 2 {
                    DAT_00bb739d = 1;
                }
            }
            wr(pb!(DAT_00b6a5aa), 4);
            i_var6 = 1;
            DAT_00bb73a6 = 1;
            if DAT_00bb7394 == 3 {
                DAT_00bb7394 = 0;
            }
            calls3();
            DAT_0052c208 = 0;
            if DAT_00aa6114 != 0 {
                DAT_00aa6134 = 0;
                if DAT_00aa6138 != 0 {
                    if up_scan() {
                        fix();
                    }
                }
            }
            post5();
        }

        FUN_004d64a0(l40, 0, -0x1c);
        if rd(pb!(DAT_00b6a5aa)) == 3 {
            down_click(4, l40, local_28, local_2c, i_var2, pc!(DAT_0052c280), 0xbf);
        } else if i_var2 == 0 {
            draw(l40, pc!(DAT_0052c288), 0xbf);
        } else if btn(l40, pc!(DAT_0052c288), 0xbf, 0xe) != 0 {
            if DAT_00b69e30 != 2 {
                let t = rd(pb!(DAT_00b6a5aa));
                if t != 3 && t != 4 {
                    DAT_00bb739e = -1;
                }
                if DAT_00bb739d == 2 {
                    DAT_00bb739d = 1;
                }
            }
            wr(pb!(DAT_00b6a5aa), 3);
            i_var6 = 1;
            DAT_00bb73a6 = 1;
            if DAT_00bb7394 == 3 {
                DAT_00bb7394 = 0;
            }
            if DAT_00bb7397 == 2 {
                DAT_00bb7397 = 0;
            }
            calls3();
            DAT_0052c208 = 0;
            DAT_00aa6134 = 0;
            if DAT_00aa6114 != 0 && DAT_00aa6138 != 0 {
                if up_scan() {
                    fix();
                }
            }
        }

        FUN_004d64a0(l40, 0, -0x1c);
        if l20.add(2).read() != 0 {
            if rd(pb!(DAT_00b6a5aa)) == 2 {
                down_click(3, l40, local_28, local_2c, i_var2, pc!(DAT_0052c270), 0xc4);
            } else if i_var2 == 0 {
                draw(l40, pc!(DAT_0052c278), 0xc4);
            } else if btn(l40, pc!(DAT_0052c278), 0xc4, 0xd) != 0 {
                let t = rd(pb!(DAT_00b6a5aa));
                if DAT_00b69e30 != 2 && t != 1 && t != 2 {
                    DAT_00bb739e = -1;
                }
                wr(pb!(DAT_00b6a5aa), 2);
                i_var6 = 1;
                DAT_00bb73a6 = 1;
                if DAT_00bb7394 == 3 {
                    DAT_00bb7394 = 0;
                }
                if DAT_00bb7397 == 2 {
                    DAT_00bb7397 = 0;
                }
                calls3();
                DAT_0052c208 = 0;
                if DAT_00aa6114 != 0 {
                    DAT_00aa6134 = 0;
                    if DAT_00aa6138 != 0 {
                        if up_scan() {
                            fix();
                        }
                    }
                }
            }
        }

        FUN_004d64a0(l40, 0, -0x1c);
        if rd(pb!(DAT_00b6a5aa)) == 1 {
            down_click(2, l40, local_28, local_2c, i_var2, pc!(DAT_0052c260), 0xbe);
        } else if i_var2 == 0 {
            draw(l40, pc!(DAT_0052c268), 0xbe);
        } else if btn(l40, pc!(DAT_0052c268), 0xbe, 0xc) != 0 {
            let t = rd(pb!(DAT_00b6a5aa));
            if DAT_00b69e30 != 2 && t != 1 && t != 2 {
                DAT_00bb739e = -1;
            }
            i_var6 = 1;
            DAT_00bb73a6 = 1;
            wr(pb!(DAT_00b6a5aa), 1);
            if DAT_00bb7394 == 3 {
                DAT_00bb7394 = 0;
            }
            if DAT_00bb7397 == 2 {
                DAT_00bb7397 = 0;
            }
            calls3();
            DAT_0052c208 = 0;
            if DAT_00aa6114 != 0 {
                DAT_00aa6134 = 0;
                if DAT_00aa6138 != 0 {
                    if up_scan() {
                        fix();
                    }
                }
            }
        }

        FUN_004d64a0(l40, 0, -0x1c);
        if rd(pb!(DAT_00b6a5aa)) == 0 {
            draw(l40, pc!(DAT_0052c250), 0xbd);
            if i_var2 != 0 && FUN_004d6b90(l40, local_28, local_2c) != 0 {
                if FUN_004dc240() == 0 {
                    if FUN_004dc220() != 0 {
                        if sec1_move(true) {
                            i_var6 = 1;
                        }
                        click_tail();
                    }
                } else {
                    if sec1_move(false) {
                        i_var6 = 1;
                    }
                    click_tail();
                }
            }
        } else if i_var2 == 0 {
            draw(l40, pc!(DAT_0052c258), 0xbd);
        } else if btn(l40, pc!(DAT_0052c258), 0xbd, 0xb) != 0 {
            if DAT_00b69e30 != 2 {
                DAT_00bb739e = -1;
            }
            i_var6 = 1;
            DAT_00bb73a6 = 1;
            wr(pb!(DAT_00b6a5aa), 0);
            if DAT_00bb7394 == 3 {
                DAT_00bb7394 = 0;
            }
            if DAT_00bb7397 == 2 {
                DAT_00bb7397 = 0;
            }
            calls3();
            DAT_0052c208 = 0;
            if DAT_00aa6114 != 0 {
                DAT_00aa6134 = 0;
                if DAT_00aa6138 != 0 {
                    if up_scan() {
                        fix();
                    }
                }
            }
        }

        if i_var6 != 0 {
            FUN_004b4bf0(pc!(DAT_0052bf74));
            FUN_004e9330();
        }
    }
    i_var6
}
