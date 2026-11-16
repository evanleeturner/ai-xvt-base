// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa615e: u8;
    static mut DAT_00a91ca0: i32;
    static mut DAT_00b6a5aa: u8;
    static mut DAT_00b6a5a6: u8;
    static mut DAT_00b6a2c0: u32;
    static mut DAT_00b69ccc: u16;
    static mut DAT_00b69d20: i8;
    static mut DAT_00b69e30: i32;
    static mut DAT_00b6a270: u8;
    static mut DAT_00aa5e10: i32;
    static mut DAT_00aa5cb0: i32;
    static mut DAT_00aa5e68: i32;
    static mut DAT_00aa5ac0: i32;
    static mut DAT_0052a6b0: i8;
    static mut DAT_0052c780: i8;
    static mut DAT_0052b46c: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d1400() -> i32;
    fn FUN_004ddc10(param_1: i32) -> *mut i8;
    fn FUN_004db4e0(param_1: i32, param_2: *mut i8, param_3: i32, param_4: i32, param_5: u32);
    fn FUN_004d11e0() -> i32;
    fn FUN_004d6460(
        param_1: *mut core::ffi::c_void,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
    );
    fn FUN_004d4cf0(param_1: *mut core::ffi::c_void);
    fn FUN_004d4d20(param_1: *mut core::ffi::c_void);
    fn sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_004ee8e0(param_1: i32) -> u32;
    fn FUN_004ee510(param_1: i32) -> i32;
    fn FUN_004ee650(param_1: i32) -> i8;
    fn FUN_004ee7c0(param_1: i32) -> i8;
    fn FUN_004db7d0(
        param_1: i32,
        param_2: *mut i8,
        param_3: *mut i32,
        param_4: i32,
        param_5: i32,
        param_6: u32,
    );
}

unsafe fn name_row(
    p: *mut u8,
    flag: *mut i32,
    y: i32,
    param_1: i32,
    uvar9: u16,
    l30: *mut core::ffi::c_void,
    l20: *mut core::ffi::c_void,
) {
    // SAFETY: the callees and globals are the game's own; p and flag point
    // into the game's record tables.
    unsafe {
        FUN_004d6460(l30, 0x58, y, 0xe2, y.wrapping_add(0xf));
        FUN_004d4cf0(l20);
        FUN_004d4d20(l30);
        let i_var8 = (p as usize as i32).wrapping_sub(0xe);
        let buf = &raw mut DAT_00b69d20;
        let u_var5: u32;
        if flag.read() == 0 {
            let rec1 = p.byte_add(4).cast::<i32>().read_unaligned();
            let pc = FUN_004ddc10(rec1.wrapping_add(0x9a));
            sprintf(
                buf,
                (&raw mut DAT_0052a6b0) as *const i8,
                6i32,
                pc,
                1u32,
                i_var8,
            );
            let r = FUN_004d11e0();
            let rec0 = p.cast::<i32>().read_unaligned();
            if r == rec0 || DAT_00b69e30 == 2 {
                let off = ((param_1 % 0x18) as u32 & 0xffff_fffe).wrapping_mul(2);
                u_var5 = (&raw mut DAT_00b6a270)
                    .byte_add(off as usize)
                    .cast::<u32>()
                    .read_unaligned();
            } else {
                u_var5 = uvar9 as u32;
            }
        } else {
            let rec1 = p.byte_add(4).cast::<i32>().read_unaligned();
            let pc = FUN_004ddc10(rec1.wrapping_add(0x9a));
            sprintf(buf, (&raw mut DAT_0052c780) as *const i8, pc);
            u_var5 = uvar9 as u32;
        }
        FUN_004db4e0(0xc, buf, 0x58, y, u_var5);
    }
}

