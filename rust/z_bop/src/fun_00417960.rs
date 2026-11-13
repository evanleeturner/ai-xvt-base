// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009d1308: i32;
    static mut DAT_00a07ce8: i32;
    static mut DAT_009a8e24: i32;
    static mut DAT_009cd274: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00417960(param_1: *mut i32, param_2: *mut i8) -> i32 {
    // SAFETY: the globals are the game's own; param_1 and param_2 point at
    // the caller's buffers, read and written where the original does.
    unsafe {
        let mut param_2 = param_2;
        let mut i_var3: i32 = 0;
        param_1.write_unaligned(DAT_009d1308.wrapping_add(DAT_00a07ce8));
        let mut pb_var1: *mut u8 = param_1.cast::<u8>().wrapping_add(4);
        let mut i_var4: i32 = 0;
        if 0 < DAT_009d1308.wrapping_add(DAT_00a07ce8) {
            loop {
                let mut pc_var5: *mut i8;
                if i_var4 < DAT_009a8e24 || {
                    pc_var5 = param_2;
                    DAT_009cd274 <= i_var4
                } {
                    let mut b_var2: u8 = 0;
                    pc_var5 = param_2.wrapping_add(1);
                    if param_2.read() != 0 {
                        b_var2 = 1;
                        pc_var5 = param_2.wrapping_add(0x24);
                        if param_2.byte_add(0x20).cast::<i32>().read_unaligned() != 0 {
                            b_var2 = 3;
                            pc_var5 = param_2.wrapping_add(0xd5);
                            if param_2.byte_add(0xcd).cast::<i32>().read_unaligned() != 0 {
                                b_var2 = 7;
                                pc_var5 = param_2.wrapping_add(0x537);
                            }
                            if param_2.byte_add(0xc9).cast::<i32>().read_unaligned() != 0 {
                                b_var2 |= 8;
                                pc_var5 = pc_var5.wrapping_add(10);
                            }
                            if param_2.byte_add(0xd1).cast::<i32>().read_unaligned() != 0 {
                                b_var2 |= 0x10;
                                pc_var5 = pc_var5.wrapping_add(0x4c);
                            }
                        }
                    }
                    if b_var2 == 0 {
                        i_var3 = i_var3.wrapping_add(1);
                        if 0x7d < i_var3 {
                            pb_var1.write((i_var3 as u8) | 0x80);
                            pb_var1 = pb_var1.wrapping_add(1);
                            i_var3 = 0;
                        }
                    } else {
                        if i_var3 != 0 {
                            pb_var1.write((i_var3 as u8) | 0x80);
                            pb_var1 = pb_var1.wrapping_add(1);
                            i_var3 = 0;
                        }
                        pb_var1.write(b_var2);
                        pb_var1 = pb_var1.wrapping_add(1);
                    }
                }
                i_var4 = i_var4.wrapping_add(1);
                param_2 = pc_var5;
                if !(i_var4 < DAT_009d1308.wrapping_add(DAT_00a07ce8)) {
                    break;
                }
            }
        }
        if i_var3 != 0 {
            pb_var1.write((i_var3 as u8) | 0x80);
            pb_var1 = pb_var1.wrapping_add(1);
        }
        (pb_var1 as usize as i32).wrapping_sub(param_1 as usize as i32)
    }
}
