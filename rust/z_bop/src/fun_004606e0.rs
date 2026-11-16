// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a08104: u32;
    static mut DAT_009a73b0: u16;
    static mut DAT_009a1fe8: u32;
    static mut DAT_009a73b6: u32;
    static mut DAT_009a73c0: u16;
    static mut DAT_009a73c2: u16;
    static mut DAT_009a73c4: u32;
    static mut DAT_009a73c8: u32;
    static mut DAT_009a73cc: u32;
    static mut DAT_009a73d8: u8;
    static mut DAT_009a73da: u8;
    static mut DAT_009a73db: u32;
    static mut DAT_009a73df: u32;
    static mut DAT_009a73e3: u32;
    static mut DAT_009d8a54: u32;
    static mut DAT_009d8a58: u32;
    static mut DAT_009d8b60: u32;
    static mut DAT_009ec604: u32;
    static mut DAT_00a08140: u32;
    static mut DAT_00a08144: u32;
    static mut DAT_00a08148: u32;
    static mut DAT_00a081e8: u32;
    static mut DAT_00a08298: u32;
    static mut DAT_009d8c84: [i8; 0];
    static mut DAT_009d9155: [i8; 0];
    static mut DAT_0051c5a6: [i8; 0];
    static mut DAT_0051c5b2: [i8; 0];
    static mut DAT_0051c5b4: [i8; 0];
    static mut DAT_0051c5b6: [i8; 0];
    static mut DAT_009d1320: [i8; 0];
    static mut DAT_009d8cd8: [u8; 0];
    static mut DAT_009d8cd9: [u8; 0];
    static mut DAT_009d8cda: [u8; 0];
    static mut DAT_009d8cdb: [u8; 0];
    static mut DAT_009d8cdc: [u8; 0];
    static mut DAT_009d8cde: [u8; 0];
    static mut DAT_009d8cdf: [u8; 0];
    static mut DAT_009d8ce0: [u8; 0];
    static mut DAT_009d8ce1: [u8; 0];
    static mut DAT_009d8ce2: [u8; 0];
    static mut s_starshipprotectpln_0051a2fc: [i8; 0];
    static mut s_starshipescortpln_005242b0: [i8; 0];
    static mut s_starshipdisablepln_0051a2d4: [i8; 0];
    static mut s_disableldr1pln_0051a310: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00461460(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16);
    fn FUN_004035d0(a: u32, b: i16, c: i16, d: i16);
    fn FUN_00460dd0(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16) -> u16;
    fn FUN_0045d5f0(a: u32) -> u16;
}

unsafe fn cmp(mut p: *const i8, mut q: *const i8, mut n: i32) -> bool {
    let mut r = false;
    loop {
        if n == 0 {
            break;
        }
        n -= 1;
        // SAFETY: reads the game's own string data, as the original repe cmpsb.
        unsafe {
            r = *p == *q;
            p = p.add(1);
            q = q.add(1);
        }
        if !r {
            break;
        }
    }
    r
}

unsafe fn rec(i: u8) -> *const i8 {
    // SAFETY: address arithmetic into the game's table.
    unsafe {
        (&raw const DAT_009d1320)
            .cast::<i8>()
            .add((i as usize).wrapping_mul(0x55))
    }
}

unsafe fn ga(p: *const u8, i: u32) -> u16 {
    // SAFETY: reads the game's own table.
    unsafe { p.add(i as usize).read() as u16 }
}

unsafe fn cur_rec() -> u8 {
    // SAFETY: reads the game's own pointer value and the byte it addresses.
    unsafe {
        let t6 = (&raw const DAT_009a73b6).read_unaligned();
        ((t6.wrapping_add(0xb)) as *const u8).read()
    }
}

