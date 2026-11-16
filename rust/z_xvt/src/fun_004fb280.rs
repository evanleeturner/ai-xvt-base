// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0081b850: i32;
    static mut DAT_00654c10: i32;
    static mut DAT_00654e6c: i32;
    static mut DAT_00654e68: i32;
    static mut DAT_0075633b: i8;
    static mut DAT_00654d48: i32;
    static mut DAT_00654d4c: i32;
    static mut DAT_00654d58: i32;
    static mut DAT_00654d5c: i32;
    static mut DAT_00654ce0: i32;
    static mut DAT_00654c50: i32;
    static mut DAT_00654d38: i32;
    static mut DAT_00654c40: i32;
    static mut DAT_00654bc0: i32;
    static mut DAT_00654c00: i32;
    static mut DAT_00654cac: i32;
    static mut DAT_00654cec: i32;
    static mut DAT_00654ca8: i32;
    static mut DAT_00654cdc: i32;
    static mut DAT_00654d44: i32;
    static mut DAT_00654bfc: i32;
    static mut DAT_00654bf8: i32;
    static mut DAT_00859c2c: i32;
    static mut DAT_0081bcd0: u32;
    static mut DAT_0081b854: u32;
    static mut DAT_0081b740: [i8; 1];

    static mut DAT_0081bfba: u8;
    static mut DAT_0081c022: u8;
    static mut DAT_0081fb96: u8;
    static mut DAT_0081bfb6: u8;
    static mut DAT_0081e016: u8;
    static mut DAT_0081e50a: u8;
    static mut DAT_0081f5de: u8;
    static mut DAT_0081f5ea: u8;
    static mut DAT_0081f5f6: u8;
    static mut DAT_0081f602: u8;
    static mut DAT_0081f60e: u8;
    static mut DAT_0081f61a: u8;
    static mut DAT_0081f626: u8;
    static mut DAT_0081f632: u8;
    static mut DAT_0081f63e: u8;
    static mut DAT_0081fc42: u8;
    static mut DAT_007438d8: u8;
    static mut DAT_0081f7da: u8;
    static mut DAT_0081f7de: u8;
    static mut DAT_0081f7e2: u8;
    static mut DAT_0081f7e6: u8;
    static mut DAT_0081fa82: u8;
    static mut DAT_0081fa8a: u8;
    static mut DAT_0081fa9e: u8;
    static mut DAT_0081e0a6: u8;
    static mut DAT_0081e0c6: u8;
    static mut DAT_0081e266: u8;
    static mut DAT_0081e286: u8;
    static mut DAT_0081e522: u8;
    static mut DAT_0081e682: u8;
    static mut DAT_0081ed62: u8;
    static mut DAT_0081f182: u8;
    static mut DAT_0081f506: u8;
    static mut DAT_0081f64a: u8;
    static mut DAT_0081f776: u8;

    static mut s__c_s__c_s__c_s__c_d__c_s_00534320: [i8; 1];
    static mut s__c_s__c_s_00531334: [i8; 1];
    static mut s__c_s__c_s__s__00534310: [i8; 1];
    static mut s__c_s__c_d_005333b4: [i8; 1];
    static mut s_rank_d_00532234: [i8; 1];
    static mut s_rcitlvl_d_00532210: [i8; 1];
    static mut s_citlvl_d_00532204: [i8; 1];
    static mut s_medlvl_d_005321f8: [i8; 1];
    static mut s__s____s_00534308: [i8; 1];
    static mut s__d___d__005321bc: [i8; 1];
    static mut DAT_005321c4: [i8; 1];
    static mut DAT_005321b8: [i8; 1];
    static mut DAT_005321b0: [i8; 1];
    static mut s__d__d__005321a8: [i8; 1];

    fn _sprintf(param_1: *mut i8, param_2: *const i8, ...) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dcfd0(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_004dcfa0(param_1: u32) -> *mut i8;
    fn FUN_004da920(param_1: i32, param_2: *mut i8, param_3: *mut i32, param_4: u32);
    fn FUN_004cdbc0() -> i32;
    fn FUN_004d9010(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
    ) -> i32;
    fn FUN_004da7c0(param_1: i32, param_2: *mut i8, param_3: i32, param_4: i32, param_5: u32);
    fn FUN_004c9300(param_1: u32);
    fn FUN_004d0e60(param_1: *mut i8, param_2: i32, param_3: i32);
    fn FUN_004db2f0(param_1: *mut i8, param_2: i32) -> i32;
}

macro_rules! cs {
    ($n:ident) => {
        (&raw const $n).cast::<i8>()
    };
}

macro_rules! buf {
    () => {
        (&raw mut DAT_0081b740).cast::<i8>()
    };
}

macro_rules! at {
    ($n:ident, $off:expr) => {
        (&raw mut $n).byte_add(($off) as usize)
    };
}

