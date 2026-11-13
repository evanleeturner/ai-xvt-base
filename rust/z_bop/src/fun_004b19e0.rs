// SPDX-License-Identifier: MIT

unsafe extern "C" {
    fn FUN_004b1810(param_1: *mut i32);
    fn FUN_004b1890(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b19e0(
    param_1: *mut i32,
    param_2: *mut i32,
    param_3: i32,
    param_4: i32,
) {
    // SAFETY: the pointers are the game's own surface structures; the pixel
    // buffers are copied exactly as the original does, forward, dwords then bytes.
    unsafe {
        FUN_004b1810(param_1);
        FUN_004b1810(param_2);
        let mut local_4: u32 = param_2.add(0x17).read_unaligned() as u32;
        let mut uvar4: u32 = 0;
        let mut puvar3: u32 = (param_1.add(9).read_unaligned() as u32 >> 3)
            .wrapping_mul(param_3 as u32)
            .wrapping_add((param_1.add(6).read_unaligned() as u32).wrapping_mul(param_4 as u32))
            .wrapping_add(param_1.add(0x17).read_unaligned() as u32);
        let uvar1: u32 = (param_2.add(9).read_unaligned() as u32 >> 3)
            .wrapping_mul(param_2.add(3).read_unaligned() as u32);
        if param_2.add(4).read_unaligned() != 0 {
            loop {
                uvar4 = uvar4.wrapping_add(1);
                let mut src = local_4;
                let mut dst = puvar3;
                let mut n = uvar1 >> 2;
                while n != 0 {
                    (dst as *mut u32).write_unaligned((src as *const u32).read_unaligned());
                    src = src.wrapping_add(4);
                    dst = dst.wrapping_add(4);
                    n -= 1;
                }
                let mut n = uvar1 & 3;
                while n != 0 {
                    (dst as *mut u8).write((src as *const u8).read());
                    src = src.wrapping_add(1);
                    dst = dst.wrapping_add(1);
                    n -= 1;
                }
                puvar3 = puvar3.wrapping_add(param_1.add(6).read_unaligned() as u32);
                local_4 = local_4.wrapping_add(param_2.add(6).read_unaligned() as u32);
                if !(uvar4 < param_2.add(4).read_unaligned() as u32) {
                    break;
                }
            }
        }
        FUN_004b1890(param_1);
        FUN_004b1890(param_2);
    }
}