unsafe fn cols(idx: i32, y: i32, u: u32) {
    // SAFETY: the callees are the game's own.
    unsafe {
        let u4 = FUN_004ee8e0(idx);
        let pc = FUN_004ddc10(u4.wrapping_add(0x15) as i32);
        FUN_004db4e0(0xc, pc, 0xe4, y, u);
        let v = FUN_004ee510(idx);
        let pc = FUN_004ddc10(v.wrapping_add(0x244));
        FUN_004db4e0(0xc, pc, 0x152, y, u);
        let c = FUN_004ee650(idx);
        let pc = FUN_004ddc10((c as u8 as u32).wrapping_add(0x24e) as i32);
        FUN_004db4e0(0xc, pc, 0x17a, y, u);
        let c = FUN_004ee7c0(idx);
        let pc = FUN_004ddc10((c as u8 as u32).wrapping_add(0x253) as i32);
        FUN_004db4e0(0xc, pc, 0x193, y, u);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004ecb90(param_1: i32) {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers index the game's tables as the original does.
    unsafe {
        let mut local_30 = [0u32; 4];
        let mut local_20 = [0u32; 4];
        let mut local_10 = [0i32; 4];
        let l30 = local_30.as_mut_ptr().cast::<core::ffi::c_void>();
        let l20 = local_20.as_mut_ptr().cast::<core::ffi::c_void>();

        let t = FUN_004d1400() as u32;
        let h = (t.wrapping_mul(0xf) as i32) >> 1;
        let mut i_var2 = 0u32.wrapping_sub(h as u32) as i32;
        let i_var8 = i_var2.wrapping_add(0x155);
        let u = DAT_00b6a2c0;
        let pc = FUN_004ddc10(0xba);
        FUN_004db4e0(0xc, pc, 0x58, i_var8, u);
        let u = DAT_00b6a2c0;
        let pc = FUN_004ddc10(0x240);
        FUN_004db4e0(0xc, pc, 0xe4, i_var8, u);
        let u = DAT_00b6a2c0;
        let pc = FUN_004ddc10(0x241);
        FUN_004db4e0(0xc, pc, 0x152, i_var8, u);
        let u = DAT_00b6a2c0;
        let pc = FUN_004ddc10(0x242);
        FUN_004db4e0(0xc, pc, 0x17a, i_var8, u);
        let u = DAT_00b6a2c0;
        let pc = FUN_004ddc10(0x243);
        i_var2 = i_var2.wrapping_add(0x164);
        FUN_004db4e0(0xc, pc, 0x193, i_var8, u);
        FUN_004d1400();

        let base = &raw mut DAT_00aa615e;
        let flags = &raw mut DAT_00a91ca0;
        let mode = (&raw mut DAT_00b6a5aa).cast::<i32>().read_unaligned();

        if mode == 0 || mode == 3 || mode == 4 {
            for idx in 0..8i32 {
                let p = base.byte_add(0x2a * idx as usize);
                let fl = flags.add(idx as usize);
                let rec0 = p.cast::<i32>().read_unaligned();
                if rec0 != 0 && FUN_004d11e0() == rec0 {
                    let mut uvar9 = DAT_00b69ccc;
                    if fl.read() == 0 {
                        uvar9 = DAT_00b6a2c0 as u16;
                    }
                    name_row(p, fl, i_var2, param_1, uvar9, l30, l20);
                    FUN_004d4d20(l20);
                    cols(idx, i_var2, uvar9 as u32);
                    i_var2 = i_var2.wrapping_add(0xf);
                }
            }
            for idx in 0..8i32 {
                let p = base.byte_add(0x2a * idx as usize);
                let fl = flags.add(idx as usize);
                let rec0 = p.cast::<i32>().read_unaligned();
                if rec0 != 0 && FUN_004d11e0() != rec0 {
                    let mut i_var13 = 0i32;
                    let mut sel = (&raw mut DAT_00b6a5a6).cast::<i32>().read_unaligned();
                    let i_var8 = (&raw mut DAT_00aa5e10).offset(sel as isize).read();
                    if 0 < i_var8 {
                        loop {
                            sel = (&raw mut DAT_00b6a5a6).cast::<i32>().read_unaligned();
                            let r0 = p.cast::<i32>().read_unaligned();
                            let o = sel.wrapping_mul(8).wrapping_add(i_var13);
                            if (&raw mut DAT_00aa5cb0).offset(o as isize).read() == r0 {
                                break;
                            }
                            i_var13 = i_var13.wrapping_add(1);
                            if !(i_var13 < i_var8) {
                                break;
                            }
                        }
                    }
                    if i_var13 != i_var8 {
                        let mut uvar9 = DAT_00b69ccc;
                        if fl.read() == 0 {
                            uvar9 = DAT_00b6a2c0 as u16;
                        }
                        name_row(p, fl, i_var2, param_1, uvar9, l30, l20);
                        FUN_004d4d20(l20);
                        cols(idx, i_var2, uvar9 as u32);
                        i_var2 = i_var2.wrapping_add(0xf);
                    }
                }
            }
            for idx in 0..8i32 {
                let p = base.byte_add(0x2a * idx as usize);
                let fl = flags.add(idx as usize);
                let rec0 = p.cast::<i32>().read_unaligned();
                if rec0 != 0 && FUN_004d11e0() != rec0 {
                    let mut i_var8 = 0i32;
                    let mut sel = (&raw mut DAT_00b6a5a6).cast::<i32>().read_unaligned();
                    let i_var14 = (&raw mut DAT_00aa5e10).offset(sel as isize).read();
                    let mut b_var12 = i_var14 == 0;
                    if 0 < i_var14 {
                        loop {
                            sel = (&raw mut DAT_00b6a5a6).cast::<i32>().read_unaligned();
                            let r0 = p.cast::<i32>().read_unaligned();
                            let o = sel.wrapping_mul(8).wrapping_add(i_var8);
                            if (&raw mut DAT_00aa5cb0).offset(o as isize).read() == r0 {
                                break;
                            }
                            i_var8 = i_var8.wrapping_add(1);
                            if !(i_var8 < i_var14) {
                                break;
                            }
                        }
                        b_var12 = i_var8 == i_var14;
                    }
                    if b_var12 {
                        let mut uvar9 = DAT_00b69ccc;
                        if fl.read() == 0 {
                            uvar9 = DAT_00b6a2c0 as u16;
                        }
                        name_row(p, fl, i_var2, param_1, uvar9, l30, l20);
                        FUN_004d4d20(l20);
                        FUN_004d6460(
                            local_10.as_mut_ptr().cast(),
                            0xe4,
                            i_var2,
                            0x193,
                            i_var2.wrapping_add(0xe),
                        );
                        let pc = FUN_004ddc10(599);
                        i_var2 = i_var2.wrapping_add(0xf);
                        FUN_004db7d0(0xc, pc, local_10.as_mut_ptr(), 1, 0, uvar9 as u32);
                    }
                }
            }
            return;
        }

        for idx in 0..8i32 {
            let p = base.byte_add(0x2a * idx as usize);
            let fl = flags.add(idx as usize);
            let rec0 = p.cast::<i32>().read_unaligned();
            if rec0 != 0 && FUN_004d11e0() == rec0 {
                let mut uvar9 = DAT_00b69ccc;
                if fl.read() == 0 {
                    uvar9 = DAT_00b6a2c0 as u16;
                }
                name_row(p, fl, i_var2, param_1, uvar9, l30, l20);
                FUN_004d4d20(l20);
                cols(idx, i_var2, uvar9 as u32);
                i_var2 = i_var2.wrapping_add(0xf);
            }
        }
        for idx in 0..8i32 {
            let p = base.byte_add(0x2a * idx as usize);
            let fl = flags.add(idx as usize);
            let rec0 = p.cast::<i32>().read_unaligned();
            let mut i_var14 = i_var2;
            if rec0 != 0 && FUN_004d11e0() != rec0 {
                let mut uvar9 = DAT_00b69ccc;
                if fl.read() == 0 {
                    uvar9 = DAT_00b6a2c0 as u16;
                }
                i_var14 = 0;
                let mut b_var12 = false;
                let mut local_38 = 0i32;
                if 0 < DAT_00aa5e68 {
                    let mut pi6 = &raw mut DAT_00aa5cb0;
                    let mut l44 = &raw mut DAT_00aa5e10;
                    loop {
                        let mut i_var13 = 0i32;
                        let i_var8 = l44.read();
                        let mut pi7 = pi6;
                        if 0 < i_var8 {
                            loop {
                                let r0 = p.cast::<i32>().read_unaligned();
                                if pi7.read() == r0 {
                                    let o = i_var14.wrapping_add(i_var13);
                                    if (&raw mut DAT_00aa5ac0).offset(o as isize).read() == -1 {
                                        b_var12 = true;
                                    }
                                    break;
                                }
                                i_var13 = i_var13.wrapping_add(1);
                                pi7 = pi7.add(1);
                                if !(i_var13 < i_var8) {
                                    break;
                                }
                            }
                            if i_var13 < i_var8 {
                                break;
                            }
                        }
                        i_var14 = i_var14.wrapping_add(8);
                        local_38 = local_38.wrapping_add(1);
                        pi6 = pi6.add(8);
                        l44 = l44.add(1);
                        if !(local_38 < DAT_00aa5e68) {
                            break;
                        }
                    }
                }
                i_var14 = i_var2.wrapping_add(0xf);
                name_row(p, fl, i_var2, param_1, uvar9, l30, l20);
                FUN_004d4d20(l20);
                let u5 = uvar9 as u32;
                if b_var12 {
                    let e = &raw mut DAT_0052b46c;
                    FUN_004db4e0(0xc, e, 0xe4, i_var2, u5);
                    FUN_004db4e0(0xc, e, 0x152, i_var2, u5);
                    FUN_004db4e0(0xc, e, 0x17a, i_var2, u5);
                    FUN_004db4e0(0xc, e, 0x193, i_var2, u5);
                } else {
                    cols(idx, i_var2, u5);
                }
            }
            i_var2 = i_var14;
        }
    }
}
