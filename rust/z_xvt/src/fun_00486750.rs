// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals, non_camel_case_types)]
unsafe extern "C" {
    static mut DAT_006da850: [i16; 1];
    static mut _DAT_005254e0: u32;
    static mut _DAT_005201ec: f32;
    static mut DAT_0071ecf4: i32;
    static mut DAT_0071bb1b: u8;
    static mut DAT_0071bb1f: u8;
    static mut DAT_0071bb23: u8;
    static mut DAT_0073255c: i32;
    static mut DAT_00732558: i32;
    static mut DAT_00732568: i32;
    static mut DAT_00732578: i32;
    static mut DAT_00732584: i32;
    static mut DAT_0073253c: i32;
    static mut DAT_00732530: i32;
    static mut DAT_00732548: i32;
    static mut DAT_00732538: i32;
    static mut DAT_0070a138: i32;
    static mut DAT_0070a134: i32;
    static mut DAT_0070a130: i32;
    static mut DAT_0070a12c: i32;
    static mut DAT_0070a010: i32;
    static mut DAT_0070a00c: i32;
    static mut _DAT_0070a008: i32;
    static mut _DAT_0070a124: i32;
    static mut _DAT_0070a120: i32;
    static mut DAT_00602678: u32;
    static mut DAT_0052a470: *mut u32;
    static mut DAT_006026c8: u32;
    static mut DAT_006026cc: u32;
    static mut DAT_006026d8: u32;
    static mut _DAT_006026dc: u32;
    static mut DAT_006026e0: u8;
    static mut DAT_0052a478: u8;
    static mut DAT_006026a4: *mut u32;
    static mut DAT_006026a0: u32;
    static mut _DAT_006026c0: u32;
    static mut _DAT_006026a8: u32;
    static mut DAT_00602690: u32;
    static mut DAT_006026bc: u32;
    static mut DAT_0052a46c: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0048f4f0(param_1: u32) -> *mut i32;
    fn FUN_00486000(param_1: *mut i32, param_2: i32);
    fn FUN_00414fe0(param_1: *mut f32, param_2: *mut f32);
    fn FUN_00486080(
        param_1: *mut core::ffi::c_void,
        param_2: *mut core::ffi::c_void,
        param_3: i32,
        param_4: i32,
    );
    fn FUN_00486b80(param_1: i32, param_2: *mut u32, param_3: *mut i32);
    fn FUN_0048f510();
}

