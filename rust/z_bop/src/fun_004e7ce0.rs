// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b69e30: i32;
    static mut DAT_00b6a5aa: [u8; 4];
    static mut DAT_00aa6114: i32;
    static mut DAT_00aa6134: i32;
    static mut DAT_00aa6120: i32;
    static mut DAT_00aa6124: i32;
    static mut DAT_00b6a2c0: u32;
    static mut DAT_00b69d20: [i8; 0];

    static mut DAT_00bb0106: [u8; 0];
    static mut DAT_00bb10a6: [u8; 0];
    static mut DAT_00bb2046: [u8; 0];
    static mut DAT_00bb260e: [u8; 0];
    static mut DAT_00bb242e: [u8; 0];
    static mut DAT_00bb010a: [u8; 0];
    static mut DAT_00bb010e: [u8; 0];
    static mut DAT_00bb10aa: [u8; 0];
    static mut DAT_00bb10ae: [u8; 0];
    static mut DAT_00bb204a: [u8; 0];
    static mut DAT_00bb2612: [u8; 0];
    static mut DAT_00bb2432: [u8; 0];

    static mut DAT_00bb736f: u8;
    static mut DAT_00bb7370: u8;
    static mut DAT_00bb7394: u8;
    static mut DAT_00bb7395: u8;
    static mut DAT_00bb7396: u8;
    static mut DAT_00bb7397: u8;
    static mut DAT_00bb7398: u8;
    static mut DAT_00bb7399: u8;
    static mut DAT_00bb739a: u8;
    static mut DAT_00bb739b: u8;
    static mut DAT_00bb739c: u8;
    static mut DAT_00bb739d: u8;
    static mut DAT_00bb739e: u8;
    static mut DAT_00bb739f: u8;
    static mut DAT_00bb73a4: u8;
    static mut DAT_00bb73a5: u8;
    static mut DAT_00bb73a6: u8;

    static mut s_settingsound_0052ad80: [i8; 0];
    static mut s__d__s_0052b4c8: [i8; 0];

    static mut DAT_00aa6af0: u32;
    static mut DAT_00aa6af4: u32;
    static mut DAT_00aa6af8: u32;
    static mut DAT_00aa6afc: u32;
    static mut DAT_00aa6b00: u32;
    static mut DAT_00aa6b04: u32;
    static mut DAT_00aa6b08: u32;
    static mut DAT_00aa6b0c: u32;
    static mut DAT_00aa6b10: u32;
    static mut DAT_00aa6b14: u32;
    static mut DAT_00aa6b18: u32;
    static mut DAT_00aa6b1c: u32;
    static mut DAT_00aa6b20: u32;
    static mut DAT_00aa6b24: u32;
    static mut DAT_00aa6b28: u32;
    static mut DAT_00aa6b2c: u32;
    static mut DAT_00aa6b30: u32;
    static mut DAT_00aa6b34: u32;
    static mut DAT_00aa6b38: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ddc10(a: u32) -> *mut i8;
    fn FUN_004db4e0(a: i32, b: *mut i8, c: i32, d: i32, e: u32);
    fn FUN_004cfee0() -> i32;
    fn FUN_004dbfa0(a: *mut i8, b: i32) -> i32;
    fn FUN_004d6460(a: *mut i32, b: i32, c: i32, d: i32, e: i32);
    fn FUN_004da650(a: *mut i32, b: *mut i8, c: i32, d: u32, e: i32, f: *mut i8) -> i32;
    fn FUN_004db7d0(a: i32, b: *mut i8, c: *mut i32, d: i32, e: i32, f: u32);
    fn _sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn _rand() -> i32;
    fn FUN_004cef70(a: i32, b: *mut u32, c: i32);
}

unsafe fn a5aa() -> i32 {
    // SAFETY: reads the game's own unaligned 4-byte global.
    unsafe { (&raw const DAT_00b6a5aa).cast::<i32>().read_unaligned() }
}

unsafe fn sel() -> i32 {
    // SAFETY: the address is computed from the game's globals exactly as the original does.
    unsafe {
        let addr = (DAT_00aa6114 as u32)
            .wrapping_add(0x140)
            .wrapping_add((DAT_00aa6134 as u32).wrapping_mul(0x148));
        (addr as usize as *const i32).read_unaligned()
    }
}

