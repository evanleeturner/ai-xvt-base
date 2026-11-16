// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe8: *mut u8;
    static mut DAT_009a8e18: *mut u8;
    static mut DAT_009e9640: u32;
    static mut DAT_00a082d8: u16;
    static mut DAT_009d8a58: i32;
    static mut DAT_009d8a54: i32;
    static mut DAT_009d8b60: i32;
    static mut DAT_0051a44f: [u8; 1];
    static mut DAT_0051a194: [u8; 1];
    static mut DAT_0051c5b2: [u8; 1];
    static mut DAT_0051c5b4: [u8; 1];
    static mut DAT_0051c5b6: [u8; 1];
    static mut DAT_009e9bfc: [u8; 1];
    static mut DAT_0051a2fc: [u8; 1];
    static mut DAT_0051a394: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00459750(param_1: i32) -> u16;
    fn FUN_0041bb00(param_1: u16);
    fn FUN_004269e0(param_1: u32) -> u32;
    fn FUN_00405860(param_1: i32) -> i16;
    fn FUN_004035d0(param_1: i32, param_2: i16, param_3: i16, param_4: i16);
}

#[inline(always)]
unsafe fn rd<T: Copy>(p: *const u8) -> T {
    // SAFETY: caller guarantees p addresses a readable T in the game's memory.
    unsafe { p.cast::<T>().read_unaligned() }
}

#[inline(always)]
unsafe fn wr<T>(p: *mut u8, v: T) {
    // SAFETY: caller guarantees p addresses a writable T in the game's memory.
    unsafe { p.cast::<T>().write_unaligned(v) }
}

