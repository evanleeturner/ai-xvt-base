// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00462920(param_1: *mut u8, param_2: i32, param_3: i32, param_4: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00486080(
    param_1: *mut u8,
    param_2: *const u8,
    param_3: i32,
    param_4: i32,
) {
    let mut param_2 = param_2;
    let mut local_30c_buf: [u8; 3] = [0; 3];
    let mut local_300: [u8; 768] = [0; 768];
    // SAFETY: the pointers are the caller's buffers, accessed at the offsets the
    // original uses; the callee is the game's own and the buffers outlive the calls.
    unsafe {
        local_300[0] = param_2.byte_add(2).read() >> 3;
        local_300[1] = param_2.byte_add(1).read() >> 3;
        let mut uvar7: u32 = (param_4 as u32).wrapping_mul(param_3 as u32);
        local_300[2] = param_2.read() >> 3;
        param_1.write(0);
        if 1 < uvar7 {
            let mut counter: u32 = uvar7.wrapping_sub(1);
            let mut uvar4: u32 = 1;
            let mut puvar9: *mut u8 = param_1;
            loop {
                puvar9 = puvar9.wrapping_add(1);
                local_30c_buf[0] = param_2.byte_add(5).read() >> 3;
                local_30c_buf[1] = param_2.byte_add(4).read() >> 3;
                local_30c_buf[2] = param_2.byte_add(3).read() >> 3;
                let mut uvar2 = FUN_00462920(
                    local_30c_buf.as_mut_ptr(),
                    local_300.as_mut_ptr() as i32,
                    0,
                    uvar4,
                );
                let mut uvar1: u8 = uvar2 as u8;
                let mut ivar5: usize = ((uvar2 & 0xff).wrapping_mul(3)) as usize;
                uvar2 = uvar4;
                if (local_300[ivar5] != local_30c_buf[0]
                    || local_300[ivar5 + 1] != local_30c_buf[1]
                    || local_300[ivar5 + 2] != local_30c_buf[2])
                    && uvar4 < 0x100
                {
                    uvar1 = uvar4 as u8;
                    uvar2 = uvar4.wrapping_add(1);
                    ivar5 = ((uvar4 & 0xff).wrapping_mul(3)) as usize;
                    local_300[ivar5] = local_30c_buf[0];
                    local_300[ivar5 + 1] = local_30c_buf[1];
                    local_300[ivar5 + 2] = local_30c_buf[2];
                }
                puvar9.write(uvar1);
                counter = counter.wrapping_sub(1);
                uvar4 = uvar2;
                param_2 = param_2.wrapping_add(3);
                if counter == 0 {
                    break;
                }
            }
        }
        let mut idx: usize = 0;
        let mut local_304: *mut u8 = param_1.wrapping_add(uvar7 as usize);
        let mut local_308: *mut i16 = local_304.wrapping_add(0x1000) as *mut i16;
        loop {
            uvar7 = 0;
            let mut psvar6: *mut i16 = local_308;
            let mut puvar9: *mut u8 = local_304;
            let b0 = local_300[idx] as u32;
            let b1 = local_300[idx + 1] as u32;
            let b2 = local_300[idx + 2] as u32;
            loop {
                let ivar3: u32;
                let ivar5: u32;
                if uvar7 < 8 {
                    local_30c_buf[0] = ((((uvar7.wrapping_mul(b0)) & 0xfffff8)
                        .wrapping_mul(0x10)
                        .wrapping_add(b0.wrapping_mul(0x80)))
                        >> 8) as u8;
                    local_30c_buf[1] = ((((uvar7.wrapping_mul(b1)) & 0xfffff8)
                        .wrapping_mul(0x10)
                        .wrapping_add(b1.wrapping_mul(0x80)))
                        >> 8) as u8;
                    ivar3 = b2 << 7;
                    ivar5 = ((uvar7.wrapping_mul(b2)) & 0xfffff8) << 4;
                } else {
                    let t = uvar7.wrapping_sub(8);
                    local_30c_buf[0] = (((((0x1fu32.wrapping_sub(b0)).wrapping_mul(t)) & 0xfffff8)
                        .wrapping_mul(0x20)
                        .wrapping_add(b0.wrapping_mul(0x100)))
                        >> 8) as u8;
                    local_30c_buf[1] = (((((0x1fu32.wrapping_sub(b1)).wrapping_mul(t)) & 0xfffff8)
                        .wrapping_mul(0x20)
                        .wrapping_add(b1.wrapping_mul(0x100)))
                        >> 8) as u8;
                    ivar3 = b2 << 8;
                    ivar5 = (((0x1fu32.wrapping_sub(b2)).wrapping_mul(t)) & 0xfffff8) << 5;
                }
                uvar7 = uvar7.wrapping_add(1);
                local_30c_buf[2] = ((ivar5.wrapping_add(ivar3)) >> 8) as u8;
                let v: u16 = ((local_30c_buf[0] as u16)
                    .wrapping_mul(0x20)
                    .wrapping_add(local_30c_buf[1] as u16))
                .wrapping_mul(0x40)
                .wrapping_add(local_30c_buf[2] as u16);
                psvar6.write_unaligned(v as i16);
                local_30c_buf[0] = local_30c_buf[0].wrapping_mul(2);
                local_30c_buf[1] = local_30c_buf[1].wrapping_mul(2);
                local_30c_buf[2] = local_30c_buf[2].wrapping_mul(2);
                let uvar4 = FUN_00462920(local_30c_buf.as_mut_ptr(), 0x6da540, 0x40, 0x100);
                puvar9.write(uvar4 as u8);
                psvar6 = psvar6.wrapping_add(0x100);
                puvar9 = puvar9.wrapping_add(0x100);
                if uvar7 >= 0x10 {
                    break;
                }
            }
            local_308 = local_308.wrapping_add(1);
            local_304 = local_304.wrapping_add(1);
            idx += 3;
            if idx >= 0x300 {
                break;
            }
        }
    }
}
