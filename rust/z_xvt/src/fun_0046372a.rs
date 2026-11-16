// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00730894: i16;
    static mut DAT_00732e58: u16;
    static mut DAT_0070acc0: i16;
    static mut DAT_005f10ec: u16;
    static mut DAT_005f10f4: u16;
    static mut DAT_005f10e8: u16;
    static mut DAT_005f10f8: u16;
    static mut DAT_006d4a4e: u8;
    static mut DAT_00732e52: u16;
    static mut DAT_006d9980: u16;
    static mut DAT_00732e8c: u16;
    static mut DAT_00732a34: u16;
    static mut DAT_0070a188: i8;
    static mut DAT_006d4998: i8;
    static mut DAT_0070acc2: u8;
    static mut DAT_00709ff4: u32;
    static mut DAT_006ffbf0: i32;
    static mut DAT_005f1d20: u32;
    static mut DAT_005f1d24: *mut u32;
    static mut DAT_0052552c: i32;
    static mut DAT_00525534: u32;
    static mut DAT_0052550c: u32;
    static mut DAT_0070acc8: u32;
    static mut DAT_0071e524: i8;
    static mut DAT_006da9fa: u8;
    static mut DAT_00732e50: u8;
    static mut DAT_0071b012: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_00463e10(param_1: i32);
}

unsafe fn urem(a: u32, b: u32) -> u32 {
    let rem: u32;
    // SAFETY: the machine's own unsigned divide, faulting on a zero divisor as the original does.
    unsafe {
        core::arch::asm!(
            "div {b}",
            b = in(reg) b,
            inout("eax") a => _,
            inout("edx") 0u32 => rem,
        );
    }
    rem
}

