// SPDX-License-Identifier: MIT

use core::arch::asm;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006fedd2: u16;
    static mut DAT_006fedd8: u16;
    static mut DAT_0071f2ea: u8;
    static mut DAT_0071f2cd: u8;
    static mut DAT_005f1d20: *mut i32;
    static mut DAT_0052552c: i32;
    static mut DAT_00525534: u32;
    static mut DAT_0052550c: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00462b20(
    param_1: *mut u8,
    param_2: u16,
    param_3: u16,
    param_4: u32,
    param_5: i32,
    param_6: i8,
    param_7: i16,
) {
    let mut param_1 = param_1;
    // SAFETY: globals are the game's own memory; the stream and the
    // destination pointers are the caller's buffers, accessed as the original does.
    unsafe {
        DAT_006fedd2 = param_2;
        DAT_006fedd8 = param_3;
        DAT_0071f2ea = param_4 as u8;
        loop {
            let mut u_var4: u32 = (DAT_005f1d20.add(DAT_006fedd8 as usize).read_unaligned() as u32)
                .wrapping_add(DAT_006fedd2 as u32);
            if DAT_0052552c != 0x13 && DAT_00525534 == 0xa0000 {
                let r: u32;
                asm!("div {d}", d = in(reg) DAT_0052550c,
                     inout("eax") u_var4 => _, inout("edx") 0u32 => r,
                     options(nostack));
                u_var4 = r;
            }
            let mut pbvar5: *mut u8 = u_var4.wrapping_add(DAT_00525534) as *mut u8;
            let mut pbvar9: *mut u8 = param_1;
            loop {
                let mut b_var2: u8 = pbvar9.read();
                param_1 = pbvar9.wrapping_add(1);
                let b_var1: u8;
                let mut u_var8: u16;
                if b_var2 <= 0xfa {
                    let mut b = b_var2 >> 2;
                    if param_6 == 0 {
                        b = b.wrapping_add(DAT_0071f2cd);
                    }
                    b_var1 = b;
                    u_var8 = (b_var2 & 3) as u16;
                } else if b_var2 < 0xfc {
                    if param_6 == 0 {
                        DAT_0071f2cd = param_1.read();
                        pbvar9 = pbvar9.wrapping_add(2);
                    } else {
                        pbvar9 = pbvar9.wrapping_add(2);
                    }
                    continue;
                } else if b_var2 == 0xfc {
                    b_var2 = param_1.read();
                    let mut b1: u8;
                    let mut b7: u8;
                    if param_6 == 0 {
                        b7 = b_var2.wrapping_add(1);
                        b1 = b_var2;
                    } else {
                        b7 = DAT_0071f2cd;
                        b1 = DAT_0071f2cd;
                        if 0 < param_7 {
                            b_var2 = b_var2
                                .wrapping_sub(param_7 as u8)
                                .wrapping_add(DAT_0071f2cd);
                            b7 = b_var2.wrapping_add(1);
                            b1 = b_var2;
                        }
                    }
                    let mut s_var3: i16 = (pbvar9.wrapping_add(2).read() as i16).wrapping_add(1);
                    pbvar9 = pbvar9.wrapping_add(3);
                    if s_var3 != 0 {
                        loop {
                            pbvar5.write(b1);
                            if param_5 == 0 {
                                pbvar5 = pbvar5.wrapping_add(1);
                            } else {
                                pbvar5 = pbvar5.wrapping_sub(1);
                            }
                            if 0 < s_var3.wrapping_sub(1) {
                                pbvar5.write(b7);
                                if param_5 == 0 {
                                    pbvar5 = pbvar5.wrapping_add(1);
                                } else {
                                    pbvar5 = pbvar5.wrapping_sub(1);
                                }
                            }
                            s_var3 = s_var3.wrapping_sub(2);
                            if !(0 < s_var3) {
                                break;
                            }
                        }
                    }
                    continue;
                } else if b_var2 == 0xfd {
                    b_var1 = pbvar9.wrapping_add(2).read();
                    u_var8 = param_1.read() as u16;
                    param_1 = pbvar9.wrapping_add(3);
                } else if b_var2 == 0xff {
                    return;
                } else {
                    break;
                }
                // LAB_00462c49
                u_var8 = u_var8.wrapping_add(1);
                pbvar9 = param_1;
                if b_var1 as u32 == param_4 {
                    if param_5 == 0 {
                        pbvar5 = pbvar5.wrapping_add(u_var8 as usize);
                    } else {
                        pbvar5 = pbvar5.wrapping_sub(u_var8 as usize);
                    }
                } else {
                    let mut b2 = b_var1;
                    if param_6 != 0 {
                        b2 = DAT_0071f2cd;
                        if 0 < param_7 {
                            b2 = b_var1
                                .wrapping_sub(param_7 as u8)
                                .wrapping_add(DAT_0071f2cd);
                        }
                    }
                    if param_5 == 0 {
                        if u_var8 != 0 {
                            let mut p = pbvar5;
                            let mut n = u_var8;
                            while n != 0 {
                                p.write(b2);
                                p = p.wrapping_add(1);
                                n -= 1;
                            }
                            pbvar5 = pbvar5.wrapping_add(u_var8 as usize);
                        }
                    } else if u_var8 != 0 {
                        let mut n = u_var8 as u32;
                        let p6 = pbvar5.wrapping_sub(n as usize);
                        let mut p = pbvar5;
                        loop {
                            pbvar5 = p6;
                            if n == 0 {
                                break;
                            }
                            p.write(b2);
                            p = p.wrapping_sub(1);
                            n -= 1;
                        }
                    }
                }
            }
            DAT_006fedd8 = DAT_006fedd8.wrapping_add(1);
        }
    }
}
