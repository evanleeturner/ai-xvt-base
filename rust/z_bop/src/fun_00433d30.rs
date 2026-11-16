// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: i32;
    static mut DAT_009a1fe0: i32;
    static mut DAT_009a2000: i16;
    static mut _DAT_00a07cec: i32;
    static mut DAT_009a20ca: [u16; 0];
    static mut DAT_009a2102: [u8; 0];
    static mut DAT_009a20d6: [u8; 0];
    static mut DAT_009a2108: [u8; 0];
    static mut DAT_009a20f6: [u8; 0];
    static mut DAT_009a2118: [u8; 0];
    static mut DAT_009a20f4: [u8; 0];
    static mut DAT_009a2117: [u8; 0];
    static mut DAT_009a20fa: [u8; 0];
    static mut DAT_009a211a: [u8; 0];
    static mut DAT_009d8c80: [u8; 0];
    static mut DAT_009d738a: [u8; 0];
    static mut DAT_009a2132: [u8; 0];
    static mut DAT_009a213c: [u8; 0];
    static mut DAT_009a20e8: [u8; 0];
    static mut DAT_009a2111: [u8; 0];
    static mut DAT_009a20d8: [u8; 0];
    static mut DAT_009a2109: [u8; 0];
    static mut DAT_009d8c88: [u8; 0];
    static mut DAT_009a215a: [u8; 0];
    static mut DAT_009a2164: [u8; 0];
    static mut DAT_009a20d4: [u8; 0];
    static mut DAT_009a2107: [u8; 0];
    static mut DAT_009a20e0: [u8; 0];
    static mut DAT_009a210d: [u8; 0];
    static mut DAT_009a20e4: [u8; 0];
    static mut DAT_009a210f: [u8; 0];
    static mut DAT_009d8ccb: [u8; 0];
    static mut DAT_009d8cca: [u8; 0];
    static mut DAT_009d8ccd: [u8; 0];
    static mut DAT_009d8ccc: [u8; 0];
    static mut DAT_009d8ccf: [u8; 0];
    static mut DAT_009d8cce: [u8; 0];
    static mut DAT_009d8cd1: [u8; 0];
    static mut DAT_009d8cd0: [u8; 0];
    static mut DAT_009d8cd2: [u8; 0];
    static mut DAT_009a20d0: [u8; 0];
    static mut DAT_009a20ee: [u8; 0];
    static mut DAT_009a20f0: [u8; 0];
    static mut DAT_009a20f2: [u8; 0];
    static mut DAT_009a20ec: [u8; 0];
    static mut DAT_00524120: [u8; 0];
    static mut DAT_009a7a40: [u8; 0];
    static mut DAT_009d1320: [u8; 0];
    static mut s_dropoffldr1pln_0051a2c4: [i8; 0];
    static mut DAT_009e9680: [i8; 0];
    static mut DAT_009ec468: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00434440(param_1: u32);
}

