// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00525568: i32;
    static mut DAT_0071b740: [i8; 1];
    static mut DAT_0071ecf4: i32;
    static mut DAT_0071f720: [i32; 1];
    static mut DAT_006db730: [i32; 1];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_0049af60(param_1: i32) {
    // SAFETY: all globals are the game's own memory; pointers are walked
    // with the same offsets and bounds checks as the original.
    unsafe {
        let cnt = (&raw mut DAT_006db730)
            .cast::<i32>()
            .offset(param_1 as isize);
        let flag = (&raw mut DAT_0071b740)
            .cast::<i8>()
            .offset(param_1.wrapping_mul(0x5bd) as isize)
            .read();
        if DAT_00525568 == 0 && flag != 0 && param_1 != DAT_0071ecf4 {
            let p1 = (&raw mut DAT_0071f720)
                .cast::<i32>()
                .offset(param_1.wrapping_mul(0x7e9) as isize);
            let mut i_var3: i32 = 0;
            let mut p6 = p1;
            if cnt.read() > 0 {
                loop {
                    if p6.read_unaligned() == 0 && p6.add(1).read_unaligned() == 2 {
                        if cnt.read() != 0 && p1 <= p6 {
                            let i_var2 = cnt.read().wrapping_sub(1);
                            let mut i_var5: i32 = 0;
                            cnt.write(i_var2);
                            let mut p4 = p1;
                            if i_var2 > 0 {
                                loop {
                                    if p6 <= p4 {
                                        p4.write_unaligned(p4.byte_add(0x12).read_unaligned());
                                        p4.add(1)
                                            .write_unaligned(p4.byte_add(0x16).read_unaligned());
                                        p4.add(2)
                                            .write_unaligned(p4.byte_add(0x1a).read_unaligned());
                                        p4.add(3)
                                            .write_unaligned(p4.byte_add(0x1e).read_unaligned());
                                        p4.add(4).cast::<u16>().write_unaligned(
                                            p4.byte_add(0x22).cast::<u16>().read_unaligned(),
                                        );
                                    }
                                    i_var5 = i_var5.wrapping_add(1);
                                    p4 = p4.byte_add(0x12);
                                    if !(i_var5 < cnt.read()) {
                                        break;
                                    }
                                }
                            }
                        }
                    } else {
                        p6 = p6.byte_add(0x12);
                        i_var3 = i_var3.wrapping_add(1);
                    }
                    if !(i_var3 < cnt.read()) {
                        break;
                    }
                }
            }
        }
    }
}
