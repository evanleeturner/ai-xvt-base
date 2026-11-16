// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a1fe4: u16;
    static mut DAT_009ed664: u16;
    static mut DAT_00a07ce8: u32;
    static mut DAT_009a1fe8: i32;
    static mut DAT_0051a558: [u32; 0];
    static mut DAT_009a8e34: u32;
    static mut _DAT_00518060: f32;
    static mut _DAT_00518080: f32;
    static mut DAT_00a90b20: i32;
    static mut DAT_00528c5c: i32;
    static mut DAT_0054f978: i32;
    static mut DAT_0054f99c: i32;
    static mut DAT_0053f8f0: [u32; 0];
    static mut DAT_0054f998: i32;
    static mut DAT_0052f870: [i32; 0];
    static mut DAT_0052f868: i32;
    static mut DAT_0054f990: i32;
    static mut DAT_0054f994: i32;
    static mut DAT_0054f9a0: i32;
    static mut DAT_0053f970: [f32; 0];
    static mut DAT_0054f988: [u32; 0];
    static mut _DAT_0054f9a4: f32;
    static mut _DAT_0053f974: f32;
    static mut DAT_0051a54c: i32;
    static mut DAT_00527e94: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004079e0();
    fn FUN_0046a760(a: u32, b: u16) -> i32;
    fn FUN_0046a650(a: u32, b: u16) -> i32;
    fn FUN_004096d0(a: i32, b: i32, c: i32);
    fn FUN_00409c10(a: i32, b: i32, c: i32);
    fn FUN_0040a1a0(a: i32, b: i32, c: i32);
    fn FUN_0040a6e0(a: i32, b: i32, c: i32);
    fn FUN_00408140();
    fn FUN_004b1c10();
    fn FUN_004b1c90();
    fn FUN_004b1d10(a: *mut f32, b: i32);
    fn FUN_004b1d70();
    fn FUN_004b1de0(a: *mut u32, b: i32);
    fn FUN_004b2020();
    fn FUN_004b1c50();
    fn FUN_00408110();
    fn FUN_00407af0(a: i32, b: i32, c: *mut u16, d: *mut u8, e: i32) -> *mut u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0040bbf0(
    param_1: u16,
    param_2: i32,
    param_3: i32,
    param_4: u32,
    param_5: i32,
) {
    // SAFETY: all globals, callees and the param_5 structure belong to the game;
    // memory is accessed where the original accesses it.
    unsafe {
        let mut local_3c0: [f32; 24] = [0.0; 24];
        let lp = local_3c0.as_mut_ptr();
        let lpi = lp as usize as i32;
        let mut local_3cc: u32 = 0;
        let local_3e0: f32;

        let tbl = (&raw mut DAT_0053f8f0).cast::<u32>();
        let tbl2 = (&raw mut DAT_0052f870).cast::<i32>();
        let vbuf = (&raw mut DAT_0053f970).cast::<f32>();
        let ibuf = (&raw mut DAT_0054f988).cast::<u32>();

        let mut i_var14: i32 = (DAT_009a1fe4 as u32).wrapping_sub(param_3 as u32) as i32;
        if (DAT_009ed664 as u32) < DAT_00a07ce8 {
            let i_var7 = (DAT_009ed664 as u32)
                .wrapping_mul(0x23)
                .wrapping_add(DAT_009a1fe8 as u32);
            let pc = i_var7 as usize as *const i8;
            if pc.byte_add(2).read() == 13 && (pc.byte_add(0x19).cast::<u8>().read()) < 0x20 {
                let idx = pc.byte_add(0x19).cast::<u8>().read() as usize;
                local_3cc = (&raw mut DAT_0051a558).cast::<u32>().add(idx).read();
            } else {
                local_3cc = 0xffff_ffff;
            }
        }
        if DAT_009a8e34 < 0x1000001 {
            let mut v =
                _DAT_00518060 / ((DAT_009a8e34 as i32 as f32) * _DAT_00518080 + _DAT_00518060);
            if DAT_00a90b20 == 2 {
                v = _DAT_00518060 - v;
            }
            local_3e0 = v;
        } else {
            local_3cc = 0xffff_ffff;
            let mut v = 0.00012205541f32;
            if DAT_00a90b20 == 2 {
                v = 0.9998779f32;
            }
            local_3e0 = v;
        }
        let p5 = param_5 as usize as *const u8;
        let mut i_var7: i32 = p5.byte_add(0x10).cast::<i32>().read_unaligned();
        let mut i_var18: i32 = p5.byte_add(0x14).cast::<i32>().read_unaligned();
        if 0x100 < i_var7 {
            i_var7 = 0x100;
            FUN_004079e0();
        }
        if 0x100 < i_var18 {
            i_var18 = 0x100;
            FUN_004079e0();
        }
        let mut i_var8: i32 = 0;
        let mut i_var15: i32 = 1;
        loop {
            let i_var10 = i_var15;
            i_var15 = i_var10.wrapping_mul(2);
            if i_var7 <= i_var15 {
                break;
            }
            i_var8 += 1;
            if i_var8 >= 8 {
                break;
            }
        }
        i_var8 = 1;
        let mut i_var9: i32 = 0;
        loop {
            i_var8 = i_var8.wrapping_mul(2);
            if i_var18 <= i_var8 {
                break;
            }
            i_var9 += 1;
            if i_var9 >= 8 {
                break;
            }
        }
        if ((DAT_00528c5c as usize + 0x1c) as *const i32).read_unaligned() != 0 {
            if i_var15 <= i_var8 {
                i_var15 = i_var8;
            } else {
                i_var8 = i_var15;
            }
        }
        let w = p5.byte_add(0x10).cast::<i32>().read_unaligned();
        let h = p5.byte_add(0x14).cast::<i32>().read_unaligned();
        let u_var20: u32 = ((((w as u32).wrapping_mul(param_4 & 0xffff)) as i32) >> 9) as u32;
        i_var9 = FUN_0046a760(u_var20, param_1);
        let mut u_var16: u32 = ((((h as u32).wrapping_mul(param_4 & 0xffff)) as i32) >> 9) as u32;
        let mut i_var10 = FUN_0046a650(u_var16, param_1);
        let mut i_var11 = FUN_0046a760(u_var16, param_1);
        let mut i_var12 = FUN_0046a650(u_var20, param_1);
        let mut i_var19: i32 = 0;
        DAT_0054f978 = 4;
        DAT_0054f99c = 4;
        tbl.write(0);
        tbl.add(1).write(1);
        tbl.add(2).write(2);
        tbl.add(3).write(3);
        lp.write(param_2.wrapping_add(i_var10).wrapping_add(i_var9) as f32);
        lp.add(1)
            .write(i_var11.wrapping_sub(i_var12).wrapping_add(i_var14) as f32);
        lp.add(3).write(0.0);
        lp.add(4).write(0.0);
        lp.add(5).write(0.0);
        let mut u_var21: u32 = 0u32.wrapping_sub(u_var20);
        lp.add(2).write(local_3e0);
        i_var10 = FUN_0046a760(u_var21, param_1);
        i_var9 = FUN_0046a650(u_var16, param_1);
        i_var11 = FUN_0046a760(u_var16, param_1);
        i_var12 = FUN_0046a650(u_var21, param_1);
        lp.add(6)
            .write(param_2.wrapping_add(i_var9).wrapping_add(i_var10) as f32);
        lp.add(9).write(0.0);
        lp.add(7)
            .write(i_var11.wrapping_sub(i_var12).wrapping_add(i_var14) as f32);
        lp.add(11).write(0.0);
        lp.add(8).write(local_3e0);
        lp.add(10).write((i_var7 as f32) / (i_var15 as f32));
        i_var10 = FUN_0046a760(u_var21, param_1);
        u_var16 = 0u32.wrapping_sub(u_var16);
        i_var9 = FUN_0046a650(u_var16, param_1);
        i_var11 = FUN_0046a760(u_var16, param_1);
        i_var12 = FUN_0046a650(u_var21, param_1);
        lp.add(12)
            .write(param_2.wrapping_add(i_var9).wrapping_add(i_var10) as f32);
        lp.add(15).write(0.0);
        lp.add(13)
            .write(i_var11.wrapping_sub(i_var12).wrapping_add(i_var14) as f32);
        lp.add(14).write(local_3e0);
        lp.add(16).write((i_var7 as f32) / (i_var15 as f32));
        lp.add(17).write((i_var18 as f32) / (i_var8 as f32));
        i_var7 = FUN_0046a760(u_var20, param_1);
        i_var10 = FUN_0046a650(u_var16, param_1);
        i_var9 = FUN_0046a760(u_var16, param_1);
        i_var11 = FUN_0046a650(u_var20, param_1);
        lp.add(18)
            .write(param_2.wrapping_add(i_var10).wrapping_add(i_var7) as f32);
        lp.add(21).write(0.0);
        lp.add(22).write(0.0);
        lp.add(23).write((i_var18 as f32) / (i_var8 as f32));
        lp.add(20).write(local_3e0);
        DAT_0054f998 = 0;
        lp.add(19)
            .write(i_var9.wrapping_sub(i_var11).wrapping_add(i_var14) as f32);

        if 0 < DAT_0054f978 {
            let mut p = tbl as *const i32;
            i_var14 = tbl.add((DAT_0054f978 - 1) as usize).read() as i32;
            loop {
                i_var7 = p.read();
                p = p.add(1);
                i_var19 += 1;
                FUN_004096d0(i_var14, i_var7, lpi);
                i_var14 = i_var7;
                if i_var19 >= DAT_0054f978 {
                    break;
                }
            }
        }
        i_var14 = 0;
        DAT_0054f978 = 0;
        if 0 < DAT_0054f998 {
            let mut p = tbl2 as *const i32;
            i_var7 = tbl2.add((DAT_0054f998 - 1) as usize).read();
            loop {
                i_var18 = p.read();
                p = p.add(1);
                i_var14 += 1;
                FUN_00409c10(i_var7, i_var18, lpi);
                i_var7 = i_var18;
                if i_var14 >= DAT_0054f998 {
                    break;
                }
            }
        }
        i_var14 = 0;
        DAT_0054f998 = 0;
        if 0 < DAT_0054f978 {
            let mut p = tbl as *const i32;
            i_var7 = tbl.add((DAT_0054f978 - 1) as usize).read() as i32;
            loop {
                i_var18 = p.read();
                p = p.add(1);
                i_var14 += 1;
                FUN_0040a1a0(i_var7, i_var18, lpi);
                i_var7 = i_var18;
                if i_var14 >= DAT_0054f978 {
                    break;
                }
            }
        }
        i_var14 = 0;
        DAT_0054f978 = 0;
        if 0 < DAT_0054f998 {
            let mut p = tbl2 as *const i32;
            i_var7 = tbl2.add((DAT_0054f998 - 1) as usize).read();
            loop {
                i_var18 = p.read();
                p = p.add(1);
                i_var14 += 1;
                FUN_0040a6e0(i_var7, i_var18, lpi);
                i_var7 = i_var18;
                if i_var14 >= DAT_0054f998 {
                    break;
                }
            }
        }
        if 2 < DAT_0054f978 {
            if DAT_0054f990 < DAT_0054f978.wrapping_add(DAT_0054f994)
                || DAT_0052f868 < DAT_0054f9a0.wrapping_add(DAT_0054f978)
            {
                FUN_00408140();
                FUN_004b1c10();
                FUN_004b1c90();
                FUN_004b1d10(vbuf, DAT_0054f994);
                FUN_004b1d70();
                FUN_004b1de0(ibuf, DAT_0054f9a0);
                FUN_004b2020();
                FUN_004b1c50();
                FUN_00408110();
                DAT_0054f9a0 = 0;
                DAT_0054f994 = 0;
            }
            if DAT_0051a54c != 0 {
                local_3cc = 0xfeff_ffff;
                DAT_0051a54c = 0;
            }
            i_var14 = 0;
            if 0 < DAT_0054f978 {
                let mut p = tbl;
                loop {
                    u_var16 = p.read();
                    i_var14 += 1;
                    let b = (u_var16 as usize).wrapping_mul(6);
                    let f1 = lp.add(b + 1).read();
                    let f3 = lp.add(b + 4).read();
                    let f4 = lp.add(b + 2).read();
                    let f5 = lp.add(b + 5).read();
                    let o = (DAT_0054f994.wrapping_mul(8)) as isize;
                    vbuf.offset(o).write(lp.add(b).read() + _DAT_0054f9a4);
                    vbuf.offset(o + 1).write(f1 + _DAT_0053f974);
                    vbuf.offset(o + 2).write(f4);
                    vbuf.offset(o + 3).write(f4);
                    vbuf.offset(o + 6).write(f3);
                    vbuf.offset(o + 7).write(f5);
                    vbuf.offset(o + 4).cast::<u32>().write(local_3cc);
                    vbuf.offset(o + 5).write(0.0);
                    p.write(DAT_0054f994 as u32);
                    DAT_0054f994 = DAT_0054f994.wrapping_add(1);
                    p = p.add(1);
                    if i_var14 >= DAT_0054f978 {
                        break;
                    }
                }
            }
            i_var14 = 2;
            let puvar13 = FUN_00407af0(
                i_var15,
                i_var8,
                (p5.byte_add(0xc)
                    .cast::<i32>()
                    .read_unaligned()
                    .wrapping_add(param_5)) as usize as *mut u16,
                (p5.byte_add(8)
                    .cast::<i32>()
                    .read_unaligned()
                    .wrapping_add(0x10)
                    .wrapping_add(param_5)) as usize as *mut u8,
                p5.byte_add(0x20).cast::<i32>().read_unaligned(),
            );
            if 2 < DAT_0054f978 {
                let mut p = tbl.add(1);
                loop {
                    let o = (DAT_0054f9a0.wrapping_mul(5)) as isize;
                    ibuf.offset(o).write(tbl.read());
                    ibuf.offset(o + 1).write(p.read());
                    ibuf.offset(o + 2).write(p.add(1).read());
                    ibuf.offset(o + 4).write(puvar13 as usize as u32);
                    ibuf.offset(o + 3).write(0x812);
                    if DAT_00527e94 != 0 {
                        let o = (DAT_0054f9a0.wrapping_mul(5)) as isize;
                        let v = ibuf.offset(o + 3).read();
                        ibuf.offset(o + 3).write(v.wrapping_add(0x180));
                    }
                    i_var14 += 1;
                    let o = (DAT_0054f9a0.wrapping_mul(5)) as isize;
                    let v = ibuf.offset(o + 3).read();
                    ibuf.offset(o + 3).write(v.wrapping_add(0x200));
                    DAT_0054f9a0 = DAT_0054f9a0.wrapping_add(1);
                    p = p.add(1);
                    if i_var14 >= DAT_0054f978 {
                        break;
                    }
                }
            }
        }
    }
}
