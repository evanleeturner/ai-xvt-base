// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00acebf9: u8;
    static mut DAT_00acebfd: u8;
    static mut DAT_00acebf1: u8;
    static mut DAT_00acec01: u8;
    static mut DAT_00acf145: u8;
    static mut DAT_00b69a89: u8;
    static mut DAT_00acec35: u8;
    static mut DAT_00acec45: u8;
    static mut DAT_00acec49: u8;
    static mut DAT_00b63f91: u8;
    static mut DAT_00b63f95: u8;
    static mut DAT_00b63f99: u8;
    static mut DAT_00b63f9d: u8;
    static mut DAT_00b63fa1: u8;
    static mut DAT_00b63fa5: u8;
    static mut DAT_00b63fa9: u8;
    static mut DAT_00b63fad: u8;
    static mut DAT_00b63fb1: u8;
    static mut DAT_00b641b1: u8;
    static mut DAT_00b641b5: u8;
    static mut DAT_00b641b9: u8;
    static mut DAT_00b641bd: u8;
    static mut DAT_00b641c1: u8;
    static mut DAT_00b641c5: u8;
    static mut DAT_00665050: i32;
    static mut DAT_00665410: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cdc20();
    fn FUN_004cef70(param_1: i32, param_2: *mut core::ffi::c_void, param_3: u32);
    fn FUN_004d13b0(param_1: i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetTickCount() -> u32;
}

unsafe fn rd(p: *const u8) -> u32 {
    // SAFETY: the caller passes an address inside the game's memory.
    unsafe { p.cast::<u32>().read_unaligned() }
}

unsafe fn wr(p: *mut u8, v: u32) {
    // SAFETY: the caller passes an address inside the game's memory.
    unsafe { p.cast::<u32>().write_unaligned(v) }
}

unsafe fn ri(a: i32) -> i32 {
    // SAFETY: the address is the caller's message block.
    unsafe { (a as u32 as usize as *const i32).read_unaligned() }
}

fn off(p: *mut u8, n: u32) -> *mut u8 {
    p.wrapping_add(n as usize)
}

unsafe fn slen1(p: *const u8) -> u32 {
    let mut n: u32 = 0;
    let mut q = p;
    loop {
        // SAFETY: reads up to and including the terminating nul.
        let c = unsafe { q.read() };
        q = q.wrapping_add(1);
        n = n.wrapping_add(1);
        if c == 0 {
            break;
        }
    }
    n
}