unsafe fn rd(p: *const u8) -> i32 {
    // SAFETY: the pointer addresses the game's own memory.
    unsafe { p.cast::<i32>().read_unaligned() }
}

unsafe fn rdu(p: *const u8) -> u32 {
    // SAFETY: the pointer addresses the game's own memory.
    unsafe { p.cast::<u32>().read_unaligned() }
}

fn inr(a: i32, b: i32) -> bool {
    b <= a && a.wrapping_sub(b) < 0x15
}

#[unsafe(no_mangle)]
#[allow(non_snake_case, unused_assignments, unused_variables, unused_mut)]
pub unsafe extern "stdcall" fn FUN_004fb280() -> u32 {
    // SAFETY: all globals and callees are the game's own; pointers walk the
    // game's tables exactly as the original does.
    unsafe {
        let mut v11: i32 = 0;
        let mut v8: i32;
        let mut v10: i32;
        let mut v12: i32;
        let mut v16: i32;
        let mut u7: u32;
        let mut u13: u32;
        let mut u14: u32;
        let mut u15: u32;
        let mut u17: u32;
        let mut u18: u32;
        let mut u19: u32;
        let mut p2: *mut i8;
        let mut p3: *mut i8;
        let mut p4: *mut i8;
        let mut p5: *mut i8;
        let mut local_14 = [0i8; 20];
        let mut local_24 = [0i32; 4];
        let mut local_28: i32;

        FUN_004dcfd0(local_24.as_mut_ptr(), 0x54, 0x5a, 0x1b2, 0x6a);
        u19 = 0xffff;
        p2 = FUN_004dcfa0(299);
        FUN_004da920(0xf, p2, local_24.as_mut_ptr(), u19);
        local_28 = 0;
        if DAT_0081b850 != 2 {
            let mut pb9 = &raw mut DAT_0081f7de;
            loop {
                v8 = FUN_004cdbc0();
                if v8 == rd(pb9) {
                    break;
                }
                pb9 = pb9.byte_add(0x58);
                local_28 = local_28.wrapping_add(1);
                if !(pb9 < (&raw mut DAT_0081fa9e)) {
                    break;
                }
            }
        }
        if local_28 == 8 {
            return 0;
        }
        if DAT_00654c10 != 0 {
            DAT_00654e6c = 0xd;
            DAT_00654e68 = 0;
            DAT_00654c10 = 0;
            if DAT_0075633b == 0 {
                if rd(&raw mut DAT_0081bfba) != 1 {
                    if rd(&raw mut DAT_0081bfba) != 2 {
                        DAT_00654e6c = 0xe;
                    }
                }
            }
            if DAT_0081b850 != 2 {
                DAT_00654e6c = DAT_00654e6c.wrapping_add(5);
            }
            if rd(&raw mut DAT_0081bfba) == 1 {
                DAT_00654e6c = DAT_00654e6c.wrapping_add(1);
            }
            if rd(&raw mut DAT_0081c022) != 0 {
                DAT_00654e6c = DAT_00654e6c.wrapping_add(1);
            }
            let t0 = rd(&raw mut DAT_0081fb96);
            let mut pb9 = at!(DAT_0081fc42, (t0 as u32).wrapping_mul(0xe824));
            v8 = 4;
            loop {
                if rd(pb9) != 0 {
                    DAT_00654e6c = DAT_00654e6c.wrapping_add(1);
                }
                pb9 = pb9.byte_add(4);
                v8 -= 1;
                if v8 == 0 {
                    break;
                }
            }
            DAT_00654d48 = 0;
            DAT_00654d58 = 0;
            DAT_00654ce0 = 0;
            DAT_00654c50 = 0;
            DAT_00654d38 = 0;
            DAT_00654c40 = 0;
            DAT_00654bc0 = 0;
            DAT_00654c00 = 0;
            DAT_00654cac = 0;
            pb9 = &raw mut DAT_0081e522;
            loop {
                v8 = 1;
                DAT_00654cec = 0;
                let mut pb6 = pb9;
                loop {
                    if !(rd(pb6) == 0 && rd(pb6.byte_add(0x420)) == 0) {
                        DAT_00654cec = 1;
                        DAT_00654cac = 1;
                    }
                    pb6 = pb6.byte_add(0x160);
                    v8 -= 1;
                    if v8 == 0 {
                        break;
                    }
                }
                if DAT_00654cec != 0 {
                    DAT_00654e6c = DAT_00654e6c.wrapping_add(1);
                }
                pb9 = pb9.byte_add(4);
                if !(pb9 < (&raw mut DAT_0081e682)) {
                    break;
                }
            }
            if DAT_00654cac != 0 {
                DAT_00654e6c = DAT_00654e6c.wrapping_add(2);
            }
            DAT_00654ca8 = 0;
            if DAT_0081b850 != 2 {
                pb9 = &raw mut DAT_0081e0a6;
                loop {
                    if !(rd(pb9) == 0 && rd(pb9.byte_add(0x20)) == 0) {
                        DAT_00654ca8 = 1;
                        DAT_00654e6c = DAT_00654e6c.wrapping_add(1);
                    }
                    pb9 = pb9.byte_add(4);
                    if !(pb9 < (&raw mut DAT_0081e0c6)) {
                        break;
                    }
                }
                if DAT_00654ca8 != 0 {
                    DAT_00654e6c = DAT_00654e6c.wrapping_add(2);
                }
            }
            DAT_00654cdc = 0;
            if DAT_0081b850 != 2 {
                pb9 = &raw mut DAT_0081e266;
                loop {
                    if !(rd(pb9) == 0 && rd(pb9.byte_add(0x20)) == 0) {
                        DAT_00654cdc = 1;
                        DAT_00654e6c = DAT_00654e6c.wrapping_add(1);
                    }
                    pb9 = pb9.byte_add(4);
                    if !(pb9 < (&raw mut DAT_0081e286)) {
                        break;
                    }
                }
                if DAT_00654cdc != 0 {
                    DAT_00654e6c = DAT_00654e6c.wrapping_add(2);
                }
            }
            v8 = 0;
            pb9 = &raw mut DAT_0081ed62;
            loop {
                v10 = 0x58;
                loop {
                    v12 = rd(pb9);
                    v16 = rd(pb9.byte_sub(0x420));
                    pb9 = pb9.byte_add(4);
                    let q = at!(DAT_00654c00, v8).cast::<i32>();
                    q.write(q.read().wrapping_add(v12));
                    v10 -= 1;
                    let q = at!(DAT_00654bc0, v8).cast::<i32>();
                    q.write(q.read().wrapping_add(v16));
                    if v10 == 0 {
                        break;
                    }
                }
                v8 += 4;
                if !(v8 < 4) {
                    break;
                }
            }
            v8 = 0;
            pb9 = &raw mut DAT_0081f182;
            loop {
                v10 = 0x19;
                loop {
                    v12 = rd(pb9);
                    v16 = rd(pb9.byte_add(0x12c));
                    pb9 = pb9.byte_add(4);
                    let q = at!(DAT_00654c40, v8).cast::<i32>();
                    q.write(q.read().wrapping_add(v12));
                    v10 -= 1;
                    let q = at!(DAT_00654d38, v8).cast::<i32>();
                    q.write(q.read().wrapping_add(v16));
                    if v10 == 0 {
                        break;
                    }
                }
                v8 += 4;
                if !(v8 < 4) {
                    break;
                }
            }
            v8 = 0;
            pb9 = &raw mut DAT_0081f506;
            loop {
                v10 = 6;
                loop {
                    v12 = rd(pb9);
                    v16 = rd(pb9.byte_add(0x48));
                    pb9 = pb9.byte_add(4);
                    let q = at!(DAT_00654c50, v8).cast::<i32>();
                    q.write(q.read().wrapping_add(v12));
                    v10 -= 1;
                    let q = at!(DAT_00654ce0, v8).cast::<i32>();
                    q.write(q.read().wrapping_add(v16));
                    if v10 == 0 {
                        break;
                    }
                }
                v8 += 4;
                if !(v8 < 4) {
                    break;
                }
            }
            let mut pi6 = &raw mut DAT_00654d58;
            pb9 = &raw mut DAT_0081f64a;
            loop {
                v8 = 0x19;
                loop {
                    v10 = rd(pb9);
                    pb9 = pb9.byte_add(4);
                    v8 -= 1;
                    pi6.write(pi6.read().wrapping_add(v10));
                    if v8 == 0 {
                        break;
                    }
                }
                pi6 = pi6.add(1);
                if !(pi6 < (&raw mut DAT_00654d5c)) {
                    break;
                }
            }
            pi6 = &raw mut DAT_00654d48;
            pb9 = &raw mut DAT_0081f776;
            loop {
                v8 = 6;
                loop {
                    v10 = rd(pb9);
                    pb9 = pb9.byte_add(4);
                    v8 -= 1;
                    pi6.write(pi6.read().wrapping_add(v10));
                    if v8 == 0 {
                        break;
                    }
                }
                pi6 = pi6.add(1);
                if !(pi6 < (&raw mut DAT_00654d4c)) {
                    break;
                }
            }
        }
        FUN_004dcfd0(local_24.as_mut_ptr(), 0x1a9, 0x6b, 0x1b2, 0x1b1);
        if 0x15 < DAT_00654e6c {
            DAT_00654e68 = FUN_004d9010(
                local_24.as_mut_ptr(),
                DAT_00654e68,
                DAT_00654e6c,
                0,
                5,
                DAT_00859c2c,
                10,
            );
        }
        v8 = 0x6f;
        let t0 = rd(&raw mut DAT_0081bfba);
        if DAT_0081b850 != 2 && t0 == 1 {
            if DAT_00654e68 < 1 && -0x15 < DAT_00654e68 {
                if DAT_00654d44 == 0 {
                    u19 = 0x2a5;
                } else {
                    u19 = 0x163;
                }
                p2 = FUN_004dcfa0(u19);
                u17 = 4;
                u15 = 1;
                u18 = DAT_00654bfc as u32;
                p3 = FUN_004dcfa0(0x142);
                u14 = 4;
                p4 = FUN_004dcfa0(DAT_00654bf8.wrapping_add(0x131) as u32);
                u13 = 1;
                p5 = FUN_004dcfa0(0x162);
                _sprintf(
                    buf!(),
                    cs!(s__c_s__c_s__c_s__c_d__c_s_00534320),
                    4i32,
                    p5,
                    u13,
                    p4,
                    u14,
                    p3,
                    u15,
                    u18,
                    u17,
                    p2,
                );
                v8 = 0x7e;
                FUN_004da7c0(0xc, buf!(), 0x58, 0x6f, 0xffff);
            }
            v11 = 1;
        }
        v10 = v8;
        'lab: {
            if DAT_0075633b != 0 {
                break 'lab;
            }
            if rd(&raw mut DAT_0081bfba) == 1 {
                break 'lab;
            }
            if rd(&raw mut DAT_0081bfba) == 2 {
                break 'lab;
            }
            if inr(v11, DAT_00654e68) {
                let t0 = rd(&raw mut DAT_0081bfb6);
                u19 = rdu(at!(DAT_0081fa82, (t0 as u32).wrapping_mul(28)));
                let t1 = rd(&raw mut DAT_0081bfba);
                if t1 == 0 {
                    u7 = (u19 == 0) as u32;
                } else {
                    let t0 = rd(&raw mut DAT_0081bfb6);
                    u7 = rdu(at!(DAT_0081fa82, ((t0 ^ 1) as u32).wrapping_mul(28)));
                }
                let simple: bool;
                if u7 == u19 {
                    u19 = 0x14e;
                    simple = true;
                } else if u19 != 1 {
                    u19 = 0x166;
                    simple = true;
                } else {
                    simple = false;
                }
                if simple {
                    p2 = FUN_004dcfa0(u19);
                    u18 = 1;
                    p3 = FUN_004dcfa0(0x164);
                    _sprintf(buf!(), cs!(s__c_s__c_s_00531334), 4i32, p3, u18, p2);
                } else {
                    let t0 = rd(at!(DAT_0081f7da, (local_28 as u32).wrapping_mul(0x58)));
                    let ut0 = *at!(DAT_007438d8, (t0 as u32).wrapping_mul(0x562)) as u32;
                    u7 = rdu(at!(DAT_0081fa8a, ut0.wrapping_mul(28)));
                    FUN_004c9300(u7);
                    u19 = 0xffffffff;
                    p2 = buf!();
                    loop {
                        p3 = p2;
                        if u19 == 0 {
                            break;
                        }
                        u19 = u19.wrapping_sub(1);
                        p3 = p2.add(1);
                        let c1 = *p2;
                        p2 = p3;
                        if c1 == 0 {
                            break;
                        }
                    }
                    u19 = !u19;
                    p2 = p3.wrapping_sub(u19 as usize);
                    p3 = local_14.as_mut_ptr();
                    u7 = u19 >> 2;
                    while u7 != 0 {
                        p3.cast::<u32>()
                            .write_unaligned(p2.cast::<u32>().read_unaligned());
                        p2 = p2.add(4);
                        p3 = p3.add(4);
                        u7 -= 1;
                    }
                    u19 &= 3;
                    while u19 != 0 {
                        *p3 = *p2;
                        p2 = p2.add(1);
                        p3 = p3.add(1);
                        u19 -= 1;
                    }
                    p2 = local_14.as_mut_ptr();
                    p3 = FUN_004dcfa0(0x165);
                    u18 = 1;
                    p4 = FUN_004dcfa0(0x164);
                    _sprintf(buf!(), cs!(s__c_s__c_s__s__00534310), 4i32, p4, u18, p3, p2);
                }
                v10 = v8.wrapping_add(0xf);
                FUN_004da7c0(0xc, buf!(), 0x58, v8, 0xffff);
            }
            v11 = v11.wrapping_add(1);
        }
        v8 = v10;
        if inr(v11, DAT_00654e68) {
            u18 = rdu(at!(DAT_0081f7e6, (local_28 as u32).wrapping_mul(0x58)));
            u13 = 1;
            p2 = FUN_004dcfa0(0x140);
            _sprintf(buf!(), cs!(s__c_s__c_d_005333b4), 4i32, p2, u13, u18);
            v8 = v10.wrapping_add(0xf);
            FUN_004da7c0(0xc, buf!(), 0x58, v10, 0xffff);
        }
        v12 = v11.wrapping_add(1);
        v10 = v8;
        if rd(&raw mut DAT_0081c022) != 0 {
            if inr(v12, DAT_00654e68) {
                let t0 = rd(&raw mut DAT_0081e016);
                _sprintf(buf!(), cs!(s_rank_d_00532234), t0);
                FUN_004d0e60(buf!(), 0x58, v8.wrapping_add(1));
                let t0 = rd(&raw mut DAT_0081e016);
                p2 = FUN_004dcfa0((t0 as u32).wrapping_add(0x6e));
                u18 = 6;
                let t1 = rd(&raw mut DAT_0081c022);
                p3 = FUN_004dcfa0((t1 as u32).wrapping_add(0x16a));
                _sprintf(buf!(), cs!(s__c_s__c_s_00531334), 4i32, p3, u18, p2);
                v10 = v8.wrapping_add(0xf);
                FUN_004da7c0(0xc, buf!(), 0x91, v8, 0xffff);
            }
            v12 = v11.wrapping_add(2);
        }
        DAT_00654cec = 0;
        v11 = 0;
        loop {
            v8 = v10;
            let t2 = rd(&raw mut DAT_0081fb96);
            let pb_entry = at!(
                DAT_0081fc42,
                (t2 as u32)
                    .wrapping_mul(0xe824)
                    .wrapping_add((v11 as u32).wrapping_mul(4))
            );
            if rd(pb_entry) != 0 {
                if inr(v12, DAT_00654e68) {
                    if DAT_00654cec == 0 {
                        v16 = 0x58;
                        u19 = DAT_0081bcd0;
                        p2 = FUN_004dcfa0(0x16c);
                        FUN_004da7c0(0xc, p2, v16, v8, u19);
                        DAT_00654cec = 1;
                    }
                    v8 = 0xc;
                    p2 = FUN_004dcfa0(0x16c);
                    v16 = FUN_004db2f0(p2, v8);
                    let fmt: *const i8;
                    if v11 == 2 {
                        let t1 = rd(&raw mut DAT_0081fb96);
                        if t1 == 0 {
                            fmt = cs!(s_rcitlvl_d_00532210);
                            u18 = rdu(at!(DAT_0081fc42, 8));
                        } else {
                            let t2 = rd(&raw mut DAT_0081fb96);
                            u18 = rdu(at!(
                                DAT_0081fc42,
                                (t2 as u32).wrapping_mul(0xe824).wrapping_add(8)
                            ));
                            fmt = cs!(s_citlvl_d_00532204);
                        }
                    } else {
                        let t2 = rd(&raw mut DAT_0081fb96);
                        u18 = rdu(at!(
                            DAT_0081fc42,
                            (t2 as u32)
                                .wrapping_mul(0xe824)
                                .wrapping_add((v11 as u32).wrapping_mul(4))
                        ));
                        fmt = cs!(s_medlvl_d_005321f8);
                    }
                    _sprintf(buf!(), fmt, u18);
                    FUN_004d0e60(buf!(), v16.wrapping_add(0x62), v10);
                    let t2 = rd(&raw mut DAT_0081fb96);
                    if v11 == 2 {
                        let t1 = rd(at!(
                            DAT_0081fc42,
                            (t2 as u32).wrapping_mul(0xe824).wrapping_add(8)
                        ));
                        u19 = (t1 as u32).wrapping_add(0x285);
                    } else {
                        let t1 = rd(at!(
                            DAT_0081fc42,
                            (t2 as u32)
                                .wrapping_mul(0xe824)
                                .wrapping_add((v11 as u32).wrapping_mul(4))
                        ));
                        u19 = (t1 as u32).wrapping_add(0x170);
                    }
                    p2 = FUN_004dcfa0(u19);
                    p3 = FUN_004dcfa0(v11.wrapping_add(0x16d) as u32);
                    _sprintf(buf!(), cs!(s__s____s_00534308), p3, p2);
                    v8 = v10.wrapping_add(0xf);
                    FUN_004da7c0(0xc, buf!(), v16.wrapping_add(0x8a), v10, 0xffff);
                }
                v12 = v12.wrapping_add(1);
            }
            v11 += 1;
            v10 = v8;
            if !(v11 < 4) {
                break;
            }
        }
        if inr(v12, DAT_00654e68) {
            v8 = v8.wrapping_add(0xf);
        }
        v11 = v8;
        if inr(v12.wrapping_add(1), DAT_00654e68) {
            v10 = 0x58;
            v11 = v8.wrapping_add(0xf);
            u19 = DAT_0081bcd0;
            p2 = FUN_004dcfa0(0x151);
            FUN_004da7c0(0xc, p2, v10, v8, u19);
        }
        v8 = v11;
        if inr(v12.wrapping_add(2), DAT_00654e68) {
            v10 = 0x58;
            u19 = DAT_0081bcd0;
            p2 = FUN_004dcfa0(0x11c);
            FUN_004da7c0(0xc, p2, v10, v8, u19);
            let t0 = rd(&raw mut DAT_0081e50a);
            _sprintf(buf!(), cs!(s__d___d__005321bc), t0, DAT_00654bc0);
            v8 = v11.wrapping_add(0xf);
            FUN_004da7c0(0xc, buf!(), 0x10c, v11, 0xffff);
        }
        v11 = v12.wrapping_add(3);
        if DAT_0081b850 != 2 {
            v10 = v8;
            if inr(v11, DAT_00654e68) {
                u19 = DAT_0081bcd0;
                p2 = FUN_004dcfa0(0x152);
                FUN_004da7c0(0xc, p2, 0x58, v8, u19);
                let t0 = rd(&raw mut DAT_0081bfba);
                if t0 == 0 {
                    _sprintf(buf!(), cs!(DAT_005321c4));
                } else {
                    _sprintf(buf!(), cs!(s__d___d__005321bc), DAT_00654c40, DAT_00654d38);
                }
                v10 = v8.wrapping_add(0xf);
                FUN_004da7c0(0xc, buf!(), 0x10c, v8, 0xffff);
            }
            v8 = v10;
            if inr(v12.wrapping_add(4), DAT_00654e68) {
                u19 = DAT_0081bcd0;
                p2 = FUN_004dcfa0(0x153);
                FUN_004da7c0(0xc, p2, 0x58, v10, u19);
                _sprintf(buf!(), cs!(s__d___d__005321bc), DAT_00654c50, DAT_00654ce0);
                v8 = v10.wrapping_add(0xf);
                FUN_004da7c0(0xc, buf!(), 0x10c, v10, 0xffff);
            }
            v11 = v12.wrapping_add(5);
        }
        v10 = v8;
        if inr(v11, DAT_00654e68) {
            v12 = 0x58;
            u19 = DAT_0081bcd0;
            p2 = FUN_004dcfa0(0x154);
            FUN_004da7c0(0xc, p2, v12, v10, u19);
            _sprintf(buf!(), cs!(DAT_005321b8), DAT_00654c00);
            v10 = v8.wrapping_add(0xf);
            FUN_004da7c0(0xc, buf!(), 0x10c, v8, 0xffff);
        }
        v8 = v10;
        if inr(v11.wrapping_add(1), DAT_00654e68) {
            v12 = 0x58;
            u19 = DAT_0081bcd0;
            p2 = FUN_004dcfa0(0x155);
            FUN_004da7c0(0xc, p2, v12, v8, u19);
            let t0 = rd(&raw mut DAT_0081f5de);
            _sprintf(buf!(), cs!(DAT_005321b8), t0);
            v8 = v10.wrapping_add(0xf);
            FUN_004da7c0(0xc, buf!(), 0x10c, v10, 0xffff);
        }
        v10 = v8;
        if inr(v11.wrapping_add(2), DAT_00654e68) {
            v12 = 0x58;
            u19 = DAT_0081bcd0;
            p2 = FUN_004dcfa0(0x156);
            FUN_004da7c0(0xc, p2, v12, v10, u19);
            u19 = 0;
            let mut ut0 = rdu(&raw mut DAT_0081f5f6);
            if ut0 != 0 {
                u7 = rdu(&raw mut DAT_0081f5ea);
                ut0 = rdu(&raw mut DAT_0081f5f6);
                u19 = u7.wrapping_mul(100) / ut0;
            }
            _sprintf(buf!(), cs!(DAT_005321b0), u19);
            v10 = v8.wrapping_add(0xf);
            FUN_004da7c0(0xc, buf!(), 0x10c, v8, 0xffff);
        }
        v8 = v10;
        if inr(v11.wrapping_add(3), DAT_00654e68) {
            v12 = 0x58;
            u19 = DAT_0081bcd0;
            p2 = FUN_004dcfa0(0x157);
            FUN_004da7c0(0xc, p2, v12, v8, u19);
            u19 = 0;
            let mut ut0 = rdu(&raw mut DAT_0081f60e);
            if ut0 != 0 {
                u7 = rdu(&raw mut DAT_0081f602);
                ut0 = rdu(&raw mut DAT_0081f60e);
                u19 = u7.wrapping_mul(100) / ut0;
            }
            _sprintf(buf!(), cs!(DAT_005321b0), u19);
            v8 = v10.wrapping_add(0xf);
            FUN_004da7c0(0xc, buf!(), 0x10c, v10, 0xffff);
        }
        v10 = v11.wrapping_add(4);
        if DAT_00654ca8 != 0 {
            v12 = v8;
            if inr(v10, DAT_00654e68) {
                v12 = v8.wrapping_add(0xf);
            }
            v8 = v12;
            if inr(v11.wrapping_add(5), DAT_00654e68) {
                v10 = 0x58;
                v8 = v12.wrapping_add(0xf);
                u19 = DAT_0081bcd0;
                p2 = FUN_004dcfa0(0x158);
                FUN_004da7c0(0xc, p2, v10, v12, u19);
            }
            local_28 = 0x18;
            v10 = v11.wrapping_add(6);
            loop {
                let mut pb9 = &raw mut DAT_0081f7e2;
                let mut pb6 = &raw mut DAT_0081e0a6;
                v11 = v8;
                loop {
                    v8 = v11;
                    if rd(pb9) == local_28 {
                        if !(rd(pb6) == 0 && rd(pb6.byte_add(0x20)) == 0) {
                            if inr(v10, DAT_00654e68) {
                                u18 = 1;
                                p2 = FUN_004dcfa0(local_28.wrapping_add(0x8e) as u32);
                                _sprintf(
                                    buf!(),
                                    cs!(s__c_s__c_s_00531334),
                                    6i32,
                                    p2,
                                    u18,
                                    pb9.byte_sub(0x16).cast::<i8>(),
                                );
                                FUN_004da7c0(0xc, buf!(), 0x58, v11, DAT_0081bcd0);
                                let t1 = rd(pb6);
                                let t2 = rd(pb6.byte_add(0x20));
                                _sprintf(buf!(), cs!(s__d__d__005321a8), t1, t2);
                                v8 = v11.wrapping_add(0xf);
                                FUN_004da7c0(0xc, buf!(), 0x10c, v11, 0xffff);
                            }
                            v10 = v10.wrapping_add(1);
                        }
                    }
                    pb9 = pb9.byte_add(0x58);
                    pb6 = pb6.byte_add(4);
                    v11 = v8;
                    if !(pb6 < (&raw mut DAT_0081e0c6)) {
                        break;
                    }
                }
                local_28 -= 1;
                if !(-1 < local_28) {
                    break;
                }
            }
        }
        if DAT_00654cac != 0 {
            v11 = v8;
            if inr(v10, DAT_00654e68) {
                v11 = v8.wrapping_add(0xf);
            }
            v8 = v11;
            if inr(v10.wrapping_add(1), DAT_00654e68) {
                v12 = 0x58;
                v8 = v11.wrapping_add(0xf);
                u19 = DAT_0081bcd0;
                p2 = FUN_004dcfa0(0x159);
                FUN_004da7c0(0xc, p2, v12, v11, u19);
            }
            v10 = v10.wrapping_add(2);
        }
        v11 = 0;
        let mut pb9 = &raw mut DAT_0081e522;
        loop {
            v12 = 1;
            DAT_00654cec = 0;
            let mut pb6 = pb9;
            loop {
                if !(rd(pb6) == 0 && rd(pb6.byte_add(0x420)) == 0) {
                    DAT_00654cec = 1;
                }
                pb6 = pb6.byte_add(0x160);
                v12 -= 1;
                if v12 == 0 {
                    break;
                }
            }
            v12 = v8;
            if DAT_00654cec != 0 {
                if inr(v10, DAT_00654e68) {
                    v16 = 0x58;
                    u19 = DAT_0081b854;
                    p2 = FUN_004dcfa0(v11.wrapping_add(0x15) as u32);
                    FUN_004da7c0(0xc, p2, v16, v12, u19);
                    let t0 = rd(pb9);
                    let t1 = rd(pb9.byte_add(0x420));
                    _sprintf(buf!(), cs!(s__d___d__005321bc), t0, t1);
                    v12 = v8.wrapping_add(0xf);
                    FUN_004da7c0(0xc, buf!(), 0x10c, v8, 0xffff);
                }
                v10 = v10.wrapping_add(1);
            }
            pb9 = pb9.byte_add(4);
            v11 += 1;
            v8 = v12;
            if !(pb9 < (&raw mut DAT_0081e682)) {
                break;
            }
        }
        if inr(v10, DAT_00654e68) {
            v12 = v12.wrapping_add(0xf);
        }
        v11 = v12;
        if inr(v10.wrapping_add(1), DAT_00654e68) {
            v8 = 0x58;
            v11 = v12.wrapping_add(0xf);
            u19 = DAT_0081bcd0;
            p2 = FUN_004dcfa0(0x15b);
            FUN_004da7c0(0xc, p2, v8, v12, u19);
        }
        v8 = v11;
        if inr(v10.wrapping_add(2), DAT_00654e68) {
            v12 = 0x58;
            u19 = DAT_0081bcd0;
            p2 = FUN_004dcfa0(0x15c);
            FUN_004da7c0(0xc, p2, v12, v8, u19);
            let t0 = rd(&raw mut DAT_0081f61a);
            _sprintf(buf!(), cs!(DAT_005321b8), t0);
            v8 = v11.wrapping_add(0xf);
            FUN_004da7c0(0xc, buf!(), 0x10c, v11, 0xffff);
        }
        v11 = v10.wrapping_add(3);
        if DAT_0081b850 != 2 {
            v12 = v8;
            if inr(v11, DAT_00654e68) {
                u19 = DAT_0081bcd0;
                p2 = FUN_004dcfa0(0x15d);
                FUN_004da7c0(0xc, p2, 0x58, v8, u19);
                _sprintf(buf!(), cs!(DAT_005321b8), DAT_00654d58);
                v12 = v8.wrapping_add(0xf);
                FUN_004da7c0(0xc, buf!(), 0x10c, v8, 0xffff);
            }
            v8 = v12;
            if inr(v10.wrapping_add(4), DAT_00654e68) {
                u19 = DAT_0081bcd0;
                p2 = FUN_004dcfa0(0x15e);
                FUN_004da7c0(0xc, p2, 0x58, v12, u19);
                _sprintf(buf!(), cs!(DAT_005321b8), DAT_00654d48);
                v8 = v12.wrapping_add(0xf);
                FUN_004da7c0(0xc, buf!(), 0x10c, v12, 0xffff);
            }
            v11 = v10.wrapping_add(5);
        }
        v10 = v8;
        if inr(v11, DAT_00654e68) {
            v12 = 0x58;
            u19 = DAT_0081bcd0;
            p2 = FUN_004dcfa0(0x15f);
            FUN_004da7c0(0xc, p2, v12, v10, u19);
            let t0 = rd(&raw mut DAT_0081f632);
            _sprintf(buf!(), cs!(DAT_005321b8), t0);
            v10 = v8.wrapping_add(0xf);
            FUN_004da7c0(0xc, buf!(), 0x10c, v8, 0xffff);
        }
        v8 = v10;
        if inr(v11.wrapping_add(1), DAT_00654e68) {
            v12 = 0x58;
            u19 = DAT_0081bcd0;
            p2 = FUN_004dcfa0(0x160);
            FUN_004da7c0(0xc, p2, v12, v8, u19);
            let t0 = rd(&raw mut DAT_0081f63e);
            _sprintf(buf!(), cs!(DAT_005321b8), t0);
            v8 = v10.wrapping_add(0xf);
            FUN_004da7c0(0xc, buf!(), 0x10c, v10, 0xffff);
        }
        v10 = v8;
        if inr(v11.wrapping_add(2), DAT_00654e68) {
            v12 = 0x58;
            u19 = DAT_0081bcd0;
            p2 = FUN_004dcfa0(0x161);
            FUN_004da7c0(0xc, p2, v12, v10, u19);
            let t0 = rd(&raw mut DAT_0081f626);
            _sprintf(buf!(), cs!(DAT_005321b8), t0);
            v10 = v8.wrapping_add(0xf);
            FUN_004da7c0(0xc, buf!(), 0x10c, v8, 0xffff);
        }
        if DAT_00654cdc != 0 {
            if inr(v11.wrapping_add(3), DAT_00654e68) {
                v10 = v10.wrapping_add(0xf);
            }
            v8 = v10;
            if inr(v11.wrapping_add(4), DAT_00654e68) {
                v12 = 0x58;
                v8 = v10.wrapping_add(0xf);
                u19 = DAT_0081bcd0;
                p2 = FUN_004dcfa0(0x177);
                FUN_004da7c0(0xc, p2, v12, v10, u19);
            }
            local_28 = 0x18;
            v11 = v11.wrapping_add(5);
            loop {
                let mut pb6 = &raw mut DAT_0081f7e2;
                let mut pb9 = &raw mut DAT_0081e266;
                v10 = v8;
                loop {
                    v8 = v10;
                    if rd(pb6) == local_28 {
                        if !(rd(pb9) == 0 && rd(pb9.byte_add(0x20)) == 0) {
                            if inr(v11, DAT_00654e68) {
                                u18 = 1;
                                p2 = FUN_004dcfa0(local_28.wrapping_add(0x8e) as u32);
                                _sprintf(
                                    buf!(),
                                    cs!(s__c_s__c_s_00531334),
                                    6i32,
                                    p2,
                                    u18,
                                    pb6.byte_sub(0x16).cast::<i8>(),
                                );
                                FUN_004da7c0(0xc, buf!(), 0x58, v10, DAT_0081bcd0);
                                let t1 = rd(pb9);
                                let t2 = rd(pb9.byte_add(0x20));
                                _sprintf(buf!(), cs!(s__d__d__005321a8), t1, t2);
                                v8 = v10.wrapping_add(0xf);
                                FUN_004da7c0(0xc, buf!(), 0x10c, v10, 0xffff);
                            }
                            v11 = v11.wrapping_add(1);
                        }
                    }
                    pb6 = pb6.byte_add(0x58);
                    pb9 = pb9.byte_add(4);
                    v10 = v8;
                    if !(pb9 < (&raw mut DAT_0081e286)) {
                        break;
                    }
                }
                local_28 -= 1;
                if !(-1 < local_28) {
                    break;
                }
            }
        }
        1
    }
}
