// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00b6bba6: [u8; 4];
    static mut DAT_00b6bbae: [u8; 4];
    static mut DAT_00b6bbb2: [u8; 40];
    static mut DAT_00b6d962: [u8; 4];
    static mut DAT_00b6da1e: [u8; 1];
    static mut DAT_00a91b90: [i8; 1];
    static mut DAT_00b69d20: [i8; 1];
    static mut DAT_00b6a2c0: i32;
    static mut DAT_00aa6114: i32;
    static mut DAT_0052adb0: [i8; 1];
    static mut DAT_0052d504: [i8; 1];
    static mut DAT_0052b49c: [i8; 1];
    static mut DAT_0052c518: [i8; 1];
    static mut DAT_0052c308: [i8; 1];
    static mut DAT_0052ada8: [i8; 1];
    static mut DAT_0052d510: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004dca90(a: i32) -> i8;
    fn FUN_004b5570(s: *mut i8, x: i32, y: i32);
    fn FUN_004d6460(r: *mut i32, a: i32, b: i32, c: i32, d: i32);
    fn FUN_004ddc10(id: i32) -> *mut i8;
    fn FUN_004db640(font: i32, s: *mut i8, r: *mut i32, color: u32);
    fn FUN_004d64a0(p: *mut i32, v: i32, n: i32);
    fn _sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_004dbfa0(s: *mut i8, font: i32) -> i32;
    fn FUN_004b4cc0(name: *mut i8, r: *mut i32);
    fn FUN_004db4e0(font: i32, s: *mut i8, x: i32, y: i32, color: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00502e20() -> u32 {
    // SAFETY: all globals and callees are the game's own; pointers refer to
    // the game's buffers and to locals that outlive every call.
    unsafe {
        let mut l: [i32; 7] = [0; 7];
        let mut local_34: [u8; 52] = [0; 52];
        let lp = l.as_mut_ptr();
        let name = local_34.as_mut_ptr() as *mut i8;
        let buf = &raw mut DAT_00b69d20 as *mut i8;
        let arr = &raw mut DAT_00b6bbb2 as *mut u8;
        let p_bba6 = &raw const DAT_00b6bba6 as *const u32;
        let p_bbae = &raw const DAT_00b6bbae as *const i32;
        let p_d962 = &raw const DAT_00b6d962 as *const u32;
        let p_da1e = &raw const DAT_00b6da1e as *const u8;

        let mut i_var7: i32 = 0;
        let mut i_var9: i32 = 0;

        if FUN_004dca90(0x12) != 0 && FUN_004dca90(0x10) != 0 && FUN_004dca90(0x78) != 0 {
            FUN_004b5570(&raw mut DAT_0052d510 as *mut i8, 0xaf, 0x156);
        }
        FUN_004d6460(lp, 0x54, 0x5a, 0x1b2, 0x6a);
        let p2 = FUN_004ddc10(0x13a);
        FUN_004db640(0xf, p2, lp, 0xffff);
        FUN_004d6460(lp, 0x54, 0x6e, 0x1b2, 0x73);
        FUN_004db640(
            0xc,
            &raw mut DAT_00a91b90 as *mut i8,
            lp,
            DAT_00b6a2c0 as u32,
        );
        FUN_004d64a0(lp, 0, 0x14);

        let mut u_var13: u32 = 0;
        loop {
            if 9 < u_var13 {
                break;
            }
            let tmp_v = arr
                .byte_add((u_var13 as usize) * 4)
                .cast::<i32>()
                .read_unaligned();
            if tmp_v == 0 {
                i_var7 = i_var7.wrapping_add(1);
            } else if tmp_v == 1 {
                i_var9 = i_var9.wrapping_add(1);
            }
            u_var13 = u_var13.wrapping_add(1);
            let tmp_u = p_bba6.read_unaligned();
            if !(u_var13 <= tmp_u) {
                break;
            }
        }

        let tmp_i = p_bbae.read_unaligned();
        let branch: Option<(i32, u32)> = if i_var7 == tmp_i {
            Some((0x159, 5))
        } else if i_var9 == tmp_i {
            Some((0x15a, 3))
        } else {
            None
        };
        if let Some((id, color_code)) = branch {
            let p2 = FUN_004ddc10(id);
            let p3 = FUN_004ddc10(0x2c5);
            _sprintf(
                buf,
                &raw const DAT_0052adb0 as *const i8,
                4i32,
                p3,
                color_code,
                p2,
            );
            FUN_004db640(0xc, buf, lp, 0xffff);
            let idx = p_d962.read_unaligned();
            let medal = p_da1e.byte_add(idx.wrapping_mul(0x4278) as usize).read();
            if medal != 0 {
                FUN_004d64a0(lp, 0, 0xf);
                let idx = p_d962.read_unaligned();
                let medal = p_da1e.byte_add(idx.wrapping_mul(0x4278) as usize).read();
                let p2 = FUN_004ddc10(i32::from(medal).wrapping_add(0x17d));
                let p3 = FUN_004ddc10(0x17d);
                let p4 = FUN_004ddc10(0x179);
                _sprintf(buf, &raw const DAT_0052d504 as *const i8, p4, p3, p2);
                let i_var5 = FUN_004dbfa0(buf, 0xc);
                let idx = p_d962.read_unaligned();
                let medal = p_da1e.byte_add(idx.wrapping_mul(0x4278) as usize).read();
                _sprintf(name, &raw const DAT_0052b49c as *const i8, i32::from(medal));
                FUN_004b4cc0(name, lp.add(4));
                let l0 = lp.read() as u32;
                let l1 = lp.add(1).read();
                let l2 = lp.add(2).read() as u32;
                let l4 = lp.add(4).read() as u32;
                let l6 = lp.add(6).read() as u32;
                let a = ((l2.wrapping_sub(l0)) as i32 >> 1) as u32;
                let b = ((((i_var5 as u32).wrapping_add(10)).wrapping_sub(l4))
                    .wrapping_add(l6)
                    .wrapping_add(0xb))
                    >> 1;
                let i_var6 = a.wrapping_sub(b).wrapping_add(l0) as i32;
                FUN_004db4e0(0xc, buf, i_var6, l1, 0xffff);
                FUN_004b5570(
                    name,
                    (i_var6 as u32)
                        .wrapping_add(i_var5 as u32)
                        .wrapping_add(10)
                        .wrapping_add(10) as i32,
                    l1,
                );
            }
        } else {
            let p2 = FUN_004ddc10(0x155);
            _sprintf(
                buf,
                &raw const DAT_0052c518 as *const i8,
                4i32,
                p2,
                1u32,
                tmp_i,
            );
            FUN_004db640(0xc, buf, lp, 0xffff);
        }

        FUN_004d64a0(lp, 0, 0x14);
        let p2 = FUN_004ddc10(0x157);
        let p3 = FUN_004ddc10(0x156);
        _sprintf(
            buf,
            &raw const DAT_0052c308 as *const i8,
            5i32,
            p3,
            1u32,
            i_var7,
            3u32,
            p2,
            1u32,
            i_var9,
        );
        FUN_004db640(0xc, buf, lp, 0xffff);
        u_var13 = 0;
        FUN_004d64a0(lp, 0, 0x14);
        let mut pi_var8: *mut u8 = arr;
        loop {
            if 0xb6bbd9u32 < (pi_var8 as usize as u32) {
                return 1;
            }
            let v = pi_var8.cast::<i32>().read_unaligned();
            if v == 0 {
                local_34[0] = 5;
            } else if v == 1 {
                local_34[0] = 3;
            } else if v == 2 {
                local_34[0] = 1;
            }
            u_var13 = u_var13.wrapping_add(1);
            let tmp_v = pi_var8.byte_add(0x50).cast::<i32>().read_unaligned();
            _sprintf(
                buf,
                &raw const DAT_0052ada8 as *const i8,
                4i32,
                DAT_00aa6114
                    .wrapping_add(tmp_v.wrapping_mul(0x148))
                    .wrapping_add(0x40),
            );
            FUN_004db640(0xc, buf, lp, 0xffff);
            FUN_004d64a0(lp, 0, 0xf);
            let tmp_v = pi_var8.cast::<i32>().read_unaligned();
            let p2 = FUN_004ddc10(tmp_v.wrapping_add(0x159));
            _sprintf(
                buf,
                &raw const DAT_0052ada8 as *const i8,
                u32::from(local_34[0]),
                p2,
            );
            FUN_004db640(0xc, buf, lp, 0xffff);
            FUN_004d64a0(lp, 0, 0xf);
            pi_var8 = pi_var8.byte_add(4);
            let tmp_u = p_bba6.read_unaligned();
            if !(u_var13 <= tmp_u) {
                break;
            }
        }
        1
    }
}
