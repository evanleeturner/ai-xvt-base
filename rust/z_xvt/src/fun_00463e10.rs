// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_005f10f8: u16;
    static mut DAT_005f1110: u32;
    static mut DAT_005f10e8: u16;
    static mut DAT_005f1d28: u32;
    static mut DAT_005f10ec: u16;
    static mut DAT_005f10f4: u16;
    static mut DAT_005f1d20: *mut u32;
    static mut DAT_005f1d24: *mut i32;
    static mut DAT_0052552c: u32;
    static mut DAT_00525534: u32;
    static mut DAT_0052550c: u32;
    static mut DAT_0070acc8: u32;
    static mut DAT_00732e50: u8;
}

unsafe fn umod(n: u32, d: u32) -> u32 {
    let r: u32;
    // SAFETY: the machine's own unsigned divide; a zero divisor faults as in the original.
    unsafe {
        core::arch::asm!(
            "div {d:e}",
            d = in(reg) d,
            inout("eax") n => _,
            inout("edx") 0u32 => r,
            options(nomem, nostack)
        );
    }
    r
}

unsafe fn row() -> u32 {
    // SAFETY: the game's table of row offsets, indexed by the game's counter.
    unsafe {
        let a = (DAT_005f1d20 as usize).wrapping_add(DAT_005f1110.wrapping_mul(4) as usize);
        (a as *const u32).read_unaligned()
    }
}

unsafe fn adj(v: u32) -> u32 {
    // SAFETY: reads the game's globals.
    unsafe {
        if DAT_0052552c != 0x13 && DAT_00525534 == 0xa0000 {
            umod(v, DAT_0052550c)
        } else {
            v
        }
    }
}

unsafe fn span() -> i16 {
    // SAFETY: reads the game's globals.
    unsafe { (DAT_005f10ec as u32).wrapping_sub(DAT_005f10f4 as u32) as i16 }
}

unsafe fn fill(mut p: *mut u8, mut n: i16) -> *mut u8 {
    while n != 0 {
        // SAFETY: writes into the game's frame buffer where the original does.
        unsafe {
            p.write(DAT_00732e50);
        }
        p = p.wrapping_add(1);
        n = n.wrapping_sub(1);
    }
    p
}

