// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00556928: i32;
    static mut DAT_00556934: u8;
    static mut DAT_00556935: u8;
    static mut DAT_00556936: u8;
    static mut _DAT_00556941: [u8; 4];
    static mut _DAT_00556945: [u8; 4];
    static mut _DAT_00556965: [u8; 4];
    static mut DAT_00521be4: [i8; 1];
    static mut DAT_00521c5c: [i8; 1];
    static mut DAT_00521c74: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00443d10(param_1: i32) -> i32;
    fn FUN_004438b0(param_1: *const i8, param_2: *const i8);
    fn FUN_004444f0(param_1: i32);
    fn FUN_00443890(param_1: *const i8);
    fn malloc(size: u32) -> *mut core::ffi::c_void;
    fn free(p: *mut core::ffi::c_void);
}

unsafe fn rd32(base: *const u8, off: usize) -> u32 {
    // SAFETY: caller supplies a valid pointer into the game's image structure.
    unsafe { base.byte_add(off).cast::<u32>().read_unaligned() }
}

unsafe fn ld(p: *mut i32, i: isize) -> i32 {
    // SAFETY: caller guarantees p + i lies within a game buffer.
    unsafe { p.wrapping_offset(i).read_unaligned() }
}

unsafe fn st(p: *mut i32, i: isize, v: i32) {
    // SAFETY: caller guarantees p + i lies within a game buffer.
    unsafe { p.wrapping_offset(i).write_unaligned(v) }
}

