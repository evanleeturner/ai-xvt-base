// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa6af0: u32;

    static mut DAT_00b6a5c6: u8;
    static mut DAT_00b6a5aa: u8;
    static mut DAT_00b6a5ae: u8;
    static mut DAT_00bb0106: u8;
    static mut DAT_00bb0112: u8;
    static mut DAT_00bb011e: u8;
    static mut DAT_00bb10a6: u8;
    static mut DAT_00bb10b2: u8;
    static mut DAT_00bb10be: u8;
    static mut DAT_00aa615e: u8;
    static mut DAT_00aa6166: u8;
    static mut DAT_00aa62ae: u8;
    static mut DAT_00aa62b4: u8;

    static mut DAT_00b6a2a8: i32;
    static mut DAT_00b69e30: i32;
    static mut DAT_00aa6114: i32;
    static mut DAT_00aa6134: i32;

    static mut DAT_00bb7394: u8;
    static mut DAT_00bb7395: u8;
    static mut DAT_00bb7396: u8;
    static mut DAT_00bb7397: u8;
    static mut DAT_00bb7398: u8;
    static mut DAT_00bb7399: u8;
    static mut DAT_00bb739a: u8;
    static mut DAT_00bb739b: u8;
    static mut DAT_00bb739c: u8;
    static mut DAT_00bb739d: u8;
    static mut DAT_00bb739e: u8;
    static mut DAT_00bb739f: u8;
    static mut DAT_00bb736f: u8;
    static mut DAT_00bb73a4: u8;
    static mut DAT_00bb7370: u8;
    static mut DAT_00bb73a5: u8;
    static mut DAT_00bb73a6: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cfe80(param_1: *mut i32) -> *mut i32;
    fn FUN_004d2250(param_1: i32) -> u32;
    fn FUN_004cef70(param_1: i32, param_2: *mut u32, param_3: u32);
    fn FUN_004d1400() -> u32;
    fn rand() -> i32;
}

#[inline(always)]
unsafe fn rd(p: *const u8) -> u32 {
    // SAFETY: caller passes a valid address in the game's memory.
    unsafe { p.cast::<u32>().read_unaligned() }
}

#[inline(always)]
unsafe fn wr(buf: *mut u32, i: i32, v: u32) {
    // SAFETY: buf is the game's packet buffer, large enough for the index.
    unsafe { buf.offset(i as isize).write(v) }
}

