// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0078072e: [u8; 4];
    static mut DAT_00780726: [u8; 4];
    static mut DAT_00780732: [u8; 4];
    static mut DAT_00780736: [u8; 4];
    static mut DAT_00780c7a: [u8; 4];
    static mut DAT_0081b5be: [u8; 4];
    static mut DAT_00815ac6: [u8; 0];
    static mut DAT_0078076a: [u8; 0];
    static mut DAT_0078077a: [u8; 0];
    static mut DAT_0064f070: [i32; 0];
    static mut DAT_0064f430: [i32; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ca450();
    fn FUN_004cb830(param_1: i32, param_2: *mut core::ffi::c_void, param_3: u32);
    fn FUN_004cddb0(param_1: i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetTickCount() -> u32;
}

unsafe fn rd(p: *const u8) -> u32 {
    // SAFETY: caller supplies a readable address.
    unsafe { p.cast::<u32>().read_unaligned() }
}

unsafe fn wr(p: *mut u8, v: u32) {
    // SAFETY: caller supplies a writable address.
    unsafe { p.cast::<u32>().write_unaligned(v) }
}

unsafe fn cnt(mut p: *const u8) -> u32 {
    let mut n: u32 = 0;
    // SAFETY: p points at a NUL-terminated string.
    unsafe {
        loop {
            n = n.wrapping_add(1);
            let c = p.read();
            p = p.add(1);
            if c == 0 {
                break;
            }
        }
    }
    n
}

unsafe fn copy(mut dst: *mut u8, mut src: *const u8, n: u32) {
    // SAFETY: caller supplies valid src/dst ranges of n bytes.
    unsafe {
        let mut k = n >> 2;
        while k != 0 {
            wr(dst, rd(src));
            src = src.add(4);
            dst = dst.add(4);
            k -= 1;
        }
        let mut k = n & 3;
        while k != 0 {
            dst.write(src.read());
            src = src.add(1);
            dst = dst.add(1);
            k -= 1;
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004cca00(param_1: i32, param_2: i32) {
    // SAFETY: all globals are the game's own memory; param_2 points at the
    // caller's message block; table accesses follow the original's strides.
    unsafe {
        let p_072e = (&raw mut DAT_0078072e).cast::<u8>();
        let p_0726 = (&raw mut DAT_00780726).cast::<u8>();
        let p_0732 = (&raw mut DAT_00780732).cast::<u8>();
        let p_0736 = (&raw mut DAT_00780736).cast::<u8>();
        let p_0c7a = (&raw mut DAT_00780c7a).cast::<u8>();
        let p_b5be = (&raw mut DAT_0081b5be).cast::<u8>();
        let t_5ac6 = (&raw mut DAT_00815ac6).cast::<u8>();
        let t_076a = (&raw mut DAT_0078076a).cast::<u8>();
        let t_077a = (&raw mut DAT_0078077a).cast::<u8>();
        let t_f070 = (&raw mut DAT_0064f070).cast::<i32>();
        let t_f430 = (&raw mut DAT_0064f430).cast::<i32>();

        let m4 = param_2.wrapping_add(4) as *const i32;
        let m8 = param_2.wrapping_add(8) as *const i32;

        if param_1 == 3 {
            if rd(p_072e) == 0 {
                wr(p_0732, 1);
                FUN_004ca450();
                return;
            }
            let i_a = rd(p_0726) as i32;
            if m4.read_unaligned() == 1 && m8.read_unaligned() != i_a {
                let mut au_buf = [0u8; 0x200];
                let b = au_buf.as_mut_ptr();
                wr(p_0732, 1);
                FUN_004ca450();
                wr(b.add(4), rd(p_0732));
                wr(b.add(8), rd(p_b5be));
                wr(b, 0x3b);
                wr(b.add(12), GetTickCount());
                let mut u_var8: u32 = 0;
                if rd(p_b5be) != 0 {
                    let mut p2 = t_5ac6;
                    let mut p3 = b.add(16);
                    loop {
                        u_var8 = u_var8.wrapping_add(1);
                        wr(p3, rd(p2));
                        p3.add(4).write(p2.add(12).read());
                        p3.add(5).write(p2.add(20).read());
                        p3.add(6).write(p2.add(16).read());
                        p3.add(7).write(p2.add(24).read());
                        p2 = p2.wrapping_add(0x238);
                        p3 = p3.add(8);
                        if !(u_var8 < rd(p_b5be)) {
                            break;
                        }
                    }
                }
                let n = rd(p_b5be);
                FUN_004cb830(
                    m8.read_unaligned(),
                    b.cast(),
                    n.wrapping_mul(8).wrapping_add(0x10),
                );
                return;
            }
        } else {
            if param_1 == 5 {
                if rd(p_072e) == 0 {
                    let i_a = rd(p_0726) as i32;
                    if m4.read_unaligned() == 1 && m8.read_unaligned() == i_a {
                        let mut au_buf = [0u8; 0x200];
                        let b = au_buf.as_mut_ptr();
                        wr(b, 0x46);
                        let i_b = rd(p_0c7a) as i32;
                        FUN_004cb830(i_b, b.cast(), 4);
                    }
                } else if m4.read_unaligned() == 1 {
                    let mut u_var8: u32 = 0;
                    if rd(p_0732) != 0 {
                        let mut p9 = t_077a;
                        loop {
                            let i_a = rd(p9) as i32;
                            if i_a == m8.read_unaligned() {
                                let i_b = rd(t_077a
                                    .wrapping_add((u_var8 as usize).wrapping_mul(0x28))
                                    .wrapping_add(4))
                                    as i32;
                                if i_b != 0 {
                                    wr(p_0736, 1);
                                }
                                break;
                            }
                            p9 = p9.wrapping_add(0x28);
                            u_var8 = u_var8.wrapping_add(1);
                            if !(u_var8 < rd(p_0732)) {
                                break;
                            }
                        }
                    }
                    FUN_004cddb0(m8.read_unaligned());
                    u_var8 = 0;
                    if rd(p_b5be) != 0 {
                        let mut p9 = t_5ac6;
                        loop {
                            let i_a = rd(p9) as i32;
                            if i_a != m8.read_unaligned() {
                                p9 = p9.wrapping_add(0x238);
                                u_var8 = u_var8.wrapping_add(1);
                                if rd(p_b5be) <= u_var8 {
                                    break;
                                }
                                continue;
                            }
                            let u_cnt = rd(p_b5be).wrapping_sub(1);
                            wr(p_b5be, u_cnt);
                            let mut p2 = t_5ac6.wrapping_add((u_cnt as usize).wrapping_mul(0x238));
                            let mut p3 = t_5ac6.wrapping_add((u_var8 as usize).wrapping_mul(0x238));
                            let mut k = 0x8e;
                            while k != 0 {
                                wr(p3, rd(p2));
                                p2 = p2.add(4);
                                p3 = p3.add(4);
                                k -= 1;
                            }
                            let ent =
                                || t_5ac6.wrapping_add((rd(p_b5be) as usize).wrapping_mul(0x238));
                            wr(ent(), 0);
                            wr(ent().add(0x04), 0x7f);
                            wr(ent().add(0x0c), 0x7f);
                            wr(ent().add(0x14), 0x7f);
                            wr(ent().add(0x08), 0x7f);
                            wr(ent().add(0x10), 0x7f);
                            wr(ent().add(0x18), 0x7f);
                            wr(ent().add(0x1c), 0);
                            ent().add(0x20).write(0x39);
                            wr(ent().add(0x220), 1);
                            wr(ent().add(0x234), 0);
                            wr(ent().add(0x224), 0);
                            wr(ent().add(0x228), 0);
                            wr(ent().add(0x22c), 0);
                            wr(ent().add(0x230), 0);
                            break;
                        }
                    }
                    let mut i_var6: usize = 0;
                    let mut p_tbl = t_f070;
                    loop {
                        if p_tbl.read() == m8.read_unaligned() {
                            let e = t_f070.add(i_var6 * 6);
                            e.write(0);
                            e.add(1).write(0);
                            e.add(2).write(0);
                            e.add(3).write(0);
                            e.add(4).write(0);
                            e.add(5).write(0);
                            break;
                        }
                        p_tbl = p_tbl.wrapping_add(6);
                        i_var6 += 1;
                        if !(p_tbl < t_f430) {
                            break;
                        }
                    }
                }
                wr(p_0732, 1);
                FUN_004ca450();
                return;
            }
            if param_1 != 0x103 {
                return;
            }
            if m4.read_unaligned() == 1 {
                let mut u_var8: u32 = 0;
                if rd(p_0732) != 0 {
                    let mut pc7 = t_076a;
                    loop {
                        let i_a = rd(pc7.add(0x10)) as i32;
                        if i_a == m8.read_unaligned() {
                            let src = param_2.wrapping_add(0x1c) as *const u8;
                            let u4 = cnt(src);
                            copy(pc7, src, u4);
                            let u4b = cnt(pc7);
                            let src2 =
                                (param_2.wrapping_add(0x1c).wrapping_add(u4b as i32)) as *const u8;
                            let u5 = cnt(src2);
                            copy(pc7.wrapping_sub(0x10), src2, u5);
                            pc7.add(0xc).write(0);
                            pc7.wrapping_sub(4).write(0);
                        }
                        pc7 = pc7.wrapping_add(0x28);
                        u_var8 = u_var8.wrapping_add(1);
                        if !(u_var8 < rd(p_0732)) {
                            break;
                        }
                    }
                }
            }
        }
    }
}
