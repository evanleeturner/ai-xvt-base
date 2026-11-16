// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00523410: i32;
    static mut DAT_009ecc34: i32;
    static mut DAT_009d695c: i8;
    static mut DAT_009ed22e: u16;
    static mut DAT_009ed22d: u8;
    static mut DAT_009ed22c: u8;
    static mut DAT_009ed228: u32;
    static mut DAT_009d8b74: u32;
    static mut DAT_009d8b70: u32;
    static mut DAT_00a08292: u16;
    static mut DAT_009d7684: u16;
    static mut DAT_009a1840: [*mut u8; 1];
    static mut DAT_00a07be0: [u16; 1];
    static mut DAT_00a08120: [*mut u8; 1];
    static mut DAT_00520eb0: [u16; 1];
    static mut DAT_00a00860: i32;
    static mut DAT_005235d4: i32;
    static mut DAT_005235d0: u16;
    static mut DAT_005235d8: i32;
    static mut DAT_009ecc3c: i32;
    static mut DAT_009ec5fc: u16;
    static mut DAT_009993fc: u32;
    static mut DAT_009d77f8: u8;
    static mut DAT_009a7b54: i32;
    static mut DAT_005569f8: i32;
    static mut DAT_00556a01: i8;
    static mut DAT_00556a50: u32;
    static mut DAT_009a6ff0: u32;
    static mut DAT_009a6ff9: u8;
    static mut DAT_009a7044: [u16; 1];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_00450550();
    fn FUN_004ac740();
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ac720(param_1: u32) -> u32;
    fn FUN_00450c90(param_1: u16);
    fn FUN_00450bc0();
}

unsafe fn copy_blk(dst: *mut u32, src: *const u32) {
    let mut i = 0usize;
    while i < 0x15 {
        // SAFETY: copies 0x15 dwords between the local block and game memory.
        unsafe {
            dst.add(i).write_unaligned(src.add(i).read_unaligned());
        }
        i += 1;
    }
}