unsafe fn q(piv6: *mut i32, v: i32) -> u32 {
    let t = v.wrapping_add(8);
    let idx = t.wrapping_add(((t >> 31) as u32 & 0xf) as i32) >> 4;
    // SAFETY: table built by the caller covers the index range.
    unsafe { ld(piv6, idx as isize) as u32 }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004446c0(param_1: i32) -> u32 {
    let base = param_1 as usize as *mut u8;
    // SAFETY: all pointers are the game's own image structure and buffers
    // allocated here; accesses mirror the original's.
    unsafe {
        if FUN_00443d10(param_1) == 0 {
            return 1;
        }
        let mem = malloc(0x100000) as *mut u32;
        let mem0 = malloc(
            rd32(base, 0x1030)
                .wrapping_mul(3)
                .wrapping_add(6)
                .wrapping_mul(8),
        ) as *mut i32;
        let mem1 = malloc(0x7fc) as *mut u8;
        let mem2 = malloc(0x300) as *mut u8;
        let mut u_var10: u32 = 0;
        if mem.is_null() || mem0.is_null() || mem1.is_null() || mem2.is_null() {
            FUN_004438b0(
                (&raw mut DAT_00521c74).cast::<i8>(),
                (&raw mut DAT_00521be4).cast::<i8>(),
            );
            return 1;
        }
        {
            let mut pu = mem;
            let mut n = 0x40000;
            while n != 0 {
                pu.write_unaligned(0xffffffff);
                pu = pu.wrapping_add(1);
                n -= 1;
            }
        }
        let mut pv15 = mem0;
        if (rd32(base, 0x1030).wrapping_add(2) & 0x7fffffff) != 0 {
            loop {
                pv15.write_unaligned(0);
                u_var10 = u_var10.wrapping_add(1);
                pv15.wrapping_add(1).write_unaligned(0);
                pv15.wrapping_add(2).write_unaligned(0);
                pv15 = pv15.wrapping_add(3);
                if !(u_var10 < rd32(base, 0x1030).wrapping_mul(2).wrapping_add(4)) {
                    break;
                }
            }
        }
        u_var10 = rd32(base, 0x1054) >> 4;
        if u_var10 < 2 {
            u_var10 = 1;
        }
        let pi6 = mem1.wrapping_add(0x3fc) as *mut i32;
        let u_var7 = 0x100u32 / u_var10;
        let mut i_var5: i32 = 0;
        let mut pi11 = pi6;
        let mut pi9 = pi6;
        let mut u_var16: u32 = 0;
        let mut u_var8 = u_var7;
        while u_var8 != 0 {
            pi9.write_unaligned(i_var5);
            pi9 = pi9.wrapping_add(1);
            pi11.write_unaligned(0u32.wrapping_sub(i_var5 as u32) as i32);
            i_var5 = i_var5.wrapping_add(1);
            pi11 = pi11.wrapping_sub(1);
            u_var16 = u_var7;
            u_var8 -= 1;
        }
        if 3 < u_var10 {
            u_var10 = 0x300u32 / u_var10;
            if u_var16 < u_var10 {
                pi9 = pi6.wrapping_add(u_var16 as usize);
                pi11 = pi6.wrapping_sub(u_var16 as usize);
                loop {
                    pi9.write_unaligned(i_var5);
                    pi9 = pi9.wrapping_add(1);
                    pi11.write_unaligned(0u32.wrapping_sub(i_var5 as u32) as i32);
                    let u8_ = !u_var16;
                    u_var16 = u_var16.wrapping_add(1);
                    i_var5 = (i_var5 as u32).wrapping_add(u8_ & 1) as i32;
                    pi11 = pi11.wrapping_sub(1);
                    if !(u_var16 < u_var10) {
                        break;
                    }
                }
            }
        }
        if (u_var16 as i32) < 0x100 {
            pi9 = pi6.wrapping_add(u_var16 as usize);
            let mut i_var13 = 0x100u32.wrapping_sub(u_var16) as i32;
            pi11 = pi6.wrapping_sub(u_var16 as usize);
            loop {
                pi9.write_unaligned(i_var5);
                pi9 = pi9.wrapping_add(1);
                i_var13 = i_var13.wrapping_sub(1);
                pi11.write_unaligned(0u32.wrapping_sub(i_var5 as u32) as i32);
                pi11 = pi11.wrapping_sub(1);
                if i_var13 == 0 {
                    break;
                }
            }
        }
        let mut i5: i32 = 0;
        loop {
            mem2.wrapping_offset(i5 as isize).write(0);
            mem2.wrapping_offset(i5 as isize + 0x100).write(i5 as u8);
            let i13 = i5 + 1;
            mem2.wrapping_offset(i5 as isize + 0x200).write(0xff);
            i5 = i13;
            if !(i13 < 0x100) {
                break;
            }
        }
        let mut local_24: u32 = 0;
        if rd32(base, 0x1034) != 0 {
            loop {
                let w = rd32(base, 0x1030);
                let tmp = rd32(base, 0x1086);
                let mut local_2c =
                    tmp.wrapping_add(w.wrapping_mul(local_24).wrapping_mul(6)) as usize as *mut u8;
                let mut pi11 = mem0.wrapping_add(w.wrapping_mul(3).wrapping_add(9) as usize);
                let step =
                    ((0u32.wrapping_sub((local_24 & 1 == 0) as u32) & 2).wrapping_sub(1)) as i32;
                let mut pi9 = mem0.wrapping_add(3);
                if step < 0 {
                    local_2c = local_2c.wrapping_add(w.wrapping_mul(6).wrapping_sub(6) as usize);
                    pi11 = mem0.wrapping_add(w.wrapping_mul(3) as usize);
                    pi9 = pi11.wrapping_add(w.wrapping_mul(3).wrapping_add(6) as usize);
                }
                let mut local_10: u32 = 0;
                if w != 0 {
                    loop {
                        let l0 = local_2c.read() as u32;
                        let l1 = local_2c.wrapping_add(1).read() as u32;
                        let l2 = local_2c.wrapping_add(2).read() as u32;
                        let b1 = mem2
                            .wrapping_add(
                                q(pi6, ld(pi9, 0)).wrapping_add(l0).wrapping_add(0x100) as usize
                            )
                            .read();
                        let l1 = {
                            let _ = l1;
                            local_2c.wrapping_add(1).read() as u32
                        };
                        let b2 = mem2
                            .wrapping_add(
                                q(pi6, ld(pi9, 1)).wrapping_add(l1).wrapping_add(0x100) as usize
                            )
                            .read();
                        let l2 = {
                            let _ = l2;
                            local_2c.wrapping_add(2).read() as u32
                        };
                        let b3 = mem2
                            .wrapping_add(
                                q(pi6, ld(pi9, 2)).wrapping_add(l2).wrapping_add(0x100) as usize
                            )
                            .read();
                        let idx = ((((b3 as u32 & 0xfffffffc) << 6) | (b2 as u32 & 0xfffffffc))
                            << 4)
                            | (b1 >> 2) as u32;
                        if (mem.wrapping_add(idx as usize).read_unaligned() as i32) < 0 {
                            let mut local_32: i16 = 7;
                            let mut i_var5 = DAT_00556928;
                            loop {
                                let sh = (local_32 as u8 & 0x1f) as u32;
                                let u16_ =
                                    ((((b1 as u32 >> sh) & 1) * 2 | ((b2 as u32 >> sh) & 1)) * 2)
                                        | ((b3 as u32 >> sh) & 1);
                                let flag = (i_var5 as u32).wrapping_add(2) as usize as *const u8;
                                if (1u32.wrapping_shl(u16_ as u8 as u32) & flag.read() as u32) == 0
                                {
                                    break;
                                }
                                local_32 = local_32.wrapping_sub(1);
                                i_var5 = rd32(
                                    (i_var5 as u32)
                                        .wrapping_add(0x32)
                                        .wrapping_add(u16_.wrapping_mul(4))
                                        as usize as *const u8,
                                    0,
                                ) as i32;
                                if local_32 == 0 {
                                    break;
                                }
                            }
                            (&raw mut _DAT_00556941)
                                .cast::<u8>()
                                .cast::<u32>()
                                .write_unaligned(0);
                            (&raw mut _DAT_00556945)
                                .cast::<u8>()
                                .cast::<u32>()
                                .write_unaligned(0x41080000);
                            DAT_00556934 = b1;
                            DAT_00556935 = b2;
                            DAT_00556936 = b3;
                            let t2 =
                                rd32((i_var5 as u32).wrapping_add(0x2e) as usize as *const u8, 0);
                            FUN_004444f0(t2 as i32);
                            let v = (&raw mut _DAT_00556965).cast::<u32>().read_unaligned();
                            mem.wrapping_add(idx as usize).write_unaligned(v);
                        }
                        let u_var4 = mem
                            .wrapping_add(idx as usize)
                            .cast::<u16>()
                            .read_unaligned();
                        let palette = rd32(base, 0x104c) as i32;
                        if rd32(base, 0x1024) as i32 == 2 {
                            local_2c
                                .wrapping_add(4)
                                .cast::<u16>()
                                .write_unaligned(u_var4);
                        } else {
                            let i = (u_var4 as u32).wrapping_mul(9);
                            let pp = (palette as u32).wrapping_add(i) as usize as *const u8;
                            local_2c.write(pp.read());
                            local_2c.wrapping_add(1).write(pp.wrapping_add(1).read());
                            local_2c.wrapping_add(2).write(pp.wrapping_add(2).read());
                        }
                        local_2c = local_2c.wrapping_offset(step as isize * 6);
                        let pb = (u_var4 as u32)
                            .wrapping_mul(9)
                            .wrapping_add(rd32(base, 0x104c))
                            as usize as *const u8;
                        let e0 = (b1 as u32).wrapping_sub(pb.read() as u32) as i32;
                        let e1 = (b2 as u32).wrapping_sub(pb.wrapping_add(1).read() as u32) as i32;
                        let e2 = (b3 as u32).wrapping_sub(pb.wrapping_add(2).read() as u32) as i32;
                        st(pi9, 0, 0);
                        st(pi9, 1, 0);
                        st(pi9, 2, 0);
                        pi9 = pi9.wrapping_offset(step as isize * 3);
                        st(pi9, 0, ld(pi9, 0).wrapping_add(e0.wrapping_mul(7)));
                        st(pi9, 1, ld(pi9, 1).wrapping_add(e1.wrapping_mul(7)));
                        pi11 = pi11.wrapping_offset(step as isize * -3);
                        st(pi9, 2, ld(pi9, 2).wrapping_add(e2.wrapping_mul(7)));
                        st(pi11, 0, ld(pi11, 0).wrapping_add(e0.wrapping_mul(3)));
                        st(pi11, 1, ld(pi11, 1).wrapping_add(e1.wrapping_mul(3)));
                        st(pi11, 2, ld(pi11, 2).wrapping_add(e2.wrapping_mul(3)));
                        pi11 = pi11.wrapping_offset(step as isize * 3);
                        st(pi11, 0, ld(pi11, 0).wrapping_add(e0.wrapping_mul(5)));
                        st(pi11, 1, ld(pi11, 1).wrapping_add(e1.wrapping_mul(5)));
                        st(pi11, 2, ld(pi11, 2).wrapping_add(e2.wrapping_mul(5)));
                        pi11 = pi11.wrapping_offset(step as isize * 3);
                        st(pi11, 0, ld(pi11, 0).wrapping_add(e0));
                        st(pi11, 1, ld(pi11, 1).wrapping_add(e1));
                        local_10 = local_10.wrapping_add(1);
                        st(pi11, 2, ld(pi11, 2).wrapping_add(e2));
                        if !(local_10 < rd32(base, 0x1030)) {
                            break;
                        }
                    }
                }
                FUN_00443890((&raw mut DAT_00521c5c).cast::<i8>());
                local_24 = local_24.wrapping_add(1);
                if !(local_24 < rd32(base, 0x1034)) {
                    break;
                }
            }
        }
        free(mem2.cast());
        free(mem1.cast());
        free(mem0.cast());
        free(mem.cast());
        0
    }
}
