// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00658020: *mut i32;
    static mut DAT_006572c0: i32;
    static mut DAT_006572b8: i32;
    static mut DAT_006572d4: i32;
    static mut DAT_00525518: u32;
    static mut DAT_00525520: u32;
    static mut DAT_00525528: u32;
    static mut DAT_00527e74: u16;
    static mut DAT_006da9f4: u32;
    static mut DAT_006d4984: u16;
    static mut DAT_007091ae: u16;
    static mut _DAT_006da4fc: u32;
    static mut _DAT_006db7bc: u32;
}

unsafe fn cp(dst: u32, src: u32, n: u32) {
    let mut d = dst;
    let mut s = src;
    let mut c = n >> 2;
    // SAFETY: addresses are the game's own buffers, copied forward like rep movsd/movsb.
    unsafe {
        while c != 0 {
            ((d as usize) as *mut u32)
                .write_unaligned(((s as usize) as *const u32).read_unaligned());
            s = s.wrapping_add(4);
            d = d.wrapping_add(4);
            c -= 1;
        }
        let mut b = n & 3;
        while b != 0 {
            ((d as usize) as *mut u8).write(((s as usize) as *const u8).read());
            s = s.wrapping_add(1);
            d = d.wrapping_add(1);
            b -= 1;
        }
    }
}