unsafe fn cpy(mut dst: *mut u8, mut src: *const u8, n: u32) {
    let mut k = n >> 2;
    while k != 0 {
        // SAFETY: copies n bytes between game buffers.
        unsafe {
            dst.cast::<u32>()
                .write_unaligned(src.cast::<u32>().read_unaligned())
        };
        src = src.wrapping_add(4);
        dst = dst.wrapping_add(4);
        k -= 1;
    }
    let mut k = n & 3;
    while k != 0 {
        // SAFETY: as above.
        unsafe { dst.write(src.read()) };
        src = src.wrapping_add(1);
        dst = dst.wrapping_add(1);
        k -= 1;
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004d00d0(param_1: i32, param_2: i32) {
    let mut local_200: [u32; 128] = [0; 128];
    // SAFETY: all globals and callees are the game's own memory; param_2
    // is the caller's message block, accessed where the original does.
    unsafe {
        if param_1 == 3 {
            if rd(&raw mut DAT_00acebf9) == 0 {
                wr(&raw mut DAT_00acebfd, 1);
                FUN_004cdc20();
                return;
            }
            let c = rd(&raw mut DAT_00acebf1);
            if ri(param_2.wrapping_add(4)) == 1 && ri(param_2.wrapping_add(8)) != c as i32 {
                wr(&raw mut DAT_00acebfd, 1);
                FUN_004cdc20();
                local_200[1] = rd(&raw mut DAT_00acebfd);
                local_200[2] = rd(&raw mut DAT_00b69a89);
                local_200[0] = 0x3b;
                local_200[3] = GetTickCount();
                let mut u8_: u32 = 0;
                let mut n = rd(&raw mut DAT_00b69a89);
                if n != 0 {
                    let mut p2 = &raw mut DAT_00b63f91;
                    let mut p3 = local_200.as_mut_ptr().wrapping_add(4).cast::<u8>();
                    loop {
                        u8_ = u8_.wrapping_add(1);
                        cpy(p3, p2, 4);
                        p3.wrapping_add(4).write(p2.wrapping_add(0xc).read());
                        p3.wrapping_add(5).write(p2.wrapping_add(0x14).read());
                        p3.wrapping_add(6).write(p2.wrapping_add(0x10).read());
                        p3.wrapping_add(7).write(p2.wrapping_add(0x18).read());
                        p2 = p2.wrapping_add(0x238);
                        p3 = p3.wrapping_add(8);
                        n = rd(&raw mut DAT_00b69a89);
                        if u8_ >= n {
                            break;
                        }
                    }
                }
                let _ = rd(&raw mut DAT_00b69a89);
                let c = rd(&raw mut DAT_00b69a89);
                FUN_004cef70(
                    ri(param_2.wrapping_add(8)),
                    local_200.as_mut_ptr().cast(),
                    c.wrapping_mul(8).wrapping_add(0x10),
                );
                return;
            }
            return;
        }
        if param_1 == 5 {
            if rd(&raw mut DAT_00acebf9) == 0 {
                let c = rd(&raw mut DAT_00acebf1);
                if ri(param_2.wrapping_add(4)) == 1 && ri(param_2.wrapping_add(8)) == c as i32 {
                    local_200[0] = 0x46;
                    let c = rd(&raw mut DAT_00acf145);
                    FUN_004cef70(c as i32, local_200.as_mut_ptr().cast(), 4);
                }
            } else if ri(param_2.wrapping_add(4)) == 1 {
                let mut u8_: u32 = 0;
                let t = rd(&raw mut DAT_00acebfd);
                if t != 0 {
                    let mut tbl = &raw mut DAT_00acec45;
                    loop {
                        let c = rd(tbl);
                        if c as i32 == ri(param_2.wrapping_add(8)) {
                            let n = rd(off(&raw mut DAT_00acec49, u8_.wrapping_mul(40)));
                            if n != 0 {
                                wr(&raw mut DAT_00acec01, 1);
                            }
                            break;
                        }
                        tbl = tbl.wrapping_add(40);
                        u8_ = u8_.wrapping_add(1);
                        if u8_ >= rd(&raw mut DAT_00acebfd) {
                            break;
                        }
                    }
                }
                FUN_004d13b0(ri(param_2.wrapping_add(8)));
                u8_ = 0;
                let n = rd(&raw mut DAT_00b69a89);
                if n != 0 {
                    let mut tbl = &raw mut DAT_00b63f91;
                    loop {
                        let c = rd(tbl);
                        if c as i32 == ri(param_2.wrapping_add(8)) {
                            let n = rd(&raw mut DAT_00b69a89).wrapping_sub(1);
                            wr(&raw mut DAT_00b69a89, n);
                            let mut p2 = off(&raw mut DAT_00b63f91, n.wrapping_mul(0x238));
                            let mut p3 = off(&raw mut DAT_00b63f91, u8_.wrapping_mul(0x238));
                            let mut i6 = 0x8e;
                            while i6 != 0 {
                                let t = rd(p2);
                                wr(p3, t);
                                p2 = p2.wrapping_add(4);
                                p3 = p3.wrapping_add(4);
                                i6 -= 1;
                            }
                            let o = n.wrapping_mul(0x238);
                            wr(off(&raw mut DAT_00b63f91, o), 0);
                            wr(off(&raw mut DAT_00b63f95, o), 0x7f);
                            wr(off(&raw mut DAT_00b63f9d, o), 0x7f);
                            wr(off(&raw mut DAT_00b63fa5, o), 0x7f);
                            wr(off(&raw mut DAT_00b63f99, o), 0x7f);
                            wr(off(&raw mut DAT_00b63fa1, o), 0x7f);
                            wr(off(&raw mut DAT_00b63fa9, o), 0x7f);
                            wr(off(&raw mut DAT_00b63fad, o), 0);
                            off(&raw mut DAT_00b63fb1, o).write(0x39);
                            wr(off(&raw mut DAT_00b641b1, o), 1);
                            wr(off(&raw mut DAT_00b641c5, o), 0);
                            wr(off(&raw mut DAT_00b641b5, o), 0);
                            wr(off(&raw mut DAT_00b641b9, o), 0);
                            wr(off(&raw mut DAT_00b641bd, o), 0);
                            wr(off(&raw mut DAT_00b641c1, o), 0);
                            break;
                        }
                        tbl = tbl.wrapping_add(0x238);
                        u8_ = u8_.wrapping_add(1);
                        if rd(&raw mut DAT_00b69a89) <= u8_ {
                            break;
                        }
                    }
                }
                let mut i6: i32 = 0;
                let base = &raw mut DAT_00665050;
                let mut p9 = base;
                loop {
                    if p9.read() == ri(param_2.wrapping_add(8)) {
                        let b = base.wrapping_add((i6 as usize).wrapping_mul(6));
                        b.write(0);
                        b.wrapping_add(1).write(0);
                        b.wrapping_add(2).write(0);
                        b.wrapping_add(3).write(0);
                        b.wrapping_add(4).write(0);
                        b.wrapping_add(5).write(0);
                        break;
                    }
                    p9 = p9.wrapping_add(6);
                    i6 = i6.wrapping_add(1);
                    if p9 as usize >= (&raw mut DAT_00665410) as usize {
                        break;
                    }
                }
            }
            wr(&raw mut DAT_00acebfd, 1);
            FUN_004cdc20();
            return;
        }
        if param_1 != 0x103 {
            return;
        }
        let t = rd(&raw mut DAT_00acebfd);
        if ri(param_2.wrapping_add(4)) == 1 && t != 0 {
            let mut u8_: u32 = 0;
            let mut p7 = &raw mut DAT_00acec35;
            loop {
                let c = rd(p7.wrapping_add(0x10));
                if c as i32 == ri(param_2.wrapping_add(8)) {
                    let src = (param_2.wrapping_add(0x1c) as u32 as usize) as *const u8;
                    let l = slen1(src);
                    cpy(p7, src, l);
                    let l1 = slen1(p7);
                    let src2 = (param_2.wrapping_add(0x1c).wrapping_add(l1 as i32) as u32 as usize)
                        as *const u8;
                    let l2 = slen1(src2);
                    cpy(p7.wrapping_sub(0x10), src2, l2);
                    p7.wrapping_add(0xc).write(0);
                    p7.wrapping_sub(4).write(0);
                }
                p7 = p7.wrapping_add(0x28);
                u8_ = u8_.wrapping_add(1);
                if u8_ >= rd(&raw mut DAT_00acebfd) {
                    break;
                }
            }
        }
    }
}
