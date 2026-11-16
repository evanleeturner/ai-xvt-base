// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b6a2b0: i32;
    static mut DAT_00b6d962: i32;
    static mut DAT_00664f2c: i32;
    static mut DAT_00b6a5a6: u32;
    static mut DAT_00b6a5aa: u32;
    static mut DAT_00b6a5ae: u32;
    static mut DAT_00b6a606: u32;
    static mut DAT_00b6a60a: u32;
    static mut DAT_00b6d96a: u32;
    static mut DAT_00b6d96e: u32;
    static mut DAT_00b6d972: u32;
    static mut DAT_00b6d9aa: u32;
    static mut DAT_00b6d9ae: u32;
    static mut DAT_00b7e34a: u32;
    static mut DAT_00b7e34e: u32;
    static mut DAT_00b7e352: u32;

    static mut DAT_0052ad74: i8;
    static mut DAT_0052b530: i8;
    static mut DAT_0052b538: i8;
    static mut DAT_0052b540: i8;
    static mut DAT_0052b548: i8;
    static mut DAT_0052b550: i8;
    static mut DAT_0052b558: i8;
    static mut DAT_0052b560: i8;
    static mut DAT_0052b568: i8;
    static mut DAT_0052b570: i8;
    static mut DAT_0052b578: i8;
    static mut DAT_0052b580: i8;
    static mut DAT_0052b588: i8;
    static mut DAT_0052b590: i8;
    static mut DAT_0052b598: i8;
    static mut DAT_0052b5a0: i8;
    static mut DAT_0052b5a8: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004daba0(param_1: *mut i32);
    fn FUN_004d6460(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_004d64a0(param_1: *mut i32, param_2: i32, param_3: i32);
    fn FUN_004ddc10(param_1: i32) -> *mut i8;
    fn FUN_004da780(
        param_1: *mut i32,
        param_2: *mut i8,
        param_3: *mut i8,
        param_4: *mut i8,
        param_5: i32,
        param_6: u32,
        param_7: i32,
        param_8: *mut i8,
    ) -> i32;
    fn FUN_004daa20(param_1: *mut i32, param_2: *mut i8, param_3: *mut i8, param_4: i32);
    fn FUN_004caf60();
}

#[allow(non_snake_case)]
unsafe fn up_btn(l: *mut i32, s: *mut i8, id: i32, snd: i32) -> i32 {
    // SAFETY: the callees and the string globals are the game's own.
    unsafe {
        let p = FUN_004ddc10(id);
        FUN_004da780(l, s, s, p, 0xc, 0, snd, &raw mut DAT_0052ad74)
    }
}

#[allow(non_snake_case)]
unsafe fn down_btn(l: *mut i32, s: *mut i8, id: i32) {
    // SAFETY: the callees and the string globals are the game's own.
    unsafe {
        let p = FUN_004ddc10(id);
        FUN_004daa20(l, s, p, 0xc);
    }
}

unsafe fn ld(p: *const u8) -> u32 {
    // SAFETY: caller passes the address of a game global.
    unsafe { p.cast::<u32>().read_unaligned() }
}

