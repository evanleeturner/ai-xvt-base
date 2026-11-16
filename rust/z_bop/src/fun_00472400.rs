// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a7ed0: [u16; 0];
    static mut _DAT_005233a0: u32;
    static mut DAT_009e9a5b: [u8; 0];
    static mut DAT_009e9a5f: [u8; 0];
    static mut DAT_009e9a63: [u8; 0];
    static mut DAT_009ecc34: i32;
    static mut DAT_00a004a8: i32;
    static mut DAT_00a004a4: i32;
    static mut DAT_00a004b0: i32;
    static mut DAT_00a004b8: i32;
    static mut DAT_00a004cc: i32;
    static mut DAT_00a00488: i32;
    static mut DAT_009fe7f4: i32;
    static mut DAT_00a00490: i32;
    static mut DAT_00a00480: i32;
    static mut DAT_009d77b8: i32;
    static mut DAT_009d77b4: i32;
    static mut DAT_009d77b0: i32;
    static mut DAT_009d77ac: i32;
    static mut DAT_009d7690: i32;
    static mut DAT_009d768c: i32;
    static mut _DAT_009d7688: i32;
    static mut _DAT_009d77a4: i32;
    static mut _DAT_009d77a0: i32;
    static mut _DAT_005181d8: f32;
    static mut _DAT_005181d0: f32;
    static mut DAT_0060f1c0: i32;
    static mut DAT_005271d8: *mut u32;
    static mut DAT_0060f210: u32;
    static mut _DAT_0060f224: i32;
    static mut DAT_0060f220: i32;
    static mut DAT_0060f214: i32;
    static mut DAT_0060f228: [u8; 0];
    static mut DAT_005271e0: [u8; 0];
    static mut DAT_0060f1e8: i32;
    static mut _DAT_0060f208: i32;
    static mut _DAT_0060f1f0: i32;
    static mut DAT_0060f1ec: *mut u32;
    static mut DAT_0060f1d8: i32;
    static mut DAT_0060f204: i32;
    static mut DAT_005271d4: i32;
    static mut DAT_005272a0: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004ac720(param_1: i32) -> *mut i32;
    fn FUN_00471ff0(param_1: *mut i32);
    fn FUN_00420fe0(param_1: *mut f32, param_2: *mut f32);
    fn FUN_004720d0(
        param_1: *mut core::ffi::c_void,
        param_2: *mut core::ffi::c_void,
        param_3: i32,
        param_4: i32,
    );
    fn FUN_004afa30(param_1: i32) -> i32;
    fn FUN_00472360(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32);
    fn FUN_00472c90(param_1: i32, param_2: *mut u32, param_3: *mut core::ffi::c_void);
    fn FUN_004ac740();
}