#[inline(always)]
fn off(p: *mut u8, o: i32) -> *mut u8 {
    p.wrapping_offset(o as isize)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004065d0(param_1: i32, param_2: i32, param_3: i32) -> u32 {
    // SAFETY: all globals, tables and records are the game's own memory;
    // accesses mirror the original's offsets and order.
    unsafe {
        let a44f = (&raw mut DAT_0051a44f).cast::<u8>();
        let a194 = (&raw mut DAT_0051a194).cast::<u8>();
        let c5b2 = (&raw mut DAT_0051c5b2).cast::<u8>();
        let c5b4 = (&raw mut DAT_0051c5b4).cast::<u8>();
        let c5b6 = (&raw mut DAT_0051c5b6).cast::<u8>();
        let e9bfc = (&raw mut DAT_009e9bfc).cast::<u8>();
        let a2fc = (&raw mut DAT_0051a2fc).cast::<u8>();
        let a394 = (&raw mut DAT_0051a394).cast::<u8>();

        let mut i_var3: i32 = rd::<i32>(off(
            DAT_009a1fe8,
            0x1b_i32.wrapping_add(param_1.wrapping_mul(0x23)),
        ));
        let u_stack_a: u8;
        let mut u_var15: u16;

        'search: {
            if i_var3 == -1 {
                u_stack_a = 7;
                u_var15 = FUN_00459750(7);
                break 'search;
            }
            u_var15 = ((i_var3 as i16 as i32)
                .wrapping_mul(0xc)
                .wrapping_add(DAT_00a082d8 as i32)) as u16;
            u_stack_a = 6;
            let mut u_var12: u16 = u_var15.wrapping_add(0xc);
            if *a44f.wrapping_offset(param_3 as isize) != 0 {
                u_var15 = u_var15.wrapping_add(8);
            }
            if u_var15 < u_var12 {
                loop {
                    let idx = (u_var15 as u32).wrapping_mul(0x23) as i32;
                    if *off(DAT_009a1fe8, idx.wrapping_add(3)) == 0 {
                        let r = rd::<*mut u8>(off(DAT_009a1fe8, 0x1f_i32.wrapping_add(idx)));
                        wr::<i16>(off(r, 0x85), 0);
                        *off(r, 1) = 0;
                        break;
                    }
                    u_var15 = u_var15.wrapping_add(1);
                    if !(u_var15 < u_var12) {
                        break;
                    }
                }
                if u_var15 < u_var12 {
                    FUN_0041bb00(u_var15);
                    break 'search;
                }
            }
            u_var15 = DAT_00a082d8.wrapping_add(0x60);
            u_var12 = DAT_00a082d8.wrapping_add(0x80);
            if u_var15 < u_var12 {
                loop {
                    let idx = (u_var15 as u32).wrapping_mul(0x23) as i32;
                    if *off(DAT_009a1fe8, idx.wrapping_add(3)) == 0 {
                        let r = rd::<*mut u8>(off(DAT_009a1fe8, 0x1f_i32.wrapping_add(idx)));
                        wr::<i16>(off(r, 0x85), 0);
                        *off(r, 1) = 0;
                        break;
                    }
                    u_var15 = u_var15.wrapping_add(1);
                    if !(u_var15 < u_var12) {
                        break;
                    }
                }
                if u_var15 < u_var12 {
                    FUN_0041bb00(u_var15);
                    break 'search;
                }
            }
            return 0xffff_ffff;
        }

        if u_var15 == 0xffff {
            return 0xffff_ffff;
        }
        let mut i_var16: *mut u8 = off(DAT_009a1fe8, param_1.wrapping_mul(0x23));
        let b_var1: u8 = *off(i_var16, 3);
        let u_var8: u32 = u_var15 as u32;
        i_var3 = u_var8.wrapping_mul(0x23) as i32;

        macro_rules! nd {
            () => {
                rd::<*mut u8>(off(DAT_009a1fe8, 0x1f_i32.wrapping_add(i_var3)))
            };
        }
        macro_rules! sd {
            () => {
                rd::<*mut u8>(off(i_var16, 0x1f))
            };
        }

        *nd!() = 1;
        *off(DAT_009a1fe8, i_var3.wrapping_add(2)) = u_stack_a;
        *off(DAT_009a1fe8, i_var3.wrapping_add(3)) = param_3 as u8;
        wr::<i16>(off(nd!(), 0x83), 1);
        wr::<i16>(off(nd!(), 0x85), param_1 as i16);
        *off(nd!(), 0x87) = b_var1;
        let u_var9: u32 = FUN_004269e0(b_var1 as u32);
        *off(nd!(), 0x88) = *off(sd!(), 0x88);
        wr::<i16>(
            off(DAT_009a1fe8, i_var3.wrapping_add(0x12)),
            rd::<i16>(off(i_var16, 0x12)),
        );
        wr::<i16>(
            off(DAT_009a1fe8, i_var3.wrapping_add(0x14)),
            rd::<i16>(off(i_var16, 0x14)),
        );
        wr::<i16>(
            off(DAT_009a1fe8, i_var3.wrapping_add(0x10)),
            rd::<i16>(off(i_var16, 0x10)),
        );
        let v79 = rd::<i16>(off(sd!(), 0x79))
            .wrapping_add(rd::<i16>(a2fc.wrapping_offset(param_3 as isize * 2 + 10)));
        wr::<i16>(off(nd!(), 0x79), v79);
        let q = (u_var8.wrapping_sub(DAT_009e9640)).wrapping_mul(10) as usize;
        wr::<i16>(
            DAT_009a8e18.wrapping_add(8).wrapping_add(q),
            rd::<i16>(off(nd!(), 0x79)),
        );
        let t194: u32 = rd::<u32>(a194.wrapping_offset(param_3 as isize * 4));
        wr::<u32>(
            off(nd!(), 0x7d),
            (rd::<u16>(off(sd!(), 0x79)) as u32).wrapping_add(t194),
        );
        let p13 = off(nd!(), 0x7d);
        if rd::<u32>(p13) < t194 {
            wr::<u32>(p13, t194);
        }
        let mut s_var7: i16 = FUN_00405860(param_3);
        wr::<i16>(off(nd!(), 0x81), s_var7);
        let mut i_var10: i32 = rd::<i32>(off(i_var16, 4));
        let mut i_var11: i32 = rd::<i32>(off(i_var16, 8));
        let mut local_10: i32 = rd::<i32>(off(i_var16, 0xc));
        let mut i_var14: i32 = ((u_var9 & 0xffff).wrapping_mul(0x101)) as i32;
        let p2 = param_2.wrapping_mul(8);
        s_var7 = rd::<i16>(c5b4.wrapping_offset(p2.wrapping_add(i_var14) as isize));
        i_var14 = i_var14.wrapping_add(p2);
        FUN_004035d0(
            i_var16 as usize as i32,
            rd::<i16>(c5b2.wrapping_offset(i_var14 as isize)),
            s_var7,
            rd::<i16>(c5b6.wrapping_offset(i_var14 as isize)),
        );
        if b_var1 == 0x35 {
            DAT_009d8a58 = DAT_009d8a58.wrapping_mul(2);
            DAT_009d8a54 = DAT_009d8a54.wrapping_mul(2);
            DAT_009d8b60 = DAT_009d8b60.wrapping_mul(2);
        }
        i_var10 = i_var10.wrapping_add(DAT_009d8a58);
        i_var11 = i_var11.wrapping_add(DAT_009d8a54);
        local_10 = local_10.wrapping_add(DAT_009d8b60);
        wr::<i32>(off(nd!(), 6), i_var10);
        wr::<i32>(off(nd!(), 10), i_var11);
        wr::<i32>(off(nd!(), 0xe), local_10);
        let v1b = rd::<i32>(off(i_var16, 0x1b));
        if v1b != -1 {
            let o = (v1b as u32).wrapping_mul(0x5bd) as i32;
            wr::<u32>(off(nd!(), 2), rd::<u32>(e9bfc.wrapping_offset(o as isize)));
        }
        let c2 = *off(i_var16, 2) as i8;
        if *a44f.wrapping_offset(param_3 as isize) == 0 || ((c2 != 4 && c2 != 3) && c2 != 5) {
            let i14 = rd::<i16>(a394.wrapping_offset(param_3 as isize * 2 + 2)) as i32;
            let l4 = (i14 as i64).wrapping_mul(rd::<i16>(off(sd!(), 0x93)) as i32 as i64);
            let l5 = (i14 as i64).wrapping_mul(rd::<i16>(off(sd!(), 0x95)) as i32 as i64);
            let l6 = (i14 as i64).wrapping_mul(rd::<i16>(off(sd!(), 0x97)) as i32 as i64);
            wr::<u32>(
                off(DAT_009a1fe8, i_var3.wrapping_add(4)),
                (i_var10 as u32).wrapping_add(
                    ((l4 as u32) >> 0xf) | (((l4 as u64) >> 0x20) as u32).wrapping_shl(0x11),
                ),
            );
            wr::<u32>(
                off(DAT_009a1fe8, i_var3.wrapping_add(8)),
                (i_var11 as u32).wrapping_add(
                    ((l5 as u32) >> 0xf) | (((l5 as u64) >> 0x20) as u32).wrapping_shl(0x11),
                ),
            );
            wr::<u32>(
                off(DAT_009a1fe8, i_var3.wrapping_add(0xc)),
                (local_10 as u32).wrapping_add(
                    ((l6 as u32) >> 0xf) | (((l6 as u64) >> 0x20) as u32).wrapping_shl(0x11),
                ),
            );
            for o in [
                0x8c, 0x8e, 0x90, 0x99, 0x9b, 0x9d, 0x9f, 0xa1, 0xa3, 0x93, 0x95, 0x97,
            ] {
                wr::<i16>(off(nd!(), o), rd::<i16>(off(sd!(), o)));
            }
            *off(nd!(), 0x92) = 0;
            i_var16 = nd!();
            *off(i_var16, 0x8b) = *off(i_var16, 0x92);
        } else {
            let mut i_var17 = rd::<i16>(a394.wrapping_offset(param_3 as isize * 2 + 2)) as i32;
            if s_var7 < 0 {
                i_var17 = i_var17.wrapping_neg();
                wr::<i16>(
                    off(DAT_009a1fe8, i_var3.wrapping_add(0x12)),
                    0x8000u16 as i16,
                );
            } else {
                wr::<i16>(off(DAT_009a1fe8, i_var3.wrapping_add(0x12)), 0);
            }
            local_10 = local_10.wrapping_add(i_var17);
            *off(nd!(), 0x92) = 1;
            i_var16 = nd!();
            *off(i_var16, 0x8b) = *off(i_var16, 0x92);
            wr::<i32>(off(DAT_009a1fe8, i_var3.wrapping_add(4)), i_var10);
            wr::<i32>(off(DAT_009a1fe8, i_var3.wrapping_add(8)), i_var11);
            wr::<i32>(off(DAT_009a1fe8, i_var3.wrapping_add(0xc)), local_10);
        }
        let d = (u_var15 as i32).wrapping_sub(DAT_009e9640 as i16 as i32) as u16;
        let i_var17: i32 = (d as u32).wrapping_mul(10) as i32;
        *off(DAT_009a8e18, i_var17.wrapping_add(1)) = 0;
        wr::<u16>(off(DAT_009a8e18, i_var17.wrapping_add(4)), 0xffff);
        wr::<u16>(off(DAT_009a8e18, i_var17.wrapping_add(6)), 0);
        wr::<u16>(off(DAT_009a8e18, i_var17.wrapping_add(2)), 0xffff);
        *off(DAT_009a8e18, i_var17) = 0xff;
        wr::<*mut u8>(off(nd!(), 0xa5), off(DAT_009a8e18, i_var17));
        u_var8
    }
}