unsafe fn strip(mut p: *mut u8, s: i16, count: u32, p1: u16) {
    // SAFETY: writes into the game's frame buffer where the original does.
    unsafe {
        let mut iv = count.wrapping_sub(1) as i32;
        if p1 != 0 {
            loop {
                p.write(DAT_00732e50);
                p = p.wrapping_add(1);
                let b = iv != 0;
                iv = iv.wrapping_sub(1);
                if !b {
                    break;
                }
            }
        }
        let off = ((s as i32 as u32).wrapping_add((p1 as u32).wrapping_mul(0xFFFFFFFE))) as i16;
        p = p.wrapping_offset(off as isize);
        iv = count.wrapping_sub(1) as i32;
        if p1 != 0 {
            loop {
                p.write(DAT_00732e50);
                let b = iv != 0;
                p = p.wrapping_add(1);
                iv = iv.wrapping_sub(1);
                if !b {
                    break;
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00463e10(param_1: u16) {
    // SAFETY: all globals are the game's own memory; the frame buffer
    // pointers are written exactly as the original does.
    unsafe {
        DAT_005f1110 = DAT_005f10f8 as u32;
        DAT_005f1d28 = (DAT_005f10e8 as u32).wrapping_sub(DAT_005f1110);
        if span() < 1 {
            return;
        }
        let mut u3 = row();
        let b8: bool;
        if DAT_0052552c == 0x13 {
            b8 = DAT_00525534 != 0xa0000;
        } else if DAT_00525534 == 0xa0000 {
            u3 = umod(u3, DAT_0052550c);
            b8 = 0xffffu32 < DAT_0070acc8.wrapping_mul(DAT_005f1d28).wrapping_add(u3);
        } else {
            b8 = true;
        }

        if b8 {
            if param_1 == 0 {
                while DAT_005f1d28 != 0 {
                    let mut u3 = (DAT_005f10f4 as u32).wrapping_add(row());
                    u3 = adj(u3);
                    let s = span();
                    let p = u3.wrapping_add(DAT_00525534) as usize as *mut u8;
                    if s < 1 {
                        return;
                    }
                    fill(p, s);
                    DAT_005f1110 = DAT_005f1110.wrapping_add(1);
                    DAT_005f1d28 = DAT_005f1d28.wrapping_sub(1);
                }
            } else {
                let mut u3: u32 = 0;
                loop {
                    let mut u6 = row().wrapping_add(DAT_005f10f4 as u32);
                    u6 = adj(u6);
                    let s = span();
                    let p = u6.wrapping_add(DAT_00525534) as usize as *mut u8;
                    if s < 1 {
                        return;
                    }
                    fill(p, s);
                    u3 = u3.wrapping_add(1);
                    DAT_005f1d28 = DAT_005f1d28.wrapping_sub(1);
                    DAT_005f1110 = DAT_005f1110.wrapping_add(1);
                    if !(u3 < param_1 as u32) {
                        break;
                    }
                }
                let u3 = param_1 as u32;
                while u3 < DAT_005f1d28 {
                    let mut u6 = (DAT_005f10f4 as u32).wrapping_add(row());
                    u6 = adj(u6);
                    let s = span();
                    let p = u6.wrapping_add(DAT_00525534) as usize as *mut u8;
                    strip(p, s, u3, param_1);
                    DAT_005f1110 = DAT_005f1110.wrapping_add(1);
                    DAT_005f1d28 = DAT_005f1d28.wrapping_sub(1);
                }
                let mut u6: u32 = 0;
                if param_1 != 0 {
                    loop {
                        let mut u4 = (DAT_005f10f4 as u32).wrapping_add(row());
                        u4 = adj(u4);
                        let s = span();
                        let p = u4.wrapping_add(DAT_00525534) as usize as *mut u8;
                        if s < 1 {
                            break;
                        }
                        fill(p, s);
                        u6 = u6.wrapping_add(1);
                        DAT_005f1d28 = DAT_005f1d28.wrapping_sub(1);
                        DAT_005f1110 = DAT_005f1110.wrapping_add(1);
                        if u3 <= u6 {
                            return;
                        }
                    }
                }
            }
        } else if param_1 == 0 {
            let mut i1 = u3.wrapping_add(DAT_00525534) as i32;
            if DAT_005f1d28 != 0 {
                loop {
                    let s = span();
                    let p = (DAT_005f10f4 as u32).wrapping_add(i1 as u32) as usize as *mut u8;
                    if s < 1 {
                        break;
                    }
                    fill(p, s);
                    DAT_005f1d28 = DAT_005f1d28.wrapping_sub(1);
                    DAT_005f1110 = DAT_005f1110.wrapping_add(1);
                    i1 = (i1 as u32).wrapping_add(DAT_005f1d24.read_unaligned() as u32) as i32;
                    if DAT_005f1d28 == 0 {
                        return;
                    }
                }
            }
        } else {
            let mut i1 = u3.wrapping_add(DAT_00525534) as i32;
            let mut u3: u32 = 0;
            loop {
                let s = span();
                let p = (DAT_005f10f4 as u32).wrapping_add(i1 as u32) as usize as *mut u8;
                if s < 1 {
                    return;
                }
                fill(p, s);
                u3 = u3.wrapping_add(1);
                DAT_005f1d28 = DAT_005f1d28.wrapping_sub(1);
                i1 = (i1 as u32).wrapping_add(DAT_005f1d24.read_unaligned() as u32) as i32;
                DAT_005f1110 = DAT_005f1110.wrapping_add(1);
                if !(u3 < param_1 as u32) {
                    break;
                }
            }
            let u3 = param_1 as u32;
            while u3 < DAT_005f1d28 {
                let s = span();
                let p = (DAT_005f10f4 as u32).wrapping_add(i1 as u32) as usize as *mut u8;
                strip(p, s, u3, param_1);
                DAT_005f1110 = DAT_005f1110.wrapping_add(1);
                i1 = (i1 as u32).wrapping_add(DAT_005f1d24.read_unaligned() as u32) as i32;
                DAT_005f1d28 = DAT_005f1d28.wrapping_sub(1);
            }
            let mut u6: u32 = 0;
            if param_1 != 0 {
                loop {
                    let s = span();
                    let p = (DAT_005f10f4 as u32).wrapping_add(i1 as u32) as usize as *mut u8;
                    if s < 1 {
                        break;
                    }
                    fill(p, s);
                    u6 = u6.wrapping_add(1);
                    DAT_005f1d28 = DAT_005f1d28.wrapping_sub(1);
                    i1 = (i1 as u32).wrapping_add(DAT_005f1d24.read_unaligned() as u32) as i32;
                    DAT_005f1110 = DAT_005f1110.wrapping_add(1);
                    if u3 <= u6 {
                        return;
                    }
                }
            }
        }
    }
}