unsafe fn rd32(addr: u32) -> i32 {
    // SAFETY: the address is a game pointer the original dereferences.
    unsafe { (addr as *const i32).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00472400(param_1: i32) {
    let p1 = param_1 as u32;
    let mut blk: [f32; 52] = [0.0; 52];
    let mut saved: [u32; 52] = [0; 52];
    let mut b_var3: i32 = 0;
    let mut local_1a4: i32 = 0;
    let mut local_1a8: i32;
    let mut local_1b0: u32;

    // SAFETY: all globals and callees are the game's own; pointers come from
    // the game's structures and are read and written where the original does.
    unsafe {
        let t0 = rd32(p1.wrapping_add(0x1f));
        if t0 == 0 {
            _DAT_005233a0 = 0;
        } else {
            _DAT_005233a0 = ((t0 as u32).wrapping_add(0x8a) as *const u8).read() as u32;
        }
        let idx = ((p1.wrapping_add(3)) as *const u8).read() as usize;
        let w = (&raw const DAT_009a7ed0).cast::<u16>().add(idx).read();
        let pi4 = FUN_004ac720(((0u32 << 16) | w as u32) as i32);
        if (pi4.read() as u32) != (pi4 as u32) {
            FUN_00471ff0(pi4);
        }

        blk[0] = f32::from_bits(param_1 as u32);

        let m = (DAT_009ecc34 as u32).wrapping_mul(0x5bd);
        let v = ((&raw mut DAT_009e9a5b)
            .cast::<u8>()
            .wrapping_byte_add(m as usize) as *const i32)
            .read_unaligned();
        blk[2] = rd32(p1.wrapping_add(4)).wrapping_sub(v) as f32;
        let m = (DAT_009ecc34 as u32).wrapping_mul(0x5bd);
        let v = ((&raw mut DAT_009e9a5f)
            .cast::<u8>()
            .wrapping_byte_add(m as usize) as *const i32)
            .read_unaligned();
        blk[3] = rd32(p1.wrapping_add(8)).wrapping_sub(v) as f32;
        blk[5] = DAT_00a004a8 as f32 * _DAT_005181d8;
        blk[6] = DAT_00a004a4 as f32 * _DAT_005181d8;
        blk[7] = DAT_00a004b0 as f32 * _DAT_005181d8;
        blk[9] = DAT_00a004b8 as f32 * _DAT_005181d8;
        blk[8] = DAT_00a004cc as f32 * _DAT_005181d8;
        blk[11] = DAT_00a00488 as f32 * _DAT_005181d8;
        blk[10] = DAT_009fe7f4 as f32 * _DAT_005181d8;
        blk[13] = DAT_00a00490 as f32 * _DAT_005181d8;
        blk[12] = DAT_00a00480 as f32 * _DAT_005181d8;
        let m = (DAT_009ecc34 as u32).wrapping_mul(0x5bd);
        let v = ((&raw mut DAT_009e9a63)
            .cast::<u8>()
            .wrapping_byte_add(m as usize) as *const i32)
            .read_unaligned();
        blk[4] = rd32(p1.wrapping_add(0xc)).wrapping_sub(v) as f32;
        FUN_00420fe0(blk.as_mut_ptr().add(2), blk.as_mut_ptr().add(5));

        blk[6] = DAT_009d77b8 as f32 * _DAT_005181d8;
        blk[7] = DAT_009d77b4 as f32 * _DAT_005181d8;
        blk[8] = DAT_009d77b0 as f32 * _DAT_005181d8;
        blk[5] = DAT_009d77ac as f32 * _DAT_005181d8;
        blk[9] = DAT_009d7690 as f32 * _DAT_005181d8;
        blk[10] = DAT_009d768c as f32 * _DAT_005181d8;
        blk[11] = _DAT_009d7688 as f32 * _DAT_005181d8;
        blk[12] = _DAT_009d77a4 as f32 * _DAT_005181d8;
        blk[13] = _DAT_009d77a0 as f32 * _DAT_005181d8;
        blk[14] = -blk[2];
        blk[15] = -blk[3];
        blk[16] = -blk[4];
        blk[17] = blk[5];
        blk[18] = blk[8];
        blk[19] = blk[11];
        blk[20] = blk[6];
        blk[21] = blk[9];
        blk[22] = blk[12];
        blk[23] = blk[7];
        blk[24] = blk[10];
        blk[25] = blk[13];
        FUN_00420fe0(blk.as_mut_ptr().add(14), blk.as_mut_ptr().add(17));

        DAT_0060f1c0 = 0;
        if DAT_005271d8.is_null() {
            DAT_005271d8 = &raw mut DAT_0060f210;
            _DAT_0060f224 = 8;
            DAT_0060f220 = 8;
            DAT_0060f214 = 0x10;
            DAT_0060f210 = 0x100;
            FUN_004720d0(
                (&raw mut DAT_0060f228).cast(),
                (&raw mut DAT_005271e0).cast(),
                8,
                8,
            );
        }
        DAT_0060f1e8 = 0;
        _DAT_0060f208 = 0;
        _DAT_0060f1f0 = 0;
        DAT_0060f1ec = DAT_005271d8;
        DAT_0060f1d8 = 0;
        DAT_0060f204 = 0;
        local_1a8 = 0;

        let pi4u = pi4 as u32;
        let t0 = rd32(pi4u.wrapping_add(6));
        if 0 < t0 {
            local_1b0 = 0;
            loop {
                blk[1] = 0.0;
                let t1 = rd32(pi4u.wrapping_add(10)) as u32;
                let pu2 = ((t1.wrapping_add(local_1b0)) as *const u32).read_unaligned() as *mut u32;
                let mut i_var5 = local_1a8;
                'next: {
                    'call: {
                        if pu2.add(1).read_unaligned() != 0x14 {
                            i_var5 = local_1a8.wrapping_add(1);
                            let t0 = rd32(p1.wrapping_add(0x1f));
                            if t0 == 0 {
                                break 'call;
                            }
                            let i_var6 = rd32((t0 as u32).wrapping_add(0xa9));
                            if i_var6 == 0 {
                                break 'call;
                            }
                            let ca = (local_1a8 as u32)
                                .wrapping_add(0x2d8)
                                .wrapping_add(i_var6 as u32);
                            if (ca as *const i8).read() == 0 {
                                let b_var1 = ((local_1a8 as u32)
                                    .wrapping_add(0x30a)
                                    .wrapping_add(i_var6 as u32)
                                    as *const u8)
                                    .read();
                                if (p1.wrapping_add(3) as *const i8).read() == 4 {
                                    'lab41: {
                                        if DAT_005272a0 == -1 {
                                            DAT_005272a0 = FUN_004afa30(pi4 as i32);
                                            if DAT_005272a0 == -1 {
                                                break 'lab41;
                                            }
                                        }
                                        let t0 = rd32(p1.wrapping_add(0x1f));
                                        let t2 = rd32((t0 as u32).wrapping_add(0xa9));
                                        let a = (t2 as u32)
                                            .wrapping_add(0x30a)
                                            .wrapping_add(DAT_005272a0 as u32);
                                        if (a as *const i8).read() != 0 {
                                            let mut k = 0;
                                            while k < 52 {
                                                saved[k] = blk[k].to_bits();
                                                k += 1;
                                            }
                                            b_var3 = 1;
                                            FUN_00472360(
                                                pi4,
                                                param_1,
                                                blk.as_mut_ptr() as u32 as i32,
                                                DAT_005272a0,
                                            );
                                        }
                                    }
                                }
                                blk[1] = b_var1 as f32 * _DAT_005181d0;
                                break 'call;
                            }
                            break 'next;
                        }
                    }
                    local_1a8 = i_var5;
                    DAT_005271d4 = DAT_005271d4.wrapping_add(1);
                    FUN_00472c90(pi4 as i32, pu2, blk.as_mut_ptr().cast());
                    i_var5 = local_1a8;
                    if b_var3 != 0 {
                        let mut k = 0;
                        while k < 52 {
                            blk[k] = f32::from_bits(saved[k]);
                            k += 1;
                        }
                        b_var3 = 0;
                    }
                }
                local_1a8 = i_var5;
                local_1b0 = local_1b0.wrapping_add(4);
                local_1a4 = local_1a4.wrapping_add(1);
                let t0 = rd32(pi4u.wrapping_add(6));
                if !(local_1a4 < t0) {
                    break;
                }
            }
        }
        FUN_004ac740();
    }
}
