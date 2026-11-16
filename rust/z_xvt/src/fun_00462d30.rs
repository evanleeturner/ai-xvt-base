// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00525518: i32;
    static mut DAT_006fedd2: u16;
    static mut DAT_006fedd8: u16;
    static mut DAT_0071f2ea: u8;
    static mut DAT_0071f2cd: u8;
    static mut DAT_005f1d20: *mut i32;
    static mut DAT_0052552c: i32;
    static mut DAT_00525534: u32;
    static mut DAT_0052550c: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00415b30(param_1: *mut u8, param_2: u16, param_3: u16, param_4: u32, param_5: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00462d30(
    param_1: *mut u8,
    param_2: u16,
    param_3: u16,
    param_4: u32,
    param_5: i32,
) {
    let mut param_1 = param_1;
    // SAFETY: the globals and callee are the game's own; param_1 points at the
    // caller's encoded stream and the destination is the game's video memory,
    // read and written exactly where the original does.
    unsafe {
        if DAT_00525518 == 2 {
            FUN_00415b30(param_1, param_2, param_3, param_4, param_5);
            return;
        }
        DAT_006fedd2 = param_2;
        DAT_006fedd8 = param_3;
        DAT_0071f2ea = param_4 as u8;
        loop {
            let mut u_var3: u32 = (DAT_005f1d20.add(DAT_006fedd8 as usize).read_unaligned() as u32)
                .wrapping_add(DAT_006fedd2 as u32);
            if DAT_0052552c != 0x13 && DAT_00525534 == 0xa0000 {
                let d = DAT_0052550c;
                let q: u32;
                core::arch::asm!(
                    "div {d:e}",
                    d = in(reg) d,
                    inout("eax") u_var3 => q,
                    inout("edx") 0u32 => _,
                );
                let _ = q;
                // remainder is in edx; recompute below
                let r: u32;
                let n = (DAT_005f1d20.add(DAT_006fedd8 as usize).read_unaligned() as u32)
                    .wrapping_add(DAT_006fedd2 as u32);
                core::arch::asm!(
                    "div {d:e}",
                    d = in(reg) d,
                    inout("eax") n => _,
                    inout("edx") 0u32 => r,
                );
                u_var3 = r;
            }
            let mut pcvar4: *mut i8 = u_var3.wrapping_add(DAT_00525534) as *mut i8;
            'inner: loop {
                let pbvar5 = param_1.add(1);
                let mut bvar1: u8 = param_1.read();
                let mut u_stack_a: u16;
                let b_stack_4: u8;
                'dispatch: {
                    if bvar1 <= 0xfa {
                        u_stack_a = (bvar1 & 3) as u16;
                        b_stack_4 = (bvar1 >> 2).wrapping_add(DAT_0071f2cd);
                        param_1 = pbvar5;
                        break 'dispatch;
                    }
                    if bvar1 < 0xfc {
                        param_1 = param_1.add(2);
                        DAT_0071f2cd = pbvar5.read();
                        continue 'inner;
                    }
                    if bvar1 == 0xfc {
                        bvar1 = pbvar5.read();
                        let p5 = param_1.add(2);
                        param_1 = param_1.add(3);
                        u_stack_a = (p5.read() as u16).wrapping_add(1);
                        if u_stack_a != 0 {
                            loop {
                                *pcvar4 = bvar1.wrapping_add(4) as i8;
                                if param_5 == 0 {
                                    pcvar4 = pcvar4.add(1);
                                } else {
                                    pcvar4 = pcvar4.sub(1);
                                }
                                if 0 < (u_stack_a.wrapping_sub(1) as i16) {
                                    *pcvar4 = bvar1.wrapping_add(5) as i8;
                                    if param_5 == 0 {
                                        pcvar4 = pcvar4.add(1);
                                    } else {
                                        pcvar4 = pcvar4.sub(1);
                                    }
                                }
                                u_stack_a = u_stack_a.wrapping_sub(2);
                                if !(0 < (u_stack_a as i16)) {
                                    break;
                                }
                            }
                        }
                        continue 'inner;
                    }
                    if bvar1 == 0xfd {
                        u_stack_a = pbvar5.read() as u16;
                        b_stack_4 = param_1.add(2).read();
                        param_1 = param_1.add(3);
                        break 'dispatch;
                    }
                    if bvar1 == 0xff {
                        return;
                    }
                    DAT_006fedd8 = DAT_006fedd8.wrapping_add(1);
                    param_1 = pbvar5;
                    break 'inner;
                }
                // LAB_00462f1f
                u_stack_a = u_stack_a.wrapping_add(1);
                if b_stack_4 as u32 == param_4 {
                    if param_5 == 0 {
                        pcvar4 = pcvar4.wrapping_add(u_stack_a as usize);
                    } else {
                        pcvar4 = pcvar4.wrapping_sub(u_stack_a as usize);
                    }
                } else if param_5 == 0 {
                    if u_stack_a != 0 {
                        let c_var2: u8 = b_stack_4.wrapping_add(4);
                        let mut pcvar6 = pcvar4;
                        let u_fill: u32 = (c_var2 as u32).wrapping_mul(0x01010101);
                        let mut n = (u_stack_a >> 2) as u32;
                        while n != 0 {
                            pcvar6.cast::<u32>().write_unaligned(u_fill);
                            pcvar6 = pcvar6.add(4);
                            n -= 1;
                        }
                        let mut n = (u_stack_a & 3) as u32;
                        while n != 0 {
                            *pcvar6 = c_var2 as i8;
                            pcvar6 = pcvar6.add(1);
                            n -= 1;
                        }
                        pcvar4 = pcvar4.wrapping_add(u_stack_a as usize);
                    }
                } else if u_stack_a != 0 {
                    let mut pcvar6 = pcvar4;
                    let mut n = u_stack_a as u32;
                    while n != 0 {
                        *pcvar6 = b_stack_4.wrapping_add(4) as i8;
                        pcvar6 = pcvar6.wrapping_sub(1);
                        n -= 1;
                    }
                    pcvar4 = pcvar4.wrapping_sub(u_stack_a as usize);
                }
            }
        }
    }
}