#[inline(always)]
unsafe fn rb(buf: *mut u32, i: i32) -> u32 {
    // SAFETY: as in wr.
    unsafe { buf.offset(i as isize).read() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004e6310(param_1: i32) {
    // SAFETY: all globals and callees are the game's own memory; pointers
    // are read and written at the offsets the original uses.
    unsafe {
        let buf: *mut u32 = &raw mut DAT_00aa6af0;
        let mut local_4: i32 = 0;
        let mut i_var6: i32;
        let mut i_var7: i32;
        let u_var5: u32;

        'a: {
            wr(buf, 0, 0x41);
            i_var6 = 0;
            while i_var6 < 8 {
                let t0 =
                    rd((&raw mut DAT_00b6a5c6 as *const u8).add((i_var6 as usize).wrapping_mul(4)));
                wr(buf, 1 + i_var6, t0);
                i_var6 += 1;
            }
            let t0 = rd(&raw mut DAT_00b6a5aa as *const u8);
            wr(buf, 10, t0);
            let t1 = rd((&raw mut DAT_00b6a5ae as *const u8).add(t0.wrapping_mul(4) as usize));
            wr(buf, 11, t1);

            if DAT_00b6a2a8 == 0 {
                wr(buf, 12, 8);
                i_var7 = 0xe;
                wr(buf, 13, FUN_004d1400());
                i_var6 = 0;
                let puvar4 = FUN_004cfe80(&mut local_4);
                if 0 < local_4 {
                    let mut pivar10 = puvar4.add(8);
                    loop {
                        if pivar10.add(1).read() == 1 {
                            wr(buf, i_var7, pivar10.read() as u32);
                            i_var7 = i_var7.wrapping_add(3);
                            let b = *(pivar10.sub(8) as *const u8);
                            wr(buf, i_var7 - 2, (b as u32).wrapping_sub(1));
                            let u = FUN_004d2250(pivar10.read());
                            wr(buf, i_var7 - 1, u);
                        }
                        pivar10 = pivar10.add(10);
                        i_var6 += 1;
                        if !(i_var6 < local_4) {
                            break;
                        }
                    }
                }
                FUN_004cef70(param_1, buf, (i_var7 as u32) << 2);
                let cmpv = rd(&raw mut DAT_00b6a5aa as *const u8) as i32;
                if cmpv != 4 {
                    break 'a;
                }
                i_var6 = 0;
                wr(buf, 5, 0);
                wr(buf, 0, 0x78);
                wr(buf, 4, 0);
                let pivar10 = (DAT_00aa6114
                    .wrapping_add(0x140)
                    .wrapping_add(DAT_00aa6134.wrapping_mul(0x148)))
                    as *const i32;
                let idx = pivar10.read_unaligned() as u32;
                let (pa, pb, pc): (*const u8, *const u8, *const u8) = if DAT_00b69e30 == 2 {
                    (
                        &raw mut DAT_00bb0106 as *const u8,
                        &raw mut DAT_00bb0112 as *const u8,
                        &raw mut DAT_00bb011e as *const u8,
                    )
                } else {
                    (
                        &raw mut DAT_00bb10a6 as *const u8,
                        &raw mut DAT_00bb10b2 as *const u8,
                        &raw mut DAT_00bb10be as *const u8,
                    )
                };
                let t0 = rd(pa.add(idx.wrapping_mul(0xa0) as usize));
                wr(buf, 1, t0);
                let t1 = rd(pb.add(idx.wrapping_mul(0xa0) as usize));
                wr(buf, 3, t1);
                let mut lim = t1 as i32;
                if -1 < lim {
                    loop {
                        if 9 < i_var6 {
                            break;
                        }
                        let off = (i_var6 as u32)
                            .wrapping_add(idx.wrapping_mul(0x28))
                            .wrapping_mul(4);
                        let t2 = rd(pc.add(off as usize)) as i32;
                        if t2 == 0 {
                            let v = rb(buf, 5).wrapping_add(1);
                            wr(buf, 5, v);
                        } else if t2 == 1 {
                            let v = rb(buf, 4).wrapping_add(1);
                            wr(buf, 4, v);
                        }
                        i_var6 += 1;
                        lim = rd(pb.add(idx.wrapping_mul(0xa0) as usize)) as i32;
                        if !(i_var6 <= lim) {
                            break;
                        }
                    }
                }
                wr(buf, 2, DAT_00bb73a6 as u32);
                u_var5 = 0x18;
            } else {
                i_var6 = 0xe;
                let mut pbvar10: *mut u8 = &raw mut DAT_00aa615e;
                wr(buf, 12, 8);
                wr(buf, 13, 8);
                let puvar4 = FUN_004cfe80(&mut local_4);
                let end1 = &raw mut DAT_00aa62ae as usize;
                loop {
                    i_var7 = 0;
                    if 0 < local_4 {
                        let mut pivar9 = puvar4.add(8);
                        loop {
                            let t0 = rd(pbvar10);
                            if pivar9.read() != 0
                                && pivar9.add(1).read() != 0
                                && (t0 as i32) == pivar9.read()
                            {
                                let b = *(puvar4.offset((i_var7 * 10) as isize) as *const u8);
                                let t1 = (b as u32).wrapping_sub(1);
                                pbvar10.add(4).cast::<u32>().write_unaligned(t1);
                                break;
                            }
                            pivar9 = pivar9.add(10);
                            i_var7 += 1;
                            if !(i_var7 < local_4) {
                                break;
                            }
                        }
                    }
                    if i_var7 == local_4 {
                        pbvar10.cast::<u32>().write_unaligned(0);
                    }
                    pbvar10 = pbvar10.add(0x2a);
                    if !((pbvar10 as usize) < end1) {
                        break;
                    }
                }
                pbvar10 = &raw mut DAT_00aa615e;
                loop {
                    let i_var1 = rd(pbvar10);
                    let i_var2 = rd(pbvar10.add(4));
                    wr(buf, i_var6, i_var1);
                    wr(buf, i_var6 + 1, i_var2);
                    i_var6 = i_var6.wrapping_add(3);
                    pbvar10 = pbvar10.add(0x2a);
                    let u = FUN_004d2250(i_var1 as i32);
                    wr(buf, i_var6 - 1, u);
                    if !((pbvar10 as usize) < end1) {
                        break;
                    }
                }
                FUN_004cef70(param_1, buf, (i_var6 as u32).wrapping_mul(4));
                i_var6 = 1;
                wr(buf, 0, 0x71);
                let mut pbvar4: *mut u8 = &raw mut DAT_00aa6166;
                let end2 = (&raw mut DAT_00aa62b4 as usize).wrapping_add(2);
                loop {
                    let t0 = rd(pbvar4);
                    let t1 = rd(pbvar4.add(4));
                    wr(buf, i_var6, t0);
                    let t2 = rd(pbvar4.add(8));
                    wr(buf, i_var6 + 1, t1);
                    i_var6 = i_var6.wrapping_add(5);
                    let t3 = rd(pbvar4.add(12));
                    wr(buf, i_var6 - 3, t2);
                    let t4 = rd(pbvar4.add(16));
                    wr(buf, i_var6 - 2, t3);
                    wr(buf, i_var6 - 1, t4);
                    pbvar4 = pbvar4.add(0x2a);
                    if !((pbvar4 as usize) < end2) {
                        break;
                    }
                }
                u_var5 = (i_var6 as u32).wrapping_mul(4);
            }
            FUN_004cef70(param_1, buf, u_var5);
        }

        wr(buf, 0, 0x59);
        wr(buf, 1, DAT_00bb7394 as u32);
        wr(buf, 2, DAT_00bb7395 as u32);
        wr(buf, 3, DAT_00bb7396 as u32);
        wr(buf, 4, DAT_00bb7397 as u32);
        wr(buf, 5, DAT_00bb7398 as u32);
        wr(buf, 6, DAT_00bb7399 as u32);
        wr(buf, 7, DAT_00bb739a as u32);
        wr(buf, 8, DAT_00bb739b as u32);
        wr(buf, 9, DAT_00bb739c as u32);
        wr(buf, 10, DAT_00bb739d as u32);
        wr(buf, 11, DAT_00bb739e as u32);
        wr(buf, 12, DAT_00bb739f as u32);
        wr(buf, 13, rand() as u32);
        wr(buf, 14, DAT_00bb736f as u32);
        wr(buf, 15, DAT_00bb73a4 as u32);
        wr(buf, 16, DAT_00bb7370 as u32);
        wr(buf, 17, DAT_00bb73a5 as u32);
        wr(buf, 18, DAT_00bb73a6 as u32);
        FUN_004cef70(param_1, buf, 0x4c);
    }
}
