// SPDX-License-Identifier: MIT

use core::arch::asm;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa6cfc: i32;
    static mut DAT_00aa7b13: i32;
    static mut DAT_00aa7b82: i32;
    static mut DAT_00aa7b86: i32;
    static mut DAT_00aa7b8a: i32;
    static mut DAT_00aa7b8e: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_005063b0(param_1: i32, param_2: i32, param_3: i32, param_4: u16);
    fn FUN_00506550(param_1: i32, param_2: i32, param_3: i32, param_4: u16);
}

#[inline(always)]
unsafe fn idiv(a: i32, b: i32) -> i32 {
    let q: i32;
    // SAFETY: the machine's own signed divide; faults as the original does.
    unsafe {
        asm!("cdq", "idiv {d:e}", d = in(reg) b, inout("eax") a => q, out("edx") _);
    }
    q
}

unsafe fn run(mut p: *mut u8, cnt: i32, ls: i32, c: u8, b13: i32, neg: bool) {
    let mut cnt = cnt;
    if cnt < 1 {
        return;
    }
    let mut i5: i32 = 0x800;
    let u1 = (ls >> 12) as u32;
    let fill = (c as u32).wrapping_mul(0x01010101);
    // SAFETY: writes the caller's framebuffer where the original does.
    unsafe {
        if u1 == 0 {
            loop {
                i5 = i5.wrapping_add(ls & 0xfff);
                p.write(c);
                if 0xfff < i5 {
                    i5 = i5.wrapping_sub(0x1000);
                    p = p.wrapping_add(1);
                }
                p = p.wrapping_offset(if neg { b13.wrapping_neg() } else { b13 } as isize);
                cnt = cnt.wrapping_sub(1);
                if cnt == 0 {
                    break;
                }
            }
        } else {
            loop {
                i5 = i5.wrapping_add(ls & 0xfff);
                let mut u6 = u1;
                if 0xfff < i5 {
                    u6 = u1.wrapping_add(1);
                    i5 = i5.wrapping_sub(0x1000);
                }
                let mut q = p;
                let mut n = u6 >> 2;
                while n != 0 {
                    q.cast::<u32>().write_unaligned(fill);
                    q = q.wrapping_add(4);
                    n -= 1;
                }
                let mut n = u6 & 3;
                while n != 0 {
                    q.write(c);
                    q = q.wrapping_add(1);
                    n -= 1;
                }
                let step = if neg {
                    u6.wrapping_sub(b13 as u32)
                } else {
                    (b13 as u32).wrapping_add(u6)
                };
                p = p.wrapping_offset(step as i32 as isize);
                cnt = cnt.wrapping_sub(1);
                if cnt == 0 {
                    break;
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00505cd0(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: u16,
) {
    let mut p1 = param_1;
    let mut p2 = param_2;
    let mut p3 = param_3;
    let mut p4 = param_4;
    // SAFETY: the globals and callees are the game's own memory and code.
    unsafe {
        if p4 == p2 {
            FUN_005063b0(p1, p3, p2, param_5);
            return;
        }
        if p3 == p1 {
            FUN_00506550(p2, p4, p1, param_5);
            return;
        }
        let g82 = (&raw mut DAT_00aa7b82).read_unaligned();
        let g86 = (&raw mut DAT_00aa7b86).read_unaligned();
        let g8a = (&raw mut DAT_00aa7b8a).read_unaligned();
        let g8e = (&raw mut DAT_00aa7b8e).read_unaligned();
        let g13 = (&raw mut DAT_00aa7b13).read_unaligned();
        let c = param_5 as u8;
        let base = DAT_00aa6cfc;
        let iv4: i32;
        let mut ls: i32;
        let mut l9: bool;

        if p2 < p4 {
            if p1 < p3 {
                // A
                ls = idiv(
                    p3.wrapping_sub(p1).wrapping_add(1).wrapping_mul(0x1000),
                    p4.wrapping_sub(p2).wrapping_add(1),
                );
                l9 = p1 < g82;
                if l9 {
                    p2 = p2.wrapping_add(idiv(g82.wrapping_sub(p1).wrapping_mul(0x1000), ls));
                    p1 = g82;
                }
                let mut v = p2;
                if p2 < g8a {
                    p1 = p1.wrapping_add(g8a.wrapping_sub(p2).wrapping_mul(ls) >> 12);
                    v = g8a;
                }
                l9 = (p2 < g8a) || l9;
                if g86 < p3 {
                    l9 = true;
                    p4 = p4.wrapping_add(idiv(g86.wrapping_sub(p3).wrapping_mul(0x1000), ls));
                    p3 = g86;
                }
                if g8e < p4 {
                    l9 = true;
                    p3 = p3.wrapping_sub(p4.wrapping_sub(g8e).wrapping_mul(ls) >> 12);
                    p4 = g8e;
                }
                if l9 {
                    if p3 < p1 {
                        return;
                    }
                    if p4 < v {
                        return;
                    }
                    ls = idiv(
                        p3.wrapping_sub(p1).wrapping_add(1).wrapping_mul(0x1000),
                        p4.wrapping_sub(v).wrapping_add(1),
                    );
                }
                iv4 = v;
                let ptr = g13.wrapping_mul(iv4).wrapping_add(base).wrapping_add(p1) as u32 as usize
                    as *mut u8;
                run(ptr, p4.wrapping_sub(iv4).wrapping_add(1), ls, c, g13, false);
                return;
            }
            // B
            ls = idiv(
                p1.wrapping_sub(p3).wrapping_add(1).wrapping_mul(0x1000),
                p4.wrapping_sub(p2).wrapping_add(1),
            );
            l9 = g86 < p1;
            if l9 {
                p2 = p2.wrapping_add(idiv(p1.wrapping_sub(g86).wrapping_mul(0x1000), ls));
                p1 = g86;
            }
            let mut v = p2;
            if p2 < g8a {
                p1 = p1.wrapping_sub(g8a.wrapping_sub(p2).wrapping_mul(ls) >> 12);
                v = g8a;
            }
            l9 = (p2 < g8a) || l9;
            if p3 < g82 {
                l9 = true;
                p4 = p4.wrapping_add(idiv(p3.wrapping_sub(g82).wrapping_mul(0x1000), ls));
                p3 = g82;
            }
            if g8e < p4 {
                l9 = true;
                p3 = p3.wrapping_add(p4.wrapping_sub(g8e).wrapping_mul(ls) >> 12);
                p4 = g8e;
            }
            if l9 {
                if p1 < p3 {
                    return;
                }
                if p4 < v {
                    return;
                }
                ls = idiv(
                    p1.wrapping_sub(p3).wrapping_add(1).wrapping_mul(0x1000),
                    p4.wrapping_sub(v).wrapping_add(1),
                );
            }
            iv4 = v;
            let ptr =
                g13.wrapping_mul(p4).wrapping_add(p3).wrapping_add(base) as u32 as usize as *mut u8;
            run(ptr, p4.wrapping_sub(iv4).wrapping_add(1), ls, c, g13, true);
            return;
        }

        if p4 < p2 && p1 < p3 {
            // C
            ls = idiv(
                p3.wrapping_sub(p1).wrapping_add(1).wrapping_mul(0x1000),
                p2.wrapping_sub(p4).wrapping_add(1),
            );
            l9 = p1 < g82;
            if l9 {
                p2 = p2.wrapping_add(idiv(p1.wrapping_sub(g82).wrapping_mul(0x1000), ls));
                p1 = g82;
            }
            let mut v = p2;
            if g8e < p2 {
                p1 = p1.wrapping_add(p2.wrapping_sub(g8e).wrapping_mul(ls) >> 12);
                v = g8e;
            }
            l9 = (g8e < p2) || l9;
            if g86 < p3 {
                l9 = true;
                p4 = p4.wrapping_add(idiv(p3.wrapping_sub(g86).wrapping_mul(0x1000), ls));
                p3 = g86;
            }
            if p4 < g8a {
                l9 = true;
                p3 = p3.wrapping_sub(g8a.wrapping_sub(p4).wrapping_mul(ls) >> 12);
                p4 = g8a;
            }
            if l9 {
                if p3 < p1 {
                    return;
                }
                if v < p4 {
                    return;
                }
                ls = idiv(
                    p3.wrapping_sub(p1).wrapping_add(1).wrapping_mul(0x1000),
                    v.wrapping_sub(p4).wrapping_add(1),
                );
            }
            iv4 = v;
            let ptr = g13.wrapping_mul(iv4).wrapping_add(base).wrapping_add(p1) as u32 as usize
                as *mut u8;
            run(ptr, iv4.wrapping_sub(p4).wrapping_add(1), ls, c, g13, true);
        } else {
            // D
            ls = idiv(
                p1.wrapping_sub(p3).wrapping_add(1).wrapping_mul(0x1000),
                p2.wrapping_sub(p4).wrapping_add(1),
            );
            l9 = g86 < p1;
            if l9 {
                p2 = p2.wrapping_add(idiv(g86.wrapping_sub(p1).wrapping_mul(0x1000), ls));
                p1 = g86;
            }
            let mut v = p2;
            if g8e < p2 {
                p1 = p1.wrapping_sub(p2.wrapping_sub(g8e).wrapping_mul(ls) >> 12);
                v = g8e;
            }
            l9 = (g8e < p2) || l9;
            if p3 < g82 {
                l9 = true;
                p4 = p4.wrapping_add(idiv(g82.wrapping_sub(p3).wrapping_mul(0x1000), ls));
                p3 = g82;
            }
            if p4 < g8a {
                l9 = true;
                p3 = p3.wrapping_add(g8a.wrapping_sub(p4).wrapping_mul(ls) >> 12);
                p4 = g8a;
            }
            if l9 {
                if p1 < p3 {
                    return;
                }
                if v < p4 {
                    return;
                }
                ls = idiv(
                    p1.wrapping_sub(p3).wrapping_add(1).wrapping_mul(0x1000),
                    v.wrapping_sub(p4).wrapping_add(1),
                );
            }
            iv4 = v;
            let ptr =
                g13.wrapping_mul(p4).wrapping_add(p3).wrapping_add(base) as u32 as usize as *mut u8;
            run(ptr, iv4.wrapping_sub(p4).wrapping_add(1), ls, c, g13, false);
        }
    }
}
