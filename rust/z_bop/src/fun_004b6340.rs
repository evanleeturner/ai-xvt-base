// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa6cfc: i32;
    static mut DAT_00aa7b13: u8;
}

unsafe fn fill(addr: i32, n: u32, v: i32) {
    let p = addr as usize as *mut u16;
    let mut k: u32 = 0;
    while k < n {
        // SAFETY: the destination is the game's surface memory, written as the original does.
        unsafe { p.add(k as usize).write_unaligned(v as u16) };
        k += 1;
    }
}

unsafe fn w16(addr: i32, v: i32) {
    // SAFETY: the destination is the game's surface memory.
    unsafe { (addr as usize as *mut u16).write_unaligned(v as u16) };
}

unsafe fn rb(base: *const u8, off: i32) -> u8 {
    // SAFETY: the run data is the game's own sprite data.
    unsafe { base.byte_offset(off as isize).read() }
}

unsafe fn pal(p1: *mut i32, idx: u8) -> i32 {
    // SAFETY: param_1 points at the game's palette/sprite header.
    unsafe { p1.add(idx as usize + 5).read() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b6340(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
    param_7: i32,
) {
    let mut param_5 = param_5;
    // SAFETY: the globals and pointers are the game's own memory, accessed as the original does.
    unsafe {
        let dat: i32 = (&raw const DAT_00aa7b13).cast::<i32>().read_unaligned();
        let step: i32 = ((dat as u32) & 0xffff_fffe) as i32;

        if param_1.read() == param_6 {
            let mut pb10 = param_1.add(4).read() as usize as *const u8;
            let mut local_c: i32 = dat
                .wrapping_mul(param_3)
                .wrapping_add(param_2.wrapping_mul(2))
                .wrapping_add(DAT_00aa6cfc);
            if 0 < param_5 {
                loop {
                    let o = pb10.cast::<i32>().read_unaligned();
                    pb10 = pb10.byte_offset(o as isize);
                    param_5 -= 1;
                    if param_5 == 0 {
                        break;
                    }
                }
            }
            if 0 < param_7 {
                let mut local_5c = param_7;
                loop {
                    let mut i12: i32 = 0;
                    let mut pb11 = pb10.add(4);
                    loop {
                        let b4 = pb11.read();
                        pb10 = pb11.add(1);
                        if b4 == 0x80 {
                            break;
                        }
                        if b4 & 0x80 == 0 {
                            if b4 & 0x40 == 0 {
                                pb11 = pb11.add(2);
                                let u9 = b4 as u32;
                                let i8 = pal(param_1, pb10.read());
                                fill(local_c.wrapping_add(i12.wrapping_mul(2)), u9, i8);
                                i12 = i12.wrapping_add(u9 as i32);
                            } else {
                                let u9 = (b4 & 0x3f) as u32;
                                let i8 = param_1.add(5).read();
                                fill(local_c.wrapping_add(i12.wrapping_mul(2)), u9, i8);
                                i12 = i12.wrapping_add(u9 as i32);
                                pb11 = pb10;
                            }
                        } else {
                            let b4 = b4 & 0x7f;
                            let mut i8: i32 = 0;
                            if b4 != 0 {
                                loop {
                                    let i14 = i12.wrapping_add(i8);
                                    let p = pb10.add(i8 as usize);
                                    i8 += 1;
                                    w16(
                                        local_c.wrapping_add(i14.wrapping_mul(2)),
                                        pal(param_1, p.read()),
                                    );
                                    if i8 >= b4 as i32 {
                                        break;
                                    }
                                }
                            }
                            i12 = i12.wrapping_add(b4 as i32);
                            pb11 = pb10.add(b4 as usize);
                        }
                    }
                    local_c = local_c.wrapping_add(step);
                    local_5c -= 1;
                    if local_5c == 0 {
                        break;
                    }
                }
                return;
            }
        } else {
            let mut local_10 = param_1.add(4).read() as usize as *const u8;
            let mut local_c: i32 = dat
                .wrapping_mul(param_3)
                .wrapping_add(param_2.wrapping_mul(2))
                .wrapping_add(DAT_00aa6cfc);
            if 0 < param_5 {
                loop {
                    let o = local_10.cast::<i32>().read_unaligned();
                    local_10 = local_10.byte_offset(o as isize);
                    param_5 -= 1;
                    if param_5 == 0 {
                        break;
                    }
                }
            }
            if 0 < param_7 {
                let mut local_28 = param_7;
                loop {
                    let mut b_var1 = false;
                    let i8 = local_10.cast::<i32>().read_unaligned();
                    let mut i12: i32 = 0;
                    let mut i14: i32 = 4;
                    let mut i13: i32;
                    let mut i3: i32;
                    let mut b4: u8;
                    'done: {
                        loop {
                            // loop A
                            loop {
                                // loop B
                                loop {
                                    // loop C
                                    loop {
                                        i13 = i14.wrapping_add(1);
                                        b4 = rb(local_10, i14);
                                        i3 = i12;
                                        if !b_var1 {
                                            if b4 == 0x80 {
                                                break 'done;
                                            }
                                            if b4 & 0x80 == 0 {
                                                if b4 & 0x40 == 0 {
                                                    i3 = (b4 as i32).wrapping_add(i12);
                                                    if i3 < param_4 {
                                                        i13 = i14.wrapping_add(2);
                                                    } else {
                                                        b_var1 = true;
                                                        i3 = 0;
                                                        b4 = b4.wrapping_sub(
                                                            (param_4 as i8).wrapping_sub(i12 as i8)
                                                                as u8,
                                                        );
                                                        if b4 == 0 {
                                                            i13 = i14.wrapping_add(2);
                                                        }
                                                    }
                                                } else {
                                                    b4 &= 0x3f;
                                                    i3 = (b4 as i32).wrapping_add(i12);
                                                    if param_4 <= i3 {
                                                        b_var1 = true;
                                                        i3 = 0;
                                                        let b5 = b4.wrapping_sub(
                                                            (param_4 as i8).wrapping_sub(i12 as i8)
                                                                as u8,
                                                        );
                                                        b4 = 0;
                                                        if b5 != 0 {
                                                            b4 = b5 | 0x40;
                                                        }
                                                    }
                                                }
                                            } else {
                                                b4 &= 0x7f;
                                                i3 = i12.wrapping_add(b4 as i32);
                                                if i3 < param_4 {
                                                    i13 = i13.wrapping_add(b4 as i32);
                                                } else {
                                                    b_var1 = true;
                                                    i3 = 0;
                                                    i13 =
                                                        i13.wrapping_add(param_4.wrapping_sub(i12));
                                                    let b5 =
                                                        b4.wrapping_sub(param_4.wrapping_sub(i12)
                                                            as i8
                                                            as u8);
                                                    b4 = 0;
                                                    if b5 != 0 {
                                                        b4 = b5 | 0x80;
                                                    }
                                                }
                                            }
                                        }
                                        i12 = i3;
                                        i14 = i13;
                                        if !(!b_var1 || b4 == 0) {
                                            break;
                                        }
                                    }
                                    if b4 == 0x80 {
                                        break 'done;
                                    }
                                    let c2 = i3 as i8;
                                    let c6 = param_6 as i8;
                                    let cnt = c6.wrapping_sub(c2) as u8;
                                    if b4 & 0x80 == 0 {
                                        break;
                                    }
                                    let u7 = (b4 & 0x7f) as u32;
                                    if param_6 <= (u7 as i32).wrapping_add(i3) {
                                        if cnt != 0 {
                                            let mut k: i32 = 0;
                                            loop {
                                                let nk = k + 1;
                                                w16(
                                                    local_c.wrapping_add(
                                                        i3.wrapping_add(k).wrapping_mul(2),
                                                    ),
                                                    pal(param_1, rb(local_10, i13.wrapping_add(k))),
                                                );
                                                k = nk;
                                                if nk >= cnt as i32 {
                                                    break;
                                                }
                                            }
                                        }
                                        break 'done;
                                    }
                                    let mut k: i32 = 0;
                                    if (b4 & 0x7f) != 0 {
                                        loop {
                                            let nk = k + 1;
                                            w16(
                                                local_c.wrapping_add(
                                                    i3.wrapping_add(k).wrapping_mul(2),
                                                ),
                                                pal(param_1, rb(local_10, i13.wrapping_add(k))),
                                            );
                                            k = nk;
                                            if nk >= u7 as i32 {
                                                break;
                                            }
                                        }
                                    }
                                    i12 = i3.wrapping_add(u7 as i32);
                                    i14 = i13.wrapping_add(u7 as i32);
                                }
                                // after C: b4 & 0x80 == 0
                                if b4 & 0x40 != 0 {
                                    break;
                                }
                                let c2 = i3 as i8;
                                let c6 = param_6 as i8;
                                i14 = i13.wrapping_add(1);
                                let u7 = b4 as u32;
                                i12 = i3.wrapping_add(u7 as i32);
                                if param_6 <= i12 {
                                    let n = c6.wrapping_sub(c2) as u8 as u32;
                                    let v = pal(param_1, rb(local_10, i13));
                                    fill(local_c.wrapping_add(i3.wrapping_mul(2)), n, v);
                                    break 'done;
                                }
                                let v = pal(param_1, rb(local_10, i13));
                                fill(local_c.wrapping_add(i3.wrapping_mul(2)), u7, v);
                            }
                            // after B: 0x40 run
                            let u7 = (b4 & 0x3f) as u32;
                            i12 = (u7 as i32).wrapping_add(i3);
                            if param_6 <= i12 {
                                break;
                            }
                            let v = param_1.add(5).read();
                            fill(local_c.wrapping_add(i3.wrapping_mul(2)), u7, v);
                        }
                        // after A
                        let c2 = i3 as i8;
                        let c6 = param_6 as i8;
                        let n = c6.wrapping_sub(c2) as u8 as u32;
                        let v = param_1.add(5).read();
                        fill(local_c.wrapping_add(i3.wrapping_mul(2)), n, v);
                    }
                    // LAB_004b6716
                    local_10 = local_10.byte_offset(i8 as isize);
                    local_c = local_c.wrapping_add(step);
                    local_28 -= 1;
                    if local_28 == 0 {
                        break;
                    }
                }
            }
        }
    }
}
