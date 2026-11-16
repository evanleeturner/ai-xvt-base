// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0051a854: [i8; 1];
    static mut DAT_00aa7b1f: [u8; 4];
    static mut DAT_00aa7b23: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cc4a0(param_1: *mut i8, param_2: *mut i8) -> *mut core::ffi::c_void;
    fn FUN_004cc6c0(param_1: *mut core::ffi::c_void, param_2: *mut core::ffi::c_void, param_3: i32);
    fn malloc(size: u32) -> *mut core::ffi::c_void;
    fn FUN_004d6dd0(param_1: *mut core::ffi::c_void, param_2: *mut u32, param_3: i32);
    fn FUN_004b7240(
        param_1: *mut core::ffi::c_void,
        param_2: i32,
        param_3: i32,
        param_4: i32,
    ) -> i32;
    fn FUN_004b7320(
        param_1: *mut core::ffi::c_void,
        param_2: i32,
        param_3: *mut i16,
        param_4: i32,
    ) -> i32;
    fn FUN_004b7510(param_1: *mut u8, param_2: i32, param_3: i32);
    fn FUN_004cc590(param_1: *mut core::ffi::c_void);
    fn free(param_1: *mut core::ffi::c_void);
    fn FUN_004b75b0(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b6fb0(
    param_1: *mut i8,
    param_2: *mut u32,
    param_3: i32,
    param_4: i32,
) -> u32 {
    let mut local_438: [u32; 10] = [0; 10];
    let mut local_410: [i16; 8] = [0; 8];
    let mut local_400: [u8; 1024] = [0; 1024];
    let mut local_43c: i32 = 0;
    let mut memory: *mut u8 = core::ptr::null_mut();

    // SAFETY: the callees and globals are the game's own; param_1/param_2 are
    // the caller's buffers, accessed where the original does.
    unsafe {
        let p_fvar1 = FUN_004cc4a0(param_1, (&raw mut DAT_0051a854).cast::<i8>());
        if !p_fvar1.is_null() {
            FUN_004cc6c0(p_fvar1, local_410.as_mut_ptr().cast(), 0xe);
            if local_410[0] == 0x4d42 {
                FUN_004cc6c0(p_fvar1, local_438.as_mut_ptr().cast(), 0x28);
                let u_var5 = ((local_438[1] as i32) >> 0x1f) as u32;
                let i_var2 = (((((local_438[1] ^ u_var5).wrapping_sub(u_var5)) & 3) ^ u_var5)
                    .wrapping_sub(u_var5)) as i32;
                let mut i_var4: i32 = 0;
                if i_var2 != 0 {
                    i_var4 = 4u32.wrapping_sub(i_var2 as u32) as i32;
                }
                let size = local_438[1]
                    .wrapping_mul(local_438[2])
                    .wrapping_add(i_var4 as u32);
                memory = malloc(size).cast::<u8>();
                if !memory.is_null() {
                    core::ptr::write_bytes(memory, 0, size as usize);
                    let planes = local_438.as_ptr().cast::<i16>().add(6).read();
                    let bitcount = local_438.as_ptr().cast::<i16>().add(7).read();
                    if planes == 1 {
                        if bitcount == 4 {
                            FUN_004d6dd0(p_fvar1, local_400.as_mut_ptr().cast::<u32>(), 0x10);
                            local_43c = FUN_004b7240(
                                p_fvar1,
                                memory as usize as i32,
                                local_410.as_mut_ptr() as usize as i32,
                                local_438.as_mut_ptr() as usize as i32,
                            );
                        } else if bitcount == 8 {
                            FUN_004d6dd0(p_fvar1, local_400.as_mut_ptr().cast::<u32>(), 0x100);
                            local_43c = FUN_004b7320(
                                p_fvar1,
                                memory as usize as i32,
                                local_410.as_mut_ptr(),
                                local_438.as_mut_ptr() as usize as i32,
                            );
                        }
                        let val = u32::from_ne_bytes(DAT_00aa7b1f) as i32;
                        if val == 8 {
                            if local_43c == 1 && param_3 == 1 {
                                FUN_004b7510(
                                    memory,
                                    local_400.as_mut_ptr() as usize as i32,
                                    local_438.as_mut_ptr() as usize as i32,
                                );
                            }
                        } else if val == 0x10 {
                            let mut idx: usize = 0;
                            let mut puvar3 = param_2.add(5);
                            loop {
                                let b0 = local_400[idx] as u32;
                                let b1 = local_400[idx + 1] as u32;
                                let b2 = local_400[idx + 2] as u32;
                                let v = if DAT_00aa7b23 == 0 {
                                    (((b0 >> 3).wrapping_mul(0x40)).wrapping_add(b1 >> 2))
                                        .wrapping_mul(0x20)
                                        .wrapping_add(b2 >> 3)
                                } else {
                                    ((b1 >> 3).wrapping_add((b0 >> 3).wrapping_mul(0x20)))
                                        .wrapping_mul(0x20)
                                        .wrapping_add(b2 >> 3)
                                };
                                puvar3.write_unaligned(v);
                                idx += 4;
                                puvar3 = puvar3.add(1);
                                if idx >= 1024 {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            FUN_004cc590(p_fvar1);
        }
        if local_43c == 1 {
            param_2.write_unaligned(local_438[1]);
            param_2.add(1).write_unaligned(local_438[2]);
            param_2.add(4).write_unaligned(memory as usize as u32);
            param_2.add(2).write_unaligned(0);
            param_2
                .add(3)
                .write_unaligned(local_438[2].wrapping_mul(local_438[1]));
            if param_4 == 1 {
                FUN_004b75b0(param_2.cast::<i32>());
            }
            return 1;
        }
        if !memory.is_null() {
            free(memory.cast());
        }
    }
    0
}
