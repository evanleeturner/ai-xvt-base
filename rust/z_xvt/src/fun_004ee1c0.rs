// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00757870: i32;
    static mut DAT_00757874: i32;
    static mut DAT_00757680: i32;
    static mut DAT_00757684: i32;
    static mut DAT_0075788c: i32;
    static mut DAT_0075769c: i32;
    static mut DAT_007579b0: i32;
    static mut DAT_007579d0: i32;
    static mut DAT_00757a28: i32;
    static mut DAT_0081b850: i32;
    static mut DAT_0081bc64: i32;
    static mut DAT_00757cee: u8;
    static mut DAT_0081f7de: u8;
    static mut DAT_0081fa9e: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004cc7b0(param_1: *mut i32) -> *mut i32;
    fn FUN_004cdbc0() -> i32;
    fn FUN_004cde00() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004ee1c0() -> u32 {
    // SAFETY: all globals and callees are the game's own memory; the table
    // returned by FUN_004cc7b0 holds local_c entries of 10 ints, and all
    // pointer walks stay within the game's tables as the original does.
    unsafe {
        let mut local_c: i32 = 0;
        let mut i_var7: i32 = 10;
        let local_8: *mut i32 = FUN_004cc7b0(&raw mut local_c);
        let mut pi_var1: *mut i32 = &raw mut DAT_00757870;
        loop {
            let mut i_var9: i32 = 8;
            loop {
                let mut i_var6: i32 = 0;
                let mut b_var10 = local_c == 0;
                if 0 < local_c {
                    let mut pi_var4 = local_8.add(9);
                    loop {
                        if pi_var4.read_unaligned() != 0
                            && pi_var4.sub(1).read_unaligned() == pi_var1.read_unaligned()
                        {
                            break;
                        }
                        pi_var4 = pi_var4.add(10);
                        i_var6 = i_var6.wrapping_add(1);
                        if !(i_var6 < local_c) {
                            break;
                        }
                    }
                    b_var10 = local_c == i_var6;
                }
                if b_var10 {
                    pi_var1.write_unaligned(0);
                }
                pi_var1 = pi_var1.add(1);
                i_var9 = i_var9.wrapping_add(-1);
                if i_var9 == 0 {
                    break;
                }
            }
            i_var7 = i_var7.wrapping_add(-1);
            if i_var7 == 0 {
                break;
            }
        }

        let mut pb_var3: *mut u8 = &raw mut DAT_00757cee;
        pi_var1 = &raw mut DAT_007579b0;
        loop {
            i_var7 = 0;
            if 0 < local_c {
                let mut pi_var4 = local_8.add(9);
                loop {
                    if pi_var4.read_unaligned() != 0
                        && pi_var4.sub(1).read_unaligned() == pi_var1.read_unaligned()
                    {
                        break;
                    }
                    pi_var4 = pi_var4.add(10);
                    i_var7 = i_var7.wrapping_add(1);
                    if !(i_var7 < local_c) {
                        break;
                    }
                }
            }
            if i_var7 == 8 {
                pi_var1.write_unaligned(0);
                pb_var3.cast::<i32>().write_unaligned(0);
            }
            pb_var3 = pb_var3.add(0x2a);
            pi_var1 = pi_var1.add(1);
            if !((pi_var1 as usize) < (&raw mut DAT_007579d0 as usize)) {
                break;
            }
        }

        let mut pb_var1: *mut u8 = &raw mut DAT_0081f7de;
        loop {
            i_var7 = 0;
            let mut b_var10 = local_c == 0;
            let val: i32 = pb_var1.cast::<i32>().read_unaligned();
            if 0 < local_c {
                let mut pi_var4 = local_8.add(9);
                loop {
                    if pi_var4.read_unaligned() != 0 && pi_var4.sub(1).read_unaligned() == val {
                        break;
                    }
                    pi_var4 = pi_var4.add(10);
                    i_var7 = i_var7.wrapping_add(1);
                    if !(i_var7 < local_c) {
                        break;
                    }
                }
                b_var10 = local_c == i_var7;
            }
            if b_var10 {
                pb_var1.cast::<i32>().write_unaligned(0);
            }
            pb_var1 = pb_var1.add(0x58);
            if !((pb_var1 as usize) < (&raw mut DAT_0081fa9e as usize)) {
                break;
            }
        }

        i_var7 = 0;
        pb_var1 = &raw mut DAT_0081f7de;
        let mut i_var9: i32;
        loop {
            let i_var6 = FUN_004cdbc0();
            i_var9 = i_var7;
            let val: i32 = pb_var1.cast::<i32>().read_unaligned();
            if i_var6 == val {
                break;
            }
            pb_var1 = pb_var1.add(0x58);
            i_var7 = i_var7.wrapping_add(1);
            i_var9 = DAT_0081bc64;
            if !((pb_var1 as usize) < (&raw mut DAT_0081fa9e as usize)) {
                break;
            }
        }
        DAT_0081bc64 = i_var9;

        if DAT_0081b850 == 2 {
            local_c = 1;
        } else {
            local_c = FUN_004cde00();
        }

        let mut local_4: *mut i32 = &raw mut DAT_007579b0;
        loop {
            i_var7 = 0;
            let mut b_var10 = local_c == 0;
            if 0 < local_c {
                loop {
                    if local_4.read_unaligned() == 0 {
                        break;
                    }
                    i_var7 = i_var7.wrapping_add(1);
                    if !(i_var7 < local_c) {
                        break;
                    }
                }
                b_var10 = local_c == i_var7;
            }
            if !b_var10 && 0 < DAT_00757a28 {
                i_var9 = 0;
                let i_var7b: i32 = local_4.read_unaligned();
                let mut tmp: i32 = DAT_00757a28;
                loop {
                    let mut i_var8: i32 = 0;
                    let mut i_var6: i32 = i_var9;
                    loop {
                        if (&raw mut DAT_00757870)
                            .byte_add(i_var6 as usize)
                            .read_unaligned()
                            == i_var7b
                        {
                            if i_var8 < 7 {
                                let mut i_var2: i32 = 7i32.wrapping_sub(i_var8);
                                let mut i_var5: i32 = i_var6;
                                loop {
                                    i_var2 = i_var2.wrapping_add(-1);
                                    let a = (&raw mut DAT_00757874)
                                        .byte_add(i_var5 as usize)
                                        .read_unaligned();
                                    (&raw mut DAT_00757870)
                                        .byte_add(i_var5 as usize)
                                        .write_unaligned(a);
                                    let b = (&raw mut DAT_00757684)
                                        .byte_add(i_var5 as usize)
                                        .read_unaligned();
                                    (&raw mut DAT_00757680)
                                        .byte_add(i_var5 as usize)
                                        .write_unaligned(b);
                                    i_var5 = i_var5.wrapping_add(4);
                                    if i_var2 == 0 {
                                        break;
                                    }
                                }
                            }
                            (&raw mut DAT_0075788c)
                                .byte_add(i_var9 as usize)
                                .write_unaligned(0);
                            (&raw mut DAT_0075769c)
                                .byte_add(i_var9 as usize)
                                .write_unaligned(-1);
                        }
                        i_var6 = i_var6.wrapping_add(4);
                        i_var8 = i_var8.wrapping_add(1);
                        if !(i_var8 < 8) {
                            break;
                        }
                    }
                    i_var9 = i_var9.wrapping_add(0x20);
                    tmp = tmp.wrapping_add(-1);
                    if tmp == 0 {
                        break;
                    }
                }
            }
            local_4 = local_4.add(1);
            if 0x7579cf < (local_4 as usize) {
                return 1;
            }
        }
    }
}
