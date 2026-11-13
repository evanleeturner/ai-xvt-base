// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00462920(
    param_1: *mut u8,
    param_2: i32,
    param_3: u32,
    param_4: u32,
) -> u32 {
    let mut param_3 = param_3;
    let mut local_10: i32 = 0x7fffffff;
    if param_3 < param_4 {
        let mut pbvar2 = param_2.wrapping_add(param_3.wrapping_mul(3) as i32) as usize as *const u8;
        let mut uvar3: u32 = param_3;
        loop {
            // SAFETY: param_1 points at a 3-byte colour and pbvar2 walks the
            // caller's palette table, read where the original reads.
            let ivar1 = unsafe {
                let a1 =
                    u32::from(param_1.add(1).read()).wrapping_sub(u32::from(pbvar2.add(1).read()));
                let a2 =
                    u32::from(param_1.add(2).read()).wrapping_sub(u32::from(pbvar2.add(2).read()));
                let a0 = u32::from(param_1.read()).wrapping_sub(u32::from(pbvar2.read()));
                a1.wrapping_mul(a1)
                    .wrapping_add(a2.wrapping_mul(a2))
                    .wrapping_add(a0.wrapping_mul(a0)) as i32
            };
            if ivar1 < local_10 {
                param_3 = uvar3;
                local_10 = ivar1;
            }
            pbvar2 = pbvar2.wrapping_add(3);
            uvar3 = uvar3.wrapping_add(1);
            if uvar3 >= param_4 {
                break;
            }
        }
    }
    param_3
}
