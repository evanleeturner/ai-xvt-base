// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052552c: i32;
    static mut DAT_00655184: u16;
    static mut DAT_00656c90: [u16; 1];
    static mut DAT_00655ab0: [u8; 1];
    static mut DAT_0071f2f0: [u32; 1];
    static mut DAT_0071ecf4: i32;
    static mut DAT_006d4988: i32;
    static mut DAT_0071b730: u8;
    static mut DAT_0070a184: unsafe extern "C" fn(u32, u16, i16, i32, i32);
}

unsafe fn rb(a: u32) -> u8 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as usize as *const u8).read() }
}

unsafe fn rw(a: u32) -> u16 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as usize as *const u16).read_unaligned() }
}

unsafe fn rd(a: u32) -> u32 {
    // SAFETY: address inside the game's own memory.
    unsafe { (a as usize as *const u32).read_unaligned() }
}

unsafe fn cur_entry() -> u32 {
    // SAFETY: the globals are the game's own; the chain of pointers is the game's data.
    unsafe {
        let p =
            (&raw mut DAT_0071b730).byte_add((DAT_0071ecf4 as u32).wrapping_mul(0x5bd) as usize);
        let iv6 = (p as *const u32).read_unaligned();
        let tmp = rd(iv6
            .wrapping_mul(0x23)
            .wrapping_add(0x1f)
            .wrapping_add(DAT_006d4988 as u32));
        rd(tmp.wrapping_add(0xa7))
    }
}

unsafe fn arr_ptr() -> *mut u16 {
    (&raw mut DAT_00656c90).cast::<u16>()
}

unsafe fn rec_ptr(idx: u32) -> *const u8 {
    // SAFETY: table is the game's own.
    unsafe {
        (&raw mut DAT_00655ab0)
            .cast::<u8>()
            .byte_add(idx.wrapping_mul(0xc) as usize) as *const u8
    }
}

unsafe fn tex(i: u32) -> u32 {
    // SAFETY: table is the game's own.
    unsafe { (&raw mut DAT_0071f2f0).cast::<u32>().add(i as usize).read() }
}

unsafe fn draw1(idx: u32, val: u16, a: i16) {
    // SAFETY: globals and tables are the game's own.
    unsafe {
        let mut u10: u16 = 0;
        arr_ptr().add(idx as usize).write(val);
        let r = rec_ptr(idx);
        let u3 = r.cast::<u16>().read_unaligned();
        let mut s9 = r.byte_add(2).cast::<i16>().read_unaligned();
        let u11 = r.byte_add(4).cast::<u16>().read_unaligned();
        loop {
            let b = (u10 < val) as u32;
            u10 = u10.wrapping_add(1);
            let f = DAT_0070a184;
            f(tex(b.wrapping_add(u11 as u32)), u3, s9, 0xfd, 0);
            s9 = (s9 as i32).wrapping_sub(a as i32) as i16;
            if u10 >= 0xc {
                break;
            }
        }
    }
}

