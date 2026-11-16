// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0070a190: [i32; 16];
    static mut DAT_006db750: [i32; 16];
    static mut DAT_00703bf0: [i32; 16];
    static mut DAT_007307f0: [i32; 16];
    static mut DAT_0071b6c0: [i32; 16];
    static mut DAT_0071e540: [i32; 16];
    static mut DAT_00732fa0: [i32; 16];
    static mut DAT_00732590: [i32; 16];
    static mut DAT_007329d0: [i32; 16];
    static mut DAT_006da1b0: [i32; 125];
    static mut DAT_006d9da0: [i32; 125];
    static mut DAT_006d9fa0: [i32; 125];
    static mut DAT_0073253c: i32;
    static mut DAT_00732548: i32;
    static mut DAT_00732538: i32;
    static mut DAT_00732530: i32;
    static mut DAT_00732578: i32;
    static mut DAT_00732584: i32;
    static mut DAT_00732568: i32;
    static mut DAT_00732558: i32;
    static mut DAT_0073255c: i32;
    static mut DAT_00732e4c: i8;
    static mut DAT_00525670: [u8; 0];
    static mut DAT_005256b0: u16;
    static mut DAT_005256b4: u16;
    static mut DAT_005256b8: u16;
    static mut DAT_005256bc: u16;
    static mut DAT_005256c0: u16;
    static mut DAT_005256c4: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00419100(a: i32, b: i32) -> u32;
    fn FUN_0044b5d0(a: i32, b: i32, c: u32, d: i16, e: u32);
}

