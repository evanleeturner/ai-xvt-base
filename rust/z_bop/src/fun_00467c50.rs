// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009a73b6: u32;
    static mut DAT_00a08104: u32;
    static mut DAT_009a1fe0: u32;
    static mut DAT_009a1fe8: u32;
    static mut DAT_009a73b0: u16;
    static mut DAT_009d1320: [i8; 1];
    static mut DAT_009a73bc: u32;
    static mut DAT_0051bfd4: [i8; 1];
    static mut DAT_009a73c0: u16;
    static mut DAT_00a07cec: u16;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_00403400();
}

#[inline(always)]
unsafe fn rd<T>(a: u32) -> T {
    // SAFETY: the address is the game's own memory, read where the original reads.
    unsafe { (a as usize as *const T).read_unaligned() }
}

#[inline(always)]
unsafe fn wr<T>(a: u32, v: T) {
    // SAFETY: the address is the game's own memory, written where the original writes.
    unsafe { (a as usize as *mut T).write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_00467c50() -> u32 {
    // SAFETY: all globals and pointers are the game's own memory, accessed
    // exactly as the original does.
    unsafe {
        let u_var2: u32 = DAT_009a73b6;
        let b_var1: u8 = rd::<u8>(DAT_00a08104.wrapping_add(5));
        if b_var1 == 0xff {
            return DAT_00a08104 & 0xffff0000;
        }
        if (DAT_009a1fe0 as i32) <= (b_var1 as i32) {
            return DAT_009a1fe0 & 0xffff0000;
        }
        let i_var5: u32 = (b_var1 as u32)
            .wrapping_mul(0x23)
            .wrapping_add(DAT_009a1fe8);

        let tmp_ptr: u32 = rd::<u32>(i_var5.wrapping_add(0x1f));
        let i_var4: u32 = rd::<u32>(tmp_ptr.wrapping_add(0xa9));

        let a: i8 = rd::<i8>(
            (DAT_009a73b0 as u32)
                .wrapping_mul(0x23)
                .wrapping_add(0x16)
                .wrapping_add(DAT_009a1fe8),
        );
        let b: i8 = rd::<i8>(i_var5.wrapping_add(0x16));
        let c: i8 = rd::<i8>(i_var5.wrapping_add(3));
        let mut local_1: i32 = (a != b || c == 0) as i32;
        if rd::<i8>(i_var4.wrapping_add(7)) == 3 || rd::<i8>(i_var4.wrapping_add(7)) == 4 {
            local_1 = 1;
        }
        if rd::<i8>(i_var4.wrapping_add(0x73)) != 0 {
            local_1 = 1;
        }

        let tmp_val: i32 = rd::<i32>(i_var5.wrapping_add(0x1b));
        if tmp_val != -1 {
            local_1 = 1;
        }

        let mut u_var3: u32;
        if local_1 == 0 {
            let mut cnt: i32 = 0xf;
            let mut b_var10: bool = true;
            let mut pc8: *const i8 = (&raw mut DAT_009d1320 as *const i8)
                .add((rd::<u8>(DAT_009a73bc.wrapping_add(0x34)) as usize).wrapping_mul(0x55));
            let mut pc9: *const i8 = &raw mut DAT_0051bfd4 as *const i8;
            loop {
                if cnt == 0 {
                    break;
                }
                cnt -= 1;
                b_var10 = *pc8 == *pc9;
                pc8 = pc8.add(1);
                pc9 = pc9.add(1);
                if !b_var10 {
                    break;
                }
            }
            u_var3 = rd::<u8>(DAT_009a73bc.wrapping_add(0x34)) as u32;
            if b_var10 {
                wr::<u32>(DAT_009a73b6.wrapping_add(0xe), 0x3b);
                u_var3 = u_var2;
            }
        } else {
            u_var3 = DAT_00a07cec as u32;
            let mut u_var7: u16 = DAT_00a07cec;
            if (DAT_00a07cec as i32) < (DAT_009a1fe0 as i32) {
                loop {
                    let i_var6: u32 = (u_var7 as u32)
                        .wrapping_mul(0x23)
                        .wrapping_add(DAT_009a1fe8);
                    let t: u32 = rd::<u32>(i_var6.wrapping_add(0x1f));
                    let i_var5b: u32 = rd::<u32>(t.wrapping_add(0xa9));

                    if rd::<i8>(i_var6.wrapping_add(3)) != 0
                        && (rd::<u8>(i_var6.wrapping_add(0x16)) as u16) == DAT_009a73c0
                    {
                        if DAT_009a73b0 == u_var7 {
                            wr::<u8>(i_var5b.wrapping_add(5), 0xff);
                            wr::<u8>(
                                i_var5b.wrapping_add(0xaf),
                                rd::<u8>(i_var4.wrapping_add(0xaf)),
                            );
                            wr::<u8>(
                                i_var5b.wrapping_add(0x36),
                                rd::<u8>(i_var4.wrapping_add(0x36)),
                            );
                            wr::<u16>(
                                i_var5b.wrapping_add(0x42),
                                rd::<u16>(i_var4.wrapping_add(0x42)),
                            );
                            wr::<u16>(
                                i_var5b.wrapping_add(0x44),
                                rd::<u16>(i_var4.wrapping_add(0x44)),
                            );
                            wr::<u8>(
                                i_var5b.wrapping_add(0x48),
                                rd::<u8>(i_var4.wrapping_add(0x48)),
                            );
                            FUN_00403400();
                        } else {
                            wr::<u8>(i_var5b.wrapping_add(5), DAT_009a73b0 as u8);
                        }
                    }
                    u_var7 = u_var7.wrapping_add(1);
                    u_var3 = u_var7 as u32;
                    if !((u_var3 as i32) < (DAT_009a1fe0 as i32)) {
                        break;
                    }
                }
            }
        }
        (u_var3 & 0xffff0000) | ((local_1 as u32) & 0xff)
    }
}