unsafe fn draw2(idx: u32, val: u16, a: i16, c: u16) {
    // SAFETY: globals and tables are the game's own.
    unsafe {
        let mut u11: u16 = 0;
        arr_ptr().add(idx as usize).write(val);
        let r = rec_ptr(idx);
        let u3 = r.cast::<u16>().read_unaligned();
        let mut s9 = r.byte_add(2).cast::<i16>().read_unaligned();
        let u5 = r.byte_add(4).cast::<u16>().read_unaligned();
        if c != 0 {
            loop {
                let b = (u11 < val) as u32;
                u11 = u11.wrapping_add(1);
                let f = DAT_0070a184;
                f(tex(b.wrapping_add(u5 as u32)), u3, s9, 0xfd, 0);
                s9 = (s9 as i32).wrapping_add((a as i32).wrapping_mul(-2)) as i16;
                if u11 >= c {
                    break;
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004ae700() {
    // SAFETY: all globals and pointers are the game's own memory.
    unsafe {
        let p =
            (&raw mut DAT_0071b730).byte_add((DAT_0071ecf4 as u32).wrapping_mul(0x5bd) as usize);
        let iv6 = (p as *const u32).read_unaligned();
        let iv12 = iv6.wrapping_mul(0x23).wrapping_add(DAT_006d4988 as u32);
        let tmp = rd(iv12.wrapping_add(0x1f));
        let iv4 = rd(tmp.wrapping_add(0xa7));

        let flag: bool;
        if iv6 == 0xffff_ffff {
            flag = false;
        } else {
            let c1 = rb(iv12.wrapping_add(3)) as i8;
            flag = c1 == 1 || c1 == 2 || c1 == 3 || c1 == 0xe || c1 == 4;
        }

        if !flag {
            let local_a: i16 = if DAT_0052552c == 0x13 {
                2
            } else {
                let c = (DAT_0052552c == 0x112) as u16;
                ((c.wrapping_neg() & 0xfffe).wrapping_add(6)) as i16
            };

            if rb(iv4.wrapping_add(0x1dc)) & 2 != 0 {
                let u7 = DAT_00655184.wrapping_add(0x1b) as u32;
                let u5 = (rb(iv4.wrapping_add(0x212)) as u16).wrapping_mul(3);
                if *arr_ptr().add(u7 as usize) != u5 {
                    draw1(u7, u5, local_a);
                }
            }

            let e = cur_entry();
            if (rb(e.wrapping_add(0x1dc)) & 8 != 0) && (rb(e.wrapping_add(0x1dd)) & 1 != 0) {
                let u7 = DAT_00655184.wrapping_add(0x1c) as u32;
                let u5 = (rb(e.wrapping_add(0x20f)) as u16).wrapping_mul(3);
                if *arr_ptr().add(u7 as usize) != u5 {
                    draw1(u7, u5, local_a);
                }
            }

            let e = cur_entry();
            if (rb(e.wrapping_add(0x1dc)) & 0x10 != 0) && (rb(e.wrapping_add(0x1de)) & 1 != 0) {
                let u7 = DAT_00655184.wrapping_add(0x1d) as u32;
                let u5 = (rb(e.wrapping_add(0x234)) as u16).wrapping_mul(3);
                if *arr_ptr().add(u7 as usize) != u5 {
                    draw1(u7, u5, local_a);
                }
            }

            let e = cur_entry();
            if rb(e.wrapping_add(0x1dc)) & 4 == 0 {
                return;
            }
            let mut u5 = (8i32.wrapping_sub(rb(e.wrapping_add(0x212)) as i32)) as u16;
            if rw(e.wrapping_add(0x1dd)) & 1 != 0 {
                u5 = ((u5 as i32)
                    .wrapping_sub(rb(e.wrapping_add(0x20f)) as i32)
                    .wrapping_add(2)) as u16;
            }
            if rw(e.wrapping_add(0x1dd)) & 0x100 != 0 {
                u5 = ((u5 as i32)
                    .wrapping_sub(rb(e.wrapping_add(0x234)) as i32)
                    .wrapping_add(2)) as u16;
            }
            let u7 = DAT_00655184.wrapping_add(0x1a) as u32;
            if *arr_ptr().add(u7 as usize) == u5 {
                return;
            }
            draw1(u7, u5, local_a);
            return;
        }

        if rw(iv4.wrapping_add(0x1db)) & 0xe00 == 0 {
            return;
        }
        let mut local_a: i16 = 0;
        let mut local_c: u16 = 0;
        if DAT_0052552c == 0x13 {
            local_a = 2;
            local_c = 4;
        } else if DAT_0052552c == 0x101 {
            local_a = 5;
            local_c = 4;
        } else if DAT_0052552c == 0x112 {
            local_a = 3;
            local_c = 4;
        }
        let b2 = rb(iv4.wrapping_add(0x20f));
        let mut local_e = rb(iv4.wrapping_add(0x212)) as u16;
        let mut u5 = ((8i32.wrapping_sub(local_e as i32)).wrapping_sub(b2 as i32)) as u16;
        let mut u7 = b2 as u32;
        if DAT_00655184 != 0 {
            local_c = local_c.wrapping_mul(2);
            local_e = (rb(iv4.wrapping_add(0x212)) as u16).wrapping_mul(2);
            u5 = u5.wrapping_mul(2);
            u7 = (b2 as u16).wrapping_mul(2) as u32;
        }
        let mut local_12: u32 = u7 << 16;

        if rw(iv4.wrapping_add(0x1db)) & 0x400 != 0 {
            let u8 = DAT_00655184.wrapping_add(0x1a) as u32;
            if *arr_ptr().add(u8 as usize) != u5 {
                arr_ptr().add(u8 as usize).write(u5);
                let r = rec_ptr(u8);
                let u3 = r.cast::<u16>().read_unaligned();
                let u11 = r.byte_add(4).cast::<u16>().read_unaligned();
                let mut u10: u16 = 0;
                local_12 = ((u7 as u16 as u32) << 16)
                    | (r.byte_add(2).cast::<u16>().read_unaligned() as u32);
                if local_c & 0x7fff != 0 {
                    loop {
                        let b = (u10 < u5) as u32;
                        u10 = u10.wrapping_add(1);
                        let f = DAT_0070a184;
                        f(
                            tex(b.wrapping_add(u11 as u32)),
                            u3,
                            (local_12 & 0xffff) as u16 as i16,
                            0xfd,
                            0,
                        );
                        let low = ((local_12 as u16 as i16 as i32).wrapping_sub(local_a as i32))
                            as u16 as u32;
                        local_12 = (local_12 & 0xffff_0000) | low;
                        if u10 >= local_c.wrapping_mul(2) {
                            break;
                        }
                    }
                }
            }
        }

        if rb(iv4.wrapping_add(0x1dc)) & 8 != 0 {
            let u7 = DAT_00655184.wrapping_add(0x1c) as u32;
            let v = (local_12 >> 16) as u16;
            if *arr_ptr().add(u7 as usize) != v {
                draw2(u7, v, local_a, local_c);
            }
        }

        if rb(iv4.wrapping_add(0x1dc)) & 2 != 0 {
            let u7 = DAT_00655184.wrapping_add(0x1b) as u32;
            if *arr_ptr().add(u7 as usize) != local_e {
                draw2(u7, local_e, local_a, local_c);
            }
        }
    }
}