unsafe fn udiv16(a: u16, b: u16) -> u16 {
    let q: u32;
    // SAFETY: the machine's own div; faults on a zero divisor like the original.
    unsafe {
        core::arch::asm!("div {0}", in(reg) b as u32, inout("eax") a as u32 => q, inout("edx") 0u32 => _, options(nostack));
    }
    q as u16
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00450650(param_1: i32, param_2: i32) {
    // SAFETY: all globals and callees are the game's own; pointers read are
    // the game's string tables, and the local block is written in bounds.
    unsafe {
        let mut b2: u8 = DAT_009d77f8;
        if DAT_00523410 == 0 && param_2 == DAT_009ecc34 {
            let mut blk = [0u32; 21];
            let p: *mut u8 = blk.as_mut_ptr().cast();
            let src: *const u32 = blk.as_ptr();
            p.cast::<u16>().write_unaligned(param_1 as u16);
            if DAT_009d695c == 0 {
                p.byte_add(4).cast::<u16>().write_unaligned(DAT_009ed22e);
                p.byte_add(6).write(DAT_009ed22d);
                p.byte_add(7).write(DAT_009ed22c);
                p.byte_add(8).write((DAT_009ed228 >> 24) as u8);
            } else {
                p.byte_add(4)
                    .cast::<u16>()
                    .write_unaligned((DAT_009d8b74 >> 16) as u16);
                p.byte_add(6).write((DAT_009d8b74 >> 8) as u8);
                p.byte_add(7).write(DAT_009d8b74 as u8);
                p.byte_add(8).write((DAT_009d8b70 >> 24) as u8);
            }
            p.byte_add(12).write(0);
            p.byte_add(13).write(0);
            p.byte_add(10).cast::<u16>().write_unaligned(DAT_00a08292);
            if param_1 == 0xcf || param_1 == 0xc4 {
                p.byte_add(2).cast::<u16>().write_unaligned(DAT_009d7684);
            } else {
                p.byte_add(2).cast::<u16>().write_unaligned(0);
            }
            let tbl = (&raw mut DAT_009a1840).cast::<*mut u8>();
            let idx_tbl = (&raw mut DAT_00a07be0).cast::<u16>();
            let hi_tbl = (&raw mut DAT_00a08120).cast::<*mut u8>();
            let div_tbl = (&raw mut DAT_00520eb0).cast::<u16>();
            let s = p.byte_add(14);

            let mut u9: u16 = 0;
            let mut pb12: *mut u8 = tbl.offset(param_1 as isize).read();
            let mut c: u8 = *pb12;
            let mut u4: u32 = (c as u32) << 0x10;
            let mut u5a: u32 = u4;
            let mut overflow = false;
            while c != 0 {
                if 0x45 < u9 {
                    overflow = true;
                    break;
                }
                c = *pb12;
                u5a = u4;
                if c == 0x2a {
                    pb12 = pb12.add(1);
                    u5a = (u5a & 0xffff0000) | (u5a as u16).wrapping_add(1) as u32;
                    let u5 = idx_tbl.add((u4 & 0xffff) as usize).read();
                    let mut p6: *mut u8 = if u5 < 0x8000 {
                        tbl.add(u5 as usize).read()
                    } else {
                        hi_tbl.add((u5 & 0x7fff) as usize).read()
                    };
                    c = *p6;
                    while c != 0 && u9 < 0x46 {
                        c = *p6;
                        p6 = p6.add(1);
                        s.add(u9 as usize).write(c);
                        c = *p6;
                        u9 = u9.wrapping_add(1);
                    }
                } else if c == 0x26 {
                    let p1 = pb12.add(1);
                    let mut u11: u32 = *p1 as u32;
                    pb12 = pb12.add(2);
                    let mut seen = false;
                    u5a = (u5a & 0xffff0000) | (u5a as u16).wrapping_add(1) as u32;
                    let mut u5c: u16 = idx_tbl.add((u4 & 0xffff) as usize).read();
                    if *p1 != 0 {
                        loop {
                            if 0x45 < u9 {
                                break;
                            }
                            let d = div_tbl.add(u11 as usize).read();
                            let mut q = udiv16(u5c, d);
                            u5c = u5c.wrapping_sub((d as u32).wrapping_mul(q as u32) as u16);
                            let ch: u16;
                            if seen || (u11 as u16) < 2 || q != 0 {
                                seen = true;
                                if 9 < q {
                                    q = 9;
                                }
                                ch = q.wrapping_add(0x30);
                            } else {
                                ch = 0x20;
                            }
                            if ch != 0x20 {
                                s.add(u9 as usize).write(ch as u8);
                                u9 = u9.wrapping_add(1);
                            }
                            let u5 = (u11 as u16).wrapping_sub(1);
                            u11 = u5 as u32;
                            if u5 == 0 {
                                break;
                            }
                        }
                    }
                } else {
                    pb12 = pb12.add(1);
                    s.add(u9 as usize).write(c);
                    u5a = u4;
                    u9 = u9.wrapping_add(1);
                }
                u4 = u5a;
                c = *pb12;
            }
            if !overflow && u9 < 0x46 {
                s.add(u9 as usize).write(0);
            } else {
                s.add(69).write(0);
            }
            let mut k = (u4 >> 0x10) as u8;
            if 8 < k {
                k = 6;
            }
            p.byte_add(9).write(k);
            let u9 = k as u16;

            if DAT_00a00860 == 0 && (u9 == 2 || u9 == 1) {
                DAT_005235d4 = DAT_005235d4.wrapping_add(1);
                DAT_005235d0 = DAT_005235d0.wrapping_add(1);
                if DAT_005235d0 == 300 {
                    if DAT_009ecc3c != 0 {
                        FUN_00450550();
                    }
                    DAT_005235d0 = 0;
                    DAT_005235d8 = 1;
                }
                DAT_009993fc = FUN_004ac720(DAT_009ec5fc as u32);
                FUN_004ac740();
                let dst =
                    DAT_009993fc.wrapping_add((DAT_005235d0 as u32).wrapping_mul(0x54)) as *mut u32;
                copy_blk(dst, src);
            }
            b2 = DAT_009d77f8;
            if u9 == 3 || u9 == 4 || u9 == 7 {
                if (DAT_009a7b54 != 0 || param_1 == 400)
                    && (DAT_005569f8 == -1 || DAT_00556a01 != 4 || u9 == 7)
                {
                    copy_blk((&raw mut DAT_005569f8).cast::<u32>(), src);
                    FUN_00450c90(u9);
                    b2 = DAT_009d77f8;
                }
            } else {
                if u9 == 8 {
                    copy_blk(&raw mut DAT_00556a50, src);
                    FUN_00450c90(8);
                    return;
                }
                if DAT_009a6ff0 as u16 == 0xffff {
                    copy_blk(&raw mut DAT_009a6ff0, src);
                    FUN_00450c90(u9);
                    return;
                }
                let slot = (&raw mut DAT_009a7044)
                    .cast::<u16>()
                    .add((DAT_009d77f8 as usize).wrapping_mul(0x2a))
                    .cast::<u32>();
                match DAT_009a6ff9 {
                    1 => {
                        if u9 != 2 && u9 != 1 {
                            FUN_00450bc0();
                            copy_blk(&raw mut DAT_009a6ff0, src);
                            FUN_00450c90(u9);
                            return;
                        }
                        copy_blk(slot, src);
                        b2 = DAT_009d77f8.wrapping_add(1);
                        if 9 < b2 {
                            return;
                        }
                    }
                    2 | 5 => {
                        if u9 != 2 {
                            FUN_00450bc0();
                            copy_blk(&raw mut DAT_009a6ff0, src);
                            FUN_00450c90(u9);
                            return;
                        }
                        copy_blk(slot, src);
                        b2 = DAT_009d77f8.wrapping_add(1);
                        if 9 < b2 {
                            return;
                        }
                    }
                    3 | 6 | 7 | 8 => {
                        copy_blk(&raw mut DAT_009a6ff0, src);
                        FUN_00450c90(u9);
                        return;
                    }
                    4 => {
                        if u9 != 3 && u9 != 8 {
                            if u9 != 2 && u9 != 1 {
                                copy_blk(&raw mut DAT_009a6ff0, src);
                                FUN_00450c90(u9);
                                return;
                            }
                            copy_blk(slot, src);
                            b2 = DAT_009d77f8.wrapping_add(1);
                            if 9 < b2 {
                                return;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        DAT_009d77f8 = b2;
    }
}
