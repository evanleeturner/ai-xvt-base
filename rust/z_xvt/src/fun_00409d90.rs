// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006db590: i8;
    static mut DAT_0085e79d: i8;
    static mut DAT_0085e795: i8;
    static mut DAT_0085e796: i8;
    static mut DAT_0085e794: i8;
    static mut DAT_0085e797: i8;

    static mut DAT_00523ae4: u32;
    static mut DAT_00523ae8: u16;
    static mut DAT_00523aec: [u32; 3];
    static mut DAT_00523af8: [u32; 3];

    static mut DAT_00523b4c: i8;
    static mut DAT_00523b1c: i8;
    static mut DAT_00523b40: [i8; 1];
    static mut DAT_00523b28: [i8; 1];
    static mut DAT_00523b34: [i8; 1];
    static mut DAT_00523b10: [i8; 1];
    static mut DAT_00523b04: [i8; 1];
    static mut DAT_00525588: [i8; 1];

    static mut DAT_0071b73a: [u8; 1];
    static mut DAT_0071ecf4: i32;
    static mut DAT_006d49a0: i16;
    static mut DAT_0070ad54: [i8; 1];
    static mut DAT_0070b252: [u8; 1];
    static mut DAT_0070ad43: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004784d0() -> u32;
    fn FUN_00409bf0(param_1: *mut i8, param_2: u16);
}

unsafe fn c_strlen(p: *const i8) -> usize {
    let mut n = 0usize;
    // SAFETY: p points at a NUL-terminated buffer.
    unsafe {
        while *p.add(n) != 0 {
            n += 1;
        }
    }
    n
}

unsafe fn c_strcat(dst: *mut i8, src: *const i8) {
    // SAFETY: dst and src are NUL-terminated and do not overlap; dst has room.
    unsafe {
        let d = dst.add(c_strlen(dst));
        let n = c_strlen(src) + 1;
        let mut i = 0usize;
        while i < n {
            *d.add(i) = *src.add(i);
            i += 1;
        }
    }
}

unsafe fn load_prefix(buf: *mut i8) {
    // SAFETY: buf holds at least 6 bytes; globals are the game's own.
    unsafe {
        buf.cast::<u32>().write_unaligned(DAT_00523ae4);
        buf.add(4).cast::<u16>().write_unaligned(DAT_00523ae8);
    }
}

