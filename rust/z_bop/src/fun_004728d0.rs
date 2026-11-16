// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_005233a0: u32;
    static mut DAT_009a7ed0: [u16; 0];
    static mut DAT_009ecc34: i32;
    static mut DAT_009e9a5b: [u8; 0];
    static mut DAT_009e9a5f: [u8; 0];
    static mut DAT_009e9a63: [u8; 0];
    static mut _DAT_005181d8: f32;
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
    static mut DAT_0060f1c0: i32;
    static mut DAT_005271d8: *mut u32;
    static mut DAT_0060f210: u32;
    static mut _DAT_0060f224: i32;
    static mut DAT_0060f220: i32;
    static mut DAT_0060f214: i32;
    static mut DAT_0060f228: [u8; 0];
    static mut DAT_005271e0: [u8; 0];
    static mut DAT_0060f1ec: *mut u32;
    static mut DAT_0060f1e8: i32;
    static mut _DAT_0060f208: i32;
    static mut _DAT_0060f1f0: i32;
    static mut DAT_0060f1d8: i32;
    static mut DAT_0060f204: i32;
    static mut DAT_005271d4: i32;
}

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_004ac720(param: u32) -> *mut i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00471ff0(param_1: *mut i32);
    fn FUN_00420fe0(param_1: *mut f32, param_2: *mut f32);
    fn FUN_004720d0(
        param_1: *mut core::ffi::c_void,
        param_2: *mut core::ffi::c_void,
        param_3: i32,
        param_4: i32,
    );
    fn FUN_00472c90(param_1: i32, param_2: *mut u32, param_3: *mut core::ffi::c_void);
    fn FUN_004ac740();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004728d0(param_1: i32, param_2: i32) {
    let mut param_2 = param_2;
    let p = param_1 as usize as *const u8;
    let mut local_d0: [f32; 0x34] = [0.0; 0x34];
    // SAFETY: all pointers are the game's own memory; reads and writes follow
    // the original's order and offsets.
    unsafe {
        let mut u3 = p.byte_add(3).read() as u32;
        let mut go_fa = true;
        if u3 == 0x59 {
            let tmp = p.byte_add(0x1f).cast::<i32>().read_unaligned();
            if tmp != 0 {
                u3 = (tmp as usize as *const u8).byte_add(0x87).read() as u32;
            } else {
                go_fa = false;
            }
        }
        if go_fa {
            let tmp = p.byte_add(0x1f).cast::<i32>().read_unaligned();
            if tmp != 0 {
                _DAT_005233a0 = (tmp as usize as *const u8).byte_add(0x8a).read() as u32;
            } else {
                _DAT_005233a0 = 0;
            }
        } else {
            _DAT_005233a0 = 0;
        }

        let idx = (&raw mut DAT_009a7ed0)
            .cast::<u16>()
            .add(u3 as usize)
            .read();
        let pi2 = FUN_004ac720(idx as u32);
        if (pi2.read() as usize as *mut i32) != pi2 {
            FUN_00471ff0(pi2);
        }

        for i in 0..0x34 {
            local_d0[i] = 0.0;
        }
        local_d0[0] = f32::from_bits(param_1 as u32);

        let off = (DAT_009ecc34 as u32).wrapping_mul(0x5bd) as usize;
        let a = (&raw mut DAT_009e9a5b)
            .cast::<u8>()
            .add(off)
            .cast::<i32>()
            .read_unaligned();
        let b = (&raw mut DAT_009e9a5f)
            .cast::<u8>()
            .add(off)
            .cast::<i32>()
            .read_unaligned();
        let k = _DAT_005181d8;
        local_d0[2] = p.byte_add(4).cast::<i32>().read_unaligned().wrapping_sub(a) as f32;
        local_d0[3] = p.byte_add(8).cast::<i32>().read_unaligned().wrapping_sub(b) as f32;
        local_d0[5] = DAT_00a004a8 as f32 * k;
        local_d0[6] = DAT_00a004a4 as f32 * k;
        local_d0[7] = DAT_00a004b0 as f32 * k;
        local_d0[9] = DAT_00a004b8 as f32 * k;
        local_d0[8] = DAT_00a004cc as f32 * k;
        local_d0[11] = DAT_00a00488 as f32 * k;
        local_d0[10] = DAT_009fe7f4 as f32 * k;
        local_d0[13] = DAT_00a00490 as f32 * k;
        local_d0[12] = DAT_00a00480 as f32 * k;
        let c = (&raw mut DAT_009e9a63)
            .cast::<u8>()
            .add(off)
            .cast::<i32>()
            .read_unaligned();
        local_d0[4] = p
            .byte_add(0xc)
            .cast::<i32>()
            .read_unaligned()
            .wrapping_sub(c) as f32;
        let base = local_d0.as_mut_ptr();
        FUN_00420fe0(base.add(2), base.add(5));
        let k = _DAT_005181d8;
        local_d0[6] = DAT_009d77b8 as f32 * k;
        local_d0[7] = DAT_009d77b4 as f32 * k;
        local_d0[8] = DAT_009d77b0 as f32 * k;
        local_d0[5] = DAT_009d77ac as f32 * k;
        local_d0[9] = DAT_009d7690 as f32 * k;
        local_d0[10] = DAT_009d768c as f32 * k;
        local_d0[11] = _DAT_009d7688 as f32 * k;
        local_d0[12] = _DAT_009d77a4 as f32 * k;
        local_d0[13] = _DAT_009d77a0 as f32 * k;
        local_d0[14] = -local_d0[2];
        local_d0[15] = -local_d0[3];
        local_d0[16] = -local_d0[4];
        local_d0[17] = local_d0[5];
        local_d0[18] = local_d0[8];
        local_d0[19] = local_d0[11];
        local_d0[20] = local_d0[6];
        local_d0[21] = local_d0[9];
        local_d0[22] = local_d0[12];
        local_d0[23] = local_d0[7];
        local_d0[24] = local_d0[10];
        local_d0[25] = local_d0[13];
        FUN_00420fe0(base.add(14), base.add(17));
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
        let mut i4: i32 = 0;
        DAT_0060f1ec = DAT_005271d8;
        DAT_0060f1e8 = 0;
        _DAT_0060f208 = 0;
        _DAT_0060f1f0 = 0;
        DAT_0060f1d8 = 0;
        DAT_0060f204 = 0;
        let pb = pi2 as *const u8;
        let mut cnt = pb.byte_add(6).cast::<i32>().read_unaligned();
        if 0 < cnt {
            loop {
                let arr = pb.byte_add(10).cast::<i32>().read_unaligned() as u32;
                let pu1 = (arr.wrapping_add((i4 as u32).wrapping_mul(4)) as usize
                    as *const *mut u32)
                    .read_unaligned();
                if pu1.add(1).read_unaligned() == 0x14 {
                    DAT_005271d4 = DAT_005271d4.wrapping_add(1);
                    param_2 = param_2.wrapping_add(1);
                    let arr = pb.byte_add(10).cast::<i32>().read_unaligned() as u32;
                    let e = (arr.wrapping_add((i4 as u32).wrapping_mul(4)) as usize
                        as *const *mut u32)
                        .read_unaligned();
                    FUN_00472c90(pi2 as usize as i32, e, base.cast());
                } else if i4 == param_2 {
                    DAT_005271d4 = DAT_005271d4.wrapping_add(1);
                    FUN_00472c90(pi2 as usize as i32, pu1, base.cast());
                }
                i4 = i4.wrapping_add(1);
                cnt = pb.byte_add(6).cast::<i32>().read_unaligned();
                if !(i4 < cnt) {
                    break;
                }
            }
        }
        FUN_004ac740();
    }
}
