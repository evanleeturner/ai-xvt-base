// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a08374: u16;
    static mut DAT_009e9670: u32;
    static mut DAT_009ecc34: u32;
    static mut DAT_009a1fe8: u32;
    static mut DAT_00a0a246: u32;
    static mut DAT_0052158b: u8;
    static mut DAT_00521588: u8;
    static mut DAT_005233ec: u16;
    static mut DAT_00521590: i32;
    static mut DAT_00521592: i32;
    static mut DAT_005215b8: i32;
    static mut DAT_005215ba: i32;
    static mut DAT_00a08ca0: i16;
    static mut DAT_00a08ca2: i16;
    static mut DAT_009d130c: unsafe extern "C" fn(u32, i16, i16, u16, u32, u16);
    static mut DAT_009ed240: u32;
    static mut DAT_00a08f0c: u16;
    static mut DAT_00a08f08: u16;
    static mut DAT_00a08f0a: u16;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00440760(param_1: i32, param_2: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0043de10() {
    // SAFETY: all globals and the callee are the game's own memory; the
    // pointers read are the game's structures, accessed as the original does.
    unsafe {
        let u_var12: u16 = DAT_00a08374.wrapping_add(0x33);
        let base =
            (&raw mut DAT_009e9670 as usize as u32).wrapping_add(DAT_009ecc34.wrapping_mul(0x5bd));
        let t1 = (base as usize as *const i32).read_unaligned();
        let addr2 = (t1 as u32)
            .wrapping_mul(0x23)
            .wrapping_add(0x1f)
            .wrapping_add(DAT_009a1fe8);
        let t2 = (addr2 as usize as *const i32).read_unaligned();
        let i_var1_base = (t2 as u32).wrapping_add(0xa9);
        let obj = (i_var1_base as usize as *const i32).read_unaligned() as u32;

        if (*((obj.wrapping_add(0x1e1)) as usize as *const u8) & 0x10) != 0 {
            let mut u_var11: u16 =
                ((obj.wrapping_add(0x23f)) as usize as *const u16).read_unaligned();
            if (u_var11 as i16) < 0 {
                u_var11 = 0;
            }
            let u_var5: u16 =
                ((obj.wrapping_add(0x1e5)) as usize as *const u16).read_unaligned() & 0x100;
            if u_var5 == 0 {
                u_var11 = 0;
            }
            let mut u_var2: u16 = (*((obj.wrapping_add(0x241)) as usize as *const i8) != 0) as u16;
            if u_var5 == 0 {
                u_var2 = 0;
            }
            FUN_00440760((DAT_00a08374 as i32).wrapping_add(0x74), u_var2 as u32);
            let u_var6: u32 = (u_var11 as i16) as i32 as u32;
            if DAT_00a0a246 != u_var6 {
                let mut local_e: i16 = 0;
                DAT_00a0a246 = u_var11 as u32;
                loop {
                    let i_var7: i32 = local_e as i32;
                    let i_var1: i32 = i_var7.wrapping_mul(5).wrapping_add(5);
                    let u_var9: u32 = (i_var1.wrapping_mul(5) as u32) >> 8;
                    let u_var8: u32;
                    if i_var1.wrapping_mul(200) < (u_var6 as i32) {
                        u_var8 = (u_var9 << 8) | (DAT_0052158b as u32);
                    } else if (u_var11 as i16) < 0 {
                        u_var8 = (u_var9 << 8) | (DAT_00521588 as u32);
                    } else {
                        let mut u_var5b: u16 = u_var11;
                        if 1000 < (u_var11 as i16) {
                            u_var5b = 1000;
                        }
                        let idx = ((u_var5b as i16) as i32) / 0x14d;
                        let b = (*(&raw mut DAT_00521588).offset(idx as isize)) as u32;
                        u_var8 = ((u_var9 << 8) | b) & 0xffff00ff;
                    }
                    let off = (u_var12 as usize) * 0xc;
                    let mut s_var13: i16 = *(&raw mut DAT_00a08ca0).add(off);
                    let mut s_var10: i16 = *(&raw mut DAT_00a08ca2).add(off);
                    if DAT_005233ec == 0x101 {
                        let s_var3: i16 = (8i16.wrapping_sub(local_e)).wrapping_mul(3);
                        s_var13 = s_var13.wrapping_add(s_var3);
                        s_var10 = s_var10.wrapping_add(s_var3);
                    } else if DAT_005233ec == 0x112 {
                        let o = (i_var7.wrapping_mul(4)) as isize;
                        s_var13 = s_var13.wrapping_add(
                            (&raw mut DAT_00521590)
                                .offset(o)
                                .cast::<i16>()
                                .read_unaligned(),
                        );
                        s_var10 = s_var10.wrapping_add(
                            (&raw mut DAT_00521592)
                                .offset(o)
                                .cast::<i16>()
                                .read_unaligned(),
                        );
                    } else {
                        let o = (i_var7.wrapping_mul(4)) as isize;
                        s_var13 = s_var13.wrapping_add(
                            (&raw mut DAT_005215b8)
                                .offset(o)
                                .cast::<i16>()
                                .read_unaligned(),
                        );
                        s_var10 = s_var10.wrapping_add(
                            (&raw mut DAT_005215ba)
                                .offset(o)
                                .cast::<i16>()
                                .read_unaligned(),
                        );
                    }
                    let off2 = (DAT_00a08374 as usize) * 0xc;
                    let u_var4: u16 = if (DAT_00521588 as u32) == (u_var8 & 0xff) {
                        0
                    } else {
                        *(&raw mut DAT_00a08f0c).add(off2)
                    };
                    u_var11 = u_var11.wrapping_sub(1000);
                    let tex_idx = ((*(&raw mut DAT_00a08f08).add(off2) as u32)
                        .wrapping_add(i_var7 as u32)) as usize;
                    let a0 = *(&raw mut DAT_009ed240).add(tex_idx);
                    let a3 = *(&raw mut DAT_00a08f0a).add(off2);
                    DAT_009d130c(a0, s_var13, s_var10, a3, u_var8 & 0xffff00ff, u_var4);
                    local_e = local_e.wrapping_add(1);
                    if !(local_e < 9) {
                        break;
                    }
                }
            }
        }
    }
}
