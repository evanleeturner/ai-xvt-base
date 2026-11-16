// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0052f8f0: u8;
    static mut DAT_0051a540: u8;
    static mut DAT_0051a530: u8;
    static mut DAT_00a00530: u8;
    static mut DAT_005233c8: u8;
    static mut DAT_00528c5c: *mut i32;
    static mut DAT_0051a638: i8;
    static mut DAT_0051a600: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004079e0(param_1: *const i8, ...);
    fn FUN_004079f0(param_1: u32) -> *mut u8;
    fn FUN_004b3280(param_1: i32);
    fn FUN_004b0b60(param_1: *mut u32, param_2: u32);
    fn FUN_004b0b90(param_1: *mut u16, param_2: u32);
    fn FUN_004b2680(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00407af0(
    param_1: i32,
    param_2: i32,
    param_3: *mut u16,
    param_4: *mut u8,
    param_5: i32,
) -> *mut u8 {
    // SAFETY: all globals and callees belong to the game; the pointers passed
    // in are the caller's buffers, accessed exactly as the original does.
    unsafe {
        let mut param_4 = param_4;
        let mut b_var2: u8 = 0;
        let mut local_e4: u32 = 0;
        let mut local_d8: [i32; 0x36] = [0; 0x36];

        if 0x10000 < (param_2 as u32).wrapping_mul(param_1 as u32) as i32 {
            FUN_004079e0(&raw const DAT_0051a638, param_1, param_2);
            return core::ptr::null_mut();
        }
        let p_var5 = FUN_004079f0(param_4 as usize as u32);
        if p_var5.byte_add(0x78).cast::<i32>().read_unaligned() != 0 {
            FUN_004b3280(p_var5 as usize as i32);
            return p_var5;
        }
        let mut pb10: *mut u8 = &raw mut DAT_0052f8f0;
        let mut local_ec: i32 = 0;
        if 0 < param_2 {
            loop {
                let mut b_var4 = param_4.read();
                if b_var4 == 0xff {
                    break;
                }
                let pb11 = pb10.offset(param_1 as isize);
                let mut local_f0: i32 = 0;
                while b_var4 != 0xfe {
                    b_var4 = param_4.read();
                    if b_var4 == 0xfb {
                        b_var2 = param_4.add(1).read();
                        param_4 = param_4.add(3);
                    } else if b_var4 == 0xfc {
                        let v = param_4.add(1).read();
                        param_4 = param_4.add(2);
                        b_var4 = v.wrapping_add(1);
                        if local_f0 < param_1 {
                            local_f0 = local_f0.wrapping_add(b_var4 as i32);
                            if param_1 < local_f0 {
                                local_f0 = (((local_f0 as u32) & 0xffff_ff00)
                                    | ((local_f0 as u8).wrapping_sub(b_var4) as u32))
                                    as i32;
                                b_var4 = (param_1 as u8).wrapping_sub(local_f0 as u8);
                                local_f0 = param_1;
                            }
                            while b_var4 != 0 {
                                pb10.write(0);
                                pb10 = pb10.add(1);
                                b_var4 = b_var4.wrapping_sub(1);
                            }
                        }
                    } else {
                        let b_var8: u8;
                        if b_var4 == 0xfd {
                            b_var4 = param_4.add(1).read();
                            b_var8 = param_4.add(2).read();
                            param_4 = param_4.add(3);
                        } else {
                            param_4 = param_4.add(1);
                            let sh = (&raw const DAT_0051a540)
                                .byte_offset(param_5 as isize)
                                .read()
                                & 0x1f;
                            b_var8 = ((b_var4 as u32 >> sh) as u8).wrapping_add(b_var2);
                            b_var4 = b_var4
                                & (&raw const DAT_0051a530)
                                    .byte_offset(param_5 as isize)
                                    .read();
                        }
                        b_var4 = b_var4.wrapping_add(1);
                        if local_e4 < b_var8 as u32 {
                            local_e4 = b_var8 as u32;
                        }
                        if local_f0 < param_1 {
                            local_f0 = local_f0.wrapping_add(b_var4 as i32);
                            if param_1 < local_f0 {
                                local_f0 = (((local_f0 as u32) & 0xffff_ff00)
                                    | ((local_f0 as u8).wrapping_sub(b_var4) as u32))
                                    as i32;
                                b_var4 = (param_1 as u8).wrapping_sub(local_f0 as u8);
                                local_f0 = param_1;
                            }
                            while b_var4 != 0 {
                                pb10.write(b_var8);
                                pb10 = pb10.add(1);
                                b_var4 = b_var4.wrapping_sub(1);
                            }
                        }
                    }
                    b_var4 = param_4.read();
                }
                param_4 = param_4.add(1);
                if pb10 < pb11 {
                    let u_var9 = (pb11 as usize).wrapping_sub(pb10 as usize);
                    pb10.write_bytes(0, u_var9);
                    pb10 = pb10.add(u_var9);
                }
                local_ec = local_ec.wrapping_add(1);
                if !(local_ec < param_2) {
                    break;
                }
            }
        }
        if local_ec < param_2 {
            let u_var9 = (param_2.wrapping_sub(local_ec) as u32).wrapping_mul(param_1 as u32);
            pb10.write_bytes(0, u_var9 as usize);
        }
        local_d8[0] = 0;
        local_d8[3] = param_1;
        local_d8[4] = param_2;
        local_d8[6] = param_1;
        local_d8[0x17] = (&raw mut DAT_0052f8f0) as usize as i32;
        local_d8[9] = 8;
        local_d8[8] = 0;
        let cache = DAT_00528c5c;
        let u_var3 = cache.add(4).read() as u32;
        if cache.add(3).read() != 0 {
            cache.add(4).write(0);
        }
        let src = (&raw const DAT_00a00530)
            .byte_add((DAT_005233c8 as usize).wrapping_mul(2))
            .cast::<u16>()
            .read_unaligned();
        if cache.add(4).read() == 0 {
            param_3.write(src);
            FUN_004b0b60(param_3 as *mut u32, local_e4.wrapping_add(1));
        } else {
            param_3.write(src);
            FUN_004b0b90(param_3, local_e4.wrapping_add(1));
        }
        let i_var7 = FUN_004b2680(local_d8.as_mut_ptr(), p_var5 as usize as i32, 1, 0);
        if i_var7 == 0 {
            FUN_004079e0(&raw const DAT_0051a600, param_1, param_2);
            if DAT_00528c5c.add(3).read() != 0 {
                DAT_00528c5c.add(4).write(u_var3 as i32);
            }
            return core::ptr::null_mut();
        }
        if DAT_00528c5c.add(3).read() != 0 {
            DAT_00528c5c.add(4).write(u_var3 as i32);
        }
        p_var5
    }
}
