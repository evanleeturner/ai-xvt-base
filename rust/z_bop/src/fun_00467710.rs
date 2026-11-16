// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a08104: *mut u8;
    static mut DAT_009a73c0: u16;
    static mut DAT_009a73b0: u16;
    static mut DAT_009a73c2: u16;
    static mut DAT_009a1fe8: *mut u8;
    static mut DAT_009d8cc5: [u8; 0];
    static mut DAT_009d8c80: [u8; 0];
    static mut DAT_009d9192: [u8; 0];
    static mut DAT_009d8c30: [u8; 0];
    static mut DAT_009d8cd2: [u8; 0];
    static mut DAT_009a20f4: [u8; 0];
    static mut DAT_009a20f6: [u8; 0];
    static mut DAT_009a2117: [u8; 0];
    static mut DAT_009a2118: [u8; 0];
    static mut DAT_0051c5aa: [u8; 0];
    static mut DAT_0051c5ac: [u8; 0];
    static mut DAT_00524120: [u8; 0];
    static mut DAT_009a7a40: [u8; 0];
    static mut DAT_009d1320: [u8; 0];
    static mut DAT_0051a358: [i8; 0];
    static mut DAT_00a08292: u16;
    static mut _DAT_00a07be2: u16;
    static mut _DAT_00a07be4: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00425ba0(a: u32, b: u32) -> u32;
    fn FUN_00405820(a: u32) -> u32;
    fn FUN_00425b70(a: u32, b: u16) -> u32;
    fn FUN_00430200(a: i32, b: i32, c: i32, d: u16) -> u32;
    fn FUN_00451bf0(a: i32, b: i32);
    fn FUN_0043c740(a: u32, b: *mut u8) -> u32;
    fn FUN_00450650(a: i32, b: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00467710() -> u8 {
    // SAFETY: all globals and callees are the game's own memory and code;
    // p points at the game's record, accessed at the original's offsets.
    unsafe {
        let p = DAT_00a08104;
        let rd16 = |q: *mut u8| q.cast::<u16>().read_unaligned();
        let rd32 = |q: *mut u8| q.cast::<u32>().read_unaligned();
        if rd16(p.byte_add(0x92)) == 0 {
            return 0;
        }
        let mut b11: u8 = 0;
        let u7 = ((DAT_009a73c0 as u32).wrapping_mul(0x2b3) >> 16) as u16;
        let c0 = DAT_009a73c0 as u32;
        let c1 = DAT_009a73b0 as u32;
        let mut u_stack4: u16 = 0;
        let sel = (&raw mut DAT_009d8cc5)
            .cast::<u8>()
            .add(c0.wrapping_mul(0x566) as usize)
            .read();
        match sel {
            1 => {
                b11 = ((p.byte_add(0x1e5).read() & 1) == 0) as u8;
                let c2 = DAT_009a1fe8
                    .add(c1.wrapping_mul(0x23).wrapping_add(2) as usize)
                    .read() as i8;
                if c2 == 3 || c2 == 4 {
                    let i0 = rd32(p.byte_add(0x215));
                    let i1 = rd32(p.byte_add(0x211));
                    if i0.wrapping_add(i1) == 0 {
                        b11 = 1;
                    }
                }
                u_stack4 = 0x186;
            }
            2 => {
                b11 = ((p.byte_add(0x1e5).read() & 0x10) == 0) as u8;
                u_stack4 = 0x18a;
            }
            3 => {
                b11 = 0;
                let mut local6: u16 = 0;
                let b1 = p.byte_add(0x232).read();
                let mut b12: u8 = 0;
                if b1 != 0 {
                    loop {
                        b11 = b12;
                        let mut u4 = local6 as u32;
                        if p.byte_add(u4.wrapping_add(0x233) as usize).read() != 0 {
                            let i8v = (p.byte_add(4).read() as u32)
                                .wrapping_mul(0x101)
                                .wrapping_add(u4) as i32;
                            let lo = (&raw mut DAT_0051c5aa)
                                .cast::<u8>()
                                .add(i8v as usize)
                                .read();
                            let hi = (&raw mut DAT_0051c5ac)
                                .cast::<u8>()
                                .add(i8v as usize)
                                .read();
                            u4 = lo as u32;
                            let mut u3 = lo as u16;
                            while u3 <= hi as u16 {
                                if p.byte_add(u4.wrapping_mul(4).wrapping_add(0x372) as usize)
                                    .read()
                                    != 0
                                {
                                    b11 = 1;
                                }
                                u3 = (u4 as u16).wrapping_add(1);
                                u4 = u3 as u32;
                            }
                        }
                        local6 = local6.wrapping_add(1);
                        b12 = b11;
                        if !(local6 < b1 as u16) {
                            break;
                        }
                    }
                }
                b11 = (b11 == 0 || (p.byte_add(0x1e5).read() & 8) == 0) as u8;
                u_stack4 = 0x18a;
            }
            4 => {
                let i0 = rd32(p.byte_add(0xcb));
                let u4 = FUN_00425ba0(i0, 0x8000);
                let i1 = rd32(p.byte_add(0xc3));
                b11 = (u4 <= i1) as u8;
                u_stack4 = 0x188;
            }
            5 => {
                let mut u3: u16 = 0;
                loop {
                    if p.byte_add((u3 as usize) + 0x1ec).read() != 0 {
                        b11 = 1;
                    }
                    u3 = u3.wrapping_add(1);
                    if !(u3 < 10) {
                        break;
                    }
                }
                u_stack4 = 0x18b;
            }
            6 => {
                let u4 = FUN_00405820(((u7 as u32) << 16) | c1);
                let u4 = FUN_00425b70(u4, 0x8000);
                let i0 = rd32(p.byte_add(0x215));
                let i1 = rd32(p.byte_add(0x211));
                b11 = (i0.wrapping_add(i1) <= (u4 & 0xffff)) as u8;
                u_stack4 = 0x184;
            }
            7 => {
                let u4 = FUN_00405820(((u7 as u32) << 16) | c1);
                let u4 = FUN_00425b70(u4, 0x4000);
                let i0 = rd32(p.byte_add(0x215));
                let i1 = rd32(p.byte_add(0x211));
                b11 = (i0.wrapping_add(i1) <= (u4 & 0xffff)) as u8;
                u_stack4 = 0x185;
            }
            8 => {
                let i0 = rd32(p.byte_add(0xcb));
                let u4 = FUN_00425ba0(i0, 0x4000);
                let i1 = rd32(p.byte_add(0xc3));
                b11 = (u4 <= i1) as u8;
                u_stack4 = 0x187;
            }
            9 => {
                let i0 = rd32(p.byte_add(0xcb));
                let u4 = FUN_00425ba0(i0, 0xc000);
                let i1 = rd32(p.byte_add(0xc3));
                b11 = (u4 <= i1) as u8;
                u_stack4 = 0x189;
            }
            _ => {}
        }
        if b11 != 0 {
            if p.byte_add(0x73).read() == 0 {
                let u4 = DAT_009a73c0 as u32;
                let off = u4.wrapping_mul(0x126) as usize;
                let q = (&raw mut DAT_009a20f4).cast::<u8>().add(off).cast::<u16>();
                q.write_unaligned(q.read_unaligned().wrapping_add(1));
                let i8v = u4.wrapping_mul(0x126) as i32;
                let pc9 = p.byte_add(0xb0).cast::<i8>();
                let c2 = (&raw mut DAT_009d8c80)
                    .cast::<u8>()
                    .add(u4.wrapping_mul(0x566) as usize)
                    .read() as i8;
                if pc9.read() == c2 {
                    (&raw mut DAT_009a2117)
                        .cast::<u8>()
                        .add(i8v as usize)
                        .write(1);
                }
                let p5 = p.byte_add(0x74);
                if p5.read() != 0 {
                    let q = (&raw mut DAT_009a20f6)
                        .cast::<u8>()
                        .add(i8v as usize)
                        .cast::<u16>();
                    q.write_unaligned(q.read_unaligned().wrapping_sub(1));
                    if pc9.read() == c2 {
                        (&raw mut DAT_009a2118)
                            .cast::<u8>()
                            .add(i8v as usize)
                            .write(0);
                    }
                    p5.write(0);
                }
                FUN_00430200(1, 0xf, DAT_009a73b0 as i32, 0xffff);
                let c0 = DAT_009a73c0 as u32;
                let mut sv = (&raw mut DAT_009d9192)
                    .cast::<u8>()
                    .add(c0.wrapping_mul(0x566) as usize)
                    .cast::<i32>()
                    .read_unaligned();
                if sv != -1 {
                    let pp = DAT_009a1fe8
                        .add((DAT_009a73b0 as u32).wrapping_mul(0x23).wrapping_add(0x1f) as usize)
                        .cast::<*mut u8>()
                        .read_unaligned();
                    DAT_00a08292 = pp.byte_add(0x88).read() as u16;
                    let base = (&raw mut DAT_009d8c30) as usize as i32;
                    FUN_00451bf0(
                        0,
                        base.wrapping_add((DAT_009a73c0 as u32).wrapping_mul(0x566) as i32),
                    );
                    let u6 = FUN_0043c740(DAT_009a73c0 as u32, DAT_00a08104);
                    _DAT_00a07be2 = u6 as u16;
                    _DAT_00a07be4 = u_stack4;
                    sv = (&raw mut DAT_009d9192)
                        .cast::<u8>()
                        .add((DAT_009a73c0 as u32).wrapping_mul(0x566) as usize)
                        .cast::<i32>()
                        .read_unaligned();
                    FUN_00450650(0x183, sv);
                }
                let mut b12 = (DAT_009a73c2 as u32).wrapping_mul(0x52) == 0;
                let mut cnt: i32 = 0x10;
                let k = (DAT_009a73c0 as u32)
                    .wrapping_mul(0x566)
                    .wrapping_add((DAT_009a73c2 as u32).wrapping_mul(0x52));
                let a = (&raw mut DAT_009d8cd2).cast::<u8>().add(k as usize).read();
                let b = (&raw mut DAT_00524120).cast::<u8>().add(a as usize).read();
                let c = (&raw mut DAT_009a7a40).cast::<u8>().add(b as usize).read();
                let mut s9 = (&raw mut DAT_009d1320)
                    .cast::<u8>()
                    .add((c as usize).wrapping_mul(0x55))
                    .cast::<i8>();
                let mut s10 = (&raw mut DAT_0051a358).cast::<i8>();
                loop {
                    if cnt == 0 {
                        break;
                    }
                    cnt -= 1;
                    b12 = s9.read() == s10.read();
                    s9 = s9.add(1);
                    s10 = s10.add(1);
                    if !b12 {
                        break;
                    }
                }
                let sh0 = rd16(p.byte_add(0xcf));
                if b12 && sh0 == 0 {
                    let sh1 = rd16(p.byte_add(0x1e3));
                    p.byte_add(0x1e5).cast::<u16>().write_unaligned(sh1);
                }
            }
            p.byte_add(0x73).write(1);
        }
        b11
    }
}
