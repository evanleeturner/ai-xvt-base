// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006d4984: u16;
    static mut DAT_0071f714: u16;
    static mut DAT_00732a38: u32;
    static mut DAT_006d4988: i32;
    static mut DAT_00527a00: [i32; 32];
    static mut DAT_006db7b4: u32;
    static mut _DAT_005201b8: f32;
    static mut _DAT_00520194: f32;
    static mut DAT_005f10b8: i32;
    static mut DAT_005f10dc: i32;
    static mut DAT_005f1030: [i32; 34];
    static mut DAT_005f10d8: i32;
    static mut DAT_005f0fb0: [i32; 32];
    static mut DAT_005f10d0: u32;
    static mut DAT_005f10d4: u32;
    static mut DAT_005f0fa8: i32;
    static mut DAT_005f10e0: u32;
    static mut DAT_005f10b0: *mut i8;
    static mut DAT_005f10c8: *mut i32;
    static mut _DAT_005f10e4: f32;
    static mut _DAT_005f10b4: f32;
    static mut DAT_005279f4: i32;
    static mut DAT_0052ae1c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0045bbe0(a: *const i8);
    fn FUN_004186e0(a: u32, b: u16) -> i32;
    fn FUN_00418840(a: u32, b: u16) -> i32;
    fn FUN_0045d7d0(a: i32, b: i32, c: i32);
    fn FUN_0045dd00(a: i32, b: i32, c: i32);
    fn FUN_0045e290(a: i32, b: i32, c: i32);
    fn FUN_0045e7d0(a: i32, b: i32, c: i32);
    fn FUN_004b5bc2();
    fn FUN_004b5c5c(a: *mut core::ffi::c_void, b: u32);
    fn FUN_004b5ce0();
    fn FUN_004b5d64(a: *mut i32, b: u32);
    fn FUN_004b6054();
    fn FUN_0045bdc0(a: i32, b: i32, c: i32, d: *mut u8) -> *mut i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0045fce0(
    param_1: u16,
    param_2: i32,
    param_3: i32,
    param_4: u32,
    param_5: i32,
) {
    // SAFETY: all globals and callees are the game's own; param_5 points at the
    // caller's bitmap header and the game's buffers are accessed as the original does.
    unsafe {
        let mut local_3cc: i32 = 0;
        let mut local_3c0: [f32; 24] = [0.0; 24];

        let i_var12: i32 = (DAT_006d4984 as u32).wrapping_sub(param_3 as u32) as i32;
        if (DAT_0071f714 as u32) < DAT_00732a38 {
            let i_var6 = (DAT_0071f714 as u32)
                .wrapping_mul(0x23)
                .wrapping_add(DAT_006d4988 as u32);
            let p = i_var6 as usize as *const u8;
            if p.byte_add(2).read() == 13 {
                let b = p.byte_add(0x19).read();
                if b < 0x20 {
                    local_3cc = DAT_00527a00[b as usize];
                } else {
                    local_3cc = -1;
                }
            } else {
                local_3cc = -1;
            }
        }
        let mut local_3d8: f32 = (DAT_006db7b4 as f32) * _DAT_005201b8;
        if _DAT_00520194 < local_3d8 {
            local_3d8 = 0.999984f32;
            local_3cc = -1;
        }
        let pb = param_5 as usize as *const u8;
        let mut i_var6: i32 = pb.byte_add(0x10).cast::<i32>().read_unaligned();
        let mut i_var17: i32 = pb.byte_add(0x14).cast::<i32>().read_unaligned();
        if 0x100 < i_var6 {
            i_var6 = 0x100;
            FUN_0045bbe0(b"TRUNCATING BITMAP TO 256 WIDE!!!\n\0".as_ptr() as *const i8);
        }
        if 0x100 < i_var17 {
            i_var17 = 0x100;
            FUN_0045bbe0(b"TRUNCATING BITMAP TO 256 HIGH!!!\n\0".as_ptr() as *const i8);
        }
        let mut i_var13: i32 = 1;
        let mut i_var7: i32 = 0;
        loop {
            i_var13 = (i_var13 as u32).wrapping_mul(2) as i32;
            if i_var6 <= i_var13 {
                break;
            }
            i_var7 += 1;
            if i_var7 >= 8 {
                break;
            }
        }
        let mut i_var7b: i32 = 1;
        let mut i_var8: i32 = 0;
        loop {
            i_var7b = (i_var7b as u32).wrapping_mul(2) as i32;
            if i_var17 <= i_var7b {
                break;
            }
            i_var8 += 1;
            if i_var8 >= 8 {
                break;
            }
        }
        let i_var7 = i_var7b;
        i_var8 = pb.byte_add(0x10).cast::<i32>().read_unaligned();
        let h: i32 = pb.byte_add(0x14).cast::<i32>().read_unaligned();
        let mut u_var19: u32 = (h.wrapping_mul((param_4 & 0xffff) as i32) >> 9) as u32;
        let mut i_var9 = FUN_004186e0(u_var19, param_1);
        let u_var14: u32 = (i_var8.wrapping_mul((param_4 & 0xffff) as i32) >> 9) as u32;
        i_var8 = FUN_00418840(u_var14, param_1);
        let mut i_var10 = FUN_00418840(u_var19, param_1);
        let mut i_var11 = FUN_004186e0(u_var14, param_1);
        let mut i_var18: i32 = 0;
        DAT_005f10b8 = 4;
        DAT_005f10dc = 4;
        DAT_005f1030[0] = 0;
        DAT_005f1030[1] = 1;
        DAT_005f1030[2] = 2;
        DAT_005f1030[3] = 3;
        local_3c0[0] = param_2.wrapping_add(i_var8).wrapping_add(i_var9) as f32;
        local_3c0[1] = (i_var10.wrapping_sub(i_var11)).wrapping_add(i_var12) as f32;
        local_3c0[3] = 0.0;
        local_3c0[4] = 0.0;
        local_3c0[5] = 0.0;
        local_3c0[2] = local_3d8;
        i_var8 = FUN_004186e0(u_var19, param_1);
        let u_var15: u32 = 0u32.wrapping_sub(u_var14);
        i_var9 = FUN_00418840(u_var15, param_1);
        i_var10 = FUN_00418840(u_var19, param_1);
        i_var11 = FUN_004186e0(u_var15, param_1);
        local_3c0[6] = param_2.wrapping_add(i_var9).wrapping_add(i_var8) as f32;
        local_3c0[9] = 0.0;
        local_3c0[7] = (i_var10.wrapping_sub(i_var11)).wrapping_add(i_var12) as f32;
        local_3c0[11] = 0.0;
        local_3c0[8] = local_3d8;
        u_var19 = 0u32.wrapping_sub(u_var19);
        local_3c0[10] = (i_var6 as f32) / (i_var13 as f32);
        i_var8 = FUN_004186e0(u_var19, param_1);
        i_var9 = FUN_00418840(u_var15, param_1);
        i_var10 = FUN_00418840(u_var19, param_1);
        i_var11 = FUN_004186e0(u_var15, param_1);
        local_3c0[12] = param_2.wrapping_add(i_var9).wrapping_add(i_var8) as f32;
        local_3c0[15] = 0.0;
        local_3c0[13] = (i_var10.wrapping_sub(i_var11)).wrapping_add(i_var12) as f32;
        local_3c0[14] = local_3d8;
        local_3c0[16] = (i_var6 as f32) / (i_var13 as f32);
        local_3c0[17] = (i_var17 as f32) / (i_var7 as f32);
        i_var6 = FUN_004186e0(u_var19, param_1);
        i_var8 = FUN_00418840(u_var14, param_1);
        i_var9 = FUN_00418840(u_var19, param_1);
        i_var10 = FUN_004186e0(u_var14, param_1);
        local_3c0[18] = param_2.wrapping_add(i_var8).wrapping_add(i_var6) as f32;
        local_3c0[21] = 0.0;
        local_3c0[22] = 0.0;
        local_3c0[23] = (i_var17 as f32) / (i_var7 as f32);
        local_3c0[20] = local_3d8;
        DAT_005f10d8 = 0;
        local_3c0[19] = (i_var9.wrapping_sub(i_var10)).wrapping_add(i_var12) as f32;

        let arr = local_3c0.as_ptr() as usize as i32;

        if 0 < DAT_005f10b8 {
            let mut p = &raw mut DAT_005f1030 as *mut i32;
            let mut i_var12 = DAT_005f1030[(DAT_005f10b8 - 1) as usize];
            loop {
                let v = p.read();
                p = p.add(1);
                i_var18 += 1;
                FUN_0045d7d0(i_var12, v, arr);
                i_var12 = v;
                if i_var18 >= DAT_005f10b8 {
                    break;
                }
            }
        }
        let mut i_var12: i32 = 0;
        DAT_005f10b8 = 0;
        if 0 < DAT_005f10d8 {
            let mut p = &raw mut DAT_005f0fb0 as *mut i32;
            let mut prev = DAT_005f0fb0[(DAT_005f10d8 - 1) as usize];
            loop {
                let v = p.read();
                p = p.add(1);
                i_var12 += 1;
                FUN_0045dd00(prev, v, arr);
                prev = v;
                if i_var12 >= DAT_005f10d8 {
                    break;
                }
            }
        }
        i_var12 = 0;
        DAT_005f10d8 = 0;
        if 0 < DAT_005f10b8 {
            let mut p = &raw mut DAT_005f1030 as *mut i32;
            let mut prev = DAT_005f1030[(DAT_005f10b8 - 1) as usize];
            loop {
                let v = p.read();
                p = p.add(1);
                i_var12 += 1;
                FUN_0045e290(prev, v, arr);
                prev = v;
                if i_var12 >= DAT_005f10b8 {
                    break;
                }
            }
        }
        i_var12 = 0;
        DAT_005f10b8 = 0;
        if 0 < DAT_005f10d8 {
            let mut p = &raw mut DAT_005f0fb0 as *mut i32;
            let mut prev = DAT_005f0fb0[(DAT_005f10d8 - 1) as usize];
            loop {
                let v = p.read();
                p = p.add(1);
                i_var12 += 1;
                FUN_0045e7d0(prev, v, arr);
                prev = v;
                if i_var12 >= DAT_005f10d8 {
                    break;
                }
            }
        }
        if 2 < DAT_005f10b8 {
            if DAT_005f10d0 < DAT_005f10d4.wrapping_add(DAT_005f10b8 as u32)
                || DAT_005f0fa8 < DAT_005f10e0.wrapping_add(DAT_005f10b8 as u32) as i32
            {
                FUN_004b5bc2();
                FUN_004b5c5c(DAT_005f10b0 as *mut core::ffi::c_void, DAT_005f10d4);
                FUN_004b5ce0();
                FUN_004b5d64(DAT_005f10c8, DAT_005f10e0);
                FUN_004b6054();
                DAT_005f10e0 = 0;
                DAT_005f10d4 = 0;
            }
            if DAT_005279f4 != 0 {
                local_3cc = -0x1000001;
                DAT_005279f4 = 0;
            }
            i_var12 = 0;
            if 0 < DAT_005f10b8 {
                let mut p = &raw mut DAT_005f1030 as *mut i32;
                loop {
                    let i_var6 = p.read();
                    i_var12 += 1;
                    let base = (i_var6 as isize * 6) as usize;
                    let f1 = local_3c0[base + 1];
                    let f3 = local_3c0[base + 4];
                    let f4 = local_3c0[base + 2];
                    let f5 = local_3c0[base + 5];
                    let q = (DAT_005f10b0 as *mut u8)
                        .byte_add(DAT_005f10d4.wrapping_mul(0x20) as usize);
                    q.cast::<f32>()
                        .write_unaligned(local_3c0[base] + _DAT_005f10e4);
                    q.byte_add(4)
                        .cast::<f32>()
                        .write_unaligned(f1 + _DAT_005f10b4);
                    q.byte_add(8).cast::<f32>().write_unaligned(f4);
                    q.byte_add(0xc).cast::<f32>().write_unaligned(f4);
                    q.byte_add(0x18).cast::<f32>().write_unaligned(f3);
                    q.byte_add(0x1c).cast::<f32>().write_unaligned(f5);
                    q.byte_add(0x10).cast::<i32>().write_unaligned(local_3cc);
                    q.byte_add(0x14).cast::<i32>().write_unaligned(0);
                    let i_var6b = DAT_005f10b8;
                    p.write(DAT_005f10d4 as i32);
                    DAT_005f10d4 = DAT_005f10d4.wrapping_add(1);
                    p = p.add(1);
                    if i_var12 >= i_var6b {
                        break;
                    }
                }
            }
            i_var12 = 2;
            let pi_var16 = FUN_0045bdc0(
                i_var13,
                i_var7,
                param_5.wrapping_add(pb.byte_add(0xc).cast::<i32>().read_unaligned()),
                param_5
                    .wrapping_add(pb.byte_add(8).cast::<i32>().read_unaligned())
                    .wrapping_add(0x10) as usize as *mut u8,
            );
            if 2 < DAT_005f10b8 {
                let mut puvar20 = (&raw mut DAT_005f1030 as *mut i32).add(1);
                loop {
                    let e = DAT_005f10c8.add(DAT_005f10e0.wrapping_mul(5) as usize);
                    e.write_unaligned(DAT_005f1030[0]);
                    e.add(1).write_unaligned(puvar20.read());
                    e.add(2).write_unaligned(puvar20.add(1).read());
                    e.add(3).write_unaligned(pi_var16 as usize as i32);
                    e.add(4).write_unaligned(0x3022);
                    if DAT_0052ae1c != 0 {
                        let v = e.add(4).read_unaligned();
                        e.add(4).write_unaligned(v.wrapping_add(0x300));
                    }
                    if local_3cc != -1 {
                        let v = e.add(4).read_unaligned();
                        e.add(4).write_unaligned(v.wrapping_add(0x400));
                    }
                    i_var12 += 1;
                    DAT_005f10e0 = DAT_005f10e0.wrapping_add(1);
                    puvar20 = puvar20.add(1);
                    if i_var12 >= DAT_005f10b8 {
                        break;
                    }
                }
            }
        }
    }
}
