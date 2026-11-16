// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_009fe7e4: u16;
    static mut DAT_00a08108: u16;
    static mut DAT_009a6fe0: u16;
    static mut DAT_00a08102: u16;
    static mut DAT_009a20ae: u8;
    static mut DAT_009d80c8: i8;
    static mut DAT_009a1ff4: i8;
    static mut DAT_009d7674: u32;
    static mut DAT_009d8c02: u16;
    static mut DAT_009d8c00: u16;
    static mut DAT_009cd270: u16;
    static mut DAT_00a0813c: u16;
    static mut DAT_00a07ce4: u16;
    static mut DAT_005233ec: u32;
    static mut DAT_005233f4: u32;
    static mut DAT_005233cc: u32;
    static mut DAT_009d8c08: u32;
    static mut DAT_009ec464: i8;
    static mut DAT_009a807a: u8;
    static mut DAT_00a08100: u8;
    static mut DAT_009e8f52: u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_00410110();
    fn FUN_00410ff0();
    fn FUN_0040e1a0() -> i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0040e190(param_1: i32) -> u32;
}

#[inline(always)]
unsafe fn machine_urem(n: u32, d: u32) -> u32 {
    let r: u32;
    // SAFETY: unsigned divide by the game's own divisor; faults as the original does.
    unsafe {
        core::arch::asm!(
            "div {d}",
            d = in(reg) d,
            inout("eax") n => _,
            inout("edx") 0u32 => r,
        );
    }
    r
}

