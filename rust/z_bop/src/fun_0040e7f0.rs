// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009e8f54: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0040e940(param_1: *mut u8, param_2: i32, param_3: u32, param_4: u32) -> u32;
    fn FUN_004453d0(param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
}

#[inline(always)]
unsafe fn rd(base: usize, off: usize) -> i32 {
    // SAFETY: the caller guarantees base+off is readable game memory.
    unsafe { (base.wrapping_add(off) as *const i32).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0040e7f0(param_1: *mut i32) {
    // SAFETY: param_1 points at the game's own image block; all offsets
    // derived from it are read and written where the original does.
    unsafe {
        let base = param_1 as usize;
        let mut puvar2: usize = (param_1.read_unaligned() as usize).wrapping_add(base);
        let mut local: [u8; 4] = [0; 4];

        if rd(base, 0x2c) == 0x18 {
            let mut u_var5: u32 = 0;
            param_1
                .add(5)
                .write_unaligned(puvar2.wrapping_sub(base) as i32);
            let mut puvar3 = puvar2;
            let mut pbvar6: usize = (rd(base, 0xc) as usize).wrapping_add(base);
            if rd(base, 0x30) != 0 {
                loop {
                    local[0] = (pbvar6 as *const u8).read() >> 2;
                    puvar2 = puvar3.wrapping_add(1);
                    u_var5 = u_var5.wrapping_add(1);
                    local[1] = (pbvar6.wrapping_add(1) as *const u8).read() >> 2;
                    local[2] = (pbvar6.wrapping_add(2) as *const u8).read() >> 2;
                    let u_var1 = FUN_0040e940(local.as_mut_ptr(), 0x9a7bc0, 0x40, 0x100);
                    (puvar3 as *mut u8).write(u_var1 as u8);
                    puvar3 = puvar2;
                    pbvar6 = pbvar6.wrapping_add(4);
                    if !(u_var5 < rd(base, 0x30) as u32) {
                        break;
                    }
                }
            }
        }

        let mut local_4: u32 = 0;
        if rd(base, 0x18) != 0 {
            loop {
                let ivar4: usize = (rd(
                    base,
                    (rd(base, 0x10) as usize).wrapping_add((local_4 as usize).wrapping_mul(4)),
                ) as usize)
                    .wrapping_add(base);
                ((ivar4.wrapping_add(0xc)) as *mut i32)
                    .write_unaligned(puvar2.wrapping_sub(ivar4) as i32);
                if rd(ivar4, 0x24) == 0x18 {
                    let mut pbvar6: usize = (rd(ivar4, 4) as usize).wrapping_add(ivar4);
                    if DAT_009e8f54 != 0 {
                        FUN_004453d0(
                            rd(ivar4, 8).wrapping_add(ivar4 as i32),
                            pbvar6 as i32,
                            rd(ivar4, 0x10),
                            rd(ivar4, 0x14),
                            rd(ivar4, 0x20),
                        );
                    }
                    let mut u_var5: u32 = 0;
                    let mut puvar3 = puvar2;
                    if rd(ivar4, 0x28) != 0 {
                        loop {
                            local[0] = (pbvar6 as *const u8).read() >> 2;
                            puvar2 = puvar3.wrapping_add(1);
                            u_var5 = u_var5.wrapping_add(1);
                            local[1] = (pbvar6.wrapping_add(1) as *const u8).read() >> 2;
                            local[2] = (pbvar6.wrapping_add(2) as *const u8).read() >> 2;
                            let u_var1 = FUN_0040e940(local.as_mut_ptr(), 0x9a7bc0, 0x40, 0x100);
                            (puvar3 as *mut u8).write(u_var1 as u8);
                            puvar3 = puvar2;
                            pbvar6 = pbvar6.wrapping_add(4);
                            if !(u_var5 < rd(ivar4, 0x28) as u32) {
                                break;
                            }
                        }
                    }
                }
                local_4 = local_4.wrapping_add(1);
                if !(local_4 < rd(base, 0x18) as u32) {
                    break;
                }
            }
        }
    }
}
