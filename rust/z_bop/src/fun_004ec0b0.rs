// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052c778: [i8; 1];
    static mut DAT_00b69e30: i32;
    static mut DAT_00b6a5aa: [u8; 4];
    static mut DAT_00b6a606: [u8; 4];
    static mut DAT_00aa615e: [u8; 1];
    static mut DAT_00aa62ae: [u8; 1];
    static mut DAT_00aa6104: i32;
    static mut DAT_00a91d1a: [u8; 1];
    static mut DAT_0052c5d8: [i32; 1];
    static mut DAT_00a91ca0: [i32; 1];
    static mut DAT_00bb7394: i8;
    static mut DAT_00bb739b: i8;
    static mut DAT_00bb739d: i8;
    static mut DAT_00aa60f0: i32;
    static mut DAT_00aa6108: i32;
    static mut DAT_00aa60ec: i32;
    static mut DAT_00aa60e8: i32;
    static mut DAT_00aa60d4: i32;
    static mut DAT_00aa60dc: i32;
    static mut DAT_00aa60e0: i32;
    static mut DAT_00b6a2c0: u32;
    static mut _DAT_00b69ccc: u32;
    static mut DAT_00b69d20: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d6460(rect: *mut i32, a: i32, b: i32, c: i32, d: i32);
    fn FUN_0042b090(a: i32, b: i32, c: i32);
    fn FUN_004ee8e0(a: i32) -> u32;
    fn FUN_0042b0d0(a: u32);
    fn FUN_0042b0e0(a: i32, b: i32, c: i32);
    fn FUN_0042a380(a: i32, b: i32, c: i32, d: i32, e: i32) -> i32;
    fn FUN_004d11e0() -> i32;
    fn FUN_004ddc10(a: u32) -> *mut i8;
    fn _sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_004db7d0(a: i32, s: *mut i8, rect: *mut i32, b: i32, c: i32, d: u32);
    fn FUN_004d64a0(rect: *mut i32, a: i32, b: i32);
    fn FUN_004ee510(a: i32) -> i32;
    fn FUN_004dbfa0(s: *mut i8, a: i32) -> i32;
    fn FUN_004ee650(a: i32) -> i8;
    fn FUN_004ee7c0(a: i32) -> i8;
    fn FUN_004cfee0() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004ec0b0() {
    // SAFETY: all globals and callees are the game's own; pointers passed
    // refer to the game's buffers and this function's locals.
    unsafe {
        let buf = &raw mut DAT_00b69d20 as *mut i8;
        let mut local_10: [i32; 4] = [0; 4];
        let r = local_10.as_mut_ptr();
        let u_14: u32;
        let u_13: u32;
        let u_11: u32;
        if DAT_00b69e30 == 2 {
            u_14 = 0x197;
            u_13 = 0x1ae;
            u_11 = 0x54;
        } else {
            u_14 = 0x14d;
            u_13 = 0x172;
            u_11 = 0x90;
        }
        FUN_004d6460(r, u_11 as i32, 0x6b, u_13 as i32, u_14 as i32);
        FUN_0042b090(0, 0, 0);
        let mut u4 = FUN_004ee8e0(-1);
        let mut v5aa: i32 = (&raw const DAT_00b6a5aa).cast::<i32>().read_unaligned();
        if v5aa == 1 || v5aa == 2 {
            let base = (&raw const DAT_00a91d1a)
                .cast::<u8>()
                .offset(((DAT_00aa6104 as u32).wrapping_mul(0x562) as i32) as isize)
                .read() as u32;
            let u8_: u32;
            match u4 {
                1 | 2 | 3 | 4 | 0xe | 0x10 => {
                    u8_ = base;
                }
                _ => {
                    let mut t = base.wrapping_add(1);
                    if 3 < t {
                        t = 0;
                    }
                    u8_ = t;
                }
            }
            FUN_0042b0d0(u8_);
        }
        let tbl = (&raw const DAT_0052c5d8).cast::<i32>();
        let ti = u4.wrapping_mul(3) as usize;
        FUN_0042b0e0(
            tbl.add(ti).read(),
            tbl.add(ti.wrapping_add(1)).read(),
            tbl.add(ti.wrapping_add(2)).read(),
        );
        let mut i9: i32 = 0;
        let mut p10 = (&raw mut DAT_00aa615e).cast::<u8>();
        FUN_0042a380(
            local_10[0],
            local_10[1],
            (local_10[2] as u32)
                .wrapping_sub(local_10[0] as u32)
                .wrapping_add(1) as i32,
            (local_10[3] as u32)
                .wrapping_sub(local_10[1] as u32)
                .wrapping_add(1) as i32,
            0,
        );
        loop {
            let i5 = FUN_004d11e0();
            let tmp: i32 = p10.cast::<i32>().read_unaligned();
            if i5 == tmp {
                break;
            }
            p10 = p10.add(0x2a);
            i9 = i9.wrapping_add(1);
            if !((p10 as usize) < ((&raw mut DAT_00aa62ae) as usize)) {
                break;
            }
        }

        if (&raw const DAT_00a91ca0)
            .cast::<i32>()
            .offset(i9 as isize)
            .read()
            != 0
            && DAT_00b69e30 != 2
        {
            FUN_004d6460(r, 0x58, 0x6f, 0xea, 0x7d);
            u4 = FUN_004ee8e0(-1);
            let p6 = FUN_004ddc10(u4.wrapping_add(0x15));
            let p7 = FUN_004ddc10(0x25d);
            _sprintf(buf, c"%s: %s".as_ptr(), p7, p6);
            FUN_004db7d0(0xc, buf, r, 0, 1, _DAT_00b69ccc);
            FUN_004d64a0(r, 0, 0xf);
            let u11 = DAT_00aa60ec as u32;
            let p6 = FUN_004ddc10(0x23d);
            _sprintf(buf, c"%s %u".as_ptr(), p6, u11);
            FUN_004db7d0(0xc, buf, r, 0, 1, _DAT_00b69ccc);
            FUN_004d64a0(r, 0, 0xf);
            if DAT_00bb739d == 2 {
                let p6 = FUN_004ddc10(0x1bf);
                let p7 = FUN_004ddc10(0x10b);
                _sprintf(buf, c"%s %s".as_ptr(), p7, p6);
            } else if DAT_00bb739d == 0 {
                let p6 = FUN_004ddc10(0x10b);
                _sprintf(buf, (&raw const DAT_0052c778).cast::<i8>(), p6);
            } else {
                let v6 = (DAT_00aa60e8 as u32).wrapping_add(1);
                let p7 = FUN_004ddc10(0x10b);
                _sprintf(buf, c"%s %u".as_ptr(), p7, v6);
            }
            FUN_004db7d0(0xc, buf, r, 0, 1, _DAT_00b69ccc);
            FUN_004d6460(r, 0x10e, 0x6f, 0x1aa, 0x7d);
            FUN_004d6460(r, 0x113, 0x6f, 0x1af, 0x7d);
            if DAT_00aa60d4 == 0 {
                u4 = 0x111;
            } else {
                let i = FUN_004ee510(-1);
                u4 = (i as u32).wrapping_add(0x111);
            }
            let p6 = FUN_004ddc10(u4);
            let p7 = FUN_004ddc10(0x108);
            _sprintf(buf, c"%s %s".as_ptr(), p7, p6);
            let i9 = local_10[2];
            local_10[0] = FUN_004dbfa0(buf, 0xc);
            local_10[0] = (i9 as u32).wrapping_sub(local_10[0] as u32) as i32;
            FUN_004db7d0(0xc, buf, r, 0, 1, _DAT_00b69ccc);
            FUN_004d64a0(r, 0, 0xf);
            let mut b1 = 1;
            'a: {
                if DAT_00aa60dc == 0 {
                    let p6 = FUN_004ddc10(0x111);
                    let p7 = FUN_004ddc10(0x10a);
                    _sprintf(buf, c"%s %s".as_ptr(), p7, p6);
                    u4 = FUN_004ee8e0(-1);
                    if (u4 as i32) < 1 || (5 < (u4 as i32) && u4 != 0xe) {
                        break 'a;
                    }
                } else {
                    u4 = FUN_004ee8e0(-1);
                    if (u4 as i32) < 1 || (5 < (u4 as i32) && u4 != 0xe) {
                        let c3 = FUN_004ee650(-1);
                        let p6 = FUN_004ddc10((c3 as u8 as u32).wrapping_add(0x11b));
                        let p7 = FUN_004ddc10(0x10a);
                        _sprintf(buf, c"%s %s".as_ptr(), p7, p6);
                        break 'a;
                    }
                    let p6 = FUN_004ddc10(0x111);
                    let p7 = FUN_004ddc10(0x10a);
                    _sprintf(buf, c"%s %s".as_ptr(), p7, p6);
                }
                b1 = 0;
            }
            let i9 = local_10[2];
            if b1 != 0 {
                local_10[0] = FUN_004dbfa0(buf, 0xc);
                local_10[0] = (i9 as u32).wrapping_sub(local_10[0] as u32) as i32;
                FUN_004db7d0(0xc, buf, r, 0, 1, _DAT_00b69ccc);
                FUN_004d64a0(r, 0, 0xf);
            }
            if DAT_00aa60e0 == 0 {
                u4 = 0x111;
            } else {
                let c3 = FUN_004ee7c0(-1);
                u4 = (c3 as u8 as u32).wrapping_add(0x120);
            }
            let p6 = FUN_004ddc10(u4);
            let p7 = FUN_004ddc10(0x109);
            _sprintf(buf, c"%s %s".as_ptr(), p7, p6);
            local_10[0] = FUN_004dbfa0(buf, 0xc);
            local_10[0] = (local_10[2] as u32).wrapping_sub(local_10[0] as u32) as i32;
            FUN_004db7d0(0xc, buf, r, 0, 1, _DAT_00b69ccc);
            return;
        }

        FUN_004d6460(r, 0x58, 0x6f, 0xea, 0x7d);
        u4 = FUN_004ee8e0(-1);
        v5aa = (&raw const DAT_00b6a5aa).cast::<i32>().read_unaligned();
        let v606: i32 = (&raw const DAT_00b6a606).cast::<i32>().read_unaligned();
        let b1: bool;
        let b2: bool;
        if DAT_00b69e30 == 2 {
            if v5aa == 0 && v606 == 1 && DAT_00bb7394 != 3 {
                b2 = true;
                b1 = false;
            } else {
                b2 = false;
                b1 = true;
            }
        } else if v5aa == 0 && v606 == 1 {
            if DAT_00bb7394 == 3 {
                b1 = true;
                b2 = false;
            } else {
                b1 = false;
                b2 = true;
            }
        } else if DAT_00bb739b == 0 {
            b1 = false;
            b2 = false;
        } else if DAT_00bb739b == 1
            || (DAT_00bb739b == 2 && (FUN_004cfee0() != 0 || DAT_00b69e30 == 2))
        {
            b2 = false;
            b1 = true;
        } else {
            b1 = false;
            b2 = false;
        }
        let c3: i32 = if (DAT_00aa60f0 < 2 && DAT_00aa6108 < 2) || !b1 {
            4
        } else {
            1
        };
        let i5 = c3;
        let p6 = FUN_004ddc10(u4.wrapping_add(0x15));
        let p7 = FUN_004ddc10(0x25d);
        _sprintf(buf, c"%c%s: %c%s".as_ptr(), 4i32, p7, i5, p6);
        FUN_004db7d0(0xc, buf, r, 0, 1, 0xffff);
        FUN_004d64a0(r, 0, 0xf);
        let u11 = DAT_00aa60ec as u32;
        let p6 = FUN_004ddc10(0x23d);
        _sprintf(buf, c"%c%s %c%u".as_ptr(), 4i32, p6, i5, u11);
        FUN_004db7d0(0xc, buf, r, 0, 1, 0xffff);
        FUN_004d64a0(r, 0, 0xf);
        if DAT_00bb739d == 2 {
            let p6 = FUN_004ddc10(0x1bf);
            let p7 = FUN_004ddc10(0x10b);
            _sprintf(buf, c"%c%s %c%s".as_ptr(), 4i32, p7, 4i32, p6);
        } else if DAT_00bb739d != 0 {
            let v6 = (DAT_00aa60e8 as u32).wrapping_add(1);
            let p7 = FUN_004ddc10(0x10b);
            _sprintf(buf, c"%c%s %c%u".as_ptr(), 4i32, p7, 4i32, v6);
        } else {
            let p6 = FUN_004ddc10(0x10b);
            _sprintf(buf, c"%c%s %c1".as_ptr(), 4i32, p6, 4u32);
        }
        FUN_004db7d0(0xc, buf, r, 0, 1, 0xffff);
        FUN_004d6460(r, 0x113, 0x6f, 0x1af, 0x7d);
        if DAT_00aa60d4 == 0 {
            let p6 = FUN_004ddc10(0x111);
            let p7 = FUN_004ddc10(0x108);
            _sprintf(buf, c"%c%s %s".as_ptr(), 4i32, p7, p6);
        } else {
            let p6: *mut i8;
            let u11: u32;
            if DAT_00aa60d4 == 1 || b2 {
                let i = FUN_004ee510(-1);
                p6 = FUN_004ddc10((i as u32).wrapping_add(0x111));
                u11 = 4;
            } else {
                let i = FUN_004ee510(-1);
                p6 = FUN_004ddc10((i as u32).wrapping_add(0x111));
                u11 = 1;
            }
            let p7 = FUN_004ddc10(0x108);
            _sprintf(buf, c"%c%s %c%s".as_ptr(), 4i32, p7, u11, p6);
        }
        let i9 = local_10[2];
        local_10[0] = FUN_004dbfa0(buf, 0xc);
        local_10[0] = (i9 as u32).wrapping_sub(local_10[0] as u32) as i32;
        FUN_004db7d0(0xc, buf, r, 0, 1, 0xffff);
        FUN_004d64a0(r, 0, 0xf);
        let mut b1b = true;
        'b: {
            if DAT_00aa60dc < 1 {
                let p6 = FUN_004ddc10(0x111);
                let p7 = FUN_004ddc10(0x10a);
                _sprintf(buf, c"%c%s %s".as_ptr(), 4i32, p7, p6);
                u4 = FUN_004ee8e0(-1);
                if (u4 as i32) < 1 || (5 < (u4 as i32) && u4 != 0xe) {
                    break 'b;
                }
            } else {
                u4 = FUN_004ee8e0(-1);
                if (u4 as i32) < 1 || (5 < (u4 as i32) && u4 != 0xe) {
                    let c = FUN_004ee650(-1);
                    let p6 = FUN_004ddc10((c as u8 as u32).wrapping_add(0x11b));
                    let u11: u32 = 1;
                    let p7 = FUN_004ddc10(0x10a);
                    _sprintf(buf, c"%c%s %c%s".as_ptr(), 4i32, p7, u11, p6);
                    break 'b;
                }
                let p6 = FUN_004ddc10(0x111);
                let p7 = FUN_004ddc10(0x10a);
                _sprintf(buf, c"%c%s %s".as_ptr(), 4i32, p7, p6);
            }
            b1b = false;
        }
        let i9 = local_10[2];
        if b1b {
            local_10[0] = FUN_004dbfa0(buf, 0xc);
            local_10[0] = (i9 as u32).wrapping_sub(local_10[0] as u32) as i32;
            let mut col = DAT_00b6a2c0;
            if DAT_00aa60dc != 1 && !b2 {
                col = 0xffff;
            }
            FUN_004db7d0(0xc, buf, r, 0, 1, col);
            FUN_004d64a0(r, 0, 0xf);
        }
        if DAT_00aa60e0 == 0 {
            let p6 = FUN_004ddc10(0x111);
            let p7 = FUN_004ddc10(0x109);
            _sprintf(buf, c"%c%s %s".as_ptr(), 4i32, p7, p6);
        } else {
            let p6: *mut i8;
            let u11: u32;
            if DAT_00aa60e0 == 1 || b2 {
                let c = FUN_004ee7c0(-1);
                p6 = FUN_004ddc10((c as u8 as u32).wrapping_add(0x120));
                u11 = 4;
            } else {
                let c = FUN_004ee7c0(-1);
                p6 = FUN_004ddc10((c as u8 as u32).wrapping_add(0x120));
                u11 = 1;
            }
            let p7 = FUN_004ddc10(0x109);
            _sprintf(buf, c"%c%s %c%s".as_ptr(), 4i32, p7, u11, p6);
        }
        local_10[0] = FUN_004dbfa0(buf, 0xc);
        local_10[0] = (local_10[2] as u32).wrapping_sub(local_10[0] as u32) as i32;
        FUN_004db7d0(0xc, buf, r, 0, 1, 0xffff);
    }
}