#[unsafe(no_mangle)]
#[allow(non_snake_case, unused_assignments, unused_mut, unused_variables)]
pub unsafe extern "stdcall" fn FUN_0040f52a() {
    let mut in_al: u8 = 0;
    let in_zf: i32 = 0;
    // SAFETY: all globals and callees are the game's own memory and code; the
    // glyph and screen pointers are read and written where the original does.
    unsafe {
        if in_zf != 0 {
            if DAT_009fe7e4 != 0 {
                FUN_00410110();
            }
            DAT_00a08108 = DAT_009a6fe0;
            DAT_00a08102 = DAT_00a08102
                .wrapping_add(DAT_009a20ae as u16)
                .wrapping_add(1);
            return;
        }
        if in_al < 0x20 {
            return;
        }
        if DAT_009d80c8 != 0 && DAT_009a1ff4 == 0 && 0x60 < in_al && in_al < 0x7b {
            in_al = in_al.wrapping_sub(0x20);
        }
        let pb8 = ((in_al.wrapping_sub(0x20) as u32)
            .wrapping_mul(DAT_009d8c02 as u32)
            .wrapping_add(DAT_009d7674)) as *const u8;
        let b_var1: u8 = pb8.read();
        let b_var2: u8 = pb8.byte_add(1).read();
        let mut p_stack: *const u8 = pb8.byte_add(2);
        let u_stack_c: u32 = b_var1 as u32;
        if (DAT_009d8c00 as i32) <= (DAT_00a08108 as u32).wrapping_add(u_stack_c) as i32
            && DAT_009cd270 != 0
        {
            if DAT_009fe7e4 != 0 {
                FUN_00410110();
            }
            DAT_00a08108 = DAT_009a6fe0;
            DAT_00a08102 = DAT_00a08102.wrapping_add(b_var2 as u16).wrapping_add(2);
        }
        let mut u_var15: u32 = 0;
        let mut b_var3: u8 = 0;
        let mut i_var5: i32 = DAT_00a08102 as i32;
        if i_var5 < DAT_00a0813c as i32 {
            i_var5 = DAT_00a0813c as i32;
        }
        let mut u_var4: u32 = FUN_0040e190(i_var5);
        'l6dd: {
            if DAT_005233ec == 0x13 {
                if DAT_005233f4 == 0xa0000 {
                    break 'l6dd;
                }
            } else if DAT_005233f4 == 0xa0000 {
                u_var4 = machine_urem(u_var4, DAT_005233cc);
                FUN_00410ff0();
                let mut i_var11: i32 = (DAT_00a08102 as u32)
                    .wrapping_add(1)
                    .wrapping_add(b_var2 as u32) as i32;
                if (DAT_00a07ce4 as i32) < i_var11 {
                    i_var11 = DAT_00a07ce4 as i32;
                }
                let t = u_var4
                    .wrapping_add((i_var11.wrapping_sub(i_var5) as u32).wrapping_mul(DAT_009d8c08))
                    as i32;
                if 0xffff < t {
                    b_var3 = 1;
                }
                if DAT_005233f4 == 0xa0000 {
                    break 'l6dd;
                }
            }
            b_var3 = 1;
        }
        if b_var3 != 0 {
            i_var5 = DAT_00a08102 as i32;
            let u_stack_10: u32 = b_var2 as u32;
            if i_var5 < u_stack_10.wrapping_add(i_var5 as u32) as i32 {
                loop {
                    u_var4 = p_stack.cast::<u32>().read_unaligned();
                    let mut u_var14: u32 = u_stack_c;
                    if DAT_009ec464 != 0 {
                        u_var14 = u_stack_c.wrapping_add(1);
                    }
                    let mut i_var11: i32 = DAT_00a08108 as i32;
                    let i_var9: i32 = DAT_009a6fe0 as i32;
                    if i_var11 < i_var9 {
                        if (u_var14 as i32) <= i_var9.wrapping_sub(i_var11) {
                            break;
                        }
                        u_var14 = u_var14
                            .wrapping_sub(i_var9 as u32)
                            .wrapping_add(i_var11 as u32);
                        let sh = ((DAT_009a6fe0 as i8 as i32)
                            .wrapping_sub(DAT_00a08108 as i8 as i32)
                            & 0x1f) as u32;
                        u_var4 = u_var4.wrapping_shl(sh);
                        i_var11 = i_var9;
                    }
                    if (DAT_00a07ce4 as i32) <= i_var5 {
                        break;
                    }
                    if (DAT_00a0813c as i32) <= i_var5 {
                        if (DAT_009d8c00 as i32) < (i_var11 as u32).wrapping_add(u_var14) as i32 {
                            u_var14 = (DAT_009d8c00 as i32).wrapping_sub(i_var11) as u32;
                            if (u_var14 as i32) < 1 {
                                break;
                            }
                        }
                        let i_var9b = FUN_0040e190(i_var5) as i32;
                        let mut u_var12: u32 = (i_var11 as u32).wrapping_add(i_var9b as u32);
                        if DAT_005233ec != 0x13 && DAT_005233f4 == 0xa0000 {
                            u_var12 = machine_urem(u_var12, DAT_005233cc);
                            FUN_00410ff0();
                        }
                        let mut p10 = u_var12.wrapping_add(DAT_005233f4) as *mut u8;
                        while u_var14 != 0 {
                            u_var14 = u_var14.wrapping_sub(1);
                            let mut u_var7: u8 = DAT_009a807a;
                            if (u_var4 & 0x80000000) == 0 {
                                u_var7 = DAT_00a08100;
                                if DAT_009ec464 != 0 && (u_var15 & 0x80000000) != 0 {
                                    u_var7 = DAT_009e8f52;
                                }
                            }
                            p10.write(u_var7);
                            p10 = p10.byte_add(1);
                            u_var4 = u_var4.wrapping_mul(2);
                            u_var15 = u_var15.wrapping_mul(2);
                        }
                    }
                    let p6 = p_stack.byte_add(8);
                    i_var5 = i_var5.wrapping_add(1);
                    u_var15 = p_stack.cast::<u32>().read_unaligned() >> 1;
                    p_stack = p6;
                    if !(i_var5 < (DAT_00a08102 as i32).wrapping_add(u_stack_10 as i32)) {
                        break;
                    }
                }
            }
        } else {
            i_var5 = u_var4.wrapping_add(DAT_005233f4) as i32;
            let mut i_var11: i32 = DAT_00a08102 as i32;
            let u_stack_10: u32 = b_var2 as u32;
            if i_var11 < u_stack_10.wrapping_add(i_var11 as u32) as i32 {
                loop {
                    u_var4 = p_stack.cast::<u32>().read_unaligned();
                    let mut u_var14: u32 = u_stack_c;
                    if DAT_009ec464 != 0 {
                        u_var14 = u_stack_c.wrapping_add(1);
                    }
                    let mut i_var9: i32 = DAT_00a08108 as i32;
                    let i_var13: i32 = DAT_009a6fe0 as i32;
                    if i_var9 < i_var13 {
                        if (u_var14 as i32) <= i_var13.wrapping_sub(i_var9) {
                            break;
                        }
                        u_var14 = u_var14
                            .wrapping_sub(i_var13 as u32)
                            .wrapping_add(i_var9 as u32);
                        let sh = ((DAT_009a6fe0 as i8 as i32)
                            .wrapping_sub(DAT_00a08108 as i8 as i32)
                            & 0x1f) as u32;
                        u_var4 = u_var4.wrapping_shl(sh);
                        i_var9 = i_var13;
                    }
                    if (DAT_00a07ce4 as i32) <= i_var11 {
                        break;
                    }
                    if (DAT_00a0813c as i32) <= i_var11 {
                        if (DAT_009d8c00 as i32) < (i_var9 as u32).wrapping_add(u_var14) as i32 {
                            u_var14 = (DAT_009d8c00 as i32).wrapping_sub(i_var9) as u32;
                            if (u_var14 as i32) < 1 {
                                break;
                            }
                        }
                        let mut p10 = (i_var9 as u32).wrapping_add(i_var5 as u32) as *mut u8;
                        while u_var14 != 0 {
                            u_var14 = u_var14.wrapping_sub(1);
                            let mut u_var7: u8 = DAT_009a807a;
                            if (u_var4 & 0x80000000) == 0 {
                                u_var7 = DAT_00a08100;
                                if DAT_009ec464 != 0 && (u_var15 & 0x80000000) != 0 {
                                    u_var7 = DAT_009e8f52;
                                }
                            }
                            p10.write(u_var7);
                            p10 = p10.byte_add(1);
                            u_var4 = u_var4.wrapping_mul(2);
                            u_var15 = u_var15.wrapping_mul(2);
                        }
                        let i_var9b = FUN_0040e1a0();
                        i_var5 = i_var5.wrapping_add(i_var9b);
                    }
                    i_var11 = i_var11.wrapping_add(1);
                    let p6 = p_stack.byte_add(8);
                    u_var15 = p_stack.cast::<u32>().read_unaligned() >> 1;
                    p_stack = p6;
                    if !(i_var11 < (DAT_00a08102 as i32).wrapping_add(u_stack_10 as i32)) {
                        break;
                    }
                }
            }
        }
        DAT_00a08108 = DAT_00a08108.wrapping_add(b_var1 as u16);
        if DAT_009d8c00 <= DAT_00a08108 && DAT_009cd270 != 0 {
            if DAT_009fe7e4 != 0 {
                FUN_00410110();
            }
            DAT_00a08108 = DAT_009a6fe0;
            DAT_00a08102 = DAT_00a08102.wrapping_add(b_var2 as u16).wrapping_add(2);
        }
    }
}