unsafe fn rd4(base: *const u8, off: u32) -> i32 {
    // SAFETY: reads the game's own table at the original's offset.
    unsafe {
        base.wrapping_byte_add(off as usize)
            .cast::<i32>()
            .read_unaligned()
    }
}

unsafe fn rd1(base: *const u8, off: u32) -> u8 {
    // SAFETY: reads the game's own table at the original's offset.
    unsafe { base.wrapping_byte_add(off as usize).read() }
}

unsafe fn copy_to_buf(src: *const i8) {
    // SAFETY: src is a NUL-terminated string from the game; the buffer is the game's own.
    unsafe {
        let dst = (&raw mut DAT_00b69d20).cast::<i8>();
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

unsafe fn label(idx: u32, x: i32, y: i32, arr: *mut i32) {
    // SAFETY: calls into the game's own drawing functions.
    unsafe {
        let p = FUN_004ddc10(idx);
        FUN_004db4e0(10, p, x, y, 0xffff);
        let p2 = FUN_004ddc10(idx);
        let w = FUN_004dbfa0(p2, 10);
        FUN_004d6460(
            arr,
            w.wrapping_add(x).wrapping_add(2),
            y,
            x.wrapping_add(0xa5),
            y.wrapping_add(0xd),
        );
    }
}

unsafe fn btn(idx: u32, arr: *mut i32, e: i32) -> i32 {
    // SAFETY: calls into the game's own functions.
    unsafe {
        let p = FUN_004ddc10(idx);
        FUN_004da650(
            arr,
            p,
            10,
            0xffff,
            e,
            (&raw mut s_settingsound_0052ad80).cast::<i8>(),
        )
    }
}

unsafe fn stat(idx: u32, arr: *mut i32) {
    // SAFETY: calls into the game's own functions.
    unsafe {
        let v = DAT_00b6a2c0;
        let p = FUN_004ddc10(idx);
        FUN_004db7d0(10, p, arr, 0, 1, v);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004e7ce0() -> u32 {
    // SAFETY: everything touched here is the game's own memory and functions.
    unsafe {
        let p0106 = (&raw const DAT_00bb0106).cast::<u8>();
        let p10a6 = (&raw const DAT_00bb10a6).cast::<u8>();
        let p2046 = (&raw const DAT_00bb2046).cast::<u8>();
        let p260e = (&raw const DAT_00bb260e).cast::<u8>();
        let p242e = (&raw const DAT_00bb242e).cast::<u8>();
        let p010a = (&raw const DAT_00bb010a).cast::<u8>();
        let p010e = (&raw const DAT_00bb010e).cast::<u8>();
        let p10aa = (&raw const DAT_00bb10aa).cast::<u8>();
        let p10ae = (&raw const DAT_00bb10ae).cast::<u8>();
        let p204a = (&raw const DAT_00bb204a).cast::<u8>();
        let p2612 = (&raw const DAT_00bb2612).cast::<u8>();
        let p2432 = (&raw const DAT_00bb2432).cast::<u8>();
        let buf = (&raw mut DAT_00b69d20).cast::<i8>();
        let fmt = (&raw const s__d__s_0052b4c8).cast::<i8>();
        let ss = (&raw mut s_settingsound_0052ad80).cast::<i8>();

        let mut l14 = [0i32; 4];
        let arr = l14.as_mut_ptr();

        let mut x: i32 = 0x58;
        let mut y: i32 = 0x14a;
        let p = FUN_004ddc10(0x292);
        FUN_004db4e0(0xf, p, 0x58, 0x137, 0xffff);
        let mut local20: i32 = 0;
        let mut b4: u8 = 0;
        let b3: u8 = if FUN_004cfee0() == 0 && DAT_00b69e30 != 2 {
            0
        } else {
            1
        };

        if a5aa() == 4 || a5aa() == 5 {
            'top: {
                if b3 != 0 {
                    let mut local4: u32 = 0;
                    let idx: u32;
                    if a5aa() == 4 {
                        let s = sel();
                        let v = if DAT_00b69e30 == 2 {
                            rd4(p0106, (s as u32).wrapping_mul(0xa0))
                        } else {
                            rd4(p10a6, (s as u32).wrapping_mul(0xa0))
                        };
                        local4 = (v == 1) as u32;
                        idx = 0x305;
                    } else {
                        let v;
                        if DAT_00b69e30 == 2 {
                            v = rd4(p2046, (sel() as u32).wrapping_mul(0x28));
                        } else {
                            let r = FUN_004cfee0();
                            let s = sel();
                            if r == 0 {
                                v = rd4(p260e, (s as u32).wrapping_mul(0x28));
                            } else {
                                v = rd4(p242e, (s as u32).wrapping_mul(0x28));
                            }
                        }
                        if v == 1 {
                            local4 = 1;
                        }
                        idx = 0x30a;
                    }
                    let p = FUN_004ddc10(idx);
                    copy_to_buf(p);
                    if local4 == 0 {
                        break 'top;
                    }
                    FUN_004db4e0(10, buf, 0x58, 0x14a, 0xffff);
                    let w = FUN_004dbfa0(buf, 10);
                    FUN_004d6460(arr, w.wrapping_add(0x5a), 0x14a, 0xfd, 0x157);
                    let r = btn(u32::from(DAT_00bb73a6).wrapping_add(0x306), arr, 0x21);
                    if r != 0 {
                        b4 = 1;
                        DAT_00bb73a6 ^= 1;
                        if DAT_00bb73a6 != 0 {
                            if a5aa() == 4 {
                                let off = (sel() as u32).wrapping_mul(0xa0);
                                if DAT_00b69e30 == 2 {
                                    DAT_00bb7398 = rd1(p010a, off);
                                    let off2 = (sel() as u32).wrapping_mul(0xa0);
                                    DAT_00bb7397 = rd1(p010e, off2);
                                } else {
                                    DAT_00bb7398 = rd1(p10aa, off);
                                    let off2 = (sel() as u32).wrapping_mul(0xa0);
                                    DAT_00bb7397 = rd1(p10ae, off2);
                                }
                            } else if a5aa() == 5 {
                                if DAT_00b69e30 == 2 {
                                    DAT_00bb7397 = rd1(p204a, (sel() as u32).wrapping_mul(0x28));
                                } else {
                                    let r2 = FUN_004cfee0();
                                    let s = sel();
                                    if r2 == 0 {
                                        DAT_00bb7397 = rd1(p2612, (s as u32).wrapping_mul(0x28));
                                    } else {
                                        DAT_00bb7397 = rd1(p2432, (s as u32).wrapping_mul(0x28));
                                    }
                                }
                            }
                        }
                    }
                } else {
                    if DAT_00aa6120 == 0 {
                        break 'top;
                    }
                    let idx: u32 = if a5aa() == 4 {
                        0x305
                    } else if a5aa() == 5 {
                        0x30a
                    } else {
                        0
                    };
                    if idx != 0 {
                        let p = FUN_004ddc10(idx);
                        copy_to_buf(p);
                    }
                    FUN_004db4e0(10, buf, 0x58, 0x14a, 0xffff);
                    let w = FUN_004dbfa0(buf, 10);
                    FUN_004d6460(arr, w.wrapping_add(0x5a), 0x14a, 0xfd, 0x157);
                    stat((DAT_00aa6124 as u32).wrapping_add(0x306), arr);
                }
                local20 = 1;
                y = 0x158;
            }
        }

        let keep: i32;
        'join: {
            'l82aa: {
                if DAT_00b69e30 != 2 {
                    if b3 != 0 {
                        label(0x1aa, 0x58, y, arr);
                        let r = btn(u32::from(DAT_00bb7399).wrapping_add(0x1b7), arr, 0x15);
                        if r != 0 {
                            b4 = 1;
                            DAT_00bb7399 ^= 1;
                        }
                        y = y.wrapping_add(0xe);
                        local20 = local20.wrapping_add(1);
                        if local20 == 6 {
                            x = 0x106;
                            y = 0x14a;
                        }
                    }
                    if DAT_00b69e30 == 2 || (a5aa() != 3 && a5aa() != 4) {
                        break 'l82aa;
                    }
                    label(0x300, x, y, arr);
                    keep = local20;
                    if b3 != 0 {
                        let r = btn(u32::from(DAT_00bb73a5).wrapping_add(0x301), arr, 0x14);
                        if r != 0 {
                            b4 = 1;
                            DAT_00bb73a5 = DAT_00bb73a5.wrapping_add(1);
                            if DAT_00bb73a5 > 3 {
                                DAT_00bb73a5 = 0;
                            }
                        }
                    } else {
                        stat(u32::from(DAT_00bb73a5).wrapping_add(0x301), arr);
                    }
                    break 'join;
                }
            }
            label(0x1a5, x, y, arr);
            keep = local20;
            if a5aa() == 5 {
                if b3 != 0 {
                    let r = btn(u32::from(DAT_00bb7394).wrapping_add(0x328), arr, 0x14);
                    if r != 0 {
                        b4 = 1;
                        DAT_00bb7394 = DAT_00bb7394.wrapping_add(1);
                        if DAT_00bb7394 > 3 {
                            DAT_00bb7394 = 0;
                        }
                    }
                } else {
                    stat(u32::from(DAT_00bb7394).wrapping_add(0x328), arr);
                }
            } else if b3 == 0 {
                stat(u32::from(DAT_00bb7394).wrapping_add(0x1b1), arr);
            } else {
                let r = btn(u32::from(DAT_00bb7394).wrapping_add(0x1b1), arr, 0x14);
                if r != 0 {
                    b4 = 1;
                    DAT_00bb7394 = DAT_00bb7394.wrapping_add(1);
                    if DAT_00bb7394 > 2 {
                        DAT_00bb7394 = 0;
                    }
                }
            }
        }

        local20 = keep.wrapping_add(1);
        y = y.wrapping_add(0xe);
        if local20 == 6 {
            x = 0x106;
            y = 0x14a;
        }

        if DAT_00b69e30 != 2 {
            label(0x1af, x, y, arr);
            if DAT_00bb739e == 0 {
                let p = FUN_004ddc10(0x1bd);
                copy_to_buf(p);
            } else if DAT_00bb739e == 0xff {
                let p = FUN_004ddc10(0x1be);
                copy_to_buf(p);
            } else {
                let p = FUN_004ddc10(0x1c0);
                _sprintf(buf, fmt, u32::from(DAT_00bb739e), p);
            }
            if b3 != 0 {
                let r = FUN_004da650(arr, buf, 10, 0xffff, 0x16, ss);
                if r == 1 {
                    b4 = 1;
                    if DAT_00bb739e == 0 {
                        DAT_00bb739e = 0xff;
                    } else if DAT_00bb739e == 0xff {
                        DAT_00bb739e = 1;
                    } else {
                        DAT_00bb739e = DAT_00bb739e.wrapping_add(1);
                        if DAT_00bb739e > 0x14 {
                            DAT_00bb739e = 0;
                        }
                    }
                }
                if r == 2 {
                    b4 = 1;
                    if DAT_00bb739e == 0xff {
                        DAT_00bb739e = 0;
                    } else if DAT_00bb739e == 0 {
                        DAT_00bb739e = 0x14;
                    } else {
                        DAT_00bb739e = DAT_00bb739e.wrapping_sub(1);
                        if DAT_00bb739e == 0 {
                            DAT_00bb739e = 0xff;
                        }
                    }
                }
            } else {
                FUN_004db7d0(10, buf, arr, 0, 1, DAT_00b6a2c0);
            }
            y = y.wrapping_add(0xe);
            if local20 == 5 {
                x = 0x106;
                y = 0x14a;
            }

            label(0x1b0, x, y, arr);
            if DAT_00bb739f == 0 {
                let p = FUN_004ddc10(0x1bd);
                copy_to_buf(p);
            } else {
                let p = FUN_004ddc10(0x1c0);
                _sprintf(buf, fmt, u32::from(DAT_00bb739f), p);
            }
            if b3 != 0 {
                let r = FUN_004da650(arr, buf, 10, 0xffff, 0x17, ss);
                if r == 1 {
                    b4 = 1;
                    DAT_00bb739f = DAT_00bb739f.wrapping_add(1);
                    if DAT_00bb739f > 10 {
                        DAT_00bb739f = 0;
                    }
                }
                if r == 2 {
                    b4 = 1;
                    if DAT_00bb739f == 0 {
                        DAT_00bb739f = 10;
                    } else {
                        DAT_00bb739f = DAT_00bb739f.wrapping_sub(1);
                    }
                }
            } else {
                FUN_004db7d0(10, buf, arr, 0, 1, DAT_00b6a2c0);
            }
            y = y.wrapping_add(0xe);
            local20 = keep.wrapping_add(3);
            if local20 == 6 {
                x = 0x106;
                y = 0x14a;
            }
        }

        if a5aa() == 4 {
            label(0x1a9, x, y, arr);
            let mut b2: u8 = 0;
            if b3 != 0 {
                'c: {
                    if DAT_00b69e30 == 2 {
                        if DAT_00bb73a6 == 1 {
                            if rd4(p0106, (sel() as u32).wrapping_mul(0xa0)) == 1 {
                                break 'c;
                            }
                        }
                    } else if DAT_00bb73a6 == 1 {
                        if rd4(p10a6, (sel() as u32).wrapping_mul(0xa0)) == 1 {
                            break 'c;
                        }
                    }
                    b2 = 1;
                }
            }
            if b2 != 0 {
                let r = btn(u32::from(DAT_00bb7398).wrapping_add(0x1b4), arr, 0x1a);
                if r != 0 {
                    b4 = 1;
                    DAT_00bb7398 = DAT_00bb7398.wrapping_add(1);
                    if DAT_00bb7398 > 2 {
                        DAT_00bb7398 = 0;
                    }
                }
            } else {
                stat(u32::from(DAT_00bb7398).wrapping_add(0x1b4), arr);
            }
            y = y.wrapping_add(0xe);
            local20 = local20.wrapping_add(1);
            if local20 == 6 {
                x = 0x106;
                y = 0x14a;
            }
        }

        if a5aa() != 5 {
            let idx: u32;
            if a5aa() == 4 {
                idx = 0x2e0;
            } else {
                idx = 0x1a8;
            }
            label(idx, x, y, arr);
            let mut b2: u8 = 0;
            if b3 != 0 {
                'd: {
                    if a5aa() == 4 {
                        if DAT_00b69e30 == 2 {
                            if DAT_00bb73a6 == 1 {
                                if rd4(p0106, (sel() as u32).wrapping_mul(0xa0)) == 1 {
                                    break 'd;
                                }
                            }
                        } else if DAT_00bb73a6 == 1 {
                            if rd4(p10a6, (sel() as u32).wrapping_mul(0xa0)) == 1 {
                                break 'd;
                            }
                        }
                    } else if a5aa() == 5 {
                        if DAT_00b69e30 == 2 {
                            if DAT_00bb73a6 == 1 {
                                if rd4(p2046, (sel() as u32).wrapping_mul(0x28)) == 1 {
                                    break 'd;
                                }
                            }
                        } else {
                            let r = FUN_004cfee0();
                            if r == 0 {
                                if DAT_00bb73a6 == 1 {
                                    if rd4(p260e, (sel() as u32).wrapping_mul(0x28)) == 1 {
                                        break 'd;
                                    }
                                }
                            } else if DAT_00bb73a6 == 1 {
                                if rd4(p242e, (sel() as u32).wrapping_mul(0x28)) == 1 {
                                    break 'd;
                                }
                            }
                        }
                    }
                    b2 = 1;
                }
            }
            if b2 != 0 {
                if a5aa() == 4 {
                    let r = btn(u32::from(DAT_00bb7397).wrapping_add(0x333), arr, 0x18);
                    if r != 0 {
                        b4 = 1;
                        DAT_00bb7397 = DAT_00bb7397.wrapping_add(1);
                        if DAT_00bb7397 > 2 {
                            DAT_00bb7397 = 0;
                        }
                    }
                } else {
                    let r = btn(u32::from(DAT_00bb7397).wrapping_add(0xec), arr, 0x18);
                    if r != 0 {
                        b4 = 1;
                        DAT_00bb7397 ^= 1;
                    }
                }
            } else {
                let idx2: u32 = if a5aa() == 4 {
                    u32::from(DAT_00bb7397).wrapping_add(0x333)
                } else {
                    u32::from(DAT_00bb7397).wrapping_add(0xec)
                };
                stat(idx2, arr);
            }
            y = y.wrapping_add(0xe);
            local20 = local20.wrapping_add(1);
            if local20 == 6 {
                x = 0x106;
                y = 0x14a;
            }
        }

        'l8f19: {
            if DAT_00b69e30 != 2 {
                if a5aa() == 1 || a5aa() == 2 {
                    label(0x23c, x, y, arr);
                    if b3 != 0 {
                        let r = btn(u32::from(DAT_00bb73a4).wrapping_add(0xec), arr, 0x19);
                        if r != 0 {
                            b4 = 1;
                            DAT_00bb73a4 ^= 1;
                        }
                    } else {
                        stat(u32::from(DAT_00bb73a4).wrapping_add(0xec), arr);
                    }
                    y = y.wrapping_add(0xe);
                    local20 = local20.wrapping_add(1);
                    if local20 == 6 {
                        x = 0x106;
                        y = 0x14a;
                    }
                }
                if DAT_00b69e30 == 2 {
                    break 'l8f19;
                }
                if a5aa() != 5 {
                    label(0x1ac, x, y, arr);
                    if b3 != 0 {
                        let r = btn(u32::from(DAT_00bb739b).wrapping_add(0x218), arr, 0x1b);
                        if r != 0 {
                            b4 = 1;
                            DAT_00bb739b = DAT_00bb739b.wrapping_add(1);
                            if DAT_00bb739b > 2 {
                                DAT_00bb739b = 0;
                            }
                        }
                    } else {
                        stat(u32::from(DAT_00bb739b).wrapping_add(0x218), arr);
                    }
                    if DAT_00bb739b == 2 && a5aa() != 1 && a5aa() != 2 {
                        DAT_00bb739b = 0;
                    }
                    y = y.wrapping_add(0xe);
                    local20 = local20.wrapping_add(1);
                    if local20 == 6 {
                        x = 0x106;
                        y = 0x14a;
                    }
                }
            }
            if DAT_00b69e30 != 2 {
                label(0x1ad, x, y, arr);
                if b3 != 0 {
                    let r = btn(u32::from(DAT_00bb739c).wrapping_add(0x1bb), arr, 0x1c);
                    if r != 0 {
                        b4 = 1;
                        DAT_00bb739c ^= 1;
                    }
                } else {
                    stat(u32::from(DAT_00bb739c).wrapping_add(0x1bb), arr);
                }
                y = y.wrapping_add(0xe);
                local20 = local20.wrapping_add(1);
                if local20 == 6 {
                    x = 0x106;
                    y = 0x14a;
                }
            }
        }

        label(0x1ae, x, y, arr);
        if b3 != 0 {
            let r = btn(u32::from(DAT_00bb739d).wrapping_add(0x1bd), arr, 0x1d);
            if r != 0 {
                b4 = 1;
                DAT_00bb739d = DAT_00bb739d.wrapping_add(1);
                if DAT_00bb739d > 2 {
                    DAT_00bb739d = 0;
                }
                if DAT_00b69e30 != 2 && (a5aa() == 3 || a5aa() == 4) && DAT_00bb739d == 2 {
                    DAT_00bb739d = 0;
                }
            }
        } else {
            stat(u32::from(DAT_00bb739d).wrapping_add(0x1bd), arr);
        }
        y = y.wrapping_add(0xe);
        if local20 == 5 {
            x = 0x106;
            y = 0x14a;
        }

        label(0x1a6, x, y, arr);
        if b3 != 0 {
            let r = btn(u32::from(DAT_00bb7395).wrapping_add(0xec), arr, 0x1f);
            if r != 0 {
                b4 = 1;
                DAT_00bb7395 ^= 1;
            }
        } else {
            stat(u32::from(DAT_00bb7395).wrapping_add(0xec), arr);
        }

        if DAT_00b69e30 != 2 && b4 != 0 {
            DAT_00aa6af0 = 0x59;
            DAT_00aa6af4 = u32::from(DAT_00bb7394);
            DAT_00aa6af8 = u32::from(DAT_00bb7395);
            DAT_00aa6afc = u32::from(DAT_00bb7396);
            DAT_00aa6b00 = u32::from(DAT_00bb7397);
            DAT_00aa6b04 = u32::from(DAT_00bb7398);
            DAT_00aa6b08 = u32::from(DAT_00bb7399);
            DAT_00aa6b0c = u32::from(DAT_00bb739a);
            DAT_00aa6b10 = u32::from(DAT_00bb739b);
            DAT_00aa6b14 = u32::from(DAT_00bb739c);
            DAT_00aa6b18 = u32::from(DAT_00bb739d);
            DAT_00aa6b1c = u32::from(DAT_00bb739e);
            DAT_00aa6b20 = u32::from(DAT_00bb739f);
            DAT_00aa6b24 = _rand() as u32;
            DAT_00aa6b28 = u32::from(DAT_00bb736f);
            DAT_00aa6b2c = u32::from(DAT_00bb73a4);
            DAT_00aa6b30 = u32::from(DAT_00bb7370);
            DAT_00aa6b34 = u32::from(DAT_00bb73a5);
            DAT_00aa6b38 = u32::from(DAT_00bb73a6);
            FUN_004cef70(0, &raw mut DAT_00aa6af0, 0x4c);
        }
        1
    }
}