unsafe fn disable_or_ldr() -> bool {
    // SAFETY: game data reads.
    unsafe {
        let mut r = cmp(
            rec(cur_rec()),
            (&raw const s_starshipdisablepln_0051a2d4).cast::<i8>(),
            0x13,
        );
        if !r {
            r = cmp(
                rec(cur_rec()),
                (&raw const s_disableldr1pln_0051a310).cast::<i8>(),
                0xf,
            );
        }
        r
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004606e0() -> u32 {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers are formed from game addresses exactly as the original does.
    unsafe {
        let mut b_var2: u8 = 1;
        if ((DAT_00a08104 + 7) as *const u8).read() == 3 {
            return DAT_00a08104 & 0xffff0000;
        }
        if ((DAT_00a08104.wrapping_add(0x1e5)) as *const u16).read_unaligned() == 0 {
            return DAT_00a08104 & 0xffff0000;
        }
        let mut i_var6: u32 = (DAT_009a73b0 as u32)
            .wrapping_mul(0x23)
            .wrapping_add(DAT_009a1fe8);
        let mut u_var4: u32 = ((i_var6.wrapping_add(0x16)) as *const u8).read() as u32;
        let idx = u_var4.wrapping_mul(0x566) as usize;
        if (&raw const DAT_009d8c84).cast::<i8>().add(idx).read() == 0x0e
            || (&raw const DAT_009d9155).cast::<i8>().add(idx).read() == 0x0e
        {
            return (u_var4.wrapping_mul(0x2b3) >> 0xf) << 0x10;
        }
        u_var4 = 0;
        let mut flag = false;
        loop {
            let k = ((DAT_00a08104.wrapping_add(4)) as *const u8).read() as u32;
            let off = k.wrapping_mul(0x101).wrapping_add(u_var4) as usize;
            if (&raw const DAT_0051c5a6).cast::<i8>().add(off).read() == 2 {
                flag = true;
            }
            let u7 = (u_var4.wrapping_add(1)) as u16;
            u_var4 = u7 as u32;
            if !(u7 < 2) {
                break;
            }
        }
        if !flag {
            return 0;
        }
        DAT_009a73d8 = 0;
        DAT_009a73da = 0x30;
        let prot = cmp(
            rec(cur_rec()),
            (&raw const s_starshipprotectpln_0051a2fc).cast::<i8>(),
            0x13,
        );
        if !prot {
            let esc = cmp(
                rec(cur_rec()),
                (&raw const s_starshipescortpln_005242b0).cast::<i8>(),
                0x12,
            );
            if !esc {
                if disable_or_ldr() {
                    DAT_009a73d8 = 1;
                } else {
                    DAT_009a73d8 = 0;
                }
            }
        }
        let is6 = ((i_var6.wrapping_add(3)) as *const i8).read() == 0x36;
        let mut local_a: u16 = 0;
        u_var4 = DAT_00a08104;
        if ((DAT_00a08104.wrapping_add(0x21d)) as *const u8).read() != 0 {
            loop {
                'cont: {
                    u_var4 = local_a as u32;
                    let puvar1_base = DAT_00a08104
                        .wrapping_add(0x3b2)
                        .wrapping_add(u_var4.wrapping_mul(4));
                    let mut puvar1 = puvar1_base as *mut u16;
                    if ((DAT_00a08104
                        .wrapping_add(0x370)
                        .wrapping_add(u_var4.wrapping_mul(4))) as *const u8)
                        .read()
                        != 2
                        || puvar1.read_unaligned() != 0xffff
                        || 0 < (puvar1.add(1).read_unaligned() as i16)
                    {
                        break 'cont;
                    }
                    puvar1
                        .add(1)
                        .write_unaligned(puvar1.add(1).read_unaligned().wrapping_add(0xec));
                    if b_var2 != 0 {
                        i_var6 = (DAT_009a73c2 as u32)
                            .wrapping_mul(0x52)
                            .wrapping_add((DAT_009a73c0 as u32).wrapping_mul(0x566));
                        FUN_00461460(
                            ga((&raw const DAT_009d8cde).cast(), i_var6),
                            ga((&raw const DAT_009d8cdf).cast(), i_var6),
                            ga((&raw const DAT_009d8ce2).cast(), i_var6),
                            ga((&raw const DAT_009d8ce0).cast(), i_var6),
                            ga((&raw const DAT_009d8ce1).cast(), i_var6),
                            0,
                        );
                        i_var6 = (DAT_009a73c2 as u32)
                            .wrapping_mul(0x52)
                            .wrapping_add((DAT_009a73c0 as u32).wrapping_mul(0x566));
                        FUN_00461460(
                            ga((&raw const DAT_009d8cd8).cast(), i_var6),
                            ga((&raw const DAT_009d8cda).cast(), i_var6),
                            ga((&raw const DAT_009d8cdc).cast(), i_var6),
                            ga((&raw const DAT_009d8cd9).cast(), i_var6),
                            ga((&raw const DAT_009d8cdb).cast(), i_var6),
                            1,
                        );
                        b_var2 = 0;
                    }
                    DAT_00a08144 = DAT_009a73c4;
                    DAT_00a08140 = DAT_009a73c8;
                    DAT_00a08148 = DAT_009a73cc;
                    if !is6 {
                        let i_var5 = (((DAT_00a08104.wrapping_add(4)) as *const u8).read() as u32)
                            .wrapping_mul(0x101);
                        i_var6 = i_var5.wrapping_add(u_var4.wrapping_mul(8));
                        let s1 = (&raw const DAT_0051c5b2)
                            .cast::<i8>()
                            .add(i_var6 as usize)
                            .cast::<i16>()
                            .read_unaligned();
                        let s2 = (&raw const DAT_0051c5b4)
                            .cast::<i8>()
                            .add(i_var6 as usize)
                            .cast::<i16>()
                            .read_unaligned();
                        let s3 = (&raw const DAT_0051c5b6)
                            .cast::<i8>()
                            .add(u_var4.wrapping_mul(8).wrapping_add(i_var5) as usize)
                            .cast::<i16>()
                            .read_unaligned();
                        FUN_004035d0(
                            (DAT_009a73b0 as u32)
                                .wrapping_mul(0x23)
                                .wrapping_add(DAT_009a1fe8),
                            s1,
                            s2,
                            s3,
                        );
                        if (((DAT_009a73b0 as u32)
                            .wrapping_mul(0x23)
                            .wrapping_add(3)
                            .wrapping_add(DAT_009a1fe8)) as *const i8)
                            .read()
                            == 0x35
                        {
                            DAT_009d8a58 = DAT_009d8a58.wrapping_mul(2);
                            DAT_009d8a54 = DAT_009d8a54.wrapping_mul(2);
                            DAT_009d8b60 = DAT_009d8b60.wrapping_mul(2);
                        }
                        DAT_009ec604 = DAT_00a08144.wrapping_add(DAT_009d8a58);
                        DAT_00a08140 = DAT_00a08140.wrapping_add(DAT_009d8a54);
                        DAT_00a08148 = DAT_00a08148.wrapping_add(DAT_009d8b60);
                        DAT_00a08144 = DAT_009ec604;
                        DAT_00a081e8 = DAT_00a08140;
                        DAT_00a08298 = DAT_00a08148;
                    }
                    (&raw mut DAT_009a73db).write_unaligned(DAT_00a08144);
                    (&raw mut DAT_009a73df).write_unaligned(DAT_00a08140);
                    (&raw mut DAT_009a73e3).write_unaligned(DAT_00a08148);
                    ((DAT_00a08104
                        .wrapping_add(0x372)
                        .wrapping_add(u_var4.wrapping_mul(4))) as *mut u8)
                        .write(0);
                    let mut pbvar3 = (&raw const DAT_009a73b6).read_unaligned().wrapping_add(0xb);
                    let mut do_72 = cmp(
                        rec((pbvar3 as *const u8).read()),
                        (&raw const s_starshipprotectpln_0051a2fc).cast::<i8>(),
                        0x13,
                    );
                    if !do_72 {
                        pbvar3 = (pbvar3 as *const u8).read() as u32;
                        do_72 = cmp(
                            rec(pbvar3 as u8),
                            (&raw const s_starshipescortpln_005242b0).cast::<i8>(),
                            0x12,
                        );
                    }
                    if do_72 {
                        let u7 = FUN_0045d5f0(((pbvar3 >> 0x10) << 0x10) | (DAT_009a73c2 as u32));
                        puvar1.write_unaligned(u7);
                        if u7 != 0xffff {
                            puvar1
                                .add(1)
                                .write_unaligned(puvar1.add(1).read_unaligned().wrapping_add(0xec));
                        }
                    } else {
                        i_var6 = (DAT_009a73c2 as u32)
                            .wrapping_mul(0x52)
                            .wrapping_add((DAT_009a73c0 as u32).wrapping_mul(0x566));
                        let mut u7 = FUN_00460dd0(
                            ga((&raw const DAT_009d8cde).cast(), i_var6),
                            ga((&raw const DAT_009d8cdf).cast(), i_var6),
                            ga((&raw const DAT_009d8ce2).cast(), i_var6),
                            ga((&raw const DAT_009d8ce0).cast(), i_var6),
                            ga((&raw const DAT_009d8ce1).cast(), i_var6),
                            0,
                        );
                        let mut found = true;
                        if u7 == 0xffff {
                            i_var6 = (DAT_009a73c2 as u32)
                                .wrapping_mul(0x52)
                                .wrapping_add((DAT_009a73c0 as u32).wrapping_mul(0x566));
                            u7 = FUN_00460dd0(
                                ga((&raw const DAT_009d8cd8).cast(), i_var6),
                                ga((&raw const DAT_009d8cda).cast(), i_var6),
                                ga((&raw const DAT_009d8cdc).cast(), i_var6),
                                ga((&raw const DAT_009d8cd9).cast(), i_var6),
                                ga((&raw const DAT_009d8cdb).cast(), i_var6),
                                1,
                            );
                            if u7 == 0xffff {
                                found = false;
                            }
                        }
                        if found {
                            puvar1.write_unaligned(u7);
                            puvar1 = puvar1.add(1);
                            puvar1.write_unaligned(puvar1.read_unaligned().wrapping_add(0xec));
                            if !disable_or_ldr() {
                                break 'cont;
                            }
                            ((DAT_00a08104
                                .wrapping_add(0x372)
                                .wrapping_add(u_var4.wrapping_mul(4)))
                                as *mut u8)
                                .write(1);
                        }
                    }
                }
                local_a = local_a.wrapping_add(1);
                u_var4 = local_a as u32;
                if !(u_var4 < ((DAT_00a08104.wrapping_add(0x21d)) as *const u8).read() as u32) {
                    break;
                }
            }
        }
        u_var4 & 0xffff0000
    }
}