#[repr(C)]
union Local {
    i: [i32; 0x34],
    f: [f32; 0x34],
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00486750(param_1: i32, mut param_2: i32) {
    // SAFETY: all pointers are the game's own memory, read and written where
    // the original does; globals and callees belong to the game.
    unsafe {
        let pb1 = param_1 as *const u8;
        let mut u_var4: u32 = pb1.byte_add(3).read() as u32;
        let mut p_field: i32 = pb1.byte_add(0x1f).cast::<i32>().read_unaligned();
        if u_var4 == 0x59 && p_field != 0 {
            u_var4 = ((p_field as u32).wrapping_add(0x85) as *const u8).read() as u32;
        }
        p_field = pb1.byte_add(0x1f).cast::<i32>().read_unaligned();
        if p_field == 0 {
            _DAT_005254e0 = 0;
        } else {
            _DAT_005254e0 = ((p_field as u32).wrapping_add(0x88) as *const u8).read() as u32;
        }
        let u_stack: core::mem::MaybeUninit<u16> = core::mem::MaybeUninit::uninit();
        let hi = core::ptr::read_volatile(u_stack.as_ptr()) as u32;
        let lo = (&raw const DAT_006da850)
            .cast::<i16>()
            .add(u_var4 as usize)
            .read() as u16 as u32;
        let pi_var3 = FUN_0048f4f0((hi << 16) | lo);
        let pb3 = pi_var3 as *mut u8;
        let mut pi_var5 = *pi_var3 as *mut i32;
        if pi_var3 != pi_var5 {
            let mut i_var6: i32 = 0;
            let i_var8: i32 = (pi_var3 as u32).wrapping_sub(pi_var5 as u32) as i32;
            *pi_var3 = (pi_var5 as u32).wrapping_add(i_var8 as u32) as i32;
            let mut tmp: i32 = pb3.byte_add(10).cast::<i32>().read_unaligned();
            if tmp != 0 {
                let mut i_var7: i32 = 0;
                tmp = tmp.wrapping_add(i_var8);
                pb3.byte_add(10).cast::<i32>().write_unaligned(tmp);
                let mut cnt: i32 = pb3.byte_add(6).cast::<i32>().read_unaligned();
                if 0 < cnt {
                    loop {
                        let ofs = pb3.byte_add(10).cast::<i32>().read_unaligned();
                        pi_var5 = ofs.wrapping_add(i_var6) as *mut i32;
                        if *pi_var5 != 0 {
                            *pi_var5 = (*pi_var5).wrapping_add(i_var8);
                            let ofs = pb3.byte_add(10).cast::<i32>().read_unaligned();
                            let p = ofs.wrapping_add(i_var6) as *const i32;
                            FUN_00486000(p.read() as *mut i32, i_var8);
                        }
                        i_var6 = i_var6.wrapping_add(4);
                        i_var7 = i_var7.wrapping_add(1);
                        cnt = pb3.byte_add(6).cast::<i32>().read_unaligned();
                        if !(i_var7 < cnt) {
                            break;
                        }
                    }
                }
            }
        }

        let mut l = Local { i: [0; 0x34] };
        l.i[0] = param_1;

        let idx_ofs = (DAT_0071ecf4 as u32).wrapping_mul(0x5bd) as usize;
        let rd = |base: *mut u8| -> i32 { base.byte_add(idx_ofs).cast::<i32>().read_unaligned() };
        let a = rd(&raw mut DAT_0071bb1b);
        l.f[2] = (*((param_1.wrapping_add(4)) as *const i32)).wrapping_sub(a) as f32;
        let a = rd(&raw mut DAT_0071bb1f);
        l.f[3] = (*((param_1.wrapping_add(8)) as *const i32)).wrapping_sub(a) as f32;
        l.f[5] = DAT_0073255c as f32 * _DAT_005201ec;
        l.f[6] = DAT_00732558 as f32 * _DAT_005201ec;
        l.f[7] = DAT_00732568 as f32 * _DAT_005201ec;
        l.f[9] = DAT_00732578 as f32 * _DAT_005201ec;
        l.f[8] = DAT_00732584 as f32 * _DAT_005201ec;
        l.f[11] = DAT_0073253c as f32 * _DAT_005201ec;
        l.f[10] = DAT_00732530 as f32 * _DAT_005201ec;
        l.f[13] = DAT_00732548 as f32 * _DAT_005201ec;
        l.f[12] = DAT_00732538 as f32 * _DAT_005201ec;
        let a = rd(&raw mut DAT_0071bb23);
        l.f[4] = (*((param_1.wrapping_add(0xc)) as *const i32)).wrapping_sub(a) as f32;
        let fp = (&raw mut l).cast::<f32>();
        FUN_00414fe0(fp.add(2), fp.add(5));

        l.f[6] = DAT_0070a138 as f32 * _DAT_005201ec;
        l.f[7] = DAT_0070a134 as f32 * _DAT_005201ec;
        l.f[8] = DAT_0070a130 as f32 * _DAT_005201ec;
        l.f[5] = DAT_0070a12c as f32 * _DAT_005201ec;
        l.f[9] = DAT_0070a010 as f32 * _DAT_005201ec;
        l.f[10] = DAT_0070a00c as f32 * _DAT_005201ec;
        l.f[11] = _DAT_0070a008 as f32 * _DAT_005201ec;
        l.f[12] = _DAT_0070a124 as f32 * _DAT_005201ec;
        l.f[13] = _DAT_0070a120 as f32 * _DAT_005201ec;
        l.f[14] = -l.f[2];
        l.f[15] = -l.f[3];
        l.f[16] = -l.f[4];
        l.f[17] = l.f[5];
        l.f[18] = l.f[8];
        l.f[19] = l.f[11];
        l.f[20] = l.f[6];
        l.f[21] = l.f[9];
        l.f[22] = l.f[12];
        l.f[23] = l.f[7];
        l.f[24] = l.f[10];
        l.f[25] = l.f[13];
        FUN_00414fe0(fp.add(14), fp.add(17));

        DAT_00602678 = 0;
        if DAT_0052a470.is_null() {
            DAT_0052a470 = &raw mut DAT_006026c8;
            _DAT_006026dc = 8;
            DAT_006026d8 = 8;
            DAT_006026cc = 0x10;
            DAT_006026c8 = 0x100;
            FUN_00486080(
                (&raw mut DAT_006026e0).cast(),
                (&raw mut DAT_0052a478).cast(),
                8,
                8,
            );
        }
        let mut i_var6: i32 = 0;
        DAT_006026a4 = DAT_0052a470;
        DAT_006026a0 = 0;
        _DAT_006026c0 = 0;
        _DAT_006026a8 = 0;
        DAT_00602690 = 0;
        DAT_006026bc = 0;
        let mut cnt = pb3.byte_add(6).cast::<i32>().read_unaligned();
        if 0 < cnt {
            let ip = (&raw mut l).cast::<i32>();
            loop {
                let ofs = pb3.byte_add(10).cast::<i32>().read_unaligned();
                let pp = (ofs as u32).wrapping_add((i_var6 as u32).wrapping_mul(4)) as *const u32;
                let pu_var2 = pp.read_unaligned() as *mut u32;
                if *pu_var2.add(1) == 0x14 {
                    DAT_0052a46c = DAT_0052a46c.wrapping_add(1);
                    param_2 = param_2.wrapping_add(1);
                    let ofs = pb3.byte_add(10).cast::<i32>().read_unaligned();
                    let pp =
                        (ofs as u32).wrapping_add((i_var6 as u32).wrapping_mul(4)) as *const u32;
                    FUN_00486b80(pi_var3 as i32, pp.read_unaligned() as *mut u32, ip);
                } else if param_2 == i_var6 {
                    DAT_0052a46c = DAT_0052a46c.wrapping_add(1);
                    FUN_00486b80(pi_var3 as i32, pu_var2, ip);
                }
                i_var6 = i_var6.wrapping_add(1);
                cnt = pb3.byte_add(6).cast::<i32>().read_unaligned();
                if !(i_var6 < cnt) {
                    break;
                }
            }
        }
        FUN_0048f510();
    }
}
