// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0081b850: i32;
    static mut DAT_0081bfba: [u8; 4];
    static mut DAT_0081bcd0: u32;
    static mut DAT_0081b740: [i8; 0];
    static mut s_settingsound_0053199c: [i8; 0];
    static mut s__d__s_0053221c: [i8; 0];
    static mut DAT_0085e78f: u8;
    static mut DAT_0085e790: u8;
    static mut DAT_0085e7b4: u8;
    static mut DAT_0085e7b5: u8;
    static mut DAT_0085e7b6: u8;
    static mut DAT_0085e7b7: u8;
    static mut DAT_0085e7b8: u8;
    static mut DAT_0085e7b9: u8;
    static mut DAT_0085e7ba: u8;
    static mut DAT_0085e7bb: u8;
    static mut DAT_0085e7bc: u8;
    static mut DAT_0085e7bd: u8;
    static mut DAT_0085e7be: u8;
    static mut DAT_0085e7bf: u8;
    static mut DAT_0085e7c4: u8;
    static mut DAT_00758620: u32;
    static mut DAT_00758624: u32;
    static mut DAT_00758628: u32;
    static mut DAT_0075862c: u32;
    static mut DAT_00758630: u32;
    static mut DAT_00758634: u32;
    static mut DAT_00758638: u32;
    static mut DAT_0075863c: u32;
    static mut DAT_00758640: u32;
    static mut DAT_00758644: u32;
    static mut DAT_00758648: u32;
    static mut DAT_0075864c: u32;
    static mut DAT_00758650: u32;
    static mut DAT_00758654: u32;
    static mut DAT_00758658: u32;
    static mut DAT_0075865c: u32;
    static mut DAT_00758660: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dcfa0(param_1: i32) -> *mut i8;
    fn FUN_004da7c0(a: i32, b: *mut i8, c: i32, d: i32, e: u32);
    fn FUN_004cc810() -> i32;
    fn FUN_004db2f0(a: *mut i8, b: i32) -> i32;
    fn FUN_004dcfd0(a: *mut i32, b: i32, c: i32, d: i32, e: i32);
    fn FUN_004d9950(a: *mut i32, b: *mut i8, c: *mut i32, d: u32, e: i32, f: *mut i8) -> i32;
    fn FUN_004daac0(a: i32, b: *mut i8, c: *mut i32, d: i32, e: i32, f: u32);
    fn sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn rand() -> i32;
    fn FUN_004cb830(a: i32, b: *mut u32, c: i32);
}

unsafe fn fba() -> i32 {
    // SAFETY: reads the game's own global.
    unsafe { i32::from_ne_bytes(DAT_0081bfba) }
}

unsafe fn draw(id: i32, x: i32, y: i32) {
    // SAFETY: calls into the game's own functions.
    unsafe {
        let s = FUN_004dcfa0(id);
        FUN_004da7c0(10, s, x, y, 0xffff);
    }
}

unsafe fn measure(id: i32) -> i32 {
    // SAFETY: calls into the game's own functions.
    unsafe {
        let s = FUN_004dcfa0(id);
        FUN_004db2f0(s, 10)
    }
}

unsafe fn std_rect(r: *mut i32, id: i32, x: i32, y: i32) {
    // SAFETY: calls into the game's own functions; r points at a local rect.
    unsafe {
        draw(id, x, y);
        let n = measure(id);
        FUN_004dcfd0(
            r,
            n.wrapping_add(x).wrapping_add(2),
            y,
            x.wrapping_add(0xa5),
            y.wrapping_add(0xd),
        );
    }
}

unsafe fn opt_ptr(r: *mut i32, b2: u8, p: *mut i8, idx: i32) -> i32 {
    // SAFETY: calls into the game's own functions and globals.
    unsafe {
        if b2 != 0 {
            FUN_004d9950(
                r,
                p,
                10usize as *mut i32,
                0xffff,
                idx,
                (&raw mut s_settingsound_0053199c).cast::<i8>(),
            )
        } else {
            FUN_004daac0(10, p, r, 0, 1, DAT_0081bcd0);
            0
        }
    }
}