unsafe fn rb(a: u32) -> u8 {
    // SAFETY: address in the game's own memory.
    unsafe { (a as usize as *const u8).read() }
}
unsafe fn wb(a: u32, v: u8) {
    // SAFETY: address in the game's own memory.
    unsafe { (a as usize as *mut u8).write(v) }
}
unsafe fn rw(a: u32) -> i16 {
    // SAFETY: address in the game's own memory.
    unsafe { (a as usize as *const i16).read_unaligned() }
}
unsafe fn ww(a: u32, v: i16) {
    // SAFETY: address in the game's own memory.
    unsafe { (a as usize as *mut i16).write_unaligned(v) }
}
unsafe fn rd(a: u32) -> i32 {
    // SAFETY: address in the game's own memory.
    unsafe { (a as usize as *const i32).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00433d30(param_1: u16, param_2: u16, param_3: u16) {
    // SAFETY: all addresses are the game's own globals and tables, accessed
    // exactly where the original does.
    unsafe {
        let base = DAT_009a1fe8 as u32;
        let p1 = param_1 as u32;
        let p3 = param_3 as u32;
        let e = rd(base.wrapping_add(0x1f).wrapping_add(p1.wrapping_mul(0x23))) as u32;
        let i5 = rd(e.wrapping_add(0xa9)) as u32;
        if rb(i5.wrapping_add(8)) == 1 {
            return;
        }
        let a_20ca = (&raw mut DAT_009a20ca) as usize as u32;
        let a_2102 = (&raw mut DAT_009a2102) as usize as u32;
        let a_20d6 = (&raw mut DAT_009a20d6) as usize as u32;
        let a_2108 = (&raw mut DAT_009a2108) as usize as u32;
        let a_20f6 = (&raw mut DAT_009a20f6) as usize as u32;
        let a_2118 = (&raw mut DAT_009a2118) as usize as u32;
        let a_20f4 = (&raw mut DAT_009a20f4) as usize as u32;
        let a_2117 = (&raw mut DAT_009a2117) as usize as u32;
        let a_20fa = (&raw mut DAT_009a20fa) as usize as u32;
        let a_211a = (&raw mut DAT_009a211a) as usize as u32;
        let a_8c80 = (&raw mut DAT_009d8c80) as usize as u32;
        let a_738a = (&raw mut DAT_009d738a) as usize as u32;
        let a_2132 = (&raw mut DAT_009a2132) as usize as u32;
        let a_213c = (&raw mut DAT_009a213c) as usize as u32;
        let a_20e8 = (&raw mut DAT_009a20e8) as usize as u32;
        let a_2111 = (&raw mut DAT_009a2111) as usize as u32;
        let a_20d8 = (&raw mut DAT_009a20d8) as usize as u32;
        let a_2109 = (&raw mut DAT_009a2109) as usize as u32;
        let a_8c88 = (&raw mut DAT_009d8c88) as usize as u32;
        let a_215a = (&raw mut DAT_009a215a) as usize as u32;
        let a_2164 = (&raw mut DAT_009a2164) as usize as u32;
        let a_20d4 = (&raw mut DAT_009a20d4) as usize as u32;
        let a_2107 = (&raw mut DAT_009a2107) as usize as u32;
        let a_20e0 = (&raw mut DAT_009a20e0) as usize as u32;
        let a_210d = (&raw mut DAT_009a210d) as usize as u32;
        let a_20e4 = (&raw mut DAT_009a20e4) as usize as u32;
        let a_210f = (&raw mut DAT_009a210f) as usize as u32;
        let a_8ccb = (&raw mut DAT_009d8ccb) as usize as u32;
        let a_8cca = (&raw mut DAT_009d8cca) as usize as u32;
        let a_8ccd = (&raw mut DAT_009d8ccd) as usize as u32;
        let a_8ccc = (&raw mut DAT_009d8ccc) as usize as u32;
        let a_8ccf = (&raw mut DAT_009d8ccf) as usize as u32;
        let a_8cce = (&raw mut DAT_009d8cce) as usize as u32;
        let a_8cd1 = (&raw mut DAT_009d8cd1) as usize as u32;
        let a_8cd0 = (&raw mut DAT_009d8cd0) as usize as u32;
        let a_8cd2 = (&raw mut DAT_009d8cd2) as usize as u32;
        let a_20d0 = (&raw mut DAT_009a20d0) as usize as u32;
        let a_20ee = (&raw mut DAT_009a20ee) as usize as u32;
        let a_20f0 = (&raw mut DAT_009a20f0) as usize as u32;
        let a_20f2 = (&raw mut DAT_009a20f2) as usize as u32;
        let a_20ec = (&raw mut DAT_009a20ec) as usize as u32;
        let a_4120 = (&raw mut DAT_00524120) as usize as u32;
        let a_7a40 = (&raw mut DAT_009a7a40) as usize as u32;
        let a_1320 = (&raw mut DAT_009d1320) as usize as u32;
        let a_str = (&raw mut s_dropoffldr1pln_0051a2c4) as usize as u32;
        let a_9680 = (&raw mut DAT_009e9680) as usize as u32;
        let a_c468 = (&raw mut DAT_009ec468) as usize as u32;

        let u3 = param_2 as u32;
        wb(i5.wrapping_add(8), 1);
        let ca = a_20ca.wrapping_add(p3.wrapping_add(u3.wrapping_mul(0x93)).wrapping_mul(2));
        ww(ca, rw(ca).wrapping_add(1));
        let i7 = u3.wrapping_mul(0x566);
        let c1 = rb(a_8c80.wrapping_add(i7));
        let u126 = u3.wrapping_mul(0x126);
        if rb(i5.wrapping_add(0xb0)) == c1 {
            wb(a_2102.wrapping_add(u126).wrapping_add(p3), 1);
        }
        if param_3 == 2 {
            if rb(i5.wrapping_add(0x1eb)) != 0 {
                let a = a_20d6.wrapping_add(u126);
                ww(a, rw(a).wrapping_add(-1));
                if rb(i5.wrapping_add(0xb0)) == c1 {
                    wb(a_2108.wrapping_add(u126), 0);
                }
                wb(i5.wrapping_add(0x1eb), 0);
            }
            if rb(i5.wrapping_add(0x74)) != 0 {
                let a = a_20f6.wrapping_add(u126);
                ww(a, rw(a).wrapping_add(-1));
                if rb(a_8c80.wrapping_add(i7)) == rb(i5.wrapping_add(0xb0)) {
                    wb(a_2118.wrapping_add(u126), 0);
                }
            }
            if rb(i5.wrapping_add(0x73)) != 0 {
                let a = a_20f4.wrapping_add(u126);
                ww(a, rw(a).wrapping_add(-1));
                if rb(a_8c80.wrapping_add(i7)) == rb(i5.wrapping_add(0xb0)) {
                    wb(a_2117.wrapping_add(u126), 0);
                }
            }
            let a = a_20fa.wrapping_add(u126);
            ww(a, rw(a).wrapping_add(1));
            if rb(a_8c80.wrapping_add(i7)) == rb(i5.wrapping_add(0xb0)) {
                wb(a_211a.wrapping_add(u126), 1);
            }
        } else {
            let i4 = base.wrapping_add(p1.wrapping_mul(0x23));
            let t1 = rd(i4.wrapping_add(0x1b));
            let t2 = rd(i4.wrapping_add(0x1f)) as u32;
            if t1 != -1
                && rb(i5.wrapping_add(0x74)) == 0
                && rb(i5.wrapping_add(0x73)) == 0
                && rb(a_738a.wrapping_add((rb(t2.wrapping_add(0x89)) as u32).wrapping_mul(3))) != 1
            {
                let a = a_20f6.wrapping_add(u126);
                ww(a, rw(a).wrapping_add(1));
                if rb(i5.wrapping_add(0xb0)) == c1 {
                    wb(a_2118.wrapping_add(u126), 1);
                }
            }
        }
        let mut u11: u32 = 0;
        loop {
            if rb(u11.wrapping_add(0x1f6).wrapping_add(i5)) == 0 {
                let a = a_2132.wrapping_add(u126).wrapping_add(u11);
                wb(a, rb(a).wrapping_add(1));
                if rb(a_8c80.wrapping_add(i7)) == rb(i5.wrapping_add(0xb0)) {
                    wb(a_213c.wrapping_add(u126).wrapping_add(u11), 1);
                }
            }
            u11 = (u11 as u16).wrapping_add(1) as u32;
            if !((u11 as u16) < 10) {
                break;
            }
        }
        if rb(i5.wrapping_add(0x1ea)) == 0 {
            let a = a_20e8.wrapping_add(u126);
            ww(a, rw(a).wrapping_add(1));
            if rb(a_8c80.wrapping_add(i7)) == rb(i5.wrapping_add(0xb0)) {
                wb(a_2111.wrapping_add(u126), 1);
            }
        }
        if rb(i5.wrapping_add(0x1eb)) == 0 {
            let cv = rb(a_8c80.wrapping_add(i7));
            let a = a_20d8.wrapping_add(u126);
            ww(a, rw(a).wrapping_add(1));
            let i4 = u126;
            if cv == rb(i5.wrapping_add(0xb0)) {
                wb(a_2109.wrapping_add(i4), 1);
            }
            let mut u11: u32 = 0;
            let b2 = rb(a_8c88.wrapping_add(i7)) as u32;
            loop {
                if b2 != u11 {
                    let a = a_215a.wrapping_add(i4).wrapping_add(u11);
                    wb(a, rb(a).wrapping_add(1));
                    wb(a_2164.wrapping_add(i4).wrapping_add(u11), 1);
                }
                u11 = (u11 as u16).wrapping_add(1) as u32;
                if !((u11 as u16) < 10) {
                    break;
                }
            }
        }
        let mut found = false;
        let mut u11: u32 = 0;
        loop {
            if rb(u11.wrapping_add(0x1ec).wrapping_add(i5)) == 1 {
                found = true;
            }
            u11 = (u11 as u16).wrapping_add(1) as u32;
            if !((u11 as u16) < 10) {
                break;
            }
        }
        if !found {
            let a = a_20d4.wrapping_add(u126);
            ww(a, rw(a).wrapping_add(1));
            if rb(a_8c80.wrapping_add(i7)) == rb(i5.wrapping_add(0xb0)) {
                wb(a_2107.wrapping_add(u126), 1);
            }
        }
        if rb(i5.wrapping_add(0x7b)) == 0 {
            let a = a_20e0.wrapping_add(u126);
            ww(a, rw(a).wrapping_add(1));
            if rb(a_8c80.wrapping_add(i7)) == rb(i5.wrapping_add(0xb0)) {
                wb(a_210d.wrapping_add(u126), 1);
            }
        }
        if rb(i5.wrapping_add(0x7d)) == 0 {
            let cv = rb(a_8c80.wrapping_add(i7));
            let a = a_20e4.wrapping_add(u126);
            ww(a, rw(a).wrapping_add(1));
            if cv == rb(i5.wrapping_add(0xb0)) {
                wb(a_210f.wrapping_add(u126), 1);
            }
        }
        if param_3 == 2 {
            let mut u10: u16 = 0;
            if 0 < DAT_009a2000 {
                loop {
                    if u10 != param_2 {
                        let u11 = u10 as u32;
                        let i5b = u11.wrapping_mul(0x566);
                        let i126 = u11.wrapping_mul(0x126);
                        if rb(a_8ccb.wrapping_add(i5b)) != 0
                            && rb(a_8cca.wrapping_add(i5b)) as u32 == u3
                        {
                            FUN_00434440(u11);
                        }
                        if rb(a_8ccd.wrapping_add(i5b)) != 0
                            && rb(a_8ccc.wrapping_add(i5b)) as u32 == u3
                        {
                            let d = a_20d0.wrapping_add(i126);
                            ww(d, rw(d).wrapping_add(rw(a_20ee.wrapping_add(i126))));
                            ww(a_20ee.wrapping_add(i126), 0);
                        }
                        if rb(a_8ccf.wrapping_add(i5b)) != 0
                            && rb(a_8cce.wrapping_add(i5b)) as u32 == u3
                        {
                            let d = a_20d0.wrapping_add(i126);
                            ww(d, rw(d).wrapping_add(rw(a_20f0.wrapping_add(i126))));
                            ww(a_20f0.wrapping_add(i126), 0);
                        }
                        if rb(a_8cd1.wrapping_add(i5b)) != 0
                            && rb(a_8cd0.wrapping_add(i5b)) as u32 == u3
                        {
                            let d = a_20d0.wrapping_add(i126);
                            ww(d, rw(d).wrapping_add(rw(a_20f2.wrapping_add(i126))));
                            ww(a_20f2.wrapping_add(i126), 0);
                        }
                    }
                    u10 = u10.wrapping_add(1);
                    if !((u10 as i32) < (DAT_009a2000 as i32)) {
                        break;
                    }
                }
            }
            let mut local_4: i32 = 4;
            let mut pb = a_8cd2.wrapping_add(i7);
            loop {
                let mut c: i32 = 0xf;
                let mut p8 = a_1320.wrapping_add(
                    (rb(a_7a40.wrapping_add(rb(a_4120.wrapping_add(rb(pb) as u32)) as u32)) as u32)
                        .wrapping_mul(0x55),
                );
                let mut p12 = a_str;
                let mut eq;
                loop {
                    if c == 0 {
                        // bVar13 keeps its previous value
                        eq = true;
                        break;
                    }
                    c -= 1;
                    eq = rb(p8) == rb(p12);
                    p8 = p8.wrapping_add(1);
                    p12 = p12.wrapping_add(1);
                    if !eq {
                        break;
                    }
                }
                if eq {
                    FUN_00434440((rb(pb.wrapping_add(3)) as u16).wrapping_sub(1) as u32);
                }
                pb = pb.wrapping_add(0x52);
                local_4 -= 1;
                if local_4 == 0 {
                    break;
                }
            }
        }
        if param_3 == 0x11 && 0 < DAT_009a2000 {
            let mut u10: u16 = 0;
            let lim = DAT_009a2000 as i32;
            loop {
                if u10 != param_2 {
                    let u11 = u10 as u32;
                    let i7b = u11.wrapping_mul(0x566);
                    let i126 = u11.wrapping_mul(0x126);
                    if rb(a_8ccd.wrapping_add(i7b)) != 0
                        && rb(a_8ccc.wrapping_add(i7b)) as u32 == u3
                    {
                        let d = a_20ec.wrapping_add(i126);
                        ww(d, rw(d).wrapping_add(rw(a_20ee.wrapping_add(i126))));
                        ww(a_20ee.wrapping_add(i126), 0);
                    }
                    if rb(a_8ccf.wrapping_add(i7b)) != 0
                        && rb(a_8cce.wrapping_add(i7b)) as u32 == u3
                    {
                        let d = a_20ec.wrapping_add(i126);
                        ww(d, rw(d).wrapping_add(rw(a_20f0.wrapping_add(i126))));
                        ww(a_20f0.wrapping_add(i126), 0);
                    }
                    if rb(a_8cd1.wrapping_add(i7b)) != 0
                        && rb(a_8cd0.wrapping_add(i7b)) as u32 == u3
                    {
                        let d = a_20ec.wrapping_add(i126);
                        ww(d, rw(d).wrapping_add(rw(a_20f2.wrapping_add(i126))));
                        ww(a_20f2.wrapping_add(i126), 0);
                    }
                }
                u10 = u10.wrapping_add(1);
                if !((u10 as i32) < lim) {
                    break;
                }
            }
        }
        if _DAT_00a07cec < DAT_009a1fe0 {
            let mut i5c = _DAT_00a07cec.wrapping_mul(0x23) as u32;
            let mut i7c = _DAT_00a07cec;
            loop {
                if rb(base.wrapping_add(i5c).wrapping_add(3)) != 0 {
                    let t = rd(base.wrapping_add(i5c).wrapping_add(0x1f)) as u32;
                    let i4 = rd(t.wrapping_add(0xa9)) as u32;
                    if rw(i4.wrapping_add(0x6a)) as u16 == param_1 {
                        ww(i4.wrapping_add(0x6a), -1);
                    }
                }
                i5c = i5c.wrapping_add(0x23);
                i7c = i7c.wrapping_add(1);
                if !(i7c < DAT_009a1fe0) {
                    break;
                }
            }
        }
        let mut p8 = a_9680;
        loop {
            if rb(p8) != 0 {
                let mut p6 = p8.wrapping_add(0xe);
                let mut n = 4;
                loop {
                    if rw(p6) as u16 == param_1 {
                        ww(p6, -1);
                    }
                    p6 = p6.wrapping_add(2);
                    n -= 1;
                    if n == 0 {
                        break;
                    }
                }
            }
            p8 = p8.wrapping_add(0x5bd);
            if !(p8 < a_c468) {
                break;
            }
        }
    }
}
