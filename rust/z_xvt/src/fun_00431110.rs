// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00733380: i32;
    static mut DAT_00733384: [u32; 0x80];
    static mut DAT_00733584: u32;
    static mut DAT_00733588: i32;
    static mut DAT_0073358c: i32;
    static mut DAT_00733590: [u32; 0x80];
    static mut DAT_00733790: u32;
    static mut DAT_00733794: i32;
    static mut DAT_007337a0: [u32; 0];
    static mut DAT_007337a8: [u32; 0];
    static mut DAT_007337b0: [u32; 0];
    static mut DAT_007337b4: [i32; 0];
    static mut DAT_007337b8: [u8; 0];
    static mut DAT_007337b9: [u8; 0];
    static mut DAT_007339b8: [u32; 0];
    static mut DAT_00739290: u32;
    static mut DAT_0085e78f: i8;
    static mut DAT_007330a0: i32;
    static mut DAT_007330f8: i32;
    static mut DAT_00733074: *mut i32;

    static mut DAT_005251a4: i32;
    static mut DAT_005251a8: i32;
    static mut DAT_005251ac: i32;
    static mut DAT_005251b4: i32;

    static mut DAT_005ce218: [u32; 0];
    static mut DAT_005ce21c: [u32; 0];
    static mut DAT_005ce220: [u8; 0];
    static mut DAT_005ce224: [u8; 0];
    static mut DAT_005ce225: [u8; 0];
    static mut DAT_005ce226: [u8; 0];
    static mut DAT_005ce228: [u8; 0];

    static mut DAT_005bda18: [u32; 0];
    static mut DAT_005bda1c: [u32; 0];
    static mut DAT_005bda20: [u8; 0];
    static mut DAT_005bda24: [u8; 0];
    static mut DAT_005bda25: [u8; 0];
    static mut DAT_005bda26: [u8; 0];
    static mut DAT_005bda28: [u8; 0];

    static mut DAT_00539a08: [u32; 0];
    static mut DAT_00539a0c: [u32; 0];
    static mut DAT_00539a10: [u32; 0];
    static mut DAT_00539a14: [u8; 0];
    static mut DAT_00539a15: [u8; 0];
    static mut DAT_00539a16: [u8; 0];
    static mut DAT_00539a17: [u8; 0];
    static mut DAT_00539a18: [u32; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00433540(param_1: i32) -> u32;
    fn FUN_004336c0();
}

unsafe fn cp(dst: *mut u8, src: *const u8, n: u32) {
    let mut i = 0u32;
    while i < n {
        // SAFETY: the caller supplies buffers valid for n bytes, as the original does.
        unsafe {
            *dst.add(i as usize) = *src.add(i as usize);
        }
        i = i.wrapping_add(1);
    }
}