unsafe fn half(dst: u32, src: u32, n: u32, stride: u32) {
    // SAFETY: same game buffers as above.
    unsafe {
        if n & 1 == 0 {
            cp(dst, src, n.wrapping_mul(stride));
        } else {
            let m = n.wrapping_sub(1);
            if m != 0 {
                cp(dst, src, m.wrapping_mul(stride));
            }
            ((dst as usize) as *mut u16)
                .write_unaligned(((src as usize) as *const u16).read_unaligned());
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0045f800() {
    let mut buf: [u32; 27] = [0; 27];
    buf[0] = 0x6c;
    const MAGIC: i32 = -0x7789fde4;
    // SAFETY: globals and the COM-like object at DAT_00658020 belong to the game;
    // all raw addresses are the game's surface memory.
    unsafe {
        loop {
            let vt = (DAT_00658020.read() as u32 as usize) as *const i32;
            let f = vt.byte_add(100).read();
            let f = core::mem::transmute::<
                usize,
                unsafe extern "stdcall" fn(i32, i32, *mut u32, i32, i32) -> i32,
            >(f as u32 as usize);
            let r = f(DAT_00658020 as usize as i32, 0, buf.as_mut_ptr(), 0, 0);
            let mut i_var5c = buf[4] as i32;
            let mut p_stack48 = buf[9];
            let mut p_var8 = p_stack48;
            let i_var4 = i_var5c;
            if r == 0 {
                // second buffer reset
                buf = [0; 27];
                buf[0] = 0x6c;
                loop {
                    let vt = (DAT_00658020.read() as u32 as usize) as *const i32;
                    let f = vt.byte_add(100).read();
                    let f = core::mem::transmute::<
                        usize,
                        unsafe extern "stdcall" fn(i32, i32, *mut u32, i32, i32) -> i32,
                    >(f as u32 as usize);
                    let r2 = f(DAT_006572c0, 0, buf.as_mut_ptr(), 0, 0);
                    i_var5c = buf[4] as i32;
                    p_stack48 = buf[9];
                    if r2 == 0 {
                        let mut src = p_stack48;
                        let mut u7: u32 = 0;
                        p_var8 = p_var8
                            .wrapping_add(
                                ((DAT_006572d4 as u32).wrapping_sub(DAT_00525528) >> 1)
                                    .wrapping_mul(i_var4 as u32),
                            )
                            .wrapping_add(
                                ((DAT_006572b8 as u32).wrapping_sub(DAT_00525520) >> 1)
                                    .wrapping_mul(DAT_00525518),
                            );
                        if _DAT_006da4fc != 0 {
                            loop {
                                half(p_var8, src, DAT_00525520, DAT_00525518);
                                src = src.wrapping_add(i_var5c as u32);
                                p_var8 = p_var8.wrapping_add(i_var4 as u32);
                                u7 = u7.wrapping_add(1);
                                if !(u7 < _DAT_006da4fc) {
                                    break;
                                }
                            }
                        }
                        let mut row: i32 = 0;
                        let mut pb9: u32 = (DAT_00527e74 as u32).wrapping_add(DAT_006da9f4);
                        if DAT_006d4984 != 0 {
                            loop {
                                if _DAT_006db7bc != 0 {
                                    half(p_var8, src, _DAT_006db7bc, DAT_00525518);
                                    let adv = _DAT_006db7bc.wrapping_mul(DAT_00525518);
                                    src = src.wrapping_add(adv);
                                    p_var8 = p_var8.wrapping_add(adv);
                                }
                                let mut b6: u8 = ((pb9 as usize) as *const u8).read();
                                pb9 = pb9.wrapping_add(1);
                                let mut u84: u32 = 0;
                                let mut pb10 = pb9;
                                if DAT_007091ae != 0 {
                                    loop {
                                        let mut v = ((pb10 as usize) as *const u8).read() as u32;
                                        pb9 = pb10.wrapping_add(1);
                                        if v == 0 {
                                            v = ((pb9 as usize) as *const u8).read() as u32;
                                            pb9 = pb10.wrapping_add(2);
                                            if v == 0 {
                                                let b = ((pb9 as usize) as *const u8).read();
                                                pb9 = pb10.wrapping_add(3);
                                                v = (b as u32).wrapping_add(0x100);
                                            }
                                            v = v.wrapping_add(0xff);
                                        }
                                        if (b6 as i8) < 0 {
                                            half(p_var8, src, v, DAT_00525518);
                                        }
                                        b6 = (b6 as i32).wrapping_neg() as u8;
                                        let adv = v.wrapping_mul(DAT_00525518);
                                        src = src.wrapping_add(adv);
                                        p_var8 = p_var8.wrapping_add(adv);
                                        u84 = u84.wrapping_add(v);
                                        pb10 = pb9;
                                        if !((u84 as i32) < (DAT_007091ae as i32)) {
                                            break;
                                        }
                                    }
                                }
                                u84 = u84.wrapping_add(_DAT_006db7bc);
                                if u84 < DAT_00525520 {
                                    u84 = DAT_00525520.wrapping_sub(u84);
                                    half(p_var8, src, u84, DAT_00525518);
                                    let adv = u84.wrapping_mul(DAT_00525518);
                                    p_var8 = p_var8.wrapping_add(adv);
                                    src = src.wrapping_add(adv);
                                    u84 = DAT_00525520;
                                }
                                let m = u84.wrapping_mul(DAT_00525518);
                                p_var8 = p_var8.wrapping_add((i_var4 as u32).wrapping_sub(m));
                                row = row.wrapping_add(1);
                                src = src.wrapping_add((i_var5c as u32).wrapping_sub(m));
                                if !(row < (DAT_006d4984 as i32)) {
                                    break;
                                }
                            }
                        }
                        let mut u7b: u32 = (DAT_006d4984 as u32).wrapping_add(_DAT_006da4fc);
                        if u7b < DAT_00525528 {
                            loop {
                                half(p_var8, src, DAT_00525520, DAT_00525518);
                                src = src.wrapping_add(i_var5c as u32);
                                p_var8 = p_var8.wrapping_add(i_var4 as u32);
                                u7b = u7b.wrapping_add(1);
                                if !(u7b < DAT_00525528) {
                                    break;
                                }
                            }
                        }
                        let vt = (DAT_00658020.read() as u32 as usize) as *const i32;
                        let g = vt.byte_add(0x80).read();
                        let g = core::mem::transmute::<
                            usize,
                            unsafe extern "stdcall" fn(i32, i32) -> i32,
                        >(g as u32 as usize);
                        g(DAT_006572c0, p_stack48 as i32);
                        let vt = (DAT_00658020.read() as u32 as usize) as *const i32;
                        let g = vt.byte_add(0x80).read();
                        let g = core::mem::transmute::<
                            usize,
                            unsafe extern "stdcall" fn(i32, i32) -> i32,
                        >(g as u32 as usize);
                        g(DAT_00658020 as usize as i32, i_var5c);
                        return;
                    }
                    if r2 != MAGIC {
                        return;
                    }
                }
            }
            if r != MAGIC {
                return;
            }
        }
    }
}
