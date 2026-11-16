// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0054f99c: i32;
    static mut DAT_0054f978: i32;
    static mut DAT_0053f8ec: [i32; 1];
    static mut DAT_009d682c: u16;
    static mut DAT_00a081ec: i32;
}

#[inline(always)]
unsafe fn rd(a: i32) -> f32 {
    // SAFETY: the address is inside the caller's vertex buffer.
    unsafe { (a as *const f32).read_unaligned() }
}

#[inline(always)]
unsafe fn wr(a: i32, v: f32) {
    // SAFETY: the address is inside the caller's vertex buffer.
    unsafe { (a as *mut f32).write_unaligned(v) }
}

#[inline(always)]
unsafe fn arr_set(i: i32, v: i32) {
    // SAFETY: the game's own table; index as in the original.
    unsafe {
        (&raw mut DAT_0053f8ec)
            .cast::<i32>()
            .wrapping_offset(i as isize)
            .write(v)
    }
}

#[inline(always)]
fn vo(base: i32, idx: i32) -> i32 {
    base.wrapping_add((idx as u32).wrapping_mul(0x18) as i32)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0040a6e0(param_1: i32, param_2: i32, param_3: i32) {
    // SAFETY: globals are the game's own; param_3 is the caller's vertex
    // buffer, read and written where the original does.
    unsafe {
        let i_var18 = DAT_0054f99c;
        let mut f_var2: f32 = rd(vo(param_3, param_1));
        let i_var19 = vo(param_3, param_1);
        let mut f_var3: f32 = rd(vo(param_3, param_2));
        let i_var1 = vo(param_3, param_2);
        let f_var12: f32 = f32::from(DAT_009d682c);

        if f_var2 <= f_var12 {
            if f_var3 > f_var12 {
                let f_var4 = rd(i_var19.wrapping_add(0xc));
                let f_var5 = rd(i_var1.wrapping_add(0xc));
                let f_var6 = rd(i_var19.wrapping_add(0x10));
                let f_var7 = rd(i_var1.wrapping_add(0x10));
                let mut f_var8 = rd(i_var19.wrapping_add(0x14));
                let f_var9 = rd(i_var1.wrapping_add(0x14));
                let f_var10 = rd(i_var19.wrapping_add(8));
                let f_var11 = rd(i_var1.wrapping_add(8));
                let f_var13 = rd(i_var1.wrapping_add(4)) - rd(i_var19.wrapping_add(4));
                let f_var14 = f_var5 - f_var4;
                let f_var15 = f_var7 - f_var6;
                let f_var16 = f_var9 - f_var8;
                let f_var17 = f_var11 - f_var10;
                let p: i32;
                if f_var3 - f_var12 <= f_var12 - f_var2 {
                    f_var2 = (f_var3 - f_var12) / (f_var3 - f_var2);
                    p = vo(param_3, DAT_0054f99c);
                    DAT_0054f99c = DAT_0054f99c.wrapping_add(1);
                    wr(
                        p.wrapping_add(4),
                        rd(i_var1.wrapping_add(4)) - f_var2 * f_var13,
                    );
                    wr(p.wrapping_add(12), f_var5 - f_var14 * f_var2);
                    f_var3 = f_var11 - f_var17 * f_var2;
                    if (if f_var17 < 0.0 { -f_var17 } else { f_var17 }) == 0.0 {
                        wr(p.wrapping_add(8), f_var3);
                    } else {
                        wr(p.wrapping_add(8), f_var3);
                        f_var2 = DAT_00a081ec as f32;
                        f_var2 = (f_var2
                            / rd(param_3.wrapping_add(8).wrapping_add(vo(0, i_var18)))
                            - f_var2 / f_var11)
                            / (f_var2 / f_var10 - f_var2 / f_var11);
                    }
                    wr(p.wrapping_add(16), f_var7 - f_var15 * f_var2);
                    f_var8 = f_var9 - f_var2 * f_var16;
                } else {
                    f_var2 = (f_var12 - f_var2) / (f_var3 - f_var2);
                    p = vo(param_3, DAT_0054f99c);
                    DAT_0054f99c = DAT_0054f99c.wrapping_add(1);
                    wr(
                        p.wrapping_add(4),
                        f_var2 * f_var13 + rd(i_var19.wrapping_add(4)),
                    );
                    wr(p.wrapping_add(12), f_var2 * f_var14 + f_var4);
                    f_var3 = f_var2 * f_var17 + f_var10;
                    if (if f_var17 < 0.0 { -f_var17 } else { f_var17 }) == 0.0 {
                        wr(p.wrapping_add(8), f_var3);
                        f_var8 = f_var2 * f_var16 + f_var8;
                        wr(p.wrapping_add(16), f_var2 * f_var15 + f_var6);
                    } else {
                        wr(p.wrapping_add(8), f_var3);
                        f_var2 = DAT_00a081ec as f32;
                        f_var2 = (f_var2
                            / rd(param_3.wrapping_add(8).wrapping_add(vo(0, i_var18)))
                            - f_var2 / f_var10)
                            / (f_var2 / f_var11 - f_var2 / f_var10);
                        f_var8 = f_var2 * f_var16 + f_var8;
                        wr(p.wrapping_add(16), f_var15 * f_var2 + f_var6);
                    }
                }
                wr(p.wrapping_add(20), f_var8);
                wr(p, f32::from(DAT_009d682c));
                DAT_0054f978 = DAT_0054f978.wrapping_add(1);
                arr_set(DAT_0054f978, i_var18);
                return;
            }
            DAT_0054f978 = DAT_0054f978.wrapping_add(1);
            arr_set(DAT_0054f978, param_2);
        } else if f_var3 <= f_var12 {
            let f_var4 = rd(i_var1.wrapping_add(0xc));
            let f_var5 = rd(i_var19.wrapping_add(0xc));
            let mut f_var6 = rd(i_var1.wrapping_add(0x10));
            let f_var7 = rd(i_var19.wrapping_add(0x10));
            let f_var8 = rd(i_var1.wrapping_add(0x14));
            let mut f_var9 = rd(i_var19.wrapping_add(0x14));
            let f_var10 = rd(i_var1.wrapping_add(8));
            let f_var11 = rd(i_var19.wrapping_add(8));
            let f_var13 = rd(i_var1.wrapping_add(4)) - rd(i_var19.wrapping_add(4));
            let f_var14 = f_var4 - f_var5;
            let f_var15 = f_var6 - f_var7;
            let f_var16 = f_var8 - f_var9;
            let f_var17 = f_var10 - f_var11;
            let p: i32;
            if f_var12 - f_var3 <= f_var2 - f_var12 {
                f_var2 = (f_var12 - f_var3) / (f_var3 - f_var2);
                p = vo(param_3, DAT_0054f99c);
                DAT_0054f99c = DAT_0054f99c.wrapping_add(1);
                wr(
                    p.wrapping_add(4),
                    f_var2 * f_var13 + rd(i_var1.wrapping_add(4)),
                );
                wr(p.wrapping_add(12), f_var14 * f_var2 + f_var4);
                f_var3 = f_var17 * f_var2 + f_var10;
                if (if f_var17 < 0.0 { -f_var17 } else { f_var17 }) == 0.0 {
                    wr(p.wrapping_add(8), f_var3);
                    f_var6 = f_var15 * f_var2 + f_var6;
                    f_var9 = f_var16 * f_var2 + f_var8;
                } else {
                    wr(p.wrapping_add(8), f_var3);
                    f_var2 = DAT_00a081ec as f32;
                    f_var2 = (f_var2 / rd(param_3.wrapping_add(8).wrapping_add(vo(0, i_var18)))
                        - f_var2 / f_var10)
                        / (f_var2 / f_var11 - f_var2 / f_var10);
                    f_var6 = f_var6 - f_var15 * f_var2;
                    f_var9 = f_var8 - f_var2 * f_var16;
                }
            } else {
                f_var2 = (f_var2 - f_var12) / (f_var3 - f_var2);
                p = vo(param_3, DAT_0054f99c);
                DAT_0054f99c = DAT_0054f99c.wrapping_add(1);
                wr(
                    p.wrapping_add(4),
                    rd(i_var19.wrapping_add(4)) - f_var2 * f_var13,
                );
                wr(p.wrapping_add(12), f_var5 - f_var2 * f_var14);
                f_var3 = f_var11 - f_var2 * f_var17;
                if (if f_var17 < 0.0 { -f_var17 } else { f_var17 }) == 0.0 {
                    wr(p.wrapping_add(8), f_var3);
                    f_var6 = f_var7 - f_var2 * f_var15;
                    f_var9 = f_var9 - f_var2 * f_var16;
                } else {
                    wr(p.wrapping_add(8), f_var3);
                    f_var2 = DAT_00a081ec as f32;
                    f_var2 = (f_var2 / rd(param_3.wrapping_add(8).wrapping_add(vo(0, i_var18)))
                        - f_var2 / f_var11)
                        / (f_var2 / f_var10 - f_var2 / f_var11);
                    f_var6 = f_var15 * f_var2 + f_var7;
                    f_var9 = f_var2 * f_var16 + f_var9;
                }
            }
            wr(p.wrapping_add(16), f_var6);
            wr(p.wrapping_add(20), f_var9);
            wr(p, f32::from(DAT_009d682c));
            let i19 = DAT_0054f978.wrapping_add(2);
            DAT_0054f978 = i19;
            arr_set(i19.wrapping_sub(1), i_var18);
            arr_set(i19, param_2);
            return;
        }
    }
}