#[allow(clippy::too_many_arguments)]
unsafe fn common(
    buf: *mut u32,
    p2: *const u32,
    p3: u32,
    b15: bool,
    b3: u8,
    hi: u16,
    cnt: *mut i32,
    flag: *mut i32,
    store: *mut u8,
    len: *mut u32,
) -> i32 {
    // SAFETY: buf is the local 1 KiB buffer; the other pointers are the game's own
    // globals and the caller's packet, used as the original does.
    unsafe {
        let c = cnt.read();
        buf.cast::<u16>().write(hi | (((c as u16) & 0x7f) << 8));
        cnt.write(c.wrapping_add(1));
        if cnt.read() > 0x7f {
            cnt.write(0);
        }
        let mut dst = buf.cast::<u8>().add(2);
        let mut l: i32 = 2;
        if b15 {
            l = 4;
            buf.cast::<i16>().add(1).write((p3 as i16).wrapping_sub(4));
            dst = buf.cast::<u8>().add(4);
        }
        let n = p3.wrapping_sub(4);
        cp(dst, p2.add(1).cast::<u8>(), n);
        let tail = dst.add(n as usize);
        l = n.wrapping_add(l as u32) as i32;
        if b15 {
            if flag.read() == 0 {
                let ln = len.read();
                cp(tail, store, ln);
                l = (l as u32).wrapping_add(len.read()) as i32;
            } else {
                *tail = 0x39;
                l = l.wrapping_add(1);
                flag.write(0);
            }
        }
        store.write(b3);
        cp(store.add(1), p2.add(1).cast::<u8>(), n);
        len.write(p3.wrapping_sub(3));
        l
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00431110(param_1: i32, param_2: *mut u32, param_3: u32) -> u8 {
    // SAFETY: all globals and callees are the game's own memory; param_2 points at
    // the caller's packet of param_3 bytes; the local buffer mirrors the original frame.
    unsafe {
        let mut buf = [0u32; 256];
        let bp = buf.as_mut_ptr();
        let mut local_404: i32 = 0;
        let local_41c: i32;
        if (param_3 as i32) < 4 {
            return 0;
        }
        let u_var1 = param_2.read();
        let b_var3 = (u_var1 as u8) & 0x7f;
        let mut u_var10 = b_var3 as u16;
        let b15: bool = !(u_var1 >= 0x3c && u_var1 <= 0x3f);
        let flag1 = u_var1 == 1 && DAT_0085e78f == 1;
        let mut u_var7_idx: u32 = 0;

        if flag1 {
            local_41c = common(
                bp,
                param_2,
                param_3,
                b15,
                b_var3,
                0x8081,
                &raw mut DAT_0073358c,
                &raw mut DAT_00733794,
                (&raw mut DAT_00733590).cast::<u8>(),
                &raw mut DAT_00733790,
            );
        } else if param_1 == 0 {
            local_41c = common(
                bp,
                param_2,
                param_3,
                b15,
                b_var3,
                u_var10,
                &raw mut DAT_00733380,
                &raw mut DAT_00733588,
                (&raw mut DAT_00733384).cast::<u8>(),
                &raw mut DAT_00733584,
            );
        } else if DAT_007330a0 == param_1 {
            local_41c = common(
                bp,
                param_2,
                param_3,
                b15,
                b_var3,
                u_var10 | 0x8080,
                &raw mut DAT_0073358c,
                &raw mut DAT_00733794,
                (&raw mut DAT_00733590).cast::<u8>(),
                &raw mut DAT_00733790,
            );
        } else {
            let u_var7 = FUN_00433540(param_1);
            u_var7_idx = u_var7;
            if u_var7 < DAT_00739290 && u_var7 < 0x28 {
                let p = (&raw mut DAT_007337b4)
                    .cast::<i32>()
                    .add(u_var7.wrapping_mul(0x8e) as usize);
                let i_var9 = p.read();
                let i_var5 = i_var9.wrapping_add(1);
                p.write(i_var5);
                u_var10 |= ((i_var9 as u16) & 0x7f) << 8;
                if 0x7f < i_var5 {
                    p.write(0);
                }
            }
            bp.cast::<u16>().write(u_var10 | 0x8000);
            let mut l: i32 = 2;
            let mut dst = bp.cast::<u8>().add(2);
            if b15 {
                bp.cast::<i16>()
                    .add(1)
                    .write((param_3 as i16).wrapping_sub(4));
                dst = bp.cast::<u8>().add(4);
                l = 4;
            }
            let n = param_3.wrapping_sub(4);
            cp(dst, param_2.add(1).cast::<u8>(), n);
            l = n.wrapping_add(l as u32) as i32;
            if b15 {
                let u_var6 = (&raw mut DAT_007339b8)
                    .cast::<u32>()
                    .add(u_var7.wrapping_mul(0x8e) as usize)
                    .read();
                let src = (&raw mut DAT_007337b8)
                    .cast::<u8>()
                    .add(u_var7.wrapping_mul(0x238) as usize);
                cp(dst.add(n as usize), src, u_var6);
                let again = (&raw mut DAT_007339b8)
                    .cast::<u32>()
                    .add(u_var7.wrapping_mul(0x8e) as usize)
                    .read();
                l = (l as u32).wrapping_add(again) as i32;
            }
            local_41c = l;
            (&raw mut DAT_007337b8)
                .cast::<u8>()
                .add(u_var7.wrapping_mul(0x238) as usize)
                .write(b_var3);
            cp(
                (&raw mut DAT_007337b9)
                    .cast::<u8>()
                    .add(u_var7.wrapping_mul(0x238) as usize),
                param_2.add(1).cast::<u8>(),
                n,
            );
            (&raw mut DAT_007339b8)
                .cast::<u32>()
                .add(u_var7.wrapping_mul(0x8e) as usize)
                .write(param_3.wrapping_sub(3));
        }
        let _ = u_var7_idx;

        let local_400 = bp.cast::<u16>().read();
        let u_var10 = local_400;

        if !flag1 && DAT_007330f8 != param_1 {
            let b3h = (local_400 >> 8) as u8;
            if u_var1 == 2 {
                let a8 = DAT_005251a8 as u32;
                cp(
                    (&raw mut DAT_005ce228)
                        .cast::<u8>()
                        .offset(a8.wrapping_mul(0x210) as i32 as isize),
                    param_2.cast::<u8>(),
                    param_3,
                );
                let i_var9 = DAT_005251a8;
                let i_var5 = (DAT_005251a8 as u32).wrapping_mul(0x210) as i32;
                (&raw mut DAT_005ce218)
                    .cast::<u32>()
                    .offset((DAT_005251a8 as u32).wrapping_mul(0x84) as i32 as isize)
                    .write(param_1 as u32);
                (&raw mut DAT_005ce21c)
                    .cast::<u32>()
                    .offset((i_var9 as u32).wrapping_mul(0x84) as i32 as isize)
                    .write(param_3);
                (&raw mut DAT_005ce220)
                    .cast::<u8>()
                    .offset(i_var5 as isize)
                    .cast::<u32>()
                    .write_unaligned(0);
                (&raw mut DAT_005ce224)
                    .cast::<u8>()
                    .offset(i_var5 as isize)
                    .write(0);
                (&raw mut DAT_005ce225)
                    .cast::<u8>()
                    .offset(i_var5 as isize)
                    .write(0);
                (&raw mut DAT_005ce226)
                    .cast::<u8>()
                    .offset(i_var5 as isize)
                    .write(b3h & 0x7f);
                DAT_005251a8 = DAT_005251a8.wrapping_add(1);
                if 0xff < DAT_005251a8 {
                    DAT_005251a8 = 0;
                }
            }
            let a4 = DAT_005251a4 as u32;
            cp(
                (&raw mut DAT_005bda28)
                    .cast::<u8>()
                    .offset(a4.wrapping_mul(0x210) as i32 as isize),
                param_2.cast::<u8>(),
                param_3,
            );
            let i_var9 = DAT_005251a4;
            let i_var5 = (DAT_005251a4 as u32).wrapping_mul(0x210) as i32;
            (&raw mut DAT_005bda18)
                .cast::<u32>()
                .offset((DAT_005251a4 as u32).wrapping_mul(0x84) as i32 as isize)
                .write(param_1 as u32);
            (&raw mut DAT_005bda1c)
                .cast::<u32>()
                .offset((i_var9 as u32).wrapping_mul(0x84) as i32 as isize)
                .write(param_3);
            (&raw mut DAT_005bda20)
                .cast::<u8>()
                .offset(i_var5 as isize)
                .cast::<u32>()
                .write_unaligned(0);
            (&raw mut DAT_005bda24)
                .cast::<u8>()
                .offset(i_var5 as isize)
                .write(0);
            let i_var9 = DAT_007330a0;
            if param_1 == 0 {
                (&raw mut DAT_005bda25)
                    .cast::<u8>()
                    .offset(i_var5 as isize)
                    .write(0);
            } else {
                (&raw mut DAT_005bda25)
                    .cast::<u8>()
                    .offset(i_var5 as isize)
                    .write(2);
                if i_var9 != param_1 {
                    (&raw mut DAT_005bda25)
                        .cast::<u8>()
                        .offset(i_var5 as isize)
                        .write(1);
                }
            }
            let i_var9 = DAT_005251a4.wrapping_add(1);
            DAT_005251a4 = i_var9;
            (&raw mut DAT_005bda26)
                .cast::<u8>()
                .offset(i_var5 as isize)
                .write(b3h & 0x7f);
            if 0x7f < i_var9 {
                DAT_005251a4 = 0;
            }
        }

        'lab: {
            if DAT_007330f8 == param_1
                || param_1 == 0
                || DAT_00733074.is_null()
                || DAT_007330a0 == param_1
            {
                if 0x3ff < DAT_005251b4 {
                    FUN_004336c0();
                    if 0x3ff < DAT_005251b4 {
                        break 'lab;
                    }
                }
                let ac = DAT_005251ac as u32;
                cp(
                    (&raw mut DAT_00539a18)
                        .cast::<u32>()
                        .offset(ac.wrapping_mul(0x84) as i32 as isize)
                        .cast::<u8>(),
                    param_2.cast::<u8>(),
                    param_3,
                );
                let i_var9 = DAT_005251ac;
                let i_var5 = (DAT_005251ac as u32).wrapping_mul(0x210) as i32;
                (&raw mut DAT_00539a08)
                    .cast::<u32>()
                    .offset((DAT_005251ac as u32).wrapping_mul(0x84) as i32 as isize)
                    .write(DAT_007330f8 as u32);
                (&raw mut DAT_00539a0c)
                    .cast::<u32>()
                    .offset((i_var9 as u32).wrapping_mul(0x84) as i32 as isize)
                    .write(param_3);
                (&raw mut DAT_00539a10)
                    .cast::<u32>()
                    .offset((i_var9 as u32).wrapping_mul(0x84) as i32 as isize)
                    .write(0);
                (&raw mut DAT_00539a14)
                    .cast::<u8>()
                    .offset(i_var5 as isize)
                    .write(0);
                (&raw mut DAT_00539a17)
                    .cast::<u8>()
                    .offset(i_var5 as isize)
                    .write(0);
                let u_var6 = FUN_00433540(DAT_007330f8);
                let u_var7 = DAT_00739290;
                let val = ((u_var10 & 0x7f00) >> 8) as u32;
                let off15 = (DAT_005251ac as u32).wrapping_mul(0x210) as i32 as isize;
                if flag1 {
                    let _b15 = (u_var6 < DAT_00739290) as u8;
                    (&raw mut DAT_00539a15).cast::<u8>().offset(off15).write(2);
                    (&raw mut DAT_007337b0)
                        .cast::<u32>()
                        .add(u_var6.wrapping_mul(0x8e) as usize)
                        .write(val);
                } else if param_1 == 0 {
                    (&raw mut DAT_00539a15).cast::<u8>().offset(off15).write(0);
                    if u_var6 < u_var7 && u_var6 < 0x28 {
                        (&raw mut DAT_007337a8)
                            .cast::<u32>()
                            .add(u_var6.wrapping_mul(0x8e) as usize)
                            .write(val);
                    }
                } else if DAT_007330a0 == param_1 {
                    let _b15 = (u_var6 < DAT_00739290) as u8;
                    (&raw mut DAT_00539a15).cast::<u8>().offset(off15).write(2);
                    (&raw mut DAT_007337b0)
                        .cast::<u32>()
                        .add(u_var6.wrapping_mul(0x8e) as usize)
                        .write(val);
                } else {
                    let _b15 = (u_var6 < DAT_00739290) as u8;
                    (&raw mut DAT_00539a15).cast::<u8>().offset(off15).write(1);
                    (&raw mut DAT_007337a0)
                        .cast::<u32>()
                        .add(u_var6.wrapping_mul(0x8e) as usize)
                        .write(val);
                }
                DAT_005251b4 = DAT_005251b4.wrapping_add(1);
                (&raw mut DAT_00539a16)
                    .cast::<u8>()
                    .offset((DAT_005251ac as u32).wrapping_mul(0x210) as i32 as isize)
                    .write(((local_400 >> 8) as u8) & 0x7f);
                DAT_005251ac = DAT_005251ac.wrapping_add(1);
                if 0x3ff < DAT_005251ac {
                    DAT_005251ac = 0;
                }
            }
        }

        if DAT_00733074.is_null() {
            return 1;
        }
        if DAT_007330f8 != param_1 {
            let vt = DAT_00733074.read() as usize as *const u8;
            let f = vt
                .add(0x68)
                .cast::<*const core::ffi::c_void>()
                .read_unaligned();
            let f: unsafe extern "stdcall" fn(*mut i32, i32, i32, i32, *mut u16, i32) -> i32 =
                core::mem::transmute(f);
            local_404 = f(
                DAT_00733074,
                DAT_007330f8,
                param_1,
                0,
                bp.cast::<u16>(),
                local_41c,
            );
        }
        (local_404 == 0) as u8
    }
}
