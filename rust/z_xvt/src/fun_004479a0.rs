// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4988: i32;
    static mut DAT_00732e54: i32;
    static mut DAT_006db798: i32;
    static mut DAT_0071b704: i32;
    static mut DAT_0071ecf4: i32;
    static mut s_w1pln_00525b86: [u8; 1];
    static mut DAT_00709310: [u32; 1];
    static mut DAT_0070ad48: [u8; 1];
    static mut DAT_0070ad44: [u8; 1];
    static mut DAT_0070b215: [u8; 1];
    static mut DAT_0071b7bd: [u8; 1];
    static mut DAT_0071b79d: [u8; 1];
    static mut DAT_0071b74a: [u8; 1];
    static mut DAT_0071b759: [u8; 1];
    static mut DAT_0071b776: [u8; 1];
    static mut DAT_0071b777: [u8; 1];
    static mut DAT_0071bcdd: [u8; 1];
    static mut DAT_0071b730: [u8; 1];
    static mut DAT_0052b0b4: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0042b150(param_1: i32) -> i32;
    fn FUN_00447cf0(param_1: i32, param_2: i32, param_3: i32) -> u32;
    fn FUN_0040a3c0(param_1: u32, param_2: i32);
    fn FUN_004a1d70(param_1: i32, param_2: i32);
    fn FUN_0040b730(
        param_1: i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: u32,
        param_6: i32,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004479a0(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: u32,
) -> u32 {
    // SAFETY: all globals and callees are the game's own memory; every
    // address computed below is one the original dereferences.
    unsafe {
        let rd8 = |a: u32| (a as usize as *const u8).read();
        let rd16 = |a: u32| (a as usize as *const u16).read_unaligned();
        let rd32 = |a: u32| (a as usize as *const u32).read_unaligned();
        let wr8 = |a: u32, v: u8| (a as usize as *mut u8).write(v);
        let wr16 = |a: u32, v: u16| (a as usize as *mut u16).write_unaligned(v);
        let wr32 = |a: u32, v: u32| (a as usize as *mut u32).write_unaligned(v);

        let w1pln = (&raw mut s_w1pln_00525b86).cast::<u8>() as usize as u32;
        let a_09310 = (&raw mut DAT_00709310).cast::<u8>() as usize as u32;
        let a_ad48 = (&raw mut DAT_0070ad48).cast::<u8>() as usize as u32;
        let a_ad44 = (&raw mut DAT_0070ad44).cast::<u8>() as usize as u32;
        let a_b215 = (&raw mut DAT_0070b215).cast::<u8>() as usize as u32;
        let a_b7bd = (&raw mut DAT_0071b7bd).cast::<u8>() as usize as u32;
        let a_b79d = (&raw mut DAT_0071b79d).cast::<u8>() as usize as u32;
        let a_b74a = (&raw mut DAT_0071b74a).cast::<u8>() as usize as u32;
        let a_b759 = (&raw mut DAT_0071b759).cast::<u8>() as usize as u32;
        let a_b776 = (&raw mut DAT_0071b776).cast::<u8>() as usize as u32;
        let a_b777 = (&raw mut DAT_0071b777).cast::<u8>() as usize as u32;
        let a_bcdd = (&raw mut DAT_0071bcdd).cast::<u8>() as usize as u32;
        let a_b730 = (&raw mut DAT_0071b730).cast::<u8>() as usize as u32;
        let a_b0b4 = (&raw mut DAT_0052b0b4).cast::<u8>() as usize as u32;

        let mut i_var1: i32 = param_1.wrapping_mul(0x23);
        let mut i_var5: i32 = rd32(
            (DAT_006d4988 as u32)
                .wrapping_add(0x1b)
                .wrapping_add(i_var1 as u32),
        ) as i32;
        let i_var8: i32 = FUN_0042b150(DAT_00732e54);
        let mut u_var9: u32 = 0xffffffff;
        let g = DAT_00732e54 as u32;
        let p2x4 = (param_2 as u32).wrapping_mul(4);
        if rd8(g.wrapping_add(0x344).wrapping_add(p2x4)) != 0
            && rd8(g.wrapping_add(p2x4).wrapping_add(0x346)) != 0
        {
            u_var9 = FUN_00447cf0(param_1, param_2, param_3);
            if u_var9 != 0xffffffff {
                let g = DAT_00732e54 as u32;
                wr8(
                    g.wrapping_add(0x24a),
                    rd8(g.wrapping_add(0x24a)).wrapping_add(1),
                );
                let w_off = w1pln.wrapping_add((param_3 as u32).wrapping_mul(2));
                if i_var5 != -1 {
                    let u_var4 = rd16(w_off);
                    let o = (i_var5 as u32).wrapping_mul(0x5bd);
                    let s = rd16(a_b7bd.wrapping_add(o)).wrapping_add(1);
                    wr16(a_b7bd.wrapping_add(o), s);
                    let v = rd32(a_b79d.wrapping_add(o)).wrapping_sub(u_var4 as u32);
                    wr32(a_b79d.wrapping_add(o), v);
                }
                let b16 = rd8((DAT_006d4988 as u32)
                    .wrapping_add(0x16)
                    .wrapping_add(i_var1 as u32));
                let u_idx = rd8(a_ad48.wrapping_add((b16 as u32).wrapping_mul(0x566))) as u32;
                let e = a_09310.wrapping_add(u_idx.wrapping_mul(4));
                wr32(e, rd32(e).wrapping_sub(rd16(w_off) as u32));
                FUN_0040a3c0(u_var9, i_var5);
                let b16 = rd8((DAT_006d4988 as u32)
                    .wrapping_add(0x16)
                    .wrapping_add(i_var1 as u32));
                i_var1 = (b16 as u32).wrapping_mul(0x566) as i32;
                if rd8(a_ad44.wrapping_add(i_var1 as u32)) != 0x15
                    && rd8(a_b215.wrapping_add(i_var1 as u32)) != 0x15
                {
                    let g = DAT_00732e54 as u32;
                    let pc = g
                        .wrapping_add(0x346)
                        .wrapping_add((param_2 as u32).wrapping_mul(4));
                    wr8(pc, rd8(pc).wrapping_sub(1));
                }
                u_var9 = u_var9.wrapping_sub(DAT_0071b704 as u32);
                if param_4 < 2 {
                    let g = DAT_00732e54 as u32;
                    let s231 = rd16(g.wrapping_add(0x231)) as i16;
                    let a = (DAT_006db798 as u32)
                        .wrapping_add(1)
                        .wrapping_add(u_var9.wrapping_mul(8));
                    wr8(a, ((s231 as i32) / 0xec) as u8);
                    if 6 < rd8(a) {
                        wr8(a, 6);
                    }
                }
                i_var1 = u_var9.wrapping_mul(8) as i32;
                let base = DAT_006db798 as u32;
                let u_var7: u16;
                if i_var5 == -1 {
                    let e = i_var8 as u32;
                    wr16(
                        (i_var1 as u32).wrapping_add(4).wrapping_add(base),
                        rd16(e.wrapping_add(0x18)),
                    );
                    u_var7 = rd16(e.wrapping_add(0x1c));
                } else {
                    let o = (i_var5 as u32).wrapping_mul(0x5bd);
                    let v = rd16(a_b74a.wrapping_add(o));
                    wr16((i_var1 as u32).wrapping_add(4).wrapping_add(base), v);
                    u_var7 = rd16(a_b759.wrapping_add(o));
                }
                wr16((i_var1 as u32).wrapping_add(2).wrapping_add(base), u_var7);
                wr8((i_var1 as u32).wrapping_add(base), i_var5 as u8);
                let i_var8b = DAT_0071ecf4;
                i_var5 = DAT_0071b704;
                let base = DAT_006db798 as u32;
                let u_var4 = rd16((i_var1 as u32).wrapping_add(4).wrapping_add(base));
                if u_var4 != 0xffff {
                    let i_val = rd32(
                        (u_var4 as u32)
                            .wrapping_mul(0x23)
                            .wrapping_add(0x1b)
                            .wrapping_add(DAT_006d4988 as u32),
                    ) as i32;
                    if i_val != -1 {
                        let u_var12 = rd16(
                            (DAT_006db798 as u32)
                                .wrapping_add(4)
                                .wrapping_add((u_var9 & 0xffff).wrapping_mul(8)),
                        );
                        i_var1 = rd32(
                            (u_var12 as u32)
                                .wrapping_mul(0x23)
                                .wrapping_add(0x1b)
                                .wrapping_add(DAT_006d4988 as u32),
                        ) as i32;
                        if i_var1 != -1 {
                            let i_var6 = (i_var1 as u32).wrapping_mul(0x5bd);
                            if rd8(a_b776.wrapping_add(i_var6)) == 0 {
                                wr8(a_b776.wrapping_add(i_var6), 1);
                                let s = (i_var5 as u16).wrapping_add(u_var9 as u16);
                                wr16(a_b777.wrapping_add(i_var6), s);
                                wr32(a_bcdd.wrapping_add(i_var6), 0x588);
                                if i_var8b == i_var1 {
                                    FUN_004a1d70(0x73, i_var1);
                                    let u_var10 = rd32(a_b730.wrapping_add(i_var6));
                                    FUN_0040b730(DAT_0071ecf4, -1, 0xc, -1, u_var10, 0xffff);
                                }
                            }
                        }
                    }
                }
                if param_4 < 2 {
                    let g = DAT_00732e54 as u32;
                    let u_idx = rd8(a_b0b4
                        .wrapping_add(param_4)
                        .wrapping_add((rd8(g.wrapping_add(4)) as u32).wrapping_mul(0xef)))
                        as u32;
                    let o = u_idx.wrapping_mul(4);
                    let pb = g.wrapping_add(0x22b).wrapping_add(param_4);
                    if rd8(g.wrapping_add(0x34a).wrapping_add(o))
                        <= rd8(g.wrapping_add(0x346).wrapping_add(o))
                    {
                        wr8(pb, rd8(pb) & 0x7f);
                        return u_var9;
                    }
                    wr8(pb, rd8(pb) | 0x80);
                }
            }
        }
        u_var9
    }
}