unsafe fn st(p: *mut u8, v: u32) {
    // SAFETY: caller passes the address of a game global.
    unsafe { p.cast::<u32>().write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004c9660() -> u32 {
    let mut local_30: [i32; 4] = [0; 4];
    let mut local_20: [i32; 8] = [1; 8];
    let l30 = local_30.as_mut_ptr();
    let l20 = local_20.as_mut_ptr();
    // SAFETY: globals, callees and indexed stack slots follow the original;
    // the unaligned globals are accessed unaligned.
    unsafe {
        if DAT_00b6a2b0 == 5 {
            *l20.add(7) = 2;
        } else {
            *l20.offset(DAT_00b6a2b0 as isize) = 2;
        }
        let mut tmp_idx: i32 = (&raw mut DAT_00b6d962).read_unaligned();
        *l20.offset((tmp_idx.wrapping_add(5)) as isize) = 2;
        FUN_004daba0(l20);
        FUN_004d6460(l30, 0x16, 0x14e, 0x2a, 0x166);
        tmp_idx = (&raw mut DAT_00b6d962).read_unaligned();
        if tmp_idx == 1 {
            down_btn(l30, &raw mut DAT_0052b5a0, 0x283);
        } else {
            let r = up_btn(l30, &raw mut DAT_0052b5a8, 0x283, 0x11);
            if r != 0 {
                DAT_00664f2c = 1;
                tmp_idx = 1;
                (&raw mut DAT_00b6d962).write_unaligned(tmp_idx);
                st(
                    (&raw mut DAT_00b6a5a6).cast(),
                    ld((&raw mut DAT_00b7e34a).cast()),
                );
                st(
                    (&raw mut DAT_00b6a5aa).cast(),
                    ld((&raw mut DAT_00b7e34e).cast()),
                );
                let mut s = (&raw mut DAT_00b7e352).cast::<u8>();
                let mut d = (&raw mut DAT_00b6a5ae).cast::<u8>();
                let mut n = 6;
                while n != 0 {
                    st(d, ld(s));
                    s = s.add(4);
                    d = d.add(4);
                    n -= 1;
                }
                let off = (tmp_idx as u32).wrapping_mul(0x109e0) as usize;
                st(
                    (&raw mut DAT_00b6a606).cast(),
                    ld((&raw mut DAT_00b6d9aa).cast::<u8>().wrapping_add(off)),
                );
                st(
                    (&raw mut DAT_00b6a60a).cast(),
                    ld((&raw mut DAT_00b6d9ae).cast::<u8>().wrapping_add(off)),
                );
                FUN_004caf60();
            }
        }
        FUN_004d64a0(l30, 0, -0x1c);
        tmp_idx = (&raw mut DAT_00b6d962).read_unaligned();
        if tmp_idx == 0 {
            down_btn(l30, &raw mut DAT_0052b590, 0x282);
        } else {
            let r = up_btn(l30, &raw mut DAT_0052b598, 0x282, 0x10);
            if r != 0 {
                DAT_00664f2c = 1;
                tmp_idx = 0;
                (&raw mut DAT_00b6d962).write_unaligned(tmp_idx);
                st(
                    (&raw mut DAT_00b6a5a6).cast(),
                    ld((&raw mut DAT_00b6d96a).cast()),
                );
                st(
                    (&raw mut DAT_00b6a5aa).cast(),
                    ld((&raw mut DAT_00b6d96e).cast()),
                );
                let mut s = (&raw mut DAT_00b6d972).cast::<u8>();
                let mut d = (&raw mut DAT_00b6a5ae).cast::<u8>();
                let mut n = 6;
                while n != 0 {
                    st(d, ld(s));
                    s = s.add(4);
                    d = d.add(4);
                    n -= 1;
                }
                let off = (tmp_idx as u32).wrapping_mul(0x109e0) as usize;
                st(
                    (&raw mut DAT_00b6a606).cast(),
                    ld((&raw mut DAT_00b6d9aa).cast::<u8>().wrapping_add(off)),
                );
                st(
                    (&raw mut DAT_00b6a60a).cast(),
                    ld((&raw mut DAT_00b6d9ae).cast::<u8>().wrapping_add(off)),
                );
                FUN_004caf60();
            }
        }
        FUN_004d6460(l30, 0x16, 0xfe, 0x2a, 0x116);
        if DAT_00b6a2b0 == 5 {
            down_btn(l30, &raw mut DAT_0052b588, 0x31c);
        } else if up_btn(l30, &raw mut DAT_0052b580, 0x31c, 0x12) != 0 {
            DAT_00664f2c = 1;
            DAT_00b6a2b0 = 5;
            FUN_004caf60();
        }
        FUN_004d64a0(l30, 0, -0x1c);
        if DAT_00b6a2b0 == 4 {
            down_btn(l30, &raw mut DAT_0052b578, 0x33a);
        } else if up_btn(l30, &raw mut DAT_0052b570, 0x33a, 0xf) != 0 {
            DAT_00664f2c = 1;
            DAT_00b6a2b0 = 4;
            FUN_004caf60();
        }
        FUN_004d64a0(l30, 0, -0x1c);
        if DAT_00b6a2b0 == 3 {
            down_btn(l30, &raw mut DAT_0052b568, 10);
        } else if up_btn(l30, &raw mut DAT_0052b560, 10, 0xe) != 0 {
            DAT_00664f2c = 1;
            DAT_00b6a2b0 = 3;
            FUN_004caf60();
        }
        FUN_004d64a0(l30, 0, -0x1c);
        if DAT_00b6a2b0 == 2 {
            down_btn(l30, &raw mut DAT_0052b558, 8);
        } else if up_btn(l30, &raw mut DAT_0052b550, 8, 0xd) != 0 {
            DAT_00664f2c = 1;
            DAT_00b6a2b0 = 2;
            FUN_004caf60();
        }
        FUN_004d64a0(l30, 0, -0x1c);
        if DAT_00b6a2b0 == 1 {
            down_btn(l30, &raw mut DAT_0052b548, 9);
        } else if up_btn(l30, &raw mut DAT_0052b540, 9, 0xc) != 0 {
            DAT_00664f2c = 1;
            DAT_00b6a2b0 = 1;
            FUN_004caf60();
        }
        FUN_004d64a0(l30, 0, -0x1c);
        if DAT_00b6a2b0 == 0 {
            down_btn(l30, &raw mut DAT_0052b538, 7);
            return 1;
        }
        if up_btn(l30, &raw mut DAT_0052b530, 7, 0xb) != 0 {
            DAT_00664f2c = 1;
            DAT_00b6a2b0 = 0;
            FUN_004caf60();
        }
        1
    }
}
