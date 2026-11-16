// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006cca80: i32;
    static mut DAT_005279e8: [u8; 256];
    static mut DAT_005279d8: [u8; 256];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0045bcb0(param_1: u32) -> *mut i32;
    fn FUN_004b696b(param_1: *mut i32);
    fn FUN_004b88a0(param_1: i32, param_2: i32, param_3: i32) -> *mut i32;
    fn FUN_0045bbe0(fmt: *const i8, ...);
    fn FUN_004b63ea(
        param_1: i32,
        param_2: i32,
        param_3: *mut i32,
        param_4: *mut core::ffi::c_void,
    ) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0045bdc0(
    param_1: i32,
    param_2: i32,
    param_3: u32,
    param_4: *mut u8,
) -> *mut i32 {
    let _ = param_3;
    let mut param_4 = param_4;
    // SAFETY: the callees and globals are the game's own memory; param_4 and
    // the locked surface pointers are read and written where the original does.
    unsafe {
        let mut local_6c: [u32; 27] = [0; 27];
        let unaff_edi: *mut i32 = core::ptr::null_mut();
        let unaff_retaddr: i32 = 0;
        let pv_stack_88: *mut core::ffi::c_void = core::ptr::null_mut();
        let i_stack_84: i32 = 0;

        let mut pi_var3 = FUN_0045bcb0(param_4 as usize as u32);
        let i_var4 = DAT_006cca80;
        if pi_var3.add(2).read_unaligned() != 0 {
            FUN_004b696b(pi_var3);
            return pi_var3;
        }
        pi_var3 = FUN_004b88a0(param_1, param_2, DAT_006cca80);
        let mut k = 0usize;
        let mut cnt: i32 = 0x1b;
        while cnt != 0 {
            local_6c[k] = 0;
            k += 1;
            cnt = cnt.wrapping_sub(1);
        }
        local_6c[0] = 0x6c;
        let obj = pi_var3.add(3).read_unaligned();
        let vtbl = (obj as usize as *const i32).read_unaligned();
        let fp = (vtbl as usize as *const u8)
            .add(100)
            .cast::<u32>()
            .read_unaligned();
        let f: unsafe extern "stdcall" fn(*mut i32, i32, *mut u32, i32) -> i32 =
            core::mem::transmute(fp as usize as *const core::ffi::c_void);
        let i_var5 = f(obj as usize as *mut i32, 0, local_6c.as_mut_ptr(), 1);
        if i_var5 != 0 {
            FUN_0045bbe0(
                c"Failed to Lock Texture surface: returned(%x)\n".as_ptr(),
                i_var5,
            );
        }
        let mut b13: u8 = 0;
        let mut i_var5: i32 = 0;
        let t_e8 = (&raw const DAT_005279e8)
            .cast::<u8>()
            .offset(unaff_retaddr as isize);
        let t_d8 = (&raw const DAT_005279d8)
            .cast::<u8>()
            .offset(unaff_retaddr as isize);
        if 0 < (local_6c[24] as i32) {
            loop {
                let mut b2 = param_4.read();
                if b2 == 0xff {
                    break;
                }
                let mut i_var8: i32 = 0;
                let pu6: u32 = (i_var4 as u32).wrapping_add(local_6c[4]);
                let mut pu11: u32 = local_6c[4];
                while b2 != 0xfe {
                    b2 = param_4.read();
                    if b2 == 0xfb {
                        b13 = param_4.add(1).read();
                        param_4 = param_4.add(3);
                    } else if b2 == 0xfc {
                        let pb1 = param_4.add(1);
                        param_4 = param_4.add(2);
                        b2 = pb1.read().wrapping_add(1);
                        if i_var8 < param_1 {
                            i_var8 = i_var8.wrapping_add(b2 as i32);
                            if param_1 < i_var8 {
                                b2 = (param_1 as i8)
                                    .wrapping_sub((i_var8 as i8).wrapping_sub(b2 as i8))
                                    as u8;
                                i_var8 = param_1;
                            }
                            while b2 != 0 {
                                (pu11 as usize as *mut u16).write_unaligned(0);
                                pu11 = pu11.wrapping_add(2);
                                b2 = b2.wrapping_sub(1);
                            }
                        }
                    } else {
                        let b12: u8;
                        if b2 == 0xfd {
                            b2 = param_4.add(1).read();
                            b12 = param_4.add(2).read();
                            param_4 = param_4.add(3);
                        } else {
                            param_4 = param_4.add(1);
                            let sh = (t_e8.read() & 0x1f) as u32;
                            b12 = (((b2 as u32) >> sh) as u8).wrapping_add(b13);
                            b2 &= t_d8.read();
                        }
                        b2 = b2.wrapping_add(1);
                        if i_var8 < param_1 {
                            i_var8 = i_var8.wrapping_add(b2 as i32);
                            if param_1 < i_var8 {
                                b2 = (param_1 as i8)
                                    .wrapping_sub((i_var8 as i8).wrapping_sub(b2 as i8))
                                    as u8;
                                i_var8 = param_1;
                            }
                            if b2 != 0 {
                                let mut pu7 = pu11;
                                loop {
                                    b2 = b2.wrapping_sub(1);
                                    pu11 = pu7.wrapping_add(2);
                                    let src =
                                        local_6c[25].wrapping_add((b12 as u32).wrapping_mul(2));
                                    let v = (src as usize as *const u16).read_unaligned();
                                    (pu7 as usize as *mut u16).write_unaligned(v);
                                    pu7 = pu11;
                                    if b2 == 0 {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    b2 = param_4.read();
                }
                param_4 = param_4.add(1);
                local_6c[4] = pu11;
                if pu11 < pu6 {
                    let mut u9: u32 = pu6.wrapping_add(1u32.wrapping_sub(pu11));
                    let u10 = u9 >> 1;
                    local_6c[4] = pu11.wrapping_add(u10.wrapping_mul(2));
                    u9 >>= 2;
                    while u9 != 0 {
                        (pu11 as usize as *mut u32).write_unaligned(0);
                        pu11 = pu11.wrapping_add(4);
                        u9 = u9.wrapping_sub(1);
                    }
                    u9 = ((u10 & 1) != 0) as u32;
                    while u9 != 0 {
                        (pu11 as usize as *mut u16).write_unaligned(0);
                        pu11 = pu11.wrapping_add(2);
                        u9 = u9.wrapping_sub(1);
                    }
                }
                i_var5 = i_var5.wrapping_add(1);
                if !(i_var5 < (local_6c[24] as i32)) {
                    break;
                }
            }
        }
        if i_var5 < (local_6c[24] as i32) {
            local_6c[24] = local_6c[24].wrapping_sub(i_var5 as u32);
            loop {
                let mut pu11: u32 = (i_var4 as u32).wrapping_add(local_6c[4]);
                if local_6c[4] < pu11 {
                    let mut u9: u32 = pu11.wrapping_add(1u32.wrapping_sub(local_6c[4]));
                    let u10 = u9 >> 1;
                    pu11 = local_6c[4];
                    u9 >>= 2;
                    while u9 != 0 {
                        (pu11 as usize as *mut u32).write_unaligned(0);
                        pu11 = pu11.wrapping_add(4);
                        u9 = u9.wrapping_sub(1);
                    }
                    u9 = ((u10 & 1) != 0) as u32;
                    while u9 != 0 {
                        (pu11 as usize as *mut u16).write_unaligned(0);
                        pu11 = pu11.wrapping_add(2);
                        u9 = u9.wrapping_sub(1);
                    }
                    local_6c[4] = local_6c[4].wrapping_add(u10.wrapping_mul(2));
                }
                local_6c[24] = local_6c[24].wrapping_sub(1);
                if local_6c[24] as i32 == 0 {
                    break;
                }
            }
        }
        let r = FUN_004b63ea(i_stack_84, 1, unaff_edi, pv_stack_88);
        if r == 0 {
            FUN_0045bbe0(c"AddToTextureCache returned NULL!\n".as_ptr());
            return core::ptr::null_mut();
        }
        unaff_edi
    }
}
