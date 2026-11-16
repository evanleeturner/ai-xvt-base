// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a0a1d0: u8;
    static mut DAT_009e9670: [u8; 0];
    static mut DAT_009e968a: [u8; 0];
    static mut DAT_009e9697: [u8; 0];
    static mut DAT_009e9698: [u8; 0];
    static mut DAT_009e9a77: [u8; 0];
    static mut DAT_009ecc34: i32;
    static mut DAT_009a1fe8: *mut u8;
    static mut DAT_0051c5a2: [u8; 0];
    static mut DAT_00a08374: u16;
    static mut DAT_00a08cc4: [u8; 0];
    static mut DAT_00a08cc6: [u8; 0];
    static mut DAT_00a08cc8: [u8; 0];
    static mut DAT_00a08cca: [u8; 0];
    static mut DAT_00a08d24: [u8; 0];
    static mut DAT_00a08d26: [u8; 0];
    static mut DAT_00a0a1e6: [u16; 0];
    static mut DAT_009ed240: [u32; 0];
    static mut DAT_005233ec: i32;
    static mut DAT_005233d8: i32;
    static mut DAT_009a806c: i32;
    static mut DAT_009a7bac: i32;
    static mut DAT_009a8c14: i32;
    static mut DAT_009d80c4: unsafe extern "C" fn(u32, i32, u32, u32, i32);
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00425c20(a: u32, b: u32) -> u32;
    fn FUN_0040e0c0(a: i32, b: i32, c: i32, d: i32);
    fn FUN_004a9640(a: i8);
    fn FUN_004a9520(a: i32, b: i32, c: i16, d: i16);
    fn FUN_004a9500(a: i16, b: u16);
    fn FUN_004a95c0(a: i32);
    fn FUN_004a9590(a: i32);
    fn FUN_004277f0(a: u16, b: u32, c: i32);
    fn FUN_00440760(a: u32, b: u32);
    fn FUN_0041ca60(a: u16, b: u16, c: u32) -> u32;
}

unsafe fn rd16(base: *mut u8, off: u32) -> u16 {
    // SAFETY: caller supplies a valid game address.
    unsafe { base.add(off as usize).cast::<u16>().read_unaligned() }
}

