// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00732f3c: u32;
    static mut DAT_007091ae: u16;
    static mut DAT_006d4984: u16;
    static mut DAT_00709ffc: i32;
    static mut _DAT_006a8a08: f32;
    static mut _DAT_00520194: f32;
    static mut DAT_005f10dc: i32;
    static mut DAT_005f10b8: i32;
    static mut DAT_005f102c: [i32; 1];
}

unsafe fn rd(p: *mut f32, i: usize) -> f32 {
    // SAFETY: caller supplies a valid vertex pointer.
    unsafe { p.add(i).read_unaligned() }
}

unsafe fn wr(p: *mut f32, i: usize, v: f32) {
    // SAFETY: caller supplies a valid vertex pointer.
    unsafe { p.add(i).write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0045ed70(param_1: i32, param_2: i32, param_3: i32) {
    // SAFETY: globals are the game's own memory; param_3 points at the
    // game's vertex array, accessed exactly where the original does.
    unsafe {
        let i_var13 = DAT_005f10dc;
        let p3 = param_3 as u32;
        let mut f_var9: f32 = rd(
            p3.wrapping_add(8)
                .wrapping_add((param_1 as u32).wrapping_mul(0x18)) as *mut f32,
            0,
        );
        let pf1 = p3.wrapping_add((param_1 as u32).wrapping_mul(0x18)) as *mut f32;
        let mut f_var10: f32 = rd(
            p3.wrapping_add(8)
                .wrapping_add((param_2 as u32).wrapping_mul(0x18)) as *mut f32,
            0,
        );
        let pf2 = p3.wrapping_add((param_2 as u32).wrapping_mul(0x18)) as *mut f32;
        let u_var9 = f_var9.to_bits();
        let u_var10 = f_var10.to_bits();
        let arr = &raw mut DAT_005f102c as *mut i32;

        if u_var9 < 0x80000001 {
            if 0x80000000 < u_var10 {
                let f_var4 = rd(pf2, 0);
                f_var9 = (DAT_00732f3c as f32) / f_var9;
                let f_var5 = rd(pf2, 1);
                let f_var12 = (rd(pf1, 0) - ((DAT_007091ae >> 1) as f32)) * f_var9 * _DAT_006a8a08;
                let f_var6 = rd(pf2, 4);
                let f_var7 = rd(pf2, 5);
                let f_var11 = (rd(pf1, 1)
                    - (((DAT_006d4984 >> 1) as u32).wrapping_add(DAT_00709ffc as u32) as i32
                        as f32))
                    * f_var9
                    * _DAT_006a8a08;
                let f_var8 = rd(pf1, 4);
                f_var10 = f_var10 / ((f_var10 - f_var9) + _DAT_00520194);
                let pf3 = p3.wrapping_add((DAT_005f10dc as u32).wrapping_mul(0x18)) as *mut f32;
                f_var9 = rd(pf1, 5);
                DAT_005f10dc = DAT_005f10dc.wrapping_add(1);
                wr(pf3, 3, rd(pf2, 3) - (rd(pf2, 3) - rd(pf1, 3)) * f_var10);
                wr(pf3, 0, f_var4 - (f_var4 - f_var12) * f_var10);
                wr(pf3, 1, f_var5 - (f_var5 - f_var11) * f_var10);
                wr(pf3, 4, f_var6 - (f_var6 - f_var8) * f_var10);
                wr(pf3, 5, f_var7 - f_var10 * (f_var7 - f_var9));
                f_var9 = DAT_00732f3c as f32;
                wr(pf3, 2, f_var9);
                wr(pf3, 0, rd(pf3, 0) * f_var9);
                wr(pf3, 1, f_var9 * rd(pf3, 1));
                wr(pf3, 0, ((DAT_007091ae >> 1) as f32) + rd(pf3, 0));
                wr(
                    pf3,
                    1,
                    (((DAT_006d4984 >> 1) as u32).wrapping_add(DAT_00709ffc as u32) as i32 as f32)
                        + rd(pf3, 1),
                );
                DAT_005f10b8 = DAT_005f10b8.wrapping_add(1);
                arr.wrapping_offset(DAT_005f10b8 as isize)
                    .write_unaligned(i_var13);
                return;
            }
            DAT_005f10b8 = DAT_005f10b8.wrapping_add(1);
            arr.wrapping_offset(DAT_005f10b8 as isize)
                .write_unaligned(param_2);
        } else if u_var10 < 0x80000001 {
            let f_var4 = rd(pf1, 0);
            let f_var5 = rd(pf1, 1);
            let f_var6 = rd(pf1, 4);
            let f_var7 = rd(pf1, 5);
            f_var10 = (DAT_00732f3c as f32) / f_var10;
            let f_var12 = (rd(pf2, 0) - ((DAT_007091ae >> 1) as f32)) * f_var10 * _DAT_006a8a08;
            let f_var11 = (rd(pf2, 1)
                - (((DAT_006d4984 >> 1) as u32).wrapping_add(DAT_00709ffc as u32) as i32 as f32))
                * f_var10
                * _DAT_006a8a08;
            let f_var8 = rd(pf2, 4);
            f_var9 = f_var9 / ((f_var10 - f_var9) - _DAT_00520194);
            let pf3 = p3.wrapping_add((DAT_005f10dc as u32).wrapping_mul(0x18)) as *mut f32;
            f_var10 = rd(pf2, 5);
            DAT_005f10dc = DAT_005f10dc.wrapping_add(1);
            wr(pf3, 3, rd(pf1, 3) - (rd(pf2, 3) - rd(pf1, 3)) * f_var9);
            wr(pf3, 0, f_var4 - (f_var12 - f_var4) * f_var9);
            wr(pf3, 1, f_var5 - (f_var11 - f_var5) * f_var9);
            wr(pf3, 4, f_var6 - (f_var8 - f_var6) * f_var9);
            wr(pf3, 5, f_var7 - f_var9 * (f_var10 - f_var7));
            f_var9 = DAT_00732f3c as f32;
            wr(pf3, 2, f_var9);
            wr(pf3, 0, rd(pf3, 0) * f_var9);
            wr(pf3, 1, f_var9 * rd(pf3, 1));
            wr(pf3, 0, ((DAT_007091ae >> 1) as f32) + rd(pf3, 0));
            wr(
                pf3,
                1,
                (((DAT_006d4984 >> 1) as u32).wrapping_add(DAT_00709ffc as u32) as i32 as f32)
                    + rd(pf3, 1),
            );
            arr.wrapping_offset(1)
                .wrapping_offset(DAT_005f10b8 as isize)
                .write_unaligned(i_var13);
            DAT_005f10b8 = DAT_005f10b8.wrapping_add(2);
            arr.wrapping_offset(DAT_005f10b8 as isize)
                .write_unaligned(param_2);
            return;
        }
    }
}
