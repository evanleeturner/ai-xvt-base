// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00556928: *mut u8;
    static mut DAT_0055692c: u32;
    static mut DAT_00556930: u32;
    static mut DAT_00556959: u8;
    static mut DAT_0055695d: u8;
    static mut _DAT_00518148: f64;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00444e60(param_1: *mut u8);
    fn FUN_00444d90(
        param_1: i8,
        param_2: i8,
        param_3: *mut u8,
        param_4: i8,
        param_5: i8,
        param_6: i8,
    ) -> *mut u8;
    fn FUN_004438b0(param_1: *const i8, param_2: *const i8);
    fn FUN_00443890(param_1: *const i8, param_2: u32, param_3: u32);
    fn _exit(code: i32);
}

unsafe extern "stdcall" {
    fn RaiseException(code: u32, flags: u32, nargs: u32, args: *const u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004441f0(param_1: i32) {
    let pb_param = param_1 as *mut u8;
    // SAFETY: all pointers are the game's own memory; reads and writes
    // follow the original's offsets and order.
    unsafe {
        let d_var3 = pb_param.byte_add(0x1030).cast::<u32>().read_unaligned() as f64;
        let u_size = pb_param.byte_add(0x1034).cast::<u32>().read_unaligned();
        let mut local_10: u32 = 0;
        let mut d_acc: f64 = DAT_00556928.byte_add(0xe).cast::<f64>().read_unaligned();
        d_acc = (u_size as f64) * d_var3 * _DAT_00518148 + d_acc;
        DAT_00556928
            .byte_add(0xe)
            .cast::<f64>()
            .write_unaligned(d_acc);

        let mut pb_var13: *mut u8 = pb_param.byte_add(0x1086).cast::<*mut u8>().read_unaligned();
        let mut u_count: u32 = pb_param.byte_add(0x108e).cast::<u32>().read_unaligned();
        if u_count == 0 {
            return;
        }
        loop {
            let u_tmp = (&raw mut DAT_0055695d).cast::<u32>().read_unaligned();
            if 0x41241 < u_tmp {
                FUN_00444e60(DAT_00556928);
                DAT_0055692c = DAT_0055692c.wrapping_sub(1);
            }
            let mut b_var9: u8 = 7;
            let mut local_18: u32 = 1;
            let u_var6: u32 = (*pb_var13.add(3) as u32).wrapping_add(1);
            let mut pb_var12: *mut u8 = DAT_00556928;
            if DAT_0055692c != 0 {
                loop {
                    let u_shift = (b_var9 as u32) & 0x1f;
                    let b_var4: u8 = (*pb_var13.add(2) as u32 >> u_shift) as u8;
                    let u_var2: u32 = (((((*pb_var13 as u32 >> u_shift) & 1).wrapping_mul(2))
                        | ((*pb_var13.add(1) as u32 >> u_shift) & 1))
                        .wrapping_mul(2));
                    let u_var10: u32 = u_var2 | ((b_var4 & 1) as u32);
                    let slot =
                        pb_var12.byte_add((u_var10.wrapping_mul(4).wrapping_add(0x32)) as usize);
                    let u_tmp = slot.cast::<u32>().read_unaligned();
                    if u_tmp == 0 {
                        *pb_var12.add(2) |= (1i32.wrapping_shl(u_var10 as i8 as i32 as u32)) as u8;
                        let c5 = ((1u32
                            .wrapping_shl(8u32.wrapping_sub(local_18 as i8 as i32 as u32) & 0x1f))
                            >> 1) as i8;
                        let mut c_var5 = c5;
                        let mut c_var11 = c5;
                        if (b_var4 & 1) == 0 {
                            c_var11 = c5.wrapping_neg();
                        }
                        let mut c_var8 = c5;
                        if (u_var2 & 2) == 0 {
                            c_var8 = c5.wrapping_neg();
                        }
                        if (u_var2 & 4) == 0 {
                            c_var5 = c5.wrapping_neg();
                        }
                        let p7 = FUN_00444d90(
                            u_var10 as i8,
                            local_18 as i8,
                            pb_var12,
                            (*pb_var12.add(3)).wrapping_add(c_var5 as u8) as i8,
                            (*pb_var12.add(4)).wrapping_add(c_var8 as u8) as i8,
                            (*pb_var12.add(5)).wrapping_add(c_var11 as u8) as i8,
                        );
                        slot.cast::<*mut u8>().write_unaligned(p7);
                        if p7.is_null() {
                            FUN_004438b0(
                                b"Unable to quantize image\0".as_ptr() as *const i8,
                                b"Memory allocation failed\0".as_ptr() as *const i8,
                            );
                            _exit(1);
                        }
                        if local_18 == DAT_0055692c {
                            DAT_00556930 = DAT_00556930.wrapping_add(1);
                        }
                    }
                    pb_var12 = slot.cast::<*mut u8>().read_unaligned();
                    b_var9 = b_var9.wrapping_sub(1);
                    local_18 = local_18.wrapping_add(1);

                    let mut u_table = (&raw mut DAT_00556959).cast::<u32>().read_unaligned();
                    let mut u_addr = u_table.wrapping_add(
                        (*pb_var13.add(2) as u32)
                            .wrapping_sub(*pb_var12.add(5) as u32)
                            .wrapping_mul(4),
                    );
                    let u_tmp = (u_addr as usize as *const u32).read_unaligned();
                    let d_t2 = u_tmp as f64;
                    u_table = (&raw mut DAT_00556959).cast::<u32>().read_unaligned();
                    u_addr = u_table.wrapping_add(
                        (*pb_var13.add(1) as u32)
                            .wrapping_sub(*pb_var12.add(4) as u32)
                            .wrapping_mul(4),
                    );
                    let u_tmp2 = (u_addr as usize as *const u32).read_unaligned();
                    let d_t1 = u_tmp2 as f64;
                    u_table = (&raw mut DAT_00556959).cast::<u32>().read_unaligned();
                    u_addr = u_table.wrapping_add(
                        (*pb_var13 as u32)
                            .wrapping_sub(*pb_var12.add(3) as u32)
                            .wrapping_mul(4),
                    );
                    let u_tmp3 = (u_addr as usize as *const u32).read_unaligned();
                    let d_t0 = u_tmp3 as f64;
                    let d_sum = d_t2 + d_t1 + d_t0;
                    d_acc = pb_var12.byte_add(0xe).cast::<f64>().read_unaligned();
                    d_acc = d_sum * (u_var6 as f64) + d_acc;
                    pb_var12.byte_add(0xe).cast::<f64>().write_unaligned(d_acc);

                    if !(local_18 <= DAT_0055692c) {
                        break;
                    }
                }
            }
            let mut u_tmp = pb_var12.byte_add(10).cast::<u32>().read_unaligned();
            u_tmp = u_tmp.wrapping_add(u_var6);
            pb_var12.byte_add(10).cast::<u32>().write_unaligned(u_tmp);

            let mut u_pix = (*pb_var13 as u32).wrapping_mul(u_var6);
            d_acc = pb_var12.byte_add(0x16).cast::<f64>().read_unaligned();
            d_acc = (u_pix as f64) + d_acc;
            pb_var12.byte_add(0x16).cast::<f64>().write_unaligned(d_acc);

            u_pix = (*pb_var13.add(1) as u32).wrapping_mul(u_var6);
            d_acc = pb_var12.byte_add(0x1e).cast::<f64>().read_unaligned();
            d_acc = (u_pix as f64) + d_acc;
            pb_var12.byte_add(0x1e).cast::<f64>().write_unaligned(d_acc);

            u_pix = (*pb_var13.add(2) as u32).wrapping_mul(u_var6);
            pb_var13 = pb_var13.add(6);
            d_acc = pb_var12.byte_add(0x26).cast::<f64>().read_unaligned();
            d_acc = (u_pix as f64) + d_acc;
            pb_var12.byte_add(0x26).cast::<f64>().write_unaligned(d_acc);

            u_count = pb_param.byte_add(0x108e).cast::<u32>().read_unaligned();
            let u_mod = pb_param.byte_add(0x1034).cast::<u32>().read_unaligned();
            let cond = if u_count.wrapping_sub(local_10) == 1 {
                true
            } else if u_mod == 0 {
                RaiseException(0xC0000094, 0, 0, core::ptr::null());
                false
            } else {
                local_10 % u_mod == 0
            };
            if cond {
                u_count = pb_param.byte_add(0x108e).cast::<u32>().read_unaligned();
                FUN_00443890(
                    b"  Classifying image colors...  \0".as_ptr() as *const i8,
                    local_10,
                    u_count,
                );
            }
            local_10 = local_10.wrapping_add(1);
            u_count = pb_param.byte_add(0x108e).cast::<u32>().read_unaligned();
            if !(local_10 < u_count) {
                break;
            }
        }
    }
}