unsafe fn class_ok(idx: i32) -> bool {
    if idx == -1 {
        return false;
    }
    // SAFETY: the game's own table pointer.
    let c = unsafe { *DAT_009a1fe8.add((idx as u32).wrapping_mul(0x23).wrapping_add(3) as usize) }
        as i8;
    !(c != 1 && c != 2 && c != 3 && c != 0xe && c != 4)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0043c9a0() {
    // SAFETY: all globals, tables and callees are the game's own memory;
    // reads and writes happen where the original does them.
    unsafe {
        DAT_00a0a1d0 = 0;
        let off = (DAT_009ecc34 as u32).wrapping_mul(0x5bd);
        let rec = (&raw mut DAT_009e9670).cast::<u8>().add(off as usize);
        let tmp_int = rec.cast::<i32>().read_unaligned();
        let p1 = DAT_009a1fe8
            .add((tmp_int as u32).wrapping_mul(0x23).wrapping_add(0x1f) as usize)
            .cast::<u32>()
            .read_unaligned();
        let p2 = (p1 as usize as *const u8)
            .add(0xa9)
            .cast::<u32>()
            .read_unaligned();
        let i_var5 = p2 as usize as *mut u8;

        let mut u_stack_20: u16 = 0;
        let b_var1 = *i_var5.add(4);
        let mut local_26: u32 = (*i_var5.add(0x21d) as u32) << 0x10;
        let mut local_22: i16 = 0;
        let mut u_stack_1e: u16 = 0;
        let mut local_1c: u16 = 0;
        let mut u_stack_1a: u16 = 0;

        if *i_var5.add(0x21d) != 0 {
            loop {
                let u_var11: u32 = ((*(&raw mut DAT_0051c5a2)
                    .cast::<u8>()
                    .add((b_var1 as u32).wrapping_mul(0x101) as usize)
                    as u32)
                    < u_stack_20 as u32) as u32;
                let i_var15: i32 = (u_stack_20 as u32).wrapping_add(DAT_00a08374 as u32) as i32;
                let i_var10: u32 = (i_var15 as u32).wrapping_mul(0xc);
                let s_var3: i16 = rd16((&raw mut DAT_00a08cc4).cast::<u8>(), i_var10) as i16;
                let mut u_var14: u16 = rd16((&raw mut DAT_00a08cc6).cast::<u8>(), i_var10);
                if (s_var3 as i32 as u32).wrapping_add(u_var14 as u32) != 0 || DAT_00a08374 != 0 {
                    let u_var8: u16 = rd16((&raw mut DAT_00a08cc8).cast::<u8>(), i_var10);
                    local_1c = (*i_var5.add(0x371usize + (u_stack_20 as usize) * 4) as i8) as u16;
                    let tmp16 = rd16(i_var5, 0x1e1);
                    if *(&raw mut DAT_009e9a77).cast::<u8>().add(off as usize) == 0
                        && (tmp16 & 2) != 0
                        && (tmp16 & 4) != 0
                    {
                        if (local_1c as i16) < 1 || (*i_var5.add(0x1e5) & 0x10) == 0 {
                            u_stack_1e = 0;
                            local_26 &= 0xffff0000;
                            local_22 = 0;
                        } else {
                            let u_var13 = local_1c.wrapping_add(1);
                            if (u_var13 as i16) < 0x41 {
                                local_26 &= 0xffff0000;
                                local_22 = 1;
                                local_1c = u_var13;
                            } else {
                                local_26 = (local_26 & 0xffff0000) | 1;
                                local_22 = 2;
                                local_1c = local_1c.wrapping_sub(0x3f);
                            }
                            u_stack_1e = local_1c / 6;
                            local_1c = u_var13;
                            if 10 < u_stack_1e {
                                u_stack_1e = 10;
                            }
                        }
                        let tbl = (&raw mut DAT_00a0a1e6)
                            .cast::<u16>()
                            .add(i_var15 as u32 as usize);
                        if tbl.read_unaligned() != u_stack_1e {
                            tbl.write_unaligned(u_stack_1e);
                            let mut s_var7: i16 = 3;
                            if DAT_005233ec != 0x13 {
                                s_var7 =
                                    ((0u16.wrapping_sub((DAT_005233ec == 0x112) as u16)) & 0xfffe)
                                        .wrapping_add(6) as i16;
                            }
                            let s_var4: i16 =
                                rd16((&raw mut DAT_00a08cca).cast::<u8>(), i_var10) as i16;
                            if s_var4 != 0 {
                                s_var7 = 0u32.wrapping_sub(s_var7 as i32 as u32) as i16;
                            }
                            if DAT_00a08374 == 0 {
                                let mut u_var9: u32 = 0;
                                loop {
                                    let mut u_var12: u32 =
                                        ((u_stack_20 as u32) << 16) | (local_22 as u16 as u32);
                                    if u_stack_1e <= u_var9 as u16 {
                                        u_var12 = local_26;
                                    }
                                    let u_var13 = (u_var9 as u16).wrapping_add(1);
                                    let idx = (u_var12 & 0xffff).wrapping_add(u_var8 as u32);
                                    let a0 = (&raw mut DAT_009ed240)
                                        .cast::<u32>()
                                        .add(idx as usize)
                                        .read_unaligned();
                                    let f = DAT_009d80c4;
                                    f(
                                        a0,
                                        u_var9
                                            .wrapping_mul(s_var7 as i32 as u32)
                                            .wrapping_add(s_var3 as i32 as u32)
                                            as i32,
                                        u_var14 as u32,
                                        0xfd,
                                        (s_var4 != 0) as i32,
                                    );
                                    u_var9 = u_var13 as u32;
                                    if !(u_var13 < 10) {
                                        break;
                                    }
                                }
                            } else if rd16((&raw mut DAT_00a08cc8).cast::<u8>(), i_var10) != 0 {
                                let u_var9 = FUN_00425c20(local_1c as u32, 0x7f);
                                FUN_0040e0c0(
                                    DAT_009a806c,
                                    DAT_009a7bac,
                                    DAT_009a8c14,
                                    (DAT_005233d8 as u32).wrapping_mul(DAT_009a7bac as u32) as i32,
                                );
                                FUN_004a9640(2);
                                FUN_004a9520(0, 0, DAT_009a7bac as i16, DAT_009a8c14 as i16);
                                FUN_004a9500(s_var3, u_var14);
                                FUN_004a95c0(0);
                                FUN_004a9590(0x4a);
                                let o2 = (DAT_00a08374 as u32)
                                    .wrapping_add(u_stack_20 as u32)
                                    .wrapping_mul(0xc);
                                FUN_004277f0(
                                    ((u_var9 & 0xffff) / 0x28f) as u16,
                                    rd16((&raw mut DAT_00a08cc8).cast::<u8>(), o2) as u32,
                                    1,
                                );
                                FUN_0040e0c0(0, 0x140, 200, 0);
                            }
                        }
                    }

                    if (local_1c as i16) < 1 || (*i_var5.add(0x1e5) & 0x10) == 0 {
                        u_stack_1a = 0;
                        u_var14 = 0;
                    } else if *(&raw mut DAT_009e9698).cast::<u8>().add(off as usize) != 0
                        || (*(&raw mut DAT_009e9697).cast::<u8>().add(off as usize) as u32)
                            != u_var11
                    {
                        u_stack_1a = 1;
                        u_var14 = 0;
                    } else {
                        let sel = *i_var5.add(0x220usize + u_var11 as usize);
                        let a224 = *i_var5.add(0x224usize + u_var11 as usize);
                        match sel {
                            0 | 4 => u_stack_1a = 0,
                            1 => {
                                u_stack_1a =
                                    ((0u16.wrapping_sub((a224 as u16 == u_stack_20) as u16)) & 2)
                                        .wrapping_add(1);
                            }
                            2 => {
                                let u_var9 = a224 as u32;
                                if u_var9 == u_stack_20 as u32
                                    || ((3 < (local_26 >> 16) as u16)
                                        && u_var9.wrapping_sub(u_stack_20 as u32) == 0xfffffffe)
                                {
                                    u_stack_1a = 3;
                                } else {
                                    u_stack_1a = 1;
                                }
                            }
                            3 => u_stack_1a = 3,
                            _ => {}
                        }
                        u_var14 = u_stack_1a;
                        if u_stack_1a == 3
                            && rd16(i_var5, 0x226u32.wrapping_add(u_var11.wrapping_mul(2))) != 0
                        {
                            u_stack_1a = 5;
                            u_var14 = 2;
                        }
                    }
                    if local_22 == 2 {
                        u_stack_1a = u_stack_1a.wrapping_add(1);
                    }
                    let cur = rec.cast::<i32>().read_unaligned();
                    let b_var6 = class_ok(cur);
                    let tmp16 = rd16(i_var5, 0x1e1);
                    if b_var6
                        && *(&raw mut DAT_009e9a77).cast::<u8>().add(off as usize) == 0
                        && (tmp16 & 2) != 0
                        && (tmp16 & 4) != 0
                    {
                        let o2 = (u_stack_20 as u32)
                            .wrapping_add(DAT_00a08374 as u32)
                            .wrapping_mul(0xc);
                        let s = (rd16((&raw mut DAT_00a08d24).cast::<u8>(), o2) as u32)
                            .wrapping_add(rd16((&raw mut DAT_00a08d26).cast::<u8>(), o2) as u32);
                        if s != 0 {
                            FUN_00440760(
                                (DAT_00a08374 as u32)
                                    .wrapping_add(u_stack_20 as u32)
                                    .wrapping_add(0xb),
                                u_stack_1a as u32,
                            );
                        }
                    }
                    FUN_00440760(
                        (DAT_00a08374 as u32)
                            .wrapping_add(u_stack_20 as u32)
                            .wrapping_add(0x35),
                        u_var14 as u32,
                    );
                    let tmp16 = rd16(i_var5, 0x1e5);
                    let u_var8b = tmp16 & 4;
                    let mut set_one = false;
                    if u_var8b == 0 || u_var14 != 3 {
                        if u_var8b == 0 {
                            u_var14 = 0;
                        } else if u_var14 == 2 {
                            set_one = true;
                        }
                    } else {
                        u_var14 = rd16((&raw mut DAT_009e968a).cast::<u8>(), off);
                        if u_var14 == 0xffff {
                            set_one = true;
                        } else {
                            let ti = rec.cast::<i32>().read_unaligned();
                            let call_res = FUN_0041ca60(
                                ti as u16,
                                u_var14,
                                ((u_stack_1e as u32) << 16) | (u_stack_20 as u32),
                            );
                            if (call_res as i16) == 0 {
                                set_one = true;
                            } else {
                                DAT_00a0a1d0 = 1;
                                u_var14 = 2;
                            }
                        }
                    }
                    if set_one {
                        u_var14 = 1;
                    }
                    if u_var14 == 3 {
                        u_var14 = 0;
                    }
                    let cur = rec.cast::<i32>().read_unaligned();
                    if class_ok(cur) {
                        FUN_00440760(
                            (DAT_00a08374 as u32)
                                .wrapping_add(u_stack_20 as u32)
                                .wrapping_add(0x3d),
                            u_var14 as u32,
                        );
                    }
                }
                u_stack_20 = u_stack_20.wrapping_add(1);
                if !(u_stack_20 < (local_26 >> 16) as u16) {
                    break;
                }
            }
        }
    }
}