#[allow(non_snake_case)]
unsafe fn process(
    idx: &mut u32,
    mut count: u32,
    p: [*const i32; 3],
    q: [*const i32; 3],
    neg: bool,
    sx: *const i32,
    sy: *const i32,
    sz: *const i32,
    ang: u32,
) {
    // SAFETY: all pointers refer to the game's own tables and globals; the
    // byte stream is read at the same indices as the original.
    unsafe {
        while count != 0 {
            count -= 1;
            let b = (&raw const DAT_00525670)
                .cast::<u8>()
                .add(*idx as usize)
                .read();
            *idx = idx.wrapping_add(1);
            let i = (b & 7) as usize;
            let mut a0 = p[0].add(i).read();
            let mut a1 = p[1].add(i).read();
            let mut a2 = p[2].add(i).read();
            if (b & 8) != 0 {
                a0 = a0.wrapping_neg();
                a1 = a1.wrapping_neg();
                a2 = a2.wrapping_neg();
            }
            let off = ((b & 0x70) >> 2) as usize;
            let mut c0 = q[0].cast::<u8>().add(off).cast::<i32>().read_unaligned();
            let mut c1 = q[1].cast::<u8>().add(off).cast::<i32>().read_unaligned();
            let mut c2 = q[2].cast::<u8>().add(off).cast::<i32>().read_unaligned();
            if (b & 0x80) != 0 {
                c0 = c0.wrapping_neg();
                c1 = c1.wrapping_neg();
                c2 = c2.wrapping_neg();
            }
            let z = if neg {
                a2.wrapping_add(c2).wrapping_sub(sz.read() >> 2)
            } else {
                a2.wrapping_add(c2).wrapping_add(sz.read() >> 2)
            };
            if z >= 0 {
                let x = if neg {
                    a0.wrapping_add(c0).wrapping_sub(sx.read() >> 2)
                } else {
                    a0.wrapping_add(c0).wrapping_add(sx.read() >> 2)
                };
                let y = if neg {
                    a1.wrapping_add(c1).wrapping_sub(sy.read() >> 2)
                } else {
                    a1.wrapping_add(c1).wrapping_add(sy.read() >> 2)
                };
                FUN_0044b5d0(x, y, z as u32, (ang as i16).wrapping_neg(), *idx);
            }
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0044ae00() {
    // SAFETY: the globals and callees are the game's own memory and code.
    unsafe {
        let i_var13 = DAT_0073253c;
        let mut i_var8: i32 = 0;
        let mut i_var11: i32 = 0;
        let mut i_var9: i32 = 0;
        let mut i_var6: i32 = 0;
        let mut local_20: i32 = 0;
        let mut local_10: i32 = 0;
        let mut local_1c: i32 = 0;
        let mut local_18: i32 = 0;
        let mut local_14: i32 = 0;
        let mut k: usize = 0;
        while k < 16 {
            DAT_0070a190[k] = i_var6 >> 5;
            DAT_006db750[k] = local_14 >> 5;
            DAT_00703bf0[k] = local_18 >> 5;
            DAT_007307f0[k] = local_1c >> 5;
            DAT_0071b6c0[k] = local_10 >> 5;
            DAT_0071e540[k] = local_20 >> 5;
            let i_var12 = i_var11 >> 5;
            i_var11 = i_var11.wrapping_add(i_var13);
            DAT_00732fa0[k] = i_var12;
            DAT_00732590[k] = i_var8 >> 5;
            DAT_007329d0[k] = i_var9 >> 5;
            i_var9 = i_var9.wrapping_add(DAT_00732548);
            i_var8 = i_var8.wrapping_add(DAT_00732538);
            local_20 = local_20.wrapping_add(DAT_00732530);
            local_10 = local_10.wrapping_add(DAT_00732578);
            local_1c = local_1c.wrapping_add(DAT_00732584);
            local_18 = local_18.wrapping_add(DAT_00732568);
            local_14 = local_14.wrapping_add(DAT_00732558);
            i_var6 = i_var6.wrapping_add(DAT_0073255c);
            k += 1;
        }
        let mut idx7: usize = 0;
        let mut local_c = DAT_00732568
            .wrapping_add(DAT_00732530)
            .wrapping_add(DAT_00732548)
            .wrapping_mul(-2);
        let mut local_8 = DAT_00732578
            .wrapping_add(DAT_00732538)
            .wrapping_add(DAT_00732558)
            .wrapping_mul(-2);
        local_10 = 5;
        let mut local_4 = DAT_00732584
            .wrapping_add(i_var13)
            .wrapping_add(DAT_0073255c)
            .wrapping_mul(-2);
        loop {
            local_1c = local_4;
            local_18 = local_8;
            local_14 = local_c;
            local_20 = 5;
            loop {
                let mut i_var2 = 5;
                i_var6 = local_14;
                i_var9 = local_1c;
                i_var11 = local_18;
                loop {
                    let i_var12 = i_var9 >> 7;
                    i_var9 = i_var9.wrapping_add(i_var13);
                    DAT_006da1b0[idx7] = i_var12;
                    DAT_006d9da0[idx7] = i_var11 >> 7;
                    DAT_006d9fa0[idx7] = i_var6 >> 7;
                    i_var6 = i_var6.wrapping_add(DAT_00732548);
                    i_var11 = i_var11.wrapping_add(DAT_00732538);
                    i_var2 -= 1;
                    idx7 += 1;
                    if i_var2 == 0 {
                        break;
                    }
                }
                local_1c = local_1c.wrapping_add(DAT_00732584);
                local_18 = local_18.wrapping_add(DAT_00732578);
                local_14 = local_14.wrapping_add(DAT_00732530);
                local_20 -= 1;
                if local_20 == 0 {
                    break;
                }
            }
            local_c = local_c.wrapping_add(DAT_00732568);
            local_8 = local_8.wrapping_add(DAT_00732558);
            local_4 = local_4.wrapping_add(DAT_0073255c);
            local_10 -= 1;
            if local_10 == 0 {
                break;
            }
        }
        if DAT_00732e4c == 0 {
            return;
        }
        let a0 = (&raw const DAT_0070a190).cast::<i32>();
        let a1 = (&raw const DAT_006db750).cast::<i32>();
        let a2 = (&raw const DAT_00703bf0).cast::<i32>();
        let b0 = (&raw const DAT_007307f0).cast::<i32>();
        let b1 = (&raw const DAT_0071b6c0).cast::<i32>();
        let b2 = (&raw const DAT_0071e540).cast::<i32>();
        let c0 = (&raw const DAT_00732fa0).cast::<i32>();
        let c1 = (&raw const DAT_00732590).cast::<i32>();
        let c2 = (&raw const DAT_007329d0).cast::<i32>();

        let mut idx: u32 = 0;
        let mut u_var3 = FUN_00419100(DAT_00732558, DAT_0073255c);
        if DAT_00732530 < 0 {
            idx = DAT_005256b0 as u32;
            process(
                &mut idx,
                DAT_005256b4 as u32,
                [a0, a1, a2],
                [c0, c1, c2],
                true,
                &raw const DAT_00732584,
                &raw const DAT_00732578,
                &raw const DAT_00732530,
                u_var3,
            );
        } else {
            process(
                &mut idx,
                DAT_005256b0 as u32,
                [a0, a1, a2],
                [c0, c1, c2],
                false,
                &raw const DAT_00732584,
                &raw const DAT_00732578,
                &raw const DAT_00732530,
                u_var3,
            );
            idx = idx.wrapping_add(DAT_005256b4 as u32);
        }
        u_var3 = FUN_00419100(DAT_00732578, DAT_00732584);
        if DAT_00732568 < 0 {
            idx = idx.wrapping_add(DAT_005256c0 as u32);
            process(
                &mut idx,
                DAT_005256c4 as u32,
                [b0, b1, b2],
                [c0, c1, c2],
                true,
                &raw const DAT_0073255c,
                &raw const DAT_00732558,
                &raw const DAT_00732568,
                u_var3,
            );
        } else {
            process(
                &mut idx,
                DAT_005256c0 as u32,
                [b0, b1, b2],
                [c0, c1, c2],
                false,
                &raw const DAT_0073255c,
                &raw const DAT_00732558,
                &raw const DAT_00732568,
                u_var3,
            );
            idx = idx.wrapping_add(DAT_005256c4 as u32);
        }
        u_var3 = FUN_00419100(DAT_00732558, DAT_0073255c);
        if DAT_00732548 < 0 {
            idx = idx.wrapping_add(DAT_005256b8 as u32);
            process(
                &mut idx,
                DAT_005256bc as u32,
                [b0, b1, b2],
                [a0, a1, a2],
                true,
                &raw const DAT_0073253c,
                &raw const DAT_00732538,
                &raw const DAT_00732548,
                u_var3,
            );
        } else {
            process(
                &mut idx,
                DAT_005256b8 as u32,
                [b0, b1, b2],
                [a0, a1, a2],
                false,
                &raw const DAT_0073253c,
                &raw const DAT_00732538,
                &raw const DAT_00732548,
                u_var3,
            );
        }
    }
}
