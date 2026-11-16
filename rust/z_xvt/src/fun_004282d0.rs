// SPDX-License-Identifier: MIT
#![no_std]

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00536ef0: *mut u8;
    static mut DAT_00536ef4: u32;
    static mut DAT_00536ef8: u32;
    static mut DAT_00536f21: [u8; 4];
    static mut DAT_00536f25: [u8; 4];
    static mut _DAT_005200e0: f64;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00428980(param_1: *mut u8);
    fn FUN_004288b0(
        param_1: i8,
        param_2: i8,
        param_3: *mut u8,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    ) -> *mut u8;
    fn FUN_0047f010(param_1: i32);
    fn _exit(param_1: i32) -> !;
}

unsafe fn rd<T>(p: *const u8, o: usize) -> T {
    // SAFETY: the caller passes a pointer into the game's own memory.
    unsafe { p.byte_add(o).cast::<T>().read_unaligned() }
}

unsafe fn wr<T>(p: *mut u8, o: usize, v: T) {
    // SAFETY: the caller passes a pointer into the game's own memory.
    unsafe { p.byte_add(o).cast::<T>().write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004282d0(param_1: i32) {
    // SAFETY: all pointers and globals are the game's own memory, accessed
    // at the offsets and in the order the original uses.
    unsafe {
        let base = param_1 as usize as *mut u8;
        let u_a: u32 = rd(base, 0x1034);
        let d_var3 = u_a as f64;
        let u_b: u32 = rd(base, 0x1030);
        let mut local_4: u32 = 0;
        let d_a: f64 = rd(DAT_00536ef0, 0xe);
        let d_b = (u_b as f64) * d_var3 * _DAT_005200e0 + d_a;
        wr(DAT_00536ef0, 0xe, d_b);
        let mut pb14: *mut u8 = rd(base, 0x1086);
        let mut cnt: u32 = rd(base, 0x108e);
        if cnt != 0 {
            loop {
                let mut pb13: *mut u8 = DAT_00536ef0;
                let u_a: u32 = (&raw mut DAT_00536f25).cast::<u32>().read_unaligned();
                if 0x41241 < u_a {
                    let pb1 = pb13.add(2);
                    if *pb1 != 0 {
                        let mut iv: i32 = 0;
                        loop {
                            if ((1i32.wrapping_shl(iv as u32 & 0x1f)) as u32 & (*pb1 as u32)) != 0 {
                                let p7: *mut u8 = rd(pb13, (iv as usize) * 4 + 0x32);
                                FUN_00428980(p7);
                            }
                            iv += 1;
                            if iv >= 8 {
                                break;
                            }
                        }
                    }
                    if (*pb13.add(1)) as u32 == DAT_00536ef4 {
                        let parent: *mut u8 = rd(pb13, 0x2e);
                        let sh = (*pb13 as u32) & 0x1f;
                        *parent.add(2) = *parent.add(2) & !((1u32 << sh) as u8);
                        let a: u32 = rd(parent, 10);
                        let b: u32 = rd(pb13, 10);
                        wr(parent, 10, a.wrapping_add(b));
                        let a: f64 = rd(pb13, 0x16);
                        let b: f64 = rd(parent, 0x16);
                        wr(parent, 0x16, a + b);
                        let a: f64 = rd(pb13, 0x1e);
                        let b: f64 = rd(parent, 0x1e);
                        wr(parent, 0x1e, a + b);
                        let a: f64 = rd(pb13, 0x26);
                        let b: f64 = rd(parent, 0x26);
                        wr(parent, 0x26, a + b);
                        let c = (&raw mut DAT_00536f25).cast::<u32>().read_unaligned();
                        (&raw mut DAT_00536f25)
                            .cast::<u32>()
                            .write_unaligned(c.wrapping_sub(1));
                    }
                    DAT_00536ef4 = DAT_00536ef4.wrapping_sub(1);
                }
                let mut b9: u8 = 7;
                let mut local_18: u32 = 1;
                let u6: u32 = (*pb14.add(3) as u32).wrapping_add(1);
                pb13 = DAT_00536ef0;
                if DAT_00536ef4 != 0 {
                    loop {
                        let s = (b9 as u32) & 0x1f;
                        let b4 = ((*pb14.add(2) as u32) >> s) as u8;
                        let u2: u32 = (((((*pb14 as u32) >> s) & 1).wrapping_mul(2))
                            | (((*pb14.add(1) as u32) >> s) & 1))
                            .wrapping_mul(2);
                        let u11: u32 = u2 | ((b4 & 1) as u32);
                        let mut p7: *mut u8 = rd(pb13, (u11 as usize) * 4 + 0x32);
                        if p7.is_null() {
                            *pb13.add(2) =
                                *pb13.add(2) | (1i32.wrapping_shl((u11 as i8) as i32 as u32) as u8);
                            let mut c5: i8 = ((1u32.wrapping_shl(
                                8u32.wrapping_sub((local_18 as i8) as i32 as u32) & 0x1f,
                            ) >> 1) as u8) as i8;
                            let mut c12 = c5;
                            if (b4 & 1) == 0 {
                                c12 = c5.wrapping_neg();
                            }
                            let mut c8 = c5;
                            if (u2 & 2) == 0 {
                                c8 = c5.wrapping_neg();
                            }
                            if (u2 & 4) == 0 {
                                c5 = c5.wrapping_neg();
                            }
                            p7 = FUN_004288b0(
                                u11 as i8,
                                local_18 as i8,
                                pb13,
                                (*pb13.add(3) as u32).wrapping_add(c5 as i32 as u32) as i32,
                                (*pb13.add(4) as u32).wrapping_add(c8 as i32 as u32) as i32,
                                (*pb13.add(5) as u32).wrapping_add(c12 as i32 as u32) as i32,
                            );
                            wr(pb13, (u11 as usize) * 4 + 0x32, p7);
                            if p7.is_null() {
                                FUN_0047f010(0);
                                _exit(1);
                            }
                            if DAT_00536ef4 == local_18 {
                                DAT_00536ef8 = DAT_00536ef8.wrapping_add(1);
                            }
                        }
                        pb13 = rd(pb13, (u11 as usize) * 4 + 0x32);
                        b9 = b9.wrapping_sub(1);
                        local_18 = local_18.wrapping_add(1);
                        let tbl: *mut u8 = (&raw mut DAT_00536f21).cast::<u32>().read_unaligned()
                            as usize as *mut u8;
                        let o2 = (*pb14.add(2) as u32)
                            .wrapping_sub(*pb13.add(5) as u32)
                            .wrapping_mul(4);
                        let t2: u32 = rd(tbl, o2 as usize);
                        let o1 = (*pb14.add(1) as u32)
                            .wrapping_sub(*pb13.add(4) as u32)
                            .wrapping_mul(4);
                        let t1: u32 = rd(tbl, o1 as usize);
                        let o0 = (*pb14 as u32)
                            .wrapping_sub(*pb13.add(3) as u32)
                            .wrapping_mul(4);
                        let t0: u32 = rd(tbl, o0 as usize);
                        let d_a: f64 = rd(pb13, 0xe);
                        let d_b = ((t2 as f64) + (t1 as f64) + (t0 as f64)) * (u6 as f64) + d_a;
                        wr(pb13, 0xe, d_b);
                        if !(local_18 <= DAT_00536ef4) {
                            break;
                        }
                    }
                }
                let a: u32 = rd(pb13, 10);
                wr(pb13, 10, a.wrapping_add(u6));
                let u_b = (*pb14 as u32).wrapping_mul(u6);
                let d_a: f64 = rd(pb13, 0x16);
                wr(pb13, 0x16, (u_b as f64) + d_a);
                let u_b = (*pb14.add(1) as u32).wrapping_mul(u6);
                let d_a: f64 = rd(pb13, 0x1e);
                wr(pb13, 0x1e, (u_b as f64) + d_a);
                let b2 = *pb14.add(2);
                pb14 = pb14.add(6);
                let u_c = (b2 as u32).wrapping_mul(u6);
                let d_a: f64 = rd(pb13, 0x26);
                wr(pb13, 0x26, (u_c as f64) + d_a);
                local_4 = local_4.wrapping_add(1);
                cnt = rd(base, 0x108e);
                if !(local_4 < cnt) {
                    break;
                }
            }
        }
    }
}