// mode 0: advance row by DAT_006d4a4e + 1; otherwise by b1 + 2
unsafe fn flush(mode: u32, b1: u8) {
    // SAFETY: the globals and the callee are the game's own.
    unsafe {
        if DAT_00730894 != 0 && (DAT_00732e58 as i16) < DAT_0070acc0 {
            DAT_005f10ec = DAT_0070acc0 as u16;
            DAT_005f10f4 = DAT_00732e58;
            DAT_005f10e8 = (DAT_006d4a4e as u16).wrapping_add(DAT_00732e52);
            DAT_005f10f8 = DAT_00732e52;
            if (DAT_00732e58 as i32) < (DAT_006d9980 as i16 as i32) {
                DAT_005f10f4 = DAT_006d9980;
            }
            if (DAT_00732e52 as i32) < (DAT_00732e8c as i16 as i32) {
                DAT_005f10f8 = DAT_00732e8c;
            }
            if (DAT_00732a34 as i16 as i32) < (DAT_005f10e8 as i32) {
                DAT_005f10e8 = DAT_00732a34;
            }
            if DAT_005f10f8 < DAT_005f10e8 {
                FUN_00463e10(0);
            }
        }
        DAT_00732e58 = DAT_006d9980;
        if mode == 0 {
            DAT_00732e52 = DAT_00732e52
                .wrapping_add(DAT_006d4a4e as u16)
                .wrapping_add(1);
        } else {
            DAT_00732e52 = DAT_00732e52.wrapping_add(b1 as u16).wrapping_add(2);
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_0046372a() {
    let mut in_al: u8 = 0;
    let mut in_zf: u8 = 0;
    // SAFETY: captures the flags and AL the caller passed, before anything else runs.
    unsafe {
        core::arch::asm!(
            "setz byte ptr [edx]",
            "mov byte ptr [ecx], al",
            in("ecx") &raw mut in_al,
            in("edx") &raw mut in_zf,
        );
    }

    if in_zf != 0 {
        // SAFETY: game-owned globals.
        unsafe {
            flush(0, 0);
        }
        return;
    }
    if in_al < 0x20 {
        return;
    }
    // SAFETY: the globals, the glyph table, the frame buffer and the row table are the game's own memory,
    // read and written where the original does.
    unsafe {
        if DAT_0070a188 != 0 && DAT_006d4998 == 0 && 0x60 < in_al && in_al < 0x7b {
            in_al = in_al.wrapping_sub(0x20);
        }
        let pbvar5 = ((in_al.wrapping_sub(0x20) as u32)
            .wrapping_mul(DAT_0070acc2 as u32)
            .wrapping_add(DAT_00709ff4)) as *const u8;
        let b_var1: u8 = pbvar5.add(1).read();
        let b_var2: u8 = pbvar5.read();
        let width: u32 = b_var2 as u32;
        if (DAT_0070acc0 as i32) <= (DAT_00732e58 as i16 as i32).wrapping_add(width as i32)
            && DAT_006ffbf0 != 0
        {
            flush(1, b_var1);
        }
        let mut i_var11: i32 = DAT_00732e52 as i16 as i32;
        let mut u_var12: u32 = 0;
        let mut i_stack4: i32 = 0;
        let mut i_var3: i32 = i_var11;
        if i_var11 < (DAT_00732e8c as i16 as i32) {
            i_var3 = DAT_00732e8c as i16 as i32;
        }
        let mut u_var6: u32 = ((DAT_005f1d20.wrapping_add((i_var3 as u32).wrapping_mul(4)))
            as *const u32)
            .read_unaligned();

        'a3d: {
            if DAT_0052552c == 0x13 {
                if DAT_00525534 == 0xa0000 {
                    break 'a3d;
                }
            } else if DAT_00525534 == 0xa0000 {
                u_var6 = urem(u_var6, DAT_0052550c);
                let mut i_var9: i32 =
                    (i_var11 as u32).wrapping_add(1).wrapping_add(b_var1 as u32) as i32;
                if (DAT_00732a34 as i16 as i32) < i_var9 {
                    i_var9 = DAT_00732a34 as i16 as i32;
                }
                if 0xffff
                    < ((i_var9.wrapping_sub(i_var3) as u32)
                        .wrapping_mul(DAT_0070acc8)
                        .wrapping_add(u_var6)) as i32
                {
                    i_stack4 = 1;
                }
                break 'a3d;
            }
            i_stack4 = 1;
        }

        if i_stack4 == 0 {
            i_var3 = u_var6.wrapping_add(DAT_00525534) as i32;
            let mut pu13 = pbvar5.add(2);
            if i_var11 < (b_var1 as i32).wrapping_add(i_var11) {
                loop {
                    u_var6 = (pu13 as *const u32).read_unaligned();
                    let mut u_var7: u32 = width;
                    if DAT_0071e524 != 0 {
                        u_var7 = width.wrapping_add(1);
                    }
                    let mut i_var9: i32 = DAT_00732e58 as i16 as i32;
                    i_stack4 = DAT_006d9980 as i16 as i32;
                    if i_var9 < i_stack4 {
                        if (u_var7 as i32) <= i_stack4.wrapping_sub(i_var9) {
                            break;
                        }
                        u_var7 = u_var7
                            .wrapping_sub(i_stack4 as u32)
                            .wrapping_add(i_var9 as u32);
                        u_var6 = u_var6.wrapping_shl(
                            ((DAT_006d9980 as u8).wrapping_sub(DAT_00732e58 as u8) & 0x1f) as u32,
                        );
                        i_var9 = i_stack4;
                    }
                    if (DAT_00732a34 as i16 as i32) <= i_var11 {
                        break;
                    }
                    if (DAT_00732e8c as i16 as i32) <= i_var11 {
                        i_stack4 = (i_var9 as u32).wrapping_add(u_var7) as i32;
                        if (DAT_0070acc0 as i32) < i_stack4 {
                            u_var7 = (DAT_0070acc0 as i32 as u32).wrapping_sub(i_var9 as u32);
                            if (u_var7 as i32) < 1 {
                                break;
                            }
                        }
                        let mut p10 = (i_var9 as u32).wrapping_add(i_var3 as u32) as *mut u8;
                        while u_var7 != 0 {
                            u_var7 = u_var7.wrapping_sub(1);
                            let mut u_var4 = DAT_006da9fa;
                            if (u_var6 & 0x8000_0000) == 0 {
                                u_var4 = DAT_00732e50;
                                if DAT_0071e524 != 0 && (u_var12 & 0x8000_0000) != 0 {
                                    u_var4 = DAT_0071b012;
                                }
                            }
                            p10.write(u_var4);
                            p10 = p10.add(1);
                            u_var6 = u_var6.wrapping_mul(2);
                            u_var12 = u_var12.wrapping_mul(2);
                        }
                        i_var3 = (i_var3 as u32).wrapping_add(DAT_005f1d24.read_unaligned()) as i32;
                    }
                    i_var11 = i_var11.wrapping_add(1);
                    let u_tmp: u32 = (pu13 as *const u32).read_unaligned();
                    u_var12 = u_tmp >> 1;
                    pu13 = pu13.add(8);
                    if !(i_var11 < (DAT_00732e52 as i16 as i32).wrapping_add(b_var1 as i32)) {
                        break;
                    }
                }
            }
        } else {
            let mut pu13 = pbvar5.add(2);
            if i_var11 < (b_var1 as i32).wrapping_add(i_var11) {
                loop {
                    u_var6 = (pu13 as *const u32).read_unaligned();
                    let mut u_var7: u32 = width;
                    if DAT_0071e524 != 0 {
                        u_var7 = width.wrapping_add(1);
                    }
                    i_var3 = DAT_00732e58 as i16 as i32;
                    let i_var9: i32 = DAT_006d9980 as i16 as i32;
                    if i_var3 < i_var9 {
                        if (u_var7 as i32) <= i_var9.wrapping_sub(i_var3) {
                            break;
                        }
                        u_var7 = u_var7
                            .wrapping_sub(i_var9 as u32)
                            .wrapping_add(i_var3 as u32);
                        u_var6 = u_var6.wrapping_shl(
                            ((DAT_006d9980 as u8).wrapping_sub(DAT_00732e58 as u8) & 0x1f) as u32,
                        );
                        i_var3 = i_var9;
                    }
                    if (DAT_00732a34 as i16 as i32) <= i_var11 {
                        break;
                    }
                    if (DAT_00732e8c as i16 as i32) <= i_var11 {
                        if (DAT_0070acc0 as i32) < (i_var3 as u32).wrapping_add(u_var7) as i32 {
                            u_var7 = (DAT_0070acc0 as i32 as u32).wrapping_sub(i_var3 as u32);
                            if (u_var7 as i32) < 1 {
                                break;
                            }
                        }
                        let mut u_var8: u32 = ((DAT_005f1d20
                            .wrapping_add((i_var11 as u32).wrapping_mul(4)))
                            as *const u32)
                            .read_unaligned()
                            .wrapping_add(i_var3 as u32);
                        if DAT_0052552c != 0x13 && DAT_00525534 == 0xa0000 {
                            u_var8 = urem(u_var8, DAT_0052550c);
                        }
                        let mut p10 = u_var8.wrapping_add(DAT_00525534) as *mut u8;
                        while u_var7 != 0 {
                            u_var7 = u_var7.wrapping_sub(1);
                            let mut u_var4 = DAT_006da9fa;
                            if (u_var6 & 0x8000_0000) == 0 {
                                u_var4 = DAT_00732e50;
                                if DAT_0071e524 != 0 && (u_var12 & 0x8000_0000) != 0 {
                                    u_var4 = DAT_0071b012;
                                }
                            }
                            p10.write(u_var4);
                            p10 = p10.add(1);
                            u_var6 = u_var6.wrapping_mul(2);
                            u_var12 = u_var12.wrapping_mul(2);
                        }
                    }
                    i_var11 = i_var11.wrapping_add(1);
                    let u_tmp: u32 = (pu13 as *const u32).read_unaligned();
                    u_var12 = u_tmp >> 1;
                    pu13 = pu13.add(8);
                    if !(i_var11 < (DAT_00732e52 as i16 as i32).wrapping_add(b_var1 as i32)) {
                        break;
                    }
                }
            }
        }
        DAT_00732e58 = DAT_00732e58.wrapping_add(b_var2 as u16);
        if DAT_0070acc0 <= (DAT_00732e58 as i16) && DAT_006ffbf0 != 0 {
            flush(1, b_var1);
        }
    }
}
