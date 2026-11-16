// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00532a34: i32;
    static mut DAT_00520278: i32;
    static mut DAT_0052027c: i32;
    static mut DAT_00520280: i32;
    static mut DAT_00520284: i32;
    static mut DAT_0085e6f0: u8;
    static mut DAT_0085e7c5: i8;
    static mut DAT_00758824: u32;
    static mut DAT_00757e40: [u8; 0];
    static mut DAT_00532acc: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cc540(
        a: i32,
        b: i32,
        c: i32,
        d: i32,
        buf: *mut core::ffi::c_void,
        count: u32,
        flag: u32,
    ) -> u32;
    fn FUN_004d84f0(a: i32, b: i32, c: i32, d: i32) -> i32;
    fn FUN_004de190(name: *const i8, a: i32, b: i32, c: i32);
    fn FUN_004d8900();
}

#[allow(non_snake_case)]
unsafe fn chk(p: *const u8) -> i32 {
    // SAFETY: p points at four consecutive 32-bit values in the game's table.
    unsafe {
        let g0 = p.cast::<i32>().read_unaligned();
        let g1 = p.byte_add(4).cast::<i32>().read_unaligned();
        let g2 = p.byte_add(8).cast::<i32>().read_unaligned();
        let g3 = p.byte_add(12).cast::<i32>().read_unaligned();
        FUN_004d84f0(g0, g1, g2, g3)
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004d8520() -> u32 {
    let mut local_buf: [i32; 384] = [0; 384];
    let mut b_var3: i32 = 0;
    // SAFETY: globals and callees are the game's own; all pointers address
    // the game's entry table or the local buffer, accessed as the original does.
    unsafe {
        let base: *mut u8 = (&raw mut DAT_00757e40).cast::<u8>();
        let local_5e0: *mut i32 = local_buf.as_mut_ptr().add(8);

        if DAT_00532a34 != -1 {
            return 0;
        }
        let mut s_var4: u32 = FUN_004cc540(
            DAT_00520278,
            DAT_0052027c,
            DAT_00520280,
            DAT_00520284,
            local_buf.as_mut_ptr().cast::<core::ffi::c_void>(),
            0x20,
            DAT_0085e6f0 as u32,
        );
        let mut local_60c: u32 = 0;
        if DAT_00758824 != 0 {
            let mut pg: *mut u8 = base.add(0x20);
            loop {
                let i_var5 = chk(pg);
                if i_var5 != 0 {
                    let mut s_var13: u32 = 0;
                    if s_var4 != 0 {
                        let mut pi15: *mut i32 = local_5e0;
                        loop {
                            let i_var6 = FUN_004d84f0(
                                pi15.read(),
                                pi15.add(1).read(),
                                pi15.add(2).read(),
                                pi15.add(3).read(),
                            );
                            if i_var6 != 0 && i_var6 == i_var5 {
                                let o = (s_var13 as usize).wrapping_mul(0xc);
                                local_5e0.add(o).write(0);
                                local_5e0.add(o + 1).write(0);
                                local_5e0.add(o + 2).write(0);
                                local_5e0.add(o + 3).write(0);
                                break;
                            }
                            pi15 = pi15.add(0xc);
                            s_var13 = s_var13.wrapping_add(1);
                            if !(s_var13 < s_var4) {
                                break;
                            }
                        }
                    }
                    if s_var13 == s_var4 {
                        core::ptr::write_bytes(pg.sub(0x20), 0, 0x3b);
                    }
                }
                pg = pg.add(0x3b);
                local_60c = local_60c.wrapping_add(1);
                if !(local_60c < DAT_00758824) {
                    break;
                }
            }
        }
        if s_var4 != 0 {
            let mut pi9: *mut i32 = local_5e0;
            loop {
                let i_var5 = FUN_004d84f0(
                    pi9.read(),
                    pi9.add(1).read(),
                    pi9.add(2).read(),
                    pi9.add(3).read(),
                );
                if i_var5 != 0 {
                    let off = (DAT_00758824.wrapping_mul(0x3b)) as usize;
                    let name = pi9.sub(8).cast::<u8>();
                    let mut len: usize = 0;
                    while name.add(len).read() != 0 {
                        len += 1;
                    }
                    let n = len + 1;
                    let dst = base.add(off);
                    let mut k = 0usize;
                    while k < n {
                        dst.add(k).write(name.add(k).read());
                        k += 1;
                    }
                    let i_var6 = pi9.add(1).read();
                    let i_var1 = pi9.add(2).read();
                    b_var3 = 1;
                    let g0 = pi9.read();
                    let g3 = pi9.add(3).read();
                    base.add(off + 0x20).cast::<i32>().write_unaligned(g0);
                    base.add(off + 0x24).cast::<i32>().write_unaligned(i_var6);
                    base.add(off + 0x28).cast::<i32>().write_unaligned(i_var1);
                    base.add(off + 0x2c).cast::<i32>().write_unaligned(g3);
                    base.add(off + 0x30).cast::<i32>().write_unaligned(9);
                    base.add(off + 0x34).cast::<i32>().write_unaligned(0);
                    base.add(off + 0x38).write(2);
                    base.add(off + 0x39).write(0);
                    DAT_00758824 = DAT_00758824.wrapping_add(1);
                }
                pi9 = pi9.add(0xc);
                s_var4 = s_var4.wrapping_sub(1);
                if s_var4 == 0 {
                    break;
                }
            }
        }
        if b_var3 != 0 && DAT_0085e7c5 != 0 {
            FUN_004de190((&raw const DAT_00532acc).cast::<i8>(), 1, 0, 0xff);
        }
        let mut u_var7: u32 = 0;
        if DAT_00758824 != 0 {
            let mut pg: *mut u8 = base.add(0x20);
            loop {
                let i_var5 = chk(pg);
                let u_var8 = DAT_00758824;
                if i_var5 == 0 {
                    let mut u_var11 = u_var7.wrapping_add(1);
                    if u_var11 < DAT_00758824 {
                        let mut pk: *mut u8 = base.add(0x20).add((u_var11 as usize) * 0x3b);
                        loop {
                            let r = chk(pk);
                            if r == 0 {
                                pk = pk.add(0x3b);
                                u_var11 = u_var11.wrapping_add(1);
                                if DAT_00758824 <= u_var11 {
                                    break;
                                }
                                continue;
                            }
                            let ps = base.add((u_var11 as usize) * 0x3b);
                            core::ptr::copy(ps, pg.sub(0x20), 0x3b);
                            core::ptr::write_bytes(ps, 0, 0x3b);
                            break;
                        }
                    }
                }
                pg = pg.add(0x3b);
                u_var7 = u_var7.wrapping_add(1);
                let lim = if i_var5 == 0 { DAT_00758824 } else { u_var8 };
                let _ = lim;
                if !(u_var7 < u_var8_after(i_var5, u_var8)) {
                    break;
                }
            }
        }
        DAT_00758824 = 0;
        loop {
            let off = (DAT_00758824.wrapping_mul(0x3b)) as usize;
            let r = chk(base.add(off + 0x20));
            if r == 0 {
                break;
            }
            DAT_00758824 = DAT_00758824.wrapping_add(1);
            if !(DAT_00758824 < 0x20) {
                break;
            }
        }
        FUN_004d8900();
        DAT_00758824
    }
}

#[inline(always)]
fn u_var8_after(_i: i32, u_var8: u32) -> u32 {
    u_var8
}