unsafe fn load_kind() -> i16 {
    // SAFETY: the table is the game's own packed array.
    unsafe {
        (&raw mut DAT_0071b73a)
            .cast::<u8>()
            .byte_add((DAT_0071ecf4 as u32).wrapping_mul(0x5bd) as usize)
            .cast::<i16>()
            .read_unaligned()
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00409d90() {
    let mut uc0: [i8; 0x40] = [0; 0x40];
    let mut u80: [i8; 0x40] = [0; 0x40];
    let mut u40: [i8; 12] = [0; 12];
    let c0 = uc0.as_mut_ptr();
    let p80 = u80.as_mut_ptr();
    let p40 = u40.as_mut_ptr();

    // SAFETY: all globals and callees are the game's own; local buffers are
    // large enough for the original's string operations.
    unsafe {
        if DAT_006db590 == 0 || DAT_0085e79d == 0 {
            return;
        }
        if DAT_0085e795 != 0 {
            load_prefix(c0);
            let s2 = load_kind();
            let src: *const i8;
            if s2 == 0 {
                let r = FUN_004784d0();
                if r & 1 == 0 {
                    src = (&raw const DAT_00523b40).cast::<i8>();
                } else {
                    src = &raw const DAT_00523b4c;
                }
            } else {
                let r = FUN_004784d0();
                if r & 1 == 0 {
                    src = (&raw const DAT_00523b28).cast::<i8>();
                } else {
                    src = (&raw const DAT_00523b34).cast::<i8>();
                }
            }
            c_strcat(c0, src);
            FUN_00409bf0(c0, 0x2b5);
        }
        if DAT_0085e796 != 0 {
            load_prefix(c0);
            let s2 = load_kind();
            let src: *const i8 = if s2 == 0 {
                &raw const DAT_00523b1c
            } else if s2 == 1 {
                (&raw const DAT_00523b10).cast::<i8>()
            } else {
                (&raw const DAT_00523b04).cast::<i8>()
            };
            c_strcat(c0, src);
            FUN_00409bf0(c0, 0x321);
        }
        if DAT_0085e794 != 0 {
            let mut i_var8: i32 = 0;
            if 0 < DAT_006d49a0 {
                let mut i_var5 = DAT_006d49a0 as i32;
                let mut p = (&raw const DAT_0070ad54).cast::<i8>();
                loop {
                    if *p != 0 {
                        i_var8 = i_var8.wrapping_add(1);
                    }
                    p = p.byte_add(0x566);
                    i_var5 = i_var5.wrapping_sub(1);
                    if i_var5 == 0 {
                        break;
                    }
                }
            }
            let mut i_var5: i32 = 0;
            if 0 < DAT_006d49a0 {
                let mut p = (&raw const DAT_0070b252).cast::<u8>();
                loop {
                    let i_var7 = p.cast::<i32>().read_unaligned();
                    if i_var7 == DAT_0071ecf4 {
                        break;
                    }
                    p = p.byte_add(0x566);
                    i_var5 = i_var5.wrapping_add(1);
                    if !(i_var5 < DAT_006d49a0 as i32) {
                        break;
                    }
                }
            }
            if i_var5 < DAT_006d49a0 as i32 {
                let cnt = *(&raw const DAT_0070ad43)
                    .cast::<u8>()
                    .byte_add((i_var5 as u32).wrapping_mul(0x566) as usize);
                if 1 < cnt || 1 < i_var8 {
                    let mut c4: u32 = cnt as u32;
                    let mut u3: u16 = FUN_004784d0() as u16;
                    let mut u4: u32 = 0;
                    if u3 / 6 != 0 {
                        u4 = (u3 as u32) % ((u3 / 6) as u32);
                    }
                    let mut b = c4 != 0;
                    c4 = c4.wrapping_sub(1);
                    if b {
                        u3 = 0x6f;
                        loop {
                            load_prefix(c0);
                            let s2 = load_kind();
                            let p10: *const u32 = if s2 == 1 {
                                (&raw const DAT_00523af8).cast::<u32>()
                            } else {
                                (&raw const DAT_00523aec).cast::<u32>()
                            };
                            p40.add(4)
                                .cast::<u32>()
                                .write_unaligned(p10.add(1).read_unaligned());
                            *p40.add(8) = *p10.add(2).cast::<i8>();
                            let src = p10.cast::<i8>();
                            *p40 = *src;
                            *p40.add(1) = *src.add(1);
                            *p40.add(2) = *src.add(2);
                            *p40.add(3) = (u4 as i8).wrapping_add(0x31);
                            let u12_0 = (u4 as u16).wrapping_add(1);
                            c_strcat(c0, p40);
                            FUN_00409bf0(c0, u3);
                            let mut u12 = u12_0;
                            if 5 < u12 {
                                u12 = 0;
                            }
                            u4 = u12 as u32;
                            b = c4 != 0;
                            c4 = c4.wrapping_sub(1);
                            u3 = u3.wrapping_add(0x61);
                            if !b {
                                break;
                            }
                        }
                    }
                }
            }
        }
        if DAT_0085e797 != 0 {
            load_prefix(c0);
            let src = (&raw const DAT_00525588).cast::<i8>();
            let n = c_strlen(src) + 1;
            let mut i = 0usize;
            while i < n {
                *p80.add(i) = *src.add(i);
                i += 1;
            }
            let u4 = (c_strlen(p80) + 1) as u32;
            let mut i_var5: i32 = 0;
            let i_var8 = u4.wrapping_sub(1) as i32;
            *p80.add(u4.wrapping_sub(4) as usize) = 0x6c;
            *p80.add(u4.wrapping_sub(3) as usize) = 0x73;
            *p80.add(u4.wrapping_sub(2) as usize) = 0x74;
            let mut found = false;
            loop {
                if *p80 == 0x5c {
                    found = true;
                    break;
                }
                if i_var8 <= i_var5 {
                    break;
                }
                let i_var7 = i_var5.wrapping_add(1);
                i_var5 = i_var5.wrapping_add(1);
                *p80 = *p80.add(i_var7 as usize);
            }
            if found && i_var5 < i_var8 {
                i_var5 = i_var5.wrapping_add(1);
            } else {
                i_var5 = 0;
            }
            let len = c_strlen(c0);
            let mut i_var7 = len as i32;
            if i_var5 < i_var8 {
                let cnt = (i_var8 - i_var5) as u32;
                let s = p80.add(i_var5 as usize);
                let d = c0.add(len);
                let mut i = 0usize;
                while i < cnt as usize {
                    *d.add(i) = *s.add(i);
                    i += 1;
                }
                i_var7 = (i_var7 as u32).wrapping_add(cnt) as i32;
            }
            *c0.add(i_var7 as usize) = 0;
            FUN_00409bf0(c0, 0x5c);
        }
    }
}
