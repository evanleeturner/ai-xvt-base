// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_0081bfb6: u32;
    static mut DAT_00757870: i32;
    static mut DAT_0081b850: i32;
    static mut DAT_00758620: u32;
    static mut DAT_00758628: u32;
    static mut DAT_00758624: u32;
    static mut DAT_00757680: u32;
    static mut DAT_0075766c: i32;
    static mut DAT_007438e4: i8;
    static mut DAT_00757874: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _rand() -> i32;
    fn FUN_004cb830(param_1: i32, param_2: *mut u32, param_3: u32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004f5990() -> u32 {
    // SAFETY: all globals and callees are the game's own memory and code;
    // offsets are applied to their addresses exactly as the original does.
    unsafe {
        let u_var1: u32 = (&raw mut DAT_0081bfb6).read_unaligned();
        let mut i_var8: i32 = 0;
        let i_var10: i32 = (u_var1 as i32).wrapping_mul(0x20);
        let mut i_var2: i32 = (&raw mut DAT_00757870)
            .byte_add(i_var10 as usize)
            .read_unaligned();
        let mut i_var4: i32 = i_var10;

        loop {
            if i_var2 == 0 {
                if (&raw mut DAT_0081b850).read_unaligned() != 2 {
                    (&raw mut DAT_00758620).write_unaligned(0x57);
                    let cur = (&raw mut DAT_0081bfb6).read_unaligned();
                    let mut p9 =
                        (&raw mut DAT_00757680).byte_add((cur as i32).wrapping_mul(0x20) as usize);
                    let mut p11 = &raw mut DAT_00758628;
                    (&raw mut DAT_00758624).write_unaligned(cur);
                    let mut n = 8;
                    while n != 0 {
                        p11.write_unaligned(p9.read_unaligned());
                        p9 = p9.byte_add(4);
                        p11 = p11.byte_add(4);
                        n -= 1;
                    }
                    FUN_004cb830(0, &raw mut DAT_00758620, 0x28);
                }
                return 1;
            }
            i_var2 = _rand();
            i_var2 = (i_var2 % 9).wrapping_add(1);
            let slot = (&raw mut DAT_00757680).byte_add(i_var4 as usize);
            slot.write_unaligned(0xffff_ffff);
            loop {
                let mut i_var5: i32 = 0;
                if 0 < (&raw mut DAT_0075766c).read_unaligned() {
                    loop {
                        let pc =
                            (&raw mut DAT_007438e4).byte_add((i_var5.wrapping_mul(0x562)) as usize);
                        if pc.read() != 0 && (pc.byte_sub(0xc).cast::<u8>().read() as u32) == u_var1
                        {
                            let mut i_var7: i32 = 0;
                            if 0 < i_var8 {
                                let mut pi = (&raw mut DAT_00757680)
                                    .byte_add(i_var10 as usize)
                                    .cast::<i32>();
                                loop {
                                    if pi.read_unaligned() == i_var5 {
                                        break;
                                    }
                                    pi = pi.byte_add(4);
                                    i_var7 += 1;
                                    if i_var7 >= i_var8 {
                                        break;
                                    }
                                }
                            }
                            if i_var7 == i_var8 {
                                i_var2 = i_var2.wrapping_sub(1);
                                if i_var2 == 0 {
                                    slot.write_unaligned(i_var5 as u32);
                                    break;
                                }
                            }
                        }
                        i_var5 += 1;
                        if i_var5 >= (&raw mut DAT_0075766c).read_unaligned() {
                            break;
                        }
                    }
                }
                if slot.read_unaligned() != 0xffff_ffff {
                    break;
                }
            }
            i_var2 = (&raw mut DAT_00757874)
                .byte_add(i_var4 as usize)
                .read_unaligned();
            i_var4 = i_var4.wrapping_add(4);
            i_var8 += 1;
        }
    }
}
