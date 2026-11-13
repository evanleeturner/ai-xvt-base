// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa7b1f: i32;
    static mut DAT_00aa7b13: i32;
    static mut DAT_00aa6cfc: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004d64e0(param_1: *mut i32) -> u32;
    fn FUN_004d64a0(param_1: *mut i32, param_2: i32, param_3: i32);
    fn FUN_004d6480(param_1: *mut i32, param_2: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b5100(
    param_1: *mut i32,
    param_2: *mut i32,
    param_3: i32,
    param_4: i32,
) -> u32 {
    if param_1.is_null() {
        return 0;
    }
    // r1 = local_20, local_1c, local_18, local_14
    // r2 = local_10, local_c, local_8, local_4
    let mut r1: [i32; 4] = [0; 4];
    let mut r2: [i32; 4] = [0; 4];
    // SAFETY: the callees and globals are the game's own; param_1 and param_2
    // point at the caller's structures and pixel memory, accessed as the original does.
    unsafe {
        FUN_004d6480(r1.as_mut_ptr(), param_2);
        FUN_004d64a0(
            r1.as_mut_ptr(),
            param_3.wrapping_sub(*param_2),
            param_4.wrapping_sub(*param_2.add(1)),
        );
        FUN_004d6480(r2.as_mut_ptr(), r1.as_mut_ptr());
        let u_var1 = FUN_004d64e0(r1.as_mut_ptr());
        if r1[0] <= r1[2] && r1[1] <= r1[3] {
            r1[0] = r1[0].wrapping_sub(r2[0]);
            r1[1] = r1[1].wrapping_sub(r2[1]);
            let i_var4 = *param_2;
            r1[2] = param_2
                .add(2)
                .read()
                .wrapping_sub(i_var4)
                .wrapping_sub(r1[0])
                .wrapping_sub(r2[2])
                .wrapping_add(1)
                .wrapping_add(r1[2]);
            let i_var2 = *param_2.add(1);
            let mut local_2c: i32 = param_2
                .add(3)
                .read()
                .wrapping_sub(i_var2)
                .wrapping_sub(r1[1])
                .wrapping_sub(r2[3])
                .wrapping_add(1)
                .wrapping_add(r1[3]);
            let stride = *param_1;
            let base = param_1.add(4).read();
            if DAT_00aa7b1f == 8 {
                let mut src = i_var2
                    .wrapping_add(r1[1])
                    .wrapping_mul(stride)
                    .wrapping_add(base)
                    .wrapping_add(r1[0])
                    .wrapping_add(i_var4);
                let mut dst = r1[1]
                    .wrapping_add(param_4)
                    .wrapping_mul(DAT_00aa7b13)
                    .wrapping_add(r1[0])
                    .wrapping_add(param_3)
                    .wrapping_add(DAT_00aa6cfc);
                if 0 < local_2c {
                    loop {
                        let mut i_var3: i32 = 0;
                        if 0 < r1[2] {
                            loop {
                                let s = (src.wrapping_add(i_var3)) as usize as *const u8;
                                if *s != 0 {
                                    let d = (dst.wrapping_add(i_var3)) as usize as *mut u8;
                                    *d = *s;
                                }
                                i_var3 = i_var3.wrapping_add(1);
                                if i_var3 >= r1[2] {
                                    break;
                                }
                            }
                        }
                        src = src.wrapping_add(*param_1);
                        dst = dst.wrapping_add(DAT_00aa7b13);
                        local_2c = local_2c.wrapping_sub(1);
                        if local_2c == 0 {
                            break;
                        }
                    }
                    return u_var1;
                }
            } else {
                if DAT_00aa7b1f != 0x10 {
                    return u_var1;
                }
                let mut src = i_var2
                    .wrapping_add(r1[1])
                    .wrapping_mul(stride)
                    .wrapping_add(base)
                    .wrapping_add(r1[0])
                    .wrapping_add(i_var4);
                let mut dst = r1[1]
                    .wrapping_add(param_4)
                    .wrapping_mul(DAT_00aa7b13)
                    .wrapping_add(r1[0].wrapping_add(param_3).wrapping_mul(2))
                    .wrapping_add(DAT_00aa6cfc);
                if 0 < local_2c {
                    loop {
                        let mut i_var3: i32 = 0;
                        if 0 < r1[2] {
                            loop {
                                let s = (src.wrapping_add(i_var3)) as usize as *const u8;
                                if *s != 0 {
                                    let d = (dst.wrapping_add(i_var3.wrapping_mul(2))) as usize
                                        as *mut i16;
                                    let v = param_1.add(*s as usize + 5).read();
                                    d.write_unaligned(v as i16);
                                }
                                i_var3 = i_var3.wrapping_add(1);
                                if i_var3 >= r1[2] {
                                    break;
                                }
                            }
                        }
                        src = src.wrapping_add(*param_1);
                        dst = dst.wrapping_add(DAT_00aa7b13 & 0xfffffffeu32 as i32);
                        local_2c = local_2c.wrapping_sub(1);
                        if local_2c == 0 {
                            break;
                        }
                    }
                }
            }
        }
        u_var1
    }
}