unsafe fn opt(r: *mut i32, b2: u8, strid: i32, idx: i32) -> i32 {
    // SAFETY: calls into the game's own functions.
    unsafe {
        let p = FUN_004dcfa0(strid);
        opt_ptr(r, b2, p, idx)
    }
}

unsafe fn copy_buf(src: *const i8) {
    // SAFETY: src is a NUL-terminated game string; the buffer is the game's own.
    unsafe {
        let dst = (&raw mut DAT_0081b740).cast::<i8>();
        let mut i = 0usize;
        loop {
            let c = *src.add(i);
            *dst.add(i) = c;
            if c == 0 {
                break;
            }
            i += 1;
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004e6960() -> u32 {
    // SAFETY: the callees and globals are the game's own; the rect pointer
    // refers to a local array.
    unsafe {
        let mut local_10: [i32; 4] = [0; 4];
        let r = local_10.as_mut_ptr();
        let mut i5: i32 = 0x58;
        let mut i6: i32;
        let mut i9: i32;
        let mut local_1c: i32;
        let mut b7: u8;
        let b2: u8;

        draw(0x284, 0x58, 0x137);
        i6 = 0x14a;
        local_1c = 0;
        b7 = 0;
        i9 = FUN_004cc810();
        if i9 == 0 && DAT_0081b850 != 2 {
            b2 = 0;
        } else {
            b2 = 1;
        }
        let go_aaf: bool;
        if DAT_0081b850 == 2 {
            go_aaf = true;
        } else {
            if b2 != 0 {
                draw(0x19d, 0x58, 0x14a);
                let n = measure(0x19d);
                FUN_004dcfd0(r, n.wrapping_add(0x5a), 0x14a, 0xfd, 0x157);
                let t = opt(r, b2, (DAT_0085e7b9 as i32).wrapping_add(0x1aa), 0x15);
                b7 = (t != 0) as u8;
                if b7 != 0 {
                    DAT_0085e7b9 ^= 1;
                }
                local_1c = 1;
                i6 = 0x158;
            }
            let f = fba();
            go_aaf = DAT_0081b850 == 2 || (f != 3 && f != 4);
        }
        if go_aaf {
            draw(0x198, 0x58, i6);
            let n = measure(0x198);
            FUN_004dcfd0(r, n.wrapping_add(0x5a), i6, 0xfd, i6.wrapping_add(0xd));
            let t = opt(r, b2, (DAT_0085e7b4 as i32).wrapping_add(0x1a4), 0x14);
            if t != 0 {
                b7 = 1;
                DAT_0085e7b4 = DAT_0085e7b4.wrapping_add(1);
                if 2 < DAT_0085e7b4 {
                    DAT_0085e7b4 = 0;
                }
            }
            i6 = i6.wrapping_add(0xe);
            local_1c = local_1c.wrapping_add(1);
            if local_1c == 6 {
                i5 = 0x106;
                i6 = 0x14a;
            }
        }
        if DAT_0081b850 != 2 {
            std_rect(r, 0x1a2, i5, i6);
            if DAT_0085e7be == 0 {
                let p = FUN_004dcfa0(0x1b0);
                copy_buf(p);
            } else if DAT_0085e7be == 0xff {
                let p = FUN_004dcfa0(0x1b1);
                copy_buf(p);
            } else {
                let p = FUN_004dcfa0(0x1b3);
                sprintf(
                    (&raw mut DAT_0081b740).cast::<i8>(),
                    (&raw const s__d__s_0053221c).cast::<i8>(),
                    DAT_0085e7be as u32,
                    p,
                );
            }
            let t = opt_ptr(r, b2, (&raw mut DAT_0081b740).cast::<i8>(), 0x16);
            if t == 1 {
                b7 = 1;
                if DAT_0085e7be == 0 {
                    DAT_0085e7be = 0xff;
                } else if DAT_0085e7be == 0xff {
                    DAT_0085e7be = 1;
                } else {
                    DAT_0085e7be = DAT_0085e7be.wrapping_add(1);
                    if 0x14 < DAT_0085e7be {
                        DAT_0085e7be = 0;
                    }
                }
            }
            if t == 2 {
                b7 = 1;
                if DAT_0085e7be == 0xff {
                    DAT_0085e7be = 0;
                } else if DAT_0085e7be == 0 {
                    DAT_0085e7be = 0x14;
                } else {
                    DAT_0085e7be = DAT_0085e7be.wrapping_sub(1);
                    if DAT_0085e7be == 0 {
                        DAT_0085e7be = 0xff;
                    }
                }
            }
            i6 = i6.wrapping_add(0xe);
            if local_1c == 5 {
                i5 = 0x106;
                i6 = 0x14a;
            }
            std_rect(r, 0x1a3, i5, i6);
            if DAT_0085e7bf == 0 {
                let p = FUN_004dcfa0(0x1b0);
                copy_buf(p);
            } else {
                let p = FUN_004dcfa0(0x1b3);
                sprintf(
                    (&raw mut DAT_0081b740).cast::<i8>(),
                    (&raw const s__d__s_0053221c).cast::<i8>(),
                    DAT_0085e7bf as u32,
                    p,
                );
            }
            let t = opt_ptr(r, b2, (&raw mut DAT_0081b740).cast::<i8>(), 0x17);
            if t == 1 {
                b7 = 1;
                DAT_0085e7bf = DAT_0085e7bf.wrapping_add(1);
                if 10 < DAT_0085e7bf {
                    DAT_0085e7bf = 0;
                }
            }
            if t == 2 {
                b7 = 1;
                if DAT_0085e7bf == 0 {
                    DAT_0085e7bf = 10;
                } else {
                    DAT_0085e7bf = DAT_0085e7bf.wrapping_sub(1);
                }
            }
            i6 = i6.wrapping_add(0xe);
            local_1c = local_1c.wrapping_add(2);
            if local_1c == 6 {
                i5 = 0x106;
                i6 = 0x14a;
            }
        }
        let id: i32;
        if fba() == 4 {
            std_rect(r, 0x19c, i5, i6);
            let t = opt(r, b2, (DAT_0085e7b8 as i32).wrapping_add(0x1a7), 0x1a);
            if t != 0 {
                b7 = 1;
                DAT_0085e7b8 = DAT_0085e7b8.wrapping_add(1);
                if 2 < DAT_0085e7b8 {
                    DAT_0085e7b8 = 0;
                }
            }
            i6 = i6.wrapping_add(0xe);
            local_1c = local_1c.wrapping_add(1);
            if local_1c == 6 {
                i5 = 0x106;
                i6 = 0x14a;
            }
            if fba() != 4 {
                draw(0x19b, i5, i6);
                id = 0x19b;
            } else {
                draw(0x2d2, i5, i6);
                id = 0x2d2;
            }
        } else {
            draw(0x19b, i5, i6);
            id = 0x19b;
        }
        let n = measure(id);
        FUN_004dcfd0(
            r,
            n.wrapping_add(i5).wrapping_add(2),
            i6,
            i5.wrapping_add(0xa5),
            i6.wrapping_add(0xd),
        );
        let t = opt(r, b2, (DAT_0085e7b7 as i32).wrapping_add(0xdf), 0x18);
        if t != 0 {
            b7 = 1;
            DAT_0085e7b7 ^= 1;
        }
        i6 = i6.wrapping_add(0xe);
        i9 = local_1c.wrapping_add(1);
        if i9 == 6 {
            i5 = 0x106;
            i6 = 0x14a;
        }
        if DAT_0081b850 != 2 {
            let f = fba();
            if f == 1 || f == 2 {
                std_rect(r, 0x22e, i5, i6);
                let t = opt(r, b2, (DAT_0085e7c4 as i32).wrapping_add(0xdf), 0x19);
                if t != 0 {
                    b7 = 1;
                    DAT_0085e7c4 ^= 1;
                }
                i6 = i6.wrapping_add(0xe);
                i9 = local_1c.wrapping_add(2);
                if i9 == 6 {
                    i5 = 0x106;
                    i6 = 0x14a;
                }
            }
            local_1c = i9;
            if DAT_0081b850 != 2 {
                std_rect(r, 0x19f, i5, i6);
                let t = opt(r, b2, (DAT_0085e7bb as i32).wrapping_add(0x20a), 0x1b);
                if t != 0 {
                    b7 = 1;
                    DAT_0085e7bb = DAT_0085e7bb.wrapping_add(1);
                    if 2 < DAT_0085e7bb {
                        DAT_0085e7bb = 0;
                    }
                }
                let f = fba();
                if DAT_0085e7bb == 2 && f != 1 && f != 2 {
                    DAT_0085e7bb = 0;
                }
                i6 = i6.wrapping_add(0xe);
                i9 = local_1c.wrapping_add(1);
                if i9 == 6 {
                    i5 = 0x106;
                    i6 = 0x14a;
                }
                local_1c = i9;
                if DAT_0081b850 != 2 {
                    std_rect(r, 0x1a0, i5, i6);
                    let t = opt(r, b2, (DAT_0085e7bc as i32).wrapping_add(0x1ae), 0x1c);
                    if t != 0 {
                        b7 = 1;
                        DAT_0085e7bc ^= 1;
                    }
                    i6 = i6.wrapping_add(0xe);
                    local_1c = local_1c.wrapping_add(1);
                    if local_1c == 6 {
                        i5 = 0x106;
                        i6 = 0x14a;
                    }
                }
            }
        } else {
            local_1c = i9;
        }
        std_rect(r, 0x1a1, i5, i6);
        let t = opt(r, b2, (DAT_0085e7bd as i32).wrapping_add(0x1b0), 0x1d);
        if t != 0 {
            b7 = 1;
            DAT_0085e7bd = DAT_0085e7bd.wrapping_add(1);
            if 2 < DAT_0085e7bd {
                DAT_0085e7bd = 0;
            }
            let f = fba();
            if DAT_0081b850 != 2 && (f == 3 || f == 4) && DAT_0085e7bd == 2 {
                DAT_0085e7bd = 0;
            }
        }
        i6 = i6.wrapping_add(0xe);
        if local_1c == 5 {
            i5 = 0x106;
            i6 = 0x14a;
        }
        std_rect(r, 0x199, i5, i6);
        let t = opt(r, b2, (DAT_0085e7b5 as i32).wrapping_add(0xdf), 0x1f);
        if t != 0 {
            b7 = 1;
            DAT_0085e7b5 ^= 1;
        }
        if DAT_0081b850 != 2 && b7 != 0 {
            DAT_00758620 = 0x59;
            DAT_00758624 = DAT_0085e7b4 as u32;
            DAT_00758628 = DAT_0085e7b5 as u32;
            DAT_0075862c = DAT_0085e7b6 as u32;
            DAT_00758630 = DAT_0085e7b7 as u32;
            DAT_00758634 = DAT_0085e7b8 as u32;
            DAT_00758638 = DAT_0085e7b9 as u32;
            DAT_0075863c = DAT_0085e7ba as u32;
            DAT_00758640 = DAT_0085e7bb as u32;
            DAT_00758644 = DAT_0085e7bc as u32;
            DAT_00758648 = DAT_0085e7bd as u32;
            DAT_0075864c = DAT_0085e7be as u32;
            DAT_00758650 = DAT_0085e7bf as u32;
            DAT_00758654 = rand() as u32;
            DAT_00758658 = DAT_0085e78f as u32;
            DAT_0075865c = DAT_0085e7c4 as u32;
            DAT_00758660 = DAT_0085e790 as u32;
            FUN_004cb830(0, &raw mut DAT_00758620, 0x44);
        }
        1
    }
}
